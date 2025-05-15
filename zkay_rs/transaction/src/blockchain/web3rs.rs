#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// import json
// import os
// import tempfile
// from abc import abstractmethod
// from contextlib import contextmanager
// from pathlib import Path
// from typing import Any, Dict, Optional, Tuple, List, Union

// from eth_tester import PyEVMBackend, EthereumTester
// from web3 import Web3
// use my_logging;
// use foundry_cli::opts::{RpcOpts,EthereumOpts};
// use foundry_cli::{handler, utils};
use alloy_chains::Chain;
use alloy_dyn_abi::{DynSolValue, JsonAbiExt, Specifier};
use alloy_json_abi::{Constructor, JsonAbi};
use alloy_network::{AnyNetwork, EthereumWallet, TransactionBuilder};
use alloy_primitives::{Address, Bytes, hex};
use alloy_provider::{PendingTransactionError, Provider, ProviderBuilder};
use alloy_rpc_types::{AnyTransactionReceipt, TransactionRequest};
use alloy_serde::WithOtherFields;
use alloy_signer::Signer;
use alloy_transport::{Transport, TransportError};
use clap::{Parser, ValueHint};
use eyre::{Context, Result};
use forge_verify::RetryArgs;
use forge_verify::VerifyArgs;
use foundry_cli::utils::did_you_mean;
use foundry_cli::{
    opts::{CoreBuildArgs, EthereumOpts, EtherscanOpts, RpcOpts, TransactionOpts},
    utils::{self, LoadConfig, read_constructor_args_file, remove_contract},
};
use foundry_common::{
    compile::{self},
    fmt::parse_tokens,
};
use foundry_compilers::{ArtifactId, Project};
use foundry_compilers::{artifacts::BytecodeObject, info::ContractInfo, utils::canonicalize};
use std::path::Path;
use std::str::FromStr;
// use alloy_json_abi::JsonAbi;
// use alloy_primitives::Address;
// use eyre::{eyre::Result, WrapErr};
use foundry_common::{TestFunctionExt, fs};
use foundry_compilers::{
    Artifact, ProjectCompileOutput,
    artifacts::{CompactBytecode, CompactDeployedBytecode, Settings},
    cache::{CacheEntry, CompilerCache},
    utils::read_json_file,
};
use foundry_config::{
    Config,
    figment::{
        self, Metadata, Profile,
        value::{Dict, Map},
    },
    merge_impl_figment_convert,
};
use std::{borrow::Borrow, marker::PhantomData, path::PathBuf, sync::Arc};

use crate::interface::{ZkayBlockchainInterface, ZkayProverInterface};
use my_logging::{log_context::log_context, logger::data};
use privacy::library_contracts;
use serde_json::{Map as JsonMap, Value as JsonValue, json};
use solidity::compiler::compile_solidity_json;
use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use zkay_config::{
    config::{CFG, library_compilation_environment, zk_print_banner},
    config_user::UserConfig,
    with_context_block, zk_print,
};
use zkay_transaction_crypto_params::params::CryptoParams;
// , IntegrityError, BlockChainError,
//     TransactionFailedException
use super::web3::{Web3, Web3Tx};
use crate::{
    arc_cell_new,
    types::{
        ARcCell, AddressValue, BlockStruct, DataType, MsgStruct, PublicKeyValue, TxStruct, Value,
    },
};
use ast_builder::process_ast::get_verification_contract_names;
use enum_dispatch::enum_dispatch;
use rccell::RcCell;
use zkay_ast::global_defs::{
    GlobalDefs, GlobalVars, array_length_member, global_defs, global_vars,
};
use zkay_utils::helpers::{get_contract_names, save_to_file};
use zkay_utils::timer::time_measure;

/// `ContractFactory` is a [`DeploymentTxFactory`] object with an
/// [`Arc`] middleware. This type alias exists to preserve backwards
/// compatibility with less-abstract Contracts.
///
/// For full usage docs, see [`DeploymentTxFactory`].
pub type ContractFactory<P, T> = DeploymentTxFactory<Arc<P>, P, T>;

/// Helper which manages the deployment transaction of a smart contract. It
/// wraps a deployment transaction, and retrieves the contract address output
/// by it.
///
/// Currently, we recommend using the [`ContractDeployer`] type alias.
#[derive(Debug)]
#[must_use = "ContractDeploymentTx does nothing unless you `send` it"]
pub struct ContractDeploymentTx<B, P, T, C> {
    /// the actual deployer, exposed for overriding the defaults
    pub deployer: Deployer<B, P, T>,
    /// marker for the `Contract` type to create afterwards
    ///
    /// this type will be used to construct it via `From::from(Contract)`
    _contract: PhantomData<C>,
}

impl<B, P, T, C> Clone for ContractDeploymentTx<B, P, T, C>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            deployer: self.deployer.clone(),
            _contract: self._contract,
        }
    }
}

impl<B, P, T, C> From<Deployer<B, P, T>> for ContractDeploymentTx<B, P, T, C> {
    fn from(deployer: Deployer<B, P, T>) -> Self {
        Self {
            deployer,
            _contract: PhantomData,
        }
    }
}

/// Helper which manages the deployment transaction of a smart contract
#[derive(Debug)]
#[must_use = "Deployer does nothing unless you `send` it"]
pub struct Deployer<B, P, T> {
    /// The deployer's transaction, exposed for overriding the defaults
    pub tx: WithOtherFields<TransactionRequest>,
    abi: JsonAbi,
    client: B,
    confs: usize,
    timeout: u64,
    _p: PhantomData<P>,
    _t: PhantomData<T>,
}

impl<B, P, T> Clone for Deployer<B, P, T>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            abi: self.abi.clone(),
            client: self.client.clone(),
            confs: self.confs,
            timeout: self.timeout,
            _p: PhantomData,
            _t: PhantomData,
        }
    }
}

impl<B, P, T> Deployer<B, P, T>
where
    B: Borrow<P> + Clone,
    P: Provider<T, AnyNetwork>,
    T: Transport + Clone,
{
    /// Broadcasts the contract deployment transaction and after waiting for it to
    /// be sufficiently confirmed (default: 1), it returns a tuple with
    /// the [`Contract`](crate::Contract) struct at the deployed contract's address
    /// and the corresponding [`AnyReceipt`].
    pub async fn send_with_receipt(
        self,
    ) -> eyre::Result<(Address, AnyTransactionReceipt), ContractDeploymentError> {
        let receipt = self
            .client
            .borrow()
            .send_transaction(self.tx)
            .await?
            .with_required_confirmations(self.confs as u64)
            .get_receipt()
            .await?;

        let address = receipt
            .contract_address
            .ok_or(ContractDeploymentError::ContractNotDeployed)?;

        Ok((address, receipt))
    }
}

