//  Declaration of interfaces for pairing-check gadgets.

//  Given that e(.,.) denotes a pairing,
//  - the gadget "check_e_equals_e_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)"; and
//  - the gadget "check_e_equals_ee_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)*e(P3,Q3)".
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::curves::{
    Fqe_mul_by_lc_gadget, Fqe_mul_gadget, Fqe_sqr_gadget, Fqe_variable, Fqk_special_mul_gadget, G1,
    G2, MulTConfig, SqrTConfig, VariableTConfig, ppTConfig,
};
use crate::prefix_format;

use crate::gadgetlib1::gadgets::curves::Fqk_variable;
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
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use rccell::RcCell;

pub type e_over_e_miller_loop_gadget<ppT, FieldT, PB> =
    mnt_e_over_e_miller_loop_gadget<ppT, FieldT, PB>;
pub type e_times_e_over_e_miller_loop_gadget<ppT, FieldT, PB> =
    mnt_e_times_e_over_e_miller_loop_gadget<ppT, FieldT, PB>;
pub type final_exp_gadget<ppT, FieldT, PB> = mnt4_final_exp_gadget<ppT, FieldT, PB>;

pub type e_over_e_miller_loop_gadgets<ppT, FieldT, PB> =
    mnt_e_over_e_miller_loop_gadgets<ppT, FieldT, PB>;
pub type e_times_e_over_e_miller_loop_gadgets<ppT, FieldT, PB> =
    mnt_e_times_e_over_e_miller_loop_gadgets<ppT, FieldT, PB>;
pub type final_exp_gadgets<ppT, FieldT, PB> = mnt4_final_exp_gadgets<ppT, FieldT, PB>;

// type FieldT = ffec::Fr<ppT,FieldT,PB> ;

#[derive(Clone, Default)]
pub struct check_e_equals_e_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<ffec::Fr<ppT,FieldT,PB>  >
    ratio: RcCell<Fqk_variable<ppT, FieldT, PB>>,
    compute_ratio: RcCell<e_over_e_miller_loop_gadgets<ppT, FieldT, PB>>,
    check_finexp: RcCell<final_exp_gadgets<ppT, FieldT, PB>>,

    lhs_G1: G1_precomputations<ppT, FieldT, PB>,
    lhs_G2: G2_precomputations<ppT, FieldT, PB>,
    rhs_G1: G1_precomputations<ppT, FieldT, PB>,
    rhs_G2: G2_precomputations<ppT, FieldT, PB>,

    result: variable<FieldT, pb_variable>,
}

#[derive(Clone, Default)]
pub struct check_e_equals_ee_gadget<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
{
    //gadget<ffec::Fr<ppT,FieldT,PB>  >

    // type FieldT=ffec::Fr<ppT,FieldT,PB> ;
    ratio: RcCell<Fqk_variable<ppT, FieldT, PB>>,
    compute_ratio: RcCell<e_times_e_over_e_miller_loop_gadgets<ppT, FieldT, PB>>,
    check_finexp: RcCell<final_exp_gadgets<ppT, FieldT, PB>>,

    lhs_G1: G1_precomputations<ppT, FieldT, PB>,
    lhs_G2: G2_precomputations<ppT, FieldT, PB>,
    rhs1_G1: G1_precomputations<ppT, FieldT, PB>,
    rhs1_G2: G2_precomputations<ppT, FieldT, PB>,
    rhs2_G1: G1_precomputations<ppT, FieldT, PB>,
    rhs2_G2: G2_precomputations<ppT, FieldT, PB>,

    result: variable<FieldT, pb_variable>,
}

pub type check_e_equals_e_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, check_e_equals_e_gadget<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    check_e_equals_e_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        lhs_G1: G1_precomputations<ppT, FieldT, PB>,
        lhs_G2: G2_precomputations<ppT, FieldT, PB>,
        rhs_G1: G1_precomputations<ppT, FieldT, PB>,
        rhs_G2: G2_precomputations<ppT, FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> check_e_equals_e_gadgets<ppT, FieldT, PB> {
        let ratio = RcCell::new(Fqk_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " ratio"),
        ));
        let compute_ratio = RcCell::new(e_over_e_miller_loop_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            lhs_G1.clone(),
            lhs_G2.clone(),
            rhs_G1.clone(),
            rhs_G2.clone(),
            ratio.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_ratio"),
        ));
        let check_finexp = RcCell::new(final_exp_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            ratio.borrow().clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " check_finexp"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    check_e_equals_e_gadgets<ppT, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_constraints();
        self.t.check_finexp.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_witness();
        self.t.check_finexp.borrow().generate_r1cs_witness();
    }
}

pub type check_e_equals_ee_gadgets<ppT, FieldT, PB> =
    gadget<FieldT, PB, check_e_equals_ee_gadget<ppT, FieldT, PB>>;

impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    check_e_equals_ee_gadget<ppT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        lhs_G1: G1_precomputations<ppT, FieldT, PB>,
        lhs_G2: G2_precomputations<ppT, FieldT, PB>,
        rhs1_G1: G1_precomputations<ppT, FieldT, PB>,
        rhs1_G2: G2_precomputations<ppT, FieldT, PB>,
        rhs2_G1: G1_precomputations<ppT, FieldT, PB>,
        rhs2_G2: G2_precomputations<ppT, FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> check_e_equals_ee_gadgets<ppT, FieldT, PB> {
        let ratio = RcCell::new(Fqk_variable::<ppT, FieldT, PB>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " ratio"),
        ));
        let compute_ratio =
            RcCell::new(e_times_e_over_e_miller_loop_gadget::<ppT, FieldT, PB>::new(
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
        let check_finexp = RcCell::new(final_exp_gadget::<ppT, FieldT, PB>::new(
            pb.clone(),
            ratio.borrow().clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " check_finexp"),
        ));
        gadget::<FieldT, PB, Self>::new(
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
impl<ppT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    check_e_equals_ee_gadgets<ppT, FieldT, PB>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_constraints();
        self.t.check_finexp.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_ratio.borrow().generate_r1cs_witness();
        self.t.check_finexp.borrow().generate_r1cs_witness();
    }
}
