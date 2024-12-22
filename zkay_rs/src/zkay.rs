#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::cmd::{call::CallArgs, compile::CompileArgs, create::CreateArgs, send::SendTxArgs};
use clap::{Parser, Subcommand, ValueHint};

use std::path::PathBuf;

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
