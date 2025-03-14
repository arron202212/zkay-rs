#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(dependency_on_unit_never_type_fallback)]
// #![feature(never_type)]
use alloy_network::{AnyNetwork, EthereumWallet};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::TransactionRequest;
use alloy_serde::WithOtherFields;
use alloy_signer::Signer;

use crate::blockchain::tx::{self, CastTxBuilder, SenderKind};
use crate::blockchain::{estimate::EstimateArgs, rpc::RpcArgs};
use alloy_dyn_abi::{DynSolValue, ErrorExt, EventExt};
use alloy_primitives::{Address, B256, eip191_hash_message, hex, keccak256};
use alloy_rpc_types::{BlockId, BlockNumberOrTag, BlockNumberOrTag::Latest};
use alloy_transport::Transport;
use cast::{Cast, SimpleCast};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, value_parser};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use eyre::Result;
use foundry_cli::handler;
use foundry_cli::{
    opts::{EthereumOpts, RpcOpts, TransactionOpts},
    utils,
};
use foundry_common::ens::NameOrAddress;
use foundry_common::{
    abi::get_event,
    ens::{ProviderEnsExt, namehash},
    fmt::{format_tokens, format_tokens_raw, format_uint_exp},
    fs,
    selectors::{
        ParsedSignatures, SelectorImportData, SelectorType, decode_calldata, decode_event_topic,
        decode_function_selector, decode_selectors, import_selectors, parse_signatures,
        pretty_calldata,
    },
    sh_println, sh_warn, shell, stdin,
};
use foundry_config::Config;
use itertools::Itertools;
use my_logging::log_context::log_context;
use std::str::FromStr;
use std::time::Instant;
use zkay_config::{config::library_compilation_environment, with_context_block};
use zkay_utils::progress_printer::{fail_print, success_print};

use std::path::{Path, PathBuf};
// use args::{Cast as CastArgs, CastSubcommand, ToBaseArgs};
use cast::traces::identifier::SignaturesIdentifier;
#[derive(Debug, Parser)]
pub enum Web3Subcommand {
    /// Get the balance of an account in wei.
    #[command(visible_alias = "b")]
    Balance {
        /// The block height to query at.
        ///
        /// Can also be the tags earliest, finalized, safe, latest, or pending.
        // #[arg(long, short = 'B')]
        block: Option<BlockId>,

        /// The account to query.
        // #[arg(value_parser = NameOrAddress::from_str)]
        who: NameOrAddress,

        /// Format the balance in ether.
        // #[arg(long, short)]
        ether: bool,

        #[command(flatten)]
        rpc: RpcOpts,

        /// erc20 address to query, with the method `balanceOf(address) return (uint256)`, alias
        /// with '--erc721'
        #[arg(long, alias = "erc721")]
        erc20: Option<Address>,
    },

    /// Get information about a block.
    // #[command(visible_alias = "bl")]
    Block {
        /// The block height to query at.
        ///
        /// Can also be the tags earliest, finalized, safe, latest, or pending.
        block: Option<BlockId>,

        /// If specified, only get the given field of the block.
        #[arg(long, short)]
        field: Option<String>,

        #[arg(long, env = "CAST_FULL_BLOCK")]
        full: bool,

        #[command(flatten)]
        rpc: RpcOpts,
    },

    /// Estimate the gas cost of a transaction.
    #[command(visible_alias = "e")]
    Estimate(EstimateArgs),

