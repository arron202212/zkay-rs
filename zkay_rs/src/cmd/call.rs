#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use zkay_config::{config_user::UserConfig,config::CFG};
use crate::tx::{CastTxBuilder, SenderKind};
use alloy_primitives::{TxKind, U256};
use alloy_rpc_types::{BlockId, BlockNumberOrTag};
use cast::{Cast, traces::TraceKind};
use clap::Parser;
use eyre::Result;
use foundry_cli::{
    opts::{EthereumOpts, TransactionOpts},
    utils::{self, TraceResult, handle_traces, parse_ether_value},
};
use foundry_common::ens::NameOrAddress;
use foundry_common::{sh_println, shell};
use foundry_compilers::artifacts::EvmVersion;
use foundry_config::{
    Config,
    figment::{
        self, Figment, Metadata, Profile,
        value::{Dict, Map},
    },
};
use foundry_evm::{executors::TracingExecutor, opts::EvmOpts};
use std::str::FromStr;
use zkay_transaction::blockchain::web3::{Web3, Web3Tx};

/// CLI arguments for `cast call`.
#[derive(Debug, Parser)]
pub struct CallArgs {
    /// The destination of the transaction.
    #[arg(value_parser = NameOrAddress::from_str)]
    to: Option<NameOrAddress>,

    /// The signature of the function to call.
    sig: Option<String>,

    /// The arguments of the function to call.
    args: Vec<String>,

    /// Data for the transaction.
    #[arg(
        long,
        conflicts_with_all = &["sig", "args"]
    )]
    data: Option<String>,

    /// Forks the remote rpc, executes the transaction locally and prints a trace
    #[arg(long, default_value_t = false)]
    trace: bool,

    /// Opens an interactive debugger.
    /// Can only be used with `--trace`.
    #[arg(long, requires = "trace")]
    debug: bool,

    #[arg(long, requires = "trace")]
    decode_internal: bool,

    /// Labels to apply to the traces; format: `address:label`.
    /// Can only be used with `--trace`.
    #[arg(long, requires = "trace")]
    labels: Vec<String>,

    #[arg(long, allow_hyphen_values = true, value_delimiter = ',')]
    blockchain_pki_addresses: Vec<String>,

    /// The EVM Version to use.
    /// Can only be used with `--trace`.
    #[arg(long, requires = "trace")]
    evm_version: Option<EvmVersion>,

    /// The block height to query at.
    ///
    /// Can also be the tags earliest, finalized, safe, latest, or pending.
    #[arg(long, short)]
    block: Option<BlockId>,

    /// Enable Alphanet features.
    #[arg(long, alias = "odyssey")]
    pub alphanet: bool,

    #[command(subcommand)]
    command: Option<CallSubcommands>,

    #[command(flatten)]
    tx: TransactionOpts,

    #[command(flatten)]
    eth: EthereumOpts,

    #[arg(id = "survey", long = "survey", alias = "is-survey")]
    is_survey: bool,
}

#[derive(Debug, Parser)]
pub enum CallSubcommands {
    /// ignores the address field and simulates creating a contract
    #[command(name = "--create")]
    Create {
        /// Bytecode of contract.
        code: String,

        /// The signature of the constructor.
        sig: Option<String>,

        /// The arguments of the constructor.
        args: Vec<String>,

        /// Ether to send in the transaction.
        ///
        /// Either specified in wei, or as a string with a unit type.
        ///
        /// Examples: 1ether, 10gwei, 0.01ether
        #[arg(long, value_parser = parse_ether_value)]
        value: Option<U256>,
    },
}

