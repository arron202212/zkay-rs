use circuit::config::config;
use circuit::operations::gadget;
use circuit::structure::wire;
use util::util;

pub struct SubsetSumHashGadget {
    inputWires: Vec<Wire>,
    outWires: Vec<Wire>,
    binaryOutput: bool,
}
use std::sync::OnceLock;
static COEFFS: OnceLock<Vec<Vec<BigInteger>>> = OnceLock::new();
impl SubsetSumHashGadget {
    pub const DIMENSION: i32 = 3; // set to 4 for higher security
    pub const INPUT_LENGTH: i32 = 2 * DIMENSION * Config.LOG2_FIELD_PRIME; // length in bits

    /**
     * @param ins
     *            The bitwires of the input.
     * @param binaryOutput
     *            Whether the output digest should be splitted into bits or not.
     * @param desc
     */
    pub fn new(ins: Vec<Wire>, binaryOutput: bool, desc: Vec<String>) -> Self {
        COEFFS::get_or_init(|| {
            let mut tmp = vec![vec![BigInteger::default(); INPUT_LENGTH]; DIMENSION];
            for i in 0..DIMENSION {
                for k in 0..INPUT_LENGTH {
                    tmp[i][k] = Util::nextRandomBigInteger(Config.FIELD_PRIME);
                }
            }
            tmp
        });
        super(desc);
        let numBlocks = (ins.length * 1.0 / INPUT_LENGTH).ceil() as i32;

        assert!(numBlocks <= 1, "Only one block is supported at this point");

        let rem = numBlocks * INPUT_LENGTH - ins.length;

        let mut pad = vec![Wire::default(); rem];
        for i in 0..pad.length {
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
            outWires = vec![Wire::default(); DIMENSION * Config.LOG2_FIELD_PRIME];
            for i in 0..DIMENSION {
                let bits = outDigest[i].getBitWires(Config.LOG2_FIELD_PRIME).asArray();
                for j in 0..bits.length {
                    outWires[j + i * Config.LOG2_FIELD_PRIME] = bits[j];
                }
            }
        }
    }

    pub fn getOutputWires() -> Vec<Wire> {
        return outWires;
    }
}
