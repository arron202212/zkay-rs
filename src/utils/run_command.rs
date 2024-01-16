// import os
// import subprocess

use crate::config::CFG;
// from typing import List, Optional, Tuple

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
    // :return: command output and error output (if not (allow_verbose and CFG.lock().unwrap().verbosity))
    // """
    //cwd=None, allow_verbose: bool = False
{
    let cwd=if let Some(cwd) = cwd {
        std::fs::canonicalize(cwd)
    }else{""};

    let (output, error) =
        if allow_verbose && CFG.lock().unwrap().verbosity >= 2 && !CFG.lock().unwrap().is_unit_test
        {
            process = subprocess.Popen(cmd, cwd = cwd);
            process.communicate() // will be None
        } else {
            //run
            let process = subprocess.Popen(
                cmd,
                stdout = subprocess.PIPE,
                stderr = subprocess.PIPE,
                cwd = cwd,
            );

            //collect output
            let (mut output, mut error) = process.communicate();

            //decode output
            output = output.decode("utf-8").rtrim();
            error = error.decode("utf-8").rtrim();
            (output, error)
        };

    //check for error
    if process.returncode != 0 {
        let cmd = get_command(cmd);
        // raise subprocess.SubprocessError(msg)
        assert!(
            false,
            "Non-zero exit status {} for command:\n{cwd}: $ {cmd}\n\n{output}\n{error}",
            process.returncode
        );
    } else if CFG.lock().unwrap().verbosity >= 2 {
        print!("Ran command {}:\n\n{output}\n{error}", get_command(cmd));
    }

    (output, error)
}

pub fn get_command(cmd: Vec<&str>) -> String {
    fn format_part(p: &str) {
        if p.contains(" ") {
            format!(r#""{p}""#)
        } else {
            p
        }
    }

    cmd.iter()
        .map(|p| format_part(p))
        .collect::<Vec<_>>()
        .join(" ")
}
