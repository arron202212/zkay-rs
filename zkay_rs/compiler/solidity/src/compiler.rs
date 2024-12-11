#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

// import json
// import os
// import pathlib
// import tempfile
// // get relevant paths
// from typing import Optional, Dict, Tuple

// from solcx import compile_standard
// from solcx.exceptions import SolcError

use serde_json::{Map, Result, Value};
use std::io::Read;
use std::path::PathBuf;
use zkay_ast::ast::get_code_error_msg;
use zkay_config::{config::CFG, config_user::UserConfig, zk_print};
// class SolcException(Exception):
//     """ Solc reported error """
//     pass
use std::collections::BTreeMap;
use std::env::{current_dir, set_current_dir};
use std::fs::File;

// Compile the given solidity file using solc json interface with the provided options.

// :param sol_filename: path to solidity file
// :param libs: [OPTIONAL] dictionary containing <LibraryContractName, LibraryContractAddress> pairs, used for linking
// :param optimizer_runs: controls the optimize-runs flag, negative values disable the optimizer
// :param output_selection: determines which fields are included in the compiler output dict
// :param cwd: working directory
// :return: dictionary with the compilation results according to output_selection
pub fn compile_solidity_json(
    sol_filename: &str,
    libs: Option<BTreeMap<String, String>>,
    optimizer_runs: i32,
    output_selection: Vec<String>,
    cwd: &str,
) -> Option<Value> {
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
            r#",{{
            "enabled": "true",
            "runs": {optimizer_runs}
        }}"#
        )
    } else {
        String::new()
    };

    let libraries = if let Some(libs) = libs {
        format!(
            ",{{
            {sol_filename}: {:?}
        }}",
            libs
        )
    } else {
        String::new()
    };
    let solps = std::fs::canonicalize(sol_filename).unwrap();
    let solp = PathBuf::from(sol_filename);
    let mut json_in = format!(
        r#"{{
        "language": "Solidity",
        "sources": {{
            "{sol_filename}": {{
                "urls": [
                    {solps:?}
                ]
            }}
        }},
        "settings": {{
            "outputSelection": {{
                "*": {{"*": {output_selection:?}  }}
            }}
            {optimizer}{libraries}
          }}
   }}"#
    );

    let mut cwd = cwd.to_string();
    if cwd.is_empty() {
        cwd = std::fs::canonicalize(solp.clone())
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    }
    let input_json_file_path = json_input(&cwd, &json_in);
    let old_cwd = std::env::current_dir().unwrap();
    let _ = set_current_dir(&cwd);
    let ret = compile(&json_in, &cwd, &input_json_file_path);
    let _ = set_current_dir(old_cwd);
    ret
}
pub fn json_input(dir: &str, json_in: &str) -> String {
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Result;
    use std::io::{BufRead, BufReader, Error, Write};
    use uuid::Uuid;

    // let mut dir = temp_dir();
    // println!("{}", dir.to_str().unwrap());

    // let file_name = format!("{}.json", Uuid::new_v4());
    // println!("{}", file_name);
    // dir.push(json_in.clone());
    let input_json_file_path = dir.to_owned() + "input.json";
    let mut file = File::create(input_json_file_path.clone()).unwrap();
    write!(file, "{}", json_in).expect("write file failed");
    // dump fake solidity code into temporary file
    // with tempfile.NamedTemporaryFile('w', suffix='.sol') as f
    //     f.write(fake_solidity_code)
    //     f.flush()
    // check_compilation(dir.to_str().unwrap(), true, zkay_code);
    input_json_file_path
}
//TODO
fn compile(input: &str, solp: &str, input_json_file_path: &str) -> Option<Value> {
    // println!("====compile======TODO==={}======",input);
    let output = solc_compile(&input, solp, input_json_file_path); //String::from("{}"); //
    assert_ne!(output.len(), 0);
    serde_json::from_str(&output).ok()
}
fn solc_compile(_input: &str, solp: &str, input_json_file_path: &str) -> String {
    use std::fs::File;
    use std::process::{Command, Stdio};

    // With the `foo.txt` file containing "Hello, world!"
    let input = File::open(input_json_file_path).unwrap();

    // let input = Command::new("echo")
    //     .arg(_input)
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("failed echo command");
    let arg = format!("{solp},.");
    let mut solc = Command::new("/Users/lisheng/mygit/arron/zkay-rs/solc-macos")
        .args(["--standard-json", "--allow-paths", &arg])
        .stdin(input)
        .output()
        .expect("failed solc command");
    let output = String::from_utf8(solc.stdout).unwrap();
    // println!(
    //     "==erro=={}========output==={output}",
    //     String::from_utf8(solc.stderr).unwrap()
    // );
    let _ = std::fs::remove_file(input_json_file_path);
    output
}

// """ Get line and column (1-based) from character index """
fn _get_line_col(code: &str, idx: i32) -> (i32, i32) {
    let i = idx as usize;
    let line = code[..i + 1].split("\n").count() as i32;
    let col = idx - (code[..i + 1].rfind("\n").unwrap() as i32 + 1);
    (line, col)
}

