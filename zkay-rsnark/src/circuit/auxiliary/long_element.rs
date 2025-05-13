
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use std::ops::{Add, Div, Mul, Neg, Sub};
use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::eval::instruction;
use circuit::structure::circuit_generator;
use circuit::structure::constant_wire;
use circuit::structure::wire;
use circuit::structure::wire_array;

/**
 * An auxiliary class that handles the operations of long integers, such as the
 * ones used in RSA operations. It applies some of the long integer
 * optimizations from xjsnark (to appear). This is a preliminary version. More
 * Other features and detailed tests will be added in the future.
 *
 * Usage examples exist in the RSA examples gadgets.
 */
pub type BigInteger=String;
struct  LongElement {
	 array:Vec<Wire>;
	 currentBitwidth:Vec<i32>;
	 currentMaxValues:Vec<BigInteger> ;
	 self.generator:CircuitGenerator;
	bits:Option<WireArray> ;
}
impl LongElement{
	// Should be declared as final, but left non-for testing purposes.
	// Don't change in the middle of circuit generation.
	// This represents the size of smaller chunks used to represent long
	// elements
	pub const CHUNK_BITWIDTH:i32 = 120;

	pub fn new(w:Wire, currentBitwidth:i32) ->Self{
        Self{array : vec![w],
		 currentBitwidth : vec![ currentBitwidth] ,
		 currentMaxValues: vec![Util::computeMaxValue(currentBitwidth)] ,
		 self.generator : CircuitGenerator::getActiveCircuitGenerator(),
            bits:None
        }
	}

	pub fn new_with_array( bits:WireArray)->Self {

		let (array,currentMaxValues,currentBitwidth)=if CHUNK_BITWIDTH >= bits.size() {
			(vec![bits.packAsBits(bits.size())] ,
			 vec![ Util::computeMaxValue(bits.size())],
			vec![ bits.size()])
		} else {
			let  maxChunkVal = Util::computeMaxValue(CHUNK_BITWIDTH);
			let  mut maxLastChunkVal = maxChunkVal;
			let  size = bits.size();
			if size % CHUNK_BITWIDTH != 0 {
				bits = bits.adjustLength(size
						+ (CHUNK_BITWIDTH - size % CHUNK_BITWIDTH));
				maxLastChunkVal = Util::computeMaxValue(size % CHUNK_BITWIDTH);
			}
			let mut array = vec![Wire::default();bits.size() / CHUNK_BITWIDTH];
			let mut currentMaxValues = vec![BigInteger::default();array.len()];
			let mut currentBitwidth = vec![0;array.len()];

			for i in  0..array.len() {
				array[i] = WireArray::new(
						&bits[i * CHUNK_BITWIDTH.. (i + 1)
								* CHUNK_BITWIDTH]).packAsBits();
				if i == array.len() - 1 {
					currentMaxValues[i] = maxLastChunkVal;
					currentBitwidth[i] = maxLastChunkVal.bitLength();
				} else {
					currentMaxValues[i] = maxChunkVal;
					currentBitwidth[i] = maxChunkVal.bitLength();
				}
			}
            (array,currentMaxValues,currentBitwidth)
		};
		Self{array,currentMaxValues,currentBitwidth,
		self.generator : CircuitGenerator::getActiveCircuitGenerator(),
 bits,
        }
	}

	pub fn new(w:Wire,  currentMaxValue:BigInteger) ->Self{
		Self{ array : vec![ w],
		currentMaxValues : vec![ currentMaxValue ];
		currentBitwidth : vec![ currentMaxValue.bitLength() ];
		self.generator : CircuitGenerator::getActiveCircuitGenerator(),
        bits:None}
	}

	pub fn new( w:Vec<Wire>, currentBitwidth:Vec<i32>)->Self {
        let mut currentMaxValues = vec![0;w.len()];
		for i in  0..w.len() {
			currentMaxValues[i] = Util::computeMaxValue(currentBitwidth[i]);
		}
		{array:w,
		currentBitwidth,
		currentMaxValues,
		self.generator : CircuitGenerator::getActiveCircuitGenerator(),
            bits:None,
        }
	}

