#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// """
// This module exposes functionality to compile and package zkay code
// """
//::importlib
//::json
//::os
//::re
//::shutil
//::sys
//::tempfile
//::zipfile
// from contextlib::contextmanager
// from copy::deepcopy
// from typing::Tuple, List, Type, Dict, Optional, Any, ContextManager

// use my_logging
use circuit_generation::backends::jsnark_generator::JsnarkGenerator;
use circuit_generation::circuit_generator::CircuitGenerator;
use circuit_helper::circuit_helper::CircuitHelper;
use privacy::library_contracts;
use privacy::manifest::Manifest;
use rccell::RcCell;
// use privacy::offchain_compiler::PythonOffchainVisitor
use proving_scheme::backends::gm17::ProvingSchemeGm17;
use proving_scheme::backends::groth16::ProvingSchemeGroth16;
use proving_scheme::proving_scheme::{ProvingScheme, VerifyingKeyMeta};
use solidity::compiler::check_compilation;
use transformation::zkay_contract_transformer::transform_ast;
use zkay_config::config::CFG;
use zkay_utils::helpers::{lines_of_code, read_file}; //, without_extension};
use zkay_utils::progress_printer::print_step;
// use zkay_utils::timer::time_measure
use ast_builder::process_ast::{get_processed_ast, get_verification_contract_names};
use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use zkay_ast::ast::IntoAST;
use zkay_ast::global_defs::{
    array_length_member, global_defs, global_vars, GlobalDefs, GlobalVars,
};
use zkay_ast::homomorphism::Homomorphism;
use zkay_ast::visitors::solidity_visitor::to_solidity;

// fn proving_scheme_classes<T,VK>(proving_scheme: &str) -> T
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
// {
//     match proving_scheme {
//         "groth16" => &ProvingSchemeGroth16,
//         _ => &ProvingSchemeGm17, //"gm17"
//     }
// }
fn generator_classes(
    _snark_backend: &String,
) -> impl FnOnce(Vec<RcCell<CircuitHelper>>, String, String) -> JsnarkGenerator //<T, VK>
// where
//     T: ProvingScheme<VerifyingKeyX = VK> + std::marker::Sync,
//     VK: VerifyingKeyMeta<Output = VK>,
{
    JsnarkGenerator::new
}

// """
// Parse, type-check and compile the given zkay contract file.

// :param input_file_path: path to the zkay contract file
// :param output_dir: path to a directory where the compilation output should be generated
// :param import_keys: | if false, zk-snark of all modified circuits will be generated during compilation
//                     | if true, zk-snark keys for all circuits are expected to be already present in the output directory, and the compilation will use the provided keys to generate the verification contracts
//                     | This option is mostly used internally when connecting to a zkay contract provided by a 3rd-party
// :raise ZkayCompilerError: if any compilation stage fails
// :raise RuntimeError: if import_keys is true and zkay file, manifest file or any of the key files is missing
// """
pub fn compile_zkay_file(
    input_file_path: &str,
    output_dir: &str,
    import_keys: bool,
) -> anyhow::Result<()> {
    let code = read_file(input_file_path);

    // log specific features of compiled program
    // my_logging.data('originalLoc', lines_of_code(code))
    // m = re.search(r'\/\/ Description: (.*)', code)
    // if m:
    //     my_logging.data('description', m.group(1))
    // m = re.search(r'\/\/ Domain: (.*)', code)
    // if m:
    //     my_logging.data('domain', m.group(1))
    let _filename = std::path::Path::new(input_file_path)
        .file_name()
        .unwrap()
        .to_str();

    // compile
    // with time_measure('compileFull'):
    // let (cg, _) =
    compile_zkay(code.as_str(), output_dir, import_keys);
    Ok(())
}

// """
// Parse, type-check and compile the given zkay code.

// Note: If a SolcException is raised, this indicates a bug in zkay
//       (i.e. zkay produced solidity code which doesn't compile, without raising a ZkayCompilerError)

