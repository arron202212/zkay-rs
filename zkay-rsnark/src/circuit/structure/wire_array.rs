#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::InstanceOf;
use crate::circuit::eval::instruction::Instruction;
use crate::circuit::operations::primitive::add_basic_op::{AddBasicOp, new_add};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::structure::circuit_generator::CGConfig;
use crate::circuit::structure::circuit_generator::CGConfigFieldsIQ;
use crate::circuit::structure::circuit_generator::CreateConstantWire;
use crate::circuit::structure::circuit_generator::{
    CircuitGenerator, CircuitGeneratorExtend, CircuitGeneratorIQ, getActiveCircuitGenerator,
};
use crate::circuit::structure::linear_combination_wire::{
    LinearCombinationWire, new_linear_combination,
};
use crate::circuit::structure::wire::GeneratorConfig;
use crate::circuit::structure::wire::{GetWireId, Wire, WireConfig, setBitsConfig};
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::ARcCell;
use crate::util::util::{BigInteger, Util};
use rccell::RcCell;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Index, IndexMut, Mul, Shl, Sub};
use std::sync::Arc;
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct WireArray {
    pub array: Vec<Option<WireType>>,
    pub generator: RcCell<CircuitGeneratorIQ>,
}

impl Index<usize> for WireArray {
    type Output = Option<WireType>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<usize> for WireArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[index]
    }
}

impl WireArray {
    pub fn newi(n: i32, generator: RcCell<CircuitGeneratorIQ>) -> Self {
        Self::newic(n, generator)
    }

    pub fn newic(n: i32, generator: RcCell<CircuitGeneratorIQ>) -> Self {
        WireArray::new(vec![None; n as usize], generator)
    }

    pub fn new(wireArray: Vec<Option<WireType>>, generator: RcCell<CircuitGeneratorIQ>) -> Self {
        Self {
            array: wireArray,
            generator,
        }
    }

    pub fn get(&self, i: usize) -> &Option<WireType> {
        &self.array[i]
    }

    pub fn set(&mut self, i: usize, w: WireType) {
        self.array[i] = Some(w);
    }

    pub fn size(&self) -> usize {
        self.array.len()
    }