impl CallArgs {
    pub async fn run(self) -> Result<()> {
        let figment = Into::<Figment>::into(&self.eth).merge(&self);
        let evm_opts = figment.extract::<EvmOpts>()?;
        let mut config = Config::try_from(figment)?.sanitized();

        let Self {
            to,
            mut sig,
            mut args,
            mut tx,
            eth,
            command,
            block,
            trace,
            evm_version,
            debug,
            decode_internal,
            labels,
            data,
            blockchain_pki_addresses,
            ..
        } = self;
        println!(
            "=====is_json==={}========{}",
            Web3::default().get_block("pending", "").await,
            shell::is_json()
        );
        if let Some(data) = data {
            sig = Some(data);
        }

        let provider = utils::get_provider(&config)?;
        let sender = SenderKind::from_wallet_opts(eth.wallet.clone()).await?;
        let from = sender.address();
        let web3tx = Web3Tx::new(eth.clone(), config.clone(), tx.clone()).await?;
        if self.is_survey {
            CFG.lock().unwrap().set_blockchain_pki_address(blockchain_pki_addresses);
            return crate::contract::main0(web3tx).await
        }
        let code = if let Some(CallSubcommands::Create {
            code,
            sig: create_sig,
            args: create_args,
            value,
        }) = command
        {
            sig = create_sig;
            args = create_args;
            if let Some(value) = value {
                tx.value = Some(value);
            }
            Some(code)
        } else {
            None
        };

        let (tx, func) = CastTxBuilder::new(&provider, tx, &config)
            .await?
            .with_to(to)
            .await?
            .with_code_sig_and_args(code, sig, args)
            .await?
            .build_raw(sender)
            .await?;

        if trace {
            if let Some(BlockId::Number(BlockNumberOrTag::Number(block_number))) = self.block {
                // Override Config `fork_block_number` (if set) with CLI value.
                config.fork_block_number = Some(block_number);
            }

            let (mut env, fork, chain, alphanet) =
                TracingExecutor::get_fork_material(&config, evm_opts).await?;

            // modify settings that usually set in eth_call
            env.cfg.disable_block_gas_limit = true;
            env.block.gas_limit = U256::MAX;

            let mut executor =
                TracingExecutor::new(env, fork, evm_version, debug, decode_internal, alphanet);

            let value = tx.value.unwrap_or_default();
            let input = tx.inner.input.into_input().unwrap_or_default();
            let tx_kind = tx.inner.to.expect("set by builder");

            let trace = match tx_kind {
                TxKind::Create => {
                    let deploy_result = executor.deploy(from, input, value, None);
                    TraceResult::try_from(deploy_result)?
                }
                TxKind::Call(to) => TraceResult::from_raw(
                    executor.transact_raw(from, to, input, value)?,
                    TraceKind::Execution,
                ),
            };

            handle_traces(trace, &config, chain, labels, debug, decode_internal, false).await?;

            return Ok(());
        }

        sh_println!(
            "{}",
            Cast::new(provider).call(&tx, func.as_ref(), block).await?
        )?;

        Ok(())
    }
}

impl figment::Provider for CallArgs {
    fn metadata(&self) -> Metadata {
        Metadata::named("CallArgs")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        let mut map = Map::new();

        if self.alphanet {
            map.insert("alphanet".into(), self.alphanet.into());
        }

        if let Some(evm_version) = self.evm_version {
            map.insert(
                "evm_version".into(),
                figment::value::Value::serialize(evm_version)?,
            );
        }

        Ok(Map::from([(Config::selected_profile(), map)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, hex};

    #[test]
    fn can_parse_call_data() {
        let data = hex::encode("hello");
        let args = CallArgs::parse_from(["foundry-cli", "--data", data.as_str()]);
        assert_eq!(args.data, Some(data));

        let data = hex::encode_prefixed("hello");
        let args = CallArgs::parse_from(["foundry-cli", "--data", data.as_str()]);
        assert_eq!(args.data, Some(data));
    }

    #[test]
    fn call_sig_and_data_exclusive() {
        let data = hex::encode("hello");
        let to = Address::ZERO;
        let args = CallArgs::try_parse_from([
            "foundry-cli",
            to.to_string().as_str(),
            "signature",
            "--data",
            format!("0x{data}").as_str(),
        ]);

        assert!(args.is_err());
    }
}
