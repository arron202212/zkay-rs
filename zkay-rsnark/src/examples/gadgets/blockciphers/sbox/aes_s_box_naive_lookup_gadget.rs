
use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;

public class AESSBoxNaiveLookupGadget extends Gadget {

	private static int SBox[] = AES128CipherGadget.SBox;

	private Wire input;
	private Wire output;

	public AESSBoxNaiveLookupGadget(Wire input, String... desc) {
		super(desc);
		this.input = input;
		buildCircuit();
	}

	protected void buildCircuit() {
		output = generator.getZeroWire();
		for i in 0..256 {
			output = output.add(input.isEqualTo(i).mul(SBox[i]));
		}
	}

	
	public Wire[] getOutputWires() {
		return new Wire[] { output };
	}
}
