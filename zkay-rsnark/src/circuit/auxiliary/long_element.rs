#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::{
    InstanceOf,
    {
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, getActiveCircuitGenerator,
            },
            constant_wire,
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_array::WireArray,
            wire_ops::{AddWire, MulWire, SubWire},
            wire_type::WireType,
        },
    },
};
use crate::util::util::{
    ARcCell, {BigInteger, Util},
};
use num_bigint::Sign;
use num_traits::Signed;
use rccell::{RcCell, WeakCell};
use serde::{Serialize, de::DeserializeOwned};
use serde_closure::{Fn, FnMut, FnOnce};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Rem, Shr, Sub},
    sync::Arc,
};
use zkay_derive::ImplStructNameConfig;
/**
 * An auxiliary class that handles the operations of long integers, such as the
 * ones used in RSA operations. It applies some of the long integer
 * optimizations from xjsnark (to appear). This is a preliminary version. More
 * Other features and detailed tests will be added in the future.
 *
 * Usage examples exist in the RSA examples gadgets.
 */
// pub type BigInteger = String;

#[derive(Debug, Clone, Hash)]
pub struct LongElement {
    pub array: Vec<Option<WireType>>,
    pub currentBitwidth: Vec<u64>,
    pub currentMaxValues: Vec<BigInteger>,
    pub bits: Option<WireArray>,
    pub generator: WeakCell<CircuitGenerator>,
}
impl LongElement {
    // Should be declared as final, but left non-for testing purposes.
    // Don't change in the middle of circuit generation.
    // This represents the size of smaller chunks used to represent long
    // elements
    pub const CHUNK_BITWIDTH: i32 = 120;

    pub fn newa(mut bits: WireArray, generator: WeakCell<CircuitGenerator>) -> Self {
        let generators = generator.clone().upgrade().unwrap();
        let (array, currentMaxValues, currentBitwidth) =
            if Self::CHUNK_BITWIDTH >= bits.size() as i32 {
                (
                    vec![Some(bits.packAsBits(None, Some(bits.size()), &None))],
                    vec![Util::computeMaxValue(bits.size() as u64)],
                    vec![bits.size() as u64],
                )
            } else {
                let mut maxChunkVal = Util::computeMaxValue(Self::CHUNK_BITWIDTH as u64);
                let mut maxLastChunkVal = maxChunkVal.clone();
                let size = bits.size() as i32;
                if size % Self::CHUNK_BITWIDTH != 0 {
                    bits = bits.adjustLength(
                        None,
                        (size + (Self::CHUNK_BITWIDTH - size % Self::CHUNK_BITWIDTH)) as usize,
                    );
                    maxLastChunkVal = Util::computeMaxValue((size % Self::CHUNK_BITWIDTH) as u64);
                }
                let mut array = vec![None; bits.size() / Self::CHUNK_BITWIDTH as usize];
                let mut currentMaxValues = vec![BigInteger::default(); array.len()];
                let mut currentBitwidth = vec![0u64; array.len()];

                for i in 0..array.len() {
                    array[i] = Some(
                        WireArray::new(
                            bits.asArray()[i * Self::CHUNK_BITWIDTH as usize
                                ..(i + 1) * Self::CHUNK_BITWIDTH as usize]
                                .to_vec(),
                            generators.borrow().me.clone().unwrap(),
                        )
                        .packAsBits(None, None, &None),
                    );
                    if i == array.len() - 1 {
                        currentMaxValues[i] = maxLastChunkVal.clone();
                        currentBitwidth[i] = maxLastChunkVal.bits() as u64;
                    } else {
                        currentMaxValues[i] = maxChunkVal.clone();
                        currentBitwidth[i] = maxChunkVal.bits() as u64;
                    }
                }
                (array, currentMaxValues, currentBitwidth)
            };
        Self {
            array,
            currentMaxValues,
            currentBitwidth,
            bits: Some(bits),
            generator,
        }
    }

    pub fn new(
        w: Vec<Option<WireType>>,
        currentBitwidth: Vec<u64>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Self {
        let mut currentMaxValues = vec![BigInteger::ZERO; w.len()];
        for i in 0..w.len() {
            currentMaxValues[i] = Util::computeMaxValue(currentBitwidth[i]);
        }
        Self {
            array: w,
            currentBitwidth,
            currentMaxValues,
            bits: None,
            generator,
        }
    }
    pub fn generator(&self) -> RcCell<CircuitGenerator> {
        // ARcCell<dyn CGConfig + Send + Sync>
        // getActiveCircuitGenerator().unwrap()
        self.generator.clone().upgrade().unwrap()
    }
    pub fn makeOutput(&mut self, desc: &Option<String>) {
        let mut generator = self.generator();

        for w in self.getArray() {
            generator.makeOutput(w.as_ref().unwrap(), desc);
        }
    }

    /**
     * A long element representing a constant.
     */
    pub fn newc(chunks: Vec<BigInteger>, generator: WeakCell<CircuitGenerator>) -> Self {
        let mut currentBitwidth = vec![0; chunks.len()];
        for i in 0..chunks.len() {
            currentBitwidth[i] = chunks[i].bits();
        }
        // let mut generator = getActiveCircuitGenerator().unwrap();
        //
        Self {
            array: generator
                .clone()
                .upgrade()
                .unwrap()
                .createConstantWireArray(&chunks, &None),
            currentMaxValues: chunks,
            currentBitwidth,
            bits: None,
            generator,
        }
    }

    pub fn newb(
        w: Vec<Option<WireType>>,
        currentMaxValues: Vec<BigInteger>,
        generator: WeakCell<CircuitGenerator>,
    ) -> Self {
        let mut currentBitwidth = vec![0; w.len()];
        for i in 0..w.len() {
            currentBitwidth[i] = currentMaxValues[i].bits();
        }
        Self {
            array: w,
            currentMaxValues,
            currentBitwidth,
            bits: None,
            generator,
        }
    }

    pub fn addOverflowCheck(&self, o: &Self) -> bool {
        self.currentMaxValues
            .iter()
            .zip(&o.currentMaxValues)
            .any(|(a, b)| a + b >= Configs.field_prime)
    }

    pub fn mulOverflowCheck(&self, o: &Self) -> bool {
        let length = self.array.len() + o.array.len() - 1;
        let mut overflow = false;
        let mut newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..o.array.len() {
                newMaxValues[i + j] = newMaxValues[i + j]
                    .clone()
                    .add(self.currentMaxValues[i].clone().mul(&o.currentMaxValues[j]));
            }
        }
        for i in 0..length {
            if newMaxValues[i] >= Configs.field_prime {
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
                isConstant &= self.array[i].as_ref().unwrap().instance_of("ConstantWire");
            }
        }
        isConstant
    }

