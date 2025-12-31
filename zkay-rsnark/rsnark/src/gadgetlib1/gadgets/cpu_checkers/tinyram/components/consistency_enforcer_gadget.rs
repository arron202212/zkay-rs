// Declaration of interfaces for the TinyRAM consistency enforcer gadget.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    inner_product_gadget, loose_multiplexing_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    tinyram_inner_product_gadget, tinyram_loose_multiplexing_gadget, tinyram_packing_gadget,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::rams::{
    ram_params::ArchitectureParamsTypeConfig,
    tinyram::tinyram_aux::{
        tinyram_opcodes_control_flow, tinyram_opcodes_register, tinyram_opcodes_stall,
    },
};
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use rccell::RcCell;
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct consistency_enforcer_gadget<FieldT: FieldTConfig> {
    // : public tinyram_standard_gadget<FieldT>
    is_register_instruction: variable<FieldT, pb_variable>,
    is_control_flow_instruction: variable<FieldT, pb_variable>,
    is_stall_instruction: variable<FieldT, pb_variable>,
    packed_desidx: variable<FieldT, pb_variable>,
    pack_desidx: RcCell<tinyram_packing_gadget<FieldT>>,
    computed_result: variable<FieldT, pb_variable>,
    computed_flag: variable<FieldT, pb_variable>,
    compute_computed_result: RcCell<tinyram_inner_product_gadget<FieldT>>,
    compute_computed_flag: RcCell<tinyram_inner_product_gadget<FieldT>>,
    pc_from_cf_or_zero: variable<FieldT, pb_variable>,
    demux_packed_outgoing_desval: RcCell<tinyram_loose_multiplexing_gadget<FieldT>>,
    opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    instruction_results: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    instruction_flags: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    desidx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    packed_incoming_pc: variable<FieldT, pb_variable>,
    packed_incoming_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    packed_incoming_desval: variable<FieldT, pb_variable>,
    incoming_flag: variable<FieldT, pb_variable>,
    packed_outgoing_pc: variable<FieldT, pb_variable>,
    packed_outgoing_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    outgoing_flag: variable<FieldT, pb_variable>,
    packed_outgoing_desval: variable<FieldT, pb_variable>,
}

//     consistency_enforcer_gadget(
// tinyram_protoboard<FieldT> &pb,
//                                 opcode_indicators:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 instruction_results:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 instruction_flags:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 desidx:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 packed_incoming_pc:variable<FieldT,pb_variable>,
//                                 packed_incoming_registers:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 packed_incoming_desval:variable<FieldT,pb_variable>,
//                                 incoming_flag:variable<FieldT,pb_variable>,
//                                 packed_outgoing_pc:variable<FieldT,pb_variable>,
//                                 packed_outgoing_registers:pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
//                                 outgoing_flag:variable<FieldT,pb_variable>,
//                                 annotation_prefix:String="");

//     pub fn  generate_r1cs_constraints();
//     pub fn  generate_r1cs_witness();
// };

// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::consistency_enforcer_gadget;

//#endif // CONSISTENCY_ENFORCER_GADGET_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for the TinyRAM consistency enforcer gadget.

See consistency_enforcer_gadget.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef CONSISTENCY_ENFORCER_GADGET_TCC_
// #define CONSISTENCY_ENFORCER_GADGET_TCC_
impl<FieldT: FieldTConfig> SubTinyRamGadgetConfig for consistency_enforcer_gadget<FieldT> {}

pub type consistency_enforcer_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, consistency_enforcer_gadget<FieldT>>>,
>;

