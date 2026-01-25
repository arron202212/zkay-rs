// Declaration of interfaces for the TinyRAM argument decoder gadget.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    dual_variable_gadget, inner_product_gadget, loose_multiplexing_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    tinyram_loose_multiplexing_gadget, tinyram_packing_gadget,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{protoboard,ProtoboardConfig};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
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
pub struct argument_decoder_gadget<FieldT: FieldTConfig> {
    // : public tinyram_standard_gadget<FieldT>
    packed_desidx: variable<FieldT, pb_variable>,
    packed_arg1idx: variable<FieldT, pb_variable>,
    packed_arg2idx: variable<FieldT, pb_variable>,
    pack_desidx: RcCell<tinyram_packing_gadget<FieldT>>,
    pack_arg1idx: RcCell<tinyram_packing_gadget<FieldT>>,
    pack_arg2idx: RcCell<tinyram_packing_gadget<FieldT>>,
    arg2_demux_result: variable<FieldT, pb_variable>,
    arg2_demux_success: variable<FieldT, pb_variable>,
    demux_des: RcCell<tinyram_loose_multiplexing_gadget<FieldT>>,
    demux_arg1: RcCell<tinyram_loose_multiplexing_gadget<FieldT>>,
    demux_arg2: RcCell<tinyram_loose_multiplexing_gadget<FieldT>>,
    arg2_is_imm: variable<FieldT, pb_variable>,
    desidx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    arg1idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    arg2idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    packed_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    packed_desval: variable<FieldT, pb_variable>,
    packed_arg1val: variable<FieldT, pb_variable>,
    packed_arg2val: variable<FieldT, pb_variable>,
}
// impl argument_decoder_gadget{
// argument_decoder_gadget(
// tinyram_protoboard<FieldT> &pb,
//                         arg2_is_imm:variable<FieldT,pb_variable>,
//                         desidx:pb_variable_array<FieldT,tinyram_protoboard<FieldT>>,
//                         arg1idx:pb_variable_array<FieldT,tinyram_protoboard<FieldT>>,
//                         arg2idx:pb_variable_array<FieldT,tinyram_protoboard<FieldT>>,
//                         packed_registers:pb_variable_array<FieldT,tinyram_protoboard<FieldT>>,
//                         packed_desval:variable<FieldT,pb_variable>,
//                         packed_arg1val:variable<FieldT,pb_variable>,
//                         packed_arg2val:variable<FieldT,pb_variable>,
//                         annotation_prefix:String="");

