#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]

// use  Util;
// use  crate::gadgetlib2::integration;
// use  crate::gadgetlib2::adapters;
// use  ffec::common::profiling;
use std::{
process,
    fmt::Debug,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufRead, BufReader},
    ops::{Add, Mul, Neg, Sub},
};


// use  <memory.h>
// use  <iostream>
// use  <sstream>
// use  <fstream>
// use  <list>
// use  <vector>
// use  <set>
// use  <map>
// use  <ctime>

// use  <termios.h>
// use  <unistd.h>
// use  <stdio.h>


// //#ifndef NO_PROCPS
// use  <proc/readproc.h>
// //#endif


// 
// using namespace gadgetlib2;
// using namespace std;
use std::collections::HashMap;

type Wire=u32;

type FieldT=ffec::Fr<ffec::default_ec_pp> ;
// type ::std::shared_ptr<LinearCombination> LinearCombinationPtr;
type WireMap=HashMap<Wire, u32> ;

const  ADD_OPCODE:u32=1;
const  MUL_OPCODE:u32=2;
const  SPLIT_OPCODE:u32=3;
const  NONZEROCHECK_OPCODE:u32=4;
const  PACK_OPCODE:u32=5;
const  MULCONST_OPCODE:u32=6;
const  XOR_OPCODE:u32=7;
const  OR_OPCODE:u32=8;
const  CONSTRAINT_OPCODE:u32=9;

pub struct CircuitReader {
// 
// 	CircuitReader(arithFilepath:&str, inputsFilepath:&str, ProtoboardPtr pb);

// 	int getNumInputs() { return numInputs;}
// 	int getNumOutputs() { return numOutputs;}
// 	std::vector<Wire> getInputWireIds() const { return inputWireIds; }
// 	std::vector<Wire> getOutputWireIds() const { return outputWireIds; }

// private:
pb:ProtoboardPtr,

variables:	Vec<VariablePtr>,
wireLinearCombinations:	Vec<LinearCombinationPtr>,
zeroPwires:	Vec<LinearCombinationPtr>,

variableMap:	WireMap,
zeropMap:	WireMap,

wireUseCounters:	Vec<u32>,
wireValues:	Vec<FieldT>,

toClean:	Vec<Wire>,

inputWireIds:	Vec<Wire>,
nizkWireIds:	Vec<Wire>,
outputWireIds:	Vec<Wire>,

numWires:	u32,
numOutputs:	u32 ,
numInputs:u32,
numNizkInputs:u32,

currentLinearCombinationIdx:	u32 ,
currentVariableIdx:u32,

	// void parseAndEval(arithFilepath:&str, inputsFilepath:&str);
	// void constructCircuit(const char*);  // Second Pass:
	// void mapValuesToProtoboard();

	// void find(unsigned int, LinearCombinationPtr&, bool intentionToEdit = false);
	// void clean();

	// void addMulConstraint(char*, char*);
	// void addXorConstraint(char*, char*);

	// void addOrConstraint(char*, char*);
	// void addAssertionConstraint(char*, char*);

	// void addSplitConstraint(char*, char*, unsigned short);
	// // void addPackConstraint(char*, char*, unsigned short);
	// void addNonzeroCheckConstraint(char*, char*);

	// void handleAddition(char*, char*);
	// void handlePackOperation(char*, char*, unsigned short);
	// void handleMulConst(char*, char*, char*);
	// void handleMulNegConst(char*, char*, char*);

}

// use  "circuit_reader.hpp"