	pub fn makeOutput(&mut self,desc:Vec<String>) {
		for  w  in  getArray() {
			self.generator.makeOutput(w, desc);
		}
	}

	/**
	 * A long element representing a constant.
	 */
	pub fn new_with_int_array(chunks:Vec<BigInteger> )->Self {
	    let mut currentBitwidth = vec![0;chunks.len()];
		for  i in  0.. chunks.len() {
			currentBitwidth[i] = currentMaxValues[i].bitLength();
		}
        let self.generator = CircuitGenerator::getActiveCircuitGenerator();
		Self{array : self.generator.createConstantWireArray(chunks) ,
        currentMaxValues :chunks,
	currentBitwidth,
		self.generator,
        bits:None}
	}

	pub fn new_with_wire_array( w:Vec<Wire>, currentMaxValues:Vec<BigInteger>  ) ->Self{
        let mut currentBitwidth = vec![0;w.len()];
		for i in 0..w.len() {
			currentBitwidth[i] = currentMaxValues[i].bitLength();
		}
        Self{
		array : w,
		 currentMaxValues,
		currentBitwidth,
		self.generator : CircuitGenerator::getActiveCircuitGenerator(),
         bits:None,
        }
	}

	pub fn addOverflowCheck( o:LongElement)->bool {
		let  length = std::cmp::min(array.len(), o.array.len());
		let mut  overflow = false;
		for i in 0..length {
			let  max1 = if i < array.len() { currentMaxValues[i]}else
					 {BigInteger::ZERO};
			let  max2 = if i < o.array.len()  {o.currentMaxValues[i]}else
					 {BigInteger::ZERO};
			if max1+max2>=Config.get().unwrap().field_prime{
				overflow = true;
				break;
			}
		}
		 overflow
	}

	pub fn mulOverflowCheck(o:LongElement)->bool {
		let  length = array.len() + o.array.len() - 1;
		let mut  overflow = false;
		let newMaxValues =vec![BigInteger::ZERO;length];
		for i in 0.. array.len() {
			for j in 0.. o.array.len() {
				newMaxValues[i + j] = newMaxValues[i + j]
						.add(currentMaxValues[i]
								.multiply(o.currentMaxValues[j]));
			}
		}
		for i in 0..length {
			if newMaxValues[i]>=Config.get().unwrap().field_prime {
				overflow = true;
				break;
			}
		}
		overflow
	}

	fn isConstant()->bool {
		let mut  isConstant = true;
		if !array.is_empty() {
			for i in 0.. array.len() {
				isConstant &= matches!(array[i], ConstantWire);
			}
		}
		isConstant
	}

	pub fn getSize()->usize {
		 array.len()
	}

	pub fn align(totalNumChunks:usize)->Self {
		let mut  newArray = array[..totalNumChunks].to_vec();
		for  i in  0..newArray.len(){
			if newArray[i].is_empty(){
				newArray[i]= self.generator.getZeroWire();
			}
		}
		let mut  newMaxValues = vec![BigInteger.ZERO;totalNumChunks];
		currentMaxValues.copy_from_slice(&newMaxValues[..totalNumChunks.min(currentMaxValues.len())]);
		let  maxAlignedChunkValue = Util::computeMaxValue(CHUNK_BITWIDTH);

		for i in  0.. totalNumChunks {
			if newMaxValues[i].bitLength() > CHUNK_BITWIDTH {
				let mut  chunkBits = newArray[i].getBitWires(newMaxValues[i].bitLength())
						.asArray();
				newArray[i] = WireArray::new(&chunkBits[..CHUNK_BITWIDTH]).packAsBits();
				let mut  rem = WireArray::new(&chunkBits[CHUNK_BITWIDTH,newMaxValues[i].bitLength()] ).packAsBits();
				if i != totalNumChunks - 1 {
					newMaxValues[i + 1] = newMaxValues[i].shiftRight(
							CHUNK_BITWIDTH).add(newMaxValues[i + 1]);
					newArray[i + 1] = rem.add(newArray[i + 1]);
				}
				newMaxValues[i] = maxAlignedChunkValue;
			}
		}
		 LongElement::new(newArray, newMaxValues)
	}


