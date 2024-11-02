// import json
// import os
// import tempfile
// from abc import abstractmethod
// from contextlib import contextmanager
// from pathlib import Path
// from typing import Any, Dict, Optional, Tuple, List, Union

// from eth_tester import PyEVMBackend, EthereumTester
// from web3 import Web3

// from zkay import my_logging
// from zkay.compiler.privacy import library_contracts
// from zkay.compiler.solidity.compiler import compile_solidity_json
// from zkay.config import cfg, zk_print, zk_print_banner
// from zkay.my_logging.log_context import log_context
// from zkay.transaction.crypto.params import CryptoParams
// from zkay.transaction.interface import ZkayBlockchainInterface, IntegrityError, BlockChainError, \
//     TransactionFailedException
// from zkay.transaction.types import PublicKeyValue, AddressValue, MsgStruct, BlockStruct, TxStruct
// from zkay.utils.helpers import get_contract_names, save_to_file
// from zkay.utils.timer import time_measure
// from zkay.zkay_ast.process_ast import get_verification_contract_names

// max_gas_limit = 10000000

// class Web3Blockchain(ZkayBlockchainInterface):
//     def __init__(self) -> None:
//         super().__init__()
//         self.w3 = self._create_w3_instance()
//         if not self.w3.is_connected():
//             raise BlockChainError(f'Failed to connect to blockchain: {self.w3.provider}')

//     @staticmethod
//     def compile_contract(sol_filename: str, contract_name: str, libs: Optional[Dict] = None, cwd=None):
//         solp = Path(sol_filename)
//         jout = compile_solidity_json(sol_filename, libs, optimizer_runs=cfg.opt_solc_optimizer_runs, cwd=cwd)['contracts'][solp.name][contract_name]
//         return {
//             'abi': json.loads(jout['metadata'])['output']['abi'],
//             'bin': jout['evm']['bytecode']['object'],
//             'deployed_bin': jout['evm']['deployedBytecode']['object']
//         }

//     def deploy_solidity_contract(self, sol_filename: str, contract_name: Optional[str], sender: Union[bytes, str]) -> str:
//         contract_name = get_contract_names(sol_filename)[0] if contract_name is None else contract_name
//         contract = self._deploy_contract(sender, self.compile_contract(sol_filename, contract_name))
//         return str(contract.address)

//     def get_special_variables(self, sender: AddressValue, wei_amount: int = 0) -> Tuple[MsgStruct, BlockStruct, TxStruct]:
//         block = self.w3.eth.get_block('pending')
//         zk_print(f'Current block timestamp: {block["timestamp"]}')
//         return MsgStruct(sender, wei_amount), \
//                BlockStruct(AddressValue(self.w3.eth.coinbase), block['difficulty'], block['gasLimit'], block['number'], block['timestamp']),\
//                TxStruct(self.w3.eth.gas_price, sender)

//     @abstractmethod
//     def _create_w3_instance(self) -> Web3:
//         pass

//     def _default_address(self) -> Union[None, bytes, str]:
//         if cfg.blockchain_default_account is None:
//             return None
//         elif isinstance(cfg.blockchain_default_account, int):
//             return self.w3.eth.accounts[cfg.blockchain_default_account]
//         else:
//             return cfg.blockchain_default_account

//     def _get_balance(self, address: Union[bytes, str]) -> int:
//         return self.w3.eth.get_balance(address)

//     def _req_public_key(self, address: Union[bytes, str], crypto_params: CryptoParams) -> PublicKeyValue:
//         return PublicKeyValue(self._req_state_var(self.pki_contract(crypto_params.crypto_name), 'getPk', address),
//                               params=crypto_params)

//     def _announce_public_key(self, address: Union[bytes, str], pk: Tuple[int, ...], crypto_params: CryptoParams) -> Any:
//         with log_context(f'announcePk'):
//             return self._transact(self.pki_contract(crypto_params.crypto_name), address, 'announcePk', pk)

//     def _req_state_var(self, contract_handle, name: str, *indices) -> Any:
//         try:
//             return contract_handle.functions[name](*indices).call()
//         except Exception as e:
//             raise BlockChainError(e.args)