impl<FieldT: FieldTConfig> consistency_enforcer_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        instruction_results: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        instruction_flags: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desidx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        packed_incoming_pc: variable<FieldT, pb_variable>,
        packed_incoming_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        packed_incoming_desval: variable<FieldT, pb_variable>,
        incoming_flag: variable<FieldT, pb_variable>,
        packed_outgoing_pc: variable<FieldT, pb_variable>,
        packed_outgoing_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        outgoing_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> consistency_enforcer_gadgets<FieldT> {
        assert!(desidx.len() == pb.borrow().t.ap.reg_arg_width());
        let mut packed_outgoing_desval = variable::<FieldT, pb_variable>::default();
        packed_outgoing_desval
            .allocate(&pb, format!("{} packed_outgoing_desval", annotation_prefix));
        let mut is_register_instruction = variable::<FieldT, pb_variable>::default();
        is_register_instruction.allocate(
            &pb,
            format!("{} is_register_instruction", annotation_prefix),
        );
        let mut is_control_flow_instruction = variable::<FieldT, pb_variable>::default();
        is_control_flow_instruction.allocate(
            &pb,
            format!("{} is_control_flow_instruction", annotation_prefix),
        );
        let mut is_stall_instruction = variable::<FieldT, pb_variable>::default();
        is_stall_instruction.allocate(&pb, format!("{} is_stall_instruction", annotation_prefix));
        let mut packed_desidx = variable::<FieldT, pb_variable>::default();
        packed_desidx.allocate(&pb, format!("{} packed_desidx", annotation_prefix));
        let pack_desidx = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            desidx.clone().into(),
            packed_desidx.clone().into(),
            format!("{}pack_desidx", annotation_prefix),
        ));
        let mut computed_result = variable::<FieldT, pb_variable>::default();
        computed_result.allocate(&pb, format!("{} computed_result", annotation_prefix));
        let mut computed_flag = variable::<FieldT, pb_variable>::default();
        computed_flag.allocate(&pb, format!("{} computed_flag", annotation_prefix));

        let compute_computed_result = RcCell::new(inner_product_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            opcode_indicators.clone().into(),
            instruction_results.clone().into(),
            computed_result.clone().into(),
            format!("{} compute_computed_result", annotation_prefix),
        ));
        let compute_computed_flag = RcCell::new(inner_product_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            opcode_indicators.clone().into(),
            instruction_flags.clone().into(),
            computed_flag.clone().into(),
            format!("{} compute_computed_flag", annotation_prefix),
        ));
        let mut pc_from_cf_or_zero = variable::<FieldT, pb_variable>::default();

        pc_from_cf_or_zero.allocate(&pb, format!("{} pc_from_cf_or_zero", annotation_prefix));

        let demux_packed_outgoing_desval = RcCell::new(loose_multiplexing_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            packed_outgoing_registers.clone().into(),
            packed_desidx.clone().into(),
            packed_outgoing_desval.clone().into(),
            variable::<FieldT, pb_variable>::from(ONE).into(),
            format!("{} demux_packed_outgoing_desval", annotation_prefix),
        ));

        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                is_register_instruction,
                is_control_flow_instruction,
                is_stall_instruction,
                packed_desidx,
                pack_desidx,
                computed_result,
                computed_flag,
                compute_computed_result,
                compute_computed_flag,
                pc_from_cf_or_zero,
                demux_packed_outgoing_desval,
                opcode_indicators,
                instruction_results,
                instruction_flags,
                desidx,
                packed_incoming_pc,
                packed_incoming_registers,
                packed_incoming_desval,
                incoming_flag,
                packed_outgoing_pc,
                packed_outgoing_registers,
                outgoing_flag,
                packed_outgoing_desval,
            },
        )
    }
}
impl<FieldT: FieldTConfig> consistency_enforcer_gadgets<FieldT> {
    pub fn generate_r1cs_constraints(&self) {
        /* pack destination index */
        self.t
            .t
            .t
            .pack_desidx
            .borrow()
            .generate_r1cs_constraints(false);

        /* demux result register */
        self.t
            .t
            .t
            .demux_packed_outgoing_desval
            .borrow()
            .generate_r1cs_constraints();

        /* is_register_instruction */
        let (mut reg_a, mut reg_b, mut reg_c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        reg_a.add_term(ONE, 1);
        for i in 0..tinyram_opcodes_register.len() {
            reg_b.add_term(
                self.t.t.t.opcode_indicators[tinyram_opcodes_register[i].clone() as usize].index,
                1,
            );
        }
        reg_c.add_term(self.t.t.t.is_register_instruction.index, 1);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(reg_a, reg_b, reg_c),
            format!("{} is_register_instruction", self.annotation_prefix),
        );

        /* is_control_flow_instruction */
        let (mut cf_a, mut cf_b, mut cf_c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        cf_a.add_term(ONE, 1);
        for i in 0..tinyram_opcodes_control_flow.len() {
            cf_b.add_term(
                self.t.t.t.opcode_indicators[tinyram_opcodes_control_flow[i].clone() as usize]
                    .index,
                1,
            );
        }
        cf_c.add_term(self.t.t.t.is_control_flow_instruction.index, 1);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(cf_a, cf_b, cf_c),
            format!("{} is_control_flow_instruction", self.annotation_prefix),
        );

        /* is_stall_instruction */
        let (mut stall_a, mut stall_b, mut stall_c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        stall_a.add_term(ONE, 1);
        for i in 0..tinyram_opcodes_stall.len() {
            stall_b.add_term(
                self.t.t.t.opcode_indicators[tinyram_opcodes_stall[i].clone() as usize].index,
                1,
            );
        }
        stall_c.add_term(self.t.t.t.is_stall_instruction.index, 1);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                stall_a, stall_b, stall_c,
            ),
            format!("{} is_stall_instruction", self.annotation_prefix),
        );

