#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::Configs,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            primitive::{
                assert_basic_op::{AssertBasicOp, new_assert},
                basic_op::BasicOp,
                mul_basic_op::{MulBasicOp, new_mul},
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::{ConstantWire, new_constant},
            variable_bit_wire::VariableBitWire,
            variable_wire::{VariableWire, new_variable},
            wire::{GetWireId, Wire, WireConfig, setBitsConfig},
            wire_type::WireType,
        },
    },
    util::{
        run_command::run_command,
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
use std::ops::{Add, Mul, Neg, Rem, Sub};
/**
 * Solves a linear system of equations over a finite field.
 *
 * Used for efficient representation of AES S-box gadget
 */

pub struct LinearSystemSolver {
    mat: Vec<Vec<BigInteger>>,
}
impl LinearSystemSolver {
    // const prime: BigInteger = Configs.field_prime.clone();
    pub fn new(mat: Vec<Vec<BigInteger>>) -> Self {
        Self { mat }
    }

    pub fn solveInPlace(&mut self) {
        // https://www.csun.edu/~panferov/math262/262_rref.pdf
        // https://www.math.purdue.edu/~shao92/documents/Algorithm%20REF.pdf
        self.guassJordan();
        self.rref();
    }

    fn guassJordan(&mut self) {
        let (numRows, numCols) = (self.mat.len(), self.mat[0].len());
        let mut rowIdx = 0;
        for colIdx in 0..numCols {
            let mut pivotRowIdx = rowIdx;
            while (pivotRowIdx < numRows && self.mat[pivotRowIdx][colIdx] == BigInteger::ZERO) {
                pivotRowIdx += 1;
            }
            if pivotRowIdx == numRows {
                rowIdx += 1;
                continue;
            }

            // swap
            self.mat.swap(pivotRowIdx, rowIdx);

            pivotRowIdx = rowIdx;

            // dividing by pivot
            let invF = Self::inverse(&self.mat[pivotRowIdx][colIdx]);
            for j in 0..numCols {
                self.mat[pivotRowIdx][j] =
                    self.mat[pivotRowIdx][j].clone().mul(&invF).rem(&Configs.field_prime);
            }

            for k in pivotRowIdx..numRows {
                let f = Self::negate(&self.mat[k][colIdx]);
                for j in 0..numCols {
                    self.mat[k][j] = self.mat[k][j].clone().add(&self.mat[pivotRowIdx][j].clone().mul(&f));
                    self.mat[k][j] = self.mat[k][j].clone().rem(&Configs.field_prime);
                }
            }
            rowIdx += 1;
        }
    }

    fn rref(&mut self) {
        let (numRows, numCols) = (self.mat.len(), self.mat[0].len());
        for rowIdx in (0..=numRows - 1).rev() {
            let mut pivotColIdx = 0;
            while (pivotColIdx < numCols && self.mat[rowIdx][pivotColIdx] == BigInteger::ZERO) {
                pivotColIdx += 1;
            }
            if pivotColIdx == numCols {
                continue;
            }

            for k in (0..=rowIdx - 1).rev() {
                let f = self.mat[k][pivotColIdx].clone();
                for j in 0..numCols {
                    self.mat[k][j] = self.mat[k][j]
                        .clone()
                        .add(&self.mat[rowIdx][j].clone().mul(&f).neg());
                    self.mat[k][j] = self.mat[k][j].clone().rem(&Configs.field_prime);
                }
            }
        }
    }

    fn negate(x: &BigInteger) -> BigInteger {
        Configs
            .field_prime
            .clone()
            .sub(x.rem(&Configs.field_prime))
            .rem(&Configs.field_prime)
    }

    fn inverse(x: &BigInteger) -> BigInteger {
        x.rem(&Configs.field_prime)
            .modinv(&Configs.field_prime)
            .unwrap()
    }
}
