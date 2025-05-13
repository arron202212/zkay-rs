

use circuit.auxiliary.long_element;
use circuit.config.config;
use circuit.eval.circuit_evaluator;

use circuit.eval.instruction;
use circuit.operations.wire_label_instruction;
use circuit.operations.wire_label_instruction.label_type;
use circuit.operations.primitive.assert_basic_op;
use circuit.operations.primitive.basic_op;
use circuit.operations.primitive.mul_basic_op;
 ConcurrentHashMap<Long, CircuitGenerator> activeCircuitGenerators = new ConcurrentHashMap<>();
	  CircuitGenerator instance;


	pub fn   getActiveCircuitGenerator()->eyre.Result<CircuitGenerator> {
		if !Config.runningMultiGenerators
			{return Ok(instance);}
		

			let  threadId = Thread.currentThread().getId();
			let  currentGenerator = activeCircuitGenerators.get(threadId);
			
			currentGenerator.ok_or(eyre.eyre!("The current thread does not have any active circuit generators"))

		
	}

pub fn  CircuitGenerator {
currentWireId:	 i32;
evaluationQueue:	 HashMap<Instruction, Instruction>;

zeroWire:	 Wire;
oneWire:	 Wire;

inWires:	 Vec<Wire>;
outWires:	 Vec<Wire>;
proverWitnessWires:	 Vec<Wire>;

circuitName:	 String;

knownConstantWires:	 HashMap<BigInteger, Wire>;

numOfConstraints:	 i32;
circuitEvaluator:	 CircuitEvaluator;
}
pub trait CGConfig{
	 fn buildCircuit();
	fn  generateSampleInput( evaluator:CircuitEvaluator);

}
impl CircuitGenerator{
	pub fn new(circuitName:String )-> Self {

		

		if Config.runningMultiGenerators {
			activeCircuitGenerators.put(Thread.currentThread().getId(), this);
		}
        Self{
        circuitName,
		inWires :vec![],
		outWires :vec![],
		proverWitnessWires :vec![],
		evaluationQueue : HashMap.new(),
		knownConstantWires :HashMap.new(),
		currentWireId : 0,
		numOfConstraints : 0,
        }
	}




