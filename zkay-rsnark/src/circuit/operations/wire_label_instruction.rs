

use circuit::config::config;
use circuit::eval::circuit_evaluator;

use circuit::eval::instruction;
use circuit::structure::wire;
	pub enum LabelType {
		input, output, nizkinput, debug
	}
pub struct WireLabelInstruction{
	  label_type:LabelType;
	  w:Wire;
	  desc:String;
}
 impl  WireLabelInstruction{

	pub  new( label_type:LabelType,  w:Wire, desc:Vec<String>)->Self {
        Self{label_type,w,desc:desc.get(0).unwrap_or(&String::new()).clone()}
	}

	pub  fn getWire()->Wire {
		 w.clone()
	}

	pub  fn toString(&self)->String {
		format!("{} {}{}",self.label_type,self.w, (if desc.length() == 0  { "" }else { "\t\t\t # " + desc}) )
	}
	pub fn  getType(&self)->LabelType {
		self.label_type.clone()
	}
 }

 impl Instruction  for WireLabelInstruction{
	pub  fn evaluate(CircuitEvaluator evaluator) {
		// nothing to do.
	}
	
	pub  fn emit(CircuitEvaluator evaluator) {
		if type == LabelType.output && Config.outputVerbose || type == LabelType.debug && Config.debugVerbose {
			println!("\t[" + type + "] Value of Wire # " + w + (desc.length() > 0  { " (" + desc + ")" }else { ""}) + " :: "
					+ evaluator.getWireValue(w).toString(Config.hexOutputEnabled  { 16 }else { 10}));
		}
	}



	pub  fn doneWithinCircuit(&self)->bool {
		self.labe_type != LabelType.debug
	}

}
