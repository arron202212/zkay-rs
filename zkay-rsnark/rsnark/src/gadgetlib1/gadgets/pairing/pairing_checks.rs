//  Declaration of interfaces for pairing-check gadgets.

//  Given that e(.,.) denotes a pairing,
//  - the gadget "check_e_equals_e_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)"; and
//  - the gadget "check_e_equals_ee_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)*e(P3,Q3)".
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqk_variable, VariableTConfig, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    e_over_e_miller_loop_gadget, e_over_e_miller_loop_gadgets, e_times_e_over_e_miller_loop_gadget,
    e_times_e_over_e_miller_loop_gadgets, final_exp_gadget, final_exp_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation::{
    mnt4_final_exp_gadget, mnt4_final_exp_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop::{
    mnt_e_over_e_miller_loop_gadget, mnt_e_over_e_miller_loop_gadgets,
    mnt_e_times_e_over_e_miller_loop_gadget, mnt_e_times_e_over_e_miller_loop_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputations, G2_precomputations,
};
use crate::gadgetlib1::pb_variable::pb_variable;
use crate::gadgetlib1::protoboard::{PBConfig,ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::variable::variable;
use ff_curves::Fr;
use ffec::FieldTConfig;
use rccell::RcCell;

type FieldT<ppT> = Fr<ppT>;

#[derive(Clone, Default)]
pub struct check_e_equals_e_gadget<ppT: ppTConfig> {
    //gadget<ffec::Fr<ppT,ppT::FieldT, PB>  >
    ratio: RcCell<Fqk_variable<ppT>>,
    compute_ratio: RcCell<e_over_e_miller_loop_gadgets<ppT>>,
    check_finexp: RcCell<final_exp_gadgets<ppT>>,

    lhs_G1: G1_precomputations<ppT>,
    lhs_G2: G2_precomputations<ppT>,
    rhs_G1: G1_precomputations<ppT>,
    rhs_G2: G2_precomputations<ppT>,

    result: variable<ppT::FieldT, pb_variable>,
}

#[derive(Clone, Default)]
pub struct check_e_equals_ee_gadget<ppT: ppTConfig> {
    //gadget<ffec::Fr<ppT,ppT::FieldT, PB>  >

    // type FieldT=ffec::Fr<ppT,ppT::FieldT, PB> ;
    ratio: RcCell<Fqk_variable<ppT>>,
    compute_ratio: RcCell<e_times_e_over_e_miller_loop_gadgets<ppT>>,
    check_finexp: RcCell<final_exp_gadgets<ppT>>,

    lhs_G1: G1_precomputations<ppT>,
    lhs_G2: G2_precomputations<ppT>,
    rhs1_G1: G1_precomputations<ppT>,
    rhs1_G2: G2_precomputations<ppT>,
    rhs2_G1: G1_precomputations<ppT>,
    rhs2_G2: G2_precomputations<ppT>,

    result: variable<ppT::FieldT, pb_variable>,
}

pub type check_e_equals_e_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, check_e_equals_e_gadget<ppT>>;

impl<ppT: ppTConfig> check_e_equals_e_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        lhs_G1: G1_precomputations<ppT>,
        lhs_G2: G2_precomputations<ppT>,
        rhs_G1: G1_precomputations<ppT>,
        rhs_G2: G2_precomputations<ppT>,
        result: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> check_e_equals_e_gadgets<ppT> {
        let ratio = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " ratio"),
        ));
        let compute_ratio = RcCell::new(e_over_e_miller_loop_gadget::<ppT>::new(
            pb.clone(),
            lhs_G1.clone(),
            lhs_G2.clone(),
            rhs_G1.clone(),
            rhs_G2.clone(),
            ratio.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_ratio"),
        ));
        let check_finexp = RcCell::new(final_exp_gadget::<ppT>::new(
            pb.clone(),
            ratio.borrow().clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " check_finexp"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                ratio,
                compute_ratio,
                check_finexp,
                lhs_G1,
                lhs_G2,
                rhs_G1,
                rhs_G2,
                result,
            },
        )
    }
}
impl<ppT: ppTConfig> check_e_equals_e_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_constraints();
        self.t.check_finexp.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_witness();
        self.t.check_finexp.borrow().generate_r1cs_witness();
    }
}

pub type check_e_equals_ee_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, check_e_equals_ee_gadget<ppT>>;

impl<ppT: ppTConfig> check_e_equals_ee_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        lhs_G1: G1_precomputations<ppT>,
        lhs_G2: G2_precomputations<ppT>,
        rhs1_G1: G1_precomputations<ppT>,
        rhs1_G2: G2_precomputations<ppT>,
        rhs2_G1: G1_precomputations<ppT>,
        rhs2_G2: G2_precomputations<ppT>,
        result: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> check_e_equals_ee_gadgets<ppT> {
        let ratio = RcCell::new(Fqk_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " ratio"),
        ));
        let compute_ratio = RcCell::new(e_times_e_over_e_miller_loop_gadget::<ppT>::new(
            pb.clone(),
            rhs1_G1.clone(),
            rhs1_G2.clone(),
            rhs2_G1.clone(),
            rhs2_G2.clone(),
            lhs_G1.clone(),
            lhs_G2.clone(),
            ratio.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_ratio"),
        ));
        let check_finexp = RcCell::new(final_exp_gadget::<ppT>::new(
            pb.clone(),
            ratio.borrow().clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " check_finexp"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                ratio,
                compute_ratio,
                check_finexp,
                lhs_G1,
                lhs_G2,
                rhs1_G1,
                rhs1_G2,
                rhs2_G1,
                rhs2_G2,
                result,
            },
        )
    }
}
impl<ppT: ppTConfig> check_e_equals_ee_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_constraints();
        self.t.check_finexp.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_witness();
        self.t.check_finexp.borrow().generate_r1cs_witness();
    }
}
