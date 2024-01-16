// import json
// import os
// import pathlib
// import tempfile
// // get relevant paths
// from typing import Optional, Dict, Tuple

// from solcx import compile_standard
// from solcx.exceptions import SolcError

use crate::{zk_print, config::CFG};
use crate::zkay_ast::ast::get_code_error_msg;

// class SolcException(Exception):
//     """ Solc reported error """
//     pass
 use std::fs::File;
use std::collections::HashMap;
use std::env::{current_dir, set_current_dir};
fn compile_solidity_json(
    sol_filename: &str,
    libs: Option<HashMap<String, String>>,
    optimizer_runs: i32,
    output_selection: Vec<String>,
    cwd: &str,
) -> HashMap<String, String>
// """
    // Compile the given solidity file using solc json interface with the provided options.

    // :param sol_filename: path to solidity file
    // :param libs: [OPTIONAL] dictionary containing <LibraryContractName, LibraryContractAddress> pairs, used for linking
    // :param optimizer_runs: controls the optimize-runs flag, negative values disable the optimizer
    // :param output_selection: determines which fields are included in the compiler output dict
    // :param cwd: working directory
    // :return: dictionary with the compilation results according to output_selection
    // """
{
    // sol_filename: str, libs: Optional[Dict[str, str]] = None, optimizer_runs: int = -1,
    //                           output_selection: Tuple = ('metadata', 'evm.bytecode', 'evm.deployedBytecode'),
    //                           cwd: str = None) -> Dict
    // let relative_path = PathBuf::from("cargo_home");
    // let mut absolute_path = try!(std::env::current_dir());
    // absolute_path.push(relative_path)
    // let absolute_path = if path.is_absolute() {
    //     path.to_path_buf()
    // } else {
    //     env::current_dir()?.join(path)
    // }
    let optimizer = if optimizer_runs >= 0 {
        format!(
            r#"{{
            "enabled": "true",
            "runs": {optimizer_runs}
        }}"#
        )
    } else {
        String::new()
    };

    let libraries = if libs.is_some() {
        format!(
            "{{
            {sol_filename}: {libs}
        }}"
        )
    } else {
        String::new()
    };
    let solps = std::fs::canonicalize(sol_filename);
    let solp = PathBuf::from(sol_filename);
    let mut json_in = format!(
        r#"{{
        "language": "Solidity",
        "sources": {{
            "{sol_filename}": {{
                "urls": [
                    {solps}
                ]
            }}
        }},
        "settings": {{
            "outputSelection": {{
                "*": {{"*": {output_selection}  }}
            }},
            {optimizer}{libraries}
          }}
   }}"#
    );

    if cwd.is_empty() {
        cwd = std::fs::canonicalize(solp).parent();
    }
    let old_cwd = std::env::current_dir();
    set_current_dir(cwd);
    let ret = solc::compile(&json_in);
    set_current_dir(old_cwd);
    ret
}

fn _get_line_col(code: str, idx: i32) -> (i32, i32)
// """ Get line and column (1-based) from character index """
{
    let line = code[..idx + 1].split("\n").count();
    let col = idx - (code[..idx + 1].rfind("\n") + 1);
    (line, col)
}

pub fn get_error_order_key(error: Map) -> i32 {
    if let Some(e) = error.get("sourceLocation") {
        *e.get("start").unwrap_or(&-1)
    } else {
        -1
    }
}
use serde_json::{Map, Result, Value};
use std::path::PathBuf;
pub fn check_compilation(filename: &str, show_errors: bool, display_code: &str)
// """
// Run the given file through solc without output to check for compiler errors.

