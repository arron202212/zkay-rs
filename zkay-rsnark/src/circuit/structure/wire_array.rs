

use util::util;
use circuit::eval::instruction;
use circuit::operations::primitive::add_basic_op;
use circuit::operations::primitive::pack_basic_op;

pub struct WireArray {
	array: Vec<Wire> ,
	  generator:CircuitGenerator,
}
	pub fn newWireArrayWithI32(n:i32)->WireArray {
		newWireArrayWithNAndGenerator(n, CircuitGenerator.getActiveCircuitGenerator())
	}
	
	pub fn newWireArrayWithNAndGenerator(n:i32, self.generator:CircuitGenerator)->WireArray {
        WireArray::new(vec![Wire::default();n as usize],self.generator)
	}
	
	pub fn newWireArrayWithArray(wireArray:Vec<Wire>)->WireArray {
		WireArray::new(wireArray, CircuitGenerator.getActiveCircuitGenerator())
	}

impl WireArray{
	pub fn new(wireArray:Vec<Wire>, self.generator:CircuitGenerator)->Self {
        Self{self.array : wireArray,self.generator}
	}
	
	pub fn get(i:i32)->Wire{
		return self.array[i];
	}
	
	pub fn set(&mut self,i:i32, w:Wire){
		self.self.array[i] = w;
	}
	
	pub fn size(&self)->usize{
		return self.self.array.len();
	}
	
	pub fn asArray(&self)-> Vec<Wire>{
		return self.self.array.clone();
	}
	