	// This method extracts (some of) the bit wires corresponding to a long
	// element based on the totalBitwidth argument.
	// If totalBitwidth is -1, all bits are returned.
	// See restrictBitwidth for restricting the bitwidth of all the long element
	// chunks

	pub fn   getBits(totalBitwidth:i32)->WireArray {

		if bits.is_some() {
			return bits.adjustLength( if totalBitwidth == -1 { bits.len()}else {totalBitwidth});
		}
		if array.len() == 1 {
			bits = array[0].getBitWires(currentMaxValues[0].bitLength());
			return bits.adjustLength( if totalBitwidth == -1 { bits.size()}else {totalBitwidth});
		} 
			if totalBitwidth <= CHUNK_BITWIDTH && totalBitwidth >= 0 {
				let out = array[0].getBitWires(currentMaxValues[0]
						.bitLength());
				return out.adjustLength(totalBitwidth);
			} 
				
				let limit = totalBitwidth;
				BigInteger maxVal = getMaxVal(CHUNK_BITWIDTH);

				let  bitWires=if totalBitwidth != -1 {
					vec![self.generator.getZeroWire();totalBitwidth]
				} else {
					limit = maxVal.bitLength();
                    vec![self.generator.getZeroWire();maxVal.bitLength()]
				};

				let newLength =  (getMaxVal(CHUNK_BITWIDTH)
						.bitLength() * 1.0 / CHUNK_BITWIDTH).ceil();
				let mut  newArray = vec![ self.generator.getZeroWire();newLength];
				let newMaxValues = vec![BigInteger.ZERO;newLength];

                newMaxValues[0..currentMaxValues.len()].copy_from_slice(&currentMaxValues);
                newArray[0..array.len()].clone_from_slice(&array);
				let idx = 0;
				let chunkIndex = 0;
				while (idx < limit && chunkIndex < newLength) {
					let mut  alignedChunkBits;
					if newMaxValues[chunkIndex].bitLength() > CHUNK_BITWIDTH {

						Vec<Wire> chunkBits = newArray[chunkIndex].getBitWires(
								newMaxValues[chunkIndex].bitLength()).asArray();

						alignedChunkBits = chunkBits[..CHUNK_BITWIDTH].to_vec();
						let rem = WireArray::new(&chunkBits[
								CHUNK_BITWIDTH..
								newMaxValues[chunkIndex].bitLength()])
								.packAsBits();

						if chunkIndex != newArray.len() - 1 {
							newMaxValues[chunkIndex + 1] = newMaxValues[chunkIndex]
									.shiftRight(CHUNK_BITWIDTH).add(
											newMaxValues[chunkIndex + 1]);
							newArray[chunkIndex + 1] = rem
									.add(newArray[chunkIndex + 1]);
						}
					} else {
						alignedChunkBits = newArray[chunkIndex].getBitWires(
								CHUNK_BITWIDTH).asArray();
					}
                    bitWires[idx..std::cmp::min(alignedChunkBits.len(), limit - idx)].copy_from_slice(&alignedChunkBits);
					chunkIndex+=1;
					idx += alignedChunkBits.len();
				}
				let  out = WireArray::new(bitWires);
				if limit >= maxVal.bitLength(){
					bits = out.adjustLength(maxVal.bitLength());
				}
				 out
			

		

	}

	pub fn  getMaxVal(bitwidth:i32)->BigInteger {
		 Util::group(currentMaxValues, bitwidth)
	}

	fn  multiplyPolys(Vec<BigInteger>  aiVals, Vec<BigInteger>  biVals)->Vec<BigInteger>  {

		let mut  solution = vec![BigInteger.ZERO;aiVals.len() + biVals.len()
				- 1];
		
		for i in 0..aiVals.len() {
			for j in 0..biVals.len() {
				solution[i + j] = solution[i + j].add(
						aiVals[i].multiply(biVals[j])).mod(Config.get().unwrap().field_prime);
			}
		}
		 solution

	}

