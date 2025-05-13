

use circuit::structure::wire;
use circuit::structure::wire_array;


pub struct ZkaySHA256Gadget
 {
      let bytes_per_word = 32;

     Vec<Wire> _uint_output;
 }

impl  ZkaySHA256Gadget{
      fn convert_inputs_to_bytes(uint256_inputs:Vec<Wire>)->Vec<Wire> {
        let input_bytes = WireArray::new(uint256_inputs).getBits(bytes_per_word * 8).packBitsIntoWords(8);
        // Reverse byte order of each input because jsnark reverses internally when packing
        for j in 0..uint256_inputs.length {
            Collections.reverse(Arrays.asList(input_bytes).subList(j * bytes_per_word, (j+1) * bytes_per_word));
        }
        return input_bytes;
    }

    pub  fn new(uint256_inputs:Vec<Wire>, truncated_bits:i32 , desc:Vec<String>)->self {
        super(convert_inputs_to_bytes(uint256_inputs), 8, uint256_inputs.length * bytes_per_word, false, true, desc);
        if truncated_bits > 253 || truncated_bits < 0 {
            panic!("Unsupported output length " + truncated_bits + " bits");
        }
        assembleOutput(truncated_bits);
    }
}
impl SHA256Gadget for ZkaySHA256Gadget{
     fn assembleOutput(truncated_length:i32 ) {
        let digest = super.getOutputWires();
        // Invert word order to get correct byte order when packed into one big word below
        Collections.reverse(Arrays.asList(digest));
        if truncated_length < 256 {
            // Keep truncated_length left-most bits as suggested in FIPS 180-4 to shorten the digest
            if truncated_length % 32 == 0 {
                let shortened_digest = vec![Wire::default();truncated_length / 32];
                System.arraycopy(digest, digest.length - shortened_digest.length, shortened_digest, 0, shortened_digest.length);
                digest = shortened_digest;
            } else {
                _uint_output = vec![Wire::default();]{WireArray::new(digest).getBits(32).shiftRight(256, 256 - truncated_length).packAsBits(truncated_length)};
                return;
            }
        }
        _uint_output = WireArray::new(digest).packWordsIntoLargerWords(32, 8);
        assert!(_uint_output.length == 1,"Wrong wire length");
    }

    
    pub  fn getOutputWires()->Vec<Wire>  {
        return _uint_output;
    }
}