//     def _call(self, contract_handle, sender: Union[bytes, str], name: str, *args) -> Union[bool, int, str]:
//         try:
//             fct = contract_handle.functions[name]
//             gas_amount = self._gas_heuristic(sender, fct(*args))
//             tx = {'from': sender, 'gas': gas_amount}
//             return fct(*args).call(tx)
//         except Exception as e:
//             raise BlockChainError(e.args)

//     def _transact(self, contract_handle, sender: Union[bytes, str], function: str, *actual_params, wei_amount: Optional[int] = None) -> Any:
//         try:
//             fct = contract_handle.constructor if function == 'constructor' else contract_handle.functions[function]
//             gas_amount = self._gas_heuristic(sender, fct(*actual_params))
//             tx = {'from': sender, 'gas': gas_amount}
//             if wei_amount:
//                 tx['value'] = wei_amount
//             tx_hash = fct(*actual_params).transact(tx)
//             tx_receipt = self.w3.eth.wait_for_transaction_receipt(tx_hash)
//         except Exception as e:
//             raise BlockChainError(e.args)

//         if tx_receipt['status'] == 0:
//             raise TransactionFailedException("Transaction failed")
//         gas = tx_receipt['gasUsed']
//         zk_print(f"Consumed gas: {gas}")
//         my_logging.data('gas', gas)
//         return tx_receipt

//     def _deploy(self, project_dir: str, sender: Union[bytes, str], contract: str, *actual_args, wei_amount: Optional[int] = None) -> Any:
//         with open(os.path.join(project_dir, 'contract.zkay')) as f:
//             verifier_names = get_verification_contract_names(f.read())

//         # Deploy verification contracts if not already done
//         external_contract_addresses =  self._deploy_dependencies(sender, project_dir, verifier_names)
//         with self.__hardcoded_external_contracts_ctx(project_dir, external_contract_addresses) as filename:
//             cout = self.compile_contract(filename, contract, cwd=project_dir)
//         with log_context('constructor'):
//             with log_context(f'{contract}'):
//                 handle = self._deploy_contract(sender, cout, *actual_args, wei_amount=wei_amount)
//         zk_print(f'Deployed contract "{contract}" at address "{handle.address}"')
//         return handle

//     def _deploy_contract(self, sender: Union[bytes, str], contract_interface, *args, wei_amount: Optional[int] = None):
//         if args is None:
//             args = []

//         contract = self.w3.eth.contract(
//             abi=contract_interface['abi'],
//             bytecode=contract_interface['bin']
//         )

//         tx_receipt = self._transact(contract, sender, 'constructor', *args, wei_amount=wei_amount)
//         contract = self.w3.eth.contract(
//             address=tx_receipt.contractAddress, abi=contract_interface['abi']
//         )
//         return contract

//     def _deploy_dependencies(self, sender: Union[bytes, str], project_dir: str, verifier_names: List[str]) -> Dict[str, AddressValue]:
//         # Deploy verification contracts if not already done
//         vf = {}
//         for verifier_name in verifier_names:
//             with log_context('constructor'):
//                 with log_context(f'{verifier_name}'):
//                     filename = os.path.join(project_dir, f'{verifier_name}.sol')
//                     cout = self.compile_contract(filename, verifier_name, self.lib_addresses)
//                     with time_measure("transaction_full"):
//                         vf[verifier_name] = AddressValue(self._deploy_contract(sender, cout).address)
//         for crypto_params in cfg.all_crypto_params():
//             pki_contract_name = cfg.get_pki_contract_name(crypto_params)
//             pki_contract_address = self.pki_contract(crypto_params.crypto_name).address
//             vf[pki_contract_name] = AddressValue(pki_contract_address)
//         return vf

//     def _connect_libraries(self):
//         if not cfg.blockchain_pki_address:
//             raise BlockChainError('Must specify pki address in config.')

//         lib_addresses = []
//         if cfg.external_crypto_lib_names:
//             lib_addresses = [addr.strip() for addr in cfg.blockchain_crypto_lib_addresses.split(',')] if cfg.blockchain_crypto_lib_addresses else []
//             if len(lib_addresses) != len(cfg.external_crypto_lib_names):
//                 raise BlockChainError('Must specify all crypto library addresses in config\n'
//                                       f'Expected {len(cfg.external_crypto_lib_names)} was {len(lib_addresses)}')