/// To deploy a contract to the Ethereum network, a `ContractFactory` can be
/// created which manages the Contract bytecode and Application Binary Interface
/// (ABI), usually generated from the Solidity compiler.
///
/// Once the factory's deployment transaction is mined with sufficient confirmations,
/// the [`Contract`](crate::Contract) object is returned.
///
/// # Example
///
/// ```
/// # fn foo() -> eyre::Result<(), Box<dyn std::error::Error>> {
/// use alloy_primitives::Bytes;
/// use ethers_contract::ContractFactory;
/// use ethers_providers::{Provider, Http};
///
/// // get the contract ABI and bytecode
/// let abi = Default::default();
/// let bytecode = Bytes::from_static(b"...");
///
/// // connect to the network
/// let client = Provider::<Http>::try_from("http://localhost:8545").unwrap();
/// let client = std::sync::Arc::new(client);
///
/// // create a factory which will be used to deploy instances of the contract
/// let factory = ContractFactory::new(abi, bytecode, client);
///
/// // The deployer created by the `deploy` call exposes a builder which gets consumed
/// // by the `send` call
/// let contract = factory
///     .deploy("initial value".to_string())?
///     .confirmations(0usize)
///     .send()
///     .await?;
/// println!("{}", contract.address());
/// # Ok(())
/// # }
#[derive(Debug)]
pub struct DeploymentTxFactory<B, P, T> {
    client: B,
    abi: JsonAbi,
    bytecode: Bytes,
    timeout: u64,
    _p: PhantomData<P>,
    _t: PhantomData<T>,
}

impl<B, P, T> Clone for DeploymentTxFactory<B, P, T>
where
    B: Clone,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            abi: self.abi.clone(),
            bytecode: self.bytecode.clone(),
            timeout: self.timeout,
            _p: PhantomData,
            _t: PhantomData,
        }
    }
}

impl<P, T, B> DeploymentTxFactory<B, P, T>
where
    B: Borrow<P> + Clone,
    P: Provider<T, AnyNetwork>,
    T: Transport + Clone,
{
    /// Creates a factory for deployment of the Contract with bytecode, and the
    /// constructor defined in the abi. The client will be used to send any deployment
    /// transaction.
    pub fn new(abi: JsonAbi, bytecode: Bytes, client: B, timeout: u64) -> Self {
        Self {
            client,
            abi,
            bytecode,
            timeout,
            _p: PhantomData,
            _t: PhantomData,
        }
    }

    /// Create a deployment tx using the provided tokens as constructor
    /// arguments
    pub fn deploy_tokens(
        self,
        params: Vec<DynSolValue>,
    ) -> eyre::Result<Deployer<B, P, T>, ContractDeploymentError>
    where
        B: Clone,
    {
        // Encode the constructor args & concatenate with the bytecode if necessary
        let data: Bytes = match (self.abi.constructor(), params.is_empty()) {
            (None, false) => return Err(ContractDeploymentError::ConstructorError),
            (None, true) => self.bytecode.clone(),
            (Some(constructor), _) => {
                let input: Bytes = constructor
                    .abi_encode_input(&params)
                    .map_err(ContractDeploymentError::DetokenizationError)?
                    .into();
                // Concatenate the bytecode and abi-encoded constructor call.
                self.bytecode.iter().copied().chain(input).collect()
            }
        };

        // create the tx object. Since we're deploying a contract, `to` is `None`
        let tx = WithOtherFields::new(TransactionRequest::default().input(data.into()));

        Ok(Deployer {
            client: self.client.clone(),
            abi: self.abi,
            tx,
            confs: 1,
            timeout: self.timeout,
            _p: PhantomData,
            _t: PhantomData,
        })
    }
}

