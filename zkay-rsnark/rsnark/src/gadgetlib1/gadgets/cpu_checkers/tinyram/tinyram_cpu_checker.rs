// Declaration of interfaces for the TinyRAM CPU checker gadget.

// The gadget checks the correct operation for the CPU of the TinyRAM architecture.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    DefaultDualVariableGadget, dual_variable_gadget, dual_variable_gadgets,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::{
    ArithmeticGadgetConfig,
    alu_gadget::{ALU_gadget, ALU_gadgets},
    argument_decoder_gadget::{argument_decoder_gadget, argument_decoder_gadgets},
    consistency_enforcer_gadget::{consistency_enforcer_gadget, consistency_enforcer_gadgets},
    memory_masking_gadget::{memory_masking_gadget, memory_masking_gadgets},
    word_variable_gadget::{
        doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
        word_variable_gadgets,
    },
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{protoboard,PBConfig,ProtoboardConfig};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::memory::memory_interface;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    tinyram_opcode, tinyram_opcode_names,
};
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::serialization;
use rccell::RcCell;
use std::collections::HashMap;
#[derive(Clone, Default)]
pub struct tinyram_cpu_checker<FieldT: FieldTConfig> {
    // : public tinyram_standard_gadget<FieldT>
    opcode: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    arg2_is_imm: variable<FieldT, pb_variable>,
    desidx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    arg1idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    arg2idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    prev_registers: Vec<word_variable_gadgets<FieldT>>,
    next_registers: Vec<word_variable_gadgets<FieldT>>,
    prev_flag: variable<FieldT, pb_variable>,
    next_flag: variable<FieldT, pb_variable>,
    prev_tape1_exhausted: variable<FieldT, pb_variable>,
    next_tape1_exhausted: variable<FieldT, pb_variable>,
    prev_pc_addr_as_word_variable: RcCell<word_variable_gadgets<FieldT>>,
    desval: RcCell<word_variable_gadgets<FieldT>>,
    arg1val: RcCell<word_variable_gadgets<FieldT>>,
    arg2val: RcCell<word_variable_gadgets<FieldT>>,
    decode_arguments: RcCell<argument_decoder_gadgets<FieldT>>,
    opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    ALU: RcCell<ALU_gadgets<FieldT>>,
    ls_prev_val_as_doubleword_variable: RcCell<doubleword_variable_gadgets<FieldT>>,
    ls_next_val_as_doubleword_variable: RcCell<doubleword_variable_gadgets<FieldT>>,
    memory_subaddress: RcCell<word_variable_gadgets<FieldT>>,
    memory_subcontents: variable<FieldT, pb_variable>,
    memory_access_is_word: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    memory_access_is_byte: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    check_memory: RcCell<memory_masking_gadgets<FieldT>>,
    next_pc_addr_as_word_variable: RcCell<word_variable_gadgets<FieldT>>,
    consistency_enforcer: RcCell<consistency_enforcer_gadgets<FieldT>>,
    instruction_results: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    instruction_flags: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    read_not1: variable<FieldT, pb_variable>,
    prev_pc_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    prev_pc_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    prev_state: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    ls_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    ls_prev_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    ls_next_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    next_state: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    next_pc_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    next_has_accepted: variable<FieldT, pb_variable>,
    // tinyram_cpu_checker(tinyram_protoboard<FieldT> &pb,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &prev_pc_addr,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &prev_pc_val,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &prev_state,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &ls_addr,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &ls_prev_val,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &ls_next_val,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &next_state,
    //                     pb_variable_array<FieldT, tinyram_protoboard<FieldT>> &next_pc_addr,
    //                     variable<FieldT, pb_variable> &next_has_accepted,
    //                     annotation_prefix:&String);

    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness() { assert!(0); }
    // pub fn  generate_r1cs_witness_address();
    // pub fn  generate_r1cs_witness_other(tinyram_input_tape_iterator &aux_it,
    //                                  aux_end:&tinyram_input_tape_iterator);
    // pub fn  dump() const;
}

