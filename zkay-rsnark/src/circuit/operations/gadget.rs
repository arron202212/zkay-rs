

use circuit::structure::circuit_generator;
use circuit::structure::wire;

pub struct Gadget<T>{
    generator:CircuitGenerator;
	  description:String;
    t:T,
}
	pub fn  newGadget(desc:Vec<String>)->(CircuitGenerator,String) {
		( CircuitGenerator.getActiveCircuitGenerator(),desc.get(0).unwrap_or(&String::new()))
	}

pub trait GadgetConfig {
	fn getOutputWires()->Vec<Wire>;
	
	fn  toString(&self)->String {
		 "getClass().getSimpleName()".to_owned() + " " + self.description
	}
	
	fn  debugStr(&self,String s)->String {
		format!( "{self:?}:{s}")
	}
}