// :param code: zkay code to compile
// :param output_dir: path to a directory where the compilation output should be generated
// :param import_keys: | if false, zk-snark of all modified circuits will be generated during compilation
//                     | if true, zk-snark keys for all circuits are expected to be already present in the output directory, \
//                       and the compilation will use the provided keys to generate the verification contracts
//                     | This option is mostly used internally when connecting to a zkay contract provided by a 3rd-party
// :raise ZkayCompilerError: if any compilation stage fails
// :raise RuntimeError: if import_keys is true and zkay file, manifest file or any of the key files is missing
// """
fn compile_zkay(code: &str, output_dir: &str, import_keys: bool) {
    // -> (CircuitGenerator, String)
    // Copy zkay code to output
    let zkay_filename = "contract.zkay";
    if import_keys
        && !PathBuf::from(output_dir)
            .join(zkay_filename)
            .try_exists()
            .unwrap_or(false)
    {
        //  raise RuntimeError('Zkay file is expected to already be in the output directory when importing keys')
        assert!(
            false,
            "Zkay file is expected to already be in the output directory when importing keys"
        );
    } else if !import_keys {
        _dump_to_output(code, output_dir, zkay_filename, false);
    }
    let global_vars = RcCell::new(global_vars(RcCell::new(global_defs())));
    // Type checking
    let zkay_ast = get_processed_ast(code, None, global_vars.clone());

    // Contract transformation
    print_step("Transforming zkay -> public contract");
    let (ast, circuits) = transform_ast(Some(zkay_ast.clone()), global_vars.clone());

    // Dump libraries
    print_step("Write library contract files");
    CFG.lock().unwrap().library_compilation_environment();
    // println!("=========================={}",line!());
    for crypto_params in ast
        .try_as_source_unit_ref()
        .unwrap()
        .borrow()
        .used_crypto_backends
        .clone()
        .unwrap()
    {
        // println!("=========================={}",line!());
        // Write pki contract
        let pki_contract_code = library_contracts::get_pki_contract(&crypto_params);
        // println!("=========================={}",line!());
        let pki_contract_file = format!(
            "{}.sol",
            CFG.lock()
                .unwrap()
                .get_pki_contract_name(&crypto_params.identifier_name())
        );
        // println!("=========================={}",line!());
        _dump_to_output(&pki_contract_code, output_dir, &pki_contract_file, true);
    }
    // println!("=========================={}",line!());
    // Write library contract
    _dump_to_output(
        &library_contracts::get_verify_libs_code(),
        output_dir,
        &<ProvingSchemeGm17 as ProvingScheme>::verify_libs_contract_filename(),
        true,
    );

    // Write public contract file
    print_step("Write public solidity code");
    let output_filename = "contract.sol";
    let _solidity_code_output =
        _dump_to_output(&to_solidity(&ast), output_dir, output_filename, false);

    // Get all circuit helpers for the transformed contract
    let circuits: Vec<_> = circuits.values().cloned().collect();

    // Generate offchain simulation code (transforms transactions, interface to deploy and access the zkay contract)
    //    let  offchain_simulation_code = PythonOffchainVisitor::new(circuits).visit(ast);//TODO LS
    //     _dump_to_output(offchain_simulation_code, output_dir, "contract.py");

    // Instantiate proving scheme and circuit generator
    // let ps = proving_scheme_classes(
    //     &CFG.lock().unwrap().user_config.proving_scheme(),
    //     // if &CFG.lock().unwrap().user_config.proving_scheme() == "groth16" {
    //     //     ProvingSchemeGroth16
    //     // } else {
    //     //     ProvingSchemeGm17
    //     // },
    // );
    let snark_backend = CFG.lock().unwrap().user_config.snark_backend();
    let proving_scheme = CFG.lock().unwrap().user_config.proving_scheme();
    let cg = generator_classes(&snark_backend)(circuits, proving_scheme, output_dir.to_string());
    // println!("==============={}==", line!());
    let mut kwargs = std::collections::HashMap::new();
    if let Some(_v) = kwargs.get("verifier_names") {
        // assert!(isinstance(v, list));
        let mut verifier_names =
            get_verification_contract_names((None, Some(zkay_ast.clone())), global_vars.clone());
        verifier_names.sort_unstable();
        let mut verifier_contract_type_codes: Vec<_> = cg
            .circuit_generator_base
            .circuits_to_prove
            .iter()
            .map(|cc| {
                cc.borrow()
                    .verifier_contract_type
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .code()
            })
            .collect();
        verifier_contract_type_codes.sort_unstable();
        assert!(verifier_names == verifier_contract_type_codes);
        kwargs.insert("verifier_names", verifier_names.clone());
    }

    // Generate manifest
    if !import_keys {
        print_step("Writing manifest file");
        // Set crypto backends for unused homomorphisms to None
        for hom in Homomorphism::fields() {
            if !ast
                .try_as_source_unit_ref()
                .unwrap()
                .borrow()
                .used_homomorphisms
                .as_ref()
                .unwrap()
                .contains(&hom)
            {
                CFG.lock()
                    .unwrap()
                    .user_config
                    .set_crypto_backend(&hom, String::new());
            }
        }

        let manifest = json!({
            Manifest::zkay_version: CFG.lock().unwrap().zkay_version(),
            Manifest::solc_version: CFG.lock().unwrap().solc_version(),
            Manifest::zkay_options: CFG.lock().unwrap().export_compiler_settings(),
        });
        _dump_to_output(&format!("{manifest}"), output_dir, "manifest.json", false);
    } else if !PathBuf::from(output_dir)
        .join("manifest.json")
        .try_exists()
        .unwrap_or(false)
    {
        // raise RuntimeError("Zkay contract::failed: Manifest file is missing")
        panic!("Zkay contract::failed: Manifest file is missing");
    }

    // Generate circuits and corresponding verification contracts
    cg.generate_circuits(import_keys);

    // Check that all verification contracts and the main contract compile
    let fns = cg
        .circuit_generator_base
        .get_verification_contract_filenames();
    let main_solidity_files = fns
        .iter()
        .map(|v| PathBuf::from(v))
        .chain([PathBuf::from(output_dir).join(output_filename)]);
    for f in main_solidity_files {
        check_compilation(f.to_str().unwrap(), false, "");
    }

    // (cg, solidity_code_output)
}

