
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::Path;
use std::collections::HashSet;
use util::util;
use circuit::auxiliary::long_element;
use circuit::config::config;
use circuit::operations::wire_label_instruction;
use circuit::operations::wire_label_instruction::label_type;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;

struct  CircuitEvaluator {
	  circuitGenerator:CircuitGenerator;
	 valueAssignment:BigInteger;
}

impl CircuitEvaluator{

	pub new( circuitGenerator:CircuitGenerator)->Self {
		let mut valueAssignment = vec![BigInteger::ZERO;circuitGenerator.getNumWires()];
		valueAssignment[circuitGenerator.getOneWire().getWireId()] = BigInteger.ONE;
        Self{circuitGenerator,valueAssignment}
	}

	pub fn setWireValue(w:Wire , v:BigInteger ) {
		assert!(v.signum() >= 0 && v<Config.FIELD_PRIME,"Only positive values that are less than the modulus are allowed for this method.");
		self.valueAssignment[w.getWireId()] = v;
	}

	pub fn  getWireValue(w:Wire )->BigInteger {
		let  mut v = self.valueAssignment[w.getWireId()];
		if v.is_none() {
			let  bits = w.getBitWiresIfExistAlready();
			if let Some(bits)=bits {
				let mut  sum = BigInteger.ZERO;
				for i in 0..bits.len() {
					sum = sum.push(self.valueAssignment[bits.get(i).getWireId()]
							.shiftLeft(i));
				}
				v = sum;
			}
		}
		 v
	}

	pub fn  getWiresValues(&self, w:Vec<Wire>)->Vec<BigInteger> {
		let mut  values = vec![BigInteger::ZERO;w.len()];
		for i in 0..w.len() {
			values[i] = self.getWireValue(w[i]);
		}
		values
	}

	pub fn  getWireValue(e:LongElement , bitwidthPerChunk:i32 )->BigInteger {
		 Util::combine(valueAssignment, e.getArray(), bitwidthPerChunk)
	}

	pub fn setWireValue(e:LongElement , value:BigInteger ,
			bitwidthPerChunk:i32 ) {
		Wire[] array = e.getArray();
		setWireValue(array, Util::split(value, bitwidthPerChunk));
	}

	pub fn setWireValue(wire:Wire , v:i64 ) {
			assert!(v>=0,"Only positive values that are less than the modulus are allowed for this method.");
		setWireValue(wire, BigInteger::from(v));
	}

	pub fn setWireValue( wires:Vec<Wire>, v:Vec<BigInteger>) {
		for i in 0..v.len() {
			setWireValue(wires[i], v[i]);
		}
		for  i in  v.len()..wires.len() {
			setWireValue(wires[i], BigInteger.ZERO);
		}
	}

	pub fn evaluate() {

		println!("Running Circuit Evaluator for < {} >",circuitGenerator.getName());
		let  evalSequence = circuitGenerator
				.getEvaluationQueue();

		for  e in  evalSequence.keys() {
			e.evaluate(self);
			e.emit(self);
		}
		// check that each wire has been assigned a value
		for i in 0..valueAssignment.len() {
			if valueAssignment[i].is_none() {
				panic!("Wire# {i}is without value");
			}
		}
		println!("Circuit Evaluation Done for < {} >\n\n",circuitGenerator.getName() );

	}

	pub fn writeInputFile() {
			let  evalSequence = circuitGenerator
					.getEvaluationQueue();
			let  printWriter = File::create(
					circuitGenerator.getName() + ".in").unwrap();
			for  e  in  evalSequence.keys() {
				if let  WireLabelInstruction(_e)=e
						&& (_e.getType() == LabelType.input || _e
								.getType() == LabelType.nizkinput) {
					let  id = _e.getWire().getWireId();
					write!(printWriter,id.to_string() + " "
							+ &format!(":x",valueAssignment[id]));
				}
			}

	}

	/**
	 * An independent old method for testing.
	 * 
	 * @param circuitFilePath
	 * @param inFilePath
	 * @
	 */

