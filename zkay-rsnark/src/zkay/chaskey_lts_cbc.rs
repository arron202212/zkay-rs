#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
// import org.bouncycastle.crypto.BufferedBlockCipher;
// import org.bouncycastle.crypto.CipherParameters;
// import org.bouncycastle.crypto.InvalidCipherTextException;
// import org.bouncycastle.crypto.modes.CBCBlockCipher;
// import org.bouncycastle.crypto.params.KeyParameter;
// import org.bouncycastle.crypto.params.ParametersWithIV;

use crate::{
    util::util::BigInteger,
    zkay::{chaskey_lts_engine::ChaskeyLTSEngine, zkay_util::ZkayUtil},
};

pub struct CBCBlockCipher;
impl CBCBlockCipher {
    pub fn new() -> Self {
        Self
    }
}
pub struct BufferedBlockCipher;
impl BufferedBlockCipher {
    pub fn new() -> Self {
        Self
    }
    pub fn getOutputSize(&self, len: usize) -> usize {
        len
    }
    pub fn processBytes(
        &self,
        ins: &Vec<u8>,
        inOff: usize,
        len: usize,
        out: &Vec<u8>,
        outOff: usize,
    ) -> i32 {
        0
    }
    pub fn doFinal(&self, out: &Vec<u8>, outOff: i32) -> i32 {
        outOff
    }
}
pub struct ParametersWithIV;
impl ParametersWithIV {
    pub fn new() -> Self {
        Self
    }
}
pub struct KeyParameter;
impl KeyParameter {
    pub fn new(key: [u8; 16]) -> Self {
        Self
    }
}
#[derive(Debug, Clone)]
pub struct ChaskeyLtsCbc;
impl ChaskeyLtsCbc {
    fn parse(val: &String, len: i32) -> Vec<u8> {
        ZkayUtil::unsignedBigintToBytesi(BigInteger::parse_bytes(val.as_bytes(), 16).unwrap(), len)
    }

    const blocksize: i32 = 16;
    const ivlen: i32 = Self::blocksize;
    const keylen: i32 = Self::blocksize;
    const msglen: i32 = 2 * Self::blocksize; // Must be multiple of blocksize

    pub fn crypt(encrypt: bool, key: &Vec<u8>, iv: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {
        // Initialize chaskey cipher in cbc mode
        // let chaskeyEngine = ChaskeyLTSEngine::new();
        // let cbc = CBCBlockCipher::new(chaskeyEngine);
        let cipher = BufferedBlockCipher::new(); // Don't need padding since size is always statically known in zkay and input is multiple of block size
        // let params = ParametersWithIV::new(KeyParameter::new(key), iv);
        // cipher.init(encrypt, params);

        // Encrypt / Decrypt
        assert!(
            cipher.getOutputSize(input.len()) == input.len(),
            "Wrong size"
        );
        let outbuf = vec![0; cipher.getOutputSize(input.len())];
        let out_size = cipher.processBytes(&input, 0, input.len(), &outbuf, 0);
        assert!(
            cipher.doFinal(&outbuf, out_size) == 0,
            "Input not aligned to block size"
        );

        outbuf
    }
}
pub fn main(args: Vec<String>) {
    // Parse inputs
    assert!(
        args.len() == 4,
        "expected 4 arguments [enc|dec, key, iv, plain|cipher]"
    );
    assert!(
        args[0] == "enc" || args[0] == "dec",
        "First argument must be either 'enc' or 'dec'"
    );
    let enc = args[0] == "enc";

    let key = ChaskeyLtsCbc::parse(&args[1], ChaskeyLtsCbc::keylen);
    let iv = ChaskeyLtsCbc::parse(&args[2], ChaskeyLtsCbc::ivlen);
    let input = ChaskeyLtsCbc::parse(&args[3], ChaskeyLtsCbc::msglen);

    // Perform encryption/decryption
    let output = ChaskeyLtsCbc::crypt(enc, &key, &iv, &input);

    // Output result
    //println!(unsignedBytesToBigInt(output).toString(16));
}
