
use circuit::operations::gadget;
use circuit::structure::wire;

/**
 * Implements the light weight cipher Chaskey128, the LTS version with 16 rounds
 * https://eprint.iacr.org/2014/386.pdf.
 * 
 * The gadget follows the reference implementation from this project:
 * https://www.nist.gov/sites/default/files/documents/2016/10/18/perrin-paper-lwc2016.pdf
 * https://www.cryptolux.org/index.php/FELICS
 */
pub struct ChaskeyLTS128CipherGadget extends Gadget {

	 Vec<Wire> plaintext; // 4 32-bit words
	 Vec<Wire> key; // 4 32-bit words
	 Vec<Wire> ciphertext; // 4 32-bit words

	pub  ChaskeyLTS128CipherGadget(inputs:Vec<Wire>, key:Vec<Wire>, desc:Vec<String>) {
		super(desc);
		if inputs.length != 4 || key.length != 4 {
			assert!("Invalid Input");
		}
		self.plaintext = inputs;
		self.key = key;

		buildCircuit();

	}

	  fn buildCircuit() {

		Vec<Wire> v = vec![Wire::default();4];
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
			;
			v[1] = v[1].rotateLeft(32, 7).xorBitwise(v[2], 32);
			v[2] = v[2].rotateLeft(32, 16);

		}

		for i in 0..4 {
			v[i] = v[i].xorBitwise(key[i], 32);
		}
		ciphertext = v;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return ciphertext;
	}

}
