// #!/usr/bin/env python3
// // PYTHON_ARGCOMPLETE_OK
// import argcomplete, argparse
// import os

// from argcomplete.completers import FilesCompleter, DirectoriesCompleter
mod tests;
// from zkay.config_user import UserConfig
pub mod zkay_frontend;
#[macro_use]
extern crate lazy_static;
use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use std::path::{Path, PathBuf};
// def parse_config_doc():
//     import textwrap
//     from typing import get_type_hints
//     __ucfg = UserConfig()

//     docs = {}
//     for name, prop in vars(UserConfig).items():
//         if name.startswith('_') or not isinstance(prop, property):
//             continue
//         t = get_type_hints(prop.fget)['return']
//         doc = prop.__doc__
//         choices = None
//         if hasattr(__ucfg, f'_{name}_values'):
//             choices = getattr(__ucfg, f'_{name}_values')
//         default_val = getattr(__ucfg, name)
//         docs[name] = (
//             f"type: {t}\n\n"
//             f"{textwrap.dedent(doc).strip()}\n\n"
//             f"Default value: {default_val}", t, default_val, choices)
//     return docs

fn parse_arguments() -> ArgMatches {
    //     class ShowSuppressedInHelpFormatter(argparse.RawTextHelpFormatter):
    //         def add_usage(self, usage, actions, groups, prefix=None):
    //             if usage is not argparse.SUPPRESS:
    //                 actions = [action for action in actions if action.metavar != '<cfg_val>']
    //                 args = usage, actions, groups, prefix
    //                 self._add_item(self._format_usage, args)

    let  main_parser= Command::new("zkay")
//     main_parser = argparse.ArgumentParser(prog='zkay')
//     zkay_files = ('zkay', 'sol')
//     zkay_package_files = ('zkp', )
//     config_files = ('json', )

//     msg = 'Path to local configuration file (defaults to "config.json" in cwd). ' \
//           'This file (if it exists), overrides settings defined in the global configuration.'
//     main_parser.add_argument('--config-file', default='config.json', metavar='<config_file>', help=msg).completer = FilesCompleter(config_files)
        .arg( Arg::new("config-file")
                        .long("config-file")
                        .help("Path to local configuration file (defaults to \"config.json\" in cwd). \n \
                        This file (if it exists), overrides settings defined in the global configuration.")
                        .default_value("config.json")
                        .value_name("<config_file>")
                        .action(ArgAction::Set));
    //     // Shared 'config' parser
    //     config_parser = argparse.ArgumentParser(add_help=False)
    //     msg = 'These parameters can be used to override settings defined (and documented) in config_user.py'
    //     cfg_group = config_parser.add_argument_group(title='Configuration Options', description=msg)

    //     // Expose config_user.py options via command line arguments, they are supported in all parsers
    //     cfg_docs = parse_config_doc()

    //     def add_config_args(parser, arg_names):
    //         for name in arg_names:
    //             doc, t, defval, choices = cfg_docs[name]

    //             if t is bool:
    //                 if defval:
    //                     parser.add_argument(f'--no-{name.replace("_", "-")}', dest=name, help=doc, action='store_false')
    //                 else:
    //                     parser.add_argument(f'--{name.replace("_", "-")}', dest=name, help=doc, action='store_true')
    //             elif t is int:
    //                 parser.add_argument(f'--{name.replace("_", "-")}', type=int, dest=name, metavar='<cfg_val>', help=doc)
    //             else:
    //                 arg = parser.add_argument(f'--{name.replace("_", "-")}', dest=name, metavar='<cfg_val>', help=doc,
    //                                           choices=choices)
    //                 if name.endswith('dir'):
    //                     arg.completer = DirectoriesCompleter()
    //     add_config_args(cfg_group, cfg_docs.keys())

    const SOLC_VERSION_HELP: &'static str = "zkay defaults to the latest installed\n \
          solidity version supported by the current zkay version.\n\n \
          If you need to use a particular minor release (e.g. because \n \
          the latest release is broken or you need determinism for testing)\n \
          you can specify a particular solc version (e.g. v0.5.12) via this argument.\n \
          Note: An internet connection is required if the selected version is not installed";

    lazy_static! {
        static ref CURR_DIRS: String = std::env::current_dir()
            .unwrap_or(PathBuf::from("."))
            .to_str()
            .unwrap()
            .to_string();
    }
    //     subparsers = main_parser.add_subparsers(title='actions', dest='cmd', required=True)
    let mut subparsers = main_parser;
    subparsers=subparsers.group(ArgGroup::new("actions").multiple(true)).next_help_heading("actions").subcommand_help_heading("actions")
//     // 'compile' parser
//     compile_parser = subparsers.add_parser('compile', parents=[config_parser], help='Compile a zkay contract.', formatter_class=ShowSuppressedInHelpFormatter)
//     msg = 'The directory to output the compiled contract to. Default: Current directory'
//     compile_parser.add_argument('-o', '--output', default=os.getcwd(), help=msg, metavar='<output_directory>').completer = DirectoriesCompleter()
//     compile_parser.add_argument('input', help='The zkay source file', metavar='<zkay_file>').completer = FilesCompleter(zkay_files)
//     compile_parser.add_argument('--log', action='store_true', help='enable logging')
//     compile_parser.add_argument('--solc-version', help=SOLC_VERSION_HELP, metavar='<cfg_val>')
 .subcommand(
            Command::new("compile")
                .help_template("Compile a zkay contract.")
                .arg( Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("The directory to output the compiled contract to. Default: Current directory")
                        .default_value(CURR_DIRS.as_str())
                        .value_name("<output_directory>")
 .value_parser(value_parser!(std::path::PathBuf))
                        .action(ArgAction::Set))
                    .arg( Arg::new("input")
                        .long("input")
                        .help("The zkay source file")
                        .value_name("<zkay_file>")
 .value_parser(value_parser!(std::path::PathBuf))
                        .action(ArgAction::Set))
.arg( Arg::new("log")
                        .long("log")
                        .help("enable logging")
                        .action(ArgAction::SetTrue))
.arg( Arg::new("solc-version")
                        .long("solc-version")
                        .help(SOLC_VERSION_HELP)
                        .value_name("<cfg_val>")
                        .action(ArgAction::Set))
        )
//     // 'check' parser
//     typecheck_parser = subparsers.add_parser('check', parents=[config_parser], help='Only type-check, do not compile.', formatter_class=ShowSuppressedInHelpFormatter)
//     typecheck_parser.add_argument('input', help='The zkay source file', metavar='<zkay_file>').completer = FilesCompleter(zkay_files)
//     typecheck_parser.add_argument('--solc-version', help=SOLC_VERSION_HELP, metavar='<cfg_val>')
.subcommand(
            Command::new("check")
                .help_template("Only type-check, do not compile.")
                    .arg( Arg::new("input")
                        .long("input")
                        .help("The zkay source file")
                        .value_name("<zkay_file>")
                        .action(ArgAction::Set))
.arg( Arg::new("solc-version")
                        .long("solc-version")
                        .help(SOLC_VERSION_HELP)
                        .value_name("<cfg_val>")
                        .action(ArgAction::Set))
        )
//     // 'solify' parser
//     msg = 'Output solidity code which corresponds to zkay code with all privacy features and comments removed, ' \
//           'useful in conjunction with analysis tools which operate on solidity code.)'
//     solify_parser = subparsers.add_parser('solify', parents=[config_parser], help=msg, formatter_class=ShowSuppressedInHelpFormatter)
//     solify_parser.add_argument('input', help='The zkay source file', metavar='<zkay_file>').completer = FilesCompleter(zkay_files)
.subcommand(
            Command::new("solify")
                .help_template("Output solidity code which corresponds to zkay code with all privacy features and comments removed,\n  \
useful in conjunction with analysis tools which operate on solidity code.")
                    .arg( Arg::new("input")
                        .long("input")
                        .help("The zkay source file")
                        .value_name("<zkay_file>")
                        .action(ArgAction::Set))
        )
//     // 'export' parser
//     export_parser = subparsers.add_parser('export', parents=[config_parser], help='Package a compiled zkay contract.', formatter_class=ShowSuppressedInHelpFormatter)
//     msg = 'Output filename. Default: ./contract.zkp'
//     export_parser.add_argument('-o', '--output', default='contract.zkp', help=msg, metavar='<output_filename>').completer = FilesCompleter(zkay_package_files)
//     msg = 'Directory with the compilation output of the contract which should be packaged.'
//     export_parser.add_argument('input', help=msg, metavar='<zkay_compilation_output_dir>').completer = DirectoriesCompleter()
.subcommand(
            Command::new("export")
                .help_template("Package a compiled zkay contract.")
                .arg( Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output filename. Default: ./contract.zkp")
                        .default_value("contract.zkp")
                        .value_name("<output_filename>")
                        .action(ArgAction::Set))
                    .arg( Arg::new("input")
                        .long("input")
                        .help("Directory with the compilation output of the contract which should be packaged.")
                        .value_name("<zkay_compilation_output_dir>")
                        .action(ArgAction::Set))
)
//     // 'import' parser
//     msg = 'Unpack a packaged zkay contract.\n' \
//           'Note: An internet connection is required if the packaged contract used a solc version which is not currently installed.'
//     import_parser = subparsers.add_parser('import', parents=[config_parser], help=msg, formatter_class=ShowSuppressedInHelpFormatter)
//     msg = 'Directory where the contract should be unpacked to. Default: Current Directory'
//     import_parser.add_argument('-o', '--output', default=os.getcwd(), help=msg, metavar='<target_directory>').completer = DirectoriesCompleter()
//     msg = 'Contract package to unpack.'
//     import_parser.add_argument('input', help=msg, metavar='<zkay_package_file>').completer = FilesCompleter(zkay_package_files)
.subcommand(
            Command::new("import")
                .help_template("Unpack a packaged zkay contract.\n \
//           Note: An internet connection is required if the packaged contract used a solc version which is not currently installed.")
                .arg( Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Directory where the contract should be unpacked to. Default: Current Directory")
                        .default_value(CURR_DIRS.as_str())
                        .value_name("<output_filentarget_directoryame>")
                        .action(ArgAction::Set))
                    .arg( Arg::new("input")
                        .long("input")
                        .help("Contract package to unpack.")
                        .value_name("<zkay_package_file>")
                        .action(ArgAction::Set))
);
    //     // 'run, deploy and connect' parsers
    //     interact_parser = argparse.ArgumentParser(add_help=False)
    //     msg = 'Directory with the compilation output of the contract with which you want to interact.'
    //     interact_parser.add_argument('input', help=msg, metavar='<zkay_compilation_output_dir>').completer = DirectoriesCompleter()
    //     interact_parser.add_argument('--log', action='store_true', help='enable logging')
    //     interact_parser.add_argument('--account', help='Sender blockchain address', metavar='<address>')
    let _interact_parser=Command::new("interact")
                .arg( Arg::new("account")
                        .long("account")
                        .help("Sender blockchain address")
                        .value_name("<address>")
                        .action(ArgAction::Set))
                    .arg( Arg::new("input")
                        .long("input")
                        .help("Directory with the compilation output of the contract with which you want to interact.")
                        .value_name("<zkay_compilation_output_dir>")
                        .action(ArgAction::Set))
.arg( Arg::new("log")
                        .long("log")
                        .help("enable logging")
                        .action(ArgAction::SetTrue));

    //     subparsers.add_parser('run', parents=[interact_parser, config_parser],
    //                           help='Enter transaction shell for a compiled zkay contract.',
    //                           formatter_class=ShowSuppressedInHelpFormatter)
    subparsers = subparsers
        .subcommand(
            Command::new("run")
                .help_template("Enter transaction shell for a compiled zkay contract."),
        )
        //     deploy_parser = subparsers.add_parser('deploy', parents=[interact_parser, config_parser],
        //                                           help='Deploy contract with given constructor arguments',
        //                                           formatter_class=ShowSuppressedInHelpFormatter)
        //     deploy_parser.add_argument('constructor_args', nargs='*', help='Constructor arguments', metavar='<args>...')
        .subcommand(
            Command::new("deploy")
                .help_template("Deploy contract with given constructor arguments")
                .arg(
                    Arg::new("constructor_args")
                        .long("constructor_args")
                        .help("Constructor arguments")
                        .default_value(CURR_DIRS.as_str())
                        .value_name("<args>")
                        .num_args(0..)
                        .action(ArgAction::Set),
                ),
        )
        //     connect_parser = subparsers.add_parser('connect', parents=[interact_parser, config_parser],
        //                                           help='Connect to contract at address and enter shell.',
        //                                           formatter_class=ShowSuppressedInHelpFormatter)
        //     connect_parser.add_argument('address', help='Blockchain address of deployed contract', metavar='<address>')
        .subcommand(
            Command::new("connect")
                .help_template("Connect to contract at address and enter shell.")
                .arg(
                    Arg::new("address")
                        .long("address")
                        .help("Blockchain address of deployed contract")
                        .value_name("<address>")
                        .action(ArgAction::Set),
                ),
        );
    //     // Common deploy libs parameters
    //     deploy_libs_parser = argparse.ArgumentParser(add_help=False)
    //     msg = 'Address of the account to use for deploying the library contracts. ' \
    //           'Its ethereum keys must be hosted in the specified node and sufficient funds ' \
    //           'to cover the deployment costs must be available. ' \
    //           'WARNING: This account will be charged with the deployment costs.'
    //     deploy_libs_parser.add_argument('account', metavar='<deployer account ethereum address>', help=msg)
    let _deploy_libs_parser = Command::new("deploy_libs").arg(
        Arg::new("account")
            .long("account")
            .help(
                "Address of the account to use for deploying the library contracts. \n \
Its ethereum keys must be hosted in the specified node and sufficient funds \n \
to cover the deployment costs must be available. \n \
WARNING: This account will be charged with the deployment costs.",
            )
            .value_name("<deployer account ethereum address>")
            .action(ArgAction::Set),
    );

    //     // 'deploy-pki' parser
    //     dpki_parser = subparsers.add_parser('deploy-pki', parents=[deploy_libs_parser],
    //                                         help='Manually deploy global pki contract compatible with a particular crypto backend to a blockchain')
    //     add_config_args(dpki_parser, {'main_crypto_backend', 'blockchain_backend', 'blockchain_node_uri'})
    subparsers=subparsers.subcommand(
            Command::new("deploy-pki")
                .help_template("Manually deploy global pki contract compatible with a particular crypto backend to a blockchain")
)
//     // 'deploy-crypto-libs' parser
//     dclibs_parser = subparsers.add_parser('deploy-crypto-libs', parents=[deploy_libs_parser],
//                                           help='Manually deploy proving-scheme specific crypto libraries (if any needed) to a blockchain')
//     add_config_args(dclibs_parser, {'proving_scheme', 'blockchain_backend', 'blockchain_node_uri'})
.subcommand(
            Command::new("deploy-crypto-libs")
                .help_template("Manually deploy proving-scheme specific crypto libraries (if any needed) to a blockchain")
)
//     subparsers.add_parser('version', help='Display zkay version information')
//     subparsers.add_parser('update-solc', help='Install latest compatible solc version (requires internet connection)')
.subcommand(
            Command::new("version")
                .help_template("Display zkay version information")
)
.subcommand(
            Command::new("update-solc")
                .help_template("Install latest compatible solc version (requires internet connection)")
);
    let main_parser = subparsers;
    //     // parse
    //     argcomplete.autocomplete(main_parser, always_complete_options=False)
    //     a = main_parser.parse_args()
    //     return a
    main_parser.get_matches()
}