	pub fn muxBit(LongElement other, w:Wire)->Self {

		let length = std::cmp::max(array.len(), other.array.len());
		let mut  newArray = vec![self.generator.getZeroWire();length];
		Vec<BigInteger> newMaxValues = vec![BigInteger.ZERO;length];
		for i in 0..length {

			let  b1 = if i < array.len()  {currentMaxValues[i]}
					else {BigInteger.ZERO};
			let  b2 = if i < other.array.len()  {other.currentMaxValues[i]}
					else {BigInteger.ZERO};
			newMaxValues[i]=  if b1.compareTo(b2) == 1  { b1 }else { b2};

			let  w1=  if i < array.len()  { array[i] }else { self.generator.getZeroWire()};
			let  w2 =  if i < other.array.len()  { other.array[i] }else { self.generator
					.getZeroWire()};
			newArray[i] = w1.add(w.mul(w2.sub(w1)));
			if matches!(newArray[i],  ConstantWire) {
				newMaxValues[i] = ((ConstantWire) newArray[i]).getConstant();
			}

		}
		 LongElement::new(newArray, newMaxValues)
	}

	pub fn checkNonZero()-> Wire{
		let mut  wireNonZero = vec![self.generator.getZeroWire();array.len()];
		for i in 0..array.len() {
			wireNonZero[i] = array[i].checkNonZero();
		}
		 WireArray::new(wireNonZero).sumAllElements().checkNonZero()
	}

	pub fn getArray()->Vec<Wire> {
		 array.clone()
	}

	pub fn getCurrentBitwidth()->Vec<i32> {
		 currentBitwidth.clone()
	}

	pub fn getCurrentMaxValues()->Vec<BigInteger>  {
		 currentMaxValues.clone()
	}

	pub fn getBits()->WireArray {
		 bits.clone()
	}

	pub fn getConstant(bitwidth_per_chunk:i32)->Option<BigInteger> {
		let mut  constants = vec![BigInteger.ZERO;array.len()];
		for i in 0.. array.len() {
			if !matches!(array[i], ConstantWire)
				return None;
			else {
				constants[i] = ((ConstantWire) array[i]).getConstant();
			}
		}
		Util::group(constants, bitwidth_per_chunk)
	}

	// the equals java method to compare objects (this is NOT for circuit
	// equality check)
	pub fn equals(&self,v: LongElement)->bool {
		// if o == null || !(o instanceof LongElement) {
		// 	return false;
		// }
		// LongElement v = (LongElement) o;
		if v.array.len() != self.array.len() {
			return false;
		}
		// let mut  check = true;
		// for i in 0.. self.array.len() {
		// 	if !v.array[i].equals(array[i]) {
		// 		check = false;
		// 		break;
		// 	}
		// }
		// return check;
        self.array.iter().zip(&v.array).all(|(a,b)| a.equals(b))
	}

	// This asserts that the current bitwidth conditions are satisfied
	pub fn restrictBitwidth(&self) {
		if !isAligned() {
			println!("Warning [restrictBitwidth()]: Might want to align before checking bitwidth constraints");
			if Config.get().unwrap().printStackTraceAtWarnings {
				// Thread.dumpStack();
               println!("Thread.dumpStack()");
			}
		}
		for i in 0.. self.array.len() {
			self.array[i].restrictBitLength(currentBitwidth[i]);
		}
	}

	pub fn isAligned(&self)->bool {
		let mut  check = true;
		for i in 0.. array.len() {
			check &= self.currentBitwidth[i] <= CHUNK_BITWIDTH;
		}
		 check
	}

	pub fn assertEqualityNaive(a:LongElement) {

		WireArray bits1 = a.getBits(a.getMaxVal(CHUNK_BITWIDTH).bitLength());
		WireArray bits2 = getBits(getMaxVal(CHUNK_BITWIDTH).bitLength());
		LongElement v1 = LongElement::new(bits1);
		LongElement v2 = LongElement::new(bits2);
		for i in 0..v1.array.len() {
			self.generator.addEqualityAssertion(v1.array[i], v2.array[i]);
		}
	}