    pub fn getSize(&self) -> usize {
        self.array.len()
    }

    pub fn align(&self, totalNumChunks: usize) -> Self {
        let generator = self.generator();
        let mut newArray = self.array[..totalNumChunks].to_vec();
        let zero_wire = generator.get_zero_wire();
        for i in 0..newArray.len() {
            if newArray[i].is_none() {
                newArray[i] = zero_wire.clone();
            }
        }
        let mut newMaxValues = vec![BigInteger::ZERO; totalNumChunks];
        newMaxValues[..totalNumChunks.min(self.currentMaxValues.len())]
            .clone_from_slice(&self.currentMaxValues);
        let mut maxAlignedChunkValue = Util::computeMaxValue(Self::CHUNK_BITWIDTH as u64);

        for i in 0..totalNumChunks {
            if newMaxValues[i].bits() > Self::CHUNK_BITWIDTH as u64 {
                let chunkBits = newArray[i]
                    .as_ref()
                    .unwrap()
                    .getBitWiresi(newMaxValues[i].bits(), &None);
                let chunkBits = chunkBits.asArray();
                newArray[i] = Some(
                    WireArray::new(
                        chunkBits[..Self::CHUNK_BITWIDTH as usize].to_vec(),
                        self.generator.clone(),
                    )
                    .packAsBits(None, None, &None),
                );
                let mut rem = WireArray::new(
                    chunkBits[Self::CHUNK_BITWIDTH as usize..newMaxValues[i].bits() as usize]
                        .to_vec(),
                    self.generator.clone(),
                )
                .packAsBits(None, None, &None);
                if i != totalNumChunks - 1 {
                    newMaxValues[i + 1] = newMaxValues[i]
                        .clone()
                        .shr(Self::CHUNK_BITWIDTH)
                        .add(&newMaxValues[i + 1]);
                    newArray[i + 1] = Some(rem.add(newArray[i + 1].as_ref().unwrap()));
                }
                newMaxValues[i] = maxAlignedChunkValue.clone();
            }
        }
        LongElement::newb(newArray, newMaxValues, self.generator.clone())
    }

    // This method extracts (some of) the bit wires corresponding to a long
    // element based on the totalBitwidth argument.
    // If totalBitwidth is -1, all bits are returned.
    // See restrictBitwidth for restricting the bitwidth of all the long element
    // chunks

    pub fn getBitsi(&mut self, totalBitwidth: i32) -> WireArray {
        let generator = self.generator();
        if let Some(bits) = &self.bits {
            return bits.adjustLength(
                None,
                if totalBitwidth == -1 {
                    bits.size()
                } else {
                    totalBitwidth as usize
                },
            );
        }
        if self.array.len() == 1 {
            self.bits = Some(
                self.array[0]
                    .as_ref()
                    .unwrap()
                    .getBitWiresi(self.currentMaxValues[0].bits(), &None),
            );
            return self.bits.as_ref().unwrap().adjustLength(
                None,
                if totalBitwidth == -1 {
                    self.bits.as_ref().unwrap().size()
                } else {
                    totalBitwidth as usize
                },
            );
        }
        if totalBitwidth <= Self::CHUNK_BITWIDTH && totalBitwidth >= 0 {
            let out = self.array[0]
                .as_ref()
                .unwrap()
                .getBitWiresi(self.currentMaxValues[0].bits(), &None);
            return out.adjustLength(None, totalBitwidth as usize);
        }

        let mut limit = totalBitwidth as usize;
        let maxVal = self.getMaxVal(Self::CHUNK_BITWIDTH);

        let mut bitWires = if totalBitwidth != -1 {
            vec![None; totalBitwidth as usize]
        } else {
            limit = maxVal.bits() as usize;
            vec![None; maxVal.bits() as usize]
        };

        let newLength = (self.getMaxVal(Self::CHUNK_BITWIDTH).bits() as f64 * 1.0
            / Self::CHUNK_BITWIDTH as f64)
            .ceil() as usize;
        let mut newArray = vec![None; newLength];
        let mut newMaxValues = vec![BigInteger::ZERO; newLength];

        newMaxValues[0..self.currentMaxValues.len()].clone_from_slice(&self.currentMaxValues);
        newArray[0..self.array.len()].clone_from_slice(&self.array);
        let mut idx = 0;
        let mut chunkIndex = 0;
        while idx < limit && chunkIndex < newLength {
            let (bits, bitwidth) = (newMaxValues[chunkIndex].bits(), Self::CHUNK_BITWIDTH as u64);
            let chunkBits = newArray[chunkIndex]
                .as_ref()
                .unwrap()
                .getBitWiresi(bits.max(bitwidth), &None);
            let chunkBits = chunkBits.asArray();
            let alignedChunkBits;
            if newMaxValues[chunkIndex].bits() > Self::CHUNK_BITWIDTH as u64 {
                alignedChunkBits = &chunkBits[..Self::CHUNK_BITWIDTH as usize];
                let rem = WireArray::new(
                    chunkBits
                        [Self::CHUNK_BITWIDTH as usize..newMaxValues[chunkIndex].bits() as usize]
                        .to_vec(),
                    self.generator.clone(),
                )
                .packAsBits(None, None, &None);

                if chunkIndex != newArray.len() - 1 {
                    newMaxValues[chunkIndex + 1] = newMaxValues[chunkIndex]
                        .clone()
                        .shr(Self::CHUNK_BITWIDTH)
                        .add(newMaxValues[chunkIndex + 1].clone());
                    newArray[chunkIndex + 1] =
                        Some(rem.add(newArray[chunkIndex + 1].as_ref().unwrap()));
                }
            } else {
                alignedChunkBits = chunkBits;
            }
            bitWires[idx..std::cmp::min(alignedChunkBits.len(), limit - idx)]
                .clone_from_slice(&alignedChunkBits);
            chunkIndex += 1;
            idx += alignedChunkBits.len();
        }
        let out = WireArray::new(bitWires, self.generator.clone());
        if limit >= maxVal.bits() as usize {
            self.bits = Some(out.adjustLength(None, maxVal.bits() as usize));
        }
        out
    }