        /* compute actual result/actual flag */
        self.t
            .t
            .t
            .compute_computed_result
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .t
            .t
            .compute_computed_flag
            .borrow()
            .generate_r1cs_constraints();

        /*
          compute new PC address (in double words, not bytes!):

          PC' = computed_result * is_control_flow_instruction + PC * is_stall_instruction + (PC+1) * (1-is_control_flow_instruction - is_stall_instruction)
          PC' - pc_from_cf_or_zero - (1-is_control_flow_instruction - is_stall_instruction) = PC * (1 - is_control_flow_instruction)
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.computed_result.clone().into(),
                self.t.t.t.is_control_flow_instruction.clone().into(),
                self.t.t.t.pc_from_cf_or_zero.clone().into(),
            ),
            format!("{} pc_from_cf_or_zero", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.packed_incoming_pc.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.is_control_flow_instruction.clone(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.packed_outgoing_pc.clone(),
                ) - self.t.t.t.pc_from_cf_or_zero.clone()
                    - (linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        FieldT::from(1),
                    ) - self.t.t.t.is_control_flow_instruction.clone()
                        - self.t.t.t.is_stall_instruction.clone()),
            ),
            format!("{} packed_outgoing_pc", self.annotation_prefix),
        );

        /*
          enforce new flag:

          flag' = computed_flag * is_register_instruction + flag * (1-is_register_instruction)
          flag' - flag = (computed_flag - flag) * is_register_instruction
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    self.t.t.t.computed_flag.clone().into(),
                    (self.t.t.t.incoming_flag.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.is_register_instruction.clone().into()],
                vec![
                    self.t.t.t.outgoing_flag.clone().into(),
                    (self.t.t.t.incoming_flag.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} outgoing_flag", self.annotation_prefix),
        );

        /*
          force carryover of unchanged registers

          (1-indicator) * (new-old) = 0

          In order to save constraints we "borrow" indicator variables
          from loose multiplexing gadget.
        */
        for i in 0..self.pb.borrow().t.ap.k {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                    vec![
                        variable::<FieldT, pb_variable>::from(ONE).into(),
                        (self.t.t.t.demux_packed_outgoing_desval.borrow().t.alpha[i].clone()
                            * (-1).into())
                        .into(),
                    ],
                    vec![
                        self.t.t.t.packed_outgoing_registers[i].clone().into(),
                        (self.t.t.t.packed_incoming_registers[i].clone() * (-1).into()).into(),
                    ],
                    vec![(variable::<FieldT, pb_variable>::from(ONE) * 0.into()).into()],
                ),
                format!("{} register_carryover_{}", self.annotation_prefix, i),
            );
        }

        /*
          enforce correct destination register value:

          next_desval = computed_result * is_register_instruction + packed_incoming_desval * (1-is_register_instruction)
          next_desval - packed_incoming_desval = (computed_result - packed_incoming_desval) * is_register_instruction
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    self.t.t.t.computed_result.clone().into(),
                    (self.t.t.t.packed_incoming_desval.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.is_register_instruction.clone().into()],
                vec![
                    self.t.t.t.packed_outgoing_desval.clone().into(),
                    (self.t.t.t.packed_incoming_desval.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} packed_outgoing_desval", self.annotation_prefix),
        );
    }

    pub fn generate_r1cs_witness(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        /* pack destination index */
        self.t
            .t
            .t
            .pack_desidx
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* is_register_instruction */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.is_register_instruction) = FieldT::zero();

        for i in 0..tinyram_opcodes_register.len() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.is_register_instruction) += self
                .pb
                .borrow()
                .val(&self.t.t.t.opcode_indicators[tinyram_opcodes_register[i].clone() as usize]);
        }

        /* is_control_flow_instruction */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.is_control_flow_instruction) = FieldT::zero();

        for i in 0..tinyram_opcodes_control_flow.len() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.is_control_flow_instruction) += self.pb.borrow().val(
                &self.t.t.t.opcode_indicators[tinyram_opcodes_control_flow[i].clone() as usize],
            );
        }

        /* is_stall_instruction */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.is_stall_instruction) = FieldT::zero();

        for i in 0..tinyram_opcodes_stall.len() {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.is_stall_instruction) += self
                .pb
                .borrow()
                .val(&self.t.t.t.opcode_indicators[tinyram_opcodes_stall[i].clone() as usize]);
        }

        /* compute actual result/actual flag */
        self.t
            .t
            .t
            .compute_computed_result
            .borrow()
            .generate_r1cs_witness();
        self.t
            .t
            .t
            .compute_computed_flag
            .borrow()
            .generate_r1cs_witness();

        /*
          compute new PC address (in double words, not bytes!):

          PC' = computed_result * is_control_flow_instruction + PC * is_stall_instruction + (PC+1) * (1-is_control_flow_instruction - is_stall_instruction)
          PC' - pc_from_cf_or_zero - (1-is_control_flow_instruction - is_stall_instruction) = PC * (1 - is_control_flow_instruction)
        */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.pc_from_cf_or_zero) =
            self.pb.borrow().val(&self.t.t.t.computed_result)
                * self
                    .pb
                    .borrow()
                    .val(&self.t.t.t.is_control_flow_instruction);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.packed_outgoing_pc) =
            self.pb.borrow().val(&self.t.t.t.pc_from_cf_or_zero)
                + self.pb.borrow().val(&self.t.t.t.packed_incoming_pc)
                    * self.pb.borrow().val(&self.t.t.t.is_stall_instruction)
                + (self.pb.borrow().val(&self.t.t.t.packed_incoming_pc) + FieldT::one())
                    * (FieldT::one()
                        - self
                            .pb
                            .borrow()
                            .val(&self.t.t.t.is_control_flow_instruction)
                        - self.pb.borrow().val(&self.t.t.t.is_stall_instruction));

        /*
          enforce new flag:

          flag' = computed_flag * is_register_instruction + flag * (1-is_register_instruction)
          flag' - flag = (computed_flag - flag) * is_register_instruction
        */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.outgoing_flag) =
            self.pb.borrow().val(&self.t.t.t.computed_flag)
                * self.pb.borrow().val(&self.t.t.t.is_register_instruction)
                + self.pb.borrow().val(&self.t.t.t.incoming_flag)
                    * (FieldT::one() - self.pb.borrow().val(&self.t.t.t.is_register_instruction));

        /*
          update registers (changed and unchanged)

          next_desval = computed_result * is_register_instruction + packed_incoming_desval * (1-is_register_instruction)
        */
        let changed_register_contents = self.pb.borrow().val(&self.t.t.t.computed_result)
            * self.pb.borrow().val(&self.t.t.t.is_register_instruction)
            + self.pb.borrow().val(&self.t.t.t.packed_incoming_desval)
                * (FieldT::one() - self.pb.borrow().val(&self.t.t.t.is_register_instruction));

        for i in 0..self.pb.borrow().t.ap.k {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.packed_outgoing_registers[i]) =
                if (self.pb.borrow().val(&self.t.t.t.packed_desidx).as_ulong() == i) {
                    changed_register_contents.clone()
                } else {
                    self.pb
                        .borrow()
                        .val(&self.t.t.t.packed_incoming_registers[i])
                };
        }

        /* demux result register (it is important to do witness generation
        here after all registers have been set to the correct
        values!) */
        self.t
            .t
            .t
            .demux_packed_outgoing_desval
            .borrow()
            .generate_r1cs_witness();
    }
}
