pub mod alu_arithmetic;
pub mod alu_control_flow;
pub mod alu_gadget;
pub mod argument_decoder_gadget;
pub mod consistency_enforcer_gadget;
pub mod memory_masking_gadget;
pub mod tinyram_protoboard;
pub mod word_variable_gadget;

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_arithmetic::{
    ALU_add_gadget, ALU_add_gadgets, ALU_and_gadget, ALU_and_gadgets, ALU_cmov_gadget,
    ALU_cmov_gadgets, ALU_cmp_gadget, ALU_cmp_gadgets, ALU_cmps_gadget, ALU_cmps_gadgets,
    ALU_divmod_gadget, ALU_divmod_gadgets, ALU_mov_gadget, ALU_mov_gadgets, ALU_not_gadget,
    ALU_not_gadgets, ALU_or_gadget, ALU_or_gadgets, ALU_shr_shl_gadget, ALU_shr_shl_gadgets,
    ALU_smul_gadget, ALU_smul_gadgets, ALU_sub_gadget, ALU_sub_gadgets, ALU_umul_gadget,
    ALU_umul_gadgets, ALU_xor_gadget, ALU_xor_gadgets,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::alu_control_flow::{
    ALU_cjmp_gadget, ALU_cjmp_gadgets, ALU_cnjmp_gadget, ALU_cnjmp_gadgets, ALU_jmp_gadget,
    ALU_jmp_gadgets,
};
use enum_dispatch::enum_dispatch;
use rccell::RcCell;
#[enum_dispatch]
pub trait ArithmeticGadgetConfig<FieldT: FieldTConfig>: Default + Clone {
    fn generate_r1cs_constraints(&self);
    fn generate_r1cs_witness(&self);
}

#[enum_dispatch(ArithmeticGadgetConfig<Field>)]
#[derive(Clone)]
pub enum ArithmeticOps<FieldT: FieldTConfig> {
    And(ALU_and_gadgets<FieldT>),
    Or(ALU_or_gadgets<FieldT>),
    Xor(ALU_xor_gadgets<FieldT>),
    Not(ALU_not_gadgets<FieldT>),
    Add(ALU_add_gadgets<FieldT>),
    Sub(ALU_sub_gadgets<FieldT>),
    Mov(ALU_mov_gadgets<FieldT>),
    Cmov(ALU_cmov_gadgets<FieldT>),
    Cmp(ALU_cmp_gadgets<FieldT>),
    Cmps(ALU_cmps_gadgets<FieldT>),
    Umul(ALU_umul_gadgets<FieldT>),
    Smul(ALU_smul_gadgets<FieldT>),
    DivMod(ALU_divmod_gadgets<FieldT>),
    ShrShl(ALU_shr_shl_gadgets<FieldT>),
    Jmp(ALU_jmp_gadgets<FieldT>),
    Cjmp(ALU_cjmp_gadgets<FieldT>),
    Cnjmp(ALU_cnjmp_gadgets<FieldT>),
}
use ffec::FieldTConfig;
impl<FieldT: FieldTConfig> Default for ArithmeticOps<FieldT> {
    fn default() -> Self {
        Self::And(ALU_and_gadgets::<FieldT>::default())
    }
}

// impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ArithmeticOps<FieldT> {
//      fn generate_r1cs_constraints(&self) {
//      }
//     fn generate_r1cs_witness(&self) {
//     }
// }

// impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for RcCell<ArithmeticOps<FieldT>> {
//      fn generate_r1cs_constraints(&self) {
//      }
//     fn generate_r1cs_witness(&self) {
//     }
// }
