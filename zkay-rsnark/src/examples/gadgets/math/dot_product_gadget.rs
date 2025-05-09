

use circuit::operations::gadget;
use circuit::structure::wire;

public class DotProductGadget extends Gadget {

	private Wire[] a;
	private Wire[] b;
	private Wire output;

	public DotProductGadget(Wire[] a, Wire[] b, String... desc) {
		super(desc);
		if a.length != b.length {
			throw new IllegalArgumentException();
		}
		this.a = a;
		this.b = b;
		buildCircuit();
	}

	private void buildCircuit() {
		output = generator.getZeroWire();
		for i in 0..a.length {
			Wire product = a[i].mul(b[i], "Multiply elements # " + i);
			output = output.add(product);
		}
	}

	
	public Wire[] getOutputWires() {
		return new Wire[] { output };
	}
}