    /// Get the current gas price.
    #[command(visible_alias = "g")]
    GasPrice {
        #[command(flatten)]
        rpc: RpcOpts,
    },
    /// Perform a raw JSON-RPC request.
    #[command(visible_alias = "rp")]
    Rpc(RpcArgs),
    /// Get the runtime bytecode of a contract.
    #[command(visible_alias = "co")]
    Code {
        /// The block height to query at.
        ///
        /// Can also be the tags earliest, finalized, safe, latest, or pending.
        #[arg(long, short = 'B')]
        block: Option<BlockId>,

        /// The contract address.
        #[arg(value_parser = NameOrAddress::from_str)]
        who: NameOrAddress,

        /// Disassemble bytecodes.
        #[arg(long, short)]
        disassemble: bool,

        #[command(flatten)]
        rpc: RpcOpts,
    },
    /// Get the transaction receipt for a transaction.
    #[command(visible_alias = "re")]
    Receipt {
        /// The transaction hash.
        tx_hash: String,

        /// If specified, only get the given field of the transaction.
        field: Option<String>,

        /// The number of confirmations until the receipt is fetched
        #[arg(long, default_value = "1")]
        confirmations: u64,

        /// Exit immediately if the transaction was not found.
        #[arg(id = "async", long = "async", env = "CAST_ASYNC", alias = "cast-async")]
        cast_async: bool,

        #[command(flatten)]
        rpc: RpcOpts,
    },

    /// Convert an address to a checksummed format (EIP-55).
    #[command(
        visible_aliases = &["--to-checksum-address",
        "--to-checksum",
        "to-checksum",
        "ta",
        "2a"]
    )]
    ToCheckSumAddress {
        /// The address to convert.
        address: Option<Address>,
    },
    /// Hash arbitrary data using Keccak-256.
    #[command(visible_aliases = &["k", "keccak256"])]
    Keccak {
        /// The data to hash.
        data: Option<String>,
    },
}
#[derive(Clone, Default)]
pub struct Web3 {
    pub rpc: RpcOpts,
}

impl Web3 {
    pub async fn get_balance(&self, who: &str) -> String {
        let who = NameOrAddress::from_str(who).unwrap();
        run(Web3Subcommand::Balance {
            who,
            block: None,
            ether: false,
            rpc: RpcOpts::default(),
            erc20: None,
        })
        .await
    }
    pub async fn get_block(&self, block: &str, field: &str) -> String {
        let block: Option<BlockId> = BlockNumberOrTag::from_str(block).ok().map(|s| s.into());
        run(Web3Subcommand::Block {
            block,
            field: (!field.is_empty()).then(|| field.to_owned()),
            full: false,
            rpc: RpcOpts::default(),
        })
        .await
    }
    pub async fn estimate_gas(&self, to: &str, sig: &str, args: Vec<&str>) -> String {
        run(Web3Subcommand::Estimate(EstimateArgs::parse_from(
            vec!["foundry-cli", "--to", to, "--sig", sig, "--args"]
                .into_iter()
                .chain(args)
                .collect::<Vec<_>>(),
        )))
        .await
    }
    pub async fn gas_price(&self) -> String {
        run(Web3Subcommand::GasPrice {
            rpc: self.rpc.clone(),
        })
        .await
    }
    pub async fn eth_accounts(&self) -> Vec<String> {
        let value = run(Web3Subcommand::Rpc(RpcArgs::parse_from([
            "method",
            "eth_accounts",
        ])))
        .await;
        serde_json::from_str(&value).unwrap_or(vec![value])
    }
    pub async fn eth_coinbase(&self) -> String {
        run(Web3Subcommand::Rpc(RpcArgs::parse_from([
            "method",
            "eth_coinbase",
        ])))
        .await
    }
    pub async fn get_code(&self, who: &Address) -> String {
        self.get_codes(NameOrAddress::Address(who.clone())).await
    }
    pub async fn get_codes(&self, who: NameOrAddress) -> String {
        // let who = NameOrAddress::from_str(who).unwrap();
        run(Web3Subcommand::Code {
            who,
            block: None,
            disassemble: false,
            rpc: RpcOpts::default(),
        })
        .await
    }
    pub async fn wait_for_transaction_receipt(&self, tx_hash: &str) -> String {
        run(Web3Subcommand::Receipt {
            tx_hash: tx_hash.to_owned(),
            field: None,
            confirmations: 1,
            cast_async: false,
            rpc: RpcOpts::default(),
        })
        .await
    }
    pub async fn to_checksum_address(&self, address: &str) -> String {
        format!("{}", Address::from_str(address).unwrap().to_checksum(None))
    }
    pub async fn solidity_keccak(&self, bytes: Vec<u8>) -> String {
        match String::from_utf8(bytes) {
            Ok(s) => {
                let s = SimpleCast::keccak(&s).unwrap();
                format!("{s}")
            }
            Err(e) => {
                let hash = keccak256(e.as_bytes());
                let s = hex::encode(hash);
                format!("0x{s}")
            }
        }
    }
}