// def use_configuration_from_manifest(contract_dir: str) -> Any:
//     from zkay.transaction.runtime::Runtime
//     manifest = Manifest.load(contract_dir)
//     Manifest.import_manifest_config(manifest)
//     Runtime.reset()

// def load_transaction_interface_from_directory(contract_dir: str) -> Any:
//     """
//     Load transaction interface module for contracts in contract_dir

//     :param contract_dir: directory with zkay contract compilation output
//     :return: module object
//     """
//     sys.path.append(str(os.path.realpath(contract_dir)))
//     contract_mod = importlib.import_module(f'contract')
//     importlib.reload(contract_mod)
//     sys.path.pop()
//     return contract_mod

// @contextmanager
// def transaction_benchmark_ctx(contract_dir: str, log_filename='log') -> ContextManager[Any]:
//     use_configuration_from_manifest(contract_dir)
//     cfg.verbosity = 0
//     cfg.log_dir = contract_dir
//     log_file = my_logging.get_log_file(filename=log_filename, include_timestamp=false, label=None)
//     my_logging.prepare_logger(log_file)
//     with time_measure('all_transactions', should_print=true):
//         yield load_transaction_interface_from_directory(contract_dir)
//         pass

// def load_contract_transaction_interface_from_module(contract_mod: Any,
//                                                     contract_name: Optional[str] = None) -> Type:
//     """
//     Load contract class from transaction interface module

//     :param contract_mod: loaded transaction interface module
//     :param contract_name: contract name, only required if file contains multiple contracts
//     :return: Contract class
//     """

//     contracts = {}
//     for name, cls in contract_mod.__dict__.items():
//         if isinstance(cls, type) and 'ContractSimulator' in [b.__name__ for b in cls.__bases__]:
//             contracts[cls.__name__] = cls

//     if contract_name is None:
//         if len(contracts) != 1:
//             raise ValueError('If file contains multiple contracts, contract name must be specified')
//         return next(iter(contracts.values()))
//     else:
//         return contracts[contract_name]

// def load_contract_transaction_interface_from_directory(contract_dir: str, contract_name: Optional[str] = None) -> Type:
//     """
//     Load contract class from transaction interface stored in contract_dir

//     :param contract_dir: directory with contract compilation output
//     :param contract_name: contract name, only required if file contains multiple contracts
//     :return: Contract class
//     """
//     contract_mod = load_transaction_interface_from_directory(contract_dir)
//     return load_contract_transaction_interface_from_module(contract_mod, contract_name)

// def deploy_contract(contract_dir: str, account, *args, contract_name: Optional[str] = None):
//     """
//     Deploy zkay contract in contract_dir using the given account and with specified constructor arguments.

//     :param contract_dir: contract's compilation output directory
//     :param account: Account from which to deploy the contract
//     :param args: constructor arguments
//     :param contract_name: contract name, only required if file contains multiple contracts
//     :raise BlockChainError: if deployment fails
//     :return: contract instance
//     """
//     c = load_contract_transaction_interface_from_directory(contract_dir, contract_name)
//     return c.deploy(*args, user=account, project_dir=contract_dir)

// def connect_to_contract_at(contract_dir: str, contract_address, account, contract_name: Optional[str] = None):
//     """
//     Connect with account to zkay contract at contract_address, with local files in contract_dir.

//     :param contract_dir: contract's compilation output directory
//     :param contract_address: blockchain address of the deployed contract
//     :param account: account from which to connect (will be used as msg.sender for transactions)
//     :param contract_name: contract name, only required if file contains multiple contracts
//     :raise BlockChainError: if connection fails
//     :raise IntegrityError: if integrity check fails
//     :return: contract instance
//     """
//     c = load_contract_transaction_interface_from_directory(contract_dir, contract_name)
//     return c.connect(address=contract_address, user=account, project_dir=contract_dir)

