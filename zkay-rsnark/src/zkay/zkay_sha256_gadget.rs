use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;

pub struct ZkaySHA256Gadget {
    _uint_output: Vec<Option<WireType>>,
}

impl ZkaySHA256Gadget {
    const bytes_per_word: i32 = 32;
    fn convert_inputs_to_bytes(uint256_inputs: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
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

    pub fn new(uint256_inputs: Vec<Option<WireType>>, truncated_bits: i32, desc: &Option<String>) -> self {
        super(
            convert_inputs_to_bytes(uint256_inputs),
            8,
            uint256_inputs.len() * bytes_per_word,
            false,
            true,
            desc,
        );
        if truncated_bits > 253 || truncated_bits < 0 {
            panic!("Unsupported output length " + truncated_bits + " bits");
        }
        assembleOutput(truncated_bits);
    }
}
impl SHA256Gadget for ZkaySHA256Gadget {
    fn assembleOutput(truncated_length: i32) {
        let digest = super.getOutputWires();
        // Invert word order to get correct byte order when packed into one big word below
        Collections.reverse(Arrays.asList(digest));
        if truncated_length < 256 {
            // Keep truncated_length left-most bits as suggested in FIPS 180-4 to shorten the digest
            if truncated_length % 32 == 0 {
                let shortened_digest = vec![None; truncated_length / 32];
                System.arraycopy(
                    digest,
                    digest.len() - shortened_digest.len(),
                    shortened_digest,
                    0,
                    shortened_digest.len(),
                );
                digest = shortened_digest;
            } else {
                _uint_output = vec![
                    WireArray::new(digest)
                        .getBits(32)
                        .shiftRight(256, 256 - truncated_length)
                        .packAsBits(None,None,truncated_length),
                ];
                return;
            }
        }
        _uint_output = WireArray::new(digest).packWordsIntoLargerWords(32, 8);
        assert!(_uint_output.len() == 1, "Wrong wire length");
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        _uint_output
    }
}
