// import os
// import re
// import hashlib
// from typing import Optional, List
// from zkay.compiler.solidity.fake_solidity_generator import WS_PATTERN, ID_PATTERN
use rs_sha512::{HasherContext, Sha512State};
use std::fs::File;
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};

pub fn save_to_file(output_directory: Option<&str>, filename: &str, code: &str) -> String {
    let target = if let Some(output_directory) = output_directory {
        Path::new(output_directory).join(filename)
    } else {
        PathBuf::from(filename)
    };
    let mut f = File::create(target).expect("");
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
    // let digest = hashlib.sha512(data).digest();
    let mut sha512hasher = Sha512State::default().build_hasher();
    sha512hasher.write(data.as_bytes());
    let digest = sha512hasher.finish();
    let bytes_result = HasherContext::finish(&mut sha512hasher);
    let digest = format!("{bytes_result:02x}");
    assert!(digest.len() == 64);
    digest[..32].bytes().collect()
}

pub fn hash_file(filename: &str, mut chunk_size: i32) -> Vec<u8> {
    if chunk_size == 0 {
        chunk_size = 1 << 27;
    }
    //chunk_size: int = 1 << 27
    // let mut digest = hashlib.sha512();
    let mut digest = Sha512State::default().build_hasher();
    let f = File::open(filename).expect("");
    loop {
        // Hash prover key in 128mb chunks
        let mut data = vec![0; chunk_size as usize];
        let res = f.read(&mut data);
        if res.is_err() {
            break;
        }
        digest.write(&data);
    }

    // let digest = digest.finish();
    let bytes_result = HasherContext::finish(&mut digest);
    let digest = format!("{bytes_result:02x}");
    assert!(digest.len() == 64);
    digest[..32].bytes().collect()
}

// pub fn without_extension(filename: str) -> str
//     ext_idx = filename.rfind(".")
//     ext_idx = len(filename) if ext_idx == -1 else ext_idx
//     return filename[:ext_idx]

// pub fn get_contract_names(sol_filename: str) -> List[str]
//     with open(sol_filename) as f
//         s = f.read()
//         matches = re.finditer(f"contract{WS_PATTERN}*({ID_PATTERN}){WS_PATTERN}*{{", s)
//         return [m.group(1) for m in matches]

// pub fn prepend_to_lines(text: str, pre: str)
//     return pre + text.replace("\n", "\n" + pre)

pub fn lines_of_code(code: &str) -> i32 {
    code.split("\n")
        .filter_map(|l| {
            if !l.trim().is_empty() && !l.trim().starts_with("//") {
                Some(l)
            } else {
                None
            }
        })
        .count() as _
}