#[derive(thiserror::Error, Debug)]
/// An Error which is thrown when interacting with a smart contract
pub enum ContractDeploymentError {
    #[error("constructor is not defined in the ABI")]
    ConstructorError,
    #[error(transparent)]
    DetokenizationError(#[from] alloy_dyn_abi::Error),
    #[error("contract was not deployed")]
    ContractNotDeployed,
    #[error(transparent)]
    RpcError(#[from] TransportError),
}

impl From<PendingTransactionError> for ContractDeploymentError {
    fn from(_err: PendingTransactionError) -> Self {
        Self::ContractNotDeployed
    }
}

// max_gas_limit = 10000000
// #[enum_dispatch]
// pub trait Web3Blockchain {
//     fn _create_w3_instance(&self);
// }

#[derive(Clone)]
pub struct Web3BlockchainBase<
    P: ZkayProverInterface + std::marker::Send,
    W: std::marker::Send + std::marker::Sync,
> {
    _lib_addresses: ARcCell<Option<BTreeMap<String, Address>>>,
    _pki_contract: ARcCell<Option<BTreeMap<String, Address>>>,
    prover: ARcCell<P>,
    next_acc_idx: ARcCell<i32>,
    web3tx: Web3Tx,
    _web3: PhantomData<W>,
}
impl<P: ZkayProverInterface + std::marker::Send, W: std::marker::Send + std::marker::Sync>
    Web3BlockchainBase<P, W>
{
    pub fn new(prover: ARcCell<P>, web3tx: Web3Tx) -> Self {
        Self {
            prover,
            _lib_addresses: arc_cell_new!(None),
            _pki_contract: arc_cell_new!(None),
            next_acc_idx: arc_cell_new!(0),
            web3tx,
            _web3: PhantomData,
        }
    }
    fn next_acc_idx(&self) -> ARcCell<i32> {
        self.next_acc_idx.clone()
    }
}

// #[samotop_async_trait::async_trait]
impl<
    P: ZkayProverInterface + std::marker::Send + std::marker::Sync,
    W: std::marker::Send + std::marker::Sync,
> ZkayBlockchainInterface<P> for Web3BlockchainBase<P, W>
{
    fn prover(&self) -> ARcCell<P> {
        self.prover.clone()
    }
    fn _pki_contract(&self) -> ARcCell<Option<BTreeMap<String, Address>>> {
        self._pki_contract.clone()
    }
    async fn default_address(&self) -> Option<Value<String, AddressValue>> {
        let address = self._default_address().await;
        address.map(|addr| Value::<String, AddressValue>::new(vec![addr], None, None))
    }
    async fn create_test_accounts(&self, count: i32) -> Vec<String> {
        let accounts: Vec<String> = Web3::default().eth_accounts().await; //"self.w3.eth.accounts";
        let next_acc_idx = *self.next_acc_idx().lock() as usize;
        assert!(
            accounts[next_acc_idx..].len() >= count as usize,
            "Can have at most {} dummy accounts in total",
            accounts.len() - 1
        );
        let dummy_accounts = accounts[next_acc_idx..next_acc_idx + count as usize].to_vec();
        *self.next_acc_idx().lock() += count;
        dummy_accounts
    }
    async fn deploy_solidity_contract(
        &self,
        sol_filename: &str,
        contract_name: Option<String>,
        sender: &Address,
    ) -> eyre::Result<Address> {
        let contract_name = if let Some(contract_name) = contract_name {
            contract_name
        } else {
            get_contract_names(sol_filename)[0].clone()
        };
        let (abi, bin, id) = self.compile_contract(
            &PathBuf::from(sol_filename),
            &contract_name,
            None,
            &PathBuf::from("."),
        )?;
        println!("========_deploy_contract=====1======");
        let contract = self
            ._deploy_contract(sender, abi.clone(), &vec![], None, abi, bin, id)
            .await;
        println!("========_deploy_contract===========");
        contract
    }
    // class Web3Blockchain(ZkayBlockchainInterface):
    //     fn __init__(self) -> None:
    //         super().__init__()
    //         self.w3 = self._create_w3_instance()
    //         if not self.w3.is_connected():
    //             raise BlockChainError(f"Failed to connect to blockchain: {self.w3.provider}")

    //     @staticmethod
    async fn get_special_variables(
        &self,
        sender: &String,
        wei_amount: i32,
    ) -> (MsgStruct, BlockStruct, TxStruct) {
        let timestamp = Web3::default().get_block("pending", "timestamp").await;
        zk_print!("Current block timestamp:{timestamp} ");
        (
            MsgStruct::new(sender.clone(), wei_amount),
            BlockStruct::new(
                sender.clone(),
                Web3::default()
                    .get_block("pending", "difficulty")
                    .await
                    .parse::<i32>()
                    .unwrap(),
                Web3::default()
                    .get_block("pending", "gasLimit")
                    .await
                    .parse::<i32>()
                    .unwrap(),
                Web3::default()
                    .get_block("pending", "number")
                    .await
                    .parse::<i32>()
                    .unwrap(),
                timestamp.parse::<i32>().unwrap(),
            ),
            TxStruct::new(
                Web3::default().gas_price().await.parse::<i32>().unwrap(),
                sender.clone(),
            ),
        )
    }
    async fn _default_address(&self) -> Option<String> {
        let default_account = CFG.lock().unwrap().blockchain_default_account();
        if let Some(acc) = default_account {
            if let Ok(i) = acc.parse::<i32>() {
                Web3::default()
                    .eth_accounts()
                    .await
                    .get(i as usize)
                    .cloned()
            } else {
                Some(acc)
            }
        } else {
            None
        }

        // elif isinstance(cfg.blockchain_default_account, int):
        //     return self.w3.eth.accounts[cfg.blockchain_default_account]
    }
    async fn _get_balance(&self, address: &str) -> String {
        // self.w3.eth.get_balance(address)
        Web3::default().get_balance(address).await
    }

    async fn _req_public_key(
        &self,
        address: &str,
        crypto_params: &CryptoParams,
    ) -> eyre::Result<Value<String, PublicKeyValue>> {
        let pki_contract = self.pki_contract(&crypto_params.crypto_name).await?;
        let len = if "elgamal" == &crypto_params.crypto_name {
            2
        } else {
            1
        };
        println!("========pki_contract,address=================={pki_contract},{address}");
        let v = self
            ._req_state_var(
                &pki_contract,
                &format!("getPk(address a) public view returns(uint[{len}] memory)"),
                vec![address.to_owned()],
            )
            .await?;

        println!("==v====before======{v:?}===================");
        let v: Vec<JsonValue> = serde_json::from_str(v.trim_matches('"')).unwrap();
        println!("==v=========={v:?}===================");
        let v: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
        println!("==v====s======{v:?}===================");
        Ok(Value::<String, PublicKeyValue>::new(
            v,
            Some(crypto_params.clone()),
            None,
        ))
    }

    async fn _announce_public_key(
        &self,
        address: &str,
        pk: &Value<String, PublicKeyValue>,
        crypto_params: &CryptoParams,
    ) -> eyre::Result<String> {
        //         with log_context(f"announcePk"):
        let pki_contract = self.pki_contract(&crypto_params.crypto_name).await?;
        let len = if "elgamal" == &crypto_params.crypto_name {
            2
        } else {
            1
        };
        println!(
            "=*****************=_announce_public_key====={pki_contract}======{len}=========={pk:?}=====pk==={}======",
            pk.to_string()
        );
        self._transact(
            &pki_contract,
            &Address::from_str(address)
                .expect(&format!("==UNEXPECT===address==============={address}")),
            &format!("announcePk(uint[{len}] calldata pk)"),
            &vec![DataType::PublicKeyValue(pk.clone())],
            None,
        )
        .await
    }

    async fn _req_state_var(
        &self,
        contract_handle: &Address,
        name: &str,
        indices: Vec<String>,
    ) -> eyre::Result<String> {
        //         try:
        // contract_handle.functions[name](indices).call()
        //         except Exception as e:
        //             raise BlockChainError(e.args)
        // futures::executor::block_on()
        println!(
            "==_req_state_var==={contract_handle}========={name}==========={indices:?}============"
        );
        let res = self
            .web3tx
            .call(
                Some(contract_handle.clone().into()),
                Some(name.to_owned()),
                indices,
            )
            .await;
        println!("==_req_state_var==res={res:?}=================");
        res.as_ref()
            .map(|re| {
                serde_json::from_str::<Vec<String>>(re)
                    .map(|r| r[0].clone())
                    .map_err(|e| eyre::eyre!("{e:?}"))
            })
            .map_err(|e| eyre::eyre!("{e:?}"))?
    }
    async fn _call(
        &self,
        contract_handle: &Address,
        _sender: &Address,
        name: &str,
        args: &Vec<DataType>,
    ) -> eyre::Result<String> {
        //         try:
        // let fct = contract_handle.functions[name];
        // let gas_amount = self._gas_heuristic(sender, fct(args));
        // let tx = json!({"from": sender, "gas": gas_amount});
        // fct(args).call(tx)
        //         except Exception as e:
        //             raise BlockChainError(e.args)
        self.web3tx
            .call(
                Some(contract_handle.clone().into()),
                Some(name.to_owned()),
                args.iter().map(|x| x.to_string()).collect(),
            )
            .await
    }
    async fn _transact(
        &self,
        contract_handle: &Address,
        _sender: &Address,
        function: &str,
        actual_args: &Vec<DataType>,
        _wei_amount: Option<i32>,
    ) -> Result<String> {
        // use ff::*;
        println!("====_transact=============before=actual_args=={actual_args:?}=======");
        // use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
        // use ark_ff::BigInteger256;
        // use ark_ff::BigInteger;
        use alloy_primitives::U256;
        // let actual_args:Vec<_>=actual_args.iter().map(|x| {let x= BigInteger256::from_str(&x.to_string())
        //             .map_or_else(|_|x.to_string(),|x|x.to_bytes_be().into_iter().map(|b| format!("{:02x}",b)).collect::<Vec<_>>().concat());println!("=x==*****==={}",x);format!("[{x}]")}).collect();
        let actual_args: Vec<_> = actual_args
            .iter()
            .map(|x| {
                format!(
                    "[{}]",
                    U256::from_str(&("0x".to_string() + &x.to_string()))
                        .map_or_else(|_| x.to_string(), |v| v.to_string())
                )
            })
            .collect();
        println!("====_transact=======actual_args========={actual_args:?}=======");
        let res = self
            .web3tx
            .send(
                Some(contract_handle.clone().into()),
                Some(function.to_owned()),
                actual_args,
            )
            .await;
        println!("====_transact=======res======{res:?}==========");
        res
        // let fct = if function == "constructor" {
        //     contract_handle.constructor
        // } else {
        //     contract_handle.functions[function]
        // };
        // let gas_amount = self._gas_heuristic(sender, fct(*actual_params));
        // let mut tx = json!({"from": sender, "gas": gas_amount});
        // if wei_amount.is_some() {
        //     tx["value"] = wei_amount;
        // }
        // let tx_hash = fct(*actual_params).transact(tx);
        // let tx_receipt = self.w3.eth.wait_for_transaction_receipt(tx_hash);
        // // except Exception as e:
        // //     raise BlockChainError(e.args)

        // if tx_receipt["status"] == 0 {
        //     eyre::bail!("Transaction failed")
        // }
        // let gas = tx_receipt["gasUsed"];
        // zk_print!("Consumed gas: {gas}");
        // data("gas", gas);
        // tx_receipt
    }

    async fn _deploy(
        &self,
        project_dir: &PathBuf,
        sender: &Address,
        contract: &str,
        actual_args: Vec<String>,
        wei_amount: Option<i32>,
    ) -> eyre::Result<Address> {
        println!("======_deploy=========begin============");
        let mut project_dir = project_dir.clone();
        let mut project_dir_parent = project_dir.clone();
        project_dir_parent.pop();
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        // with open(os.path.join(project_dir, "contract.zkay")) as f:
        let f = project_dir.join("contract.zkay");
        assert!(f.try_exists().map_or(false, |b| b), "{f:?}");
        let verifier_names =
            get_verification_contract_names((std::fs::read_to_string(f).ok(), None), global_vars);
        println!("====verifier_names===================={verifier_names:?}");
        // Deploy verification contracts if not already done
        let external_contract_addresses = self
            ._deploy_dependencies(sender, &project_dir, verifier_names)
            .await
            .expect("===_deploy_dependencies===");
        let inst_target_path = std::env::temp_dir().join(
            project_dir
                .iter()
                .nth_back(1)
                .map_or("_inst".to_owned(), |s| {
                    s.to_string_lossy().to_string() + "_inst"
                }),
        );
        std::fs::create_dir_all(inst_target_path.clone()).expect("===create_dir_all===");
        // let project_dir_parent=project_dir.parent().unwrap();
        // println!(
        //     "==project_dir========{}========={}",
        //     project_dir.display(),
        //     inst_target_path.display()
        // );
        walkdir::WalkDir::new(project_dir.clone())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            // .filter_entry(|e| e
            //  .file_name()
            //  .to_str()
            //  .map_or(false,|s| {println!("{}",s);e.depth()==0 || e.file_type().is_file() || !s.starts_with(".") || !s.starts_with("..") }))
            .filter_map(eyre::Result::ok)
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |s| s != "contract.sol" && s.ends_with(".sol"))
            })
            .for_each(|x| {
                let _ = std::fs::copy(x.path(), inst_target_path.join(x.file_name()));
            });
        let inst_config = Config::load_with_root(inst_target_path).sanitized();
        let _inst_project = inst_config.project().expect("===project===");
        // let cout;
        let (abi, bin, id);
        with_context_block!(var filename= self.__hardcoded_external_contracts_ctx(&project_dir,&project_dir, &external_contract_addresses) =>{
            println!("==={filename:?}======project_dir====={project_dir:?}=============");
            (abi, bin, id) = self.compile_contract(&PathBuf::from(filename), contract, None,&project_dir).expect("===compile_contract===");
        });
        let handle;
        with_context_block!(var _a= log_context("constructor")=>{
                   with_context_block!(var _b= log_context(&format!("{contract}"))=>{
        println!("========_deploy_contract====3=======");
                       handle = self._deploy_contract(sender, abi.clone(), &actual_args, wei_amount,abi, bin, id).await;
                   });
               });
        zk_print!(r#"Deployed contract "{contract}" at address "{handle:?}""#);
        handle
    }

