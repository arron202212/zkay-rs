#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::operations::primitive::{
    add_basic_op::AddBasicOp,
    assert_basic_op::{AssertBasicOp, new_assert},
    basic_op::{BasicOp, Op},
    const_mul_basic_op::{ConstMulBasicOp, new_const_mul},
    mul_basic_op::{MulBasicOp, new_mul},
    non_zero_check_basic_op::{NonZeroCheckBasicOp, new_non_zero_check},
    or_basic_op::{OrBasicOp, new_or},
    pack_basic_op::{PackBasicOp, new_pack},
    split_basic_op::{SplitBasicOp, new_split},
    xor_basic_op::{XorBasicOp, new_xor},
};

use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum OpType {
    Add(Op<AddBasicOp>),
    Assert(Op<AssertBasicOp>),
    ConstMul(Op<ConstMulBasicOp>),
    Mul(Op<MulBasicOp>),
    NonZeroCheck(Op<NonZeroCheckBasicOp>),
    Or(Op<OrBasicOp>),
    Pack(Op<PackBasicOp>),
    Split(Op<SplitBasicOp>),
    Xor(Op<XorBasicOp>),
}
