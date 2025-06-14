#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::circuit_generator::{CircuitGenerator, getActiveCircuitGenerator};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::util::util::{BigInteger, Util};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Add, Mul, Neg, Rem, Sub};
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct SHA256Gadget {
    unpaddedInputs: Vec<Option<WireType>>,

    bitWidthPerInputElement: usize,
    totalLengthInBytes: usize,

    numBlocks: usize,
    binaryOutput: bool,
    paddingRequired: bool,

    preparedInputBits: Vec<Option<WireType>>,
    output: Vec<Option<WireType>>,
}
impl SHA256Gadget {
    const H: [i64; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    const K: [i64; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    pub fn new(
        ins: Vec<Option<WireType>>,
        bitWidthPerInputElement: usize,
        totalLengthInBytes: usize,
        binaryOutput: bool,
        paddingRequired: bool,
        desc: &Option<String>,
    ) -> Self {
        // super(desc);
        assert!(
            totalLengthInBytes * 8 <= ins.len() * bitWidthPerInputElement
                && totalLengthInBytes * 8 >= (ins.len() - 1) * bitWidthPerInputElement,
            "Inconsistent Length Information"
        );

        assert!(
            paddingRequired
                || totalLengthInBytes % 64 == 0
                || ins.len() * bitWidthPerInputElement == totalLengthInBytes,
            "When padding is not forced, totalLengthInBytes % 64 must be zero."
        );

        let mut _self = Self {
            unpaddedInputs: ins,
            bitWidthPerInputElement,
            totalLengthInBytes,
            numBlocks: 0,
            binaryOutput,
            paddingRequired,
            preparedInputBits: vec![],
            output: vec![],
        };
        _self.buildCircuit();
        _self
    }

    fn buildCircuit(&mut self) {
        let mut generator = getActiveCircuitGenerator().unwrap();
        let mut generator = generator.lock();
        // pad if needed
        self.prepare();

        let mut outDigest = vec![None; 8];
        let mut hWires = vec![None; Self::H.len()];
        for i in 0..Self::H.len() {
            hWires[i] = Some(generator.createConstantWirei(Self::H[i], &None));
        }

        for blockNum in 0..self.numBlocks {
            let mut wsSplitted = vec![vec![]; 64];
            let mut w = vec![None; 64];

            for i in 0..64 {
                if i < 16 {
                    wsSplitted[i] = Util::reverseBytes(
                        self.preparedInputBits
                            [blockNum * 512 + i * 32..blockNum * 512 + (i + 1) * 32]
                            .to_vec(),
                    );

                    w[i] = Some(WireArray::new(wsSplitted[i].clone()).packAsBits(
                        None,
                        Some(32),
                        &None,
                    ));
                } else {
                    let t1 = w[i - 15].as_ref().unwrap().rotateRight(32, 7, &None);
                    let t2 = w[i - 15].as_ref().unwrap().rotateRight(32, 18, &None);
                    let t3 = w[i - 15].as_ref().unwrap().shiftRight(32, 3, &None);
                    let mut s0 = t1.xorBitwise(t2, 32, &None);
                    s0 = s0.xorBitwise(t3, 32, &None);

                    let t4 = w[i - 2].as_ref().unwrap().rotateRight(32, 17, &None);
                    let t5 = w[i - 2].as_ref().unwrap().rotateRight(32, 19, &None);
                    let t6 = w[i - 2].as_ref().unwrap().shiftRight(32, 10, &None);
                    let mut s1 = t4.xorBitwise(t5, 32, &None);
                    s1 = s1.xorBitwise(t6, 32, &None);

                    w[i] = w[i - 16]
                        .as_ref()
                        .map(|x| x.clone().add(w[i - 7].clone().unwrap()));
                    w[i] = w[i].as_ref().map(|x| x.clone().add(s0).add(s1));
                    w[i] = w[i].as_ref().map(|x| x.clone().trimBits(34, 32, &None));
                }
            }

            let mut a = hWires[0].clone().unwrap();
            let mut b = hWires[1].clone().unwrap();
            let mut c = hWires[2].clone().unwrap();
            let mut d = hWires[3].clone().unwrap();
            let mut e = hWires[4].clone().unwrap();
            let mut f = hWires[5].clone().unwrap();
            let mut g = hWires[6].clone().unwrap();
            let mut h = hWires[7].clone().unwrap();

            for i in 0..64 {
                let t1 = e.rotateRight(32, 6, &None);
                let t2 = e.rotateRight(32, 11, &None);
                let t3 = e.rotateRight(32, 25, &None);
                let mut s1 = t1.xorBitwise(t2, 32, &None);
                s1 = s1.xorBitwise(t3, 32, &None);

                let ch = Self::computeCh(e.clone(), f.clone(), g.clone(), 32);

                let t4 = a.rotateRight(32, 2, &None);
                let t5 = a.rotateRight(32, 13, &None);
                let t6 = a.rotateRight(32, 22, &None);
                let mut s0 = t4.xorBitwise(t5, 32, &None);
                s0 = s0.xorBitwise(t6, 32, &None);

                let mut maj;
                // since after each iteration, SHA256 does c = b; and b = a;, we can make use of that to save multiplications in maj computation.
                // To do this, we make use of the caching feature, by just changing the order of wires sent to maj(). Caching will take care of the rest.
                if i % 2 == 1 {
                    maj = Self::computeMaj(c.clone(), b.clone(), a.clone(), 32);
                } else {
                    maj = Self::computeMaj(a.clone(), b.clone(), c.clone(), 32);
                }

                let temp1 = w[i]
                    .clone()
                    .unwrap()
                    .addi(Self::K[i], &None)
                    .add(s1)
                    .add(h.clone())
                    .add(ch);

                let temp2 = maj.add(s0);

                h = g;
                g = f;
                f = e;
                e = temp1.clone().add(d);
                e = e.trimBits(35, 32, &None);

                d = c;
                c = b;
                b = a;
                a = temp2.add(temp1);
                a = a.trimBits(35, 32, &None);
            }

            hWires[0] = hWires[0]
                .clone()
                .map(|x| x.add(a.clone()).trimBits(33, 32, &None));
            hWires[1] = hWires[1]
                .clone()
                .map(|x| x.add(b.clone()).trimBits(33, 32, &None));
            hWires[2] = hWires[2]
                .clone()
                .map(|x| x.add(c.clone()).trimBits(33, 32, &None));
            hWires[3] = hWires[3]
                .clone()
                .map(|x| x.add(d.clone()).trimBits(33, 32, &None));
            hWires[4] = hWires[4]
                .clone()
                .map(|x| x.add(e.clone()).trimBits(33, 32, &None));
            hWires[5] = hWires[5]
                .clone()
                .map(|x| x.add(f.clone()).trimBits(33, 32, &None));
            hWires[6] = hWires[6]
                .clone()
                .map(|x| x.add(g.clone()).trimBits(33, 32, &None));
            hWires[7] = hWires[7]
                .clone()
                .map(|x| x.add(h.clone()).trimBits(33, 32, &None));
        }

        outDigest[0] = hWires[0].clone();
        outDigest[1] = hWires[1].clone();
        outDigest[2] = hWires[2].clone();
        outDigest[3] = hWires[3].clone();
        outDigest[4] = hWires[4].clone();
        outDigest[5] = hWires[5].clone();
        outDigest[6] = hWires[6].clone();
        outDigest[7] = hWires[7].clone();

        if !self.binaryOutput {
            self.output = outDigest;
            return;
        }
        self.output = vec![None; 8 * 32];
        for i in 0..8 {
            let bits = outDigest[i]
                .as_ref()
                .unwrap()
                .getBitWiresi(32, &None)
                .asArray();
            for j in 0..32 {
                self.output[j + i * 32] = bits[j].clone();
            }
        }
    }

    fn computeMaj(a: WireType, b: WireType, c: WireType, numBits: usize) -> WireType {
        let mut result = vec![None; numBits];
        let aBits = a.getBitWiresi(numBits as u64, &None).asArray();
        let bBits = b.getBitWiresi(numBits as u64, &None).asArray();
        let cBits = c.getBitWiresi(numBits as u64, &None).asArray();

        for i in 0..numBits {
            let t1 = aBits[i].clone().unwrap().mul(bBits[i].clone().unwrap());
            let t2 = aBits[i]
                .clone()
                .unwrap()
                .add(bBits[i].clone().unwrap())
                .add(t1.muli(-2, &None));
            result[i] = Some(t1.add(cBits[i].clone().unwrap().mul(t2)));
        }
        return WireArray::new(result).packAsBits(None, None, &None);
    }

    fn computeCh(a: WireType, b: WireType, c: WireType, numBits: usize) -> WireType {
        let mut result = vec![None; numBits];

        let aBits = a.getBitWiresi(numBits as u64, &None).asArray();
        let bBits = b.getBitWiresi(numBits as u64, &None).asArray();
        let cBits = c.getBitWiresi(numBits as u64, &None).asArray();

        for i in 0..numBits {
            let t1 = bBits[i].clone().unwrap().sub(cBits[i].clone().unwrap());
            let t2 = t1.mul(aBits[i].clone().unwrap());
            result[i] = Some(t2.add(cBits[i].clone().unwrap()));
        }
        return WireArray::new(result).packAsBits(None, None, &None);
    }

    fn prepare(&mut self) {
        let mut generator = getActiveCircuitGenerator().unwrap();
        let mut generator = generator.lock();
        self.numBlocks = (self.totalLengthInBytes as f64 * 1.0 / 64.0).ceil() as usize;
        let bits = WireArray::new(self.unpaddedInputs.clone())
            .getBits(self.bitWidthPerInputElement, &None)
            .asArray();
        let tailLength = self.totalLengthInBytes % 64;
        if self.paddingRequired {
            let mut pad;
            if 64 - tailLength >= 9 {
                pad = vec![None; 64 - tailLength];
            } else {
                pad = vec![None; 128 - tailLength];
            }
            self.numBlocks = (self.totalLengthInBytes + pad.len()) / 64;
            pad[0] = Some(generator.createConstantWirei(0x80, &None));
            for i in 1..pad.len() - 8 {
                pad[i] = generator.get_zero_wire();
            }
            let lengthInBits = self.totalLengthInBytes * 8;
            let mut lengthBits = vec![None; 64];
            let pn = pad.len();
            for i in 0..8 {
                pad[pn - 1 - i] = Some(
                    generator.createConstantWirei(((lengthInBits >> (8 * i)) & 0xFF) as i64, &None),
                );
                let tmp = pad[pn - 1 - i]
                    .as_ref()
                    .unwrap()
                    .getBitWiresi(8, &None)
                    .asArray();
                lengthBits[(7 - i) * 8..(7 - i + 1) * 8].clone_from_slice(&tmp);
            }
            let totalNumberOfBits = self.numBlocks * 512;
            self.preparedInputBits = vec![generator.get_zero_wire(); totalNumberOfBits];
            self.preparedInputBits[..self.totalLengthInBytes * 8].clone_from_slice(&bits);
            self.preparedInputBits[self.totalLengthInBytes * 8 + 7] = generator.get_one_wire();
            let n = self.preparedInputBits.len();
            self.preparedInputBits[n - 64..].clone_from_slice(&lengthBits);
        } else {
            self.preparedInputBits = bits;
        }
    }
}
impl GadgetConfig for SHA256Gadget {
    /**
     * outputs digest as 32-bit words
     */

    fn getOutputWires(&self) -> Vec<Option<WireType>> {
        return self.output.clone();
    }
}