// use crate::gadgetlib1::gadgets::cpu_checkers/tinyram/tinyram_cpu_checker;

//#endif // TINYRAM_CPU_CHECKER_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for the TinyRAM CPU checker gadget.

See tinyram_cpu_checker.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TINYRAM_CPU_CHECKER_TCC_
// #define TINYRAM_CPU_CHECKER_TCC_
use ffec::algebra::field_utils::field_utils;
pub type tinyram_cpu_checkers<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, tinyram_cpu_checker<FieldT>>>,
>;
impl<FieldT: FieldTConfig> SubTinyRamGadgetConfig for tinyram_cpu_checker<FieldT> {}
impl<FieldT: FieldTConfig> tinyram_cpu_checker<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        prev_pc_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        prev_pc_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        prev_state: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        ls_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        ls_prev_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        ls_next_val: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        next_state: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        next_pc_addr: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        next_has_accepted: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> tinyram_cpu_checkers<FieldT> {
        /* parse previous PC value as an instruction (note that we start
        parsing from LSB of the instruction doubleword and go to the
        MSB) */
        let prev_pc_val_contents = &prev_pc_val.contents;
        let mut pc_val_it = 0;

        let arg2idx = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            prev_pc_val_contents[pc_val_it..pc_val_it + pb.borrow().t.ap.reg_arg_or_imm_width()]
                .to_vec(),
        );
        pc_val_it += pb.borrow().t.ap.reg_arg_or_imm_width();
        pc_val_it += pb.borrow().t.ap.instruction_padding_width();
        let arg1idx = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            prev_pc_val_contents[pc_val_it..pc_val_it + pb.borrow().t.ap.reg_arg_width()].to_vec(),
        );
        pc_val_it += pb.borrow().t.ap.reg_arg_width();
        let desidx = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            prev_pc_val_contents[pc_val_it..pc_val_it + pb.borrow().t.ap.reg_arg_width()].to_vec(),
        );
        pc_val_it += pb.borrow().t.ap.reg_arg_width();
        let arg2_is_imm = variable::<FieldT, pb_variable>::from(pc_val_it);
        pc_val_it += 1;
        let opcode = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            prev_pc_val_contents[pc_val_it..pc_val_it + pb.borrow().t.ap.opcode_width()].to_vec(),
        );
        pc_val_it += pb.borrow().t.ap.opcode_width();

        assert!(pc_val_it == prev_pc_val_contents.len());

        /* parse state as registers + flags */
        let (mut packed_prev_registers, mut packed_next_registers) = (
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default(),
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default(),
        );
        let (mut prev_registers, mut next_registers) = (vec![], vec![]);
        for i in 0..pb.borrow().t.ap.k {
            prev_registers.push(word_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    prev_state.contents[i * pb.borrow().t.ap.w..(i + 1) * pb.borrow().t.ap.w]
                        .to_vec(),
                ),
                format!("{annotation_prefix} prev_registers_{}", i),
            ));
            next_registers.push(word_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    next_state.contents[i * pb.borrow().t.ap.w..(i + 1) * pb.borrow().t.ap.w]
                        .to_vec(),
                ),
                format!("{annotation_prefix} next_registers_{}", i),
            ));

            packed_prev_registers
                .contents
                .push(prev_registers[i].t.packed.clone());
            packed_next_registers
                .contents
                .push(next_registers[i].t.packed.clone());
        }
        let prev_flag = prev_state.contents[prev_state.contents.len() - 2].clone();
        let next_flag = next_state.contents[next_state.contents.len() - 2].clone();
        let prev_tape1_exhausted = prev_state.iter().last().unwrap().clone();
        let next_tape1_exhausted = next_state.iter().last().unwrap().clone();

        /* decode arguments */
        let prev_pc_addr_as_word_variable =
            RcCell::new(word_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                prev_pc_addr.clone(),
                format!("{annotation_prefix} prev_pc_addr_as_word_variable"),
            ));
        let desval = RcCell::new(word_variable_gadget::<FieldT>::new(
            pb.clone(),
            format!("{annotation_prefix} desval"),
        ));
        let arg1val = RcCell::new(word_variable_gadget::<FieldT>::new(
            pb.clone(),
            format!("{annotation_prefix} arg1val"),
        ));
        let arg2val = RcCell::new(word_variable_gadget::<FieldT>::new(
            pb.clone(),
            format!("{annotation_prefix} arg2val"),
        ));

        let decode_arguments = RcCell::new(argument_decoder_gadget::<FieldT>::new(
            pb.clone(),
            arg2_is_imm.clone(),
            desidx.clone(),
            arg1idx.clone(),
            arg2idx.clone(),
            packed_prev_registers.clone(),
            desval.borrow().t.packed.clone(),
            arg1val.borrow().t.packed.clone(),
            arg2val.borrow().t.packed.clone(),
            format!("{annotation_prefix} decode_arguments"),
        ));

        /* create indicator variables for opcodes */
        let mut opcode_indicators =
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        opcode_indicators.allocate(
            &pb,
            1usize << pb.borrow().t.ap.opcode_width(),
            format!("{annotation_prefix} opcode_indicators"),
        );

        /* perform the ALU operations */
        let mut instruction_results =
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        instruction_results.allocate(
            &pb,
            1usize << pb.borrow().t.ap.opcode_width(),
            format!("{annotation_prefix} instruction_results"),
        );
        let mut instruction_flags =
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        instruction_flags.allocate(
            &pb,
            1usize << pb.borrow().t.ap.opcode_width(),
            format!("{annotation_prefix} instruction_flags"),
        );

        let ALU = RcCell::new(ALU_gadget::<FieldT>::new(
            pb.clone(),
            opcode_indicators.clone(),
            prev_pc_addr_as_word_variable.borrow().clone(),
            desval.borrow().clone(),
            arg1val.borrow().clone(),
            arg2val.borrow().clone(),
            prev_flag.clone(),
            instruction_results.clone(),
            instruction_flags.clone(),
            format!("{annotation_prefix} ALU"),
        ));

        /* check correctness of memory operations */
        let ls_prev_val_as_doubleword_variable =
            RcCell::new(doubleword_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                ls_prev_val.clone(),
                format!("{annotation_prefix} ls_prev_val_as_doubleword_variable"),
            ));
        let ls_next_val_as_doubleword_variable =
            RcCell::new(doubleword_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                ls_next_val.clone(),
                format!("{annotation_prefix} ls_next_val_as_doubleword_variable"),
            ));
        let memory_subaddress = RcCell::new(word_variable_gadget::<FieldT>::new_with_bits(
            pb.clone(),
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                arg2val.borrow().t.bits.contents[..pb.borrow().t.ap.subaddr_len()].to_vec(),
            ),
            format!("{annotation_prefix} memory_subaddress"),
        ));
        let mut memory_subcontents = variable::<FieldT, pb_variable>::default();
        memory_subcontents.allocate(&pb, format!("{annotation_prefix} memory_subcontents"));
        let mut memory_access_is_word =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        memory_access_is_word.assign(
            &pb,
            &(linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                FieldT::from(1),
            ) - (opcode_indicators[tinyram_opcode::tinyram_opcode_LOADB.clone() as usize]
                .clone()
                + opcode_indicators[tinyram_opcode::tinyram_opcode_STOREB.clone() as usize]
                    .clone()
                    .into())),
        );
        let mut memory_access_is_byte =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        memory_access_is_byte.assign(
            &pb,
            &(opcode_indicators[tinyram_opcode::tinyram_opcode_LOADB.clone() as usize].clone()
                + opcode_indicators[tinyram_opcode::tinyram_opcode_STOREB.clone() as usize]
                    .clone()
                    .into()),
        );

        let check_memory = RcCell::new(memory_masking_gadget::<FieldT>::new(
            pb.clone(),
            ls_prev_val_as_doubleword_variable.borrow().clone(),
            memory_subaddress.borrow().clone(),
            memory_subcontents.clone().into(),
            memory_access_is_word.clone(),
            memory_access_is_byte.clone(),
            ls_next_val_as_doubleword_variable.borrow().clone(),
            format!("{annotation_prefix} check_memory"),
        ));

        /* handle reads */
        let mut read_not1 = variable::<FieldT, pb_variable>::default();
        read_not1.allocate(&pb, format!("{annotation_prefix} read_not1"));

        /* check consistency of the states according to the ALU results */
        let next_pc_addr_as_word_variable =
            RcCell::new(word_variable_gadget::<FieldT>::new_with_bits(
                pb.clone(),
                next_pc_addr.clone(),
                format!("{annotation_prefix} next_pc_addr_as_word_variable"),
            ));

        let consistency_enforcer = RcCell::new(consistency_enforcer_gadget::<FieldT>::new(
            pb.clone(),
            opcode_indicators.clone(),
            instruction_results.clone(),
            instruction_flags.clone(),
            desidx.clone(),
            prev_pc_addr_as_word_variable.borrow().t.packed.clone(),
            packed_prev_registers.clone(),
            desval.borrow().t.packed.clone(),
            prev_flag.clone(),
            next_pc_addr_as_word_variable.borrow().t.packed.clone(),
            packed_next_registers.clone(),
            next_flag.clone(),
            format!("{annotation_prefix} consistency_enforcer"),
        ));

        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                opcode,
                arg2_is_imm,
                desidx,
                arg1idx,
                arg2idx,
                prev_registers,
                next_registers,
                prev_flag,
                next_flag,
                prev_tape1_exhausted,
                next_tape1_exhausted,
                prev_pc_addr_as_word_variable,
                desval,
                arg1val,
                arg2val,
                decode_arguments,
                opcode_indicators,
                ALU,
                ls_prev_val_as_doubleword_variable,
                ls_next_val_as_doubleword_variable,
                memory_subaddress,
                memory_subcontents,
                memory_access_is_word,
                memory_access_is_byte,
                check_memory,
                next_pc_addr_as_word_variable,
                consistency_enforcer,
                instruction_results,
                instruction_flags,
                read_not1,
                prev_pc_addr,
                prev_pc_val,
                prev_state,
                ls_addr,
                ls_prev_val,
                ls_next_val,
                next_state,
                next_pc_addr,
                next_has_accepted,
            },
        )
    }
}