//         with cfg.library_compilation_environment():
//             with tempfile.TemporaryDirectory() as tmpdir:
//                 for crypto_params in cfg.all_crypto_params():
//                     contract_name = cfg.get_pki_contract_name(crypto_params)
//                     pki_sol = save_to_file(tmpdir, f'{contract_name}.sol', library_contracts.get_pki_contract(crypto_params))
//                     self._pki_contract = self._verify_contract_integrity(cfg.blockchain_pki_address, pki_sol, contract_name=contract_name)

//                 verify_sol = save_to_file(tmpdir, 'verify_libs.sol', library_contracts.get_verify_libs_code())
//                 self._lib_addresses = {}
//                 for lib, addr in zip(cfg.external_crypto_lib_names, lib_addresses):
//                     out = self._verify_contract_integrity(addr, verify_sol, contract_name=lib, is_library=True)
//                     self._lib_addresses[lib] = out.address

//     def _connect(self, project_dir: str, contract: str, address: Union[bytes, str]) -> Any:
//         filename = os.path.join(project_dir, 'contract.sol')
//         cout = self.compile_contract(filename, contract)
//         return self.w3.eth.contract(
//             address=address, abi=cout['abi']
//         )

//     def _verify_contract_integrity(self, address: Union[bytes, str], sol_filename: str, *,
//                                    libraries: Dict = None, contract_name: str = None, is_library: bool = False,
//                                    cwd=None) -> Any:
//         if isinstance(address, bytes):
//             address = self.w3.toChecksumAddress(address)

//         if contract_name is None:
//             contract_name = get_contract_names(sol_filename)[0]
//         actual_byte_code = self.__normalized_hex(self.w3.eth.get_code(address))
//         if not actual_byte_code:
//             raise IntegrityError(f'Expected contract {contract_name} is not deployed at address {address}')

//         cout = self.compile_contract(sol_filename, contract_name, libs=libraries, cwd=cwd)
//         expected_byte_code = self.__normalized_hex(cout['deployed_bin'])

//         if is_library:
//             # https://github.com/ethereum/solidity/issues/7101
//             expected_byte_code = expected_byte_code[:2] + self.__normalized_hex(address) + expected_byte_code[42:]

//         if actual_byte_code != expected_byte_code:
//             raise IntegrityError(f'Deployed contract at address {address} does not match local contract {sol_filename}')
//         zk_print(f'Contract@{address} matches {sol_filename[sol_filename.rfind("/") + 1:]}:{contract_name}')

//         return self.w3.eth.contract(
//             address=address, abi=cout['abi']
//         )

//     def _verify_library_integrity(self, libraries: List[Tuple[str, str]], contract_with_libs_addr: str, sol_with_libs_filename: str) -> Dict[str, str]:
//         cname = get_contract_names(sol_with_libs_filename)[0]
//         actual_code = self.__normalized_hex(self.w3.eth.getCode(contract_with_libs_addr))
//         if not actual_code:
//             raise IntegrityError(f'Expected contract {cname} is not deployed at address {contract_with_libs_addr}')
//         code_with_placeholders = self.__normalized_hex(self.compile_contract(sol_with_libs_filename, cname)['deployed_bin'])

//         if len(actual_code) != len(code_with_placeholders):
//             raise IntegrityError(f'Local code of contract {cname} has different length than remote contract')

//         addresses = {}
//         for lib_name, lib_sol in libraries:
//             # Compute placeholder according to
//             # https://solidity.readthedocs.io/en/v0.5.13/using-the-compiler.html#using-the-commandline-compiler
//             hash = self.w3.solidityKeccak(['string'], [f'{lib_sol[lib_sol.rfind("/") + 1:]}:{lib_name}'])
//             placeholder = f'__${self.__normalized_hex(hash)[:34]}$__'

//             # Retrieve concrete address in deployed code at placeholder offset in local code and verify library contract integrity
//             lib_address_offset = code_with_placeholders.find(placeholder)
//             if lib_address_offset != -1:
//                 lib_address = self.w3.toChecksumAddress(actual_code[lib_address_offset:lib_address_offset+40])
//                 with cfg.library_compilation_environment():
//                     self._verify_contract_integrity(lib_address, lib_sol, contract_name=lib_name, is_library=True)
//                 addresses[lib_name] = lib_address
//         return addresses

//     def _verify_zkay_contract_integrity(self, address: str, project_dir: str, pki_verifier_addresses: Dict):
//         with self.__hardcoded_external_contracts_ctx(project_dir, pki_verifier_addresses) as sol_file:
//             self._verify_contract_integrity(address, sol_file, cwd=project_dir)

