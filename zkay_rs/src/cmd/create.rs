#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
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
use foundry_cli::{
    opts::{CoreBuildArgs, EthereumOpts, EtherscanOpts, TransactionOpts},
    utils::{self, LoadConfig, read_constructor_args_file, remove_contract},
};
use foundry_common::{
    compile::{self},
    fmt::parse_tokens,
};
use foundry_compilers::{ArtifactId, Project};
use foundry_compilers::{artifacts::BytecodeObject, info::ContractInfo, utils::canonicalize};
// use alloy_json_abi::JsonAbi;
// use alloy_primitives::Address;
// use eyre::{Result, WrapErr};
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

use my_logging::{log_context::log_context, logger::data};
use privacy::library_contracts;
use serde_json::{Value, json};
use solidity::compiler::compile_solidity_json;
use std::collections::BTreeMap;
use zkay_config::{
    config::{CFG, library_compilation_environment, zk_print_banner},
    config_user::UserConfig,
    with_context_block, zk_print,
};
use zkay_transaction::{
    blockchain::web3::Web3Tx, interface::ZkayBlockchainInterface, offchain::new_contract_simulator,
};
use zkay_transaction_crypto_params::params::CryptoParams;
// use  zkay_transaction:: crypto:: params ::  CryptoParams;
// use zkay_transaction::interface::ZkayBlockchainInterface;
// , IntegrityError, BlockChainError,
//     TransactionFailedException
// use  zkay_transaction:: types ::  PublicKeyValue, AddressValue, MsgStruct, BlockStruct, TxStruct;
use ast_builder::process_ast::get_verification_contract_names;
use rccell::RcCell;
use std::str::FromStr;
use zkay_ast::global_defs::{
    GlobalDefs, GlobalVars, array_length_member, global_defs, global_vars,
};
use zkay_utils::helpers::{get_contract_names, save_to_file};
use zkay_utils::timer::time_measure;
alloy_sol_types::sol!(
    function greet(string greeting) public;
);
alloy_sol_types::sol!(
    function greeting(uint256 i) public returns (string);
);
merge_impl_figment_convert!(CreateArgs, opts, eth);

/// CLI arguments for `forge create`.
#[derive(Clone, Debug, Parser)]
pub struct CreateArgs {
    /// The contract identifier in the form `<path>:<contractname>`.
    contract: ContractInfo,

    /// The constructor arguments.
    #[arg(
        long,
        num_args(1..),
        conflicts_with = "constructor_args_path",
        value_name = "ARGS",
        allow_hyphen_values = true,
    )]
    constructor_args: Vec<String>,

    /// The path to a file containing the constructor arguments.
    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_name = "PATH",
    )]
    constructor_args_path: Option<PathBuf>,

    // /// The constructor arguments.
    // #[arg(long, allow_hyphen_values = true)]
    // blockchain_pki_addresses: Vec<String>,
    /// The constructor arguments.
    #[arg(long, allow_hyphen_values = true)]
    blockchain_crypto_lib_addresses: Vec<String>,

    /// Deploy pki lib.
    #[arg(long)]
    deploy_pki: bool,

    /// Deploy external lib.
    #[arg(long)]
    deploy_external: bool,

    /// Deploy lib.
    #[arg(long)]
    is_lib: bool,

    /// Call contract function.
    #[arg(long)]
    is_call: bool,

    /// Send contract function.
    #[arg(long)]
    is_send: bool,

    /// Print the deployment information as JSON.
    #[arg(long, help_heading = "Display options")]
    json: bool,

    /// Verify contract after creation.
    #[arg(long)]
    verify: bool,

    /// Send via `eth_sendTransaction` using the `--from` argument or `$ETH_FROM` as sender
    #[arg(long, requires = "from")]
    unlocked: bool,

    /// Prints the standard json compiler input if `--verify` is provided.
    ///
    /// The standard json compiler input can be used to manually submit contract verification in
    /// the browser.
    #[arg(long, requires = "verify")]
    show_standard_json_input: bool,

    /// Timeout to use for broadcasting transactions.
    #[arg(long, env = "ETH_TIMEOUT")]
    pub timeout: Option<u64>,

    #[command(flatten)]
    opts: CoreBuildArgs,

    #[command(flatten)]
    tx: TransactionOpts,

    #[command(flatten)]
    eth: EthereumOpts,

    #[command(flatten)]
    pub verifier: forge_verify::VerifierArgs,

    #[command(flatten)]
    retry: RetryArgs,

    #[arg(long, allow_hyphen_values = true, value_delimiter = ',')]
    blockchain_pki_addresses: Vec<String>,

    #[arg(id = "survey", long = "survey", alias = "is-survey")]
    is_survey: bool,
}