	pub fn mulWireArray(v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = adjustLength( self.array, desiredLength);
let ws2 = adjustLength( v.self.array, desiredLength);
let out = vec![Wire::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].mul(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	
	pub fn sumAllElements(desc:Vec<String>)->Wire {
let allConstant = true;
let sum = BigInteger.ZERO;
		for w in  &self.self.array {
			if w.ConstantWire().is_none() {
				allConstant = false;
				break;
			} else {
				sum = sum.add( w.getConstant());
			}
		}
		if !allConstant {

			let output = LinearCombinationWire::new(self.generator.currentWireId);
            self.generator.currentWireId+=1;
let op = AddBasicOp::new(self.array, output, desc);
//			self.generator.addToEvaluationQueue(op);
let cachedOutputs = self.generator.addToEvaluationQueue(op);
			return if let Some(cachedOutputs) =cachedOutputs{
				self.self.generator.currentWireId-=1;
				 cachedOutputs[0].clone()
			}	else{
             output}
		}

		self.generator.createConstantWire(sum, desc)
	}
	
	
	pub fn addWireArray(v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = adjustLength(self.array, desiredLength);
let ws2 = adjustLength( v.self.array, desiredLength);
let out = vec![Wire::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].add(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArray(v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = adjustLength(self.array, desiredLength);
let ws2 = adjustLength(v.self.array, desiredLength);
let out = vec![Wire::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].xor(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn xorWireArray(v:WireArray , desc:Vec<String>)->WireArray {
		assert!(self.self.size() == v.self.size());
let ws1 = self.array;
let ws2 = v.self.array;
		
let out = vec![Wire::default();self.size()];
		for i in 0..out.len() {
			out[i] = ws1[i].xor(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn andWireArray(v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = adjustLength( self.array, desiredLength);
let ws2 = adjustLength( v.self.array, desiredLength);
let out = vec![Wire::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].mul(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	pub fn orWireArray(v:WireArray , desiredLength:i32 , desc:Vec<String>)->WireArray {
let ws1 = adjustLength(self.array, desiredLength);
let ws2 = adjustLength(v.self.array, desiredLength);
let out = vec![Wire::default();desiredLength];
		for i in 0..out.len() {
			out[i] = ws1[i].or(ws2[i], desc);
		}
		return WireArray::new(out);
	}
	
	
	
	pub fn invAsBits(desiredBitWidth:i32 , desc:Vec<String>) ->WireArray{
let out = vec![Wire::default();desiredBitWidth];
		for i in  0.. desiredBitWidth{
			if i < self.array.len(){
				out[i] = self.array[i].invAsBit(desc);
			}
			else{
				out[i] = self.generator.oneWire;
			}
		}
		return WireArray::new(out);
	}	
	
	
	fn adjustLength(ws:Vec<Wire> , desiredLength:i32 )->Vec<Wire> {
		if ws.len() == desiredLength{
			return ws;
		}
        let mut newWs = vec![Wire::default();desiredLength];
		newWs[0, std::cmp::min(ws.len(), desiredLength)].clone_from_slice(&ws);
		if ws.len() < desiredLength {
			for i in  ws.len().. desiredLength{
				newWs[i] = self.generator.zeroWire;
			}
		}
		return newWs;
	}
	
	pub fn adjustLength(desiredLength:i32 ) ->WireArray{
		if self.array.len() == desiredLength{
			return this;
		}
let newWs = vec![Wire::default();desiredLength];
		newWs[.. std::cmp::min(self.array.len(), desiredLength)].clone_from_slice(&self.array);
		if self.array.len() < desiredLength {
			for i in  self.array.len().. desiredLength{
				newWs[i] = self.generator.zeroWire;
			}
		}
		return WireArray::new(newWs);
	}
	
	
	
	pub fn packAsBits(n:i32, desc:Vec<String>)->Wire {
		return packAsBits(0, n, desc);
	}
	
	pub fn packAsBits(desc:Vec<String>) ->Wire{
		return packAsBits(self.array.len(), desc);
	}
	
	 fn  checkIfConstantBits(desc:Vec<String>)->Option<BigInteger>{
let mut allConstant = true;
let sum = BigInteger.ZERO;
		for i in  0.. self.array.len(){
			let w = self.array[i];
			if let Some(cw)=w.ConstantWire(){
                let v = cw.constant;
				if v.equals(BigInteger.ONE){
					sum = sum.add(v.shiftLeft(i));
				}
				else if !v.equals(BigInteger.ZERO){
					println!("Warning, one of the bit wires is constant but not binary : {}" , Util::getDesc(desc));					
				}
				
			}
			else{
				allConstant = false;
			}
		}
		 allConstant.then_some(sum)
	}

	pub fn packAsBits(from:i32 , to:i32 , desc:Vec<String>)->Wire {
		assert!(from <= to && to <= self.array.len(),"Invalid bounds: from > to");
		
let bits =self.array[from.. to].to_vec();
let allConstant = true;
let sum = BigInteger.ZERO;
		for i in  0.. bits.len(){
			let w = bits[i];
			if let Some(cw)=w.ConstantWire(){
let v = cw.constant;
				if v.equals(BigInteger.ONE){
					sum = sum.add(v.shiftLeft(i));
				}
				else {
					assert!(v.equals(BigInteger.ZERO),"Trying to pack non-binary constant bits : {}" , Util::getDesc(desc));					
				}
				
			}
			else{
				allConstant = false;
			}
		}
		if !allConstant{
let out = LinearCombinationWire::new(self.generator.currentWireId+=1);
			out.setBits(WireArray::new(bits));
let op = PackBasicOp::new(bits, out, desc);
let cachedOutputs = self.generator.addToEvaluationQueue(op);
			return if let Some(cachedOutputs) =cachedOutputs{
                self.self.generator.currentWireId-=1;         
				 cachedOutputs[0].clone()
            else
				 {out}	
			}
			

		} 
			 self.generator.createConstantWire(sum, desc)

		
	}
	
	
	pub fn rotateLeft(numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
		Vec<Wire> bits = adjustLength(self.array, numBits);
let mut rotatedBits = vec![Wire::default();numBits];
		for i in 0..numBits {
			if i < s
				{rotatedBits[i] = bits[i + (numBits - s)].clone();}
			else
				{rotatedBits[i] = bits[i - s].clone();}
		}
		return WireArray::new(rotatedBits);
	}
	
	pub fn rotateRight(numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = adjustLength(self.array, numBits);
let rotatedBits = vec![Wire::default();numBits];
		for i in 0..numBits {
			if i >= numBits - s
				{rotatedBits[i] = bits[i - (numBits - s)];}
			else
				{rotatedBits[i] = bits[i + s];}
		}
		return WireArray::new(rotatedBits);
	}
	
	

	pub fn shiftLeft(numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = adjustLength( self.array, numBits);
let shiftedBits = vec![Wire::default();numBits];
		for i in 0..numBits {
			if i < s
				{shiftedBits[i] = self.generator.zeroWire;}
			else
				{shiftedBits[i] = bits[i - s];}
		}
		return WireArray::new(shiftedBits);
	}
	
	pub fn shiftRight(numBits:i32 , s:i32 , desc:Vec<String>)->WireArray {
let bits = adjustLength(self.array, numBits);
let shiftedBits = vec![Wire::default();numBits];
		for i in 0..numBits {
			if i >= numBits - s
				{shiftedBits[i] = self.generator.zeroWire;}
			else
				{shiftedBits[i] = bits[i + s];}
		}
		return WireArray::new(shiftedBits);
	}
		
	pub fn packBitsIntoWords(wordBitwidth:i32 , desc:Vec<String>)-> Vec<Wire> {
let numWords = (self.array.len() as f64*1.0/wordBitwidth as f64).ceil() as i32;
let padded = adjustLength( self.array, wordBitwidth*numWords);
let result = vec![Wire::default();numWords];
		for i in  0.. numWords{
			result[i] = WireArray::new(Arrays.copyOfRange(padded, i*wordBitwidth, (i+1)*wordBitwidth)).packAsBits();
		}
		return result;
	}
	
	pub fn packWordsIntoLargerWords(wordBitwidth:i32 , numWordsPerLargerWord:i32 , desc:Vec<String>)-> Vec<Wire> {
let numLargerWords = (i32)Math.ceil(self.array.len()*1.0/numWordsPerLargerWord);
let result = vec![Wire::default();numLargerWords];
		Arrays.fill(result, self.generator.zeroWire);
		for i in  0.. self.array.len(){
let subIndex = i % numWordsPerLargerWord;
			result[i/numWordsPerLargerWord] = result[i/numWordsPerLargerWord].add(self.array[i]
					.mul(BigInteger::new("2").pow(subIndex*wordBitwidth)));
 		}
		return result;
		
	}

	pub fn getBits(bitwidth:i32 , desc:Vec<String>) ->WireArray{
let bits = vec![Wire::default();bitwidth * self.array.len()];
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