    pub fn asArray(&self) -> Vec<Option<WireType>> {
        self.array.clone()
    }
    pub fn generator(&self) -> &RcCell<CircuitGeneratorIQ> {
        &self.generator
    }
    pub fn mulWireArray(&self, v: WireArray, desiredLength: usize, desc: &Option<String>) -> Self {
        let ws1 = self
            .adjustLength(Some(self.array.clone()), desiredLength)
            .asArray();
        let ws2 = self
            .adjustLength(Some(v.array.clone()), desiredLength)
            .asArray();
        let mut out = vec![None; desiredLength];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().mulw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn sumAllElements(&self, desc: &Option<String>) -> WireType {
        let mut generator = self.generator().clone();

        let mut allConstant = true;
        let mut sum = BigInteger::ZERO;
        for w in &self.array {
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                sum = sum.add(
                    w.as_ref()
                        .unwrap()
                        .try_as_constant_ref()
                        .unwrap()
                        .getConstant(),
                );
            } else {
                allConstant = false;
                break;
            }
        }
        if !allConstant {
            let output = WireType::LinearCombination(new_linear_combination(
                generator.get_current_wire_id(),
                None,
                generator.clone(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            let op = new_add(
                self.array.clone(),
                output.clone(),
                desc.as_ref()
                    .map_or_else(|| String::new(), |d| d.to_owned()),
            );
            //			generator.addToEvaluationQueue(Box::new(op));
            let cachedOutputs = generator.addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                generator.borrow_mut().current_wire_id -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                output
            };
        }

        generator.create_constant_wire(sum, desc)
    }

    pub fn addWireArray(&self, v: WireArray, desiredLength: usize, desc: &Option<String>) -> Self {
        let ws1 = self
            .adjustLength(Some(self.array.clone()), desiredLength)
            .asArray();
        let ws2 = self
            .adjustLength(Some(v.array.clone()), desiredLength)
            .asArray();
        let mut out = vec![None; desiredLength];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().addw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn xorWireArray(&self, v: WireArray, desiredLength: usize, desc: &Option<String>) -> Self {
        let ws1 = self
            .adjustLength(Some(self.array.clone()), desiredLength)
            .asArray();
        let ws2 = self
            .adjustLength(Some(v.array.clone()), desiredLength)
            .asArray();
        let mut out = vec![None; desiredLength];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().xorw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn xorWireArrayi(&self, v: WireArray, desc: &Option<String>) -> Self {
        assert!(self.size() == v.size());
        let ws1 = self.array.clone();
        let ws2 = v.array.clone();

        let mut out = vec![None; self.size()];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.xorw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn andWireArray(&self, v: WireArray, desiredLength: usize, desc: &Option<String>) -> Self {
        let ws1 = self
            .adjustLength(Some(self.array.clone()), desiredLength)
            .asArray();
        let ws2 = self
            .adjustLength(Some(v.array.clone()), desiredLength)
            .asArray();
        let mut out = vec![None; desiredLength];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().mulw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn orWireArray(&self, v: WireArray, desiredLength: usize, desc: &Option<String>) -> Self {
        let ws1 = self
            .adjustLength(Some(self.array.clone()), desiredLength)
            .asArray();
        let ws2 = self
            .adjustLength(Some(v.array.clone()), desiredLength)
            .asArray();
        let mut out = vec![None; desiredLength];
        for i in 0..out.len() {
            out[i] = ws1[i]
                .as_ref()
                .map(|x| x.clone().orw(ws2[i].clone().unwrap(), desc));
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn invAsBits(&self, desiredBitWidth: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator().clone();

        let mut out = vec![None; desiredBitWidth];
        for i in 0..desiredBitWidth {
            if i < self.array.len() {
                out[i] = self.array[i].clone().and_then(|x| x.clone().invAsBit(desc));
            } else {
                out[i] = generator.get_one_wire();
            }
        }
        WireArray::new(out, self.generator.clone())
    }

    pub fn adjustLength(&self, ws: Option<Vec<Option<WireType>>>, desiredLength: usize) -> Self {
        let mut generator = self.generator().clone();

        let ws = ws.as_ref().unwrap_or(&self.array);
        if ws.len() == desiredLength {
            return WireArray::new(ws.clone(), self.generator.clone());
        }
        let mut newWs = vec![generator.get_zero_wire(); desiredLength];
        newWs[..std::cmp::min(ws.len(), desiredLength)].clone_from_slice(&ws);

        WireArray::new(newWs, self.generator.clone())
    }

    // pub fn adjustLengthi(&self, desiredLength: usize) ->Self{
    //     if self.array.len() == desiredLength {
    //         self.clone()
    //     }
    //     let mut newWs = vec![None; desiredLength];
    //     newWs[..std::cmp::min(self.array.len(), desiredLength)].clone_from_slice(&self.array);
    //     if self.array.len() < desiredLength {
    //         for i in self.array.len()..desiredLength {
    //             newWs[i] = generator.get_zero_wire();
    //         }
    //     }
    //     WireArray::new(newWs,self.generator.clone())
    // }

    // pub fn packAsBitsi(&self, n: usize, desc: &Option<String>) -> WireType{
    //     self.packAsBitsii(0, n, desc)
    // }

    // pub fn packAsBits(None,None,&self, desc: &Option<String>) -> WireType{
    //     self.packAsBitsi(self.array.len(), desc)
    // }

    pub fn checkIfConstantBits(&self, desc: &Option<String>) -> Option<BigInteger> {
        let mut allConstant = true;
        let mut sum = BigInteger::ZERO;
        for i in 0..self.array.len() {
            let w = self.array[i].clone();
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                let cw = w;
                let v = cw
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .getConstant();
                if v == Util::one() {
                    sum = sum.add(v.shl(i));
                } else if !v == BigInteger::ZERO {
                    println!(
                        "Warning, one of the bit wires is constant but not binary : {}",
                        Util::getDesc(desc)
                    );
                }
            } else {
                allConstant = false;
            }
        }
        allConstant.then_some(sum)
    }

    pub fn packAsBits(
        &self,
        from: Option<usize>,
        to: Option<usize>,
        desc: &Option<String>,
    ) -> WireType {
        let mut generator = self.generator().clone();

        let (from, to) = (from.unwrap_or(0), to.unwrap_or(self.array.len()));
        assert!(
            from <= to && to <= self.array.len(),
            "Invalid bounds: from > to"
        );

        let bits = self.array[from..to].to_vec();
        let mut allConstant = true;
        let mut sum = BigInteger::ZERO;
        for i in 0..bits.len() {
            let w = bits[i].clone();
            if w.as_ref().unwrap().instance_of("ConstantWire") {
                let cw = w;
                let v = cw
                    .as_ref()
                    .unwrap()
                    .try_as_constant_ref()
                    .unwrap()
                    .getConstant();
                if v == Util::one() {
                    sum = sum.add(v.shl(i));
                } else {
                    assert!(
                        v == BigInteger::ZERO,
                        "Trying to pack non-binary constant bits : {}",
                        Util::getDesc(desc)
                    );
                }
            } else {
                allConstant = false;
            }
        }
        if !allConstant {
            let out = WireType::LinearCombination(new_linear_combination(
                generator.get_current_wire_id(),
                None,
                generator.clone(),
            ));
            generator.borrow_mut().current_wire_id += 1;
            out.setBits(Some(WireArray::new(bits.clone(), generator.clone())));
            let op = new_pack(
                bits,
                out.clone(),
                desc.as_ref()
                    .map_or_else(|| String::new(), |d| d.to_owned()),
            );
            let cachedOutputs = generator.addToEvaluationQueue(Box::new(op));
            return if let Some(cachedOutputs) = cachedOutputs {
                generator.borrow_mut().current_wire_id -= 1;
                cachedOutputs[0].clone().unwrap()
            } else {
                out
            };
        }
        generator.create_constant_wire(sum, desc)
    }

    pub fn rotateLeft(&self, numBits: usize, s: usize, desc: &Option<String>) -> Self {
        let mut bits = self
            .adjustLength(Some(self.array.clone()), numBits)
            .asArray();
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i < s {
                rotatedBits[i] = bits[i + (numBits - s)].clone();
            } else {
                rotatedBits[i] = bits[i - s].clone();
            }
        }
        WireArray::new(rotatedBits, self.generator.clone())
    }

    pub fn rotateRight(&self, numBits: usize, s: usize, desc: &Option<String>) -> Self {
        let bits = self
            .adjustLength(Some(self.array.clone()), numBits)
            .asArray();
        let mut rotatedBits = vec![None; numBits];
        for i in 0..numBits {
            if i >= numBits - s {
                rotatedBits[i] = bits[i - (numBits - s)].clone();
            } else {
                rotatedBits[i] = bits[i + s].clone();
            }
        }
        WireArray::new(rotatedBits, self.generator.clone())
    }

    pub fn shiftLeft(&self, numBits: usize, s: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator().clone();

        let bits = self
            .adjustLength(Some(self.array.clone()), numBits)
            .asArray();
        let mut shiftedBits = vec![None; numBits as usize];
        for i in 0..numBits as usize {
            if i < s as usize {
                shiftedBits[i] = generator.get_zero_wire();
            } else {
                shiftedBits[i] = bits[i - s as usize].clone();
            }
        }
        WireArray::new(shiftedBits, self.generator.clone())
    }

    pub fn shiftRight(&self, numBits: usize, s: usize, desc: &Option<String>) -> Self {
        let mut generator = self.generator().clone();

        //println!("======================{},{}",file!(),line!());
        let bits = self
            .adjustLength(Some(self.array.clone()), numBits)
            .asArray();
        let mut shiftedBits = vec![None; numBits];
        for i in 0..numBits as usize {
            if i >= numBits - s as usize {
                shiftedBits[i] = generator.get_zero_wire();
            } else {
                shiftedBits[i] = bits[i + s as usize].clone();
            }
        }
        WireArray::new(shiftedBits, self.generator.clone())
    }

    pub fn packBitsIntoWords(
        &self,
        wordBitwidth: usize,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let numWords = (self.array.len() as f64 * 1.0 / wordBitwidth as f64).ceil() as usize;
        let padded = self
            .adjustLength(Some(self.array.clone()), wordBitwidth * numWords)
            .asArray();
        let mut result = vec![None; numWords];
        for i in 0..numWords {
            result[i] = Some(
                WireArray::new(
                    padded[i * wordBitwidth..(i + 1) * wordBitwidth].to_vec(),
                    self.generator.clone(),
                )
                .packAsBits(None, None, &None),
            );
        }
        result
    }

    pub fn packWordsIntoLargerWords(
        &self,
        wordBitwidth: i32,
        numWordsPerLargerWord: i32,
        desc: &Option<String>,
    ) -> Vec<Option<WireType>> {
        let mut generator = self.generator().clone();

        let numLargerWords =
            (self.array.len() as f64 * 1.0 / numWordsPerLargerWord as f64).ceil() as usize;
        let mut result = vec![generator.get_zero_wire(); numLargerWords];
        for i in 0..self.array.len() {
            let subIndex = i % numWordsPerLargerWord as usize;
            result[i / numWordsPerLargerWord as usize] = result[i / numWordsPerLargerWord as usize]
                .as_ref()
                .map(|x| {
                    x.clone().addw(
                        self.array[i].clone().unwrap().mulb(
                            BigInteger::from(2).pow(subIndex as u32 * wordBitwidth as u32),
                            &None,
                        ),
                        &None,
                    )
                });
        }
        result
    }

    pub fn getBits(&self, bitwidth: usize, desc: &Option<String>) -> Self {
        let mut bits = vec![None; bitwidth * self.array.len()];
        let mut idx = 0;
        for i in 0..self.array.len() {
            let tmp = self.array[i]
                .as_ref()
                .unwrap()
                .getBitWiresi(bitwidth as u64, desc)
                .asArray();
            for j in 0..bitwidth {
                bits[idx] = tmp[j].clone();
                idx += 1;
            }
        }
        WireArray::new(bits, self.generator.clone())
    }
}