impl CreateArgs {
    /// Executes the command to create a contract
    pub async fn run(mut self) -> Result<()> {
        if self.deploy_pki {
            return self.deploy_pki().await;
        }
        if self.deploy_external {
            return self.deploy_external().await;
        }
        if self.is_lib {
            return self.deploy_lib().await;
        }
        if self.is_call {
            return self.call().await;
        }
        if self.is_send {
            return self.send().await;
        }
        let config = self.try_load_config_emit_warnings()?;

        let Self {
            // to,
            // mut sig,
            // mut args,
            mut tx,
            eth,
            // command,
            // block,
            // trace,
            // evm_version,
            // debug,
            // decode_internal,
            // labels,
            // data,
            blockchain_pki_addresses,
            ..
        } = self;

        let web3tx = Web3Tx::new(eth.clone(), config.clone(), tx.clone()).await?;
        if self.is_survey {
            CFG.lock()
                .unwrap()
                .set_blockchain_pki_address(blockchain_pki_addresses);
            return crate::contract::main0(web3tx).await;
        }
        // Find Project & Compile
        let project = config.project()?;

        let target_path = if let Some(ref mut path) = self.contract.path {
            canonicalize(project.root().join(path))?
        } else {
            project.find_contract_path(&self.contract.name)?
        };
        let _target_path_str = format!("{}", target_path.display());

        // let inst_parent_target_path = std::env::temp_dir().join(
        //     target_path
        //         .iter()
        //         .nth_back(1)
        //         .map_or("_inst".to_owned(), |s| {
        //             s.to_string_lossy().to_string() + "_inst"
        //         }),
        // );
        // self._deploy(&target_path, &project, &self.contract.name)
        //     .await?;
        // // let inst_target_path=PathBuf::from(inst_target_path_str.clone());
        // // let inst_config = Config::load_with_root(inst_target_path_str).sanitized();
        // // let inst_project = inst_config.project()?;
        // println!("==target_path========={},", target_path.display());
        // // let mut output = compile::compile_target(&inst_target_path, &inst_project, self.json)?;
        // let mut output = compile::compile_target(&target_path, &project, self.json)?;
        // let (abi, bin, _) = remove_contract(&mut output, &target_path, &self.contract.name)?;

        // let bin = match bin.object {
        //     BytecodeObject::Bytecode(_) => bin.object,
        //     _ => {
        //         let link_refs = bin
        //             .link_references
        //             .iter()
        //             .flat_map(|(path, names)| {
        //                 names.keys().map(move |name| format!("\t{name}: {path}"))
        //             })
        //             .collect::<Vec<String>>()
        //             .join("\n");
        //         eyre::bail!("Dynamic linking not supported in `create` command - deploy the following library contracts first, then provide the address to link at compile time\n{}", link_refs)
        //     }
        // };

        // // Add arguments to constructor
        // let params = if let Some(constructor) = &abi.constructor {
        //     let constructor_args = self
        //         .constructor_args_path
        //         .clone()
        //         .map(read_constructor_args_file)
        //         .transpose()?;
        //     self.parse_constructor_args(
        //         constructor,
        //         constructor_args
        //             .as_deref()
        //             .unwrap_or(&self.constructor_args),
        //     )?
        // } else {
        //     vec![]
        // };

        // let provider = utils::get_provider(&config)?;

        // // respect chain, if set explicitly via cmd args
        // let chain_id = if let Some(chain_id) = self.chain_id() {
        //     chain_id
        // } else {
        //     provider.get_chain_id().await?
        // };
        // if self.unlocked {
        //     // Deploy with unlocked account
        //     let sender = self.eth.wallet.from.expect("required");
        //     self.deploy(
        //         abi,
        //         bin,
        //         params,
        //         provider,
        //         chain_id,
        //         sender,
        //         config.transaction_timeout,
        //     )
        //     .await?
        // } else {
        //     // Deploy with signer
        //     let signer = self.eth.wallet.signer().await?;
        //     let deployer = signer.address();
        //     let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
        //         .wallet(EthereumWallet::new(signer))
        //         .on_provider(provider);
        //     self.deploy(
        //         abi,
        //         bin,
        //         params,
        //         provider,
        //         chain_id,
        //         deployer,
        //         config.transaction_timeout,
        //     )
        //     .await?
        // };
        Ok(())
    }