fn main() {
    println!("==main=====");
    //     // parse arguments
    let a = parse_arguments();
    println!("==main==1===");
    //     from zkay.config_version import Versions

    if let Some(("version", _)) = a.subcommand() {
        println!("Versions.ZKAY_VERSION");
        return;
    }

    if let Some(("update-solc", _)) = a.subcommand() {
        // import solcx
        // solcx.install_solc_pragma(Versions.ZKAY_SOLC_VERSION_COMPATIBILITY.expression)
        println!("ERROR: Error while updating solc\n");
        return;
    }

    //     from pathlib import Path

    use crate::zkay_frontend as frontend;
    //     from zkay import my_logging
    //     from zkay.config import cfg
    //     use zkay_utils::helpers::{read_file, save_to_file};
    //     from zkay.errors.exceptions import ZkayCompilerError
    //     from zkay.my_logging.log_context import log_context
    //     from zkay.utils.progress_printer import fail_print, success_print
    #[warn(dead_code)]
    fn fail_print() {}
    #[warn(dead_code)]
    fn success_print() {}
    //     from zkay.zkay_ast.process_ast import get_processed_ast, get_parsed_ast_and_fake_code

    //     // Load configuration files
    //     try:
    //         cfg.load_configuration_from_disk(a.config_file)
    //     except Exception as e:
    //         with fail_print():
    //             print(f"ERROR: Failed to load configuration files\n{e}")
    //             exit(42)

    //     // Support for overriding any user config setting via command line
    //     // The evaluation order for configuration loading is:
    //     // Default values in config.py -> Site config.json -> user config.json -> local config.json -> cmdline arguments
    //     // Settings defined at a later stage override setting values defined at an earlier stage
    //     override_dict = {}
    //     for name in vars(UserConfig):
    //         if name[0] != '_' and hasattr(a, name):
    //             val = getattr(a, name)
    //             if val is not None:
    //                 override_dict[name] = val
    //     cfg.override_defaults(override_dict)

    if let Some(("deploy-pki", _)) | Some(("deploy-crypto-libs", _)) = a.subcommand() {
        //         import tempfile
        //         from zkay.compiler.privacy import library_contracts
        //         from zkay.transaction.runtime import Runtime
        //         with tempfile.TemporaryDirectory() as tmpdir:
        //             try:

        // if let Err(e) = cfg.library_compilation_environment() {
        //     fail_print();
        //     println!("ERROR: Deployment failed {e}\n");
        // } else
        {
            if let Some(("deploy-pki", _)) = a.subcommand() {
                //                         for crypto_params in cfg.all_crypto_params():
                //                             pki_contract_code = library_contracts.get_pki_contract(crypto_params)
                //                             pki_contract_name = cfg.get_pki_contract_name(crypto_params)
                //                             file = save_to_file(tmpdir, f'{pki_contract_name}.sol', pki_contract_code)
                //                             addr = Runtime.blockchain().deploy_solidity_contract(file, pki_contract_name, a.account)
                //                             print(f'Deployed pki contract for crypto backend "{crypto_params.crypto_name}" at: {addr}')
            } else {
                //                         if not cfg.external_crypto_lib_names:
                //                             print('Current proving scheme does not require library deployment')
                //                         else:
                //                             file = save_to_file(tmpdir, 'verify_libs.sol', library_contracts.get_verify_libs_code())
                //                             for lib in cfg.external_crypto_lib_names:
                //                                 addr = Runtime.blockchain().deploy_solidity_contract(file, lib, a.account)
                //                                 print(f'Deployed crypto library {lib} at: {addr}')
            }
        }
    } else {
        //         // Solc version override
        // if let Some(solc_version) = a.get_one::<String>("solc_version") {
        //     // if Err(e) = cfg.override_solc(solc_version) {
        //     //     fail_print();
        //         println!("Error: {solc_version}");
        //     //     std::process::exit(10);
        //     // }
        // }
        //
        println!("Using solc version Versions.SOLC_VERSION");

        match a.subcommand() {
            Some(("check", _)) => {
                //             // only type-check
                //             print(f'Type checking file {input_path.name}:')
                //             code = read_file(str(input_path))
                //             try:
                //                 get_processed_ast(code)
                //             except ZkayCompilerError as e:
                //                 with fail_print():
                //                     print(f'{e}')
                //                 exit(3)
            }
            Some(("solify", _)) => {
                //             was_unit_test = cfg.is_unit_test
                //             cfg._is_unit_test = True  // Suppress other output
                //             try:
                //                 _, fake_code = get_parsed_ast_and_fake_code(read_file(str(input_path)))
                //                 print(fake_code)
                //             except ZkayCompilerError as e:
                //                 with fail_print():
                //                     print(f'{e}')
                //                 exit(3)
                //             finally:
                //                 cfg._is_unit_test = was_unit_test
                //             exit(0)
            }
            Some(("compile", sub_compile)) => {
                println!("========compile======================{:?}", 1);
                let input_path =
                    if let Ok(Some(input_path)) = sub_compile.try_get_one::<PathBuf>("input") {
                        if let Err(_) | Ok(false) = Path::new(input_path).try_exists() {
                            fail_print();
                            println!("Error: input file \'{input_path:?}\' does not exist");
                            std::process::exit(10);
                        }
                        input_path.clone()
                    } else {
                        PathBuf::new()
                    };
                // create output directory
                let output = sub_compile
                    .try_get_one::<PathBuf>("output")
                    .unwrap()
                    .unwrap();
                // println!("============================={:?}", output);
                use path_absolutize::*;
                let output_dir = Path::new(output).absolutize().expect("absolute path fail");
                if let Err(_) | Ok(false) = output_dir.try_exists() {
                    let _ = std::fs::create_dir_all(output_dir.clone());
                } else if !output_dir.is_dir() {
                    fail_print();
                    println!("Error: \'{output_dir:?}\' is not a directory");
                    std::process::exit(2);
                }

                // // Enable logging
                if let Some(true) = sub_compile.get_one::<bool>("log") {
                    // log_file = my_logging.get_log_file(filename='compile', include_timestamp=False, label=None)
                    // my_logging.prepare_logger(log_file)
                }
                // // only type-check
                println!("Compiling file {:?}:", input_path);

                // // compile
                // log_context(os.path.basename(a.input));

                if let Err(e) = frontend::compile_zkay_file(
                    &input_path.to_str().expect(""),
                    output_dir.to_str().expect(""),
                    false,
                ) {
                    //ZkayCompilerError
                    fail_print();
                    println!("===compile_zkay_file===fail==={e}");
                    std::process::exit(3);
                }
            }
            Some(("import", _)) => {
                //             // create output directory
                //             output_dir = Path(a.output).absolute()
                //             if output_dir.exists():
                //                 with fail_print():
                //                     print(f'Error: \'{output_dir}\' already exists')
                //                 exit(2)

                //             try:
                //                 frontend.extract_zkay_package(str(input_path), str(output_dir))
                //             except ZkayCompilerError as e:
                //                 with fail_print():
                //                     print(f"ERROR while compiling unpacked zkay contract.\n{e}")
                //                 exit(3)
                //             except Exception as e:
                //                 with fail_print():
                //                     print(f"ERROR while unpacking zkay contract\n{e}")
                //                 exit(5)
            }
            Some(("export", _)) => {
                //             output_filename = Path(a.output).absolute()
                //             os.makedirs(output_filename.parent, exist_ok=True)
                //             try:
                //                 frontend.package_zkay_contract(str(input_path), str(output_filename))
                //             except Exception as e:
                //                 with fail_print():
                //                     print(f"ERROR while exporting zkay contract\n{e}")
                //                 exit(4)
            }
            Some(("run", _)) | Some(("deploy", _)) | Some(("connect", _)) => {
                //             from enum import IntEnum
                //             from zkay.transaction.offchain import ContractSimulator
                //             def echo_only_simple_expressions(e):
                //                 if isinstance(e, (bool, int, str, list, tuple, IntEnum)):
                //                     print(e)

                //             // Enable logging
                if let Some(true) = a.get_one::<bool>("log") {
                    //                 log_file = my_logging.get_log_file(filename=f'transactions_{input_path.name}', include_timestamp=True, label=None)
                    //                 my_logging.prepare_logger(log_file)
                }

                //             contract_dir = str(input_path.absolute())
                //             frontend.use_configuration_from_manifest(contract_dir)
                let _me = if let Some(me) = a.get_one::<String>("account") {
                    me.to_string()
                } else {
                    // if Some(me) = ContractSimulator.default_address() {
                    //     me.val
                    // } else {
                    String::new()
                    // }
                };

                //             import code
                //             import sys
                if let Some(("run", _)) = a.subcommand() {
                    //                 // Dynamically load module and replace globals with module globals
                    //                 contract_mod = frontend.load_transaction_interface_from_directory(contract_dir)
                    //                 contract_mod.me = me
                    //                 sys.displayhook = echo_only_simple_expressions
                    //                 code.interact(local=contract_mod.__dict__)
                } else {
                    //                 cmod = frontend.load_transaction_interface_from_directory(contract_dir)
                    //                 c = frontend.load_contract_transaction_interface_from_module(cmod)
                    if let Some(("deploy", _)) = a.subcommand() {
                        //                     from ast import literal_eval
                        //                     cargs = a.constructor_args
                        //                     args = []
                        //                     for arg in cargs:
                        //                         try:
                        //                             val = literal_eval(arg)
                        //                         except Exception:
                        //                             val = arg
                        //                         args.append(val)
                        //                     try:
                        //                         c.deploy(*args, user=me, project_dir=contract_dir)
                        //                     except (ValueError, TypeError) as e:
                        //                         with fail_print():
                        //                             print(f'ERROR invalid arguments.\n{e}\n\nExpected contructor signature: ', end='')
                        //                             if hasattr(c, 'constructor'):
                        //                                 from inspect import signature
                        //                                 sig = str(signature(c.constructor))
                        //                                 sig = sig[5:] if not sig[5:].startswith(",") else sig[7:] // without self
                        //                                 print(f'({sig}')
                        //                             else:
                        //                                 print('()')
                        //                             exit(11)
                        //                     except Exception as e:
                        //                         with fail_print():
                        //                             print(f'ERROR: failed to deploy contract\n{e}')
                        //                             exit(12)
                    }
                    if let Some(("connect", _)) = a.subcommand() {
                        //                     try:
                        //                         c_inst = c.connect(address=a.address, user=me, project_dir=contract_dir)
                        //                     except Exception as e:
                        //                         with fail_print():
                        //                             print(f'ERROR: failed to connect to contract\n{e}')
                        //                             exit(13)

                        //                     // Open interactive shell in context of contract object
                        //                     import inspect
                        //                     contract_scope = {name: getattr(c_inst, name) for name in dir(c_inst)
                        //                                       if inspect.isclass(getattr(c_inst, name)) \
                        //                                          or (name != 'constructor' and hasattr(getattr(c_inst, name), '_can_be_external')
                        //                                              and getattr(c_inst, name)._can_be_external)
                        //                                          or name in ['state', 'api']}
                        //                     contract_scope['me'] = me
                        //                     contract_scope['help'] = lambda o=None: help(o) if o is not None else ContractSimulator.reduced_help(c)
                        //                     sys.displayhook = echo_only_simple_expressions
                        //                     code.interact(local=contract_scope)
                    } else {
                        println!(" ValueError(\"unexpected command a.cmd\")");
                        std::process::exit(0);
                    }
                }
            }
            _ => println!("NotImplementedError(a.cmd)"),
        }
        success_print();
        println!("Finished successfully");
    }
}

// if __name__ == '__main__':
//     main()