//     @contextmanager
//     def __hardcoded_external_contracts_ctx(self, contract_dir: str, pki_verifier_addresses):
//         # Hardcode contract addresses
//         with open(os.path.join(contract_dir, 'contract.sol')) as f:
//             c = f.read()
//         for key, val in pki_verifier_addresses.items():
//             c = c.replace(f'{key}(0)', f'{key}({self.w3.toChecksumAddress(val.val)})')

//         with tempfile.TemporaryDirectory() as tempd:
//             # Save in temporary file to compile
//             output_filename = os.path.join(tempd, "contract.inst.sol")
//             with open(output_filename, 'w') as f:
//                 f.write(c)
//             yield output_filename
//             pass

//     def __normalized_hex(self, val: Union[str, bytes]) -> str:
//         if not isinstance(val, str):
//             val = val.hex()
//         val = val[2:] if val.startswith('0x') else val
//         return val.lower()

//     def _gas_heuristic(self, sender, tx) -> int:
//         limit = self.w3.eth.get_block('latest')['gasLimit']
//         estimate = tx.estimateGas({'from': sender, 'gas': limit})
//         return min(int(estimate * 1.2), limit)

// class Web3TesterBlockchain(Web3Blockchain):
//     def __init__(self) -> None:
//         self.eth_tester = None
//         super().__init__()
//         self.next_acc_idx = 1

//     @classmethod
//     def is_debug_backend(cls) -> bool:
//         return True

//     def _connect_libraries(self):
//         zk_print_banner(f'Deploying Libraries')

//         sender = self.w3.eth.accounts[0]
//         # Since eth-tester is not persistent -> always automatically deploy libraries
//         with cfg.library_compilation_environment():
//             with tempfile.TemporaryDirectory() as tmpdir:
//                 with log_context('deploy_pki'):
//                     self._pki_contract = {}
//                     for crypto_params in cfg.all_crypto_params():
//                         with log_context(crypto_params.crypto_name):
//                             pki_contract_code = library_contracts.get_pki_contract(crypto_params)
//                             pki_contract_name = cfg.get_pki_contract_name(crypto_params)
//                             pki_sol = save_to_file(tmpdir, f'{pki_contract_name}.sol', pki_contract_code)
//                             contract = self._deploy_contract(sender, self.compile_contract(pki_sol, pki_contract_name))
//                             backend_name = crypto_params.crypto_name
//                             self._pki_contract[backend_name] = contract
//                             zk_print(f'Deployed pki contract for crypto back-end {backend_name} at address "{contract.address}"')

//                 with log_context('deploy_verify_libs'):
//                     verify_sol = save_to_file(tmpdir, 'verify_libs.sol', library_contracts.get_verify_libs_code())
//                     self._lib_addresses = {}
//                     for lib in cfg.external_crypto_lib_names:
//                         out = self._deploy_contract(sender, self.compile_contract(verify_sol, lib))
//                         self._lib_addresses[lib] = out.address
//                         zk_print(f'Deployed crypto lib {lib} at address "{out.address}"')

//     def _create_w3_instance(self) -> Web3:
//         genesis_overrides = {'gas_limit': int(max_gas_limit * 1.2)}
//         custom_genesis_params = PyEVMBackend._generate_genesis_params(overrides=genesis_overrides)
//         self.eth_tester = EthereumTester(backend=PyEVMBackend(genesis_parameters=custom_genesis_params))
//         w3 = Web3(Web3.EthereumTesterProvider(self.eth_tester))
//         return w3

//     def create_test_accounts(self, count: int) -> Tuple:
//         accounts = self.w3.eth.accounts
//         if len(accounts[self.next_acc_idx:]) < count:
//             raise ValueError(f'Can have at most {len(accounts)-1} dummy accounts in total')
//         dummy_accounts = tuple(accounts[self.next_acc_idx:self.next_acc_idx + count])
//         self.next_acc_idx += count
//         return dummy_accounts

//     def _gas_heuristic(self, sender, tx) -> int:
//         return max_gas_limit

// class Web3IpcBlockchain(Web3Blockchain):
//     def _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.IPCProvider(cfg.blockchain_node_uri))