async fn run(cmd: Web3Subcommand) -> String {
    match cmd {
        Web3Subcommand::Balance {
            block,
            who,
            ether,
            rpc,
            erc20,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config).unwrap();
            let account_addr = who.resolve(&provider).await.unwrap();

            match erc20 {
                Some(token) => {
                    let balance = Cast::new(&provider)
                        .erc20_balance(token, account_addr, block)
                        .await
                        .unwrap();
                    format!("{}", format_uint_exp(balance))
                }
                None => {
                    let value = Cast::new(&provider)
                        .balance(account_addr, block)
                        .await
                        .unwrap();
                    if ether {
                        format!(
                            "{}",
                            SimpleCast::from_wei(&value.to_string(), "eth").unwrap()
                        )
                    } else {
                        format!("{value}")
                    }
                }
            }
        }
        Web3Subcommand::Block {
            block,
            full,
            field,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config).unwrap();
            format!(
                "{}",
                Cast::new(provider)
                    .block(block.unwrap_or(BlockId::Number(Latest)), full, field)
                    .await
                    .unwrap()
            )
        }
        Web3Subcommand::Estimate(cmd) => utils::block_on(cmd.run()).unwrap(),
        Web3Subcommand::GasPrice { rpc } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config).unwrap();
            format!("{}", Cast::new(provider).gas_price().await.unwrap())
        }
        Web3Subcommand::Rpc(cmd) => cmd.run().await.unwrap(),
        Web3Subcommand::Code {
            block,
            who,
            disassemble,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config).unwrap();
            let who = who.resolve(&provider).await.unwrap();
            format!(
                "{}",
                Cast::new(provider)
                    .code(who, block, disassemble)
                    .await
                    .unwrap()
            )
        }
        Web3Subcommand::Receipt {
            tx_hash,
            field,
            cast_async,
            confirmations,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config).unwrap();
            format!(
                "{}",
                Cast::new(provider)
                    .receipt(tx_hash, field, confirmations, None, cast_async)
                    .await
                    .unwrap()
            )
        }
        Web3Subcommand::ToCheckSumAddress { address } => {
            let value = stdin::unwrap_line(address).unwrap();
            format!("{}", value.to_checksum(None))
        }
        // Misc
        Web3Subcommand::Keccak { data } => {
            let bytes = match data {
                Some(data) => data.into_bytes(),
                None => stdin::read_bytes(false).unwrap(),
            };
            match String::from_utf8(bytes) {
                Ok(s) => {
                    let s = SimpleCast::keccak(&s).unwrap();
                    format!("{s}")
                }
                Err(e) => {
                    let hash = keccak256(e.as_bytes());
                    let s = hex::encode(hash);
                    format!("0x{s}")
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Web3Tx {
    pub eth: EthereumOpts,
    pub config: Config,
    pub tx: TransactionOpts,
}

impl Web3Tx {
    pub async fn new(eth: EthereumOpts, config: Config, tx: TransactionOpts) -> Result<Self> {
        Ok(Self { eth, config, tx })
    }

    pub async fn call(
        &self,
        to: Option<NameOrAddress>,
        sig: Option<String>,
        args: Vec<String>,
    ) -> Result<String> {
        let (code, block) = (None, None);
        let provider = utils::get_provider(&self.config)?;
        let sender = SenderKind::from_wallet_opts(self.eth.wallet.clone()).await?;
        // let from = sender.address();
        println!("======={sig:?},===== {args:?}==========call======{to:?}=====");
        let (tx, func) = CastTxBuilder::new(&provider, self.tx.clone(), &self.config)
            .await?
            .with_to(to)
            .await?
            .with_code_sig_and_args(code, sig, args)
            .await?
            .build_raw(sender)
            .await?;

        Ok(format!(
            "{}",
            Cast::new(provider).call(&tx, func.as_ref(), block).await?
        ))
    }

    pub async fn send(
        &self,
        to: Option<NameOrAddress>,
        sig: Option<String>,
        args: Vec<String>,
    ) -> Result<String> {
        let confirmations = 1;
        let unlocked = false;
        let cast_async = false;
        let (code, blob_data) = (None, None);
        let provider = utils::get_provider(&self.config)?;
        let builder = CastTxBuilder::new(&provider, self.tx.clone(), &self.config)
            .await?
            .with_to(to)
            .await?
            .with_code_sig_and_args(code, sig, args)
            .await?
            .with_blob_data(blob_data)?;

        let timeout = self.config.transaction_timeout; //timeout.unwrap_or(config.transaction_timeout);

        // Case 1:
        // Default to sending via eth_sendTransaction if the --unlocked flag is passed.
        // This should be the only way this RPC method is used as it requires a local node
        // or remote RPC with unlocked accounts.
        if unlocked {
            // only check current chain id if it was specified in the config
            if let Some(config_chain) = self.config.chain {
                let current_chain_id = provider.get_chain_id().await?;
                let config_chain_id = config_chain.id();
                // switch chain if current chain id is not the same as the one specified in the
                // config
                if config_chain_id != current_chain_id {
                    sh_warn!("Switching to chain {}", config_chain)?;
                    // let _x:Result<String,!>=provider
                    //     .raw_request(
                    //         "wallet_switchEthereumChain".into(),
                    //         [serde_json::json!({
                    //             "chainId": format!("0x{:x}", config_chain_id),
                    //         })],
                    //     )
                    //     .await?;
                }
            }

            let (tx, _) = builder.build(self.config.sender.clone()).await?;

            cast_send(provider, tx, cast_async, confirmations, timeout).await
        // Case 2:
        // An option to use a local signer was provided.
        // If we cannot successfully instantiate a local signer, then we will assume we don't have
        // enough information to sign and we must bail.
        } else {
            // Retrieve the signer, and bail if it can't be constructed.
            let signer = self.eth.wallet.signer().await?;
            let from = signer.address();

            tx::validate_from_address(self.eth.wallet.from, from)?;

            let (tx, _) = builder.build(&signer).await?;

            let wallet = EthereumWallet::from(signer);
            let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
                .wallet(wallet)
                .on_provider(&provider);

            cast_send(provider, tx, cast_async, confirmations, timeout).await
        }
    }
}

pub async fn call_tx<P: Provider<T, AnyNetwork>, T: Transport + Clone>(
    provider: P,
    config: &Config,
    to: Option<NameOrAddress>,
    sender: Address,
    tx: TransactionOpts,
    sig: Option<String>,
    args: Vec<String>,
) -> Result<String> {
    let (code, block) = (None, None);
    let (tx, func) = CastTxBuilder::new(&provider, tx, &config)
        .await?
        .with_to(to)
        .await?
        .with_code_sig_and_args(code, sig, args)
        .await?
        .build_raw(sender)
        .await?;
    Ok(format!(
        "{}",
        Cast::new(provider).call(&tx, func.as_ref(), block).await?
    ))
}

async fn send_tx<P: Provider<T, AnyNetwork>, T: Transport + Clone>(
    provider: P,
    config: &Config,
    to: Option<NameOrAddress>,
    tx: TransactionOpts,
    eth: EthereumOpts,
    sig: Option<String>,
    args: Vec<String>,
) -> Result<String> {
    let confirmations = 1;
    let unlocked = false;
    let cast_async = false;
    let (code, blob_data) = (None, None);
    let builder = CastTxBuilder::new(&provider, tx, &config)
        .await?
        .with_to(to)
        .await?
        .with_code_sig_and_args(code, sig, args)
        .await?
        .with_blob_data(blob_data)?;

    let timeout = config.transaction_timeout; //timeout.unwrap_or(config.transaction_timeout);

    // Case 1:
    // Default to sending via eth_sendTransaction if the --unlocked flag is passed.
    // This should be the only way this RPC method is used as it requires a local node
    // or remote RPC with unlocked accounts.
    if unlocked {
        // only check current chain id if it was specified in the config
        if let Some(config_chain) = config.chain {
            let current_chain_id = provider.get_chain_id().await?;
            let config_chain_id = config_chain.id();
            // switch chain if current chain id is not the same as the one specified in the
            // config
            if config_chain_id != current_chain_id {
                sh_warn!("Switching to chain {}", config_chain)?;
                //    let _x:Result<String,!>= provider
                //         .raw_request(
                //             "wallet_switchEthereumChain".into(),
                //             [serde_json::json!({
                //                 "chainId": format!("0x{:x}", config_chain_id),
                //             })],
                //         )
                //         .await?;
            }
        }

        let (tx, _) = builder.build(config.sender).await?;

        cast_send(provider, tx, cast_async, confirmations, timeout).await
    // Case 2:
    // An option to use a local signer was provided.
    // If we cannot successfully instantiate a local signer, then we will assume we don't have
    // enough information to sign and we must bail.
    } else {
        // Retrieve the signer, and bail if it can't be constructed.
        let signer = eth.wallet.signer().await?;
        let from = signer.address();

        tx::validate_from_address(eth.wallet.from, from)?;

        let (tx, _) = builder.build(&signer).await?;

        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::<_, _, AnyNetwork>::default()
            .wallet(wallet)
            .on_provider(&provider);

        cast_send(provider, tx, cast_async, confirmations, timeout).await
    }
}

async fn cast_send<P: Provider<T, AnyNetwork>, T: Transport + Clone>(
    provider: P,
    tx: WithOtherFields<TransactionRequest>,
    cast_async: bool,
    confs: u64,
    timeout: u64,
) -> Result<String> {
    let cast = Cast::new(provider);
    let pending_tx = cast.send(tx).await?;

    let tx_hash = pending_tx.inner().tx_hash();

    Ok(if cast_async {
        format!("{tx_hash:#x}")
    } else {
        let receipt = cast
            .receipt(format!("{tx_hash:#x}"), None, confs, Some(timeout), false)
            .await?;
        format!("{receipt}")
    })
}

fn value_or_string(value: String) -> serde_json::Value {
    serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web3_gas_price() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(!web3.gas_price().is_empty());
        assert_eq!(&web3.gas_price(), "2000000000");
    }
    #[test]
    fn test_web3_to_checksum_address() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(
            !web3
                .to_checksum_address("0x0F08283DB051c5BF85aCC4B91A257d87a0c58649")
                .is_empty()
        );
        assert_eq!(
            &web3.to_checksum_address("0x0F08283DB051c5BF85aCC4B91A257d87a0c58649"),
            "0x0F08283DB051c5BF85aCC4B91A257d87a0c58649"
        );
    }
    #[test]
    fn test_web3_solidity_keccak() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(!web3.solidity_keccak(b"test".into()).is_empty());
        assert_eq!(
            &web3.solidity_keccak(b"test".into()),
            "0x9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658"
        );
    }

    #[test]
    fn test_web3_get_balance() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(
            !web3
                .get_balance("0x0F08283DB051c5BF85aCC4B91A257d87a0c58649")
                .is_empty()
        );
        assert_eq!(
            &web3.get_balance("0x0F08283DB051c5BF85aCC4B91A257d87a0c58649"),
            "999996311260259907316"
        );
    }

    #[test]
    fn test_web3_eth_accounts() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(!web3.eth_accounts().is_empty());
        assert_eq!(
            &web3.eth_accounts(),
            "[\"0x0f08283db051c5bf85acc4b91a257d87a0c58649\",\"0x85372c241942d72ca13bd510d6e2b71f0f1c2959\",\"0x7b0b1a92ecd80da8f8a1340f24dfefed17dc2dbe\",\"0x195825ecaaddb2c0885d2505da2ba4383ab469fa\",\"0xb59152d3037f5d5363b5e29c6ad2cabe565136a2\",\"0xaf2c727ef24f6d5d61990e9b0e0ae65ad25b886f\",\"0x6b66e3200b1ac53e4c127048dfa0828549c95664\",\"0xbb25fc005ad3cf67064aee9bacef57952eb183c0\",\"0xcb586fc7da30c3799d02e336c1bc36ec9a9a2587\",\"0x3a49a5b08b1d03d253eab323b5e3920bc9f9d4d1\"]"
        );
    }
    #[test]
    fn test_web3_eth_coinbase() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(!web3.eth_coinbase().is_empty());
        assert_eq!(
            &web3.eth_coinbase(),
            "\"0x0000000000000000000000000000000000000000\""
        );
    }
    #[test]
    fn test_web3_get_block() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(!web3.get_block("pending").is_empty());
        println!("==={:?}", value_or_string(web3.get_block("pending")));
        assert_eq!(
            &web3.get_block("pending"),
            "\n\nbaseFeePerGas        277486607\ndifficulty           0\nextraData            0x\ngasLimit             30000000\ngasUsed              1514855\nhash                 0x506cc92e699aafe4c525e439ac6ccf53ef29a21b0924e6129f6b516cf07e21bc\nlogsBloom            0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\nminer                0x0000000000000000000000000000000000000000\nmixHash              0x6140959f13fdb37b2032266380f7ccceda3cce15c58d9761609379ab5e835107\nnonce                0x0000000000000000\nnumber               10\nparentHash           0x5f0f207e09738388092cebd6cfbc2d3dff6cf1774fb66a2db851ba14f7b357d6\nparentBeaconRoot     \ntransactionsRoot     0x297ac33459363e5d439a1498af6c6f3ddbf9f36bb2dd3032bc26476882303671\nreceiptsRoot         0xd2464457b6aad8b010531dddf20f00f697bf1a3c11f944051879e71ca4e43478\nsha3Uncles           0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347\nsize                 7334\nstateRoot            0xf8349d488b59ff73ad860cffa204faf1eede89c43b5b1889827be39e497c4acd\ntimestamp            1735204102 (Thu, 26 Dec 2024 09:08:22 +0000)\nwithdrawalsRoot      0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421\ntotalDifficulty      0\nblobGasUsed          \nexcessBlobGas        \nrequestsHash         \ntransactions:        [\n\t0xfffd43103fd54845d2b3a81e4ab776c6c1a89e5a97909890ac22c267ba24c80c\n]"
        );
    }
    #[test]
    fn test_web3_wait_for_transaction_receipt() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(
            !web3
                .wait_for_transaction_receipt(
                    "0xc619af4165c618a31b879d81562d951cffeb9f6e2d2dd6de313e4db0cbb8899f"
                )
                .is_empty()
        );
        assert_eq!(
            &web3.wait_for_transaction_receipt(
                "0xc619af4165c618a31b879d81562d951cffeb9f6e2d2dd6de313e4db0cbb8899f"
            ),
            "\nblockHash               0xab02791ba1264d119e6e854d2f87de11b23957d6df82e2f5df771e35d6f574f9\nblockNumber             1\ncontractAddress         0xa71526142E3105850B6b2a5dCa6A110E135924E0\ncumulativeGasUsed       827453\neffectiveGasPrice       875000001\nfrom                    0x0F08283DB051c5BF85aCC4B91A257d87a0c58649\ngasUsed                 827453\nlogs                    []\nlogsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\nroot                    \nstatus                  1 (success)\ntransactionHash         0xc619af4165c618a31b879d81562d951cffeb9f6e2d2dd6de313e4db0cbb8899f\ntransactionIndex        0\ntype                    2\nblobGasPrice            \nblobGasUsed             \nauthorizationList       "
        );
    }
    #[test]
    fn test_web3_get_code() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(
            !web3
                .get_code("0xa71526142e3105850b6b2a5dca6a110e135924e0")
                .is_empty()
        );
        assert_eq!(
            &web3.get_code("0xa71526142e3105850b6b2a5dca6a110e135924e0"),
            "0x608060405234801561001057600080fd5b50600436106100365760003560e01c8063077913dc1461003b5780637f1ae88214610199575b600080fd5b610197600480360361014081101561005257600080fd5b810190808061010001906008806020026040519081016040528092919082600860200280828437600092019190915250919493926020810192503590506401000000008111156100a157600080fd5b8201836020820111156100b357600080fd5b803590602001918460208302840111640100000000831117156100d557600080fd5b919080806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250929594936020810193503591505064010000000081111561012557600080fd5b82018360208201111561013757600080fd5b8035906020019184602083028401116401000000008311171561015957600080fd5b9190808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152509295506101b3945050505050565b005b6101a1610430565b60408051918252519081900360200190f35b8151601c146101c157600080fd5b80516019146101cf57600080fd5b6101d7610c71565b6040805180820182528551815260208087015181830152908352815160808082018452878401518285019081526060808a01519084015282528351808501855290880151815260a08801518184015281830152838201528151808301835260c0870151815260e08701519181019190915290820152610254610ca3565b61025c610454565b9050600060036002868660405160200180838051906020019060200280838360005b8381101561029657818101518382015260200161027e565b50505050905001828051906020019060200280838360005b838110156102c65781810151838201526020016102ae565b50505050905001925050506040516020818303038152906040526040518082805190602001908083835b6020831061030f5780518252601f1990920191602091820191016102f0565b51815160209384036101000a60001901801990921691161790526040519190930194509192505080830381855afa15801561034e573d6000803e3d6000fd5b5050506040513d602081101561036357600080fd5b5051901c9050610371610cea565b50608082015160208101516040909101516103979061039090846107cd565b8290610826565b6080840151519091506103ab908290610826565b90506103e6846000015185602001516103c384610875565b86604001516103d58960400151610875565b6060890151895160208b0151610901565b610427576040805162461bcd60e51b815260206004820152600d60248201526c34b73b30b634b210383937b7b360991b604482015290519081900360640190fd5b50505050505050565b7f343962316163396364623332363835643762656133333731313865663638393281565b61045c610ca3565b6040805180820182527f05021524417f47372e6981beedcaff2fb3cbfbaada41df8d2a497b71a147c9f681527f18c99b1392595b65a638a2fc06c6629e48d3e34134a49410519452ed325948ef6020808301919091529083528151608080820184527f12ec00b89f7a2a7b392f9f50d690b7ca965c2aa9dde784b766120ce3e973fa5f8285019081527f15677384fa4fee8c8899e77f98eb9841d15565588131c1742fa443803d8120cc606080850191909152908352845180860186527f2afed9d4db18534ffad6437d3bd84cc30a130bf05c25f3c4f2893a299547181381527f1edc7f5a2bf6cb146ae8fe5f1621d5cf00fdf119c753001e645c808f0975553a818601528385015285840192909252835180820185527f049c3788e9a93827355c44bfd9cac7f0034f0b386741994a18cd11d443b3ea2d8186019081527f17ef8e677f3c0a8c30a59e4c6212fef1b0f1ce1f9937a34eaeb5bd19992345c6828501528152845180860186527f24cccee589ab14ec56bb9052ecb74612d152a9cfcb5001bdf961bf123d18471081527f0b58e210e67510765ce865b8e3447951b9e14b73d859c73c691a342ff0d4a715818601528185015285850152835180820185527f1e258a803ce1f4ec1ab71b334feaadf7268ae6959e01426753fbd0118a6bd34c8186019081527f18399d2a1555d9a273140a1cbf59bf37decbfba9f576dd433eaada673a445214828501528152845180860186527f0c91fc8fbc46de9cd4f53dd1ad3f10c0610be8b02ce275596b1073f6e8553fca81527f0fa6b3beb33ce3acc95944741148f85d0a8ed6db7a99e30ce91c4d0609ed3120818601528185015291850191909152825180840184527f1eee7910efdfbb43dc92efc53b9009464637615cac027529919e28aa9520319181527f0bb8ae5fbeee5974fc1c25363cf3d47c6c056f481a2008b6a49f4df78aa66e7b81840152908401805191909152825180840184527f22ce1eb7c643eb8d4a7360b96771fa1438c742965358d8be217609064571070281527f2ac0d99dff0eda642f5cbddc95c61f0d7302d62d04d3451aebe18e0a2d639c78818401528151830152825180840184527f2ef4bc5171d89e4dd94fcce592b000e9a4978a97d4cd05d66e69174d20a1e2b681527f2defea16e9f494e9f95c3459b402110de91cde783e6dafe24013692a4da46c2992810192909252519091015290565b6107d5610cea565b6107dd610d04565b83518152602080850151908201526040810183905260006060836080848460076107d05a03f1905080801561081157610813565bfe5b508061081e57600080fd5b505092915050565b61082e610cea565b610836610d22565b8351815260208085015181830152835160408301528301516060808301919091526000908360c0848460066107d05a03f1905080801561081157610813565b61087d610cea565b81517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47901580156108b057506020830151155b156108d057505060408051808201909152600080825260208201526108fc565b604051806040016040528084600001518152602001828560200151816108f257fe5b0683038152509150505b919050565b60408051600480825260a0820190925260009160609190816020015b610925610cea565b81526020019060019003908161091d57505060408051600480825260a0820190925291925060609190602082015b61095b610d40565b8152602001906001900390816109535790505090508a8260008151811061097e57fe5b6020026020010181905250888260018151811061099757fe5b602002602001018190525086826002815181106109b057fe5b602002602001018190525084826003815181106109c957fe5b602002602001018190525089816000815181106109e257fe5b602002602001018190525087816001815181106109fb57fe5b60200260200101819052508581600281518110610a1457fe5b60200260200101819052508381600381518110610a2d57fe5b6020026020010181905250610a428282610a51565b9b9a5050505050505050505050565b60008151835114610a6157600080fd5b82516006810260608167ffffffffffffffff81118015610a8057600080fd5b50604051908082528060200260200182016040528015610aaa578160200160208202803683370190505b50905060005b83811015610c2f57868181518110610ac457fe5b602002602001015160000151828260060260000181518110610ae257fe5b602002602001018181525050868181518110610afa57fe5b602002602001015160200151828260060260010181518110610b1857fe5b602002602001018181525050858181518110610b3057fe5b602090810291909101015151518251839060026006850201908110610b5157fe5b602002602001018181525050858181518110610b6957fe5b60209081029190910101515160016020020151828260060260030181518110610b8e57fe5b602002602001018181525050858181518110610ba657fe5b602002602001015160200151600060028110610bbe57fe5b6020020151828260060260040181518110610bd557fe5b602002602001018181525050858181518110610bed57fe5b602002602001015160200151600160028110610c0557fe5b6020020151828260060260050181518110610c1c57fe5b6020908102919091010152600101610ab0565b50610c38610d60565b60006020826020860260208601600060086107d05a03f19050808015610811575080610c6357600080fd5b505115159695505050505050565b6040518060600160405280610c84610cea565b8152602001610c91610d40565b8152602001610c9e610cea565b905290565b6040518060a00160405280610cb6610cea565b8152602001610cc3610d40565b8152602001610cd0610d40565b8152602001610cdd610d40565b8152602001610c9e610d7e565b604051806040016040528060008152602001600081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280610d53610dab565b8152602001610c9e610dab565b60405180602001604052806001906020820280368337509192915050565b60405180606001604052806003905b610d95610cea565b815260200190600190039081610d8d5790505090565b6040518060400160405280600290602082028036833750919291505056fea26469706673582212202a8fa84b3dfd8c843698e3368debf20f71aabd8172952e541d47cb60f6e0f4f464736f6c634300060c0033"
        );
    }

    #[test]
    fn test_web3_estimate_gas() {
        let web3 = Web3 {
            rpc: RpcOpts::default(),
        };
        assert!(
            !web3
                .estimate_gas(
                    "0xa71526142e3105850b6b2a5dca6a110e135924e0",
                    "publish_results(uint[] calldata zk__out, uint[8] calldata zk__proof) external",
                    vec!["1", "12345678"]
                )
                .is_empty()
        );
        assert_eq!(
            &web3.estimate_gas("0xa71526142e3105850b6b2a5dca6a110e135924e0", "", vec![]),
            "0"
        );
    }
}
