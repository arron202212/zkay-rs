#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// use super::{install, watch::WatchArgs};
use clap::{Parser, ValueHint};

use eyre::Result;
use foundry_cli::{opts::CoreBuildArgs, utils::LoadConfig};
use foundry_common::{compile::ProjectCompiler, sh_println};
use foundry_compilers::{
    compilers::{multi::MultiCompilerLanguage, Language},
    utils::source_files_iter,
    Project, ProjectCompileOutput,
};
use foundry_config::{
    figment::{
        self,
        error::Kind::InvalidType,
        value::{Dict, Map, Value},
        Metadata, Profile, Provider,
    },
    Config,
};
use serde::Serialize;
use std::path::{Path, PathBuf};

use my_logging::log_context::log_context;
use zkay_config::{config::library_compilation_environment, with_context_block};
use zkay_utils::progress_printer::{fail_print, success_print};

// foundry_config::merge_impl_figment_convert!(CompileArgs, args);
const SOLC_VERSION_HELP: &'static str = "zkay defaults to the latest installed\n \
          solidity version supported by the current zkay version.\n\n \
          If you need to use a particular minor release (e.g. because \n \
          the latest release is broken or you need determinism for testing)\n \
          you can specify a particular solc version (e.g. v0.5.12) via this argument.\n \
          Note: An internet connection is required if the selected version is not installed";

/// CLI arguments for `forge build`.
///
/// CLI arguments take the highest precedence in the Config/Figment hierarchy.
/// In order to override them in the foundry `Config` they need to be merged into an existing
/// `figment::Provider`, like `foundry_config::Config` is.
///
/// # Example
///
/// ```
/// use foundry_cli::cmd::forge::build::CompileArgs;
/// use foundry_config::Config;
/// # fn t(args: CompileArgs) {
/// let config = Config::from(&args);
/// # }
/// ```
///
/// `CompileArgs` implements `figment::Provider` in which all config related fields are serialized and
/// then merged into an existing `Config`, effectively overwriting them.
///
/// Some arguments are marked as `#[serde(skip)]` and require manual processing in
/// `figment::Provider` implementation
#[derive(Clone, Debug, Default, Serialize, Parser)]
#[command(next_help_heading = "Build options", about = "Compile a zkay contract.", long_about = None)] // override doc
pub struct CompileArgs {
    /// Build source files from specified paths.
    #[arg(short,long,value_hint = ValueHint::DirPath, value_name = "OUTPUT_DIRECTORY",default_value=".",help="The directory to output the compiled contract to. Default: Current directory")]
    #[serde(skip)]
    pub output: PathBuf,

    #[arg(long,value_hint = ValueHint::FilePath, value_name = "ZKAY_FILE",help="The zkay source file")]
    #[serde(skip)]
    pub input: PathBuf,

    /// Print compiled contract names.
    #[arg(long, help = "enable logging")]
    #[serde(skip)]
    pub log: bool,

    #[arg(long,value_name="CFG_VAL",default_value="0.6.12",help=SOLC_VERSION_HELP)]
    #[serde(skip)]
    pub solc_version: String,
    // /// Print compiled contract sizes.
    // /// Constructor argument length is not included in the calculation of initcode size.
    // #[arg(long)]
    // #[serde(skip)]
    // pub sizes: bool,

    // /// Ignore initcode contract bytecode size limit introduced by EIP-3860.
    // #[arg(long, alias = "ignore-initcode-size")]
    // #[serde(skip)]
    // pub ignore_eip_3860: bool,

    // #[command(flatten)]
    // #[serde(flatten)]
    // pub args: CoreBuildArgs,

    // #[command(flatten)]
    // #[serde(skip)]
    // pub watch: WatchArgs,
    // /// Output the compilation errors in the json format.
    // /// This is useful when you want to use the output in other tools.
    // #[arg(long)]
    // #[serde(skip)]
    // pub format_json: bool,
}