impl CircuitReader{
    pub fn new(arithFilepath:&str, inputsFilepath:&str,
		 pb:ProtoboardPtr)->Self {
    let mut _self=Self{
pb,
variables:	vec![],
wireLinearCombinations:	vec![],
zeroPwires:	vec![],
variableMap:	WireMap::new(),
zeropMap:	WireMap::new(),
wireUseCounters:	vec![],
wireValues:	vec![],
toClean:	vec![],
inputWireIds:	vec![],
nizkWireIds:	vec![],
outputWireIds:	vec![],
numWires:	0,
numOutputs:	0 ,
numInputs:0,
numNizkInputs:0,
currentLinearCombinationIdx:	0 ,
currentVariableIdx:0,
    };

	_self.parseAndEval(arithFilepath, inputsFilepath);
	_self.constructCircuit(arithFilepath);
	_self.mapValuesToProtoboard();

	_self.wireLinearCombinations.clear();
	_self.wireValues.clear();
	_self.variables.clear();
	_self.variableMap.clear();
	_self.zeropMap.clear();
	_self.zeroPwires.clear();
    _self
}

pub fn parseAndEval(&mut self,arithFilepath:&str, inputsFilepath:&str) {

	ffec::enter_block("Parsing and Evaluating the circuit");
    let mut arithfs = fs::read_to_string(arithFilepath);
    let mut inputfs = fs::read_to_string(inputsFilepath);

	let mut  line;

	if arithfs.is_err() {
		println!("Unable to open circuit file {} \n", arithFilepath);
		process::process::exit(-1);
	}
    let arithfs=arithfs.unwrap().lines();
	line=arithfs.next().unwrap();
	let ret = scan_fmt!(line, "total {d}",u32);

	if ret.is_err() {
		println!("File Format Does not Match\n");
		process::exit(-1);
	}
    self.numWires=ret.unwrap().0;
	self.wireValues.resize(self.numWires);
	self.wireUseCounters.resize(self.numWires);
	self.wireLinearCombinations.resize(self.numWires);

	if inputfs.is_err() {
		println!("Unable to open input file {} \n", inputsFilepath);
		process::exit(-1);
	} 
     let inputfs=inputfs.unwrap().lines();
		let mut inputStr;
		for line in inputfs {
			if line.is_empty() {
				continue;
			}
			let mut  wireId;
			if let Ok((wireId, inputStr))=scan_fmt!(line, "{} {}",u32,String) {
				wireValues[wireId] = Util::readFieldElementFromHex(inputStr);
			} else {
				println!("Error in Input\n");
				process::exit(-1);
			}
			
		}
		
	

	if wireValues[0] != FieldT::one() {
		println!(">> Warning: when using jsnark circuit generator, the first input wire (#0) must have the value of 1.\n");
		println!("\t If the circuit was generated using Pinocchio *without modification*, you can ignore this warning. Pinocchio uses a different indexing for the one-wire input. \n");
	}

	// char types[200];
	let mut inputStr:&str;
	let mut outputStr:&str;
	let  (mut numGateInputs, mut numGateOutputs);

	let mut wireId;

	let  oneElement = FieldT::one();
	let zeroElement = FieldT::zero();
	let negOneElement = FieldT(-1);

	// long long evalTime;
	// long long begin, end;
	// evalTime = 0;

	// Parse the circuit: few lines were imported from Pinocchio's code.

	for line in arithfs {
		if line.is_empty(){
			continue;
		}

		if line[0] == '#' {
			continue;
		}
        if let Ok((wireId))= scan_fmt!(line, "input {}",u32) {
			self.numInputs+=1;
			self.inputWireIds.push(wireId);
		} else if let Ok((wireId))= scan_fmt!(line, "nizkinput {}",u32) {
			self.numNizkInputs+=1;
			self.nizkWireIds.push(wireId);
		} else if let Ok((wireId))= scan_fmt!(line, "output {}",u32) {
			self.numOutputs+=1;
			self.outputWireIds.push(wireId);
			self.wireUseCounters[wireId]+=1;
		} else if let Ok((types,
						numGateInputs, inputStr, &numGateOutputs, outputStr))= scan_fmt!(line, "{} in {} <{:/[^>]+/}> out {} <{:/[^>]+/}>",String,u32,String,u32,String) {

			let mut  iss_i=inputStr.lines();
			let mut inValues=vec![];
			let mut outWires=vec![];
	
			for  s in inputStr.split_ascii_whitespace() {
                let inWireId=s.parse::<u32>().unwrap() as usize;
				self.wireUseCounters[inWireId]+=1;
				inValues.push(wireValues[inWireId]);
			}
			Util::readIds(outputStr, outWires);

			let mut  opcode;
			let mut  constant;
            match types{
             "add"=> {
				opcode = ADD_OPCODE;
			} "mul"=> {
				opcode = MUL_OPCODE;
			} 
             "xor"=> {
				opcode = XOR_OPCODE;
			} 
            "or"=>{
				opcode = OR_OPCODE;
			} "assert"=> {
				wireUseCounters[outWires[0]]+=1;
				opcode = CONSTRAINT_OPCODE;
			} 
             "pack"=> {
				opcode = PACK_OPCODE;
			} 
             "zerop"=> {
				opcode = NONZEROCHECK_OPCODE;
			} 
             "split"=> {
				opcode = SPLIT_OPCODE;
			} 
            _ if types.contains("const-mul-neg-") =>{
				opcode = MULCONST_OPCODE;
				let constStr = &types["const-mul-neg-".len() - 1..];
				constant = Util::readFieldElementFromHex(constStr) * negOneElement;
			}  
            _ if types.contains("const-mul-") =>{
				opcode = MULCONST_OPCODE;
				let constStr =  &types["const-mul-".len() - 1..];
				constant = Util::readFieldElementFromHex(constStr);
			} _ =>{
				println!("Error: unrecognized line: {line}\n");
				panic!("0");
			}
            }
			// TODO: separate evaluation from parsing completely to get accurate evaluation cost
			//	 Calling  ffec::get_nsec_time(); repetitively as in the old version adds much overhead 
			// TODO 2: change circuit format to enable skipping some lines during evaluation
			//       Not all intermediate wire values need to be computed in this phase
			// TODO 3: change circuit format to make common constants defined once			
	
			//begin = ffec::get_nsec_time();
			if opcode == ADD_OPCODE {
				wireValues[outWires[0]] = inValues.iter().sum();
			} else if opcode == MUL_OPCODE {
				wireValues[outWires[0]] = inValues[0] * inValues[1];
			} else if opcode == XOR_OPCODE {
				wireValues[outWires[0]] =
						 if inValues[0] == inValues[1]{ zeroElement} else {oneElement};
			} else if opcode == OR_OPCODE {
				wireValues[outWires[0]] =
						if inValues[0] == zeroElement
								&& inValues[1] == zeroElement
								{zeroElement} else {oneElement};
			} else if opcode == NONZEROCHECK_OPCODE {
				wireValues[outWires[1]] =
						if inValues[0] == zeroElement  {zeroElement} else {oneElement};
			} else if opcode == PACK_OPCODE {
				let  (mut sum,mut  coeff);
				let mut  two = oneElement;
				for   &v in  &inValues {
					sum += two * v;
					two += two;
				}
				wireValues[outWires[0]] = sum;
			} else if opcode == SPLIT_OPCODE {
				let  size = outWires.len();
				let  inVal = inValues[0];
				for i in 0..size {
					wireValues[outWires[i]] = inVal.getBit(i, R1P);
				}
			} else if opcode == MULCONST_OPCODE {
				wireValues[outWires[0]] = constant * inValues[0];
			}
			//end =  ffec::get_nsec_time();
			//evalTime += (end - begin);
		} else {
			println!("Error: unrecognized line: {line}\n");
			panic!("0");
		}

	}
	

	// println!("\t Evaluation Done in %lf seconds \n", (double) (evalTime) * 1e-9);
	 ffec::leave_block("Parsing and Evaluating the circuit");
}

pub fn constructCircuit(arithFilepath:&str) {



	println!("Translating Constraints ... ");

	
	// //#ifndef NO_PROCPS
	// struct proc_t usage1, usage2;
	// look_up_our_self(&usage1);
    //     //#endif
	

	

	let (mut currentVariableIdx ,mut  currentLinearCombinationIdx) = (0,0);
	for i in 0..numInputs {
		variables.push(Variable::new("input"));
		variableMap[inputWireIds[i]] = currentVariableIdx;
		currentVariableIdx+=1;
	}
	for i in 0..numOutputs {
		variables.push(Variable::new("output"));
		variableMap[outputWireIds[i]] = currentVariableIdx;
		currentVariableIdx+=1;
	}
	for i in  0.. numNizkInputs {
		variables.push(Variable::new("nizk input"));
		variableMap[nizkWireIds[i]] = currentVariableIdx;
		currentVariableIdx+=1;
	}

	// char types[200];
	// inputStr:&str;
	// outputStr:&str;
	// string line;
	let (mut  numGateInputs, mut numGateOutputs);

	let  mut ifs2=fs::read_to_string(arithFilepath);

	if ifs2.is_err() {
		println!("Unable to open circuit file:\n");
		process::exit(5);
	}

	// Parse the circuit: few lines were imported from Pinocchio's code.
    let ifs2=ifs2.unwrap().lines();
	let mut line=ifs2.next().unwrap();
	let Ok((numWires))=scan_fmt(line.c, "total {i32}") else{
        eprintln!("=======================");
    return };

	let mut  lineCount = 0;
	for line in ifs2 {
		lineCount+=1;
//		if lineCount % 100000 == 0 {
//			println!("At Line:: %d\n", lineCount);
//		}

		if line.is_empty() {
			continue;
		}


		if let Ok((types,
						numGateInputs, inputStr, numGateOutputs, outputStr))
				= scan_fmt!(line.c_str(), "{} in {} <{:/[^>]+/}> out {} <{:/[^>]+/}>",String,i32,String,i32,String ) {
            match types{
			 "add"=> {
				assert!(numGateOutputs == 1);
				self.handleAddition(inputStr, outputStr);
			}  "mul"=> {
				assert!(numGateInputs == 2 && numGateOutputs == 1);
				self.addMulConstraint(inputStr, outputStr);
			}  "xor"=> {
				assert!(numGateInputs == 2 && numGateOutputs == 1);
				self.addXorConstraint(inputStr, outputStr);
			}  "or"=> {
				assert!(numGateInputs == 2 && numGateOutputs == 1);
				self.addOrConstraint(inputStr, outputStr);
			}  "assert"=> {
				assert!(numGateInputs == 2 && numGateOutputs == 1);
				self.addAssertionConstraint(inputStr, outputStr);
			} _ if types.contains( "const-mul-neg-") =>{
				assert!(numGateInputs == 1 && numGateOutputs == 1);
				self.handleMulNegConst(types, inputStr, outputStr);
			} _ if types.contains("const-mul-")=> {
				assert!(numGateInputs == 1 && numGateOutputs == 1);
				self.handleMulConst(types, inputStr, outputStr);
			}  "zerop"=> {
				assert!(numGateInputs == 1 && numGateOutputs == 2);
				self.addNonzeroCheckConstraint(inputStr, outputStr);
			} _ if types.contains( "split") =>{
				assert!(numGateInputs == 1);
				self.addSplitConstraint(inputStr, outputStr, numGateOutputs);
			} _ if types.contains( "pack") =>{
				assert!(numGateOutputs == 1);
				// addPackConstraint(inputStr, outputStr, numGateInputs);
				self.handlePackOperation(inputStr, outputStr, numGateInputs);

			}
        }
		} else {
//			assert!(0);
		}

		self.clean();
	}



	println!("\tConstraint translation done\n");


	
	// //#ifndef NO_PROCPS
	// look_up_our_self(&usage2);
	// unsigned long diff = usage2.vsize - usage1.vsize;
	// println!("\tMemory usage for constraint translation: %lu MB\n", diff >> 20);
    //     //#endif
        
}

pub fn mapValuesToProtoboard(&self) {

	let mut  zeropGateIndex = 0;
	for (wireId,v) in variableMap.keys() {
		pb.val(variables[v]) = wireValues[wireId];
		if let Some(z)=zeropMap.get(wireId) {
			let  l = zeroPwires[zeropGateIndex];
            zeropGateIndex+=1;
			if pb.val(l) == FieldT::zero() {
				pb.val(variables[z]) = FieldT::zero();
			} else {
				pb.val(variables[z]) = pb.val(l).inverse(
						pb.fieldType_);
			}
		}
	}
	if !pb.isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED) {
		println!("Note: Protoboard Not Satisfied .. \n");
		// assert!(false);
	}
	println!("Assignment of values done .. \n");

}