impl<FieldT: FieldTConfig> tinyram_cpu_checkers<FieldT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .t
            .t
            .decode_arguments
            .borrow()
            .generate_r1cs_constraints();

        /* generate indicator variables for opcode */
        for i in 0..1usize << self.pb.borrow().t.ap.opcode_width() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.t.t.opcode_indicators[i].clone().into(),
                    pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                        &(self.t.t.t.opcode.clone().into()),
                    ) - variable::<FieldT, pb_variable>::from(i),
                    FieldT::from(0).into(),
                ),
                format!("{} opcode_indicators_{}", self.annotation_prefix, i),
            );
        }
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                pb_sum::<FieldT, tinyram_protoboard<FieldT>, pb_variable>(
                    &(self.t.t.t.opcode_indicators.clone().into()),
                ),
                FieldT::from(1).into(),
            ),
            format!("{} opcode_indicators_sum_to_1", self.annotation_prefix),
        );

        /* consistency checks for repacked variables */
        for i in 0..self.pb.borrow().t.ap.k {
            self.t.t.t.prev_registers[i].generate_r1cs_constraints(true);
            self.t.t.t.next_registers[i].generate_r1cs_constraints(true);
        }
        self.t
            .t
            .t
            .prev_pc_addr_as_word_variable
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .next_pc_addr_as_word_variable
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .ls_prev_val_as_doubleword_variable
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .ls_next_val_as_doubleword_variable
            .borrow()
            .generate_r1cs_constraints(true);

        /* main consistency checks */
        self.t
            .t
            .t
            .decode_arguments
            .borrow()
            .generate_r1cs_constraints();
        self.t.t.t.ALU.borrow().generate_r1cs_constraints();
        self.t
            .t
            .t
            .consistency_enforcer
            .borrow()
            .generate_r1cs_constraints();

        /* check correct access to memory */
        self.t
            .t
            .t
            .ls_prev_val_as_doubleword_variable
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .ls_next_val_as_doubleword_variable
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .memory_subaddress
            .borrow()
            .generate_r1cs_constraints(false);
        self.t.t.t.check_memory.borrow().generate_r1cs_constraints();

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &(pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                        self.t.t.t.arg2val.borrow().t.bits.contents
                            [self.pb.borrow().t.ap.subaddr_len()..]
                            .to_vec(),
                    )
                    .into()),
                ),
                pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &(self.t.t.t.ls_addr.clone().into()),
                ),
            ),
            format!(
                "{} ls_addr_is_arg2val_minus_subaddress",
                self.annotation_prefix
            ),
        );

        /* We require that if opcode is one of load.{b,w}, then
        subcontents is appropriately stored in instruction_results. If
        opcode is store.b we only take the necessary portion of arg1val
        (i.e. last byte), and take entire arg1val for store.w.

        Note that ls_addr is *always* going to be arg2val. If the
        instruction is a non-memory instruction, we will treat it as a
        load from that memory location. */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_LOADB.clone() as usize]
                    .clone()
                    .into(),
                (linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.memory_subcontents.clone(),
                ) - self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_LOADB.clone() as usize]
                    .clone())
                .into(),
                FieldT::from(0).into(),
            ),
            format!("{} handle_loadb", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_LOADW.clone() as usize]
                    .clone()
                    .into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.memory_subcontents.clone(),
                ) - self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_LOADW.clone() as usize]
                    .clone(),
                FieldT::from(0).into(),
            ),
            format!("{} handle_loadw", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.opcode_indicators
                    [tinyram_opcode::tinyram_opcode_STOREB.clone() as usize]
                    .clone()
                    .into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.memory_subcontents.clone(),
                ) - pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &(pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                        self.t.t.t.desval.borrow().t.bits.contents[..8].to_vec(),
                    )
                    .into()),
                ),
                FieldT::from(0).into(),
            ),
            format!("{} handle_storeb", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.opcode_indicators
                    [tinyram_opcode::tinyram_opcode_STOREW.clone() as usize]
                    .clone()
                    .into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.memory_subcontents.clone(),
                ) - self.t.t.t.desval.borrow().t.packed.clone(),
                FieldT::from(0).into(),
            ),
            format!("{} handle_storew", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - (self.t.t.t.opcode_indicators
                    [tinyram_opcode::tinyram_opcode_STOREB.clone() as usize]
                    .clone()
                    + self.t.t.t.opcode_indicators
                        [tinyram_opcode::tinyram_opcode_STOREW.clone() as usize]
                        .clone()
                        .into()),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t
                        .t
                        .t
                        .ls_prev_val_as_doubleword_variable
                        .borrow()
                        .t
                        .packed
                        .clone(),
                ) - self
                    .t
                    .t
                    .t
                    .ls_next_val_as_doubleword_variable
                    .borrow()
                    .t
                    .packed
                    .clone(),
                FieldT::from(0).into(),
            ),
            format!(
                "{} non_store_instructions_dont_change_memory",
                self.annotation_prefix
            ),
        );

        /* specify that accepting state implies opcode = answer && arg2val == 0 */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.next_has_accepted.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.opcode_indicators
                    [tinyram_opcode::tinyram_opcode_ANSWER.clone() as usize]
                    .clone(),
                FieldT::from(0).into(),
            ),
            format!("{} accepting_requires_answer", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.next_has_accepted.clone().into(),
                self.t.t.t.arg2val.borrow().t.packed.clone().into(),
                FieldT::from(0).into(),
            ),
            format!(
                "{} accepting_requires_arg2val_equal_zero",
                self.annotation_prefix
            ),
        );

        /*
           handle tapes:

           we require that:
           prev_tape1_exhausted implies next_tape1_exhausted,
           prev_tape1_exhausted implies flag to be set
           reads other than from tape 1 imply flag to be set
           flag implies result to be 0
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.prev_tape1_exhausted.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.next_tape1_exhausted.clone(),
                FieldT::from(0).into(),
            ),
            format!(
                "{} prev_tape1_exhausted_implies_next_tape1_exhausted",
                self.annotation_prefix
            ),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.prev_tape1_exhausted.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.instruction_flags
                    [tinyram_opcode::tinyram_opcode_READ.clone() as usize]
                    .clone(),
                FieldT::from(0).into(),
            ),
            format!(
                "{} prev_tape1_exhausted_implies_flag",
                self.annotation_prefix
            ),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_READ.clone() as usize]
                    .clone()
                    .into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.arg2val.borrow().t.packed.clone(),
                self.t.t.t.read_not1.clone().into(),
            ),
            format!("{} read_not1", self.annotation_prefix),
        ); /* will be nonzero for read X for X != 1 */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.read_not1.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.instruction_flags
                    [tinyram_opcode::tinyram_opcode_READ.clone() as usize]
                    .clone(),
                FieldT::from(0).into(),
            ),
            format!("{} other_reads_imply_flag", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.instruction_flags[tinyram_opcode::tinyram_opcode_READ.clone() as usize]
                    .clone()
                    .into(),
                self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_READ.clone() as usize]
                    .clone()
                    .into(),
                FieldT::from(0).into(),
            ),
            format!("{} read_flag_implies_result_0", self.annotation_prefix),
        );
    }

    pub fn generate_r1cs_witness_address(&self) {
        /* decode instruction and arguments */
        self.t
            .t
            .t
            .prev_pc_addr_as_word_variable
            .borrow()
            .generate_r1cs_witness_from_bits();
        for i in 0..self.pb.borrow().t.ap.k {
            self.t.t.t.prev_registers[i].generate_r1cs_witness_from_bits();
        }

        self.t.t.t.decode_arguments.borrow().generate_r1cs_witness();

        self.t
            .t
            .t
            .desval
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .t
            .t
            .arg1val
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .t
            .t
            .arg2val
            .borrow()
            .generate_r1cs_witness_from_packed();

        /* clear out ls_addr and fill with everything of arg2val except the subaddress */
        self.t.t.t.ls_addr.fill_with_bits_of_field_element(
            &self.pb,
            &FieldT::from(
                self.pb
                    .borrow()
                    .val(&self.t.t.t.arg2val.borrow().t.packed)
                    .as_ulong()
                    >> self.pb.borrow().t.ap.subaddr_len(),
            ),
        );
    }

    pub fn generate_r1cs_witness_other(&self, aux: &[usize]) {
        /* now ls_prev_val is filled with memory contents at ls_addr. we
        now ensure consistency with its doubleword representation */
        self.t
            .t
            .t
            .ls_prev_val_as_doubleword_variable
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* fill in the opcode indicators */
        let opcode_val = self
            .t
            .t
            .t
            .opcode
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        for i in 0..1usize << self.pb.borrow().t.ap.opcode_width() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.opcode_indicators[i]) = if i == opcode_val {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }

        /* execute the ALU */
        self.t.t.t.ALU.borrow().generate_r1cs_witness();

        /* fill memory_subaddress */
        self.t.t.t.memory_subaddress.borrow().t.bits.fill_with_bits(
            &self.pb,
            &pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                self.t.t.t.arg2val.borrow().t.bits.contents[..self.pb.borrow().t.ap.subaddr_len()]
                    .to_vec(),
            )
            .get_bits(&self.pb),
        );
        self.t
            .t
            .t
            .memory_subaddress
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* we distinguish four cases for memory handling:
        a) load.b
        b) store.b
        c) store.w
        d) load.w or any non-memory instruction */
        let prev_doubleword = self
            .pb
            .borrow()
            .val(
                &self
                    .t
                    .t
                    .t
                    .ls_prev_val_as_doubleword_variable
                    .borrow()
                    .t
                    .packed,
            )
            .as_ulong();
        let subaddress = self
            .pb
            .borrow()
            .val(&self.t.t.t.memory_subaddress.borrow().t.packed)
            .as_ulong();

        if self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_LOADB.clone() as usize],
        ) == FieldT::one()
        {
            let loaded_byte = (prev_doubleword >> (8 * subaddress)) & 0xFF;
            *self.pb.borrow_mut().val_ref(
                &self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_LOADB.clone() as usize],
            ) = FieldT::from(loaded_byte);
            *self.pb.borrow_mut().val_ref(&self.t.t.t.memory_subcontents) =
                FieldT::from(loaded_byte);
        } else if self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_STOREB.clone() as usize],
        ) == FieldT::one()
        {
            let stored_byte = (self
                .pb
                .borrow()
                .val(&self.t.t.t.desval.borrow().t.packed)
                .as_ulong())
                & 0xFF;
            *self.pb.borrow_mut().val_ref(&self.t.t.t.memory_subcontents) =
                FieldT::from(stored_byte);
        } else if self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_STOREW.clone() as usize],
        ) == FieldT::one()
        {
            let stored_word = (self
                .pb
                .borrow()
                .val(&self.t.t.t.desval.borrow().t.packed)
                .as_ulong());
            *self.pb.borrow_mut().val_ref(&self.t.t.t.memory_subcontents) =
                FieldT::from(stored_word);
        } else {
            let access_is_word0 = (self.pb.borrow().val(
                &*self
                    .t
                    .t
                    .t
                    .memory_subaddress
                    .borrow()
                    .t
                    .bits
                    .iter()
                    .last()
                    .unwrap(),
            ) == FieldT::zero());
            let loaded_word = (prev_doubleword
                >> (if access_is_word0 {
                    0
                } else {
                    self.pb.borrow().t.ap.w
                }))
                & ((1usize << self.pb.borrow().t.ap.w) - 1);
            *self.pb.borrow_mut().val_ref(
                &self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_LOADW.clone() as usize],
            ) = FieldT::from(loaded_word); /* does not hurt even for non-memory instructions */
            *self.pb.borrow_mut().val_ref(&self.t.t.t.memory_subcontents) =
                FieldT::from(loaded_word);
        }

        self.t.t.t.memory_access_is_word.evaluate_pb(&self.pb);
        self.t.t.t.memory_access_is_byte.evaluate_pb(&self.pb);

        self.t.t.t.check_memory.borrow().generate_r1cs_witness();

        /* handle reads */
        if self.pb.borrow().val(&self.t.t.t.prev_tape1_exhausted) == FieldT::one() {
            /* if tape was exhausted before, it will always be
            exhausted. we also need to only handle reads from tape 1,
            so we can safely set flag here */
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.next_tape1_exhausted) = FieldT::one();
            *self.pb.borrow_mut().val_ref(
                &self.t.t.t.instruction_flags[tinyram_opcode::tinyram_opcode_READ.clone() as usize],
            ) = FieldT::one();
        }

        *self.pb.borrow_mut().val_ref(&self.t.t.t.read_not1) = self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_READ.clone() as usize],
        ) * (FieldT::one()
            - self.pb.borrow().val(&self.t.t.t.arg2val.borrow().t.packed));
        if self.pb.borrow().val(&self.t.t.t.read_not1) != FieldT::one() {
            /* reading from tape other than 0 raises the flag */
            *self.pb.borrow_mut().val_ref(
                &self.t.t.t.instruction_flags[tinyram_opcode::tinyram_opcode_READ.clone() as usize],
            ) = FieldT::one();
        } else {
            /* otherwise perform the actual read */
            if !aux.is_empty() {
                *self.pb.borrow_mut().val_ref(
                    &self.t.t.t.instruction_results
                        [tinyram_opcode::tinyram_opcode_READ.clone() as usize],
                ) = FieldT::from(aux[0]);
                if aux.len() == 1 {
                    /* tape has ended! */
                    *self
                        .pb
                        .borrow_mut()
                        .val_ref(&self.t.t.t.next_tape1_exhausted) = FieldT::one();
                }
            } else {
                /* handled above, so nothing to do here */
            }
        }

        /* flag implies result zero */
        if self.pb.borrow().val(
            &self.t.t.t.instruction_flags[tinyram_opcode::tinyram_opcode_READ.clone() as usize],
        ) == FieldT::one()
        {
            *self.pb.borrow_mut().val_ref(
                &self.t.t.t.instruction_results
                    [tinyram_opcode::tinyram_opcode_READ.clone() as usize],
            ) = FieldT::zero();
        }

        /* execute consistency enforcer */
        self.t
            .t
            .t
            .consistency_enforcer
            .borrow()
            .generate_r1cs_witness();
        self.t
            .t
            .t
            .next_pc_addr_as_word_variable
            .borrow()
            .generate_r1cs_witness_from_packed();

        for i in 0..self.pb.borrow().t.ap.k {
            self.t.t.t.next_registers[i].generate_r1cs_witness_from_packed();
        }

        /* finally set has_accepted to 1 if both the opcode is ANSWER and arg2val is 0 */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.next_has_accepted) = if (self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_ANSWER.clone() as usize],
        ) == FieldT::one()
            && self.pb.borrow().val(&self.t.t.t.arg2val.borrow().t.packed) == FieldT::zero())
        {
            FieldT::one()
        } else {
            FieldT::zero()
        };
    }

    pub fn dump(&self) {
        print!(
            "   pc = {}, flag = {}\n",
            self.pb
                .borrow()
                .val(&self.t.t.t.prev_pc_addr_as_word_variable.borrow().t.packed)
                .as_ulong(),
            self.pb.borrow().val(&self.t.t.t.prev_flag).as_ulong()
        );
        print!("   ");

        for j in 0..self.pb.borrow().t.ap.k {
            print!(
                "r{} = {:2} ",
                j,
                self.pb
                    .borrow()
                    .val(&self.t.t.t.prev_registers[j].t.packed)
                    .as_ulong()
            );
        }
        print!("\n");

        let opcode_val = self
            .t
            .t
            .t
            .opcode
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        let tinyram_opcode_names_map: HashMap<_, _> =
            tinyram_opcode_names.iter().cloned().collect();
        print!(
            "   {} r{}, r{}, {}{}\n",
            tinyram_opcode_names_map[&((opcode_val as u8).into())],
            self.t
                .t
                .t
                .desidx
                .get_field_element_from_bits(&self.pb)
                .as_ulong(),
            self.t
                .t
                .t
                .arg1idx
                .get_field_element_from_bits(&self.pb)
                .as_ulong(),
            if self.pb.borrow().val(&self.t.t.t.arg2_is_imm) == FieldT::one() {
                ""
            } else {
                "r"
            },
            self.t
                .t
                .t
                .arg2idx
                .get_field_element_from_bits(&self.pb)
                .as_ulong()
        );
    }
}
