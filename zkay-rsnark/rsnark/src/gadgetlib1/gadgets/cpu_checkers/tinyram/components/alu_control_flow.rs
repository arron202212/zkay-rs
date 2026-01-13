use crate::gadgetlib1::gadget::gadget;
/** @file
*****************************************************************************

Declaration of interfaces for the TinyRAM ALU control-flow gadgets.

These gadget check the correct execution of control-flow TinyRAM instructions.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef ALU_CONTROL_FLOW_HPP_
// #define ALU_CONTROL_FLOW_HPP_
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::SubTinyRamGadgetConfig;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::ArithmeticGadgetConfig;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    word_variable_gadget, word_variable_gadgets,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_packing_sum, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    generate_tinyram_prelude, tinyram_architecture_params, tinyram_program,
};
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::profiling::print_time;
use rccell::RcCell;
use std::marker::PhantomData;
/* control flow gadgets */
#[derive(Clone, Default)]
pub struct ALU_control_flow_gadget<FieldT: FieldTConfig, T: Default + Clone> {
    // : public tinyram_standard_gadget<FieldT>
    pc: word_variable_gadgets<FieldT>,
    argval2: word_variable_gadgets<FieldT>,
    flag: variable<FieldT, pb_variable>,
    result: variable<FieldT, pb_variable>,
    t: T,
}
impl<FieldT: FieldTConfig, T: Default + Clone> SubTinyRamGadgetConfig
    for ALU_control_flow_gadget<FieldT, T>
{
}
pub type ALU_control_flow_gadgets<FieldT, T> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, ALU_control_flow_gadget<FieldT, T>>>,
>;
impl<FieldT: FieldTConfig, T: Default + Clone> ALU_control_flow_gadget<FieldT, T> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        pc: word_variable_gadgets<FieldT>,
        argval2: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
        t: T,
    ) -> ALU_control_flow_gadgets<FieldT, T> {
        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                pc,
                argval2,
                flag,
                result,
                t,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_jmp_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type ALU_jmp_gadgets<FieldT> = ALU_control_flow_gadgets<FieldT, ALU_jmp_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_jmp_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        pc: word_variable_gadgets<FieldT>,
        argval2: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_jmp_gadgets<FieldT> {
        ALU_control_flow_gadget::<FieldT, Self>::new(
            pb,
            pc,
            argval2,
            flag,
            result,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_cjmp_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type ALU_cjmp_gadgets<FieldT> = ALU_control_flow_gadgets<FieldT, ALU_cjmp_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_cjmp_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        pc: word_variable_gadgets<FieldT>,
        argval2: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_cjmp_gadgets<FieldT> {
        ALU_control_flow_gadget::<FieldT, Self>::new(
            pb,
            pc,
            argval2,
            flag,
            result,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_cnjmp_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type ALU_cnjmp_gadgets<FieldT> = ALU_control_flow_gadgets<FieldT, ALU_cnjmp_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_cnjmp_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        pc: word_variable_gadgets<FieldT>,
        argval2: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_cnjmp_gadgets<FieldT> {
        ALU_control_flow_gadget::<FieldT, Self>::new(
            pb,
            pc,
            argval2,
            flag,
            result,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

/* jmp */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_jmp_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.argval2.t.packed.clone().into()],
                vec![self.t.t.t.result.clone().into()],
            ),
            format!("{} jmp_result", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) =
            self.pb.borrow().val(&self.t.t.t.argval2.t.packed);
    }
}

pub fn test_ALU_jmp_gadget<FieldT: FieldTConfig>() {
    print_time("starting jmp test");

    let mut ap = tinyram_architecture_params::new(16, 16);
    let mut P = tinyram_program::default();
    P.instructions = generate_tinyram_prelude(&ap);
    let mut pb = RcCell::new(tinyram_protoboard::<FieldT>::new(ap.clone())); // P.len(), 0, 10);

    let mut pc = word_variable_gadget::<FieldT>::new(pb.clone(), "pc".to_owned());
    let mut argval2 = word_variable_gadget::<FieldT>::new(pb.clone(), "argval2".to_owned());
    let (mut flag, mut result) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(&pb, "flag".to_owned());
    result.allocate(&pb, "result".to_owned());

    let mut jmp = ALU_jmp_gadget::<FieldT>::new(
        pb.clone(),
        pc.clone(),
        argval2.clone(),
        flag.clone(),
        result.clone(),
        "jmp".to_owned(),
    );
    jmp.generate_r1cs_constraints();

    *pb.borrow_mut().val_ref(&argval2.t.packed) = FieldT::from(123);
    argval2.generate_r1cs_witness_from_packed();

    jmp.generate_r1cs_witness();

    assert!(pb.borrow().val(&result) == FieldT::from(123));
    assert!(pb.borrow().is_satisfied());
    print_time("positive jmp test successful");

    *pb.borrow_mut().val_ref(&result) = FieldT::from(1);
    assert!(!pb.borrow().is_satisfied());
    print_time("negative jmp test successful");
}

/* cjmp */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_cjmp_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /*
          flag1 * argval2 + (1-flag1) * (pc1 + 1) = cjmp_result
          flag1 * (argval2 - pc1 - 1) = cjmp_result - pc1 - 1

          Note that instruction fetch semantics require program counter to
          be aligned to the double word by rounding down, and pc_addr in
          the outer reduction is expressed as a double word address. To
          achieve this we just discard the first ap.subaddr_len() bits of
          the byte address of the PC.
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.flag.clone().into(),
                (pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                        self.t.t.t.argval2.t.bits.contents[self.pb.borrow().t.ap.subaddr_len()..]
                            .to_vec(),
                    )
                    .into(),
                ) - self.t.t.t.pc.t.packed.clone()
                    - 1)
                .into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.result.clone(),
                ) - self.t.t.t.pc.t.packed.clone()
                    - 1,
            ),
            format!("{} cjmp_result", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) =
            (if (self.pb.borrow().val(&self.t.t.t.flag) == FieldT::one()) {
                FieldT::from(
                    self.pb
                        .borrow()
                        .val(&self.t.t.t.argval2.t.packed)
                        .as_ulong()
                        >> self.pb.borrow().t.ap.subaddr_len(),
                )
            } else {
                self.pb.borrow().val(&self.t.t.t.pc.t.packed) + FieldT::one()
            });
    }
}

pub fn test_ALU_cjmp_gadget<FieldT: FieldTConfig>() {
    // TODO: update
    print_time("starting cjmp test");

    let mut ap = tinyram_architecture_params::new(16, 16);
    let mut P = tinyram_program::default();
    P.instructions = generate_tinyram_prelude(&ap);
    let mut pb = RcCell::new(tinyram_protoboard::<FieldT>::new(ap.clone())); // P.len(), 0, 10);

    let mut pc = word_variable_gadget::<FieldT>::new(pb.clone(), "pc".to_owned());
    let mut argval2 = word_variable_gadget::<FieldT>::new(pb.clone(), "argval2".to_owned());
    let (mut flag, mut result) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(&pb, "flag".to_owned());
    result.allocate(&pb, "result".to_owned());

    let mut cjmp = ALU_cjmp_gadget::<FieldT>::new(
        pb.clone(),
        pc.clone(),
        argval2.clone(),
        flag.clone(),
        result.clone(),
        "cjmp".to_owned(),
    );
    cjmp.generate_r1cs_constraints();

    *pb.borrow_mut().val_ref(&argval2.t.packed) = FieldT::from(123);
    argval2.generate_r1cs_witness_from_packed();
    *pb.borrow_mut().val_ref(&pc.t.packed) = FieldT::from(456);
    pc.generate_r1cs_witness_from_packed();

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(1);
    cjmp.generate_r1cs_witness();

    assert!(pb.borrow().val(&result) == FieldT::from(123));
    assert!(pb.borrow().is_satisfied());
    print_time("positive cjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(0);
    assert!(!pb.borrow().is_satisfied());
    print_time("negative cjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(0);
    cjmp.generate_r1cs_witness();

    assert!(pb.borrow().val(&result) == FieldT::from(456 + 2 * ap.w / 8));
    assert!(pb.borrow().is_satisfied());
    print_time("positive cjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(1);
    assert!(!pb.borrow().is_satisfied());
    print_time("negative cjmp test successful");
}

/* cnjmp */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_cnjmp_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /*
          flag1 * (pc1 + inc) + (1-flag1) * argval2 = cnjmp_result
          flag1 * (pc1 + inc - argval2) = cnjmp_result - argval2

          Note that instruction fetch semantics require program counter to
          be aligned to the double word by rounding down, and pc_addr in
          the outer reduction is expressed as a double word address. To
          achieve this we just discard the first ap.subaddr_len() bits of
          the byte address of the PC.
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.flag.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.pc.t.packed.clone(),
                ) + FieldT::from(1)
                    - pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                        &(pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                            self.t.t.t.argval2.t.bits.contents
                                [self.pb.borrow().t.ap.subaddr_len()..]
                                .to_vec(),
                        )
                        .into()),
                    ),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.result.clone(),
                ) - pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &(pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                        self.t.t.t.argval2.t.bits.contents[self.pb.borrow().t.ap.subaddr_len()..]
                            .to_vec(),
                    )
                    .into()),
                ),
            ),
            format!("{} cnjmp_result", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) =
            (if (self.pb.borrow().val(&self.t.t.t.flag) == FieldT::one()) {
                self.pb.borrow().val(&self.t.t.t.pc.t.packed) + FieldT::one()
            } else {
                FieldT::from(
                    self.pb
                        .borrow()
                        .val(&self.t.t.t.argval2.t.packed)
                        .as_ulong()
                        >> self.pb.borrow().t.ap.subaddr_len(),
                )
            });
    }
}

pub fn test_ALU_cnjmp_gadget<FieldT: FieldTConfig>() {
    // TODO: update
    print_time("starting cnjmp test");

    let mut ap = tinyram_architecture_params::new(16, 16);
    let mut P = tinyram_program::default();
    P.instructions = generate_tinyram_prelude(&ap);
    let mut pb = RcCell::new(tinyram_protoboard::<FieldT>::new(ap.clone())); //, P.len(), 0, 10);

    let mut pc = word_variable_gadget::<FieldT>::new(pb.clone(), "pc".to_owned());
    let mut argval2 = word_variable_gadget::<FieldT>::new(pb.clone(), "argval2".to_owned());
    let (mut flag, mut result) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );

    pc.generate_r1cs_constraints(true);
    argval2.generate_r1cs_constraints(true);
    flag.allocate(&pb, "flag".to_owned());
    result.allocate(&pb, "result".to_owned());

    let mut cnjmp = ALU_cnjmp_gadget::<FieldT>::new(
        pb.clone(),
        pc.clone(),
        argval2.clone(),
        flag.clone(),
        result.clone(),
        "cjmp".to_owned(),
    );
    cnjmp.generate_r1cs_constraints();

    *pb.borrow_mut().val_ref(&argval2.t.packed) = FieldT::from(123);
    argval2.generate_r1cs_witness_from_packed();
    *pb.borrow_mut().val_ref(&pc.t.packed) = FieldT::from(456);
    pc.generate_r1cs_witness_from_packed();

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(0);
    cnjmp.generate_r1cs_witness();

    assert!(pb.borrow().val(&result) == FieldT::from(123));
    assert!(pb.borrow().is_satisfied());
    print_time("positive cnjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(1);
    assert!(!pb.borrow().is_satisfied());
    print_time("negative cnjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(1);
    cnjmp.generate_r1cs_witness();

    assert!(pb.borrow().val(&result) == FieldT::from(456 + (2 * pb.borrow().t.ap.w / 8)));
    assert!(pb.borrow().is_satisfied());
    print_time("positive cnjmp test successful");

    *pb.borrow_mut().val_ref(&flag) = FieldT::from(0);
    assert!(!pb.borrow().is_satisfied());
    print_time("negative cnjmp test successful");
}
