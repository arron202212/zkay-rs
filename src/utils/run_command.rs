// import os
// import subprocess
use crate::config::CFG;
use std::process::{Command, Stdio};
// from typing import List, Optional, Tuple
use std::path::PathBuf;
pub fn run_command(
    cmd: Vec<&str>,
    cwd: Option<&str>,
    allow_verbose: bool,
) -> (Option<String>, Option<String>)
// """
    // Run arbitrary command.

    // :param cmd: the command to run (list of command and arguments)
    // :param cwd: if specified, use this path as working directory (otherwise current working directory is used)
    // :param allow_verbose: if true, redirect command output to stdout (WARNING, causes return values to be None)
    // :return: command output and error output (if not (allow_verbose and CFG.lock().unwrap().user_config.verbosity))
    // """
    //cwd=None, allow_verbose: bool = False
{
    let cwd = if let Some(cwd) = cwd {
        std::fs::canonicalize(cwd).unwrap().to_str().unwrap()
    } else {
        ""
    };

    let (output, error, process) = if allow_verbose
        && CFG.lock().unwrap().user_config.verbosity() >= 2
        && !CFG.lock().unwrap().is_unit_test()
    {
        let process = Command::new(cmd.join(" "))
            .current_dir(cwd)
            .output()
            .expect("");
        (process.stdout, process.stderr, process)
    } else {
        //run
        let process = Command::new(cmd.join(" "))
            .current_dir(cwd)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("")
            .wait_with_output()
            .expect("");

        //collect output
        //decode output
        //     let output = output.decode("utf-8").rtrim();
        //    let  error = error.decode("utf-8").rtrim();
        (process.stdout, process.stderr, process)
    };

    //check for error
    if !process.status.success() {
        let cmd = get_command(cmd);
        // raise subprocess.SubprocessError(msg)
        assert!(
            false,
            "Non-zero exit status {} for command:\n{cwd}: $ {cmd}\n\n{output:?}\n{error:?}",
            process.status
        );
    } else if CFG.lock().unwrap().user_config.verbosity() >= 2 {
        print!("Ran command {}:\n\n{output:?}\n{error:?}", get_command(cmd));
    }

    (
        String::from_utf8(output).ok(),
        String::from_utf8(error).ok(),
    )
}

pub fn get_command(cmd: Vec<&str>) -> String {
    fn format_part(p: &str) -> String {
        if p.contains(" ") {
            format!(r#""{p}""#)
        } else {
            p.to_owned()
        }
    }

    cmd.iter()
        .map(|p| format_part(p))
        .collect::<Vec<_>>()
        .join(" ")
}