    async fn _deploy_dependencies(
        &self,
        sender: &Address,
        project_dir: &PathBuf,
        verifier_names: Vec<String>,
    ) -> eyre::Result<BTreeMap<String, Address>> {
        println!("========_deploy_dependencies======before=====");
        // # Deploy verification contracts if not already done
        let mut vf = BTreeMap::new();
        let lib_addresses = self.lib_addresses().await.ok();
        for verifier_name in verifier_names {
            with_context_block!(var _a= log_context("constructor")=>{
                            with_context_block!(var _b=log_context(&format!("{verifier_name}"))=>{
                                let filename = project_dir.join( &format!("{verifier_name}.sol"));
                                let (abi, bin, id) = self.compile_contract(&filename, &verifier_name, lib_addresses.as_ref().and_then(|la|la.lock().as_ref().cloned()),&PathBuf::from(".")).expect("=======compile_contract======_deploy_dependencies=====");
                                with_context_block!(var _tm= time_measure("transaction_full",false,false)=>{
            println!("========_deploy_contract======2=====");
                                    vf.insert(verifier_name.clone(),self._deploy_contract(sender, abi.clone(),&vec![],None,abi, bin, id).await.unwrap());
                                });
                        });
                        });
        }
        println!("========_deploy_contract=====7=====");
        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        for crypto_params in all_crypto_params {
            let pki_contract_name = CFG
                .lock()
                .unwrap()
                .get_pki_contract_name(&CryptoParams::new(crypto_params.clone()).identifier_name());
            let pki_contract_address = self
                .pki_contract(&crypto_params)
                .await
                .expect("========pki_contract=============");
            vf.insert(pki_contract_name, pki_contract_address.into());
        }
        println!("========_deploy_contract====vf==={vf:?}===");
        Ok(vf)
    }

    async fn _connect_libraries(&self) -> eyre::Result<BTreeMap<String, Address>> {
        // println!("======_connect_libraries==============");
        assert!(
            !CFG.lock().unwrap().blockchain_pki_address().is_empty(),
            "Must specify pki address in config."
        );

        let mut lib_addresses = vec![];
        if !CFG.lock().unwrap().external_crypto_lib_names().is_empty() {
            if !CFG
                .lock()
                .unwrap()
                .blockchain_crypto_lib_addresses()
                .is_empty()
            {
                lib_addresses = CFG
                    .lock()
                    .unwrap()
                    .blockchain_crypto_lib_addresses()
                    .split(",")
                    .map(|addr| addr.trim().to_owned())
                    .collect();
            }

            assert!(
                lib_addresses.len() == CFG.lock().unwrap().external_crypto_lib_names().len(),
                "Must specify all crypto library addresses in config\nExpected {} was {}",
                CFG.lock().unwrap().external_crypto_lib_names().len(),
                lib_addresses.len()
            );
        }
        // with_context_block!(var _lce=library_compilation_environment()=>{
        let tmpdir = std::env::temp_dir();
        let mut _pki_contract = BTreeMap::new();
        let blockchain_pki_address = CFG.lock().unwrap().blockchain_pki_address();

        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        assert!(
            blockchain_pki_address.len() == all_crypto_params.len(),
            "Must specify all pki addresses in config\nExpected {} was {}",
            all_crypto_params.len(),
            blockchain_pki_address.len()
        );
        for (crypto_params, pki_address) in all_crypto_params.iter().zip(&blockchain_pki_address) {
            let crypto_param = CryptoParams::new(crypto_params.clone());
            let pki_contract_code = library_contracts::get_pki_contract(&crypto_param);
            let pki_contract_name = CFG
                .lock()
                .unwrap()
                .get_pki_contract_name(&crypto_param.identifier_name());
            let pki_sol = save_to_file(
                Some(tmpdir.clone()),
                &format!("{pki_contract_name}.sol"),
                &pki_contract_code,
            );
            let address = Address::from_str(pki_address).unwrap();
            let _ = self
                ._verify_contract_integrity(
                    &address,
                    &PathBuf::from(pki_sol),
                    None,
                    Some(pki_contract_name),
                    false,
                    None,
                )
                .await
                .expect("_verify_contract_integrity failed");
            _pki_contract.insert(crypto_params.clone(), address);
        }
        *self._pki_contract.lock() = Some(_pki_contract);
        let verify_sol = save_to_file(
            Some(tmpdir),
            "verify_libs.sol",
            &library_contracts::get_verify_libs_code(),
        );
        // let mut _lib_addresses = BTreeMap::new();
        for (lib, addr) in CFG
            .lock()
            .unwrap()
            .external_crypto_lib_names()
            .iter()
            .zip(&lib_addresses)
        {
            let _out = self
                ._verify_contract_integrity(
                    &Address::from_str(addr).unwrap(),
                    &PathBuf::from(verify_sol.clone()),
                    None,
                    Some(lib.clone()),
                    true,
                    None,
                )
                .await?;
            self._lib_addresses
                .lock()
                .as_mut()
                .unwrap()
                .insert(lib.clone(), Address::from_str(addr).unwrap());
        }
        // self._lib_addresses = _lib_addresses;
        // });
        self._lib_addresses.lock().clone().ok_or(eyre::eyre!(""))
    }