pub fn get_error_order_key(error: &Value) -> i64 {
    if let Value::Object(error) = error {
        if let Some(Value::Object(error)) = error.get(&String::from("sourceLocation")) {
            if let Some(Value::Number(error)) = error.get(&String::from("start")) {
                if let Some(error) = error.as_i64() {
                    return error;
                }
            }
        }
    }
    -1
}

// Run the given file through solc without output to check for compiler errors.

// :param filename: file to dry-compile
// :param show_errors: if true, errors and warnings are printed
// :param display_code: code to use when displaying the compiler errors
// :raise SolcException: raised if solc reports a compiler error
pub fn check_compilation(filename: &str, show_errors: bool, display_code: &str) {
    let p = PathBuf::from(filename);
    let sol_name = p.file_name().unwrap();
    let mut f = File::open(filename).unwrap();
    let mut code = String::new();
    f.read_to_string(&mut code).unwrap();
    // println!("==filename========{filename}==code={code}");
    let display_code = if display_code.is_empty() {
        code.clone()
    } else {
        String::from(display_code)
    };

    let mut had_error = false;
    // try:
    let errors = compile_solidity_json(filename, None, -1, vec![], "");
    if !show_errors || errors.is_none() {
        return;
    }
    // except SolcError as e:
    //     errors = json.loads(e.stdout_data)
    //     if not show_errors:
    //         raise SolcException()
    // if solc reported any errors or warnings, print them and throw exception
    if let Some(errors) = errors.unwrap().get_mut(&String::from("errors")) {
        zk_print!("");
        if let Value::Array(errors) = errors {
            errors.sort_unstable_by_key(|x| get_error_order_key(x));

            let mut fatal_error_report = String::new();
            for error in errors {
                if !error.is_object() {
                    continue;
                }
                let error = error.as_object().unwrap();
                use zkay_utils::progress_printer::{colored_print, TermColor};
                let is_error = error.get(&String::from("severity"))
                    == Some(&Value::String(String::from("error")));

                colored_print(if is_error {
                    TermColor::FAIL
                } else {
                    TermColor::WARNING
                });
                let mut report =
                    if let Some(Value::Object(sourceLocation)) = error.get("sourceLocation") {
                        let file = sourceLocation.get("file").unwrap().as_str().unwrap();
                        let start = sourceLocation
                            .get(&String::from("start"))
                            .unwrap()
                            .as_i64()
                            .unwrap() as i32;
                        if file == sol_name {
                            let (line, column) = _get_line_col(&code, start);
                            had_error |= is_error;
                            format!(
                                "{:?}\n",
                                get_code_error_msg(
                                    line,
                                    column + 1,
                                    display_code.split("\n").map(String::from).collect(),
                                    None,
                                    None,
                                    None
                                )
                            )
                        } else {
                            format!("In imported file \"{file}\" idx: {}\n", start)
                        }
                    } else {
                        String::new()
                    };
                report = format!(
                    "\n{}: {}\n{report}\n{}\n",
                    error
                        .get(&String::from("severity"))
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_ascii_uppercase(),
                    if is_error {
                        error.get(&String::from("type")).unwrap().as_str().unwrap()
                    } else {
                        ""
                    },
                    error
                        .get(&String::from("message"))
                        .unwrap()
                        .as_str()
                        .unwrap()
                );

                if is_error {
                    fatal_error_report += &report;
                } else if !error.contains_key("errorCode")
                    || String::from("1878")
                        != error
                            .get(&String::from("errorCode"))
                            .unwrap()
                            .as_str()
                            .unwrap()
                // Suppress SPDX license warning
                {
                    zk_print!("{:?}", report);
                }
            }

            zk_print!("");
            if had_error {
                // raise SolcException(fatal_error_report)
                panic!("{}", fatal_error_report);
            }
        }
    }
}

// Run fake solidity code (stripped privacy features) through solc and report errors in the context of the original zkay code.
// Fake solidity code = zkay code with privacy features removed in a source-location preserving way (whitespace padding)
// :param zkay_code: Original zkay code
// :param fake_solidity_code: Corresponding "fake solidity code"
pub fn check_for_zkay_solc_errors(zkay_code: &str, fake_solidity_code: &str) {
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Result;
    use std::io::{BufRead, BufReader, Error, Write};
    use uuid::Uuid;

    let mut dir = temp_dir();
    println!("{}", dir.to_str().unwrap());

    let file_name = format!("{}.sol", Uuid::new_v4());
    println!("{}", file_name);
    dir.push(file_name.clone());

    let mut file = File::create(dir.clone()).unwrap();
    write!(file, "{}", fake_solidity_code).expect("write file failed");
    // dump fake solidity code into temporary file
    // with tempfile.NamedTemporaryFile('w', suffix='.sol') as f
    //     f.write(fake_solidity_code)
    //     f.flush()
    check_compilation(dir.to_str().unwrap(), true, zkay_code);
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
