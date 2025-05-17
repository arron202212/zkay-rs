#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::{config::config::Configs,eval::{circuit_evaluator::CircuitEvaluator,instruction::Instruction},structure::{circuit_generator::CircuitGenerator,constant_wire,wire::Wire,wire_array::WireArray}};
use crate::circuit::structure::wire_type::WireType;
use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::util::util::{Util,BigInteger};

/**
 * An auxiliary class that handles the operations of long integers, such as the
 * ones used in RSA operations. It applies some of the long integer
 * optimizations from xjsnark (to appear). This is a preliminary version. More
 * Other features and detailed tests will be added in the future.
 *
 * Usage examples exist in the RSA examples gadgets.
 */
// pub type BigInteger = String;
 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct LongElement {
    array: Vec<WireType>,
    currentBitwidth: Vec<i32>,
    currentMaxValues: Vec<BigInteger>,

    bits: Option<WireArray>,
}
impl LongElement {
    // Should be declared as final, but left non-for testing purposes.
    // Don't change in the middle of circuit generation.
    // This represents the size of smaller chunks used to represent long
    // elements
    pub const CHUNK_BITWIDTH: i32 = 120;

    pub fn new(w: WireType, currentBitwidth: i32) -> Self {
        Self {
            array: vec![w],
            currentBitwidth: vec![currentBitwidth],
            currentMaxValues: vec![Util::computeMaxValue(currentBitwidth)],
            bits: None,
        }
    }

    pub fn new_with_array(bits: WireArray) -> Self {
        let (array, currentMaxValues, currentBitwidth) = if Self::CHUNK_BITWIDTH >= bits.size() as i32 {
            (
                vec![bits.packAsBits(bits.size() as i32)],
                vec![Util::computeMaxValue(bits.size())],
                vec![bits.size()],
            )
        } else {
            let maxChunkVal = Util::computeMaxValue(Self::CHUNK_BITWIDTH);
            let mut maxLastChunkVal = maxChunkVal;
            let size = bits.size();
            if size % Self::CHUNK_BITWIDTH != 0 {
                bits = bits.adjustLength(size + (Self::CHUNK_BITWIDTH - size % Self::CHUNK_BITWIDTH));
                maxLastChunkVal = Util::computeMaxValue(size % Self::CHUNK_BITWIDTH);
            }
            let mut array = vec![WireType::default(); bits.size() / Self::CHUNK_BITWIDTH];
            let mut currentMaxValues = vec![BigInteger::default(); array.len()];
            let mut currentBitwidth = vec![0; array.len()];

            for i in 0..array.len() {
                array[i] = WireArray::new(&bits[i * Self::CHUNK_BITWIDTH..(i + 1) * Self::CHUNK_BITWIDTH])
                    .packAsBits();
                if i == array.len() - 1 {
                    currentMaxValues[i] = maxLastChunkVal;
                    currentBitwidth[i] = maxLastChunkVal.bitLength();
                } else {
                    currentMaxValues[i] = maxChunkVal;
                    currentBitwidth[i] = maxChunkVal.bitLength();
                }
            }
            (array, currentMaxValues, currentBitwidth)
        };
        Self {
            array,
            currentMaxValues,
            currentBitwidth,
            bits,
        }
    }

    pub fn newb(w: WireType, currentMaxValue: BigInteger) -> Self {
        Self {
            array: vec![w],
            currentMaxValues: vec![currentMaxValue],
            currentBitwidth: vec![currentMaxValue.bitLength()],
            bits: None,
        }
    }

    pub fn newa(w: Vec<WireType>, currentBitwidth: Vec<i32>) -> Self {
        let mut currentMaxValues = vec![0; w.len()];
        for i in 0..w.len() {
            currentMaxValues[i] = Util::computeMaxValue(currentBitwidth[i]);
        }
        Self {
            array: w,
            currentBitwidth,
            currentMaxValues,
            bits: None,
        }
    }
    
    pub fn makeOutput(&mut self, desc: Vec<String>) {
        for w in getArray() {
            self.generator().makeOutput(w, desc);
        }
    }