// def _collect_package_contents(contract_dir: str, check_all_files: bool) -> List[str]:
//     """
//     Return list of relative paths of all files which should be part of the package for the contract in contract_dir.

//     Raises an exception if contract.zkay, manifest.json or any of the files required by contract.zkay is missing.

//     :param contract_dir: path to directory containing manifest and zkay file
//     :param check_all_files: if true, checks whether all expected files are present
//     :raise FileNotFoundError: if any of the expected files is not present
//     :return: list of relative paths (relative to contract_dir)
//     """

//     zkay_filename = os.path.join(contract_dir, 'contract.zkay')
//     if not os.path.exists(zkay_filename):
//         raise FileNotFoundError('contract.zkay not found in package')

//     manifest_filename = os.path.join(contract_dir, 'manifest.json')
//     if not os.path.exists(manifest_filename):
//         raise FileNotFoundError('manifest.json not found in package')
//     manifest = Manifest.load(contract_dir)

//     files = ['contract.zkay', 'manifest.json']
//     with open(zkay_filename) as f:
//         verifier_names = get_verification_contract_names(f.read())
//     with Manifest.with_manifest_config(manifest):
//         gen_cls = generator_classes[cfg.snark_backend]
//         files += [os.path.join(cfg.get_circuit_output_dir_name(v), k)
//                   for k in gen_cls.get_vk_and_pk_filenames() for v in verifier_names]

//     if check_all_files:
//         for f in files:
//             path = os.path.join(contract_dir, f)
//             if not os.path.exists(path) or not os.path.isfile(path):
//                 raise FileNotFoundError(f)
//     return files

// def package_zkay_contract(zkay_output_dir: str, output_filename: str):
//     """Package zkay contract for distribution."""
//     if not output_filename.endswith('.zkp'):
//         output_filename += '.zkp'

//     with print_step('Packaging for distribution'):
//         files = _collect_package_contents(zkay_output_dir, true)

//         with tempfile.TemporaryDirectory() as tmpdir:
//             for file in files:
//                 src = os.path.join(zkay_output_dir, file)
//                 dest = os.path.join(tmpdir, file)
//                 os.makedirs(os.path.dirname(dest), exist_ok=true)
//                 shutil.copyfile(src, dest)
//             shutil.make_archive(without_extension(output_filename), 'zip', tmpdir)
//         os.rename(f'{without_extension(output_filename)}.zip', output_filename)

// def extract_zkay_package(zkp_filename: str, output_dir: str):
//     """
//     Unpack and compile a zkay contract.

//     :param zkp_filename: path to the packaged contract
//     :param output_dir: directory where to unpack and compile the contract
//     :raise Exception: if::fails
//     """
//     os.makedirs(output_dir)
//     try:
//         with zipfile.ZipFile(zkp_filename) as zkp:
//             with print_step('Checking zip file integrity'):
//                 if zkp.testzip() is not None:
//                     raise ValueError('Corrupt archive')

//             with print_step('Checking for correct file structure'):
//                 zkp.extract('contract.zkay', output_dir)
//                 zkp.extract('manifest.json', output_dir)
//                 expected_files = sorted(_collect_package_contents(output_dir, false))
//                 contained_files = sorted([d.filename for d in zkp.infolist() if not d.is_dir()])
//                 if expected_files != contained_files:
//                     raise ValueError(f'Package is invalid, does not match expected contents')

//             with print_step('Extracting archive'):
//                 zkp.extractall(output_dir)

//         // Compile extracted contract
//         zkay_filename = os.path.join(output_dir, 'contract.zkay')
//         manifest = Manifest.load(output_dir)
//         with Manifest.with_manifest_config(manifest):
//             compile_zkay_file(zkay_filename, output_dir, import_keys=true)
//     except Exception as e:
//         // If there was an exception, the archive is not safe -> remove extracted contents
//         print(f'Package {zkp_filename} is either corrupt or incompatible with this zkay version.')
//         shutil.rmtree(output_dir)
//         raise e

// """
// Dump 'content' into file 'output_dir/filename' and optionally check if it compiles error-free with solc.

// :raise SolcException: if dryrun_solc is true and there are compilation errors
// :return: dumped content as string
// """
fn _dump_to_output(content: &str, output_dir: &str, filename: &str, _dryrun_solc: bool) -> String {
    use std::io::Write;
    //  println!("====================={:?},{:?}",1,filename);
    let path = std::path::Path::new(output_dir).join(filename);
    // println!("====================={:?},{:?}",path,1);
    let mut f = std::fs::File::create(path).expect("create file {path} fail");
    write!(f, "{content}").expect("write content fail");

    // if dryrun_solc {
    //     check_compilation(path, false);
    // }
    content.to_string()
}
