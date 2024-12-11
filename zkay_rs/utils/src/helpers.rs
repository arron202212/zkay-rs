#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// import os
// import re
// import hashlib
// from typing import Optional, List
// use solidity::fake_solidity_generator::{ID_PATTERN, WS_PATTERN};
use regex::Regex;
use rs_sha512::{HasherContext, Sha512State};
use std::fs::File;
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};
const WS_PATTERN: &str = r"[ \t\r\n\u000C]";
const ID_PATTERN: &str = r"[a-zA-Z\$_][a-zA-Z0-9\$_]*";
pub fn save_to_file(output_directory: Option<PathBuf>, filename: &str, code: &str) -> String {
    let target = if let Some(output_directory) = output_directory {
        output_directory.join(filename)
    } else {
        PathBuf::from(filename)
    };
    let mut f = File::create(target.clone()).expect("");
    write!(f, "{}", code).expect("");
    target.to_str().unwrap().to_string()
}

pub fn read_file(filename: &str) -> String {
    let f = File::open(filename).unwrap();

    let mut buffered = BufReader::new(f);
    let mut buf = String::new();
    let _ = buffered.read_to_string(&mut buf);
    buf
}

pub fn hash_string(data: &str) -> Vec<u8> {
    // println!("=====3===");
    // let digest = hashlib.sha512(data).digest();
    let mut sha512hasher = Sha512State::default().build_hasher();
    sha512hasher.write(data.as_bytes());
    let _digest = sha512hasher.finish();
    let bytes_result = HasherContext::finish(&mut sha512hasher);
    let digest = format!("{bytes_result:02x}");
    assert!(digest.len() == 128);
    // println!("=====4===");
    digest[..32].bytes().collect()
}

pub fn hash_file(filename: &str, mut chunk_size: i32) -> Vec<u8> {
    if chunk_size == 0 {
        chunk_size = 1 << 27;
    }
    //chunk_size: int = 1 << 27
    // let mut digest = hashlib.sha512();
    let mut digest = Sha512State::default().build_hasher();
    let mut f = File::open(filename).expect(filename);
    use std::time::{Duration, Instant};
    let start = Instant::now();
    for i in 0..10 {
        let start1 = Instant::now();
        // Hash prover key in 128mb chunks
        let mut data = vec![0; chunk_size as usize];
        let res = f.read(&mut data);
        if res.is_err() {
            println!("===hash_file=err=={res:?}========");
            break;
        }
        println!("===hash_file==={i}=======");
        digest.write(&data);
        println!("===hash_file==={i}====={:?}==", start1.elapsed());
    }
    println!("===hash_file=====end===={:?}==", start.elapsed());
    // let digest = digest.finish();
    let bytes_result = HasherContext::finish(&mut digest);
    let digest = format!("{bytes_result:02x}");
    println!(
        "==digest==len===={}={bytes_result:02x}=={bytes_result:?}=======",
        digest.len()
    );
    assert!(digest.len() == 128);
    digest[..32].bytes().collect()
}

// pub fn without_extension(filename: str) -> str
//     ext_idx = filename.rfind(".")
//     ext_idx = len(filename) if ext_idx == -1 else ext_idx
//     return filename[:ext_idx]

pub fn get_contract_names(sol_filename: &str) -> Vec<String> {
    let mut f = File::open(sol_filename).expect("");
    // with open(sol_filename) as f
    // s = f.read()
    // return [m.group(1) for m in matches.ite
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    Regex::new(&format!(
        r"contract{WS_PATTERN}*({ID_PATTERN}){WS_PATTERN}*\{{"
    ))
    .unwrap()
    .captures_iter(&s)
    .map(|c| c.get(1).unwrap().as_str().to_owned())
    .collect()
}

// pub fn prepend_to_lines(text: str, pre: str)
//     return pre + text.replace("\n", "\n" + pre)

pub fn lines_of_code(code: &str) -> i32 {
    code.split('\n')
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//"))
        .count() as _
}