    /**
     * A long element representing a constant.
     */
    pub fn new_with_int_array(chunks: Vec<BigInteger>) -> Self {
        let mut currentBitwidth = vec![0; chunks.len()];
        for i in 0..chunks.len() {
            currentBitwidth[i] = chunks[i].bitLength();
        }
        let generator = CircuitGenerator::getActiveCircuitGenerator();
        Self {
            array: generator.createConstantWireArray(chunks),
            currentMaxValues: chunks,
            currentBitwidth,
            generator,
            bits: None,
        }
    }

    pub fn new_with_wire_array(w: Vec<WireType>, currentMaxValues: Vec<BigInteger>) -> Self {
        let mut currentBitwidth = vec![0; w.len()];
        for i in 0..w.len() {
            currentBitwidth[i] = currentMaxValues[i].bitLength();
        }
        Self {
            array: w,
            currentMaxValues,
            currentBitwidth,
            bits: None,
        }
    }

    pub fn addOverflowCheck(&self,o: LongElement) -> bool {
        let length = std::cmp::min(self.array.len(), o.array.len());
        let mut overflow = false;
        for i in 0..length {
            let max1 = if i < self.array.len() {
                self.currentMaxValues[i]
            } else {
                BigInteger::ZERO
            };
            let max2 = if i < o.array.len() {
                o.currentMaxValues[i]
            } else {
                BigInteger::ZERO
            };
            if max1 + max2 >= Configs.get().unwrap().field_prime {
                overflow = true;
                break;
            }
        }
        overflow
    }