    async fn call(&self) -> eyre::Result<()> {
        use alloy_primitives::{Address, Bytes, U256};
        use alloy_provider::{ProviderBuilder, RootProvider, network::AnyNetwork};
        use alloy_rpc_types::TransactionRequest;
        use alloy_serde::WithOtherFields;
        use alloy_sol_types::{SolCall, sol};
        use cast::Cast;
        use std::str::FromStr;

        sol!(
            function is_result_published() public view returns (bool) ;
        );
        let alloy_provider = ProviderBuilder::<_, _, AnyNetwork>::default()
            .on_builtin("http://localhost:8545")
            .await?;
        let to = Address::from_str("0x5b98D45C450fff4998d9Eb408C460C8b8B344Afa")?;
        let greeting = is_result_publishedCall {}.abi_encode(); //is_result_publishedCall
        let bytes = Bytes::from_iter(greeting.iter());
        let tx = TransactionRequest::default().to(to).input(bytes.into());
        let tx = WithOtherFields::new(tx);
        let cast = Cast::new(alloy_provider);
        let data = cast.call(&tx, None, None).await?;
        println!("=========call==========={}", data);
        Ok(())
    }

    async fn send(&self) -> eyre::Result<()> {
        use alloy_primitives::{Address, Bytes, U256};
        use alloy_provider::{ProviderBuilder, RootProvider, network::AnyNetwork};
        use alloy_rpc_types::TransactionRequest;
        use alloy_serde::WithOtherFields;
        use alloy_sol_types::{SolCall, sol};
        use cast::Cast;
        use std::str::FromStr;

        let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
            .on_builtin("http://localhost:8545")
            .await?;
        let from = Address::from_str("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")?;
        let to = Address::from_str("0xB3C95ff08316fb2F2e3E52Ee82F8e7b605Aa1304")?;
        let greeting = greetCall {
            greeting: "hello".to_string(),
        }
        .abi_encode();
        let bytes = Bytes::from_iter(greeting.iter());
        let _gas = U256::from_str("200000").unwrap();
        let _value = U256::from_str("1").unwrap();
        let _nonce = U256::from_str("1").unwrap();
        let tx = TransactionRequest::default()
            .to(to)
            .input(bytes.into())
            .from(from);
        let tx = WithOtherFields::new(tx);
        let cast = Cast::new(provider);
        let data = cast.send(tx).await?;
        println!("{:#?}", data);
        Ok(())
    }
    pub async fn deploy_pki(mut self) -> Result<()> {
        let config = self.try_load_config_emit_warnings()?;
        let Self { mut tx, eth, .. } = self;
        let web3tx = Web3Tx::new(eth.clone(), config.clone(), tx.clone()).await?;
        let sender = if let Some(address) = eth.wallet.from {
            address
        } else {
            let signer = eth.wallet.signer().await?;
            signer.address()
        };
        let contract_simulator = new_contract_simulator(
            "/Users/lisheng/mygit/arron/zkay-rs/survey_compiled/",
            "",
            sender,
            web3tx,
        );
        let tmpdir = std::env::temp_dir();
        // with log_context("deploy_pki"):
        // let mut _pki_contract = BTreeMap::new();
        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        for crypto_params in all_crypto_params {
            // with log_context(crypto_params.crypto_name):
            println!("=====crypto_params========{crypto_params}");
            let crypto_param = CryptoParams::new(crypto_params.clone());
            let pki_contract_code = library_contracts::get_pki_contract(&crypto_param);
            let pki_contract_name = CFG
                .unwrap()
                .get_pki_contract_name(&crypto_param.identifier_name());
            let file = save_to_file(
                Some(tmpdir.clone()),
                &format!("{pki_contract_name}.sol"),
                &pki_contract_code,
            );
            let _ = contract_simulator
                .runtime
                .blockchain()
                .deploy_solidity_contract(&file, Some(pki_contract_name), &sender)
                .await;
        }
        Ok(())
    }
    pub async fn deploy_external(mut self) -> Result<()> {
        let config = self.try_load_config_emit_warnings()?;
        let Self { mut tx, eth, .. } = self;
        let web3tx = Web3Tx::new(eth.clone(), config.clone(), tx.clone()).await?;
        let sender = if let Some(address) = eth.wallet.from {
            address
        } else {
            let signer = eth.wallet.signer().await?;
            signer.address()
        };
        let contract_simulator = new_contract_simulator(
            "/Users/lisheng/mygit/arron/zkay-rs/survey_compiled/",
            "",
            sender,
            web3tx,
        );
        let tmpdir = std::env::temp_dir();
        let external_crypto_lib_names = CFG.lock().unwrap().external_crypto_lib_names();
        if !external_crypto_lib_names.is_empty() {
            let file = save_to_file(
                Some(tmpdir),
                "verify_libs.sol",
                &library_contracts::get_verify_libs_code(),
            );
            for lib in external_crypto_lib_names {
                let _ = contract_simulator
                    .runtime
                    .blockchain()
                    .deploy_solidity_contract(&file, Some(lib), &sender)
                    .await;
            }
        }

        Ok(())
    }
    /// Executes the command to create a contract
    pub async fn deploy_lib(mut self) -> Result<()> {
        let config = self.try_load_config_emit_warnings()?;
        // Find Project & Compile
        let project = config.project()?;

        let target_path = if let Some(ref mut path) = self.contract.path {
            canonicalize(project.root().join(path))?
        } else {
            project.find_contract_path(&self.contract.name)?
        };
        let _target_path_str = format!("{}", target_path.display());

        let _inst_parent_target_path = std::env::temp_dir().join(
            target_path
                .iter()
                .nth_back(1)
                .map_or("_inst".to_owned(), |s| {
                    s.to_string_lossy().to_string() + "_inst"
                }),
        );

        println!("==target_path========={},", target_path.display());
        let output = compile::compile_target(&target_path, &project, self.json)?;
        let (abi, bin, id) = remove_contract(output, &target_path, &self.contract.name)?;

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
            let constructor_args = self
                .constructor_args_path
                .clone()
                .map(read_constructor_args_file)
                .transpose()?;
            self.parse_constructor_args(
                constructor,
                constructor_args
                    .as_deref()
                    .unwrap_or(&self.constructor_args),
            )?
        } else {
            vec![]
        };