    pub fn getMaxVal(&self, bitwidth: i32) -> BigInteger {
        Util::group(&self.currentMaxValues, bitwidth)
    }

    fn multiplyPolys(aiVals: Vec<BigInteger>, biVals: Vec<BigInteger>) -> Vec<BigInteger> {
        let mut solution = vec![BigInteger::ZERO; aiVals.len() + biVals.len() - 1];

        for i in 0..aiVals.len() {
            for j in 0..biVals.len() {
                solution[i + j] = solution[i + j]
                    .clone()
                    .add(aiVals[i].clone().mul(biVals[j].clone()))
                    .rem(Configs.field_prime.clone());
            }
        }
        solution
    }

    pub fn muxBit(&self, other: Self, w: WireType) -> Self {
        let length = std::cmp::max(self.array.len(), other.array.len());
        let mut newArray = vec![None; length];
        let mut newMaxValues = vec![BigInteger::ZERO; length];
        let zero_wire = self.generator().get_zero_wire().unwrap();
        for i in 0..length {
            let b1 = if i < self.array.len() {
                self.currentMaxValues[i].clone()
            } else {
                BigInteger::ZERO
            };
            let b2 = if i < other.array.len() {
                other.currentMaxValues[i].clone()
            } else {
                BigInteger::ZERO
            };
            newMaxValues[i] = if b1 > b2 { b1 } else { b2 };

            let w1 = if i < self.array.len() {
                self.array[i].clone().unwrap()
            } else {
                zero_wire.clone()
            };
            let w2 = if i < other.array.len() {
                other.array[i].clone().unwrap()
            } else {
                zero_wire.clone()
            };
            newArray[i] = Some(w1.clone().add(w.clone().mul(w2.clone().sub(w1))));
            if newArray[i].as_ref().unwrap().instance_of("ConstantWire") {
                newMaxValues[i] = newArray[i]
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .getConstant();
            }
        }
        LongElement::newb(newArray, newMaxValues, self.generator.clone())
    }

    pub fn checkNonZero(&self) -> WireType {
        let mut wireNonZero = vec![None; self.array.len()];
        for i in 0..self.array.len() {
            wireNonZero[i] = self.array[i].as_ref().map(|x| x.checkNonZero(&None));
        }
        WireArray::new(wireNonZero, self.generator.clone())
            .sumAllElements(&None)
            .checkNonZero(&None)
    }

    pub fn getArray(&self) -> &Vec<Option<WireType>> {
        &self.array
    }

    pub fn getCurrentBitwidth(&self) -> Vec<u64> {
        self.currentBitwidth.clone()
    }

    pub fn getCurrentMaxValues(&self) -> Vec<BigInteger> {
        self.currentMaxValues.clone()
    }

    pub fn getBits(&self) -> Option<WireArray> {
        self.bits.clone()
    }

    pub fn getConstant(&self, bitwidth_per_chunk: i32) -> Option<BigInteger> {
        let mut constants = vec![BigInteger::ZERO; self.array.len()];
        for i in 0..self.array.len() {
            if self.array[i].as_ref().unwrap().instance_of("ConstantWire") {
                return None;
            } else {
                constants[i] = self.array[i]
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .getConstant();
            }
        }
        Some(Util::group(&constants, bitwidth_per_chunk))
    }

    // the equals java method to compare objects (this is NOT for circuit
    // equality check)
    // pub fn equals(&self, v:Self) -> bool {
    //     // if o == null || !(o instance_of LongElement) {
    //     // 	return false;
    //     // }
    //     // LongElement v = (LongElement) o;
    //     if v.array.len() != self.array.len() {
    //         return false;
    //     }
    //     // let mut  check = true;
    //     // for i in 0.. self.array.len() {
    //     // 	if !v.array[i]==self.array[i] {
    //     // 		check = false;
    //     // 		break;
    //     // 	}
    //     // }
    //     // return check;
    //     self.array.iter().zip(&v.array).all(|(a, b)| a == b)
    // }

    // This asserts that the current bitwidth conditions are satisfied
    pub fn restrictBitwidth(&self) {
        if !self.isAligned() {
            println!(
                "Warning [restrictBitwidth()]: Might want to align before checking bitwidth constraints"
            );
            if Configs.print_stack_trace_at_warnings {
                // Thread.dumpStack();
                //println!("Thread.dumpStack()");
            }
        }
        for i in 0..self.array.len() {
            self.array[i]
                .as_ref()
                .unwrap()
                .restrictBitLength(self.currentBitwidth[i], &None);
        }
    }