	// an improved equality assertion algorithm from xjsnark
	pub fn assertEquality(&self.e:LongElement) {

		let  (mut a1 ,mut a2)= (self.array.clone(),e.array.clone());
		let  (mut bounds1,mut bounds2) = (self.currentMaxValues.clone(),e.currentMaxValues.clone());

		let limit = std::cmp::max(a1.len(), a2.len());

		// padding
		if e.array.len() != limit {
			a2 = WireArray::new(a2).adjustLength(limit).asArray();
			bounds2 = vec![BigInteger.ZERO;limit];
			bounds2[..e.currentMaxValues.len()].copy_from_slice(&e.currentMaxValues);
		}
		if self.array.len() != limit {
			a1 = WireArray::new(a1).adjustLength(limit).asArray();
			bounds1 = vec![BigInteger.ZERO;limit];
			bounds1[..self.currentMaxValues.len()].copy_from_slice(&self.currentMaxValues);
		}

		// simple equality assertion cases
		if a1.len() == a2.len() && a1.len() == 1 {
			self.generator.addEqualityAssertion(a1[0], a2[0],
					"Equality assertion of long elements | case 1");
			return;
		} else if isAligned() && e.isAligned() {
			for i in 0..limit {
				self.generator.addEqualityAssertion(a1[i], a2[i],
						"Equality assertion of long elements | case 2 | index ".to_string()
								+ &i.to_string());
			}
			return;
		}

		// To make the equality check more efficient, group the chunks together
		// while ensuring that there are no overflows.

		let mut  group1 = vec![];
		let mut  group1_bounds =vec![];
		let mut  group2 = vec![];
		let mut  group2_bounds = vec![];

		// This array will store how many chunks were grouped together for every
		// wire in group1 or group2
		// The grouping needs to happen in the same way for the two operands, so
		// it's one steps array
		let mut  steps = vec![];

		let  shift = BigInteger::from(2).pow(CHUNK_BITWIDTH);
		let i = 0;
		while (i < limit) {
			let step = 1;
			let w1 = a1[i];
			let w2 = a2[i];
			let b1 = bounds1[i];
			let b2 = bounds2[i];
			while (i + step <= limit - 1) {
				let delta = shift.pow(step);
				if b1.add(bounds1[i + step].multiply(delta)).bitLength() < Config.get().unwrap(.LOG2_FIELD_PRIME - 2
						&& b2.add(bounds2[i + step].multiply(delta))
								.bitLength() < Config.get().unwrap().LOG2_FIELD_PRIME - 2) {
					w1 = w1.add(a1[i + step].mul(delta));
					w2 = w2.add(a2[i + step].mul(delta));
					b1 = b1.add(bounds1[i + step].multiply(delta));
					b2 = b2.add(bounds2[i + step].multiply(delta));
					step+=1;
				} else {
					break;
				}
			}
			group1.add(w1);
			group1_bounds.add(b1);
			group2.add(w2);
			group2_bounds.add(b2);
			steps.add(step);
			i += step;
		}

		let numOfGroupedChunks = group1.size();

		// After grouping, subtraction will be needed to compare the grouped
		// chunks and propagate carries.
		// To avoid dealing with cases where the first operand in the
		// subtraction is less than the second operand,
		// we introduce an auxiliary constant computed based on the bounds of
		// the second operand. The chunks
		// of this auxConstant will be added to the chunks of the first operand
		// before subtraction.

		let auxConstant = BigInteger.ZERO;
		let auxConstantChunks = vec![BigInteger.ZERO;numOfGroupedChunks];

		let mut carries = self.generator
				.createProverWitnessWireArray(numOfGroupedChunks - 1);
		let mut carriesBitwidthBounds = vec![0;carries.len()];

		// computing the auxConstantChunks, and the total auxConstant
		let mut accumStep = 0;
		for  j in  0.. auxConstantChunks.len() - 1 {
			auxConstantChunks[j] = BigInteger.valueOf(2).pow(
					group2_bounds.get(j).bitLength());
			auxConstant = auxConstant.add(auxConstantChunks[j].multiply(shift
					.pow(accumStep)));
			accumStep += steps.get(j);
			carriesBitwidthBounds[j] = std::cmp::max(auxConstantChunks[j]
					.bitLength(), group1_bounds.get(j).bitLength())
					- steps.get(j) * CHUNK_BITWIDTH + 1;
		}

		// since the two elements should be equal, we should not need any aux
		// chunk in the last step
		auxConstantChunks[auxConstantChunks.len() - 1] = BigInteger.ZERO;

		// Note: the previous auxConstantChunks are not aligned. We compute an
		// aligned version as follows.

		// First split the aux constant into small chunks based on
		// CHUNK_BITWIDTH
		let  alignedAuxConstantSmallChunks = Util::split(auxConstant,
				CHUNK_BITWIDTH);

		// second, group the small aux chunks based on the steps array computed
		// earlier to get the alignedAuxConstantChunks
		// alignedAuxConstantChunks is the grouped version of
		// alignedAuxConstantSmallChunks

		let mut alignedAuxConstantChunks = vec![BigInteger.ZERO;numOfGroupedChunks];

		let idx = 0;
		'loop1: for j in 0..numOfGroupedChunks {
			for k in 0..steps.get(j) {
				alignedAuxConstantChunks[j] = alignedAuxConstantChunks[j]
						.add(alignedAuxConstantSmallChunks[idx].multiply(shift
								.pow(k)));
				idx+=1;
				if idx == alignedAuxConstantSmallChunks.len() {
					break 'loop1;
				}
			}
		}
		if idx != alignedAuxConstantSmallChunks.len() {
			if idx == alignedAuxConstantSmallChunks.len() - 1 {
				alignedAuxConstantChunks[numOfGroupedChunks - 1] = alignedAuxConstantChunks[numOfGroupedChunks - 1]
						.add(alignedAuxConstantSmallChunks[idx].multiply(shift
								.pow(steps.get(numOfGroupedChunks - 1))));
			} else {
				panic!("Case not expected. Please report.");
			}
		}

