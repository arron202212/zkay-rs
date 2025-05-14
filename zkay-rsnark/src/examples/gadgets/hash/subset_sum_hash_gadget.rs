

use util::util;
use circuit::config::config;
use circuit::operations::gadget;
use circuit::structure::wire;

pub struct SubsetSumHashGadget  {




	 inputWires:Vec<Wire>,
	 outWires:Vec<Wire>,
	 binaryOutput:bool,

	
}
impl  SubsetSumHashGadget{
	pub   let DIMENSION = 3; // set to 4 for higher security
	pub   let INPUT_LENGTH = 2 * DIMENSION * Config.LOG2_FIELD_PRIME; // length in bits
	  Vec<Vec<BigInteger>> COEFFS;
 {
		COEFFS = vec![BigInteger::default();DIMENSION][INPUT_LENGTH];
		for i in 0..DIMENSION {
			for k in 0..INPUT_LENGTH {
				COEFFS[i][k] = Util::nextRandomBigInteger(Config.FIELD_PRIME);
			}

		}
	}

	/**
	 * @param ins
	 *            The bitwires of the input.
	 * @param binaryOutput
	 *            Whether the output digest should be splitted into bits or not.
	 * @param desc
	 */
	pub  fn new(ins:Vec<Wire>, bool binaryOutput, desc:Vec<String>)  ->Self{

		super(desc);
		let numBlocks = (i32) Math.ceil(ins.length * 1.0 / INPUT_LENGTH);

		if numBlocks > 1 {
			assert!("Only one block is supported at this point");
		}

		let rem = numBlocks * INPUT_LENGTH - ins.length;

		let pad = vec![Wire::default();rem];
		for i in 0..pad.length {
			pad[i] = generator.getZeroWire(); // TODO: adjust padding
		}
		inputWires = Util::concat(ins, pad);
		self.binaryOutput = binaryOutput;
		buildCircuit();
	}
}
impl Gadget for SubsetSumHashGadget{
	  fn buildCircuit() {

		let outDigest = vec![Wire::default();DIMENSION];
		Arrays.fill(outDigest, generator.getZeroWire());

		for i in 0..DIMENSION {
			for j in 0..INPUT_LENGTH {
				let t = inputWires[j].mul(COEFFS[i][j]);
				outDigest[i] = outDigest[i].add(t);
			}
		}
		if !binaryOutput {
			outWires = outDigest;
		} else {
			outWires = vec![Wire::default();DIMENSION * Config.LOG2_FIELD_PRIME];
			for i in 0..DIMENSION {
				let bits = outDigest[i].getBitWires(Config.LOG2_FIELD_PRIME).asArray();
				for j in 0..bits.length {
					outWires[j + i * Config.LOG2_FIELD_PRIME] = bits[j];
				}
			}
		}
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return outWires;
	}
}
