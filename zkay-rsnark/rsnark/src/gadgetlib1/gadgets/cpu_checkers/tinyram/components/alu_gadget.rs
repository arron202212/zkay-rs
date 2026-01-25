// Declaration of interfaces for the TinyRAM ALU gadget.

// The gadget checks the correct execution of a given TinyRAM instruction.

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::ArithmeticGadgetConfig;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::ArithmeticOps;
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

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    dual_variable_gadget, inner_product_gadget, loose_multiplexing_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
    tinyram_standard_gadgets,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
    word_variable_gadgets,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    tinyram_loose_multiplexing_gadget, tinyram_packing_gadget,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_opcode;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    generate_tinyram_prelude, tinyram_architecture_params, tinyram_program,
};
use crate::relations::ram_computations::rams::{
    ram_params::ArchitectureParamsTypeConfig,
    tinyram::tinyram_aux::{
        tinyram_opcodes_control_flow, tinyram_opcodes_register, tinyram_opcodes_stall,
    },
};
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use ffec::common::profiling::print_time;
use rccell::RcCell;
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct ALU_gadget<FieldT: FieldTConfig> {
    // : public tinyram_standard_gadget<FieldT>
    components: Vec<RcCell<ArithmeticOps<FieldT>>>,
    opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pc: word_variable_gadgets<FieldT>,
    desval: word_variable_gadgets<FieldT>,
    arg1val: word_variable_gadgets<FieldT>,
    arg2val: word_variable_gadgets<FieldT>,
    flag: variable<FieldT, pb_variable>,
    instruction_results: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    instruction_flags: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
}

pub type ALU_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, ALU_gadget<FieldT>>>,
>;
impl<FieldT: FieldTConfig> SubTinyRamGadgetConfig for ALU_gadget<FieldT> {}

impl<FieldT: FieldTConfig> ALU_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        pc: word_variable_gadgets<FieldT>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        instruction_results: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        instruction_flags: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        annotation_prefix: String,
    ) -> ALU_gadgets<FieldT> {
        let mut components =
            vec![RcCell::new(Default::default()); 1usize << pb.borrow().t.ap.opcode_width()];
        /* arithmetic */
        components[tinyram_opcode::tinyram_opcode_AND.usize()] =
            RcCell::new(ArithmeticOps::And(ALU_and_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_AND.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_AND.usize()].clone(),
                format!("{} AND", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_OR.usize()] =
            RcCell::new(ArithmeticOps::Or(ALU_or_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_OR.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_OR.usize()].clone(),
                format!("{} OR", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_XOR.usize()] =
            RcCell::new(ArithmeticOps::Xor(ALU_xor_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_XOR.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_XOR.usize()].clone(),
                format!("{} XOR", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_NOT.usize()] =
            RcCell::new(ArithmeticOps::Not(ALU_not_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_NOT.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_NOT.usize()].clone(),
                format!("{} NOT", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_ADD.usize()] =
            RcCell::new(ArithmeticOps::Add(ALU_add_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_ADD.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_ADD.usize()].clone(),
                format!("{} ADD", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_SUB.usize()] =
            RcCell::new(ArithmeticOps::Sub(ALU_sub_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_SUB.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_SUB.usize()].clone(),
                format!("{} SUB", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_MOV.usize()] =
            RcCell::new(ArithmeticOps::Mov(ALU_mov_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_MOV.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_MOV.usize()].clone(),
                format!("{} MOV", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_CMOV.usize()] =
            RcCell::new(ArithmeticOps::Cmov(ALU_cmov_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMOV.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMOV.usize()].clone(),
                format!("{} CMOV", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_CMPA.usize()] =
            RcCell::new(ArithmeticOps::Cmp(ALU_cmp_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMPE.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMPE.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMPA.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMPA.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMPAE.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMPAE.usize()].clone(),
                format!("{} CMP_unsigned", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_CMPG.usize()] =
            RcCell::new(ArithmeticOps::Cmps(ALU_cmps_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMPG.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMPG.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CMPGE.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_CMPGE.usize()].clone(),
                format!("{} CMP_signed", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_UMULH.usize()] =
            RcCell::new(ArithmeticOps::Umul(ALU_umul_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_MULL.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_MULL.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_UMULH.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_UMULH.usize()].clone(),
                format!("{} MUL_unsigned", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_SMULH.usize()] =
            RcCell::new(ArithmeticOps::Smul(ALU_smul_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_SMULH.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_SMULH.usize()].clone(),
                format!("{} MUL_signed", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_UDIV.usize()] =
            RcCell::new(ArithmeticOps::DivMod(ALU_divmod_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_UDIV.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_UDIV.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_UMOD.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_UMOD.usize()].clone(),
                format!("{} DIV", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_SHR.usize()] =
            RcCell::new(ArithmeticOps::ShrShl(ALU_shr_shl_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_SHR.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_SHR.usize()].clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_SHL.usize()].clone(),
                instruction_flags[tinyram_opcode::tinyram_opcode_SHL.usize()].clone(),
                format!("{} SHR_SHL", annotation_prefix),
            )));

        /* control flow */
        components[tinyram_opcode::tinyram_opcode_JMP.usize()] =
            RcCell::new(ArithmeticOps::Jmp(ALU_jmp_gadget::<FieldT>::new(
                pb.clone(),
                pc.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_JMP.usize()].clone(),
                format!("{} JMP", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_CJMP.usize()] =
            RcCell::new(ArithmeticOps::Cjmp(ALU_cjmp_gadget::<FieldT>::new(
                pb.clone(),
                pc.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CJMP.usize()].clone(),
                format!("{} CJMP", annotation_prefix),
            )));

        components[tinyram_opcode::tinyram_opcode_CNJMP.usize()] =
            RcCell::new(ArithmeticOps::Cnjmp(ALU_cnjmp_gadget::<FieldT>::new(
                pb.clone(),
                pc.clone(),
                arg2val.clone(),
                flag.clone(),
                instruction_results[tinyram_opcode::tinyram_opcode_CNJMP.usize()].clone(),
                format!("{} CNJMP", annotation_prefix),
            )));

        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                components,
                opcode_indicators,
                pc,
                desval,
                arg1val,
                arg2val,
                flag,
                instruction_results,
                instruction_flags,
            },
        )
    }

    // pub fn  generate_r1cs_constraints();

    // pub fn  generate_r1cs_witness();
}

impl<FieldT: FieldTConfig> ALU_gadgets<FieldT> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..1usize << self.pb.borrow().t.ap.opcode_width() {
            // if self.t.t.t.components[i] {
            self.t.t.t.components[i]
                .borrow()
                .generate_r1cs_constraints();
            // }
        }
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..1usize << self.pb.borrow().t.ap.opcode_width() {
            // if self.t.t.t.components[i] {
            self.t.t.t.components[i].borrow().generate_r1cs_witness();
            // }
        }
    }
}
