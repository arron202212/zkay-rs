
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;

pub struct AESSBoxNaiveLookupGadget extends Gadget {

	  i32 Vec<SBox> = AES128CipherGadget.SBox;

	 Wire input;
	 Wire output;

	pub  AESSBoxNaiveLookupGadget(Wire input, desc:Vec<String>) {
		super(desc);
		self.input = input;
		buildCircuit();
	}

	  fn buildCircuit() {
		output = generator.getZeroWire();
		for i in 0..256 {
			output = output.add(input.isEqualTo(i).mul(SBox[i]));
		}
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { output };
	}
}
