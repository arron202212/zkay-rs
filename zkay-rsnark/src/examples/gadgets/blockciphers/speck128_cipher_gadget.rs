
use circuit::operations::gadget;
use circuit::structure::circuit_generator;
use circuit::structure::wire;

/**
 * Implements the Speck lightweight block cipher
 * https://eprint.iacr.org/2015/585.pdf
 *
 */

pub struct Speck128CipherGadget  {

	 plaintext:Vec<Wire>,
	 expandedKey:Vec<Wire>,
	 ciphertext:Vec<Wire>,

	}
impl  Speck128CipherGadget{
	/**
	 * 
	 * @param inputs
	 *            : Array of 2 64-bit elements.
	 * @param expandedKey
	 *            : Array of 32 64-bit elements. (Call expandKey(..))
	 * @param desc
	 */
	pub  fn new(plaintext:Vec<Wire>, expandedKey:Vec<Wire>,
			desc:Vec<String>) {
		super(desc);
		if plaintext.length != 2 || expandedKey.length != 32 {
			assert!("Invalid Input");
		}
		self.plaintext = plaintext;
		self.expandedKey = expandedKey;
		buildCircuit();
	}
}
impl Gadget for Speck128CipherGadget{
	  fn buildCircuit() {

		Wire x, y;
		x = plaintext[1];
		y = plaintext[0];
		ciphertext = vec![Wire::default();2];
		for i in 0..=31{
			x = x.rotateRight(64, 8).add(y);
			x = x.trimBits(65, 64);
			x = x.xorBitwise(expandedKey[i], 64);
			y = y.rotateLeft(64, 3).xorBitwise(x, 64);
		}
		ciphertext[1] = x;
		ciphertext[0] = y;
	}

	/**
	 * 
	 * @param key
	 *            : 2 64-bit words
	 * @return
	 */
	pub fn expandKey(key:Vec<Wire>)->  Vec<Wire> {
		let generator = CircuitGenerator
				.getActiveCircuitGenerator();
		let k = vec![Wire::default();32];
		let l = vec![Wire::default();32];
		k[0] = key[0];
		l[0] = key[1];
		for i in 0..=32 - 2{
			l[i + 1] = k[i].add(l[i].rotateLeft(64, 56));
			l[i + 1] = l[i + 1].trimBits(65, 64);
			l[i + 1] = l[i + 1].xorBitwise(generator.createConstantWire(i), 64);
			k[i + 1] = k[i].rotateLeft(64, 3).xorBitwise(l[i + 1], 64);
		}
		return k;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return ciphertext;
	}
}
