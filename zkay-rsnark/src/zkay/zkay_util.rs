#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
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

    pub fn reverse_bytes(
        bit_array: WireArray,
        target_word_bits: i32,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        WireArray::new(
            Util::reverse_bytes(bit_array.as_array()),
            generator.downgrade(),
        )
        .pack_bits_into_words(target_word_bits as usize)
    }

    pub fn unsigned_bytes_to_big_int(bytes: &[u8]) -> BigInteger {
        let signum = if bytes.iter().all(|&b| b == 0) {
            Sign::NoSign
        } else {
            Sign::Plus
        };

        BigInteger::from_bytes_be(signum, bytes)
    }

    pub fn unsigned_bigint_to_bytes(val: BigInteger) -> Vec<u8> {
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

    pub fn unsigned_bigint_to_bytesi(val: BigInteger, byte_count: i32) -> Vec<u8> {
        let byte_count = byte_count as usize;
        let t = Self::unsigned_bigint_to_bytes(val);
        assert!(
            t.len() <= byte_count,
            "Value too large to fit into {byte_count} bytes"
        );
        let mut ret = vec![0; byte_count];
        ret[byte_count - t.len()..byte_count].clone_from_slice(&t);
        ret
    }

    pub fn run_zkay_jsnark_interface() {
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