	pub fn  eval(circuitFilePath:String , inFilePath:String )
			 {

		let  circuitScanner =  BufReader::new(
				File::open(circuitFilePath).unwrap()).lines();
		let inFileScanner = BufReader::new(
				File::open(inFilePath).unwrap());

		let  totalWires = circuitScanner.next().unwrap().replace(
				"total ", "").parse::<i32>().unwrap();

		let mut  assignment = vec![BigInteger::ZERO;totalWires];

		let mut  wiresToReport = vec![];
		let mut  ignoreWires =  HashSet::new();

		// Hashtable<Integer, BigInteger> assignment = new Hashtable<>();
		while let Some(wireNumber)=inFileScanner.next() {
			let num = inFileScanner.next().unwrap();
			assignment[wireNumber] =  BigInteger.parse_bytes(num.as_bytes(), 16).unwrap();
			wiresToReport.push(wireNumber);
			// assignment.put(wireNumber, new BigInteger(num));
		}

		let  prime = BigInteger::parse_bytes(
				"21888242871839275222246405745257275088548364400416034343698204186575808495617",10).unwrap();

		circuitScanner.next();
		while let Some(mut line)=circuitScanner.next() {
			if line.contains("#") {
				line = line[..line.find("#").unwrap()].to_string();
				line = line.trim().to_string();
			}
			if line.starts_with("input") || line.starts_with("nizkinput") {
				continue;
			} 
             if line.starts_with("output ") {
				let line = line.replace("output ", "").parse<i32>().unwrap();
				println!("{}::{:x}",line, assignment[line]);
				wiresToReport.push(line);
			} else if line.starts_with("DEBUG ") {
				line = line.replace("DEBUG ", "");
				let mut  scanner = line.split_whitespace();
				let  id = scanner.next().unwrap().parse::<i32>().unwrap();
				println!( "{id}::{:x} >> {}", assignment[id],scanner.next().uwrap().split("\n").next().unwrap());
			} else {
				let  ins = self.getInputs(line);
				for i in  ins {
					if assignment[i].is_none() {
						println!("Undefined value for a wire:used , at line {line}");
					}
				}
				let  outs = getOutputs(line);
				if line.starts_with("mul ") {
					let  out = BigInteger.ONE;
					for  w  in  ins {
						out = out.multiply(assignment[w]);
					}
					wiresToReport.push(outs.get(0));
					assignment[outs.get(0)] = out.mod(prime);
					
				} else if line.starts_with("add ") {
					let mut  out = BigInteger.ZERO;
					for  w in  ins {
						out = out.add(assignment[w]);
					}
					assignment[outs.get(0)] = out.mod(prime);
				} else if line.starts_with("xor ") {
					let  out = if assignment[ins.get(0)]
							.equals(assignment[ins.get(1)]) { BigInteger.ZERO}
							else {BigInteger.ONE};
					assignment[outs.get(0)] = out;
					wiresToReport.push(outs.get(0));

				} else if line.starts_with("zerop ") {
					ignoreWires.push(outs.get(0));
					if assignment[ins.get(0)].signum() == 0 {
						assignment[outs.get(1)] = BigInteger.ZERO;
					} else {

						assignment[outs.get(1)] = BigInteger.ONE;
					}
					wiresToReport.push(outs.get(1));

				} else if line.starts_with("split ") {
					if outs.len() < assignment[ins.get(0)].bitLength() {

						println!("Error in Split");
						println!("{:x}",assignment[ins.get(0)]);
						println!(line);
					}
					for i in 0..outs.len() {
						assignment[outs.get(i)] =if  assignment[ins.get(0)]
								.testBit(i)  { BigInteger.ONE }else { BigInteger.ZERO};
						wiresToReport.push(outs.get(i));
					}

				} else if line.starts_with("pack ") {
					let  sum = BigInteger.ZERO;
					for i in 0..ins.len() {
						sum = sum.push(assignment[ins.get(i)]
								.multiply(new BigInteger("2").pow(i)));
					}
					wiresToReport.push(outs.get(0));
					assignment[outs.get(0)] = sum;
				} else if line.starts_with("const-mul-neg-") {
					let constantStr = line[
							"const-mul-neg-".len(), line.find(" ").unwrap()];
					let constant = prime.subtract(BigInteger::parse_bytes(
							constantStr.as_bytes(), 16));
					assignment[outs.get(0)] = assignment[ins.get(0)].multiply(
							constant).mod(prime);
				} else if line.starts_with("const-mul-") {
					let constantStr = line["const-mul-".len(),
							line.find(" ").unwrap()];
					let constant = BigInteger::parse_bytes(constantStr.as_bytes(), 16).unwrap();
					assignment[outs.get(0)] = assignment[ins.get(0)].multiply(
							constant).mod(prime);
				} else {
					println!("Unknown Circuit Statement");
				}

			}
		}

		for i in 0..totalWires {
			if assignment[i].is_none() && !ignoreWires.contains(i) {
				println!("Wire  {i } is Null");
			}
		}


		let  printWriter = File::Create(inFilePath + ".full.2");
		for  id in  wiresToReport {
			write!(printWriter,"{id} {:x}",assignment[id]);
		}
		
	}

	fn   getOutputs(line:String )->Vec<i32> {
		// println!(line);
		let  scanner = line[line.rfind("<").unwrap() + 1..
				line.rfind(">").unwrap()];
		let mut  outs = vec![];
		while let Some(v)=scanner.split_whitespace() {
    		// println!(v);
			outs.push(v);
		}
		outs
	}

	fn   getInputs(line:String )->Vec<i32> {
		let  scanner = line[line.find("<").unwrap() + 1,
				line.find(">").unwrap()];
		let mut  ins = vec![];
		while let Some(v)=scanner.next() {
			ins.push(v);
		}
		
		 ins
	}

	pub fn  getAssignment(&self)->Vec<BigInteger> {
		self.valueAssignment.clone()
	}

}