pub fn find(&self, wireId:Wire,  lc:&mut LinearCombinationPtr,
		 intentionToEdit:bool) {

	if !wireLinearCombinations[wireId]{
		wireLinearCombinations[wireId] = LinearCombination::new(
				LinearCombination(variables[variableMap[wireId]]));
	}
	wireUseCounters[wireId]-=1;
	if wireUseCounters[wireId] == 0 {
		toClean.push(wireId);
		*lc = wireLinearCombinations[wireId];
	} else {
		if intentionToEdit {
			*lc = LinearCombination::new(wireLinearCombinations[wireId]);
		} else {
			*lc = wireLinearCombinations[wireId];
		}
	}
}



pub fn clean(&mut self) {
	for  wireId in  toClean {
		wireLinearCombinations[wireId].reset();
	}
	toClean.clear();
}

pub fn addMulConstraint(&self,inputStr:&str, outputStr:&str) {

	let ( outputWireId, inWireId1, inWireId2);

	let mut iss_i=inputStr.lines();
	 inWireId1=iss_i.next().unwrap();
	inWireId2=iss_i.next().unwrap();
	let mut  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();

	let  (l1, l2);
	self.find(inWireId1, l1);
	self.find(inWireId2, l2);

	if let Some(v)=variableMap.get(outputWireId) {
		pb.addRank1Constraint(l1, l2, variables[v],
				"Mul ..");
	}else {
		variables.push(Variable::new("mul out"));
		variableMap[outputWireId] = currentVariableIdx;
		pb.addRank1Constraint(l1, l2, variables[currentVariableIdx],
				"Mul ..");
		currentVariableIdx+=1;
	} 
}

