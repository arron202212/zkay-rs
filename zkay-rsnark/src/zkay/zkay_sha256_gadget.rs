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
use crate::circuit::structure::wire_array;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::hash::sha256_gadget::SHA256Gadget;
use crate::zkay::zkay_sha256_gadget::wire_array::WireArray;

pub struct ZkaySHA256Gadget {
    _uint_output: Vec<Option<WireType>>,
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
            Self::convert_inputs_to_bytes(uint256_inputs),
            8,
            uint256_inputs.len() * bytes_per_word,
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
    }

    fn convert_inputs_to_bytes(uint256_inputs: &Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let input_bytes = WireArray::new(uint256_inputs)
            .getBits(bytes_per_word * 8)
            .packBitsIntoWords(8);
        // Reverse byte order of each input because jsnark reverses internally when packing
        for j in 0..uint256_inputs.len() {
            Collections.reverse(
                Arrays
                    .asList(input_bytes)
                    .subList(j * bytes_per_word, (j + 1) * bytes_per_word),
            );
        }
        input_bytes
    }
}

impl Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn assembleOutput(&self, truncated_length: i32) {
        let mut digest = self.getOutputWires();
        // Invert word order to get correct byte order when packed into one big word below
        digest.reverse();
        if truncated_length < 256 {
            // Keep truncated_length left-most bits as suggested in FIPS 180-4 to shorten the digest
            if truncated_length % 32 == 0 {
                let shortened_digest = vec![None; truncated_length / 32];
                shortened_digest.clone_from_slice(&digest[digest.len() - shortened_digest.len()]);

                digest = shortened_digest;
            } else {
                _uint_output = vec![
                    WireArray::new(digest)
                        .getBits(32)
                        .shiftRight(256, 256 - truncated_length)
                        .packAsBits(None, None, truncated_length),
                ];
                return;
            }
        }
        _uint_output = WireArray::new(digest)
            .packWordsIntoLargerWords(32, 8)
            .clone();
        assert!(_uint_output.len() == 1, "Wrong wire length");
    }
}

impl GadgetConfig for Gadget<SHA256Gadget<ZkaySHA256Gadget>> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        &self.t.t._uint_output
    }
}