    pub fn isAligned(&self) -> bool {
        let mut check = true;
        for i in 0..self.array.len() {
            check &= self.currentBitwidth[i] <= Self::CHUNK_BITWIDTH as u64;
        }
        check
    }

    pub fn assertEqualityNaive(&mut self, a: &mut Self) {
        let bits1 = a.getBitsi(a.getMaxVal(Self::CHUNK_BITWIDTH).bits() as i32);
        let bits2 = self.getBitsi(self.getMaxVal(Self::CHUNK_BITWIDTH).bits() as i32);
        let v1 = LongElement::newa(bits1, self.generator.clone());
        let v2 = LongElement::newa(bits2, self.generator.clone());
        let mut generator = self.generator();

        for i in 0..v1.array.len() {
            generator.addEqualityAssertion(
                v1.array[i].as_ref().unwrap(),
                v2.array[i].as_ref().unwrap(),
                &None,
            );
        }
    }

    // an improved equality assertion algorithm from xjsnark
    pub fn assertEquality(&self, e: &Self) {
        let generator = self.generator();
        let (mut a1, mut a2) = (self.array.clone(), e.array.clone());
        let (mut bounds1, mut bounds2) =
            (self.currentMaxValues.clone(), e.currentMaxValues.clone());

        let limit = std::cmp::max(a1.len(), a2.len());

        // padding
        if e.array.len() != limit {
            a2 = WireArray::new(a2, self.generator.clone())
                .adjustLength(None, limit)
                .asArray()
                .clone();
            bounds2 = vec![BigInteger::ZERO; limit];
            bounds2[..e.currentMaxValues.len()].clone_from_slice(&e.currentMaxValues);
        }
        if self.array.len() != limit {
            a1 = WireArray::new(a1, self.generator.clone())
                .adjustLength(None, limit)
                .asArray()
                .clone();
            bounds1 = vec![BigInteger::ZERO; limit];
            bounds1[..self.currentMaxValues.len()].clone_from_slice(&self.currentMaxValues);
        }
        let mut generator = self.generator();

        // simpl e equality assertion cases
        if a1.len() == a2.len() && a1.len() == 1 {
            generator.addEqualityAssertion(
                a1[0].as_ref().unwrap(),
                a2[0].as_ref().unwrap(),
                &Some("Equality assertion of long elements | case 1".to_owned()),
            );
            return;
        } else if self.isAligned() && e.isAligned() {
            for i in 0..limit {
                generator.addEqualityAssertion(
                    a1[i].as_ref().unwrap(),
                    a2[i].as_ref().unwrap(),
                    &Some(format! {"Equality assertion of long elements | case 2 | index {i}"}),
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

        let shift = BigInteger::from(2).pow(Self::CHUNK_BITWIDTH as u32);
        let mut i = 0;
        while i < limit {
            let mut step = 1;
            let mut w1 = a1[i].clone().unwrap();
            let mut w2 = a2[i].clone().unwrap();
            let mut b1 = bounds1[i].clone();
            let mut b2 = bounds2[i].clone();
            while i + step <= limit - 1 {
                let delta = shift.pow(step as u32);
                if b1.clone().add(bounds1[i + step].clone().mul(&delta)).bits()
                    < Configs.log2_field_prime - 2
                    && b2.clone().add(bounds2[i + step].clone().mul(&delta)).bits()
                        < Configs.log2_field_prime - 2
                {
                    w1 = w1.add(a1[i + step].as_ref().unwrap().mulb(&delta, &None));
                    w2 = w2.add(a2[i + step].as_ref().unwrap().mulb(&delta, &None));
                    b1 = b1.add(bounds1[i + step].clone().mul(&delta));
                    b2 = b2.add(bounds2[i + step].clone().mul(&delta));
                    step += 1;
                } else {
                    break;
                }
            }
            group1.push(Some(w1));
            group1_bounds.push(b1);
            group2.push(Some(w2));
            group2_bounds.push(b2);
            steps.push(step);
            i += step;
        }

        let numOfGroupedChunks = group1.len();

        // After grouping, subtraction will be needed to compare the grouped
        // chunks and propagate carries.
        // To avoid dealing with cases where the first operand in the
        // subtraction is less than the second operand,
        // we introduce an auxiliary constant computed based on the bounds of
        // the second operand. The chunks
        // of this auxConstant will be added to the chunks of the first operand
        // before subtraction.

        let mut auxConstant = BigInteger::ZERO;
        let mut auxConstantChunks = vec![BigInteger::ZERO; numOfGroupedChunks];

        let mut carries = generator.createProverWitnessWireArray(numOfGroupedChunks - 1, &None);
        let mut carriesBitwidthBounds = vec![0; carries.len()];

        // computing the auxConstantChunks, and the total auxConstant
        let mut accumStep = 0;
        for j in 0..auxConstantChunks.len() - 1 {
            auxConstantChunks[j] = BigInteger::from(2).pow(group2_bounds[j].bits() as u32);
            auxConstant = auxConstant.add(auxConstantChunks[j].clone().mul(shift.pow(accumStep)));
            accumStep += steps[j] as u32;
            carriesBitwidthBounds[j] =
                std::cmp::max(auxConstantChunks[j].bits(), group1_bounds[j].bits())
                    - steps[j] as u64 * Self::CHUNK_BITWIDTH as u64
                    + 1;
        }

        // since the two elements should be equal, we should not need any aux
        // chunk in the last step
        *auxConstantChunks.last_mut().unwrap() = BigInteger::ZERO;

        // Note: the previous auxConstantChunks are not aligned. We compute an
        // aligned version as follows.

        // First split the aux constant into small chunks based on
        // Self::CHUNK_BITWIDTH
        let alignedAuxConstantSmallChunks = Util::split(&auxConstant, Self::CHUNK_BITWIDTH);

        // second, group the small aux chunks based on the steps self.array computed
        // earlier to get the alignedAuxConstantChunks
        // alignedAuxConstantChunks is the grouped version of
        // alignedAuxConstantSmallChunks

        let mut alignedAuxConstantChunks = vec![BigInteger::ZERO; numOfGroupedChunks];

        let mut idx = 0;
        'loop1: for j in 0..numOfGroupedChunks {
            for k in 0..steps[j] {
                alignedAuxConstantChunks[j] = alignedAuxConstantChunks[j].clone().add(
                    alignedAuxConstantSmallChunks[idx]
                        .clone()
                        .mul(shift.pow(k as u32)),
                );
                idx += 1;
                if idx == alignedAuxConstantSmallChunks.len() {
                    break 'loop1;
                }
            }
        }
        if idx != alignedAuxConstantSmallChunks.len() {
            if idx == alignedAuxConstantSmallChunks.len() - 1 {
                alignedAuxConstantChunks[numOfGroupedChunks - 1] = alignedAuxConstantChunks
                    [numOfGroupedChunks - 1]
                    .clone()
                    .add(
                        alignedAuxConstantSmallChunks[idx]
                            .clone()
                            .mul(shift.pow(steps[numOfGroupedChunks - 1] as u32)),
                    );
            } else {
                panic!("Case not expected. Please report.");
            }
        }
        let steps: Vec<_> = steps.iter().map(|&i| i as i32).collect();
        // specify how the values of carries are obtained during runtime
        let prover = crate::impl_prover!(
                        eval( carries: Vec<Option<WireType>>,
                        group1: Vec<Option<WireType>>,
                        group2: Vec<Option<WireType>>,
                        steps: Vec<i32>,
                        auxConstantChunks: Vec<BigInteger>,
                        alignedAuxConstantChunks: Vec<BigInteger>)  {
        impl Instruction for Prover{
         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                            let mut prevCarry = BigInteger::ZERO;
                            for i in 0..self.carries.len() {
                                let a = evaluator.getWireValue(self.group1[i].as_ref().unwrap());
                                let b = evaluator.getWireValue(self.group2[i].as_ref().unwrap());
                                let mut carryValue = self.auxConstantChunks[i]
                                    .clone()
                                    .add(a)
                                    .sub(b)
                                    .sub(&self.alignedAuxConstantChunks[i])
                                    .add(prevCarry);
                                carryValue = carryValue.shr(self.steps[i] * LongElement::CHUNK_BITWIDTH);
                                evaluator
                                    .setWireValue(self.carries[i].as_ref().unwrap(), carryValue.clone());
                                prevCarry = carryValue;
                            }
        }
        }
                    }
                );
        generator.specifyProverWitnessComputation(prover);
        // self.generator()
        //     .specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //         let mut prevCarry = BigInteger::ZERO;
        //         for i in 0..carries.len() {
        //             let a = evaluator.getWireValue(group1[i].clone().unwrap());
        //             let b = evaluator.getWireValue(group2[i].clone().unwrap());
        //             let mut carryValue = auxConstantChunks[i]
        //                 .clone()
        //                 .add(a)
        //                 .sub(b)
        //                 .sub(alignedAuxConstantChunks[i].clone())
        //                 .add(prevCarry);
        //             carryValue = carryValue.shr(steps[i] * LongElement::CHUNK_BITWIDTH as usize);
        //             evaluator.setWireValue(carries[i].clone().unwrap(), carryValue.clone());
        //             prevCarry = carryValue;
        //         }
        //     });
        //     {
        //     #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
        //     struct Prover {
        //         carries: Vec<Option<WireType>>,
        //         group1: Vec<Option<WireType>>,
        //         group2: Vec<Option<WireType>>,
        //         steps: Vec<i32>,
        //         auxConstantChunks: Vec<BigInteger>,
        //         alignedAuxConstantChunks: Vec<BigInteger>,
        //     }
        //     impl Instruction for Prover{
        //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //             let mut prevCarry = BigInteger::ZERO;
        //             for i in 0..carries.len() {
        //                 let a = evaluator.getWireValue(self.group1[i].clone().unwrap());
        //                 let b = evaluator.getWireValue(self.group2[i].clone().unwrap());
        //                 let mut carryValue = self.auxConstantChunks[i]
        //                     .clone()
        //                     .add(a)
        //                     .sub(b)
        //                     .sub(self.alignedAuxConstantChunks[i].clone())
        //                     .add(prevCarry);
        //                 carryValue = carryValue.shr(self.steps[i] * LongElement::CHUNK_BITWIDTH);
        //                 evaluator
        //                     .setWireValue(self.carries[i].clone().unwrap(), carryValue.clone());
        //                 prevCarry = carryValue;
        //             }
        //         }
        //     }
        //     Box::new(Prover {
        //         carries: carries.clone(),
        //         group1: group1.clone(),
        //         group2: group2.clone(),
        //         steps: steps.iter().map(|&i| i as i32).collect(),
        //         auxConstantChunks: auxConstantChunks.clone(),
        //         alignedAuxConstantChunks: alignedAuxConstantChunks.clone(),
        //     })
        // });

        // We must make sure that the carries values are bounded.

        for j in 0..carries.len() {
            // carries[j].getBitWires(carriesBitwidthBounds[j]);
            carries[j]
                .as_ref()
                .unwrap()
                .restrictBitLength(carriesBitwidthBounds[j], &None);

            // Note: in this context restrictBitLength and getBitWires will be
            // the same, but it's safer to use restrictBitLength
            // when enforcing constraints.
        }

        // Now apply the main constraints
        let zero_wire = generator.get_zero_wire();
        let mut prevCarry = zero_wire.clone().unwrap();
        let mut prevBound = BigInteger::ZERO;

        // recall carries.len() = numOfGroupedChunks - 1
        for j in 0..carries.len() + 1 {
            let auxConstantChunkWire = generator.createConstantWire(&auxConstantChunks[j], &None);
            let alignedAuxConstantChunkWire =
                generator.createConstantWire(&alignedAuxConstantChunks[j], &None);

            // the last carry value must be zero
            let currentCarry = if j == carries.len() {
                zero_wire.clone()
            } else {
                carries[j].clone()
            };

            // overflow check for safety
            if auxConstantChunks[j]
                .clone()
                .add(group1_bounds[j].clone())
                .add(BigInteger::from((prevBound >= Configs.field_prime) as u8))
                != BigInteger::ZERO
            {

                //println!("Overflow possibility @ ForceEqual()");
            }

            let w1 = auxConstantChunkWire
                .add(group1[j].clone().unwrap().sub(group2[j].as_ref().unwrap()))
                .add(prevCarry);
            let w2 = alignedAuxConstantChunkWire.add(
                currentCarry
                    .clone()
                    .unwrap()
                    .mulb(&shift.pow(steps[j] as u32), &None),
            );

            // enforce w1 = w2
            // note: in the last iteration, both auxConstantChunkWire and
            // currentCarry will be zero,
            // i.e., there will be no more values to be checked.

            generator.addEqualityAssertion(
                &w1,
                &w2,
                &Some(format!(
                    "Equality assertion of long elements | case 3 | index {j}"
                )),
            );

            prevCarry = currentCarry.clone().unwrap();
            if j != carries.len() {
                prevBound = Util::computeMaxValue(carriesBitwidthBounds[j]);
            }
        }
    }

    // applies an improved technique to assert comparison
    pub fn assertLessThan(&self, other: Self) {
        // first verify that both elements are aligned
        assert!(
            self.isAligned() && other.isAligned(),
            "input chunks are not aligned"
        );

        let a1 = self.getArray();
        let a2 = other.getArray();
        let length = std::cmp::max(a1.len(), a2.len());
        let mut generator = self.generator();

        let zero_wire = generator.get_zero_wire().unwrap();
        let paddedA1 = Util::padWireArray(&a1, length, &zero_wire);
        let paddedA2 = Util::padWireArray(&a2, length, &zero_wire);

        /*
         * Instead of doing the comparison naively (which will involve all the
         * bits) let the prover help us by pointing to the first chunk in the
         * other element that is more than the corresponding chunk in this
         * element.
         */
        let helperBits = generator.createProverWitnessWireArray(length, &None);
        // set the value of the helperBits outside the circuits
        let prover = crate::impl_prover!(
                eval(
                         length: usize,
                        paddedA1: Vec<Option<WireType>>,
                        paddedA2: Vec<Option<WireType>>,
                        helperBits: Vec<Option<WireType>>)  {
                        impl Instruction for Prover{
                            fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                                let mut found = false;
                                for i in (0..self.length).rev() {
                                    let v1 = evaluator.getWireValue(self.paddedA1[i].as_ref().unwrap());
                                    let v2 = evaluator.getWireValue(self.paddedA2[i].as_ref().unwrap());

                                    let check = v2 > v1 && !found;
                                    evaluator.setWireValue(
                                        self.helperBits[i].as_ref().unwrap(),
                                        if check { Util::one() } else { BigInteger::ZERO },
                                    );
                                    if check {
                                        found = true;
                                    }
                                }
                            }
                        }
            }
        );
        generator.specifyProverWitnessComputation(prover);
        // self.generator()
        //     .specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //         let mut found = false;
        //         for i in (0..length).rev() {
        //             let v1 = evaluator.getWireValue(paddedA1[i].clone().unwrap());
        //             let v2 = evaluator.getWireValue(paddedA2[i].clone().unwrap());

        //             let check = v2 > v1 && !found;
        //             evaluator.setWireValue(
        //                 helperBits[i].clone().unwrap(),
        //                 if check { Util::one() } else { BigInteger::ZERO },
        //             );
        //             if check {
        //                 found = true;
        //             }
        //         }
        //     });
        // {
        //             #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
        //             struct Prover {
        //                 pub length: usize,
        //                 pub paddedA1: Vec<Option<WireType>>,
        //                 pub paddedA2: Vec<Option<WireType>>,
        //                 pub helperBits: Vec<Option<WireType>>,
        //             }
        //             impl Instruction for Prover{
        //                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //                     let mut found = false;
        //                     for i in (0..length).rev() {
        //                         let v1 = evaluator.getWireValue(self.paddedA1[i].clone().unwrap());
        //                         let v2 = evaluator.getWireValue(self.paddedA2[i].clone().unwrap());

        //                         let check = v2 > v1 && !found;
        //                         evaluator.setWireValue(
        //                             self.helperBits[i].clone().unwrap(),
        //                             if check { Util::one() } else { BigInteger::ZERO },
        //                         );
        //                         if check {
        //                             found = true;
        //                         }
        //                     }
        //                 }
        //             }
        //             Box::new(Prover {
        //                 length,
        //                 paddedA1: paddedA1.clone(),
        //                 paddedA2: paddedA2.clone(),
        //                 helperBits: helperBits.clone(),
        //             })
        //         });

        // verify constraints about helper bits.
        for w in &helperBits {
            generator.addBinaryAssertion(w.as_ref().unwrap(), &None);
        }
        // Only one bit should be set.
        generator.addOneAssertion(
            &WireArray::new(helperBits.clone(), self.generator.clone()).sumAllElements(&None),
            &None,
        );

        // verify "the greater than condition" for the specified chunk
        let mut chunk1 = zero_wire.clone();
        let mut chunk2 = zero_wire.clone();

        for i in 0..helperBits.len() {
            chunk1 = chunk1.add(
                paddedA1[i]
                    .clone()
                    .unwrap()
                    .mul(helperBits[i].as_ref().unwrap()),
            );
            chunk2 = chunk2.add(
                paddedA2[i]
                    .clone()
                    .unwrap()
                    .mul(helperBits[i].as_ref().unwrap()),
            );
        }
        generator.addOneAssertion(
            &chunk1.isLessThan(&chunk2, Self::CHUNK_BITWIDTH, &None),
            &None,
        );

        // check that the other more significant chunks are equal
        let mut helperBits2: Vec<Option<WireType>> = vec![None; helperBits.len()];
        for i in 1..helperBits.len() {
            helperBits2[i] = helperBits2[i - 1]
                .as_ref()
                .map(|x| x.clone().add(helperBits[i - 1].as_ref().unwrap()));
            //			self.generator().addZeroAssertion(helperBits2[i].mul(paddedA1[i]
            //					.sub(paddedA2[i])));
            generator.addAssertion(
                helperBits2[i].as_ref().unwrap(),
                &paddedA1[i]
                    .clone()
                    .unwrap()
                    .sub(paddedA2[i].as_ref().unwrap()),
                &zero_wire,
                &None,
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
        if rhs.sign() == Sign::NoSign {
            return self;
        }
        if rhs.sign() == Sign::Minus {
            return self.sub(rhs.neg());
        }
        let generator = self.generator.clone();
        self.add(LongElement::newc(
            Util::split(&rhs, Self::CHUNK_BITWIDTH),
            generator,
        ))
    }
}

impl Add for LongElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let generator = self.generator();
        if self.addOverflowCheck(&rhs) {
            //println!("Warning: Addition overflow could happen");
        }

        let length = std::cmp::max(self.array.len(), rhs.array.len());
        let w1 =
            WireArray::new(self.array.clone(), self.generator.clone()).adjustLength(None, length);
        let w1 = w1.asArray();
        let w2 =
            WireArray::new(rhs.array.clone(), self.generator.clone()).adjustLength(None, length);
        let w2 = w2.asArray();
        let mut result = vec![None; length];
        let mut newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..length {
            result[i] = w1[i].clone().map(|x| x.add(w2[i].as_ref().unwrap()));
            let max1 = if i < self.array.len() {
                self.currentMaxValues[i].clone()
            } else {
                BigInteger::ZERO
            };
            let max2 = if i < rhs.array.len() {
                &rhs.currentMaxValues[i]
            } else {
                &BigInteger::ZERO
            };

            newMaxValues[i] = max1.add(max2);
        }
        LongElement::newb(result, newMaxValues, self.generator.clone())
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
        if rhs.sign() == Sign::NoSign {
            return self;
        }
        if rhs.sign() == Sign::Minus {
            return self.add(rhs.neg());
        }
        let generator = self.generator.clone();
        self.sub(LongElement::newc(
            Util::split(&rhs, Self::CHUNK_BITWIDTH),
            generator,
        ))
    }
}
impl Sub for LongElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.isAligned() && rhs.isAligned(),
            "Subtraction arguments must be properly aligned"
        );
        let mut generator = self.generator();

        let result = generator.createLongElementProverWitness(
            self.getMaxVal(Self::CHUNK_BITWIDTH).bits() as i32,
            &None,
        );
        let other = &rhs;
        let long_element = &self;
        let prover = crate::impl_prover!(
                eval( long_element:LongElement,
                        other:LongElement,
                        result:LongElement)  {
                        impl Instruction for Prover{
                            fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                               let myValue = evaluator
                                .getWireValuei(&self.long_element, LongElement::CHUNK_BITWIDTH);
                            let otherValue =
                                evaluator.getWireValuei(&self.other, LongElement::CHUNK_BITWIDTH);
                            let resultValue = myValue.sub(otherValue);
                            assert!(
                                resultValue.sign() != Sign::Minus,
                                "Result of subtraction is negative!"
                            );
                            evaluator.setWireValuebi(
                                &self.result,
                                &resultValue,
                                LongElement::CHUNK_BITWIDTH,
                            );
                            }
                        }
            }
        );
        generator.specifyProverWitnessComputation(prover);
        // self.generator()
        //     .specifyProverWitnessComputation(&|evaluator: &mut CircuitEvaluator| {
        //         let myValue = evaluator.getWireValuei(self.clone(), LongElement::CHUNK_BITWIDTH);
        //         let otherValue =
        //             evaluator.getWireValuei(other.clone(), LongElement::CHUNK_BITWIDTH);
        //         let resultValue = myValue.sub(otherValue);
        //         assert!(
        //             resultValue.sign() != Sign::Minus,
        //             "Result of subtraction is negative!"
        //         );
        //         evaluator.setWireValuebi(result.clone(), resultValue, LongElement::CHUNK_BITWIDTH);
        //     });
        // {
        // #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
        //             struct Prover<'a+Hash+Clone+Debug> {
        //                 pub long_element:LongElement<'a>,
        //                 pub other:LongElement<'a>,
        //                 pub result:LongElement<'a>,
        //             }

        //             impl <'a+Hash+Clone+Debug> Instruction for Prover<'a> {
        //                 fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
        //                     let myValue = evaluator
        //                         .getWireValuei(self.long_element.clone(), LongElement::CHUNK_BITWIDTH);
        //                     let otherValue =
        //                         evaluator.getWireValuei(self.other.clone(), LongElement::CHUNK_BITWIDTH);
        //                     let resultValue = myValue.sub(otherValue);
        //                     assert!(
        //                         resultValue.sign() != Sign::Minus,
        //                         "Result of subtraction is negative!"
        //                     );
        //                     evaluator.setWireValuebi(
        //                         self.result.clone(),
        //                         resultValue,
        //                         LongElement::CHUNK_BITWIDTH,
        //                     );
        //                 }
        //             }

        //             Box::new(Prover::<'_,C> {
        //                 long_element: self.clone(),
        //                 other: rhs.clone(),
        //                 result: result.clone(),
        //             })
        //         });
        // let generator = self.generator();
        result.restrictBitwidth();
        self.assertEquality(&result.clone().add(rhs));
        result
    }
}