    fn _connect(
        &self,
        _project_dir: &str,
        _contract: &str,
        address: Address,
    ) -> eyre::Result<Address> {
        //(JsonAbi, CompactBytecode, ArtifactId)
        // let filename = PathBuf::from(project_dir).join("contract.sol");
        // let (abi, bin, id) =
        //     self.compile_contract(&filename, contract, None, &PathBuf::from("."), project)?;
        // //  self.w3.eth.contract(
        // //     address=address, abi=cout["abi"]
        // // )
        // Ok((abi, bin, id))
        Ok(address)
    }

    async fn _verify_contract_integrity(
        &self,
        mut address: &Address,
        sol_filename: &PathBuf,
        libraries: Option<&BTreeMap<String, Address>>,
        mut contract_name: Option<String>,
        is_library: bool,
        cwd: Option<PathBuf>,
    ) -> eyre::Result<Address> {
        // if is_instance(address, bytes)
        //     {address = self.w3.toChecksumAddress(address);}

        let contract_name = if let Some(contract_name) = contract_name {
            contract_name
        } else {
            get_contract_names(&sol_filename.to_string_lossy().to_string())[0].clone()
        };

        let actual_byte_code = self.__normalized_hex_str(Web3::default().get_code(address).await);
        assert!(
            !actual_byte_code.is_empty(),
            "Expected contract {} is not deployed at address {address}",
            contract_name
        );

        let (_abi, _bin, deployed_bin, _id) = self.compile_contract_ex(
            sol_filename,
            &contract_name,
            libraries,
            cwd.as_ref().unwrap_or(&PathBuf::from(".")),
        )?;
        let bytes = deployed_bin.bytecode.unwrap().object.into_bytes();
        let bytes: Vec<u8> = bytes.unwrap().into();
        let mut expected_byte_code = self.__normalized_hex(bytes);

        if is_library {
            // # https://github.com/ethereum/solidity/issues/7101
            expected_byte_code = expected_byte_code[..2].to_owned()
                + &self.__normalized_hex_str(address.to_string())
                + &expected_byte_code[42..]
        }

        assert!(
            actual_byte_code == expected_byte_code,
            "Deployed contract at address {address} does not match local contract {sol_filename:?}==={actual_byte_code} == {expected_byte_code}"
        );
        zk_print!(
            "Contract@{address} matches {}:{contract_name}",
            sol_filename.file_name().unwrap().to_str().unwrap()
        );

        // self.w3.eth.contract(
        //     address=address, abi=cout["abi"]
        // )
        Ok(address.clone())
    }
    async fn _verify_library_integrity(
        &self,
        libraries: BTreeMap<String, PathBuf>,
        contract_with_libs_addr: &String,
        sol_with_libs_filename: &PathBuf,
    ) -> eyre::Result<BTreeMap<String, Address>> {
        let cname =
            get_contract_names(&sol_with_libs_filename.to_string_lossy().to_string())[0].clone();
        let actual_code = self.__normalized_hex(
            Web3::default()
                .get_code(&Address::from_str(contract_with_libs_addr).unwrap())
                .await
                .into(),
        );
        assert!(
            !actual_code.is_empty(),
            "Expected contract {cname} is not deployed at address {contract_with_libs_addr}"
        );
        let (_abi, bin, _id) =
            self.compile_contract(sol_with_libs_filename, &cname, None, &PathBuf::from("."))?;
        let bytes = bin.object.into_bytes();
        let bytes: Vec<u8> = bytes.unwrap().into();
        let code_with_placeholders = self.__normalized_hex(bytes);

        assert!(
            actual_code.len() == code_with_placeholders.len(),
            "Local code of contract {cname} has different length than remote contract"
        );

        let mut addresses = BTreeMap::new();
        for (lib_name, lib_sol) in libraries {
            // # Compute placeholder according to
            // # https://solidity.readthedocs.io/en/v0.5.13/using-the-compiler.html#using-the-commandline-compiler
            let hash = Web3::default()
                .solidity_keccak(
                    format!(
                        "string {}:{lib_name}",
                        lib_sol.file_name().unwrap().to_string_lossy().to_string()
                    )
                    .into_bytes(),
                )
                .await;

            let placeholder = format!("__${}$__", &self.__normalized_hex(hash.into())[..34]);

            // # Retrieve concrete address in deployed code at placeholder offset in local code and verify library contract integrity
            let lib_address_offset = code_with_placeholders.find(&placeholder);
            if let Some(_lib_address_offset) = lib_address_offset {
                let lib_address: Address = Address::default(); // self
                // .w3
                // .toChecksumAddress(actual_code[lib_address_offset..lib_address_offset + 40]);
                // with cfg.library_compilation_environment():
                let _ = self._verify_contract_integrity(
                    &lib_address,
                    &lib_sol,
                    None,
                    Some(lib_name.clone()),
                    true,
                    None,
                );
                addresses.insert(lib_name.clone(), lib_address.clone());
            }
        }
        Ok(addresses)
    }
    fn _verify_zkay_contract_integrity(
        &self,
        address: &Address,
        project_dir: &PathBuf,
        pki_verifier_addresses: &BTreeMap<String, Address>,
    ) {
        let sol_file = self.__hardcoded_external_contracts_ctx(
            project_dir,
            project_dir,
            &pki_verifier_addresses,
        );
        let _ = self._verify_contract_integrity(
            address,
            &PathBuf::from(sol_file),
            None,
            None,
            false,
            Some(project_dir.to_owned()),
        );
    }

    //     @contextmanager
    async fn lib_addresses(&self) -> eyre::Result<ARcCell<Option<BTreeMap<String, Address>>>> {
        if self._lib_addresses.lock().is_none() {
            let _ = self._connect_libraries().await?; //.expect("==_connect_libraries=====in========lib_addresses=========");
        }
        Ok(self._lib_addresses.clone())
    }
}