		// specify how the values of carries are obtained during runtime
		self.generator.specifyProverWitnessComputation(& {
            struct Prover;
            impl Instruction  for Prover
			{
			 fn evaluate(evaluator:CircuitEvaluator) {
				let mut  prevCarry = BigInteger.ZERO;
				for i in 0..carries.len() {
					let  a = evaluator.getWireValue(group1.get(i));
					let b = evaluator.getWireValue(group2.get(i));
					let mut carryValue = auxConstantChunks[i].add(a)
							.subtract(b).subtract(alignedAuxConstantChunks[i])
							.add(prevCarry);
					carryValue = carryValue.shiftRight(steps.get(i)
							* CHUNK_BITWIDTH);
					evaluator.setWireValue(carries[i], carryValue);
					prevCarry = carryValue;
				}
			}}
            Prover
		});

		// We must make sure that the carries values are bounded.

		for j in 0..carries.len() {
			// carries[j].getBitWires(carriesBitwidthBounds[j]);
			carries[j].restrictBitLength(carriesBitwidthBounds[j]);

			// Note: in this context restrictBitLength and getBitWires will be
			// the same, but it's safer to use restrictBitLength
			// when enforcing constraints.
		}

		// Now apply the main constraints

		let mut  prevCarry = self.generator.getZeroWire();
		let mut  prevBound = BigInteger.ZERO;

