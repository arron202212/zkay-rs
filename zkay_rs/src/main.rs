#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// #!/usr/bin/env python3
// // PYTHON_ARGCOMPLETE_OK
// import argcomplete, argparse
// import os
use alloy_dyn_abi::{DynSolValue, ErrorExt, EventExt};
use alloy_primitives::{Address, B256, eip191_hash_message, hex, keccak256};
use alloy_provider::Provider;
use alloy_rpc_types::{BlockId, BlockNumberOrTag::Latest};
use cast::{Cast, SimpleCast};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, value_parser};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use eyre::Result;
use foundry_cli::{handler, utils};
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
    sh_println, shell, stdin,
};
use foundry_config::Config;
use itertools::Itertools;
use my_logging::log_context::log_context;
use std::time::Instant;
use zkay_config::{config::library_compilation_environment, with_context_block};
use zkay_utils::progress_printer::{fail_print, success_print};

use std::path::{Path, PathBuf};
// use args::{Cast as CastArgs, CastSubcommand, ToBaseArgs};
use cast::traces::identifier::SignaturesIdentifier;
// from argcomplete.completers import FilesCompleter, DirectoriesCompleter
mod tests;
// from zkay.config_user import UserConfig
// pub mod cmd;
// pub mod zkay;
pub mod contract;
pub mod tx;
pub mod zkay_frontend;

#[macro_use]
extern crate lazy_static;

mod cmd;
// use cmd::{cache::CacheSubcommands, generate::GenerateSubcommands, watch};

mod zkay;
use zkay::{Zkay, ZkaySubcommand};

// #[macro_use]
// extern crate foundry_common;

// #[macro_use]
// extern crate tracing;

// #[cfg(all(feature = "jemalloc", unix))]
// #[global_allocator]
// static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    assert!(std::env::var("ZKAY_PATH").is_ok(), "ZKAY_PATH Not Found");
    if let Err(err) = utils::block_on(run()) {
        // let _ = foundry_common::Shell::get().error(&err);
        println!("=========={err:?}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Zkay::parse();
    args.global.init()?;
    match args.cmd {
        ZkaySubcommand::Compile(cmd) => cmd.run().map(drop),
        ZkaySubcommand::Create(cmd) => cmd.run().await,
        ZkaySubcommand::SendTx(cmd) => cmd.run().await,
        ZkaySubcommand::Call(cmd) => cmd.run().await,
        ZkaySubcommand::Balance {
            block,
            who,
            ether,
            rpc,
            erc20,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config)?;
            let account_addr = who.resolve(&provider).await?;

            match erc20 {
                Some(token) => {
                    let balance = Cast::new(&provider).erc20_balance(
                        token,
                        account_addr,
                        block,
                    ).await?;
                    sh_println!("{}", format_uint_exp(balance))
                }
                None => {
                    let value = Cast::new(&provider).balance(account_addr, block).await?;
                    if ether {
                        sh_println!("{}", SimpleCast::from_wei(&value.to_string(), "eth")?)
                    } else {
                        sh_println!("{value}")
                    }
                }
            }
        }
        ZkaySubcommand::Block {
            block,
            full,
            field,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config)?;
            sh_println!(
                "{}",
               Cast::new(provider).block(
                    block.unwrap_or(BlockId::Number(Latest)),
                    full,
                    field
                ).await?
            )
        }

        ZkaySubcommand::Estimate(cmd) => cmd.run().await,
        ZkaySubcommand::GasPrice { rpc } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config)?;
            sh_println!("{}", Cast::new(provider).gas_price().await?)
        }

        ZkaySubcommand::Rpc(cmd) => cmd.run().await,
        ZkaySubcommand::Code {
            block,
            who,
            disassemble,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config)?;
            let who = who.resolve(&provider).await?;
            sh_println!(
                "{}",
                Cast::new(provider).code(who, block, disassemble).await?
            )
        }
        ZkaySubcommand::Receipt {
            tx_hash,
            field,
            cast_async,
            confirmations,
            rpc,
        } => {
            let config = Config::from(&rpc);
            let provider = utils::get_provider(&config)?;
            sh_println!(
                "{}",
                Cast::new(provider).receipt(
                    tx_hash,
                    field,
                    confirmations,
                    None,
                    cast_async
                ).await?
            )
        }
        ZkaySubcommand::ToCheckSumAddress { address } => {
            let value = stdin::unwrap_line(address)?;
            sh_println!("{}", value.to_checksum(None))
        }
        // Misc
        ZkaySubcommand::Keccak { data } => {
            let bytes = match data {
                Some(data) => data.into_bytes(),
                None => stdin::read_bytes(false)?,
            };
            match String::from_utf8(bytes) {
                Ok(s) => {
                    let s = SimpleCast::keccak(&s)?;
                    sh_println!("{s}")
                }
                Err(e) => {
                    let hash = keccak256(e.as_bytes());
                    let s = hex::encode(hash);
                    sh_println!("0x{s}")
                }
            }
        }
    }
}