// }
pub type argument_decoder_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, argument_decoder_gadget<FieldT>>>,
>;
impl<FieldT: FieldTConfig> SubTinyRamGadgetConfig for argument_decoder_gadget<FieldT> {}
impl<FieldT: FieldTConfig> argument_decoder_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        arg2_is_imm: variable<FieldT, pb_variable>,
        desidx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        arg1idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        arg2idx: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        packed_registers: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        packed_desval: variable<FieldT, pb_variable>,
        packed_arg1val: variable<FieldT, pb_variable>,
        packed_arg2val: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> argument_decoder_gadgets<FieldT> {
        assert!(desidx.len() == pb.borrow().t.ap.reg_arg_width());
        assert!(arg1idx.len() == pb.borrow().t.ap.reg_arg_width());
        assert!(arg2idx.len() == pb.borrow().t.ap.reg_arg_or_imm_width());

        /* decode accordingly */
        let mut packed_desidx = variable::<FieldT, pb_variable>::default();
        packed_desidx.allocate(&pb, format!("{} packed_desidx", annotation_prefix));
        let mut packed_arg1idx = variable::<FieldT, pb_variable>::default();
        packed_arg1idx.allocate(&pb, format!("{} packed_arg1idx", annotation_prefix));
        let mut packed_arg2idx = variable::<FieldT, pb_variable>::default();
        packed_arg2idx.allocate(&pb, format!("{} packed_arg2idx", annotation_prefix));

        let pack_desidx = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            desidx.clone().into(),
            packed_desidx.clone().into(),
            format!("{}pack_desidx", annotation_prefix),
        ));
        let pack_arg1idx = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            arg1idx.clone().into(),
            packed_arg1idx.clone().into(),
            format!("{}pack_arg1idx", annotation_prefix),
        ));
        let pack_arg2idx = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            arg2idx.clone().into(),
            packed_arg2idx.clone().into(),
            format!("{}pack_arg2idx", annotation_prefix),
        ));
        let mut arg2_demux_result = variable::<FieldT, pb_variable>::default();
        arg2_demux_result.allocate(&pb, format!("{} arg2_demux_result", annotation_prefix));
        let mut arg2_demux_success = variable::<FieldT, pb_variable>::default();
        arg2_demux_success.allocate(&pb, format!("{} arg2_demux_success", annotation_prefix));

        let demux_des = RcCell::new(loose_multiplexing_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            packed_registers.clone().into(),
            packed_desidx.clone().into(),
            packed_desval.clone().into(),
            ONE.clone().into(),
            format!("{} demux_des", annotation_prefix),
        ));
        let demux_arg1 = RcCell::new(loose_multiplexing_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            packed_registers.clone().into(),
            packed_arg1idx.clone().into(),
            packed_arg1val.clone().into(),
            ONE.clone().into(),
            format!("{} demux_arg1", annotation_prefix),
        ));
        let demux_arg2 = RcCell::new(loose_multiplexing_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            packed_registers.clone().into(),
            packed_arg2idx.clone().into(),
            arg2_demux_result.clone().into(),
            arg2_demux_success.clone().into(),
            format!("{} demux_arg2", annotation_prefix),
        ));
        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                packed_desidx,
                packed_arg1idx,
                packed_arg2idx,
                pack_desidx,
                pack_arg1idx,
                pack_arg2idx,
                arg2_demux_result,
                arg2_demux_success,
                demux_des,
                demux_arg1,
                demux_arg2,
                arg2_is_imm,
                desidx,
                arg1idx,
                arg2idx,
                packed_registers,
                packed_desval,
                packed_arg1val,
                packed_arg2val,
            },
        )
    }
}
impl<FieldT: FieldTConfig> argument_decoder_gadgets<FieldT> {
    pub fn generate_r1cs_constraints(&self) {
        /* pack */
        self.t
            .t
            .t
            .pack_desidx
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .pack_arg1idx
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .pack_arg2idx
            .borrow()
            .generate_r1cs_constraints(true);

        /* demux */
        self.t.t.t.demux_des.borrow().generate_r1cs_constraints();
        self.t.t.t.demux_arg1.borrow().generate_r1cs_constraints();
        self.t.t.t.demux_arg2.borrow().generate_r1cs_constraints();

        /* enforce correct handling of arg2val */

        /* it is false that arg2 is reg and demux failed:
        (1 - arg2_is_imm) * (1 - arg2_demux_success) = 0 */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.arg2_is_imm.clone() * (-1).into()).into(),
                ],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.arg2_demux_success.clone() * (-1).into()).into(),
                ],
                vec![(variable::<FieldT, pb_variable>::from(ONE) * 0.into()).into()],
            ),
            format!("{} ensure_correc_demux", self.annotation_prefix),
        );

        /*
          arg2val = arg2_is_imm * packed_arg2idx +
          (1 - arg2_is_imm) * arg2_demux_result

          arg2val - arg2_demux_result = arg2_is_imm * (packed_arg2idx - arg2_demux_result)
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.arg2_is_imm.clone().into()],
                vec![
                    self.t.t.t.packed_arg2idx.clone().into(),
                    (self.t.t.t.arg2_demux_result.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.packed_arg2val.clone().into(),
                    (self.t.t.t.arg2_demux_result.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} compute_arg2val", self.annotation_prefix),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        /* pack */
        self.t
            .t
            .t
            .pack_desidx
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .t
            .t
            .pack_arg1idx
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .t
            .t
            .pack_arg2idx
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* demux */
        self.t.t.t.demux_des.borrow().generate_r1cs_witness();
        self.t.t.t.demux_arg1.borrow().generate_r1cs_witness();
        self.t.t.t.demux_arg2.borrow().generate_r1cs_witness();

        /* handle arg2val */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.packed_arg2val) =
            (if self.pb.borrow().val(&self.t.t.t.arg2_is_imm) == FieldT::one() {
                self.pb.borrow().val(&self.t.t.t.packed_arg2idx)
            } else {
                self.pb.borrow().val(&self.t.t.t.arg2_demux_result)
            });
    }
}