		// recall carries.len() = numOfGroupedChunks - 1
		for  j in 0.. carries.len() + 1 {
			let  auxConstantChunkWire = self.generator
					.createConstantWire(auxConstantChunks[j]);
			let alignedAuxConstantChunkWire = self.generator
					.createConstantWire(alignedAuxConstantChunks[j]);

			// the last carry value must be zero
			let currentCarry = if j == carries.len()  {self.generator.getZeroWire()}
					else {carries[j]};

			// overflow check for safety
			if auxConstantChunks[j].add(group1_bounds.get(j)).add(prevBound
					.compareTo(Config.get().unwrap().field_prime) >= 0) {
				println!("Overflow possibility @ ForceEqual()");
			}

			let w1 = auxConstantChunkWire
					.add(group1.get(j).sub(group2.get(j))).add(prevCarry);
			let w2 = alignedAuxConstantChunkWire.add(currentCarry.mul(shift
					.pow(steps.get(j))));

			// enforce w1 = w2
			// note: in the last iteration, both auxConstantChunkWire and
			// currentCarry will be zero,
			// i.e., there will be no more values to be checked.

			self.generator
					.addEqualityAssertion(w1, w2,
							"Equality assertion of long elements | case 3 | index "
									+ j);

			prevCarry = currentCarry;
			if j != carries.len() {
				prevBound = Util::computeMaxValue(carriesBitwidthBounds[j]);
			}

		}
	}

	// applies an improved technique to assert comparison
	pub fn assertLessThan(&self,other:LongElement) {

		// first verify that both elements are aligned
		assert!(self.isAligned() && other.isAligned(),"input chunks are not aligned");

		let  a1 = self.getArray();
		let  a2 = other.getArray();
		let length = std::cmp::max(a1.len(), a2.len());
		let  paddedA1 = Util::padWireArray(a1, length,
				self.generator.getZeroWire());
		let paddedA2 = Util::padWireArray(a2, length,
				self.generator.getZeroWire());

		/*
		 * Instead of doing the comparison naively (which will involve all the
		 * bits) let the prover help us by pointing to the first chunk in the
		 * other element that is more than the corresponding chunk in this
		 * element.
		 */
		let  helperBits = self.generator.createProverWitnessWireArray(length);
		// set the value of the helperBits outside the circuits

		self.generator.specifyProverWitnessComputation(&{
			 struct Prover;
            impl Instruction  for Prover
			{
			pub fn evaluate( evaluator:CircuitEvaluator) {
				let  mut found = false;
				for  i in  (0..length ).rev() {
					let  v1 = evaluator.getWireValue(paddedA1[i]);
					let v2 = evaluator.getWireValue(paddedA2[i]);

					let  check = v2.compareTo(v1) > 0 && !found;
					evaluator.setWireValue(helperBits[i],
                            if check  { BigInteger.ONE }else { BigInteger.ZERO});
					if check
						{found = true;}
				}
			}
            }
        Prover
		});

		// verify constraints about helper bits.
		for  w in  helperBits {
			self.generator.addBinaryAssertion(w);
		}
		// Only one bit should be set.
		self.generator.addOneAssertion(WireArray::new(helperBits).sumAllElements());

		// verify "the greater than condition" for the specified chunk
		let chunk1 = self.generator.getZeroWire();
		let chunk2 = self.generator.getZeroWire();

		for i in 0..helperBits.len() {
			chunk1 = chunk1.add(paddedA1[i].mul(helperBits[i]));
			chunk2 = chunk2.add(paddedA2[i].mul(helperBits[i]));
		}
		self.generator.addOneAssertion(chunk1.isLessThan(chunk2,
				LongElement.CHUNK_BITWIDTH));

		// check that the other more significant chunks are equal
		let mut  helperBits2 = vec![self.generator.getZeroWire();helperBits.len()];
		for i in 1..helperBits.len() {
			helperBits2[i] = helperBits2[i - 1].add(helperBits[i - 1]);
//			self.generator.addZeroAssertion(helperBits2[i].mul(paddedA1[i]
//					.sub(paddedA2[i])));
			self.generator.addAssertion(helperBits2[i], paddedA1[i].sub(paddedA2[i]), self.generator.getZeroWire());
		}

		// no checks needed for the less significant chunks
	}

}




impl Add<u64> for LongElement {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
         self.add(BigInteger.valueOf(value))
    }
}

impl Add<BigInteger> for LongElement {
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
       	if rhs.signum() == 0{ return self;}
		if rhs.signum() < 0{ 
            return self.sub(rhs.negate());
        }
		self.add(LongElement::new(Util::split(rhs, CHUNK_BITWIDTH)))
    }
}

impl Add for LongElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if addOverflowCheck(o) {
			println!("Warning: Addition overflow could happen");
		}

		let length = std::cmp::max(self.array.len(), o.array.len());
		let  w1 = WireArray::new(self.array).adjustLength(length).asArray();
		let w2 = WireArray::new(o.array).adjustLength(length).asArray();
		let result = vec![self.generator.getZeroWire();length];
		let newMaxValues = vec![BigInteger.ZERO;length];
		for i in 0..length {
			result[i] = w1[i].add(w2[i]);
			let max1=  if i < self.array.len()  { currentMaxValues[i] }else { BigInteger.ZERO};
			let max2=  if i < o.array.len()  { o.currentMaxValues[i] }else { BigInteger.ZERO};

			newMaxValues[i] = max1.add(max2);
		}
		LongElement::new(result, newMaxValues)
    }
}
impl Sub<u64> for LongElement {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::sub(self,BigInteger.valueOf(value))
    }
}
impl Sub<BigInteger> for LongElement {
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        if rhs.signum() == 0 {return self;}
		if rhs.signum() < 0) {return self.add(rhs.negate();}
		self.sub(LongElement::new(Util::split(rhs, CHUNK_BITWIDTH)))
    }
}

