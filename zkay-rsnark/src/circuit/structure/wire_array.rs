

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::util::util::{Util,BigInteger};
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::add_basic_op::AddBasicOp;
use crate::circuit::operations::primitive::pack_basic_op::PackBasicOp;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::linear_combination_wire::LinearCombinationWire;

 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct WireArray {
	array: Vec<WireType> ,
}
	// pub fn newWireArrayWithI32(n:i32)->WireArray {
	// 	newWireArrayWithNAndGenerator(n, CircuitGenerator::getActiveCircuitGenerator())
	// }
	
	// pub fn newWireArrayWithNAndGenerator(n:i32, generator:CircuitGenerator)->WireArray {
    //     WireArray::new(vec![WireType::default();n as usize],generator)
	// }
	
	// pub fn newWireArrayWithArray(wireArray:Vec<WireType>)->WireArray {
	// 	WireArray::new(wireArray)
	// }

impl WireArray{
	pub fn new(wireArray:Vec<WireType>)->Self {
        Self{array : wireArray}
	}
	
	pub fn get(&self,i:i32)->WireType{
		return self.array[i];
	}
	
	pub fn set(&mut self,i:i32, w:WireType){
		self.array[i] = w;
	}
	
	pub fn size(&self)->usize{
		return self.array.len();
	}
	
