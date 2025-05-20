

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
use crate::circuit::structure::wire::WireConfig;
use std::ops::{Mul,Add,Sub};
use std::hash::{DefaultHasher, Hash, Hasher};
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash,PartialEq)]
pub struct WireArray {
pub	array: Vec<Option<WireType>> ,
}


impl WireArray{
	pub fn newi(n:i32)->WireArray {
		Self::newic(n, CircuitGenerator::getActiveCircuitGenerator())
	}
	
	pub fn newic(n:i32, generator:CircuitGenerator)->WireArray {
        WireArray::new(vec![WireType::default();n as usize],generator)
	}
	
	pub fn new(wireArray:Vec<Option<WireType>>)->Self {
        Self{array : wireArray}
	}
	
	pub fn get(&self,i:usize)->WireType{
		return self.array[i];
	}
	
	pub fn set(&mut self,i:usize, w:WireType){
		self.array[i] = w;
	}
	
	pub fn size(&self)->usize{
		return self.array.len();
	}
	
	pub fn asArray(&self)-> Vec<Option<WireType>>{
		return self.array.clone();
	}
	pub fn generator(&self)->CircuitGenerator{
        CircuitGenerator::getActiveCircuitGenerator().unwrap().clone()
    }
	pub fn mulWireArray(&self,v:WireArray , desiredLength:usize , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength( self.array, desiredLength);
let ws2 = self.adjustLength( v.array, desiredLength);
let mut out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().mul(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	
	pub fn sumAllElements(&self,desc:Vec<String>)->WireType {
let mut allConstant = true;
let mut sum = BigInteger::ZERO;
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
	
	
	pub fn addWireArray(&self,v:WireArray , desiredLength:usize , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array.clone(), desiredLength);
let ws2 = self.adjustLength( v.array.clone(), desiredLength);
let mut out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().add(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArray(&self,v:WireArray , desiredLength:usize , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array.clone(), desiredLength);
let ws2 = self.adjustLength(v.array.clone(), desiredLength);
let mut out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().xor(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArrayi(&self,v:WireArray , desc:Vec<String>)->WireArray {
		assert!(self.size() == v.size());
let ws1 = self.array.clone();
let ws2 = v.array.clone();
		
let mut out = vec![WireType::default();self.size()];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().xor(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	pub fn andWireArray(&self,v:WireArray , desiredLength:usize , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength( self.array, desiredLength);
let ws2 = self.adjustLength( v.array, desiredLength);
let mut  out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().mul(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	pub fn orWireArray(&self,v:WireArray , desiredLength:usize , desc:Vec<String>)->WireArray {
let ws1 = self.adjustLength(self.array, desiredLength);
let ws2 = self.adjustLength(v.array, desiredLength);
let mut out = vec![WireType::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].clone().or(ws2[i].clone(), desc.clone());
		}
		return WireArray::new(out);
	}
	
	
	
	pub fn invAsBits(&self,desiredBitWidth:usize , desc:Vec<String>) ->WireArray{
let mut out = vec![WireType::default();desiredBitWidth];
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
	
	
	pub fn adjustLength(&self,ws:Vec<Option<WireType>> , desiredLength:usize )->Vec<Option<WireType>> {
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
	
	pub fn adjustLengthi(&self,desiredLength:usize ) ->WireArray{
		if self.array.len() == desiredLength{
			return self.clone();
		}
let mut newWs = vec![WireType::default();desiredLength];
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
let mut sum = BigInteger::ZERO;
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

	pub fn packAsBitsii(&self,from:usize , to:usize , desc:Vec<String>)->WireType {
		assert!(from <= to && to <= self.array.len(),"Invalid bounds: from > to");
		
let bits =self.array[from.. to].to_vec();
let mut allConstant = true;
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
	
	
	pub fn rotateLeft(&self,numBits:usize , s:usize , desc:Vec<String>)->WireArray {
		let mut  bits = self.adjustLength(self.array.clone(), numBits);
let mut rotatedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i < s
				{rotatedBits[i] = bits[i + (numBits - s)].clone();}
			else
				{rotatedBits[i] = bits[i - s].clone();}
		}
		return WireArray::new(rotatedBits);
	}
	
	pub fn rotateRight(&self,numBits:usize , s:usize , desc:Vec<String>)->WireArray {
let bits = self.adjustLength(self.array.clone(), numBits);
let mut rotatedBits = vec![WireType::default();numBits];
		for i in 0..numBits {
			if i >= numBits - s
				{rotatedBits[i] = bits[i - (numBits - s)].clone();}
			else
				{rotatedBits[i] = bits[i + s].clone();}
		}
		return WireArray::new(rotatedBits);
	}
	
	

	pub fn shiftLeft(&self,numBits:usize , s:usize , desc:Vec<String>)->WireArray {
let bits = self.adjustLength( self.array.clone(), numBits);
let mut shiftedBits = vec![WireType::default();numBits as usize];
		for i in 0..numBits as usize {
			if i < s as usize
				{shiftedBits[i] = self.generator().zeroWire;}
			else
				{shiftedBits[i] = bits[i - s as usize].clone();}
		}
		return WireArray::new(shiftedBits);
	}
	
	pub fn shiftRight(&self,numBits:usize , s:usize , desc:Vec<String>)->WireArray {
let bits = self.adjustLength(self.array.clone(), numBits);
let mut shiftedBits = vec![WireType::default();numBits];
		for i in 0..numBits as usize {
			if i >= numBits - s as  usize
				{shiftedBits[i] = self.generator().zeroWire;}
			else
				{shiftedBits[i] = bits[i + s as usize].clone();}
		}
		return WireArray::new(shiftedBits);
	}
		
	pub fn packBitsIntoWords(&self,wordBitwidth:i32 , desc:Vec<String>)-> Vec<Option<WireType>> {
let numWords = (self.array.len() as f64*1.0/wordBitwidth as f64).ceil() as usize;
let padded = self.adjustLength( self.array, wordBitwidth*numWords);
let result = vec![WireType::default();numWords];
		for i in  0.. numWords{
			result[i] = WireArray::new(padded[i*wordBitwidth as usize.. (i+1)*wordBitwidth as usize].to_vec()).packAsBits(vec![]);
		}
		return result;
	}
	
	pub fn packWordsIntoLargerWords(&self,wordBitwidth:i32 , numWordsPerLargerWord:i32 , desc:Vec<String>)-> Vec<Option<WireType>> {
let numLargerWords = (self.array.len() as f64*1.0/numWordsPerLargerWord as f64).ceil() as usize;
let result = vec![self.generator().zeroWire.clone();numLargerWords];
		for i in  0.. self.array.len(){
let subIndex = i % numWordsPerLargerWord as usize;
			result[i/numWordsPerLargerWord as usize] = result[i/numWordsPerLargerWord as usize].clone().add(self.array[i].clone().unwrap()
					.mul(BigInteger::from(2).pow(subIndex as u32*wordBitwidth as  u32)));
 		}
		return result;
		
	}

	pub fn getBits(&self,bitwidth:i32 , desc:Vec<String>) ->WireArray{
let bits = vec![WireType::default();bitwidth as usize * self.array.len()];
let idx = 0;
		for i in 0..self.array.len() {
let tmp = self.array[i].getBitWires(bitwidth, desc).asArray();
			for j in 0..bitwidth {
				bits[idx] = tmp[j];
                idx+=1;
			}
		}
		return WireArray::new(bits);
	}
	
}