impl Sub for LongElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
			assert!(isAligned() && other.isAligned(),"Subtraction arguments must be properly aligned");

		let  result = self.generator.createLongElementProverWitness(getMaxVal(CHUNK_BITWIDTH).bitLength());

		self.generator.specifyProverWitnessComputation(& {
			 struct Prover;
            impl Instruction  for Prover
			{
			pub fn evaluate(evaluator:CircuitEvaluator) {
				BigInteger myValue = evaluator.getWireValue(LongElement.this, CHUNK_BITWIDTH);
				BigInteger otherValue = evaluator.getWireValue(other, CHUNK_BITWIDTH);
				BigInteger resultValue = myValue.subtract(otherValue);
				if resultValue.signum() < 0 {
					assert!("Result of subtraction is negative!");
				}
				evaluator.setWireValue(result, resultValue, CHUNK_BITWIDTH);
			}
            }
            Prover
		});

		result.restrictBitwidth();
		self.assertEquality(result.add(other));
		result
    }
}

impl Mul for LongElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
       /**
	 * Implements the improved long integer multiplication from xjsnark
	 */
		if mulOverflowCheck(o) {
			println!("Warning: Mul overflow could happen");

		}
		let length = self.array.len() + o.array.len() - 1;
		let mut  result;

		// check if we can just apply the simple non-costly multiplication
		if o.array.len() == 1 || self.array.len() == 1 || self.isConstant(
				|| o.isConstant()) {
			result = vec![self.generator.getZeroWire();length];

			// O(n*m) multiplication. Fine to apply if any of the operands has
			// dim 1
			// or any of them is constant
			for i in 0.. self.array.len() {
				for j in 0.. o.array.len() {
					result[i + j] = result[i + j].add(self.array[i].mul(o.array[j]));
				}
			}
		} else {

			// implement the optimization

			result = self.generator.createProverWitnessWireArray(length);

			// for safety
			let  array1 = self..array.clone();
			let  array2 = o.array.clone();
			self.generator.specifyProverWitnessComputation(& {
				 struct Prover;
            impl Instruction  for Prover
			{
				pub fn evaluate(evaluator:CircuitEvaluator) {
					let  a = evaluator.getWiresValues(array1);
					let  b = evaluator.getWiresValues(array2);
					let  resultVals = multiplyPolys(a, b);
					evaluator.setWireValue(result, resultVals);
				}
            }
            Prover
			});

			for k in 0..length {
				let  constant = BigInteger::new((k + 1) + "");
				let coeff = BigInteger.ONE;

				let vector1 = vec![self.generator.getZeroWire();array.len()];
				let vector2 = vec![self.generator.getZeroWire();o.array.len()];
				let vector3 = vec![self.generator.getZeroWire();length];
				for i in 0..length {
					if i < array.len() {
						vector1[i] = array[i].mul(coeff);
					}
					if i < o.array.len() {
						vector2[i] = o.array[i].mul(coeff);
					}
					vector3[i] = result[i].mul(coeff);
					coeff = Util::mod(coeff.multiply(constant), Config.get().unwrap().field_prime);
				}

				let v1 = WireArray::new(vector1).sumAllElements();
				let v2 = WireArray::new(vector2).sumAllElements();
				let v3 = WireArray::new(vector3).sumAllElements();
				self.generator.addAssertion(v1, v2, v3);
			}
		}

		let mut  newMaxValues = vec![BigInteger.ZERO;length];
		for i in 0.. self.array.len() {
			for j in 0.. o.array.len() {
				newMaxValues[i + j] = newMaxValues[i + j]
						.add(currentMaxValues[i]
								.multiply(o.currentMaxValues[j]));
			}
		}
		 LongElement::new(result, newMaxValues)
	
    }
}