impl Mul for LongElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // Implements the improved long integer multiplication from xjsnark

        if self.mulOverflowCheck(&rhs) {
            //println!("Warning: Mul overflow could happen");
        }
        let length = self.array.len() + rhs.array.len() - 1;
        let mut result: Vec<Option<WireType>>;

        // check if we can just apply the simpl e non-costly multiplication
        if rhs.array.len() == 1 || self.array.len() == 1 || self.isConstant() || rhs.isConstant() {
            result = vec![None; length];

            // O(n*m) multiplication. Fine to apply if any of the operands has
            // dim 1
            // or any of them is constant
            for i in 0..self.array.len() {
                for j in 0..rhs.array.len() {
                    result[i + j] = result[i + j].clone().map(|x| {
                        x.add(
                            self.array[i]
                                .as_ref()
                                .unwrap()
                                .clone()
                                .mul(rhs.array[j].as_ref().unwrap().clone()),
                        )
                    });
                }
            }
        } else {
            // impl ement the optimization
            let mut generator = self.generator();

            result = generator.createProverWitnessWireArray(length, &None);

            // for safety
            let (array1, array2) = (&self.array, &rhs.array);
            let prover = crate::impl_prover!(
                            eval(result: Vec<Option<WireType>>,
            array1: Vec<Option<WireType>>,
            array2: Vec<Option<WireType>>)  {
            impl Instruction for Prover{
             fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
                                       let a = evaluator.getWiresValues(&self.array1);
                                    let b = evaluator.getWiresValues(&self.array2);
                                    let resultVals = LongElement::multiplyPolys(a, b);
                                    evaluator.setWireValuea(&self.result, &resultVals);
            }
            }
                        }
                    );
            generator.specifyProverWitnessComputation(prover);
            // self.generator().specifyProverWitnessComputation(
            //     &|evaluator: &mut CircuitEvaluator| {
            //         let a = evaluator.getWiresValues(self.array.clone());
            //         let b = evaluator.getWiresValues(rhs.array.clone());
            //         let resultVals = LongElement::multiplyPolys(a, b);
            //         evaluator.setWireValuea(result.clone(), resultVals);
            //     },
            // );
            // {
            //     #[derive(Hash, Clone, Debug, ImplStructNameConfig)]
            //     struct Prover {
            //         pub result: Vec<Option<WireType>>,
            //         pub array1: Vec<Option<WireType>>,
            //         pub array2: Vec<Option<WireType>>,
            //     }
            //     impl Instruction for Prover{
            //         fn evaluate(&self, evaluator: &mut CircuitEvaluator) {
            //             let a = evaluator.getWiresValues(self.array1.clone());
            //             let b = evaluator.getWiresValues(self.array2.clone());
            //             let resultVals = LongElement::multiplyPolys(a, b);
            //             evaluator.setWireValuea(self.result.clone(), resultVals);
            //         }
            //     }
            //     Box::new(Prover {
            //         result: result.clone(),
            //         array1: self.array.clone(),
            //         array2: rhs.array.clone(),
            //     })
            // });

            for k in 0..length {
                let constant = BigInteger::from(k as u64 + 1);
                let mut coeff = Util::one();

                let mut vector1 = vec![None; self.array.len()];
                let mut vector2 = vec![None; rhs.array.len()];
                let mut vector3 = vec![None; length];
                for i in 0..length {
                    if i < self.array.len() {
                        vector1[i] = self.array[i]
                            .as_ref()
                            .map(|x| x.clone().mulb(&coeff, &None));
                    }
                    if i < rhs.array.len() {
                        vector2[i] = rhs.array[i].as_ref().map(|x| x.clone().mulb(&coeff, &None));
                    }
                    vector3[i] = result[i].clone().map(|x| x.mulb(&coeff, &None));
                    coeff = Util::modulo(&coeff.mul(&constant), &Configs.field_prime);
                }

                let v1 = WireArray::new(vector1, self.generator.clone()).sumAllElements(&None);
                let v2 = WireArray::new(vector2, self.generator.clone()).sumAllElements(&None);
                let v3 = WireArray::new(vector3, self.generator.clone()).sumAllElements(&None);
                generator.addAssertion(&v1, &v2, &v3, &None);
            }
        }

        let mut newMaxValues = vec![BigInteger::ZERO; length];
        for i in 0..self.array.len() {
            for j in 0..rhs.array.len() {
                newMaxValues[i + j] = newMaxValues[i + j].clone().add(
                    self.currentMaxValues[i]
                        .clone()
                        .mul(rhs.currentMaxValues[j].clone()),
                );
            }
        }
        LongElement::newb(result, newMaxValues, self.generator.clone())
    }
}

impl Eq for LongElement {}
impl PartialEq for LongElement {
    fn eq(&self, other: &Self) -> bool {
        // if o == null || !(o instance_of LongElement) {
        // 	return false;
        // }
        // LongElement v = (LongElement) o;
        if other.array.len() != self.array.len() {
            return false;
        }
        // let mut  check = true;
        // for i in 0.. self.array.len() {
        // 	if !v.array[i]==self.array[i] {
        // 		check = false;
        // 		break;
        // 	}
        // }
        // return check;
        self.array.iter().zip(&other.array).all(|(a, b)| a == b)
    }
}
