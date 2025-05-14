

use circuit::operations::gadget;
use circuit::structure::wire;

pub struct DotProductGadget  {
	 a:Vec<Wire>,
	 b:Vec<Wire>,
	 output:Wire,
}
impl DotProductGadget{
	pub  fn new(a:Vec<Wire>, b:Vec<Wire>, desc:Vec<String>)  ->Self{
		super(desc);
			assert!(a.len() == b.len());
		self.a = a;
		self.b = b;
		buildCircuit();
	}
}
impl Gadget for DotProductGadget{
	  fn buildCircuit() {
		output = generator.getZeroWire();
		for i in 0..a.len() {
			let product = a[i].mul(b[i], format!("Multiply elements # {i}"));
			output = output.add(product);
		}
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return vec![output];
	}
}