pub fn addXorConstraint(&self,inputStr:&str, outputStr:&str) {

	let  (outputWireId, inWireId1, inWireId2);

	let  mut iss_i=inputStr.lines();
	 inWireId1=iss_i.next().unwrap();
	 inWireId2=iss_i.next().unwrap();
	let mut  iss_o=outputStr.lines();
	 outputWireId=iss_o.next().unwrap();

	let ( lp1, lp2);
	find(inWireId1, lp1);
	find(inWireId2, lp2);
	let  (l1, l2)=(lp1.clone(),lp2.clone());

	if let Some(v)=variableMap.get(outputWireId) {
		pb.addRank1Constraint(2 * l1, l2,
				l1 + l2 - *variables[variableMap[outputWireId]], "XOR ..");
	}else{
		variables.push(Variable::new("xor out"));
		variableMap[outputWireId] = currentVariableIdx;
		pb.addRank1Constraint(2 * l1, l2,
				l1 + l2 - *variables[currentVariableIdx], "XOR ..");
		currentVariableIdx+=1;
	} 
}

pub fn addOrConstraint(&self,inputStr:&str, outputStr:&str) {

	let  (outputWireId, inWireId1, inWireId2);

	let mut  iss_i=inputStr.lines();
	 inWireId1=iss_i.next().unwrap();
	inWireId2=iss_i.next().unwrap();
	let  iss_o=outputStr.lines();
     outputWireId=iss_o.next().unwrap();

	let  (lp1, lp2);
	find(inWireId1, lp1);
	find(inWireId2, lp2);
	let  (l1, l2)=(lp1,lp2);

	if let Some(v)=variableMap.get(outputWireId) {
		pb.addRank1Constraint(l1, l2,
				l1 + l2 - *variables[v], "OR ..");
	}else{
		variables.push(Variable::new("or out"));
		variableMap[outputWireId] = currentVariableIdx;
		pb.addRank1Constraint(l1, l2, l1 + l2 - *variables[currentVariableIdx],
				"OR ..");
		currentVariableIdx+=1;
	}  
}