        let provider = utils::get_provider(&config)?;

        // respect chain, if set explicitly via cmd args
        let chain_id = if let Some(chain_id) = self.chain_id() {
            chain_id
        } else {
            provider.get_chain_id().await?
        };
        if self.unlocked {
            // Deploy with unlocked account
            let sender = self.eth.wallet.from.expect("required");
            self.deploy(
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
        } else {
            // Deploy with signer
            let signer = self.eth.wallet.signer().await?;
            let deployer = signer.address();
            let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
                .wallet(EthereumWallet::new(signer))
                .on_provider(provider);
            self.deploy(
                abi,
                bin,
                params,
                provider,
                chain_id,
                deployer,
                config.transaction_timeout,
                id,
            )
            .await
        }?;
        Ok(())
    }

    async fn _deploy(
        &self,
        project_dir: &PathBuf,
        project: &Project,
        contract: &str,
    ) -> Result<()> {
        let mut project_dir = project_dir.clone();
        project_dir.pop();
        let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
        // with open(os.path.join(project_dir, "contract.zkay")) as f:
        let verifier_names = get_verification_contract_names(
            (
                std::fs::read_to_string(project_dir.join("contract.zkay")).ok(),
                None,
            ),
            global_vars,
        );

        // Deploy verification contracts if not already done
        let external_contract_addresses = self
            ._deploy_dependencies(&project_dir, verifier_names, project)
            .await?;
        // let cout;
        let inst_target_path = std::env::temp_dir().join(
            project_dir
                .iter()
                .nth_back(1)
                .map_or("_inst".to_owned(), |s| {
                    s.to_string_lossy().to_string() + "_inst"
                }),
        );
        std::fs::create_dir_all(inst_target_path.clone())?;
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
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map_or(false, |s| s != "contract.sol" && s.ends_with(".sol"))
            })
            .for_each(|x| {
                let _ = std::fs::copy(x.path(), inst_target_path.join(x.file_name()));
            });
        let filename = self.__hardcoded_external_contracts_ctx(
            &project_dir,
            &inst_target_path,
            &external_contract_addresses,
        );
        // let inst_target_path=PathBuf::from(inst_target_path_str.clone());
        let inst_config = Config::load_with_root(inst_target_path).sanitized();
        let _inst_project = inst_config.project()?;
        // with_context_block!(var filename= self.__hardcoded_external_contracts_ctx(&project_dir, &external_contract_addresses) =>{
        let (abi, bin, id) = self.compile_contract(&filename, project, contract)?;
        // });
        let handle;
        // with_context_block!(var _a= log_context("constructor")=>{
        // with_context_block!(var _b= log_context(&format!("{contract}"))=>{
        handle = self._deploy_contract(abi, bin, id).await?;
        // });
        // });
        zk_print!(r#"Deployed contract "{contract}" at address "{handle:?}""#);
        // handle
        Ok(())
    }

    async fn _deploy_dependencies(
        &self,
        project_dir: &PathBuf,
        verifier_names: Vec<String>,
        project: &Project,
    ) -> Result<BTreeMap<String, Address>> {
        // # Deploy verification contracts if not already done
        let mut vf = BTreeMap::new();
        for verifier_name in verifier_names {
            // with_context_block!(var _a= log_context("constructor")=>{
            //     with_context_block!(var _b=log_context(&format!("{verifier_name}"))=>{
            let filename = project_dir.join(&format!("{verifier_name}.sol"));
            let (abi, bin, id) = self.compile_contract(&filename, project, &verifier_name)?;
            let v = self._deploy_contract(abi, bin, id).await?;
            // with_context_block!(var _tm= time_measure("transaction_full",false,false)=>{
            vf.insert(verifier_name.clone(), v);
            // });
            // });
            // });
        }
        // for crypto_params in CFG.lock().unwrap().all_crypto_params() {
        //     let pki_contract_name = CFG.lock().unwrap().get_pki_contract_name(&crypto_params);
        //     let pki_contract_address = String::new(); //self.pki_contract(&crypto_params);
        //     vf.insert(pki_contract_name, pki_contract_address);
        // }
        let mut pki = self._connect_libraries(project_dir, project).await?;
        vf.append(&mut pki);
        Ok(vf)
    }
    fn _verify_contract_integrity(
        &self,
        mut address: String,
        sol_filename: &PathBuf,
        _args: Vec<String>,
        _libraries: Option<BTreeMap<String, String>>,
        mut contract_name: Option<String>,
        is_library: bool,
        project: &Project,
    ) -> String {
        // if is_instance(address, bytes)
        //     {address = self.w3.toChecksumAddress(address);}

        let contract_name = if let Some(contract_name) = contract_name {
            contract_name
        } else {
            get_contract_names(&format!("{}", sol_filename.display()))[0].clone()
        };
        let actual_byte_code = self.__normalized_hex(address.clone()); //self.w3.eth.get_code(address) MYTODO
        assert!(
            !actual_byte_code.is_empty(),
            "Expected contract {} is not deployed at address {address}",
            contract_name
        );

        let _cout = self.compile_contract(sol_filename, project, &contract_name);
        let mut expected_byte_code = String::new();
        // self.__normalized_hex(cout["deployed_bin"].as_str().unwrap().to_owned());

        if is_library {
            // # https://github.com/ethereum/solidity/issues/7101
            expected_byte_code = expected_byte_code[..2].to_owned()
                + &self.__normalized_hex(address.clone())
                + &expected_byte_code[42..]
        };

        assert!(
            actual_byte_code == expected_byte_code,
            "Deployed contract at address {address} does not match local contract {}",
            sol_filename.display()
        );
        zk_print!(
            "Contract@{address} matches {}:{contract_name}",
            sol_filename
                .iter()
                .nth_back(0)
                .map_or(String::new(), |x| x.to_string_lossy().to_string()) // &sol_filename[sol_filename.rfind("/").map_or(0, |i| i + 1)..]
        );

        // self.w3.eth.contract(
        //     address=address, abi=cout["abi"]
        // )
        String::new()
    }
    async fn _connect_libraries(
        &self,
        project_dir: &PathBuf,
        project: &Project,
    ) -> Result<BTreeMap<String, Address>> {
        // assert!(
        //     !CFG.lock().unwrap().blockchain_pki_address().is_empty(),
        //     "Must specify pki address in config."
        // );

        // let mut lib_addresses = vec![];
        // if !CFG.lock().unwrap().external_crypto_lib_names().is_empty() {
        //     if !CFG
        //
        //         .unwrap()
        //         .blockchain_crypto_lib_addresses()
        //         .is_empty()
        //     {
        //         lib_addresses = CFG
        //
        //             .unwrap()
        //             .blockchain_crypto_lib_addresses()
        //             .split(",")
        //             .map(|addr| addr.trim().to_owned())
        //             .collect();
        //     }

        //     assert!(
        //         lib_addresses.len() == CFG.lock().unwrap().external_crypto_lib_names().len(),
        //         "Must specify all crypto library addresses in config\nExpected {} was {}",
        //         CFG.lock().unwrap().external_crypto_lib_names().len(),
        //         lib_addresses.len()
        //     );
        // }
        // with_context_block!(var _lce=library_compilation_environment()=>{
        // let tmpdir = std::env::temp_dir();
        if !self.blockchain_pki_addresses.is_empty() {
            return Ok(CFG
                .unwrap()
                .all_crypto_params()
                .into_iter()
                .zip(
                    self.blockchain_pki_addresses
                        .iter()
                        .map(|v| Address::from_str(&v).unwrap()),
                )
                .collect());
        }

        let mut contract_name2address = BTreeMap::new();
        let all_crypto_params = CFG.lock().unwrap().all_crypto_params();
        for crypto_params in all_crypto_params {
            let contract_name = CFG
                .unwrap()
                .get_pki_contract_name(&CryptoParams::new(crypto_params).identifier_name());
            // let pki_sol = save_to_file(
            //     Some(tmpdir.clone()),
            //     &format!("{contract_name}.sol"),
            //     &library_contracts::get_pki_contract(&CryptoParams::new(crypto_params)),
            // );
            // *self._pki_contract.borrow_mut() = self._verify_contract_integrity(
            //     CFG.lock().unwrap().blockchain_pki_address(),
            //     &pki_sol,
            //     vec![],
            //     None,
            //     Some(contract_name),
            //     false,
            //     None,
            // );
            let filename = project_dir.join(&format!("{contract_name}.sol"));
            let (abi, bin, id) = self.compile_contract(&filename, project, &contract_name)?;
            let v = self._deploy_contract(abi, bin, id).await?;
            // with_context_block!(var _tm= time_measure("transaction_full",false,false)=>{
            contract_name2address.insert(contract_name.clone(), v);
        }
        Ok(contract_name2address)
        // let verify_sol = save_to_file(
        //     Some(tmpdir),
        //     "verify_libs.sol",
        //     &library_contracts::get_verify_libs_code(),
        // );
        // let mut _lib_addresses = BTreeMap::new();
        // for (lib, addr) in CFG
        //
        //     .unwrap()
        //     .external_crypto_lib_names()
        //     .iter()
        //     .zip(&lib_addresses)
        // {
        //     let out = self._verify_contract_integrity(
        //         addr.to_owned(),
        //         &verify_sol,
        //         vec![],
        //         None,
        //         Some(lib.clone()),
        //         true,
        //         None,
        //     );
        //     _lib_addresses.insert(lib.clone(), out);
        // }
        // *self._lib_addresses.borrow_mut() = Some(_lib_addresses);
        // });
        // CFG.lock()
        //     .unwrap()
        //     .all_crypto_params()
        //     .into_iter()
        //     .zip(self.blockchain_pki_addresses.clone())
        //     .collect()
    }
    fn __normalized_hex(&self, mut val: String) -> String {
        // if not isinstance(val, str):
        // val = val.hex()
        if val.starts_with("0x") {
            val = val[2..].to_owned();
        }
        val.make_ascii_lowercase();
        val
    }
    pub fn compile_contract(
        &self,
        sol_filename: &PathBuf,
        project: &Project,
        contract_name: &str,
    ) -> Result<(JsonAbi, CompactBytecode, ArtifactId)> {
        // let solp = std::path::PathBuf::from(sol_filename);
        // let config = Config::load_with_root(sol_filename).sanitized();
        // let project = config.project()?;
        // let jout = compile_solidity_json(
        //     sol_filename,
        //     libs,
        //     CFG.lock().unwrap().opt_solc_optimizer_runs(),
        //     vec![],
        //     cwd.to_str().unwrap(),
        // )
        // .unwrap();
        // let jout = &jout["contracts"][solp.file_name().unwrap().to_str().unwrap()][&contract_name];
        // json!({
        //     "abi": jout["metadata"]["output"]["abi"],
        //     "bin": jout["evm"]["bytecode"]["object"],
        //     "deployed_bin": jout["evm"]["deployedBytecode"]["object"],
        // })
        let mut output = compile::compile_target(&sol_filename, project, self.json)?;
        remove_contract(output, &sol_filename, contract_name)
    }
    async fn _deploy_contract(
        &self,
        abi: JsonAbi,
        bin: CompactBytecode,
        id: ArtifactId,
    ) -> Result<Address> {
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
            let constructor_args = self
                .constructor_args_path
                .clone()
                .map(read_constructor_args_file)
                .transpose()?;
            self.parse_constructor_args(
                constructor,
                constructor_args
                    .as_deref()
                    .unwrap_or(&self.constructor_args),
            )?
        } else {
            vec![]
        };
        let config = self.try_load_config_emit_warnings()?;

        let provider = utils::get_provider(&config)?;

        // respect chain, if set explicitly via cmd args
        let chain_id = if let Some(chain_id) = self.chain_id() {
            chain_id
        } else {
            provider.get_chain_id().await?
        };
        if self.unlocked {
            // Deploy with unlocked account
            let sender = self.eth.wallet.from.expect("required");
            self.clone()
                .deploy(
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
        } else {
            // Deploy with signer
            let signer = self.eth.wallet.signer().await?;
            let deployer = signer.address();
            let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
                .wallet(EthereumWallet::new(signer))
                .on_provider(provider);
            self.clone()
                .deploy(
                    abi,
                    bin,
                    params,
                    provider,
                    chain_id,
                    deployer,
                    config.transaction_timeout,
                    id,
                )
                .await
        }
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
        let mut c = std::fs::read_to_string(contract_dir.join("contract.sol")).unwrap();
        for (key, val) in pki_verifier_addresses {
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
        println!("{}", output_filename.display());
        output_filename
        // pass
    }

    /// Returns the provided chain id, if any.
    fn chain_id(&self) -> Option<u64> {
        self.eth.etherscan.chain.map(|chain| chain.id())
    }

    /// Ensures the verify command can be executed.
    ///
    /// This is supposed to check any things that might go wrong when preparing a verify request
    /// before the contract is deployed. This should prevent situations where a contract is deployed
    /// successfully, but we fail to prepare a verify request which would require manual
    /// verification.
    async fn verify_preflight_check(
        &self,
        constructor_args: Option<String>,
        chain: u64,
        id: &ArtifactId,
    ) -> Result<()> {
        // NOTE: this does not represent the same `VerifyArgs` that would be sent after deployment,
        // since we don't know the address yet.
        let mut verify = VerifyArgs {
            address: Default::default(),
            contract: Some(self.contract.clone()),
            compiler_version: Some(id.version.to_string()),
            constructor_args,
            constructor_args_path: None,
            num_of_optimizations: None,
            etherscan: EtherscanOpts {
                key: self.eth.etherscan.key.clone(),
                chain: Some(chain.into()),
            },
            rpc: Default::default(),
            flatten: false,
            force: false,
            skip_is_verified_check: true,
            watch: true,
            retry: self.retry,
            libraries: self.opts.libraries.clone(),
            root: None,
            verifier: self.verifier.clone(),
            via_ir: self.opts.via_ir,
            evm_version: self.opts.compiler.evm_version,
            show_standard_json_input: self.show_standard_json_input,
            guess_constructor_args: false,
            compilation_profile: Some(id.profile.to_string()),
        };

        // Check config for Etherscan API Keys to avoid preflight check failing if no
        // ETHERSCAN_API_KEY value set.
        let config = verify.load_config_emit_warnings();
        verify.etherscan.key = config
            .get_etherscan_config_with_chain(Some(chain.into()))?
            .map(|c| c.key);

        let context = verify.resolve_context().await?;

        verify
            .verification_provider()?
            .preflight_verify_check(verify, context)
            .await?;
        Ok(())
    }

    /// Deploys the contract
    #[allow(clippy::too_many_arguments)]
    async fn deploy<P: Provider<T, AnyNetwork>, T: Transport + Clone>(
        self,
        abi: JsonAbi,
        bin: BytecodeObject,
        args: Vec<DynSolValue>,
        provider: P,
        chain: u64,
        deployer_address: Address,
        timeout: u64,
        id: ArtifactId,
    ) -> Result<Address> {
        let bin = bin.into_bytes().unwrap_or_else(|| {
            panic!("no bytecode found in bin object for {}", self.contract.name)
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
        let is_legacy = self.tx.legacy || Chain::from(chain).is_legacy();

        deployer.tx.set_from(deployer_address);
        deployer.tx.set_chain_id(chain);
        // `to` field must be set explicitly, cannot be None.
        if deployer.tx.to.is_none() {
            deployer.tx.set_create();
        }
        deployer.tx.set_nonce(if let Some(nonce) = self.tx.nonce {
            Ok(nonce.to())
        } else {
            provider.get_transaction_count(deployer_address).await
        }?);

        // set tx value if specified
        if let Some(value) = self.tx.value {
            deployer.tx.set_value(value);
        }

        deployer
            .tx
            .set_gas_limit(if let Some(gas_limit) = self.tx.gas_limit {
                Ok(gas_limit.to())
            } else {
                provider.estimate_gas(&deployer.tx).await
            }?);

        if is_legacy {
            let gas_price = if let Some(gas_price) = self.tx.gas_price {
                gas_price.to()
            } else {
                provider.get_gas_price().await?
            };
            deployer.tx.set_gas_price(gas_price);
        } else {
            let estimate = provider.estimate_eip1559_fees(None).await.wrap_err("Failed to estimate EIP1559 fees. This chain might not support EIP1559, try adding --legacy to your command.")?;
            let priority_fee = if let Some(priority_fee) = self.tx.priority_gas_price {
                priority_fee.to()
            } else {
                estimate.max_priority_fee_per_gas
            };
            let max_fee = if let Some(max_fee) = self.tx.gas_price {
                max_fee.to()
            } else {
                estimate.max_fee_per_gas
            };

            deployer.tx.set_max_fee_per_gas(max_fee);
            deployer.tx.set_max_priority_fee_per_gas(priority_fee);
        }

        // Before we actually deploy the contract we try check if the verify settings are valid
        let mut constructor_args = None;
        if self.verify {
            if !args.is_empty() {
                let encoded_args = abi
                    .constructor()
                    .ok_or_else(|| eyre::eyre!("could not find constructor"))?
                    .abi_encode_input(&args)?;
                constructor_args = Some(hex::encode(encoded_args));
            }

            self.verify_preflight_check(constructor_args.clone(), chain, &id)
                .await?;
        }

        // Deploy the actual contract
        let (deployed_contract, receipt) = deployer.send_with_receipt().await?;

        let address = deployed_contract.clone();
        if self.json {
            let output = json!({
                "deployer": deployer_address.to_string(),
                "deployedTo": address.to_string(),
                "transactionHash": receipt.transaction_hash
            });
            println!("{output}");
        } else {
            println!("Deployer: {deployer_address}");
            println!("Deployed to: {address}");
            println!("Transaction hash: {:?}", receipt.transaction_hash);
        };

        if !self.verify {
            return Ok(deployed_contract);
        }

        println!("Starting contract verification...");

        let num_of_optimizations = if self.opts.compiler.optimize.unwrap_or_default() {
            self.opts.compiler.optimizer_runs
        } else {
            None
        };
        let verify = forge_verify::VerifyArgs {
            address,
            contract: Some(self.contract),
            compiler_version: None,
            constructor_args,
            constructor_args_path: None,
            num_of_optimizations,
            etherscan: EtherscanOpts {
                key: self.eth.etherscan.key(),
                chain: Some(chain.into()),
            },
            rpc: Default::default(),
            flatten: false,
            force: false,
            skip_is_verified_check: true,
            watch: true,
            retry: self.retry,
            libraries: self.opts.libraries.clone(),
            root: None,
            verifier: self.verifier,
            via_ir: self.opts.via_ir,
            evm_version: self.opts.compiler.evm_version,
            show_standard_json_input: self.show_standard_json_input,
            guess_constructor_args: false,
            compilation_profile: Some(id.profile.to_string()),
        };
        println!(
            "Waiting for {} to detect contract deployment...",
            verify.verifier.verifier
        );
        verify.run().await?;
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
    ) -> Result<Vec<DynSolValue>> {
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

impl figment::Provider for CreateArgs {
    fn metadata(&self) -> Metadata {
        Metadata::named("Create Args Provider")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        let mut dict = Dict::default();
        if let Some(timeout) = self.timeout {
            dict.insert("transaction_timeout".to_string(), timeout.into());
        }
        Ok(Map::from([(Config::selected_profile(), dict)]))
    }
}

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
    ) -> Result<(Address, AnyTransactionReceipt), ContractDeploymentError> {
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
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
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
/// // by the async `send` call
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
    ) -> Result<Deployer<B, P, T>, ContractDeploymentError>
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::I256;

    #[test]
    fn can_parse_create() {
        let args: CreateArgs = CreateArgs::parse_from([
            "foundry-cli",
            "src/Domains.sol:Domains",
            "--verify",
            "--retries",
            "10",
            "--delay",
            "30",
        ]);
        assert_eq!(args.retry.retries, 10);
        assert_eq!(args.retry.delay, 30);
    }
    #[test]
    fn can_parse_chain_id() {
        let args: CreateArgs = CreateArgs::parse_from([
            "foundry-cli",
            "src/Domains.sol:Domains",
            "--verify",
            "--retries",
            "10",
            "--delay",
            "30",
            "--chain-id",
            "9999",
        ]);
        assert_eq!(args.chain_id(), Some(9999));
    }

    #[test]
    fn test_parse_constructor_args() {
        let args: CreateArgs = CreateArgs::parse_from([
            "foundry-cli",
            "src/Domains.sol:Domains",
            "--constructor-args",
            "Hello",
        ]);
        let constructor: Constructor = serde_json::from_str(r#"{"type":"constructor","inputs":[{"name":"_name","type":"string","internalType":"string"}],"stateMutability":"nonpayable"}"#).unwrap();
        let params = args
            .parse_constructor_args(&constructor, &args.constructor_args)
            .unwrap();
        assert_eq!(params, vec![DynSolValue::String("Hello".to_string())]);
    }

    #[test]
    fn test_parse_tuple_constructor_args() {
        let args: CreateArgs = CreateArgs::parse_from([
            "foundry-cli",
            "src/Domains.sol:Domains",
            "--constructor-args",
            "[(1,2), (2,3), (3,4)]",
        ]);
        let constructor: Constructor = serde_json::from_str(r#"{"type":"constructor","inputs":[{"name":"_points","type":"tuple[]","internalType":"struct Point[]","components":[{"name":"x","type":"uint256","internalType":"uint256"},{"name":"y","type":"uint256","internalType":"uint256"}]}],"stateMutability":"nonpayable"}"#).unwrap();
        let _params = args
            .parse_constructor_args(&constructor, &args.constructor_args)
            .unwrap();
    }

    #[test]
    fn test_parse_int_constructor_args() {
        let args: CreateArgs = CreateArgs::parse_from([
            "foundry-cli",
            "src/Domains.sol:Domains",
            "--constructor-args",
            "-5",
        ]);
        let constructor: Constructor = serde_json::from_str(r#"{"type":"constructor","inputs":[{"name":"_name","type":"int256","internalType":"int256"}],"stateMutability":"nonpayable"}"#).unwrap();
        let params = args
            .parse_constructor_args(&constructor, &args.constructor_args)
            .unwrap();
        assert_eq!(
            params,
            vec![DynSolValue::Int(I256::unchecked_from(-5), 256)]
        );
    }
}
