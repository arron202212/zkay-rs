#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        operations::{gadget::Gadget, gadget::GadgetConfig},
        structure::{
            circuit_generator::CircuitGenerator, wire_array::WireArray, wire_type::WireType,
        },
    },
    examples::gadgets::hash::sha256_gadget::SHA256Gadget,
};

use rccell::RcCell;

#[derive(Debug, Clone)]
pub struct ZkaySHA256Gadget {
    pub _uint_output: Vec<Option<WireType>>,
}

impl ZkaySHA256Gadget {
    const bytes_per_word: usize = 32;
    pub fn new(
        uint256_inputs: Vec<Option<WireType>>,
        truncated_bits: i32,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<SHA256Gadget<Self>> {
        let mut _self = SHA256Gadget::<Self>::new(
            Self::convert_inputs_to_bytes(&uint256_inputs, generator.clone()),
            8,
            uint256_inputs.len() * Self::bytes_per_word,
            false,
            true,
            desc,
            generator,
            Self {
                _uint_output: vec![],
            },
        );

        assert!(
            truncated_bits <= 253 && truncated_bits >= 0,
            "Unsupported output length {truncated_bits} bits"
        );
        _self.assemble_output(truncated_bits);
        _self
    }

    fn convert_inputs_to_bytes(
        uint256_inputs: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let mut input_bytes = WireArray::new(uint256_inputs.clone(), generator.downgrade())
            .get_bits(Self::bytes_per_word * 8, &None)
            .pack_bits_into_words(8, &None);
        // Reverse byte order of each input because jsnark reverses internally when packing
        for j in 0..uint256_inputs.len() {
            input_bytes[j * Self::bytes_per_word..(j + 1) * Self::bytes_per_word].reverse();
        }
        input_bytes
    }
}

impl Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn assemble_output(&mut self, truncated_length: i32) {
        let mut digest = self.super_get_output_wires().clone();
        // Invert word order to get correct byte order when packed into one big word below
        digest.reverse();
        if truncated_length < 256 {
            // Keep truncated_length left-most bits as suggested in FIPS 180-4 to shorten the digest
            if truncated_length % 32 == 0 {
                let mut shortened_digest = vec![None; truncated_length as usize / 32];
                let n = shortened_digest.len();
                let m = digest.len().saturating_sub(n);
                shortened_digest.clone_from_slice(&digest[m..]);

                digest = shortened_digest;
            } else {
                self.t.t._uint_output = vec![Some(
                    WireArray::new(digest, self.generator.clone().downgrade())
                        .get_bits(32, &None)
                        .shift_right(256, 256 - truncated_length as usize, &None)
                        .pack_as_bits(None, None, &Some(truncated_length.to_string())),
                )];
                return;
            }
        }
        self.t.t._uint_output = WireArray::new(digest, self.generator.clone().downgrade())
            .pack_words_into_larger_words(32, 8, &None);
        assert!(self.t.t._uint_output.len() == 1, "Wrong wire length");
    }
}

impl GadgetConfig for Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn get_output_wires(&self) -> &Vec<Option<WireType>> {
        &self.t.t._uint_output
    }
}