    pub fn mulOverflowCheck(&self,o: LongElement) -> bool {
        let length = self.array.len() + o.array.len() - 1;
        let mut overflow = false;
        let newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..o.array.len() {
                newMaxValues[i + j] =
                    newMaxValues[i + j].add(self.currentMaxValues[i].multiply(o.currentMaxValues[j]));
            }
        }
        for i in 0..length {
            if newMaxValues[i] >= Configs.get().unwrap().field_prime {
                overflow = true;
                break;
            }
        }
        overflow
    }

    fn isConstant(&self) -> bool {
        let mut isConstant = true;
        if !self.array.is_empty() {
            for i in 0..self.array.len() {
                isConstant &= matches!(self.array[i], ConstantWire);
            }
        }
        isConstant
    }

    pub fn getSize(&self) -> usize {
        self.array.len()
    }

    pub fn align(&self,totalNumChunks: usize) -> Self {
        let mut newArray = self.array[..totalNumChunks].to_vec();
        for i in 0..newArray.len() {
            if newArray[i].is_empty() {
                newArray[i] = self.generator().getZeroWire();
            }
        }
        let mut newMaxValues = vec![BigInteger::ZERO; totalNumChunks];
       newMaxValues[..totalNumChunks.min(self.currentMaxValues.len())] 
            .copy_from_slice(&self.currentMaxValues);
        let maxAlignedChunkValue = Util::computeMaxValue(Self::CHUNK_BITWIDTH);

        for i in 0..totalNumChunks {
            if newMaxValues[i].bitLength() > Self::CHUNK_BITWIDTH {
                let mut chunkBits = newArray[i]
                    .getBitWires(newMaxValues[i].bitLength())
                    .asArray();
                newArray[i] = WireArray::new(&chunkBits[..Self::CHUNK_BITWIDTH]).packAsBits();
                let mut rem =
                    WireArray::new(&chunkBits[Self::CHUNK_BITWIDTH..newMaxValues[i].bitLength()])
                        .packAsBits();
                if i != totalNumChunks - 1 {
                    newMaxValues[i + 1] = newMaxValues[i]
                        .shiftRight(Self::CHUNK_BITWIDTH)
                        .add(newMaxValues[i + 1]);
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

    pub fn getBitsi(&self,totalBitwidth: i32) -> WireArray {
        if self.bits.is_some() {
            return self.bits.adjustLength(if totalBitwidth == -1 {
                self.bits.len()
            } else {
                totalBitwidth
            });
        }
        if self.array.len() == 1 {
            self.bits = self.array[0].getBitWires(self.currentMaxValues[0].bitLength());
            return self.bits.adjustLength(if totalBitwidth == -1 {
                self.bits.size()
            } else {
                totalBitwidth
            });
        }
        if totalBitwidth <= Self::CHUNK_BITWIDTH && totalBitwidth >= 0 {
            let out = self.array[0].getBitWires(self.currentMaxValues[0].bitLength());
            return out.adjustLength(totalBitwidth);
        }

        let limit = totalBitwidth;
        let maxVal = getMaxVal(Self::CHUNK_BITWIDTH);

        let bitWires = if totalBitwidth != -1 {
            vec![self.generator().getZeroWire(); totalBitwidth]
        } else {
            limit = maxVal.bitLength();
            vec![self.generator().getZeroWire(); maxVal.bitLength()]
        };

        let newLength = (getMaxVal(Self::CHUNK_BITWIDTH).bitLength() * 1.0 / Self::CHUNK_BITWIDTH).ceil();
        let mut newArray = vec![self.generator().getZeroWire(); newLength];
        let newMaxValues = vec![BigInteger::ZERO; newLength];

        newMaxValues[0..self.currentMaxValues.len()].copy_from_slice(&self.currentMaxValues);
        newArray[0..self.array.len()].clone_from_slice(&self.array);
        let idx = 0;
        let chunkIndex = 0;
        while idx < limit && chunkIndex < newLength {
            let mut alignedChunkBits;
            if newMaxValues[chunkIndex].bitLength() > Self::CHUNK_BITWIDTH {
                let chunkBits = newArray[chunkIndex]
                    .getBitWires(newMaxValues[chunkIndex].bitLength())
                    .asArray();

                alignedChunkBits = chunkBits[..Self::CHUNK_BITWIDTH].to_vec();
                let rem = WireArray::new(
                    &chunkBits[Self::CHUNK_BITWIDTH..newMaxValues[chunkIndex].bitLength()],
                )
                .packAsBits();

                if chunkIndex != newArray.len() - 1 {
                    newMaxValues[chunkIndex + 1] = newMaxValues[chunkIndex]
                        .shiftRight(Self::CHUNK_BITWIDTH)
                        .add(newMaxValues[chunkIndex + 1]);
                    newArray[chunkIndex + 1] = rem.add(newArray[chunkIndex + 1]);
                }
            } else {
                alignedChunkBits = newArray[chunkIndex].getBitWires(Self::CHUNK_BITWIDTH).asArray();
            }
            bitWires[idx..std::cmp::min(alignedChunkBits.len(), limit - idx)]
                .copy_from_slice(&alignedChunkBits);
            chunkIndex += 1;
            idx += alignedChunkBits.len();
        }
        let out = WireArray::new(bitWires);
        if limit >= maxVal.bitLength() {
            self.bits = out.adjustLength(maxVal.bitLength());
        }
        out
    }

    pub fn getMaxVal(&self,bitwidth: i32) -> BigInteger {
        Util::group(self.currentMaxValues.clone(), bitwidth)
    }

    fn multiplyPolys(aiVals: Vec<BigInteger>, biVals: Vec<BigInteger>) -> Vec<BigInteger> {
        let mut solution = vec![BigInteger::ZERO; aiVals.len() + biVals.len() - 1];

        for i in 0..aiVals.len() {
            for j in 0..biVals.len() {
                solution[i + j] = solution[i + j]
                    .add(aiVals[i].multiply(biVals[j]))
                    .modulo(Configs.get().unwrap().field_prime);
            }
        }
        solution
    }

    pub fn muxBit(&self,other: LongElement, w: WireType) -> Self {
        let length = std::cmp::max(self.array.len(), other.array.len());
        let mut newArray = vec![self.generator().getZeroWire(); length];
        let newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..length {
            let b1 = if i < self.array.len() {
                self.currentMaxValues[i].clone()
            } else {
                BigInteger::ZERO
            };
            let b2 = if i < other.self.array.len() {
                other.currentMaxValues[i]
            } else {
                BigInteger::ZERO
            };
            newMaxValues[i] = if b1.compareTo(b2) == 1 { b1 } else { b2 };

            let w1 = if i < self.array.len() {
                self.array[i]
            } else {
                self.generator().getZeroWire()
            };
            let w2 = if i < other.self.array.len() {
                other.self.array[i]
            } else {
                self.generator().getZeroWire()
            };
            newArray[i] = w1.add(w.mul(w2.sub(w1)));
            if matches!(newArray[i], ConstantWire) {
                newMaxValues[i] = (newArray[i]).getConstant();
            }
        }
        LongElement::new(newArray, newMaxValues)
    }

    pub fn checkNonZero(&self) -> WireType {
        let mut wireNonZero = vec![self.generator().getZeroWire(); self.array.len()];
        for i in 0..self.array.len() {
            wireNonZero[i] = self.array[i].checkNonZero();
        }
        WireArray::new(wireNonZero).sumAllElements().checkNonZero()
    }

    pub fn getArray(&self) -> Vec<WireType> {
        self.array.clone()
    }

    pub fn getCurrentBitwidth(&self) -> Vec<i32> {
        self.currentBitwidth.clone()
    }

    pub fn getCurrentMaxValues(&self) -> Vec<BigInteger> {
        self.currentMaxValues.clone()
    }

    pub fn getBits(&self) -> WireArray {
        self.bits.clone()
    }

    pub fn getConstant(&self,bitwidth_per_chunk: i32) -> Option<BigInteger> {
        let mut constants = vec![BigInteger::ZERO; self.array.len()];
        for i in 0..self.array.len() {
            if !matches!(self.array[i], ConstantWire) {
                return None;
            } else {
                constants[i] = (self.array[i]).getConstant();
            }
        }
        Util::group(constants, bitwidth_per_chunk)
    }

    // the equals java method to compare objects (this is NOT for circuit
    // equality check)
    pub fn equals(&self, v: LongElement) -> bool {
        // if o == null || !(o instanceof LongElement) {
        // 	return false;
        // }
        // LongElement v = (LongElement) o;
        if v.array.len() != self.array.len() {
            return false;
        }
        // let mut  check = true;
        // for i in 0.. self.array.len() {
        // 	if !v.array[i].equals(self.array[i]) {
        // 		check = false;
        // 		break;
        // 	}
        // }
        // return check;
        self.array.iter().zip(&v.array).all(|(a, b)| a.equals(b))
    }

    // This asserts that the current bitwidth conditions are satisfied
    pub fn restrictBitwidth(&self) {
        if !self.isAligned() {
            println!(
                "Warning [restrictBitwidth()]: Might want to align before checking bitwidth constraints"
            );
            if Configs.get().unwrap().printStackTraceAtWarnings {
                // Thread.dumpStack();
                println!("Thread.dumpStack()");
            }
        }
        for i in 0..self.array.len() {
            self.array[i].restrictBitLength(self.currentBitwidth[i]);
        }
    }

    pub fn isAligned(&self) -> bool {
        let mut check = true;
        for i in 0..self.array.len() {
            check &= self.currentBitwidth[i] <= Self::CHUNK_BITWIDTH;
        }
        check
    }

    pub fn assertEqualityNaive(&self,a: LongElement) {
        let bits1 = a.getBits(a.getMaxVal(Self::CHUNK_BITWIDTH).bitLength());
        let bits2 = getBits(getMaxVal(Self::CHUNK_BITWIDTH).bitLength());
        let v1 = LongElement::new(bits1);
        let v2 = LongElement::new(bits2);
        for i in 0..v1.array.len() {
            self.generator
                .addEqualityAssertion(v1.array[i], v2.array[i]);
        }
    }

    // an improved equality assertion algorithm from xjsnark
    pub fn assertEquality(&self,e: LongElement) {
        let (mut a1, mut a2) = (self.array.clone(), e.array.clone());
        let (mut bounds1, mut bounds2) =
            (self.currentMaxValues.clone(), e.currentMaxValues.clone());

        let limit = std::cmp::max(a1.len(), a2.len());

        // padding
        if e.array.len() != limit {
            a2 = WireArray::new(a2).adjustLength(limit).asArray();
            bounds2 = vec![BigInteger::ZERO; limit];
            bounds2[..e.currentMaxValues.len()].copy_from_slice(&e.currentMaxValues);
        }
        if self.array.len() != limit {
            a1 = WireArray::new(a1).adjustLength(limit).asArray();
            bounds1 = vec![BigInteger::ZERO; limit];
            bounds1[..self.currentMaxValues.len()].copy_from_slice(&self.currentMaxValues);
        }

        // simple equality assertion cases
        if a1.len() == a2.len() && a1.len() == 1 {
            self.generator().addEqualityAssertion(
                a1[0],
                a2[0],
                "Equality assertion of long elements | case 1",
            );
            return;
        } else if self.isAligned() && e.isAligned() {
            for i in 0..limit {
                self.generator().addEqualityAssertion(
                    a1[i],
                    a2[i],
                    "Equality assertion of long elements | case 2 | index ".to_string()
                        + &i.to_string(),
                );
            }
            return;
        }

        // To make the equality check more efficient, group the chunks together
        // while ensuring that there are no overflows.

        let mut group1 = vec![];
        let mut group1_bounds = vec![];
        let mut group2 = vec![];
        let mut group2_bounds = vec![];

        // This self.array will store how many chunks were grouped together for every
        // wire in group1 or group2
        // The grouping needs to happen in the same way for the two operands, so
        // it's one steps self.array
        let mut steps = vec![];

        let shift = BigInteger::from(2).pow(Self::CHUNK_BITWIDTH);
        let i = 0;
        while i < limit {
            let step = 1;
            let w1 = a1[i];
            let w2 = a2[i];
            let b1 = bounds1[i];
            let b2 = bounds2[i];
            while i + step <= limit - 1 {
                let delta = shift.pow(step);
                if b1.add(bounds1[i + step].multiply(delta)).bitLength()
                    < Configs.get().unwrap().LOG2_FIELD_PRIME - 2
                    && b2.add(bounds2[i + step].multiply(delta)).bitLength()
                        < Configs.get().unwrap().LOG2_FIELD_PRIME - 2
                {
                    w1 = w1.add(a1[i + step].mul(delta));
                    w2 = w2.add(a2[i + step].mul(delta));
                    b1 = b1.add(bounds1[i + step].multiply(delta));
                    b2 = b2.add(bounds2[i + step].multiply(delta));
                    step += 1;
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

        let auxConstant = BigInteger::ZERO;
        let auxConstantChunks = vec![BigInteger::ZERO; numOfGroupedChunks];

        let mut carries = self
            .generator
            .createProverWitnessWireArray(numOfGroupedChunks - 1);
        let mut carriesBitwidthBounds = vec![0; carries.len()];

        // computing the auxConstantChunks, and the total auxConstant
        let mut accumStep = 0;
        for j in 0..auxConstantChunks.len() - 1 {
            auxConstantChunks[j] = BigInteger::from(2).pow(group2_bounds.get(j).bitLength());
            auxConstant = auxConstant.add(auxConstantChunks[j].multiply(shift.pow(accumStep)));
            accumStep += steps.get(j);
            carriesBitwidthBounds[j] = std::cmp::max(
                auxConstantChunks[j].bitLength(),
                group1_bounds.get(j).bitLength(),
            ) - steps.get(j) * Self::CHUNK_BITWIDTH
                + 1;
        }

        // since the two elements should be equal, we should not need any aux
        // chunk in the last step
        auxConstantChunks[auxConstantChunks.len() - 1] = BigInteger::ZERO;

        // Note: the previous auxConstantChunks are not aligned. We compute an
        // aligned version as follows.

        // First split the aux constant into small chunks based on
        // Self::CHUNK_BITWIDTH
        let alignedAuxConstantSmallChunks = Util::split(auxConstant, Self::CHUNK_BITWIDTH);

        // second, group the small aux chunks based on the steps self.array computed
        // earlier to get the alignedAuxConstantChunks
        // alignedAuxConstantChunks is the grouped version of
        // alignedAuxConstantSmallChunks

        let mut alignedAuxConstantChunks = vec![BigInteger::ZERO; numOfGroupedChunks];

        let idx = 0;
        'loop1: for j in 0..numOfGroupedChunks {
            for k in 0..steps.get(j) {
                alignedAuxConstantChunks[j] = alignedAuxConstantChunks[j]
                    .add(alignedAuxConstantSmallChunks[idx].multiply(shift.pow(k)));
                idx += 1;
                if idx == alignedAuxConstantSmallChunks.len() {
                    break 'loop1;
                }
            }
        }
        if idx != alignedAuxConstantSmallChunks.len() {
            if idx == alignedAuxConstantSmallChunks.len() - 1 {
                alignedAuxConstantChunks[numOfGroupedChunks - 1] =
                    alignedAuxConstantChunks[numOfGroupedChunks - 1].add(
                        alignedAuxConstantSmallChunks[idx]
                            .multiply(shift.pow(steps.get(numOfGroupedChunks - 1))),
                    );
            } else {
                panic!("Case not expected. Please report.");
            }
        }

        // specify how the values of carries are obtained during runtime
        self.generator().specifyProverWitnessComputation(&{
            #[derive(Hash,Clone,Debug)]
            struct Prover{
                carries:Vec<WireType>,
                group1: Vec<WireType>,
                group2: Vec<WireType>,
                steps:Vec<i32>,
auxConstantChunks:Vec<BigInteger>,
alignedAuxConstantSmallChunks:Vec<BigInteger>,
            }
            impl Instruction for Prover {
                fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let mut prevCarry = BigInteger::ZERO;
                    for i in 0..self.carries.len() {
                        let a = evaluator.getWireValue(self.group1.get(i));
                        let b = evaluator.getWireValue(self.group2.get(i));
                        let mut carryValue = self.auxConstantChunks[i]
                            .add(a)
                            .subtract(b)
                            .subtract(self.alignedAuxConstantChunks[i])
                            .add(prevCarry);
                        carryValue = carryValue.shiftRight(self.steps.get(i) * Self::CHUNK_BITWIDTH);
                        evaluator.setWireValue(self.carries[i], carryValue);
                        prevCarry = carryValue;
                    }
                }
            }
            Prover{ carries:carries.clone(),
                group1: group1.clone(),
                group2: group2.clone(),
                steps:steps.clone(),
auxConstantChunks:auxConstantChunks.clone(),
alignedAuxConstantSmallChunks:alignedAuxConstantSmallChunks.clone(),
                }
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

        let mut prevCarry = self.generator().getZeroWire();
        let mut prevBound = BigInteger::ZERO;

        // recall carries.len() = numOfGroupedChunks - 1
        for j in 0..carries.len() + 1 {
            let auxConstantChunkWire = self.generator().createConstantWire(auxConstantChunks[j]);
            let alignedAuxConstantChunkWire = self
                .generator
                .createConstantWire(alignedAuxConstantChunks[j]);

            // the last carry value must be zero
            let currentCarry = if j == carries.len() {
                self.generator().getZeroWire()
            } else {
                carries[j]
            };

            // overflow check for safety
            if auxConstantChunks[j]
                .add(group1_bounds.get(j))
                .add(prevBound.compareTo(Configs.get().unwrap().field_prime) >= 0)
            {
                println!("Overflow possibility @ ForceEqual()");
            }

            let w1 = auxConstantChunkWire
                .add(group1.get(j).sub(group2.get(j)))
                .add(prevCarry);
            let w2 = alignedAuxConstantChunkWire.add(currentCarry.mul(shift.pow(steps.get(j))));

            // enforce w1 = w2
            // note: in the last iteration, both auxConstantChunkWire and
            // currentCarry will be zero,
            // i.e., there will be no more values to be checked.

            self.generator().addEqualityAssertion(
                w1,
                w2,
                "Equality assertion of long elements | case 3 | index " + j,
            );

            prevCarry = currentCarry;
            if j != carries.len() {
                prevBound = Util::computeMaxValue(carriesBitwidthBounds[j]);
            }
        }
    }

    // applies an improved technique to assert comparison
    pub fn assertLessThan(&self, other: LongElement) {
        // first verify that both elements are aligned
        assert!(
            self.isAligned() && other.isAligned(),
            "input chunks are not aligned"
        );

        let a1 = self.getArray();
        let a2 = other.getArray();
        let length = std::cmp::max(a1.len(), a2.len());
        let paddedA1 = Util::padWireArray(a1, length, self.generator().getZeroWire());
        let paddedA2 = Util::padWireArray(a2, length, self.generator().getZeroWire());

        /*
         * Instead of doing the comparison naively (which will involve all the
         * bits) let the prover help us by pointing to the first chunk in the
         * other element that is more than the corresponding chunk in this
         * element.
         */
        let helperBits = self.generator().createProverWitnessWireArray(length);
        // set the value of the helperBits outside the circuits

        self.generator().specifyProverWitnessComputation(&{
#[derive(Hash,Clone,Debug)]
            struct Prover{
               pub length:usize,
              pub  paddedA1:Vec<WireType> ,
               pub paddedA2:Vec<WireType> ,
              pub  helperBits:Vec<WireType> ,
            }
            impl Instruction for Prover {
                 fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let mut found = false;
                    for i in (0..self.length).rev() {
                        let v1 = evaluator.getWireValue(self.paddedA1[i]);
                        let v2 = evaluator.getWireValue(self.paddedA2[i]);

                        let check = v2.compareTo(v1) > 0 && !found;
                        evaluator.setWireValue(
                            self.helperBits[i],
                            if check {
                                Util::one()
                            } else {
                                BigInteger::ZERO
                            },
                        );
                        if check {
                            found = true;
                        }
                    }
                }
            }
            Prover{
                length,
                paddedA1:paddedA1.clone() ,
                paddedA2:paddedA2.clone(),
                helperBits:helperBits.clone(),
            }
        });

        // verify constraints about helper bits.
        for w in helperBits {
            self.generator().addBinaryAssertion(w);
        }
        // Only one bit should be set.
        self.generator
            .addOneAssertion(WireArray::new(helperBits).sumAllElements());

        // verify "the greater than condition" for the specified chunk
        let chunk1 = self.generator().getZeroWire();
        let chunk2 = self.generator().getZeroWire();

        for i in 0..helperBits.len() {
            chunk1 = chunk1.add(paddedA1[i].mul(helperBits[i]));
            chunk2 = chunk2.add(paddedA2[i].mul(helperBits[i]));
        }
        self.generator
            .addOneAssertion(chunk1.isLessThan(chunk2, Self::CHUNK_BITWIDTH));

        // check that the other more significant chunks are equal
        let mut helperBits2 = vec![self.generator().getZeroWire(); helperBits.len()];
        for i in 1..helperBits.len() {
            helperBits2[i] = helperBits2[i - 1].add(helperBits[i - 1]);
            //			self.generator().addZeroAssertion(helperBits2[i].mul(paddedA1[i]
            //					.sub(paddedA2[i])));
            self.generator().addAssertion(
                helperBits2[i],
                paddedA1[i].sub(paddedA2[i]),
                self.generator().getZeroWire(),
            );
        }

        // no checks needed for the less significant chunks
    }
}

impl Add<u64> for LongElement {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        self.add(BigInteger::from(rhs))
    }
}

impl Add<BigInteger> for LongElement {
    type Output = Self;

    fn add(self, rhs: BigInteger) -> Self::Output {
        if rhs.signum() == 0 {
            return self;
        }
        if rhs.signum() < 0 {
            return self.sub(rhs.negate());
        }
        self.add(LongElement::new(Util::split(rhs, Self::CHUNK_BITWIDTH)))
    }
}

impl Add for LongElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.addOverflowCheck(rhs) {
            println!("Warning: Addition overflow could happen");
        }

        let length = std::cmp::max(self.array.len(), rhs.array.len());
        let w1 = WireArray::new(self.array).adjustLength(length).asArray();
        let w2 = WireArray::new(rhs.array).adjustLength(length).asArray();
        let result = vec![self.generator().getZeroWire(); length];
        let newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..length {
            result[i] = w1[i].add(w2[i]);
            let max1 = if i < self.array.len() {
                self.currentMaxValues[i]
            } else {
                BigInteger::ZERO
            };
            let max2 = if i < rhs.array.len() {
                rhs.currentMaxValues[i]
            } else {
                BigInteger::ZERO
            };

            newMaxValues[i] = max1.add(max2);
        }
        LongElement::new(result, newMaxValues)
    }
}
impl Sub<u64> for LongElement {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self::sub(self, BigInteger::from(rhs))
    }
}
impl Sub<BigInteger> for LongElement {
    type Output = Self;

    fn sub(self, rhs: BigInteger) -> Self::Output {
        if rhs.signum() == 0 {
            return self;
        }
        if rhs.signum() < 0 {
            return self.add(rhs.negate());
        }
        self.sub(LongElement::new(Util::split(rhs, Self::CHUNK_BITWIDTH)))
    }
}

impl Sub for LongElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            isAligned() && rhs.isAligned(),
            "Subtraction arguments must be properly aligned"
        );

        let result = self
            .generator
            .createLongElementProverWitness(getMaxVal(Self::CHUNK_BITWIDTH).bitLength());

        self.generator().specifyProverWitnessComputation(&{
#[derive(Hash,Clone,Debug)]
            struct Prover{
               pub long_element:LongElement,
               pub other:Vec<WireType>,
               pub result:Vec<WireType>,
            }
            impl Instruction for Prover {
                fn evaluate(&self,evaluator: CircuitEvaluator) {
                    let myValue = evaluator.getWireValue(self.long_element.clone(), Self::CHUNK_BITWIDTH);
                    let otherValue = evaluator.getWireValue(self.other.clone(), Self::CHUNK_BITWIDTH);
                    let resultValue = myValue.subtract(otherValue);
                    assert!(
                        resultValue.signum() >= 0,
                        "Result of subtraction is negative!"
                    );
                    evaluator.setWireValue(self.result.clone(), resultValue, Self::CHUNK_BITWIDTH);
                }
            }
            Prover{
                long_element:self.clone(),
                other:rhs.clone(),
                result:result.clone(),
            }
        });

        result.restrictBitwidth();
        self.assertEquality(result.add(rhs));
        result
    }
}