pub fn addAssertionConstraint(&self,inputStr:&str, outputStr:&str) {

	let  (outputWireId, inWireId1, inWireId2);

	let mut  iss_i=inputStr.lines();
	inWireId1=iss_i.next().unwrap();
	inWireId2=iss_i.next().unwrap();
	let  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();

	let (lp1, lp2, lp3);
	find(inWireId1, lp1);
	find(inWireId2, lp2);
	find(outputWireId, lp3);

	let  (l1, l2, l3)=(lp1,lp2,lp3);
	pb.addRank1Constraint(l1, l2, l3, "Assertion ..");

}

pub fn addSplitConstraint(&self,inputStr:&str, outputStr:&str,
		 n:u16) {


	let mut  iss_i=inputStr.lines();
	let  inWireId=iss_i.next().unwrap();

	let mut l;
	find(inWireId, l);

	let mut  iss_o=outputStr.lines();

	let mut  sum;
	let mut  two_i = ffec::Fr::<ffec::default_ec_pp> ("1");



	for i in 0..n {
		let  bitWireId=iss_o.next().unwrap();
		let mut  vptr;
		if let Some(v)=variableMap.get(bitWireId){
			vptr = variables[v];
		}else  {
			variables.push(Variable::new("bit out"));
			variableMap[bitWireId] = currentVariableIdx;
			vptr = variables[currentVariableIdx];
			currentVariableIdx+=1;
		}  
		pb.enforceBooleanity(*vptr);
		sum += LinearTerm(*vptr, two_i);
		two_i += two_i;
	}


	pb.addRank1Constraint(*l, 1, sum, "Split Constraint");
}

