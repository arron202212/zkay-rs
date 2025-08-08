#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;

use crate::util::{
    run_command::run_command,
    util::{BigInteger, Util},
};
use crate::zkay::zkay_type::ZkayType;
use crate::zkay::zkay_util::wire_array::WireArray;
use std::io::BufReader;

pub struct ZkayUtil;
impl ZkayUtil {
    pub const ZKAY_RESTRICT_EVERYTHING: bool = false; // if set to true for debugging, each typed wire constructor restricts bitwidth (rather than just  inputs)

    pub fn reverseBytes(bitArray: WireArray, targetWordBits: i32) -> Vec<Option<WireType>> {
        WireArray::new(Util::reverseBytes(bitArray.asArray())).packBitsIntoWords(targetWordBits)
    }

    pub fn unsignedBytesToBigInt(bytes: Vec<u8>) -> BigInteger {
        let signum = bytes.iter().any(|&b| b != 0) as i32;

        BigInteger::new(signum, bytes)
    }

    pub fn unsignedBigintToBytes(val: BigInteger) -> Vec<u8> {
        let b = val.toByteArray();
        let mut ret;
        if b[0] == 0 && b.len() > 1 {
            ret = vec![0; b.len() - 1];
            ret[..b.len() - 1].clone_from_slice(&b[1..]);
        } else {
            ret = b;
        }
        ret
    }

    pub fn unsignedBigintToBytesi(val: BigInteger, byteCount: i32) -> Vec<u8> {
        let t = Self::unsignedBigintToBytes(val);
        assert!(
            t.len() <= byteCount,
            "Value too large to fit into {byteCount} bytes"
        );
        let ret = vec![0; byteCount];
        ret[byteCount - t.len()..byteCount].clone_from_slice(&t);
        ret
    }

    pub fn runZkayJsnarkInterface() {
        let p = run_command(vec![
            "../libsnark/build/libsnark/zkay_interface/run_snark",
            "keygen",
            ".",
            ".",
            "1",
        ]);
        println!(
            "\n-----------------------------------RUNNING LIBSNARK KEYGEN -----------------------------------------"
        );
        let input = BufReader::new(p.getInputStream());
        let mut buf = String::new();
        for line in input.lines() {
            buf.push_str(line);
            buf.push_str("\n");
        }

        //println!(buf.toString());

        let p = run_command(vec![
            "../libsnark/build/libsnark/zkay_interface/run_snark",
            "proofgen",
            ".",
            "proof.out",
            ".",
            "1",
            "1",
        ]);
        println!(
            "\n-----------------------------------RUNNING LIBSNARK PROOFGEN -----------------------------------------"
        );
        let input = BufReader::new(p);
        let buf = String::new();
        for line in input.lines() {
            buf.push_str(line);
            buf.push_str("\n");
        }

        //println!(buf.toString());
    }
}