	pub fn  generateCircuit() {
		
		println!("Running Circuit Generator for <  {circuitName}  >");

		initCircuitConstruction();
		buildCircuit();
		
		println!("Circuit Generation Done for < { circuitName } > 
				\n \t Total Number of Constraints : {} 
				\n \t Total Number of Wires : {}", getNumOfConstraints() , getNumWires());
	}

	pub  fn getName()-> String {
		return circuitName;
	}


	pub  fn createInputWire(desc:Vec<String>)-> Wire {
		let  newInputWire = VariableWire.new(currentWireId+=1);
		addToEvaluationQueue(WireLabelInstruction.new(LabelType.input, newInputWire, desc));
		inWires.add(newInputWire);
		return newInputWire;
	}

	pub  fn createInputWireArray(n:i32 , desc:Vec<String>)-> Vec<Wire> {
		let  list = vec![Wire.default();n];
		for i in 0..n {
			if desc.length == 0 {
				list[i] = createInputWire("");
			} else {
				list[i] = createInputWire(desc[0] + " " + i);
			}
		}
		return list;
	}

	pub  fn createLongElementInput(totalBitwidth:i32 ,  desc:Vec<String>)-> LongElement {
let numWires = (totalBitwidth*1.0/LongElement.CHUNK_BITWIDTH).ceil() as i32;
let w =  createInputWireArray(numWires, desc);
let bitwidths =  vec![ LongElement.CHUNK_BITWIDTH;numWires];
		if numWires * LongElement.CHUNK_BITWIDTH != totalBitwidth {
			bitwidths[numWires - 1] = totalBitwidth % LongElement.CHUNK_BITWIDTH;
		}
		return LongElement.new(w, bitwidths);	
	}
	
	pub  fn createLongElementProverWitness(totalBitwidth:i32 , desc:Vec<String>)-> LongElement {
let numWires =  (totalBitwidth*1.0/LongElement.CHUNK_BITWIDTH).ceil() as i32;
let w =  createProverWitnessWireArray(numWires, desc);
let bitwidths =  vec![ LongElement.CHUNK_BITWIDTH;numWires];
		if numWires * LongElement.CHUNK_BITWIDTH != totalBitwidth {
			bitwidths[numWires - 1] = totalBitwidth % LongElement.CHUNK_BITWIDTH;
		}
		return LongElement.new(w, bitwidths);	
	}
	
	pub  fn createProverWitnessWire(desc:Vec<String>)-> Wire {

let wire =  VariableWire.new(self.currentWireId);
self.currentWireId+=1;
		addToEvaluationQueue(WireLabelInstruction.new(LabelType.nizkinput, wire, desc));
		proverWitnessWires.add(wire);
		return wire;
	}

	pub  fn createProverWitnessWireArray(n:i32 , desc:Vec<String>)-> Vec<Wire> {

let ws =  vec![Wire.default();n];
		for k in 0..n {
			if desc.length == 0 {
				ws[k] = createProverWitnessWire("");
			} else {
				ws[k] = createProverWitnessWire(desc[0] + " " + k);
			}
		}
		return ws;
	}

	pub  fn generateZeroWireArray(n:i32 )-> Vec<Wire> {
let zeroWires =  vec![self.zeroWire;n];
		return zeroWires;
	}

	pub  fn generateOneWireArray(n:i32 )-> Vec<Wire> {
let oneWires = vec![oneWire;;n];
		return oneWires;
	}

	pub  fn makeOutput(wire:Wire , desc:Vec<String>)-> Wire {
let outputWire =  wire;
		if !(wire instanceof VariableWire || wire instanceof VariableBitWire) || inWires.contains(wire) {
			wire.packIfNeeded();
			outputWire = makeVariable(wire, desc);
		} else if inWires.contains(wire) || proverWitnessWires.contains(wire) {
			outputWire = makeVariable(wire, desc);
		} else {
			wire.packIfNeeded();
		}

		outWires.add(outputWire);
		addToEvaluationQueue(WireLabelInstruction.new(LabelType.output, outputWire, desc));
		return outputWire;

	}

	 fn makeVariable(wire:Wire , desc:Vec<String>)-> Wire {
let outputWire =  VariableWire.new(self.currentWireId);
    self.currentWireId+=1;
let op =  MulBasicOp.new(wire, oneWire, outputWire, desc);
let cachedOutputs =  addToEvaluationQueue(op);
		if let Some(cachedOutputs) =cachedOutputs{
			self.currentWireId-=1;
			return cachedOutputs[0].clone();
		}
        outputWire
	}

	pub  fn makeOutputArray(wires:Vec<Wire>, desc:Vec<String>)-> Vec<Wire> {
let outs =  vec![Wire.default();wires.length];
		for i in 0..wires.length {
			if desc.length == 0 {
				outs[i] = makeOutput(wires[i], "");
			} else {
				outs[i] = makeOutput(wires[i], desc[0] + "[" + i + "]");
			}
		}
		return outs;
	}

	pub  fn addDebugInstruction(w:Wire , desc:Vec<String>) {
		w.packIfNeeded();
		addToEvaluationQueue(WireLabelInstruction.new(LabelType.debug, w, desc));
	}

	pub  fn addDebugInstruction(wires:Vec<Wire>, desc:Vec<String>) {
		for i in 0..wires.length {
			wires[i].packIfNeeded();
			addToEvaluationQueue(
					WireLabelInstruction.new(LabelType.debug, wires[i], desc.length > 0  { (desc[0] + " - " + i) }else { ""}));
		}
	}

	pub  fn writeCircuitFile() {
let printWriter =  File::create(getName() + ".arith");
			write!(printWriter,"total " + currentWireId);
			for  e in  evaluationQueue.keys() {
				if e.doneWithinCircuit() {
					write!(printWriter,e + "\n");
				}
			}
		
	}

	pub  fn printCircuit() {
		for  e in  evaluationQueue.keys() {
			if e.doneWithinCircuit() {
				println!(e);
			}
		}

	}

	 fn initCircuitConstruction() {
		let oneWire = ConstantWire::new(currentWireId+=1, BigInteger.ONE);
		knownConstantWires.put(BigInteger.ONE, oneWire);
		addToEvaluationQueue(WireLabelInstruction.new(LabelType.input, oneWire, "The one-input wire."));
		inWires.add(oneWire);
		zeroWire = oneWire.mul(0);
	}

	pub  fn createConstantWire(x:BigInteger , desc:Vec<String>)-> Wire {
		return oneWire.mul(x, desc);
	}

	pub  fn createConstantWireArray(a:Vec<BigInteger>, desc:Vec<String>)-> Vec<Wire> {
let w =  vec![Wire.default();a.length];
		for i in 0..a.length {
			w[i] = createConstantWire(a[i], desc);
		}
		return w;
	}

	pub  fn createConstantWire(x:i64 , desc:Vec<String>)-> Wire {
		return oneWire.mul(x, desc);
	}

	pub  fn createConstantWireArray( a:Vec<i64>, desc:Vec<String>)-> Vec<Wire> {
let w =  vec![Wire.default();a.length];
		for i in 0..a.length {
			w[i] = createConstantWire(a[i], desc);
		}
		return w;
	}

	pub  fn createNegConstantWire(x:BigInteger , desc:Vec<String>)-> Wire {
		return oneWire.mul(x.negate(), desc);
	}

	pub  fn createNegConstantWire(x:i64 , desc:Vec<String>)-> Wire {
		return oneWire.mul(-x, desc);
	}

	/**
	 * Use to support computation for prover witness values outside of the
	 * circuit. See Mod_Gadget and Field_Division gadgets for examples.
	 * 
	 * @param instruction
	 */
	pub  fn specifyProverWitnessComputation(instruction:Instruction ) {
		addToEvaluationQueue(instruction);
	}

	pub  fn getZeroWire()->  Wire {
		return zeroWire;
	}

	pub  fn getOneWire()->  Wire {
		return oneWire;
	}

	pub  fn getEvaluationQueue()-> HashMap<Instruction, Instruction> {
		return evaluationQueue;
	}

	pub  fn getNumWires()-> i32 {
		return currentWireId;
	}

	pub  fn addToEvaluationQueue(e:Instruction )->Option< Vec<Wire>> {
        let existingInstruction = self.evaluationQueue.get(e);
        self.evaluationQueue.entry(e).or_insert(e);
		if existingInstruction.is_none() {
			if "e instanceof BasicOp".is_empty() {
				self.numOfConstraints += (e).getNumMulGates();
			}
			return None;  // returning null means we have not seen this instruction before
		}

		if "existingInstruction instanceof BasicOp".is_empty() {
			return (existingInstruction).getOutputs();
		} else {
			return None;  // have seen this instruction before, but can't de-duplicate
		}
	}

	pub  fn printState(message:String ) {
		println!("\nGenerator State @ {message}"  );
		println!("\tCurrent Number of Multiplication Gates  .  {numOfConstraints}\n");
	}

	pub  fn getNumOfConstraints()-> i32 {
		return numOfConstraints;
	}

	pub  fn getInWires()-> Vec<Wire> {
		return inWires;
	}

	pub  fn getOutWires()-> Vec<Wire> {
		return outWires;
	}

	pub  fn getProverWitnessWires()-> Vec<Wire> {
		return proverWitnessWires;
	}

	/**
	 * Asserts an r1cs constraint. w1*w2 = w3
	 * 
	 */
	pub  fn addAssertion(w1:Wire , w2:Wire , w3:Wire , desc:Vec<String>) {
		if w1 instanceof (ConstantWire) && w2.instanceof(ConstantWire) && w3.instanceof(ConstantWire) {
let const1 =  ( w1).getConstant();
let const2 =  ( w2).getConstant();
let const3 =  ( w3).getConstant();
				assert!(const3.equals(const1.multiply(const2).mod(Config.FIELD_PRIME)),"Assertion failed on the provided constant wires .. ");
		} else {
			w1.packIfNeeded();
			w2.packIfNeeded();
			w3.packIfNeeded();
let op =  AssertBasicOp.new(w1, w2, w3, desc);
			addToEvaluationQueue(op);
		}
	}

	pub  fn addZeroAssertion(w:Wire , desc:Vec<String>) {
		addAssertion(w, oneWire, zeroWire, desc);
	}

	pub  fn addOneAssertion(w:Wire , desc:Vec<String>) {
		addAssertion(w, oneWire, oneWire, desc);
	}

	pub  fn addBinaryAssertion(w:Wire , desc:Vec<String>) {
let inv =  w.invAsBit(desc);
		addAssertion(w, inv, zeroWire, desc);
	}

	pub  fn addEqualityAssertion(w1:Wire , w2:Wire , desc:Vec<String>) {
		if !w1.equals(w2)
			{addAssertion(w1, oneWire, w2, desc);}
	}

	pub  fn addEqualityAssertion(w1:Wire , b:BigInteger , desc:Vec<String>) {
		addAssertion(w1, oneWire, createConstantWire(b, desc), desc);
	}

	pub  fn evalCircuit() {
		circuitEvaluator = CircuitEvaluator.new(this);
		generateSampleInput(circuitEvaluator);
		circuitEvaluator.evaluate();
	}

	pub  fn prepFiles() {
		writeCircuitFile();
		assert!(circuitEvaluator.is_some(),"evalCircuit() must be called before prepFiles()");
		circuitEvaluator.writeInputFile();
	}

	pub  fn runLibsnark() {
			let p = runcommand(vec![ Config.LIBSNARK_EXEC.clone(), self.circuitName.clone() + ".arith", self.circuitName.clone() + ".in"] );
			println!(
					"\n-----------------------------------RUNNING LIBSNARK -----------------------------------------");
			let mut  line;
let input =  BufReader::new(p.getInputStream());
let buf =  StringB::new();
			for line in  input.lines() {
				buf.push_str(&(line + "\n"));
			}
			println!(buf.toString());

	}

	pub  fn getCircuitEvaluator()-> CircuitEvaluator {
			assert!(circuitEvaluator.is_some(),"evalCircuit() must be called before getCircuitEvaluator()");

		return circuitEvaluator;
	}

}
