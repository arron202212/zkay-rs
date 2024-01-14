// import os
// import re
// import hashlib
// from typing import Optional, List
// from zkay.compiler.solidity.fake_solidity_generator import WS_PATTERN, ID_PATTERN

// pub fn save_to_file(output_directory: Optional[str], filename: str, code: str)
//     if output_directory is not None
//         target = os.path.join(output_directory, filename)
//     else
//         target = filename
//     with open(target, "w") as f
//         f.write(code)
//     return target

pub fn read_file(filename: &str) -> String {
    let f = std::fs::File::open(filename).unwrap();
    use std::io::Read;
    let mut buffered = std::io::BufReader::new(f);
    let mut buf = String::new();
    let _ = buffered.read_to_string(&mut buf);
    buf
}

pub fn hash_string(data: &str) -> Vec<u8> {
    let digest = hashlib.sha512(data).digest();
    assert!(digest.len() == 64);
    digest[..32].to_vec()
}

pub fn hash_file(filename: &str, chunk_size: i32) -> Vec<u8> {
    //chunk_size: int = 1 << 27
    let mut digest = hashlib.sha512();
    let f = open(filename, "rb");
    loop {
        // Hash prover key in 128mb chunks
        let data = f.read(chunk_size);
        if data.is_empty() {
            break;
        }
        digest.update(data);
    }

    digest = digest.digest();
    assert!(digest.len() == 64);
    return digest[..32];
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