// class Web3WebsocketBlockchain(Web3Blockchain):
//     def _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.WebsocketProvider(cfg.blockchain_node_uri))

// class Web3HttpBlockchain(Web3Blockchain):
//     def _create_w3_instance(self) -> Web3:
//         assert cfg.blockchain_node_uri is None or isinstance(cfg.blockchain_node_uri, str)
//         return Web3(Web3.HTTPProvider(cfg.blockchain_node_uri))

// class Web3HttpGanacheBlockchain(Web3HttpBlockchain):
//     def __init__(self) -> None:
//         super().__init__()
//         self.next_acc_idx = 1

//     @classmethod
//     def is_debug_backend(cls) -> bool:
//         return True

//     def create_test_accounts(self, count: int) -> Tuple:
//         accounts = self.w3.eth.accounts
//         if len(accounts[self.next_acc_idx:]) < count:
//             raise ValueError(f'Can have at most {len(accounts)-1} dummy accounts in total')
//         dummy_accounts = tuple(accounts[self.next_acc_idx:self.next_acc_idx + count])
//         self.next_acc_idx += count
//         return dummy_accounts

//     def _gas_heuristic(self, sender, tx) -> int:
//         return self.w3.eth.get_block('latest')['gasLimit']

// class Web3CustomBlockchain(Web3Blockchain):
//     def _create_w3_instance(self) -> Web3:
//         assert isinstance(cfg.blockchain_node_uri, Web3)
//         return cfg.blockchain_node_uri

use alloy_chains::Chain;
use alloy_dyn_abi::{DynSolValue, JsonAbiExt, Specifier};
use alloy_json_abi::{Constructor, JsonAbi};
use alloy_network::{AnyNetwork, EthereumWallet, TransactionBuilder};
use alloy_primitives::{hex, Address, Bytes};
use alloy_provider::{PendingTransactionError, Provider, ProviderBuilder};
use alloy_rpc_types::{AnyTransactionReceipt, TransactionRequest};
use alloy_serde::WithOtherFields;
use alloy_signer::Signer;
use alloy_transport::{Transport, TransportError};
use clap::{Parser, ValueHint};
use eyre::{Context, Result};
use forge_verify::RetryArgs;
use foundry_cli::{
    opts::{CoreBuildArgs, EthereumOpts, EtherscanOpts, TransactionOpts},
    utils::{self, read_constructor_args_file, remove_contract, LoadConfig},
};
use foundry_common::{
    compile::{self},
    fmt::parse_tokens,
};
use foundry_compilers::{artifacts::BytecodeObject, info::ContractInfo, utils::canonicalize};
use foundry_config::{
    figment::{
        self,
        value::{Dict, Map},
        Metadata, Profile,
    },
    merge_impl_figment_convert, Config,
};
use serde_json::json;
use std::{borrow::Borrow, marker::PhantomData, path::PathBuf, sync::Arc};

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
}

impl CreateArgs {
    /// Executes the command to create a contract
    pub async fn run(mut self) -> Result<()> {
        let config = self.try_load_config_emit_warnings()?;
        // Find Project & Compile
        let project = config.project()?;

        let target_path = if let Some(ref mut path) = self.contract.path {
            canonicalize(project.root().join(path))?
        } else {
            project.find_contract_path(&self.contract.name)?
        };

        let mut output = compile::compile_target(&target_path, &project, self.json)?;

        let (abi, bin, _) = remove_contract(&mut output, &target_path, &self.contract.name)?;

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
                eyre::bail!("Dynamic linking not supported in `create` command - deploy the following library contracts first, then provide the address to link at compile time\n{}", link_refs)
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
            )
            .await
        }
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
    ) -> Result<()> {
        // NOTE: this does not represent the same `VerifyArgs` that would be sent after deployment,
        // since we don't know the address yet.
        let mut verify = forge_verify::VerifyArgs {
            address: Default::default(),
            contract: Some(self.contract.clone()),
            compiler_version: None,
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
    ) -> Result<()> {
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

            self.verify_preflight_check(constructor_args.clone(), chain)
                .await?;
        }

        // Deploy the actual contract
        let (deployed_contract, receipt) = deployer.send_with_receipt().await?;

        let address = deployed_contract;
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
            return Ok(());
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
        };
        println!(
            "Waiting for {} to detect contract deployment...",
            verify.verifier.verifier
        );
        verify.run().await
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
