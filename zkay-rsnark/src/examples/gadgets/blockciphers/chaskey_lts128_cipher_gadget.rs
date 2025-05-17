use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

/**
 * Implements the light weight cipher Chaskey128, the LTS version with 16 rounds
 * https://eprint.iacr.org/2014/386.pdf.
 *
 * The gadget follows the reference implementation from this project:
 * https://www.nist.gov/sites/default/files/documents/2016/10/18/perrin-paper-lwc2016.pdf
 * https://www.cryptolux.org/index.php/FELICS
 */
pub struct ChaskeyLTS128CipherGadget {
    plaintext: Vec<WireType>,  // 4 32-bit words
    key: Vec<WireType>,        // 4 32-bit words
    ciphertext: Vec<WireType>, // 4 32-bit words
}
impl ChaskeyLTS128CipherGadget {
    pub fn new(inputs: Vec<WireType>, key: Vec<WireType>, desc: Vec<String>) -> Self {
        super(desc);
        assert!(inputs.len() == 4 && key.len() == 4, "Invalid Input");

        self.plaintext = inputs;
        self.key = key;

        buildCircuit();
    }
}
impl Gadget for ChaskeyLTS128CipherGadget {
    fn buildCircuit() {
        let v = vec![WireType::default(); 4];
        for i in 0..4 {
            v[i] = (plaintext[i].xorBitwise(key[i], 32));
        }

        for i in 0..16 {
            v[0] = v[0].add(v[1]);
            v[0] = v[0].trimBits(33, 32);
            v[1] = v[1].rotateLeft(32, 5).xorBitwise(v[0], 32);
            v[0] = v[0].rotateLeft(32, 16);

            v[2] = v[2].add(v[3]).trimBits(33, 32);
            v[3] = v[3].rotateLeft(32, 8).xorBitwise(v[2], 32);

            v[0] = v[0].add(v[3]).trimBits(33, 32);
            v[3] = v[3].rotateLeft(32, 13).xorBitwise(v[0], 32);

            v[2] = v[2].add(v[1]).trimBits(33, 32);
            v[1] = v[1].rotateLeft(32, 7).xorBitwise(v[2], 32);
            v[2] = v[2].rotateLeft(32, 16);
        }

        for i in 0..4 {
            v[i] = v[i].xorBitwise(key[i], 32);
        }
        ciphertext = v;
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return ciphertext;
    }
}