// :param filename: file to dry-compile
// :param show_errors: if true, errors and warnings are printed
// :param display_code: code to use when displaying the compiler errors
// :raise SolcException: raised if solc reports a compiler error
// """
{
    let sol_name = PathBuf::PathBuf(filename).name;
    let mut f = File::open(filename).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code)?;

    let display_code = if display_code.is_empty() {
        code
    } else {
        display_code
    };

    let mut had_error = false;
    // try:
    let errors = compile_solidity_json(filename, None, -1, vec![], "");
    if !show_errors {
        return;
    }
    // except SolcError as e:
    //     errors = json.loads(e.stdout_data)
    //     if not show_errors:
    //         raise SolcException()
    let v: Value = serde_json::from_str(&errors).unwrap();
    // if solc reported any errors or warnings, print them and throw exception
    if let Value::Object(errors) = v {
        if errors.contains("errors") {
            zk_print!("");
            let mut errors = errors["errors"].clone();
            errors.sort_unstable_by_key(|x| get_error_order_key(x));

            let mut fatal_error_report = String::new();
            for error in errors {
                use crate::utils::progress_printer::{colored_print, TermColor};
                let is_error = error.get("severity") == Some("error");

                colored_print(if is_error {
                    TermColor::FAIL
                } else {
                    TermColor::WARNING
                });
                let mut report =if error.contains_key("sourceLocation") {
                    let file = error["sourceLocation"]["file"];
                    if file == sol_name {
                        let (line, column) = _get_line_col(code, error["sourceLocation"]["start"]);
                        had_error |= is_error;
                        format!(
                            "{:?}\n",
                            get_code_error_msg(line, column + 1, display_code.split("\n"))
                        )
                    } else {
                         format!(
                            "In imported file \"{file}\" idx: {}\n",
                            error["sourceLocation"]["start"]
                        )
                    }
                }else{String::new()};
                report = format!(
                    "\n{}: {}\n{report}\n{}\n",
                    error["severity"].upper_ascii_case(),
                    if is_error { error["type"] } else { "" },
                    error["message"]
                );

                if is_error {
                    fatal_error_report += &report;
                } else if !error.contains_key("errorCode") || !["1878"].contains(error["errorCode"])
                // Suppress SPDX license warning
                {
                    zk_print(report);
                }
            }

            zk_print!("");
            if had_error {
                // raise SolcException(fatal_error_report)
                assert!(false, "{}", fatal_error_report);
            }
        }
    }
}

pub fn check_for_zkay_solc_errors(zkay_code: &str, fake_solidity_code: &str)
// """
// Run fake solidity code (stripped privacy features) through solc and report errors in the context of the original zkay code.
// Fake solidity code = zkay code with privacy features removed in a source-location preserving way (whitespace padding)
// :param zkay_code: Original zkay code
// :param fake_solidity_code: Corresponding "fake solidity code"
// """
{
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Result;
    use std::io::{BufRead, BufReader, Error, Write};
    use uuid::Uuid;

    let mut dir = temp_dir();
    println!("{}", dir.to_str().unwrap());

    let file_name = format!("{}.sol", Uuid::new_v4());
    println!("{}", file_name);
    dir.push(file_name);

    let mut file = File::create(dir).unwrap();
    write!(file, "{}", fake_solidity_code);
    // dump fake solidity code into temporary file
    // with tempfile.NamedTemporaryFile('w', suffix='.sol') as f
    //     f.write(fake_solidity_code)
    //     f.flush()
    check_compilation(file_name, true, zkay_code);
}

// def compile_solidity_code(code: str, working_directory: Optional[str] = None, optimizer_runs=cfg.opt_solc_optimizer_runs) -> Dict:
//     """
//     Compile the given solidity code with default settings.

//     :param code: code to compile
//     :param working_directory: (Optional) compiler working directory
//     :param optimizer_runs: solc optimizer argument "runs", a negative value disables the optimizer
//     :return: json compilation output
//     """

//     with tempfile.NamedTemporaryFile('w', suffix='.sol') as f:
//         if working_directory is None:
//             working_directory = os.path.dirname(f.name)
//         elif not os.path.exists(working_directory):
//             os.makedirs(working_directory)

//         f.write(code)
//         f.flush()
//         return compile_solidity_json(f.name, cwd=working_directory, optimizer_runs=optimizer_runs)
