

use circuit::operations::gadget;
use circuit::structure::wire;

pub struct DotProductGadget extends Gadget {

	 Vec<Wire> a;
	 Vec<Wire> b;
	 Wire output;

	pub  DotProductGadget(a:Vec<Wire>, b:Vec<Wire>, desc:Vec<String>) {
		super(desc);
		if a.length != b.length {
			assert!();
		}
		self.a = a;
		self.b = b;
		buildCircuit();
	}

	  fn buildCircuit() {
		output = generator.getZeroWire();
		for i in 0..a.length {
			Wire product = a[i].mul(b[i], "Multiply elements # " + i);
			output = output.add(product);
		}
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![Wire::default();] { output };
	}
}
