#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::structure::{circuit_generator::CircuitGenerator, wire_array, wire_type::WireType},
    util::{
        run_command::run_command,
        util::{BigInteger, Util},
    },
    zkay::{zkay_type::ZkayType, zkay_util::wire_array::WireArray},
};
use num_bigint::Sign;
use rccell::RcCell;
use std::io::BufReader;

pub struct ZkayUtil;
impl ZkayUtil {
    pub const ZKAY_RESTRICT_EVERYTHING: bool = false; // if set to true for debugging, each typed wire constructor restricts bitwidth (rather than just  inputs)

    pub fn reverseBytes(
        bitArray: WireArray,
        targetWordBits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        WireArray::new(
            Util::reverseBytes(bitArray.as_array()),
            generator.downgrade(),
        )
        .pack_bits_into_words(targetWordBits as usize, &None)
    }

    pub fn unsignedBytesToBigInt(bytes: &[u8]) -> BigInteger {
        let signum = if bytes.iter().all(|&b| b == 0) {
            Sign::NoSign
        } else {
            Sign::Plus
        };

        BigInteger::from_bytes_be(signum, bytes)
    }

    pub fn unsignedBigintToBytes(val: BigInteger) -> Vec<u8> {
        let (_, b) = val.to_bytes_be();
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
        let byte_count = byteCount as usize;
        let t = Self::unsignedBigintToBytes(val);
        assert!(
            t.len() <= byte_count,
            "Value too large to fit into {byte_count} bytes"
        );
        let mut ret = vec![0; byte_count];
        ret[byte_count - t.len()..byte_count].clone_from_slice(&t);
        ret
    }

    pub fn runZkayJsnarkInterface() {
        let (Some(p), _) = run_command(
            vec![
                "../libsnark/build/libsnark/zkay_interface/run_snark",
                "keygen",
                ".",
                ".",
                "1",
            ],
            None,
            false,
        ) else {
            return;
        };
        println!(
            "\n-----------------------------------RUNNING LIBSNARK KEYGEN -----------------------------------------"
        );
        let input = p.split("\n");
        let mut buf = String::new();
        for line in input {
            buf.push_str(line);
            buf.push_str("\n");
        }

        //println!(buf.toString());

        let (Some(p), _) = run_command(
            vec![
                "../libsnark/build/libsnark/zkay_interface/run_snark",
                "proofgen",
                ".",
                "proof.out",
                ".",
                "1",
                "1",
            ],
            None,
            false,
        ) else {
            return;
        };
        println!(
            "\n-----------------------------------RUNNING LIBSNARK PROOFGEN -----------------------------------------"
        );
        let input = p.split("\n");
        let mut buf = String::new();
        for line in input {
            buf.push_str(line);
            buf.push_str("\n");
        }

        //println!(buf.toString());
    }
}