impl<P: ZkayProverInterface + std::marker::Send, W: std::marker::Send + std::marker::Sync>
    Web3BlockchainBase<P, W>
{
    pub fn compile_contract(
        &self,
        sol_filename: &PathBuf,
        contract_name: &str,
        _libs: Option<BTreeMap<String, Address>>,
        _cwd: &PathBuf,
    ) -> eyre::Result<(JsonAbi, CompactBytecode, ArtifactId)> {
        // let solp = sol_filename; //std::path::PathBuf::from(sol_filename);
        // let jout = compile_solidity_json(
        //     &sol_filename.to_string_lossy().to_string(),
        //     libs.map(|s| {
        //         s.into_iter()
        //             .map(|(k, v)| (k.clone(), v.to_string()))
        //             .collect()
        //     }),
        //     CFG.lock().unwrap().opt_solc_optimizer_runs(),
        //     vec![],
        //     cwd,
        // )
        // .unwrap();
        // let jout = &jout["contracts"][solp.file_name().unwrap().to_str().unwrap()][&contract_name];
        // json!({
        //     "abi": jout["metadata"]["output"]["abi"],
        //     "bin": jout["evm"]["bytecode"]["object"],
        //     "deployed_bin": jout["evm"]["deployedBytecode"]["object"],
        // })
        let project = self.web3tx.config.project()?;
        let mut output = compile::compile_target(&sol_filename, &project, false)?;
        remove_contract(output, &sol_filename, contract_name)
    }
    pub fn compile_contract_ex(
        &self,
        sol_filename: &PathBuf,
        contract_name: &str,
        _libs: Option<&BTreeMap<String, Address>>,
        _cwd: &PathBuf,
    ) -> eyre::Result<(
        JsonAbi,
        CompactBytecode,
        CompactDeployedBytecode,
        ArtifactId,
    )> {
        // let solp = sol_filename; //std::path::PathBuf::from(sol_filename);
        // let jout = compile_solidity_json(
        //     &sol_filename.to_string_lossy().to_string(),
        //     libs.map(|s| {
        //         s.into_iter()
        //             .map(|(k, v)| (k.clone(), v.to_string()))
        //             .collect()
        //     }),
        //     CFG.lock().unwrap().opt_solc_optimizer_runs(),
        //     vec![],
        //     cwd,
        // )
        // .unwrap();
        // let jout = &jout["contracts"][solp.file_name().unwrap().to_str().unwrap()][&contract_name];
        // json!({
        //     "abi": jout["metadata"]["output"]["abi"],
        //     "bin": jout["evm"]["bytecode"]["object"],
        //     "deployed_bin": jout["evm"]["deployedBytecode"]["object"],
        // })
        let project = self.web3tx.config.project()?;
        let mut output = compile::compile_target(&sol_filename, &project, false)?;
        remove_contract_ex(output, &sol_filename, contract_name)
    }
    async fn _deploy_contract(
        &self,
        _sender: &Address,
        _contract_interface: JsonAbi,
        args: &Vec<String>,
        _wei_amount: Option<i32>,
        abi: JsonAbi,
        bin: CompactBytecode,
        id: ArtifactId,
    ) -> eyre::Result<Address> {
        // let contract = self.w3.eth.contract(
        //     abi=contract_interface["abi"],
        //     bytecode=contract_interface["bin"]
        // );

        // let _tx_receipt = self._transact(
        //     &"contract".into(),
        //     sender,
        //     "constructor",
        //     &args
        //         .into_iter()
        //         .map(|s| DataType::String(s.clone()))
        //         .collect(),
        //     wei_amount,
        // );
        // // let contract = self.w3.eth.contract(
        // //     tx_receipt.contractAddress, contract_interface["abi"]
        // // );
        // //  contract
        // Ok(json!({}))
        let bin = match bin.object {
            BytecodeObject::Bytecode(_) => bin.object,
            _ => {
                let link_refs = bin
                    .link_references
                    .iter()
                    .flat_map(|(path, names)| {
                        names.keys().map(move |name| format!("\t{name}: {path}"))
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                eyre::bail!(
                    "Dynamic linking not supported in `create` command - deploy the following library contracts first, then provide the address to link at compile time\n{}",
                    link_refs
                )
            }
        };

        // Add arguments to constructor
        let params = if let Some(constructor) = &abi.constructor {
            // let constructor_args = self
            //     .constructor_args_path
            //     .clone()
            //     .map(read_constructor_args_file)
            //     .transpose()?;
            self.parse_constructor_args(constructor, args)?
        } else {
            vec![]
        };
        let config = Config::from(&self.web3tx.eth);

        let provider = utils::get_provider(&config)?;

        // respect chain, if set explicitly via cmd args
        let chain_id = provider.get_chain_id().await?;
        //  if let Some(chain_id) = self.chain_id() {
        //     chain_id
        // } else {
        //     provider.get_chain_id().await?
        // };
        // if self.unlocked {
        // Deploy with unlocked account
        let signer = self.web3tx.eth.wallet.signer().await?;
        let sender = signer.address();
        // self.web3tx.eth.wallet.from.expect("required");
        self.deploys(
            abi,
            bin,
            params,
            provider,
            chain_id,
            sender,
            config.transaction_timeout,
            id,
        )
        .await

        // } else {
        //     // Deploy with signer
        //     let signer = self.eth.wallet.signer().await?;
        //     let deployer = signer.address();
        //     let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
        //         .wallet(EthereumWallet::new(signer))
        //         .on_provider(provider);
        //     self.clone()
        //         .deploy(
        //             abi,
        //             bin,
        //             params,
        //             provider,
        //             chain_id,
        //             deployer,
        //             config.transaction_timeout,
        //             id,
        //         )
        //         .await?
        // }
    }

    fn __hardcoded_external_contracts_ctx(
        &self,
        contract_dir: &PathBuf,
        dest_contract_dir: &PathBuf,
        pki_verifier_addresses: &BTreeMap<String, Address>,
    ) -> PathBuf {
        // # Hardcode contract addresses
        //     use std::fs::OpenOptions;
        // let f=OpenOptions::new().read(true).open(output_filename);
        // with open(os.path.join(contract_dir, "contract.sol")) as f:
        // let _ = std::fs::copy(contract_dir.join("contract_original.sol"), contract_dir.join("contract.sol"));
        let mut c = std::fs::read_to_string(contract_dir.join("contract.sol")).unwrap();
        for (key, val) in pki_verifier_addresses {
            println!("===key==val======{key}==={val}====");
            c = c.replace(
                &format!("{key}(0)"),
                &format!("{key}({})", val.to_checksum(None)),
            );
        }

        // with tempfile.TemporaryDirectory() as tempd:
        // # Save in temporary file to compile
        let output_filename = dest_contract_dir.join("contract.inst.sol");
        // let f=OpenOptions::new().write(true).create(true).open(output_filename);
        let _ = std::fs::write(output_filename.clone(), c);
        // format!("{}", output_filename.display())
        // pass
        output_filename
    }
    fn __normalized_hex_str(&self, mut val: String) -> String {
        if let Some(v) = val.strip_prefix("0x") {
            val = v.to_owned();
        }
        val.make_ascii_lowercase();
        val
    }

    fn __normalized_hex(&self, val: Vec<u8>) -> String {
        let mut val = hex::encode(val);
        if let Some(v) = val.strip_prefix("0x") {
            val = v.to_owned();
        }
        val.make_ascii_lowercase();
        val
    }

    async fn _gas_heuristic(&self, to: &str, sig: &str, args: Vec<&str>) -> u128 {
        let limit = Web3::default()
            .get_block("latest", "gasLimit")
            .await
            .parse::<u128>()
            .unwrap();
        let estimate = Web3::default()
            .estimate_gas(to, sig, args)
            .await
            .parse::<u128>()
            .unwrap();
        limit.min((estimate as f64 * 1.2) as u128)
    }

    /// Deploys the contract
    #[allow(clippy::too_many_arguments)]
    async fn deploys<PR: Provider<T, AnyNetwork>, T: Transport + Clone>(
        &self,
        abi: JsonAbi,
        bin: BytecodeObject,
        args: Vec<DynSolValue>,
        provider: PR,
        chain: u64,
        deployer_address: Address,
        timeout: u64,
        _id: ArtifactId,
    ) -> eyre::Result<Address> {
        let bin = bin.into_bytes().unwrap_or_else(|| {
            panic!(
                "no bytecode found in bin object for {}",
                "self.contract.name"
            )
        });
        let provider = Arc::new(provider);
        let factory = ContractFactory::new(abi.clone(), bin.clone(), provider.clone(), timeout);

        let is_args_empty = args.is_empty();
        let mut deployer =
            factory.deploy_tokens(args.clone()).context("failed to deploy contract").map_err(|e| {
                if is_args_empty {
                    e.wrap_err("no arguments provided for contract constructor; consider --constructor-args or --constructor-args-path")
                } else {
                    e
                }
            })?;
        let is_legacy = Chain::from(chain).is_legacy(); //self.tx.legacy ||

        deployer.tx.set_from(deployer_address);
        deployer.tx.set_chain_id(chain);
        // `to` field must be set explicitly, cannot be None.
        if deployer.tx.to.is_none() {
            deployer.tx.set_create();
        }
        // deployer.tx.set_nonce(if let Some(nonce) = self.tx.nonce {
        //     Ok(nonce.to())
        // } else {
        //     provider.get_transaction_count(deployer_address).await
        // }?);
        deployer
            .tx
            .set_nonce(provider.get_transaction_count(deployer_address).await?);
        // set tx value if specified
        // if let Some(value) = self.tx.value {
        //     deployer.tx.set_value(value);
        // }

        // deployer
        //     .tx
        //     .set_gas_limit(if let Some(gas_limit) = self.tx.gas_limit {
        //         Ok(gas_limit.to())
        //     } else {
        //         provider.estimate_gas(&deployer.tx).await
        //     }?);
        deployer
            .tx
            .set_gas_limit(provider.estimate_gas(&deployer.tx).await?);
        if is_legacy {
            let gas_price = provider.get_gas_price().await?;
            //  if let Some(gas_price) = self.tx.gas_price {
            //     gas_price.to()
            // } else {
            //     provider.get_gas_price().await?
            // };
            deployer.tx.set_gas_price(gas_price);
        } else {
            let estimate = provider.estimate_eip1559_fees(None).await.wrap_err("Failed to estimate EIP1559 fees. This chain might not support EIP1559, try adding --legacy to your command.")?;
            let priority_fee = estimate.max_priority_fee_per_gas;
            //  if let Some(priority_fee) = self.tx.priority_gas_price {
            //     priority_fee.to()
            // } else {
            //     estimate.max_priority_fee_per_gas
            // };
            let max_fee = estimate.max_fee_per_gas;
            // if let Some(max_fee) = self.tx.gas_price {
            //     max_fee.to()
            // } else {
            //     estimate.max_fee_per_gas
            // };

            deployer.tx.set_max_fee_per_gas(max_fee);
            deployer.tx.set_max_priority_fee_per_gas(priority_fee);
        }

        // Before we actually deploy the contract we try check if the verify settings are valid
        // let mut constructor_args = None;
        // if self.verify {
        //     if !args.is_empty() {
        //         let encoded_args = abi
        //             .constructor()
        //             .ok_or_else(|| eyre::eyre!("could not find constructor"))?
        //             .abi_encode_input(&args)?;
        //         constructor_args = Some(hex::encode(encoded_args));
        //     }

        //     self.verify_preflight_check(constructor_args.clone(), chain, &id)
        //         .await?;
        // }

        // Deploy the actual contract
        let (deployed_contract, _receipt) = deployer.send_with_receipt().await?;

        let address = deployed_contract.clone();
        // if self.json {
        //     let output = json!({
        //         "deployer": deployer_address.to_string(),
        //         "deployedTo": address.to_string(),
        //         "transactionHash": receipt.transaction_hash
        //     });
        //     println!("{output}");
        // } else {
        //     println!("Deployer: {deployer_address}");
        //     println!("Deployed to: {address}");
        //     println!("Transaction hash: {:?}", receipt.transaction_hash);
        // };

        // if !self.verify {
        //     return Ok(deployed_contract);
        // }

        println!("Deploying contract address:{address}...");

        // let num_of_optimizations = None;
        // if self.opts.compiler.optimize.unwrap_or_default() {
        //     self.opts.compiler.optimizer_runs
        // } else {
        //     None
        // };
        // let verify = forge_verify::VerifyArgs {
        //     address,
        //     contract: Some(self.contract),
        //     compiler_version: None,
        //     constructor_args,
        //     constructor_args_path: None,
        //     num_of_optimizations,
        //     etherscan: EtherscanOpts {
        //         key: self.eth.etherscan.key(),
        //         chain: Some(chain.into()),
        //     },
        //     rpc: Default::default(),
        //     flatten: false,
        //     force: false,
        //     skip_is_verified_check: true,
        //     watch: true,
        //     retry: self.retry,
        //     libraries: self.opts.libraries.clone(),
        //     root: None,
        //     verifier: self.verifier,
        //     via_ir: self.opts.via_ir,
        //     evm_version: self.opts.compiler.evm_version,
        //     show_standard_json_input: self.show_standard_json_input,
        //     guess_constructor_args: false,
        //     compilation_profile: Some(id.profile.to_string()),
        // };
        // println!(
        //     "Waiting for {} to detect contract deployment...",
        //     verify.verifier.verifier
        // );
        // verify.run().await?;
        Ok(deployed_contract)
    }
    /// Parses the given constructor arguments into a vector of `DynSolValue`s, by matching them
    /// against the constructor's input params.
    ///
    /// Returns a list of parsed values that match the constructor's input params.
    fn parse_constructor_args(
        &self,
        constructor: &Constructor,
        constructor_args: &[String],
    ) -> eyre::Result<Vec<DynSolValue>> {
        let mut params = Vec::with_capacity(constructor.inputs.len());
        for (input, arg) in constructor.inputs.iter().zip(constructor_args) {
            // resolve the input type directly
            let ty = input
                .resolve()
                .wrap_err_with(|| format!("Could not resolve constructor arg: input={input}"))?;
            params.push((ty, arg));
        }
        let params = params.iter().map(|(ty, arg)| (ty, arg.as_str()));
        parse_tokens(params).map_err(Into::into)
    }
}
/// Given a `Project`'s output, removes the matching ABI, Bytecode and
/// Runtime Bytecode of the given contract.
#[track_caller]
pub fn remove_contract_ex(
    output: ProjectCompileOutput,
    path: &Path,
    name: &str,
) -> Result<(
    JsonAbi,
    CompactBytecode,
    CompactDeployedBytecode,
    ArtifactId,
)> {
    let mut other = Vec::new();
    let Some((id, contract)) = output.into_artifacts().find_map(|(id, artifact)| {
        if id.name == name && id.source == path {
            Some((id, artifact))
        } else {
            other.push(id.name);
            None
        }
    }) else {
        let mut err = format!("could not find artifact: `{name}`");
        if let Some(suggestion) = did_you_mean(name, other).pop() {
            if suggestion != name {
                err = format!(
                    r#"{err}

        Did you mean `{suggestion}`?"#
                );
            }
        }
        eyre::bail!(err)
    };

    let abi = contract
        .get_abi()
        .ok_or_else(|| eyre::eyre!("contract {} does not contain abi", name))?
        .into_owned();

    let bin = contract
        .get_bytecode()
        .ok_or_else(|| eyre::eyre!("contract {} does not contain bytecode", name))?
        .into_owned();

    let deployed_bin = contract
        .get_deployed_bytecode()
        .ok_or_else(|| eyre::eyre!("contract {} does not contain bytecode", name))?
        .into_owned();

    Ok((abi, bin, deployed_bin, id))
}

#[derive(Clone)]
pub struct Web3TesterBlockchain;
// class Web3TesterBlockchain(Web3Blockchain):
//     fn __init__(self) -> None:
//         self.eth_tester = None
//         super().__init__()
//         self.next_acc_idx = 1
impl<P: ZkayProverInterface + std::marker::Send> Web3BlockchainBase<P, Web3TesterBlockchain> {
    //     @classmethod
    fn is_debug_backend(&self) -> bool {
        true
    }
    // fn next_acc_idx(&self) -> ARcCell<i32> {
    //     self.next_acc_idx.clone()
    // }
    async fn _connect_libraries(&self) -> eyre::Result<BTreeMap<String, Address>> {
        zk_print_banner("Deploying Libraries".to_owned());

        let sender = self.web3tx.eth.wallet.from.expect("required"); //"self.w3.eth.accounts[0]";
        // # Since eth-tester is not persistent -> always automatically deploy libraries
        // with cfg.library_compilation_environment():
        // with tempfile.TemporaryDirectory() as tmpdir:
        let tmpdir = std::env::temp_dir();
        // with log_context("deploy_pki"):
        let mut _pki_contract = BTreeMap::new();
        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        for crypto_params in all_crypto_params {
            // with log_context(crypto_params.crypto_name):
            let crypto_param = CryptoParams::new(crypto_params.clone());
            let pki_contract_code = library_contracts::get_pki_contract(&crypto_param);
            let pki_contract_name = CFG
                .lock()
                .unwrap()
                .get_pki_contract_name(&crypto_param.identifier_name());
            let pki_sol = save_to_file(
                Some(tmpdir.clone()),
                &format!("{pki_contract_name}.sol"),
                &pki_contract_code,
            );
            let (abi, bin, id) = self.compile_contract(
                &PathBuf::from(pki_sol),
                &pki_contract_name,
                None,
                &PathBuf::from("."),
            )?;
            println!("========_deploy_contract=====4======");
            let contract = self
                ._deploy_contract(&sender, abi.clone(), &vec![], None, abi, bin, id)
                .await?;
            let backend_name = crypto_params.clone();
            _pki_contract.insert(backend_name.clone(), contract.clone());
            zk_print!(
                r#"Deployed pki contract for crypto back-end {backend_name} at address "{:?}""#,
                contract
            );
        }
        *self._pki_contract.lock() = Some(_pki_contract);
        // with log_context("deploy_verify_libs"):
        let verify_sol = save_to_file(
            Some(tmpdir.clone()),
            "verify_libs.sol",
            &library_contracts::get_verify_libs_code(),
        );
        let mut _lib_addresses = BTreeMap::new();
        for lib in CFG.lock().unwrap().external_crypto_lib_names() {
            let (abi, bin, id) = self.compile_contract(
                &PathBuf::from(verify_sol.clone()),
                &lib,
                None,
                &PathBuf::from("."),
            )?;
            println!("========_deploy_contract======5=====");
            let out = self
                ._deploy_contract(&sender, abi.clone(), &vec![], None, abi, bin, id)
                .await?;
            _lib_addresses.insert(lib.clone(), out.clone());
            zk_print!(r#"Deployed crypto lib {lib} at address "{out:?}""#);
        }
        Ok(_lib_addresses)
    }

    fn _gas_heuristics(&self, _sender: &str, _tx: &str) -> i32 {
        MAX_GAS_LIMIT
    }
}
const MAX_GAS_LIMIT: i32 = 10000000;
// impl<P: ZkayProverInterface+ std::marker::Send> Web3Blockchain
//     for Web3BlockchainBase<P, Web3TesterBlockchain>
// {
//     fn _create_w3_instance(&self) {
//         // let genesis_overrides = json!({"gas_limit": max_gas_limit * 1.2});
//         // let custom_genesis_params = PyEVMBackend._generate_genesis_params(genesis_overrides);
//         // self.eth_tester = EthereumTester(PyEVMBackend(genesis_parameters = custom_genesis_params));
//         // let w3 = Web3(Web3.EthereumTesterProvider(self.eth_tester));
//         //  w3
//     }
// }
// impl<P: ZkayProverInterface+ std::marker::Send> Web3Blockchain for Web3BlockchainBase<P, Web3HttpGanacheBlockchain> {
//     fn _create_w3_instance(&self) {
//         // let genesis_overrides = json!({"gas_limit": max_gas_limit * 1.2});
//         // let custom_genesis_params = PyEVMBackend._generate_genesis_params(genesis_overrides);
//         // self.eth_tester = EthereumTester(PyEVMBackend(genesis_parameters = custom_genesis_params));
//         // let w3 = Web3(Web3.EthereumTesterProvider(self.eth_tester));
//         //  w3
//     }
// }
pub struct Web3IpcBlockchain;
// class Web3IpcBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.IPCProvider(cfg.blockchain_node_uri))
// pub struct Web3WebsocketBlockchain;
// class Web3WebsocketBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.WebsocketProvider(cfg.blockchain_node_uri))

// class Web3HttpBlockchain(Web3Blockchain):
pub struct Web3HttpBlockchain;
// impl Web3Blockchain for Web3HttpBlockchain
// {
//     fn _create_w3_instance(&self)
//       {  // assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//          Web3(Web3.HTTPProvider(cfg.blockchain_node_uri))}
// }
#[derive(Clone)]
pub struct Web3HttpGanacheBlockchain {
    next_acc_idx: i32,
}
// class Web3HttpGanacheBlockchain(Web3HttpBlockchain):
//     fn __init__(self) -> None:
//         super().__init__()
//         self.next_acc_idx = 1
// impl Web3Blockchain for Web3HttpGanacheBlockchain {
//     fn _create_w3_instance(&self) {
//         // assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         // Web3(Web3.HTTPProvider(cfg.blockchain_node_uri))
//     }
// }
// impl Web3HttpGanacheBlockchain {
//     //     @classmethod
//     fn is_debug_backend(&self) -> bool {
//         true
//     }

//     fn create_test_accounts(&self, count: i32) -> Vec<String> {
//         let accounts = vec![]; //self.w3.eth.accounts;
//         assert!(
//             accounts.len() >= self.next_acc_idx + count,
//             "Can have at most {} dummy accounts in total",
//             accounts.len() - 1
//         );
//         let idx = self.next_acc_idx as usize;
//         let dummy_accounts = accounts[idx..idx + count as usize].to_vec();
//         self.next_acc_idx += count;
//         dummy_accounts
//     }

//     fn _gas_heuristic(&self, sender: &str, tx: &str) -> i32 {
//         // self.w3.eth.get_block("latest")["gasLimit"]
//         0
//     }
// }

pub struct Web3CustomBlockchain;
// impl Web3Blockchain for Web3CustomBlockchain {}
// class Web3CustomBlockchain(Web3Blockchain):
//     fn _create_w3_instance(self) -> Web3:
//         assert isinstance(cfg.blockchain_node_uri, Web3)
//         return cfg.blockchain_node_uri