pub fn test_argument_decoder_gadget<FieldT: FieldTConfig, T: Default + Clone>() {
    print_time("starting argument_decoder_gadget test");

    let mut ap = tinyram_architecture_params::new(16, 16);
    let mut P = tinyram_program::default();
    P.instructions = generate_tinyram_prelude(&ap);
    let mut pb = RcCell::new(tinyram_protoboard::<FieldT>::new(ap.clone())); //, P.len(), 0, 10));

    let mut packed_registers = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
    packed_registers.allocate(&pb, ap.k, "packed_registers");

    let mut arg2_is_imm = variable::<FieldT, pb_variable>::default();
    arg2_is_imm.allocate(&pb, "arg_is_imm".to_owned());

    let mut desidx = dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, T>::new(
        pb.clone(),
        ap.reg_arg_width(),
        "desidx".to_owned(),
        T::default(),
    );
    let mut arg1idx = dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, T>::new(
        pb.clone(),
        ap.reg_arg_width(),
        "arg1idx".to_owned(),
        T::default(),
    );
    let mut arg2idx = dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, T>::new(
        pb.clone(),
        ap.reg_arg_or_imm_width(),
        "arg2idx".to_owned(),
        T::default(),
    );

    let (mut packed_desval, mut packed_arg1val, mut packed_arg2val) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );
    packed_desval.allocate(&pb, "packed_desval".to_owned());
    packed_arg1val.allocate(&pb, "packed_arg1val".to_owned());
    packed_arg2val.allocate(&pb, "packed_arg2val".to_owned());

    let mut g = argument_decoder_gadget::<FieldT>::new(
        pb.clone(),
        arg2_is_imm.clone(),
        desidx.t.bits.clone(),
        arg1idx.t.bits.clone(),
        arg2idx.t.bits.clone(),
        packed_registers.clone(),
        packed_desval.clone(),
        packed_arg1val.clone(),
        packed_arg2val.clone(),
        "g".to_owned(),
    );

    g.generate_r1cs_constraints();
    for i in 0..ap.k {
        *pb.borrow_mut().val_ref(&packed_registers[i]) = FieldT::from(1000 + i);
    }

    *pb.borrow_mut().val_ref(&desidx.t.packed) = FieldT::from(2);
    *pb.borrow_mut().val_ref(&arg1idx.t.packed) = FieldT::from(5);
    *pb.borrow_mut().val_ref(&arg2idx.t.packed) = FieldT::from(7);
    *pb.borrow_mut().val_ref(&arg2_is_imm) = FieldT::zero();

    desidx.generate_r1cs_witness_from_packed();
    arg1idx.generate_r1cs_witness_from_packed();
    arg2idx.generate_r1cs_witness_from_packed();

    g.generate_r1cs_witness();

    assert!(pb.borrow().val(&packed_desval) == FieldT::from(1002));
    assert!(pb.borrow().val(&packed_arg1val) == FieldT::from(1005));
    assert!(pb.borrow().val(&packed_arg2val) == FieldT::from(1007));
    assert!(pb.borrow().is_satisfied());
    print!("positive test (get reg) successful\n");

    *pb.borrow_mut().val_ref(&arg2_is_imm) = FieldT::one();
    g.generate_r1cs_witness();

    assert!(pb.borrow().val(&packed_desval) == FieldT::from(1002));
    assert!(pb.borrow().val(&packed_arg1val) == FieldT::from(1005));
    assert!(pb.borrow().val(&packed_arg2val) == FieldT::from(7));
    assert!(pb.borrow().is_satisfied());
    print!("positive test (get imm) successful\n");

    print_time("argument_decoder_gadget tests successful");
}
