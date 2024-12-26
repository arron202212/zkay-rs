#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::cmd::{
    call::CallArgs, compile::CompileArgs, create::CreateArgs, estimate::EstimateArgs, rpc::RpcArgs,
    send::SendTxArgs,
};
// use crate::cmd::{
//     access_list::AccessListArgs, artifact::ArtifactArgs, bind::BindArgs, call::CallArgs,
//     constructor_args::ConstructorArgsArgs, create2::Create2Args, creation_code::CreationCodeArgs,
//     estimate::EstimateArgs, find_block::FindBlockArgs, interface::InterfaceArgs, logs::LogsArgs,
//     mktx::MakeTxArgs, rpc::RpcArgs, run::RunArgs, send::SendTxArgs, storage::StorageArgs,
//     wallet::WalletSubcommands,
// };
use alloy_primitives::{Address, B256, U256};
use alloy_rpc_types::BlockId;
use eyre::Result;
use foundry_cli::opts::{EtherscanOpts, GlobalOpts, RpcOpts};
use foundry_common::ens::NameOrAddress;
use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand, ValueHint};

const VERSION_MESSAGE: &str = env!("ZKAY_VERSION");

/// Build, test, fuzz, debug and deploy Solidity contracts.
#[derive(Parser)]
#[command(
    name = "zkay",
    version = VERSION_MESSAGE,
    after_help = "Find more information in the book: http://book.getfoundry.sh/reference/forge/forge.html",
    next_display_order = None,
)]
pub struct Zkay {
    #[command(subcommand)]
    pub cmd: ZkaySubcommand,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum ZkaySubcommand {
    /// Compile the project's smart contracts.
    #[command(visible_aliases = ["z", "compilez"])]
    Compile(CompileArgs),

    /// Deploy a smart contract.
    #[command(visible_alias = "d")]
    Create(CreateArgs),

    /// Sign and publish a transaction.
    #[command(name = "send", visible_alias = "s")]
    SendTx(SendTxArgs),

    /// Perform a call on an account without publishing a transaction.
    #[command(visible_alias = "c")]
    Call(CallArgs),

    /// Get information about a block.
    #[command(visible_alias = "bl")]
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

    /// Get the balance of an account in wei.
    #[command(visible_alias = "b")]
    Balance {
        /// The block height to query at.
        ///
        /// Can also be the tags earliest, finalized, safe, latest, or pending.
        #[arg(long, short = 'B')]
        block: Option<BlockId>,

        /// The account to query.
        #[arg(value_parser = NameOrAddress::from_str)]
        who: NameOrAddress,

        /// Format the balance in ether.
        #[arg(long, short)]
        ether: bool,

        #[command(flatten)]
        rpc: RpcOpts,

        /// erc20 address to query, with the method `balanceOf(address) return (uint256)`, alias
        /// with '--erc721'
        #[arg(long, alias = "erc721")]
        erc20: Option<Address>,
    },

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
    /// Perform a raw JSON-RPC request.
    #[command(visible_alias = "rp")]
    Rpc(RpcArgs),
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Zkay::command().debug_assert();
    }
}