impl CompileArgs {
    pub fn run(self) -> Result<()> {
        // let mut config = self.try_load_config_emit_warnings()?;

        // if install::install_missing_dependencies(&mut config) && config.auto_detect_remappings {
        //     // need to re-configure here to also catch additional remappings
        //     config = self.load_config();
        // }

        // let project = config.project()?;

        // // Collect sources to compile if build subdirectories specified.
        // let mut files = vec![];
        // if let Some(paths) = &self.paths {
        //     for path in paths {
        //         let joined = project.root().join(path);
        //         let path = if joined.exists() { &joined } else { path };
        //         files.extend(source_files_iter(
        //             path,
        //             MultiCompilerLanguage::FILE_EXTENSIONS,
        //         ));
        //     }
        // }

        // let compiler = ProjectCompiler::new()
        //     .files(files)
        //     .print_names(self.names)
        //     .print_sizes(self.sizes)
        //     .ignore_eip_3860(self.ignore_eip_3860)
        //     .quiet(self.format_json)
        //     .bail(!self.format_json);

        // let output = compiler.compile(&project)?;

        // if self.format_json {
        //     sh_println!("{}", serde_json::to_string_pretty(&output.output())?)?;
        // }
        // println!("========compile======================{:?}", 1);
        let input_path = self.input.clone();
        // if let Ok(Some(input_path)) = self.input{
        //     if let Err(_) | Ok(false) = Path::new(input_path).try_exists() {
        //         with_context_block!(var _fp=fail_print()=>{
        //         println!("Error: input file \'{input_path:?}\' does not exist");
        //         });
        //         std::process::exit(10);
        //     }
        //     input_path.clone()
        // } else {
        //     PathBuf::new()
        // };
        // create output directory
        let output = self.output;
        // println!("============================={:?}", output);
        use path_absolutize::*;
        let output_dir = Path::new(&output).absolutize().expect("absolute path fail");
        if let Err(_) | Ok(false) = output_dir.try_exists() {
            let _ = std::fs::create_dir_all(output_dir.clone());
        } else if !output_dir.is_dir() {
            with_context_block!(var _fp=fail_print()=>{
                    println!("Error: \'{output_dir:?}\' is not a directory");});
            std::process::exit(2);
        }

        // // Enable logging
        // if let Some(true) = self.log {
        //     // log_file = my_logging.get_log_file(filename='compile', include_timestamp=False, label=None)
        //     // my_logging.prepare_logger(log_file)
        // }
        // // only type-check
        println!("Compiling file {:?}:", input_path);

        // // compile
        let input_basename = input_path.file_name().unwrap().to_str().unwrap();
        with_context_block!(var _lc=log_context(input_basename)=>
        {if let Err(e) = crate::zkay_frontend::compile_zkay_file(
            &input_path.to_str().expect(""),
            output_dir.to_str().expect(""),
            false,
        ) {
            //ZkayCompilerError
            with_context_block!(var _fp=fail_print()=>{
            println!("===compile_zkay_file===fail==={e}");});
            std::process::exit(3);
        }});
        Ok(())
    }

    // /// Returns the `Project` for the current workspace
    // ///
    // /// This loads the `foundry_config::Config` for the current workspace (see
    // /// [`utils::find_project_root`] and merges the cli `CompileArgs` into it before returning
    // /// [`foundry_config::Config::project()`]
    // pub fn project(&self) -> Result<Project> {
    //     self.args.project()
    // }

    // /// Returns whether `CompileArgs` was configured with `--watch`
    // pub fn is_watch(&self) -> bool {
    //     self.watch.watch.is_some()
    // }

    //// Returns the [`watchexec::InitConfig`] and [`watchexec::RuntimeConfig`] necessary to
    //// bootstrap a new [`watchexe::Watchexec`] loop.
    // pub(crate) fn watchexec_config(&self) -> Result<watchexec::Config> {
    //     // Use the path arguments or if none where provided the `src`, `test` and `script`
    //     // directories as well as the `foundry.toml` configuration file.
    //     self.watch.watchexec_config(|| {
    //         let config = Config::from(self);
    //         let foundry_toml: PathBuf = config.root.0.join(Config::FILE_NAME);
    //         [config.src, config.test, config.script, foundry_toml]
    //     })
    // }
}

// // Make this args a `figment::Provider` so that it can be merged into the `Config`
// impl Provider for CompileArgs {
//     fn metadata(&self) -> Metadata {
//         Metadata::named("Build Args Provider")
//     }

//     fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
//         let value = Value::serialize(self)?;
//         let error = InvalidType(value.to_actual(), "map".into());
//         let mut dict = value.into_dict().ok_or(error)?;

//         if self.names {
//             dict.insert("names".to_string(), true.into());
//         }

//         if self.sizes {
//             dict.insert("sizes".to_string(), true.into());
//         }

//         if self.ignore_eip_3860 {
//             dict.insert("ignore_eip_3860".to_string(), true.into());
//         }

//         Ok(Map::from([(Config::selected_profile(), dict)]))
//     }
// }