	pub fn asArray(&self)-> Vec<WireType>{
		return self.array.clone();
	}
	pub fn generator(&self)->CircuitGenerator{
        CircuitGenerator::getActiveCircuitGenerator().unwrap().clone()
    }
	pub fn mulWireArray(&self,v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength( self.array, desiredLength);
let ws2 = self.adjustLength( v.array, desiredLength);
let out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].mul(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	
	pub fn sumAllElements(&self,desc:Vec<String>)->WireType {
let allConstant = true;
let sum = BigInteger::ZERO;
		for w in  &self.array {
			if w.ConstantWire().is_none() {
				allConstant = false;
				break;
			} else {
				sum = sum.add( w.getConstant());
			}
		}
		if !allConstant {

			let output = LinearCombinationWire::new(self.generator().currentWireId);
            self.generator().currentWireId+=1;
let op = AddBasicOp::new(self.array, output, desc);
//			self.generator().addToEvaluationQueue(op);
let cachedOutputs = self.generator().addToEvaluationQueue(op);
			return if let Some(cachedOutputs) =cachedOutputs{
				self.generator().currentWireId-=1;
				 cachedOutputs[0].clone()
			}	else{
             output}
		}

		self.generator().createConstantWire(sum, desc)
	}
	
	
	pub fn addWireArray(&self,v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array, desiredLength);
let ws2 = self.adjustLength( v.array, desiredLength);
let out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].add(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArray(&self,v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array, desiredLength);
let ws2 = self.adjustLength(v.array, desiredLength);
let out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].xor(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArrayi(&self,v:WireArray , desc:Vec<String>)->WireArray {
		assert!(self.size() == v.size());
let ws1 = self.array;
let ws2 = v.array;
		
let out = vec![WireType::default();self.size()];
		for i in 0..out.len() {
			out[i] = ws1[i].xor(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn andWireArray(&self,v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength( self.array, desiredLength);
let ws2 = self.adjustLength( v.array, desiredLength);
let out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].mul(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn orWireArray(&self,v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array, desiredLength);
let ws2 = self.adjustLength(v.array, desiredLength);
let out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].or(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	
	
	pub fn invAsBits(&self,desiredBitWidth:i32 , desc:Vec<String>) ->WireArray{
let out = vec![WireType::default();desiredBitWidth];
		for i in  0.. desiredBitWidth{
			if i < self.array.len(){
				out[i] = self.array[i].invAsBit(desc);
			}
			else{
				out[i] = self.generator().oneWire;
			}
		}
		return WireArray::new(out);
	}	
	
	
	fn adjustLength(&self,ws:Vec<WireType> , desiredLength:i32 )->Vec<WireType> {
		if ws.len() == desiredLength{
			return ws;
		}
        let mut newWs = vec![WireType::default();desiredLength];
		newWs[.. std::cmp::min(ws.len(), desiredLength)].clone_from_slice(&ws);
		if ws.len() < desiredLength {
			for i in  ws.len().. desiredLength{
				newWs[i] = self.generator().zeroWire;
			}
		}
		return newWs;
	}
	
	pub fn adjustLengths(&self,desiredLength:i32 ) ->WireArray{
		if self.array.len() == desiredLength{
			return self.clone();
		}
let newWs = vec![WireType::default();desiredLength];
		newWs[.. std::cmp::min(self.array.len(), desiredLength)].clone_from_slice(&self.array);
		if self.array.len() < desiredLength {
			for i in  self.array.len().. desiredLength{
				newWs[i] = self.generator().zeroWire;
			}
		}
		return WireArray::new(newWs);
	}
	
	
	
	pub fn packAsBitsi(&self,n:i32, desc:Vec<String>)->WireType {
		return self.packAsBits(0, n, desc);
	}
	
	pub fn packAsBits(&self,desc:Vec<String>) ->WireType{
		return self.packAsBits(self.array.len(), desc);
	}
	
	 fn  checkIfConstantBits(&self,desc:Vec<String>)->Option<BigInteger>{
let mut allConstant = true;
let sum = BigInteger::ZERO;
		for i in  0.. self.array.len(){
			let w = self.array[i];
			if let Some(cw)=w.ConstantWire(){
                let v = cw.constant;
				if v.equals(Util::one()){
					sum = sum.add(v.shiftLeft(i));
				}
				else if !v.equals(BigInteger::ZERO){
					println!("Warning, one of the bit wires is constant but not binary : {}" , Util::getDesc(desc));					
				}
				
			}
			else{
				allConstant = false;
			}
		}
		 allConstant.then_some(sum)
	}

	pub fn packAsBitsii(&self,from:i32 , to:i32 , desc:Vec<String>)->WireType {
		assert!(from <= to && to <= self.array.len(),"Invalid bounds: from > to");
		
let bits =self.array[from.. to].to_vec();
let allConstant = true;
let sum = BigInteger::ZERO;
		for i in  0.. bits.len(){
			let w = bits[i];
			if let Some(cw)=w.ConstantWire(){
let v = cw.constant;
				if v.equals(Util::one()){
					sum = sum.add(v.shiftLeft(i));
				}
				else {
					assert!(v.equals(BigInteger::ZERO),"Trying to pack non-binary constant bits : {}" , Util::getDesc(desc));					
				}
				
			}
			else{
				allConstant = false;
			}
		}
		if !allConstant{
let out = LinearCombinationWire::new(self.generator().currentWireId);
            self.generator().currentWireId+=1;
			out.setBits(WireArray::new(bits));
let op = PackBasicOp::new(bits, out, desc);
let cachedOutputs = self.generator().addToEvaluationQueue(op);
			return if let Some(cachedOutputs) =cachedOutputs{
                self.generator().currentWireId-=1;         
				 cachedOutputs[0].clone()
            }else
				 {out	
			}
			

		} 
			 self.generator().createConstantWire(sum, desc)

		
	}
	
	
	pub fn rotateLeft(&self,numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
		let mut  bits = self.adjustLength(self.array, numBits);
let mut rotatedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i < s
				{rotatedBits[i] = bits[i + (numBits - s)].clone();}
			else
				{rotatedBits[i] = bits[i - s].clone();}
		}
		return WireArray::new(rotatedBits);
	}
	
	pub fn rotateRight(&self,numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = self.adjustLength(self.array, numBits);
let rotatedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i >= numBits - s
				{rotatedBits[i] = bits[i - (numBits - s)];}
			else
				{rotatedBits[i] = bits[i + s];}
		}
		return WireArray::new(rotatedBits);
	}
	
	

	pub fn shiftLeft(&self,numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = self.adjustLength( self.array, numBits);
let shiftedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i < s
				{shiftedBits[i] = self.generator().zeroWire;}
			else
				{shiftedBits[i] = bits[i - s];}
		}
		return WireArray::new(shiftedBits);
	}
	
	pub fn shiftRight(&self,numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = self.adjustLength(self.array, numBits);
let shiftedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i >= numBits - s
				{shiftedBits[i] = self.generator().zeroWire;}
			else
				{shiftedBits[i] = bits[i + s];}
		}
		return WireArray::new(shiftedBits);
	}
		
	pub fn packBitsIntoWords(&self,wordBitwidth:i32 , desc:Vec<String>)-> Vec<WireType> {
let numWords = (self.array.len() as f64*1.0/wordBitwidth as f64).ceil() as i32;
let padded = self.adjustLength( self.array, wordBitwidth*numWords);
let result = vec![WireType::default();numWords];
		for i in  0.. numWords{
			result[i] = WireArray::new(padded[i*wordBitwidth.. (i+1)*wordBitwidth].to_vec()).packAsBits();
		}
		return result;
	}
	
	pub fn packWordsIntoLargerWords(&self,wordBitwidth:i32 , numWordsPerLargerWord:i32 , desc:Vec<String>)-> Vec<WireType> {
let numLargerWords = (self.array.len()*1.0/numWordsPerLargerWord).ceil() as i32;
let result = vec![self.generator().zeroWire.clone();numLargerWords];
		for i in  0.. self.array.len(){
let subIndex = i % numWordsPerLargerWord;
			result[i/numWordsPerLargerWord] = result[i/numWordsPerLargerWord].add(self.array[i]
					.mul(BigInteger::new("2").pow(subIndex*wordBitwidth)));
 		}
		return result;
		
	}

	pub fn getBits(&self,bitwidth:i32 , desc:Vec<String>) ->WireArray{
let bits = vec![WireType::default();bitwidth * self.array.len()];
let idx = 0;
		for i in 0..self.array.len() {
let tmp = self.array[i].getBitWires(bitwidth, desc).asArray();
			for j in 0..bitwidth {
				bits[idx+=1] = tmp[j];
			}
		}
		return WireArray::new(bits);
	}
	
}
