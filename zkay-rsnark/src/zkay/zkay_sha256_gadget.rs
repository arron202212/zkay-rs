#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::Gadget;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::SHA256Gadget;

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
        let _self = SHA256Gadget::<Self>::new(
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
        _self.assembleOutput(truncated_bits);
        _self
    }

    fn convert_inputs_to_bytes(
        uint256_inputs: &Vec<Option<WireType>>,
        generator: RcCell<CircuitGenerator>,
    ) -> Vec<Option<WireType>> {
        let mut input_bytes = WireArray::new(uint256_inputs.clone(), generator.downgrade())
            .getBits(Self::bytes_per_word * 8, &None)
            .packBitsIntoWords(8, &None);
        // Reverse byte order of each input because jsnark reverses internally when packing
        for j in 0..uint256_inputs.len() {
            input_bytes[j * Self::bytes_per_word..(j + 1) * Self::bytes_per_word].reverse();
        }
        input_bytes
    }
}

impl Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn assembleOutput(&mut self, truncated_length: i32) {
        let mut digest = self.getOutputWires().clone();
        // Invert word order to get correct byte order when packed into one big word below
        digest.reverse();
        if truncated_length < 256 {
            // Keep truncated_length left-most bits as suggested in FIPS 180-4 to shorten the digest
            if truncated_length % 32 == 0 {
                let shortened_digest = vec![None; truncated_length as usize / 32];
                shortened_digest.clone_from_slice(&digest[digest.len() - shortened_digest.len()..]);

                digest = shortened_digest;
            } else {
                self.t.t._uint_output = vec![Some(
                    WireArray::new(digest, self.generator.clone().downgrade())
                        .getBits(32, &None)
                        .shiftRight(256, 256 - truncated_length as usize, &None)
                        .packAsBits(None, None, &Some(truncated_length.to_string())),
                )];
                return;
            }
        }
        self.t.t._uint_output = WireArray::new(digest, self.generator.clone().downgrade())
            .packWordsIntoLargerWords(32, 8, &None);
        assert!(self.t.t._uint_output.len() == 1, "Wrong wire length");
    }
}

impl GadgetConfig for Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t._uint_output
    }
}