impl Mul for LongElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        
         // Implements the improved long integer multiplication from xjsnark
         
        if mulOverflowCheck(rhs) {
            println!("Warning: Mul overflow could happen");
        }
        let length = self.array.len() + rhs.array.len() - 1;
        let mut result;

        // check if we can just apply the simple non-costly multiplication
        if rhs.array.len() == 1 || self.array.len() == 1 || self.isConstant(|| rhs.isConstant()) {
            result = vec![self.generator().getZeroWire(); length];

            // O(n*m) multiplication. Fine to apply if any of the operands has
            // dim 1
            // or any of them is constant
            for i in 0..self.array.len() {
                for j in 0..rhs.array.len() {
                    result[i + j] = result[i + j].add(self.array[i].mul(rhs.array[j]));
                }
            }
        } else {
            // implement the optimization

            result = self.generator().createProverWitnessWireArray(length);

            // for safety
           
            self.generator().specifyProverWitnessComputation(&{
#[derive(Hash,Clone,Debug)]
                struct Prover{
               pub result:Vec<WireType>,
               pub array1:Vec<WireType>,
               pub array2:Vec<WireType>,
            }
                impl Instruction for Prover {
                    fn evaluate(&self,evaluator: CircuitEvaluator) {
                        let a = evaluator.getWiresValues(self.array1.clone());
                        let b = evaluator.getWiresValues(self.array2.clone());
                        let resultVals = multiplyPolys(a, b);
                        evaluator.setWireValue(self.result.clone(), resultVals);
                    }
                }
                Prover{
                result:result.clone(),
                 array1 : self.array.clone(),
                     array2:rhs.array.clone(),
                }
            });

            for k in 0..length {
                let constant = BigInteger::new((k + 1) + "");
                let coeff = Util::one();

                let vector1 = vec![self.generator().getZeroWire(); self.array.len()];
                let vector2 = vec![self.generator().getZeroWire(); rhs.array.len()];
                let vector3 = vec![self.generator().getZeroWire(); length];
                for i in 0..length {
                    if i < self.array.len() {
                        vector1[i] = self.array[i].mul(coeff);
                    }
                    if i < rhs.array.len() {
                        vector2[i] = rhs.array[i].mul(coeff);
                    }
                    vector3[i] = result[i].mul(coeff);
                    coeff =
                        Util::modulo(coeff.multiply(constant), Configs.get().unwrap().field_prime);
                }

                let v1 = WireArray::new(vector1).sumAllElements();
                let v2 = WireArray::new(vector2).sumAllElements();
                let v3 = WireArray::new(vector3).sumAllElements();
                self.generator().addAssertion(v1, v2, v3);
            }
        }

        let mut newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..rhs.array.len() {
                newMaxValues[i + j] =
                    newMaxValues[i + j].add(self.currentMaxValues[i].multiply(rhs.currentMaxValues[j]));
            }
        }
        LongElement::new(result, newMaxValues)
    }
}