pub fn addNonzeroCheckConstraint(&self,inputStr:&str, outputStr:&str) {

	let mut  auxConditionInverse_;
	let  (outputWireId, inWireId);

	let mut  iss_i=inputStr.lines();
	 inWireId=iss_i.next().unwrap();
	let  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();
	outputWireId=iss_o.next().unwrap();
	let mut  l;

	find(inWireId, l);
	let mut  vptr;
	if let Some(v)=variableMap.get(outputWireId){
		vptr = variables[v];
	}else {
		variables.push(Variable::new("zerop out"));
		variableMap[outputWireId] = currentVariableIdx;
		vptr = variables[currentVariableIdx];
		currentVariableIdx+=1;
	}  
	variables.push(Variable::new("zerop aux"));
	pb.addRank1Constraint(*l, 1 - *vptr, 0, "condition * not(output) = 0");
	pb.addRank1Constraint(*l, *variables[currentVariableIdx], *vptr,
			"condition * auxConditionInverse = output");

	zeroPwires.push(LinearCombination::new(*l));
	zeropMap[outputWireId] = currentVariableIdx;
	currentVariableIdx+=1;

}


pub fn handlePackOperation(&self,inputStr:&str, outputStr:&str, n:u16){

	
	let  iss_o=outputStr.lines();
	let outputWireId=iss_o.next().unwrap();

	if variableMap.contains_key(outputWireId){
		println!("An output of a pack operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n");
                println!("\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n");
		process::exit(-1);
	}


	let mut  iss_i=inputStr.lines();
	let mut  sum;
	let  mut bitWireId=iss_i.next().unwrap();

	find(bitWireId, sum, true);	       
	let mut  two_i = ffec::Fr::<ffec::default_ec_pp> ("1");
	for i in 1..n {
		bitWireId=iss_i.next().unwrap();
		let mut  l;
		find(bitWireId, l);
		two_i += two_i;
		*sum += two_i * (*l);
	}
	wireLinearCombinations[outputWireId] = sum;
}

pub fn handleAddition(&self,inputStr:&str, outputStr:&str) {

	let  (inWireId, outputWireId);
	let  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();

	if variableMap.contains_key(outputWireId) {
		println!("An output of an add operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n");
                println!("\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n");
		process::exit(-1);
	}

	let mut  iss_i=inputStr.lines();
	let  (s, l);
	inWireId=iss_i.next().unwrap();
	find(inWireId, l, true);
	s = l;
	for inWireId in iss_i {
		find(inWireId, l);
		*s += *l;
	}
	wireLinearCombinations[outputWireId] = s;
}

pub fn handleMulConst(&self,types:&str, inputStr:&str,
		outputStr:&str) {

	let constStr= &types["const-mul-".len() - 1..];
	let  (outputWireId, inWireId);

	let  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();

	if variableMap.contains_key(outputWireId)  {
		println!("An output of a const-mul operation was either defined before, or is declared directly as a circuit output. Non-compliant Circuit.\n");
                println!("\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n");
		process::exit(-1);
	}

	let mut  iss_i=inputStr.lines();
	inWireId=iss_i.next().unwrap();
	let mut  l;
	find(inWireId, l, true);
	wireLinearCombinations[outputWireId] = l;
	*(wireLinearCombinations[outputWireId]) *= readFieldElementFromHex(
			constStr);
}

pub fn handleMulNegConst(&self,types:&str, inputStr:&str,
		outputStr:&str) {

	let constStr = &types["const-mul-neg-".len() - 1..];
	let  (outputWireId, inWireId);
	let  iss_o=outputStr.lines();
	outputWireId=iss_o.next().unwrap();

	if variableMap.contains_key(outputWireId) {
		println!("An output of a const-mul-neg operation was either defined before, or is declared directly as circuit output. Non-compliant Circuit.\n");
                println!("\t If the second, the wire has to be multiplied by a wire the has the value of 1 first (input #0 in circuits generated by jsnark) . \n");
		process::exit(-1);
	}

	let mut  iss_i=inputStr.lines();
	inWireId=iss_i.next().unwrap();

	let mut  l;
	find(inWireId, l, true);

	wireLinearCombinations[outputWireId] = l;
	*(wireLinearCombinations[outputWireId]) *= readFieldElementFromHex(
			constStr);
	*(wireLinearCombinations[outputWireId]) *= FieldT(-1); //TODO: make shared FieldT constants

}
}

