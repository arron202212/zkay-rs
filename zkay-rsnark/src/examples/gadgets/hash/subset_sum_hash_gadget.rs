use crate::circuit::config::config::Configs;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{Util,BigInteger};

pub struct SubsetSumHashGadget {
    inputWires: Vec<Option<WireType>>,
    outWires: Vec<Option<WireType>>,
    binaryOutput: bool,
}
use std::sync::OnceLock;
static COEFFS: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();
impl SubsetSumHashGadget {
    pub const DIMENSION: i32 = 3; // set to 4 for higher security
    pub const INPUT_LENGTH: i32 = 2 * DIMENSION * Config.log2_field_prime; // length in bits

    /**
     * @param ins
     *            The bitwires of the input.
     * @param binaryOutput
     *            Whether the output digest should be splitted into bits or not.
     * @param desc
     */
    pub fn new(ins: Vec<Option<WireType>>, binaryOutput: bool, desc: &String) -> Self {
        COEFFS::get_or_init(|| {
            let mut tmp = vec![vec![BigInteger::default(); INPUT_LENGTH]; DIMENSION];
            for i in 0..DIMENSION {
                for k in 0..INPUT_LENGTH {
                    tmp[i][k] = Util::nextRandomBigInteger(Configs.get().unwrap().field_prime);
                }
            }
            tmp
        });
        super(desc);
        let numBlocks = (ins.len() * 1.0 / INPUT_LENGTH).ceil() as i32;

        assert!(numBlocks <= 1, "Only one block is supported at this point");

        let rem = numBlocks * INPUT_LENGTH - ins.len();

        let mut pad = vec![None; rem];
        for i in 0..pad.len() {
            pad[i] = generator.getZeroWire(); // TODO: adjust padding
        }
        inputWires = Util::concat(ins, pad);
        self.binaryOutput = binaryOutput;
        buildCircuit();
    }
}
impl Gadget for SubsetSumHashGadget {
    fn buildCircuit() {
        let mut outDigest = vec![generator.getZeroWire(); DIMENSION];

        for i in 0..DIMENSION {
            for j in 0..INPUT_LENGTH {
                let t = inputWires[j].mul(COEFFS[i][j]);
                outDigest[i] = outDigest[i].add(t);
            }
        }
        if !binaryOutput {
            outWires = outDigest;
        } else {
            outWires = vec![None; DIMENSION * Config.log2_field_prime];
            for i in 0..DIMENSION {
                let bits = outDigest[i].getBitWires(Config.log2_field_prime).asArray();
                for j in 0..bits.len() {
                    outWires[j + i * Config.log2_field_prime] = bits[j];
                }
            }
        }
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return outWires;
    }
}
