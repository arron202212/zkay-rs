#![allow(dead_code)]
//#![allow(non_snake_case)]
//#![allow(non_upper_case_globals)]
//#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    arc_cell_new,
    circuit::{
        InstanceOf, StructNameConfig,
        auxiliary::long_element::LongElement,
        config::config::CONFIGS,
        eval::{circuit_evaluator::CircuitEvaluator, instruction::Instruction},
        operations::{
            gadget::Gadget,
            primitive::{
                assert_basic_op::AssertBasicOp, basic_op::BasicOp, mul_basic_op::MulBasicOp,
            },
            wire_label_instruction::LabelType,
            wire_label_instruction::WireLabelInstruction,
        },
        structure::{
            circuit_generator::{CGConfig, CGConfigFields, CircuitGenerator},
            constant_wire::ConstantWire,
            variable_bit_wire::VariableBitWire,
            variable_wire::VariableWire,
            wire::{GetWireId, SetBitsConfig, Wire, WireConfig},
            wire_type::WireType,
        },
    },
    util::{
        util::ARcCell,
        util::{BigInteger, Util},
    },
};
use std::ops::{Add, Mul, Neg, Rem, Sub};

//  * Solves a linear system of equations over a finite field.
//  *
//  * Used for efficient representation of AES S-box gadget

pub struct LinearSystemSolver {
    pub mat: Vec<Vec<BigInteger>>,
}
impl LinearSystemSolver {
    // const prime: BigInteger = CONFIGS.field_prime.clone();
    pub fn new(mat: Vec<Vec<BigInteger>>) -> Self {
        Self { mat }
    }

    pub fn solve_in_place(&mut self) -> Vec<Vec<BigInteger>> {
        // https://www.csun.edu/~panferov/math262/262_rref.pdf
        // https://www.math.purdue.edu/~shao92/documents/Algorithm%20REF.pdf
        self.guass_jordan();
        self.rref();
        self.mat.clone()
    }

    fn guass_jordan(&mut self) {
        let (num_rows, num_cols) = (self.mat.len(), self.mat[0].len());
        let mut row_idx = 0;
        for col_idx in 0..num_cols {
            let mut pivot_row_idx = row_idx;
            while (pivot_row_idx < num_rows && self.mat[pivot_row_idx][col_idx] == BigInteger::ZERO)
            {
                pivot_row_idx += 1;
            }
            if pivot_row_idx == num_rows {
                row_idx += 1;
                continue;
            }

            // swap
            self.mat.swap(pivot_row_idx, row_idx);

            pivot_row_idx = row_idx;

            // dividing by pivot
            let inv_f = Self::inverse(&self.mat[pivot_row_idx][col_idx]);
            for j in 0..num_cols {
                self.mat[pivot_row_idx][j] = self.mat[pivot_row_idx][j]
                    .clone()
                    .mul(&inv_f)
                    .rem(&CONFIGS.field_prime);
            }

            for k in pivot_row_idx + 1..num_rows {
                let f = Self::negate(&self.mat[k][col_idx]);

                for j in 0..num_cols {
                    self.mat[k][j] = self.mat[k][j]
                        .clone()
                        .add(&self.mat[pivot_row_idx][j].clone().mul(&f));
                    let old = self.mat[k][j].clone();
                    self.mat[k][j] = self.mat[k][j].clone().rem(&CONFIGS.field_prime);
                }
            }
            row_idx += 1;
        }
    }

    fn rref(&mut self) {
        let (num_rows, num_cols) = (self.mat.len(), self.mat[0].len());
        for row_idx in (0..num_rows).rev() {
            let mut pivot_col_idx = 0;
            while (pivot_col_idx < num_cols && self.mat[row_idx][pivot_col_idx] == BigInteger::ZERO)
            {
                pivot_col_idx += 1;
            }
            if pivot_col_idx == num_cols {
                continue;
            }

            for k in (0..row_idx).rev() {
                let f = self.mat[k][pivot_col_idx].clone();
                for j in 0..num_cols {
                    self.mat[k][j] = self.mat[k][j]
                        .clone()
                        .add(Self::negate(&self.mat[row_idx][j].clone().mul(&f)));
                    self.mat[k][j] = self.mat[k][j].clone().rem(&CONFIGS.field_prime);
                }
            }
        }
    }

    fn negate(x: &BigInteger) -> BigInteger {
        CONFIGS
            .field_prime
            .clone()
            .sub(x.rem(&CONFIGS.field_prime))
            .rem(&CONFIGS.field_prime)
    }

    fn inverse(x: &BigInteger) -> BigInteger {
        x.rem(&CONFIGS.field_prime)
            .modinv(&CONFIGS.field_prime)
            .unwrap()
    }
}
