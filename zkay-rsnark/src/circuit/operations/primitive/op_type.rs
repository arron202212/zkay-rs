#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use crate::circuit::operations::primitive::add_basic_op::AddBasicOp;
use crate::circuit::operations::primitive::assert_basic_op::{AssertBasicOp, new_assert};
use crate::circuit::operations::primitive::basic_op::BasicOp;
use crate::circuit::operations::primitive::basic_op::Op;
use crate::circuit::operations::primitive::const_mul_basic_op::{ConstMulBasicOp, new_const_mul};
use crate::circuit::operations::primitive::mul_basic_op::{MulBasicOp, new_mul};
use crate::circuit::operations::primitive::non_zero_check_basic_op::{
    NonZeroCheckBasicOp, new_non_zero_check,
};
use crate::circuit::operations::primitive::or_basic_op::{OrBasicOp, new_or};
use crate::circuit::operations::primitive::pack_basic_op::{PackBasicOp, new_pack};
use crate::circuit::operations::primitive::split_basic_op::{SplitBasicOp, new_split};
use crate::circuit::operations::primitive::xor_basic_op::{XorBasicOp, new_xor};
use crate::circuit::structure::circuit_generator::CGConfig;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
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
