//  Declaration of interfaces for the the R1CS ppzkSNARK verifier gadget.

//  The gadget r1cs_ppzksnark_verifier_gadget verifiers correct computation of r1cs_ppzksnark_verifier_strong_IC.
//  The gadget is built from two main sub-gadgets:
//  - r1cs_ppzksnark_verifier_process_vk_gadget, which verifies correct computation of r1cs_ppzksnark_verifier_process_vk, and
//  - r1cs_ppzksnark_online_verifier_gadget, which verifies correct computation of r1cs_ppzksnark_online_verifier_strong_IC.
//  See r1cs_ppzksnark.hpp for description of the aforementioned functions.
use crate::common::data_structures::accumulation_vector::accumulation_vector;
use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTS};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::generate_boolean_r1cs_constraint;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    conjunction_gadget, conjunction_gadgets, multipacking_gadget, multipacking_gadgets,
};
use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::{
    G1_add_gadget, G1_add_gadgets, G1_checker_gadget, G1_checker_gadgets,
    G1_multiscalar_mul_gadget, G1_multiscalar_mul_gadgets, G1_variable, G1_variables,
};
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{
    G2_checker_gadget, G2_checker_gadgets, G2_variable, G2_variables,
};
use crate::gadgetlib1::gadgets::fields::exponentiation_gadget::{
    exponentiation_gadget, exponentiation_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::pairing_checks::{
    check_e_equals_e_gadget, check_e_equals_e_gadgets, check_e_equals_ee_gadget,
    check_e_equals_ee_gadgets,
};

use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqk_mul_gadget, Fqk_sqr_gadget, Fqk_variable, other_curve, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputation, G1_precomputations, G2_precomputation, G2_precomputations,
    precompute_G1_gadget, precompute_G1_gadgets, precompute_G2_gadget,
    precompute_G2_gadget_coeffss, precompute_G2_gadgets,
};
// affine_ate_miller_loop, affine_ate_precompute_G1, affine_ate_precompute_G2, pairing_loop_count,

use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};

use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_proof, r1cs_ppzksnark_verification_key,
};
use ff_curves::Fr;
use ff_curves::algebra::curves::public_params;
use ff_curves::{G1, G2};
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::common::profiling::print_indent;
use ffec::common::utils::bit_vector;
use ffec::field_utils::bigint::bigint;
use ffec::{One, Zero};
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::{Add, Mul};

// #[derive(Clone, Default)]
// pub struct r1cs_ppzksnark_proof<
//     ppT: ppTConfig,
//
//     const N: usize,
//     G1T: PpConfig,
//     G2T: PpConfig,
// > {
//     pub g_A: knowledge_commitment<G1T, G1T>,
//     pub g_B: knowledge_commitment<G2T, G1T>,
//     pub g_C: knowledge_commitment<G1T, G1T>,
//     pub g_H: G1<ppT>,
//     pub g_K: G1<ppT>,
//     _t: PhantomData<(ppT::FieldT,PB)>,
// }
// #[derive(Clone, Default)]
// pub struct r1cs_ppzksnark_verification_key<ppT: ppTConfig> {
//     pub alphaA_g2: G2<ppT>,
//     pub alphaB_g1: G1<ppT>,
//     pub alphaC_g2: G2<ppT>,
//     pub gamma_g2: G2<ppT>,
//     pub gamma_beta_g1: G1<ppT>,
//     pub gamma_beta_g2: G2<ppT>,
//     pub rC_Z_g2: G2<ppT>,

//     pub encoded_IC_query: accumulation_vector<G1<ppT>>,
//     _t: PhantomData<(ppT::FieldT,PB)>,
// }

type FieldT<ppT> = Fr<ppT>;
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_proof_variable<ppT: ppTConfig> {
    //gadget<Fr<ppT,  PB> >

    // type FieldT<ppT>=Fr<ppT;
    pub g_A_g: RcCell<G1_variables<ppT>>,
    pub g_A_h: RcCell<G1_variables<ppT>>,
    pub g_B_g: RcCell<G2_variables<ppT>>,
    pub g_B_h: RcCell<G1_variables<ppT>>,
    pub g_C_g: RcCell<G1_variables<ppT>>,
    pub g_C_h: RcCell<G1_variables<ppT>>,
    pub g_H: RcCell<G1_variables<ppT>>,
    pub g_K: RcCell<G1_variables<ppT>>,

    pub all_G1_vars: Vec<RcCell<G1_variables<ppT>>>,
    pub all_G2_vars: Vec<RcCell<G2_variables<ppT>>>,

    pub all_G1_checkers: Vec<RcCell<G1_checker_gadgets<ppT>>>,
    pub G2_checker: RcCell<G2_checker_gadgets<ppT>>,

    pub proof_contents: pb_variable_array<ppT::FieldT, ppT::PB>,
}
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_verification_key_variable<ppT: ppTConfig> {
    //gadget<Fr<ppT,  PB> >

    // type FieldT<ppT>=Fr<ppT,  PB>;
    pub alphaA_g2: RcCell<G2_variables<ppT>>,
    pub alphaB_g1: RcCell<G1_variables<ppT>>,
    pub alphaC_g2: RcCell<G2_variables<ppT>>,
    pub gamma_g2: RcCell<G2_variables<ppT>>,
    pub gamma_beta_g1: RcCell<G1_variables<ppT>>,
    pub gamma_beta_g2: RcCell<G2_variables<ppT>>,
    pub rC_Z_g2: RcCell<G2_variables<ppT>>,
    pub encoded_IC_base: RcCell<G1_variables<ppT>>,
    pub encoded_IC_query: Vec<RcCell<G1_variables<ppT>>>,

    pub all_bits: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub all_vars: pb_linear_combination_array<ppT::FieldT, ppT::PB>,
    pub input_size: usize,

    pub all_G1_vars: Vec<RcCell<G1_variables<ppT>>>,
    pub all_G2_vars: Vec<RcCell<G2_variables<ppT>>>,

    pub packer: RcCell<multipacking_gadgets<ppT::FieldT, ppT::PB>>,
    // Unfortunately, g++ 4.9 and g++ 5.0 have a bug related to
    // incorrect inlining of small functions:
    // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=65307, which
    // produces wrong assembly even at -O1. The test case at the bug
    // report is directly derived from this code here. As a temporary
    // work-around we mark the key functions noinline to hint compiler
    // that inlining should not be performed.

    // TODO: remove later, when g++ developers fix the bug.
}
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT: ppTConfig> {
    // type FieldT<ppT>=Fr<ppT,  PB>;
    pub encoded_IC_base: RcCell<G1_variables<ppT>>,
    pub encoded_IC_query: Vec<RcCell<G1_variables<ppT>>>,

    pub vk_alphaB_g1_precomp: RcCell<G1_precomputations<ppT>>,
    pub vk_gamma_beta_g1_precomp: RcCell<G1_precomputations<ppT>>,

    pub pp_G2_one_precomp: RcCell<G2_precomputations<ppT>>,
    pub vk_alphaA_g2_precomp: RcCell<G2_precomputations<ppT>>,
    pub vk_alphaC_g2_precomp: RcCell<G2_precomputations<ppT>>,
    pub vk_gamma_beta_g2_precomp: RcCell<G2_precomputations<ppT>>,
    pub vk_gamma_g2_precomp: RcCell<G2_precomputations<ppT>>,
    pub vk_rC_Z_g2_precomp: RcCell<G2_precomputations<ppT>>,
}
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_verifier_process_vk_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT,  PB> >

    // type FieldT<ppT>=Fr<ppT,  PB>;
    pub compute_vk_alphaB_g1_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_vk_gamma_beta_g1_precomp: RcCell<precompute_G1_gadgets<ppT>>,

    pub compute_vk_alphaA_g2_precomp: RcCell<precompute_G2_gadgets<ppT>>,
    pub compute_vk_alphaC_g2_precomp: RcCell<precompute_G2_gadgets<ppT>>,
    pub compute_vk_gamma_beta_g2_precomp: RcCell<precompute_G2_gadgets<ppT>>,
    pub compute_vk_gamma_g2_precomp: RcCell<precompute_G2_gadgets<ppT>>,
    pub compute_vk_rC_Z_g2_precomp: RcCell<precompute_G2_gadgets<ppT>>,

    pub vk: r1cs_ppzksnark_verification_key_variables<ppT>,
    pub pvk: r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>, // important to have a reference here
}
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_online_verifier_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT,  PB> >

    // type FieldT<ppT>=Fr<ppT>;
    pub pvk: r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>,

    pub input: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub elt_size: usize,
    pub proof: r1cs_ppzksnark_proof_variables<ppT>,
    pub result: variable<ppT::FieldT, pb_variable>,
    pub input_len: usize,

    pub acc: RcCell<G1_variables<ppT>>,
    pub accumulate_input: RcCell<G1_multiscalar_mul_gadgets<ppT>>,

    pub proof_g_A_g_acc: RcCell<G1_variables<ppT>>,
    pub compute_proof_g_A_g_acc: RcCell<G1_add_gadgets<ppT>>,
    pub proof_g_A_g_acc_C: RcCell<G1_variables<ppT>>,
    pub compute_proof_g_A_g_acc_C: RcCell<G1_add_gadgets<ppT>>,

    pub proof_g_A_h_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_A_g_acc_C_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_A_g_acc_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_A_g_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_B_h_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_C_h_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_C_g_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_K_precomp: RcCell<G1_precomputations<ppT>>,
    pub proof_g_H_precomp: RcCell<G1_precomputations<ppT>>,

    pub proof_g_B_g_precomp: RcCell<G2_precomputations<ppT>>,

    pub compute_proof_g_A_h_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_A_g_acc_C_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_A_g_acc_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_A_g_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_B_h_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_C_h_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_C_g_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_K_precomp: RcCell<precompute_G1_gadgets<ppT>>,
    pub compute_proof_g_H_precomp: RcCell<precompute_G1_gadgets<ppT>>,

    pub compute_proof_g_B_g_precomp: RcCell<precompute_G2_gadgets<ppT>>,

    pub check_kc_A_valid: RcCell<check_e_equals_e_gadgets<ppT>>,
    pub check_kc_B_valid: RcCell<check_e_equals_e_gadgets<ppT>>,
    pub check_kc_C_valid: RcCell<check_e_equals_e_gadgets<ppT>>,
    pub check_QAP_valid: RcCell<check_e_equals_ee_gadgets<ppT>>,
    pub check_CC_valid: RcCell<check_e_equals_ee_gadgets<ppT>>,

    pub kc_A_valid: variable<ppT::FieldT, pb_variable>,
    pub kc_B_valid: variable<ppT::FieldT, pb_variable>,
    pub kc_C_valid: variable<ppT::FieldT, pb_variable>,
    pub QAP_valid: variable<ppT::FieldT, pb_variable>,
    pub CC_valid: variable<ppT::FieldT, pb_variable>,

    pub all_test_results: pb_variable_array<ppT::FieldT, ppT::PB>,
    pub all_tests_pass: RcCell<conjunction_gadgets<ppT::FieldT, ppT::PB>>,
}
#[derive(Clone, Default)]
pub struct r1cs_ppzksnark_verifier_gadget<ppT: ppTConfig> {
    //gadget<Fr<ppT,  PB> >

    // type FieldT<ppT>=Fr<ppT,  PB>;
    pub pvk: RcCell<r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>>,
    pub compute_pvk: RcCell<r1cs_ppzksnark_verifier_process_vk_gadgets<ppT>>,
    pub online_verifier: RcCell<r1cs_ppzksnark_online_verifier_gadgets<ppT>>,
}

pub type r1cs_ppzksnark_proof_variables<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, r1cs_ppzksnark_proof_variable<ppT>>;

impl<ppT: ppTConfig> r1cs_ppzksnark_proof_variable<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_proof_variables<ppT> {
        let num_G1 = 7;
        let num_G2 = 1;

        let g_A_g = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_A_g"),
        ));
        let g_A_h = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_A_h"),
        ));
        let g_B_g = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_B_g"),
        ));
        let g_B_h = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_B_h"),
        ));
        let g_C_g = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_C_g"),
        ));
        let g_C_h = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_C_h"),
        ));
        let g_H = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_H"),
        ));
        let g_K = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " g_K"),
        ));

        let mut all_G1_vars = vec![
            g_A_g.clone(),
            g_A_h.clone(),
            g_B_h.clone(),
            g_C_g.clone(),
            g_C_h.clone(),
            g_H.clone(),
            g_K.clone(),
        ];
        let mut all_G2_vars = vec![g_B_g.clone()];

        let mut all_G1_checkers =
            vec![RcCell::new(G1_checker_gadgets::<ppT>::default()); all_G1_vars.len()];

        for i in 0..all_G1_vars.len() {
            all_G1_checkers[i] = RcCell::new(G1_checker_gadget::<ppT>::new(
                pb.clone(),
                all_G1_vars[i].borrow().clone(),
                prefix_format!(annotation_prefix, " all_G1_checkers_{}", i),
            ));
        }
        let G2_checker = RcCell::new(G2_checker_gadget::<ppT>::new(
            pb.clone(),
            g_B_g.borrow().clone(),
            prefix_format!(annotation_prefix, " G2_checker"),
        ));

        assert!(all_G1_vars.len() == num_G1);
        assert!(all_G2_vars.len() == num_G2);
        let proof_contents = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                g_A_g,
                g_A_h,
                g_B_g,
                g_B_h,
                g_C_g,
                g_C_h,
                g_H,
                g_K,

                all_G1_vars,
                all_G2_vars,

                all_G1_checkers,
                G2_checker,

                proof_contents,
            },
        )
    }
}
impl<ppT: ppTConfig> r1cs_ppzksnark_proof_variables<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        for G1_checker in &self.t.all_G1_checkers {
            G1_checker.borrow().generate_r1cs_constraints();
        }

        self.t.G2_checker.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self, proof: &r1cs_ppzksnark_proof<other_curve<ppT>>)
    // where
    //     ppT::other_curve_type: ppTConfig,
    {
        let G1_elems = vec![
            proof.g_A.g.clone(),
            proof.g_A.h.clone(),
            proof.g_B.h.clone(),
            proof.g_C.g.clone(),
            proof.g_C.h.clone(),
            proof.g_H.clone().into(),
            proof.g_K.clone().into(),
        ];
        let G2_elems = vec![proof.g_B.g.clone()];

        assert_eq!(G1_elems.len(), self.t.all_G1_vars.len());
        assert_eq!(G2_elems.len(), self.t.all_G2_vars.len());

        for i in 0..G1_elems.len() {
            self.t.all_G1_vars[i]
                .borrow()
                .generate_r1cs_witness(&(G1_elems[i].clone().into()));
        }

        for i in 0..G2_elems.len() {
            self.t.all_G2_vars[i]
                .borrow()
                .generate_r1cs_witness(&(G2_elems[i]));
        }

        for G1_checker in &self.t.all_G1_checkers {
            G1_checker.borrow().generate_r1cs_witness();
        }

        self.t.G2_checker.borrow().generate_r1cs_witness();
    }

    pub fn size() -> usize {
        let num_G1 = 7;
        let num_G2 = 1;
        num_G1 * G1_variable::<ppT>::num_field_elems()
            + num_G2 * G2_variable::<ppT>::num_field_elems()
    }
}
pub type r1cs_ppzksnark_verification_key_variables<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    r1cs_ppzksnark_verification_key_variable<ppT>,
>;
impl<ppT: ppTConfig> r1cs_ppzksnark_verification_key_variable<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        all_bits: pb_variable_array<ppT::FieldT, ppT::PB>,
        input_size: usize,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_verification_key_variables<ppT> {
        let num_G1 = 2 + (input_size + 1);
        let num_G2 = 5;

        assert!(
            all_bits.len()
                == (G1_variable::<ppT>::size_in_bits() * num_G1
                    + G2_variable::<ppT>::size_in_bits() * num_G2)
        );

        let alphaA_g2 = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " alphaA_g2"),
        ));
        let alphaB_g1 = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " alphaB_g1"),
        ));
        let alphaC_g2 = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " alphaC_g2"),
        ));
        let gamma_g2 = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_g2"),
        ));
        let gamma_beta_g1 = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_beta_g1"),
        ));
        let gamma_beta_g2 = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " gamma_beta_g2"),
        ));
        let rC_Z_g2 = RcCell::new(G2_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " rC_Z_g2"),
        ));

        let mut all_G1_vars = vec![alphaB_g1.clone(), gamma_beta_g1.clone()];
        let mut all_G2_vars = vec![
            alphaA_g2.clone(),
            alphaC_g2.clone(),
            gamma_g2.clone(),
            gamma_beta_g2.clone(),
            rC_Z_g2.clone(),
        ];

        let mut encoded_IC_query = vec![RcCell::new(G1_variables::<ppT>::default()); input_size];
        let encoded_IC_base = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " encoded_IC_base"),
        ));
        all_G1_vars.push(encoded_IC_base.clone());

        for i in 0..input_size {
            encoded_IC_query[i] = RcCell::new(G1_variable::<ppT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " encoded_IC_query_{}", i),
            ));
            all_G1_vars.push(encoded_IC_query[i].clone());
        }
        let mut all_vars = pb_linear_combination_array::<ppT::FieldT, ppT::PB>::default();
        for G1_var in &all_G1_vars {
            all_vars.contents.extend(G1_var.borrow().t.all_vars.clone());
        }

        for G2_var in &all_G2_vars {
            all_vars.contents.extend(G2_var.borrow().t.all_vars.clone());
        }

        assert!(all_G1_vars.len() == num_G1);
        assert!(all_G2_vars.len() == num_G2);
        assert!(
            all_vars.len()
                == (num_G1 * G1_variable::<ppT>::num_variables()
                    + num_G2 * G2_variable::<ppT>::num_variables())
        );

        let packer = RcCell::new(multipacking_gadget::<ppT::FieldT, ppT::PB>::new(
            pb.clone(),
            all_bits.clone().into(),
            all_vars.clone(),
            FieldT::<ppT>::size_in_bits(),
            prefix_format!(annotation_prefix, " packer"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                alphaA_g2,
                alphaB_g1,
                alphaC_g2,
                gamma_g2,
                gamma_beta_g1,
                gamma_beta_g2,
                rC_Z_g2,
                encoded_IC_base,
                encoded_IC_query,

                all_bits,
                all_vars,
                input_size,

                all_G1_vars,
                all_G2_vars,

                packer,
            },
        )
    }
    pub fn size_in_bits(input_size: usize) -> usize {
        let num_G1 = 2 + (input_size + 1);
        let num_G2 = 5;
        let result = G1_variable::<ppT>::size_in_bits() * num_G1
            + G2_variable::<ppT>::size_in_bits() * num_G2;
        print!(
            "G1_size_in_bits = {}, G2_size_in_bits = {}\n",
            G1_variable::<ppT>::size_in_bits(),
            G2_variable::<ppT>::size_in_bits()
        );
        print!(
            "r1cs_ppzksnark_verification_key_variable::<ppT,  PB>::size_in_bits({}) = {}\n",
            input_size, result
        );
        result
    }
    pub fn get_verification_key_bits(
        r1cs_vk: &r1cs_ppzksnark_verification_key<other_curve<ppT>>,
    ) -> bit_vector {
        // type FieldT<ppT> = Fr<ppT,  PB>;

        let input_size_in_elts = r1cs_vk.encoded_IC_query.rest.indices.len(); // this might be approximate for bound verification keys, however they are not supported by r1cs_ppzksnark_verification_key_variable
        let vk_size_in_bits =
            r1cs_ppzksnark_verification_key_variable::<ppT>::size_in_bits(input_size_in_elts);

        let mut pb = RcCell::new(protoboard::<ppT::FieldT, ppT::PB>::default());
        let mut vk_bits = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        vk_bits.allocate(&pb, vk_size_in_bits, "vk_bits");
        let mut vk = r1cs_ppzksnark_verification_key_variable::<ppT>::new(
            pb.clone(),
            vk_bits.clone(),
            input_size_in_elts.clone(),
            "translation_step_vk".to_owned(),
        );
        vk.generate_r1cs_witness(r1cs_vk);

        vk.get_bits()
    }
}
impl<ppT: ppTConfig> r1cs_ppzksnark_verification_key_variables<ppT> {
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        self.t
            .packer
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
    }

    pub fn generate_r1cs_witness(&self, vk: &r1cs_ppzksnark_verification_key<other_curve<ppT>>) {
        let mut G1_elems = vec![vk.alphaB_g1.clone(), vk.gamma_beta_g1.clone()];
        let mut G2_elems = vec![
            vk.alphaA_g2.clone(),
            vk.alphaC_g2.clone(),
            vk.gamma_g2.clone(),
            vk.gamma_beta_g2.clone(),
            vk.rC_Z_g2.clone(),
        ];

        assert!(vk.encoded_IC_query.rest.indices.len() == self.t.input_size);
        G1_elems.push(vk.encoded_IC_query.first.clone());
        for i in 0..self.t.input_size {
            assert!(vk.encoded_IC_query.rest.indices[i] == i);
            G1_elems.push(vk.encoded_IC_query.rest.values[i].clone());
        }

        assert!(G1_elems.len() == self.t.all_G1_vars.len());
        assert!(G2_elems.len() == self.t.all_G2_vars.len());

        for i in 0..G1_elems.len() {
            self.t.all_G1_vars[i]
                .borrow()
                .generate_r1cs_witness(&G1_elems[i]);
        }

        for i in 0..G2_elems.len() {
            self.t.all_G2_vars[i]
                .borrow()
                .generate_r1cs_witness(&G2_elems[i]);
        }

        self.t.packer.borrow().generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_witnessv(&self, vk_bits: &bit_vector) {
        self.t.all_bits.fill_with_bits(&self.pb, vk_bits);
        self.t.packer.borrow().generate_r1cs_witness_from_bits();
    }

    pub fn get_bits(&self) -> bit_vector {
        self.t.all_bits.get_bits(&self.pb)
    }
}
pub type r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT>,
>;
impl<ppT: ppTConfig> r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        r1cs_vk: r1cs_ppzksnark_verification_key<other_curve<ppT>>,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>
// where
    //     <ppT::other_curve_type as ff_curves::PublicParams>::Fr: Mul<
    //             <ppT as ff_curves::PublicParams>::Fr,
    //             Output = <ppT::other_curve_type as ff_curves::PublicParams>::Fr,
    //         >,
    {
        let encoded_IC_base = RcCell::new(G1_variable::<ppT>::new2(
            pb.clone(),
            r1cs_vk.encoded_IC_query.first.clone(),
            prefix_format!(annotation_prefix, " encoded_IC_base"),
        ));
        let mut encoded_IC_query = vec![
            RcCell::new(G1_variables::<ppT>::default());
            r1cs_vk.encoded_IC_query.rest.indices.len()
        ];
        for i in 0..r1cs_vk.encoded_IC_query.rest.indices.len() {
            assert!(r1cs_vk.encoded_IC_query.rest.indices[i] == i);
            encoded_IC_query[i] = RcCell::new(G1_variable::<ppT>::new2(
                pb.clone(),
                r1cs_vk.encoded_IC_query.rest.values[i].clone(),
                prefix_format!(annotation_prefix, " encoded_IC_query"),
            ));
        }

        let vk_alphaB_g1_precomp = RcCell::new(G1_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.alphaB_g1.clone(),
            prefix_format!(annotation_prefix, " vk_alphaB_g1_precomp"),
        ));
        let vk_gamma_beta_g1_precomp = RcCell::new(G1_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.gamma_beta_g1.clone(),
            prefix_format!(annotation_prefix, " vk_gamma_beta_g1_precomp"),
        ));

        let pp_G2_one_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            G2::<other_curve<ppT>>::one(),
            prefix_format!(annotation_prefix, " pp_G2_one_precomp"),
        ));
        let vk_alphaA_g2_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.alphaA_g2.clone(),
            prefix_format!(annotation_prefix, " vk_alphaA_g2_precomp"),
        ));
        let vk_alphaC_g2_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.alphaC_g2.clone(),
            prefix_format!(annotation_prefix, " vk_alphaC_g2_precomp"),
        ));
        let vk_gamma_beta_g2_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.gamma_beta_g2.clone(),
            prefix_format!(annotation_prefix, " vk_gamma_beta_g2_precomp"),
        ));
        let vk_gamma_g2_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.gamma_g2.clone(),
            prefix_format!(annotation_prefix, " vk_gamma_g2_precomp"),
        ));
        let vk_rC_Z_g2_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            r1cs_vk.rC_Z_g2.clone(),
            prefix_format!(annotation_prefix, " vk_rC_Z_g2_precomp"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb.clone(),
            annotation_prefix,
            Self {
                encoded_IC_base,
                encoded_IC_query,

                vk_alphaB_g1_precomp,
                vk_gamma_beta_g1_precomp,

                pp_G2_one_precomp,
                vk_alphaA_g2_precomp,
                vk_alphaC_g2_precomp,
                vk_gamma_beta_g2_precomp,
                vk_gamma_g2_precomp,
                vk_rC_Z_g2_precomp,
            },
        )
    }
}

pub type r1cs_ppzksnark_verifier_process_vk_gadgets<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    r1cs_ppzksnark_verifier_process_vk_gadget<ppT>,
>;
impl<ppT: ppTConfig> r1cs_ppzksnark_verifier_process_vk_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        vk: r1cs_ppzksnark_verification_key_variables<ppT>,
        mut pvk: r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_verifier_process_vk_gadgets<ppT> {
        pvk.t.encoded_IC_base = vk.t.encoded_IC_base.clone();
        pvk.t.encoded_IC_query = vk.t.encoded_IC_query.clone();

        pvk.t.vk_alphaB_g1_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        pvk.t.vk_gamma_beta_g1_precomp = RcCell::new(G1_precomputations::<ppT>::default());

        pvk.t.pp_G2_one_precomp = RcCell::new(G2_precomputations::<ppT>::default());
        pvk.t.vk_alphaA_g2_precomp = RcCell::new(G2_precomputations::<ppT>::default());
        pvk.t.vk_alphaC_g2_precomp = RcCell::new(G2_precomputations::<ppT>::default());
        pvk.t.vk_gamma_beta_g2_precomp = RcCell::new(G2_precomputations::<ppT>::default());
        pvk.t.vk_gamma_g2_precomp = RcCell::new(G2_precomputations::<ppT>::default());
        pvk.t.vk_rC_Z_g2_precomp = RcCell::new(G2_precomputations::<ppT>::default());

        let compute_vk_alphaB_g1_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            vk.t.alphaB_g1.borrow().clone(),
            pvk.t.vk_alphaB_g1_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_alphaB_g1_precomp"),
        ));
        let compute_vk_gamma_beta_g1_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            vk.t.gamma_beta_g1.borrow().clone(),
            pvk.t.vk_gamma_beta_g1_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_gamma_beta_g1_precomp"),
        ));

        pvk.t.pp_G2_one_precomp = RcCell::new(G2_precomputation::<ppT>::new(
            pb.clone(),
            G2::<other_curve<ppT>>::one(),
            prefix_format!(annotation_prefix, " pp_G2_one_precomp"),
        ));
        let compute_vk_alphaA_g2_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            vk.t.alphaA_g2.borrow().clone(),
            pvk.t.vk_alphaA_g2_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_alphaA_g2_precomp"),
        ));
        let compute_vk_alphaC_g2_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            vk.t.alphaC_g2.borrow().clone(),
            pvk.t.vk_alphaC_g2_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_alphaC_g2_precomp"),
        ));
        let compute_vk_gamma_beta_g2_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            vk.t.gamma_beta_g2.borrow().clone(),
            pvk.t.vk_gamma_beta_g2_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_gamma_beta_g2_precomp"),
        ));
        let compute_vk_gamma_g2_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            vk.t.gamma_g2.borrow().clone(),
            pvk.t.vk_gamma_g2_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_gamma_g2_precomp"),
        ));
        let compute_vk_rC_Z_g2_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            vk.t.rC_Z_g2.borrow().clone(),
            pvk.t.vk_rC_Z_g2_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_vk_rC_Z_g2_precomp"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                compute_vk_alphaB_g1_precomp,
                compute_vk_gamma_beta_g1_precomp,

                compute_vk_alphaA_g2_precomp,
                compute_vk_alphaC_g2_precomp,
                compute_vk_gamma_beta_g2_precomp,
                compute_vk_gamma_g2_precomp,
                compute_vk_rC_Z_g2_precomp,

                vk,
                pvk,
            },
        )
    }
}
impl<ppT: ppTConfig> r1cs_ppzksnark_verifier_process_vk_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .compute_vk_alphaB_g1_precomp
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_vk_gamma_beta_g1_precomp
            .borrow()
            .generate_r1cs_constraints();

        self.t
            .compute_vk_alphaA_g2_precomp
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_vk_alphaC_g2_precomp
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_vk_gamma_beta_g2_precomp
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_vk_gamma_g2_precomp
            .borrow()
            .generate_r1cs_constraints();
        self.t
            .compute_vk_rC_Z_g2_precomp
            .borrow()
            .generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t
            .compute_vk_alphaB_g1_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_vk_gamma_beta_g1_precomp
            .borrow()
            .generate_r1cs_witness();

        self.t
            .compute_vk_alphaA_g2_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_vk_alphaC_g2_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_vk_gamma_beta_g2_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_vk_gamma_g2_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_vk_rC_Z_g2_precomp
            .borrow()
            .generate_r1cs_witness();
    }
}

pub type r1cs_ppzksnark_online_verifier_gadgets<ppT> = gadget<
    <ppT as ppTConfig>::FieldT,
    <ppT as ppTConfig>::PB,
    r1cs_ppzksnark_online_verifier_gadget<ppT>,
>;
impl<ppT: ppTConfig> r1cs_ppzksnark_online_verifier_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        pvk: r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables<ppT>,
        input: pb_variable_array<ppT::FieldT, ppT::PB>,
        elt_size: usize,
        proof: r1cs_ppzksnark_proof_variables<ppT>,
        result: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_online_verifier_gadgets<ppT> {
        let input_len = input.len();
        // accumulate input and store base in acc
        let acc = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " acc"),
        ));
        let mut IC_terms = vec![];
        for i in 0..pvk.t.encoded_IC_query.len() {
            IC_terms.push(pvk.t.encoded_IC_query[i].borrow().clone());
        }
        let accumulate_input = RcCell::new(G1_multiscalar_mul_gadget::<ppT>::new(
            pb.clone(),
            pvk.t.encoded_IC_base.borrow().clone(),
            input.clone(),
            elt_size,
            IC_terms.clone(),
            acc.borrow().clone(),
            prefix_format!(annotation_prefix, " accumulate_input"),
        ));

        // allocate results for precomputation
        let proof_g_A_h_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_A_g_acc_C_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_A_g_acc_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_A_g_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_B_h_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_C_h_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_C_g_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_K_precomp = RcCell::new(G1_precomputations::<ppT>::default());
        let proof_g_H_precomp = RcCell::new(G1_precomputations::<ppT>::default());

        let proof_g_B_g_precomp = RcCell::new(G2_precomputations::<ppT>::default());

        // do the necessary precomputations
        // compute things not available in plain from proof/vk
        let proof_g_A_g_acc = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " proof_g_A_g_acc"),
        ));
        let compute_proof_g_A_g_acc = RcCell::new(G1_add_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_A_g.borrow().clone(),
            acc.borrow().clone(),
            proof_g_A_g_acc.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_g_acc"),
        ));
        let proof_g_A_g_acc_C = RcCell::new(G1_variable::<ppT>::new(
            pb.clone(),
            prefix_format!(annotation_prefix, " proof_g_A_g_acc_C"),
        ));
        let compute_proof_g_A_g_acc_C = RcCell::new(G1_add_gadget::<ppT>::new(
            pb.clone(),
            proof_g_A_g_acc.borrow().clone(),
            proof.t.g_C_g.borrow().clone(),
            proof_g_A_g_acc_C.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_g_acc_C"),
        ));

        let compute_proof_g_A_g_acc_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof_g_A_g_acc.borrow().clone(),
            proof_g_A_g_acc_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_g_acc_precomp"),
        ));
        let compute_proof_g_A_g_acc_C_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof_g_A_g_acc_C.borrow().clone(),
            proof_g_A_g_acc_C_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_g_acc_C_precomp"),
        ));

        // do other precomputations
        let compute_proof_g_A_h_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_A_h.borrow().clone(),
            proof_g_A_h_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_h_precomp"),
        ));
        let compute_proof_g_A_g_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_A_g.borrow().clone(),
            proof_g_A_g_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_A_g_precomp"),
        ));
        let compute_proof_g_B_h_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_B_h.borrow().clone(),
            proof_g_B_h_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_B_h_precomp"),
        ));
        let compute_proof_g_C_h_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_C_h.borrow().clone(),
            proof_g_C_h_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_C_h_precomp"),
        ));
        let compute_proof_g_C_g_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_C_g.borrow().clone(),
            proof_g_C_g_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_C_g_precomp"),
        ));
        let compute_proof_g_H_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_H.borrow().clone(),
            proof_g_H_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_H_precomp"),
        ));
        let compute_proof_g_K_precomp = RcCell::new(precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_K.borrow().clone(),
            proof_g_K_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_K_precomp"),
        ));
        let compute_proof_g_B_g_precomp = RcCell::new(precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            proof.t.g_B_g.borrow().clone(),
            proof_g_B_g_precomp.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_proof_g_B_g_precomp"),
        ));

        // check validity of A knowledge commitment
        let mut kc_A_valid = variable::<ppT::FieldT, pb_variable>::default();
        kc_A_valid.allocate(&pb, prefix_format!(annotation_prefix, " kc_A_valid"));
        let check_kc_A_valid = RcCell::new(check_e_equals_e_gadget::<ppT>::new(
            pb.clone(),
            proof_g_A_g_precomp.borrow().clone(),
            pvk.t.vk_alphaA_g2_precomp.borrow().clone(),
            proof_g_A_h_precomp.borrow().clone(),
            pvk.t.pp_G2_one_precomp.borrow().clone(),
            kc_A_valid.clone(),
            prefix_format!(annotation_prefix, " check_kc_A_valid"),
        ));

        // check validity of B knowledge commitment
        let mut kc_B_valid = variable::<ppT::FieldT, pb_variable>::default();
        kc_B_valid.allocate(&pb, prefix_format!(annotation_prefix, " kc_B_valid"));
        let check_kc_B_valid = RcCell::new(check_e_equals_e_gadget::<ppT>::new(
            pb.clone(),
            pvk.t.vk_alphaB_g1_precomp.borrow().clone(),
            proof_g_B_g_precomp.borrow().clone(),
            proof_g_B_h_precomp.borrow().clone(),
            pvk.t.pp_G2_one_precomp.borrow().clone(),
            kc_B_valid.clone(),
            prefix_format!(annotation_prefix, " check_kc_B_valid"),
        ));

        // check validity of C knowledge commitment
        let mut kc_C_valid = variable::<ppT::FieldT, pb_variable>::default();
        kc_C_valid.allocate(&pb, prefix_format!(annotation_prefix, " kc_C_valid"));
        let check_kc_C_valid = RcCell::new(check_e_equals_e_gadget::<ppT>::new(
            pb.clone(),
            proof_g_C_g_precomp.borrow().clone(),
            pvk.t.vk_alphaC_g2_precomp.borrow().clone(),
            proof_g_C_h_precomp.borrow().clone(),
            pvk.t.pp_G2_one_precomp.borrow().clone(),
            kc_C_valid.clone(),
            prefix_format!(annotation_prefix, " check_kc_C_valid"),
        ));

        // check QAP divisibility
        let mut QAP_valid = variable::<ppT::FieldT, pb_variable>::default();
        QAP_valid.allocate(&pb, prefix_format!(annotation_prefix, " QAP_valid"));
        let check_QAP_valid = RcCell::new(check_e_equals_ee_gadget::<ppT>::new(
            pb.clone(),
            proof_g_A_g_acc_precomp.borrow().clone(),
            proof_g_B_g_precomp.borrow().clone(),
            proof_g_H_precomp.borrow().clone(),
            pvk.t.vk_rC_Z_g2_precomp.borrow().clone(),
            proof_g_C_g_precomp.borrow().clone(),
            pvk.t.pp_G2_one_precomp.borrow().clone(),
            QAP_valid.clone(),
            prefix_format!(annotation_prefix, " check_QAP_valid"),
        ));

        // check coefficients
        let mut CC_valid = variable::<ppT::FieldT, pb_variable>::default();
        CC_valid.allocate(&pb, prefix_format!(annotation_prefix, " CC_valid"));
        let check_CC_valid = RcCell::new(check_e_equals_ee_gadget::<ppT>::new(
            pb.clone(),
            proof_g_K_precomp.borrow().clone(),
            pvk.t.vk_gamma_g2_precomp.borrow().clone(),
            proof_g_A_g_acc_C_precomp.borrow().clone(),
            pvk.t.vk_gamma_beta_g2_precomp.borrow().clone(),
            pvk.t.vk_gamma_beta_g1_precomp.borrow().clone(),
            proof_g_B_g_precomp.borrow().clone(),
            CC_valid.clone(),
            prefix_format!(annotation_prefix, " check_CC_valid"),
        ));

        let mut all_test_results = pb_variable_array::<ppT::FieldT, ppT::PB>::default();
        // final constraint
        all_test_results.contents.push(kc_A_valid.clone());
        all_test_results.contents.push(kc_B_valid.clone());
        all_test_results.contents.push(kc_C_valid.clone());
        all_test_results.contents.push(QAP_valid.clone());
        all_test_results.contents.push(CC_valid.clone());

        let all_tests_pass = RcCell::new(conjunction_gadget::<ppT::FieldT, ppT::PB>::new(
            pb.clone(),
            all_test_results.clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " all_tests_pass"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                pvk,
                input,
                elt_size,
                proof,
                result,
                input_len,
                acc,
                accumulate_input,

                proof_g_A_g_acc,
                compute_proof_g_A_g_acc,
                proof_g_A_g_acc_C,
                compute_proof_g_A_g_acc_C,

                proof_g_A_h_precomp,
                proof_g_A_g_acc_C_precomp,
                proof_g_A_g_acc_precomp,
                proof_g_A_g_precomp,
                proof_g_B_h_precomp,
                proof_g_C_h_precomp,
                proof_g_C_g_precomp,
                proof_g_K_precomp,
                proof_g_H_precomp,

                proof_g_B_g_precomp,

                compute_proof_g_A_h_precomp,
                compute_proof_g_A_g_acc_C_precomp,
                compute_proof_g_A_g_acc_precomp,
                compute_proof_g_A_g_precomp,
                compute_proof_g_B_h_precomp,
                compute_proof_g_C_h_precomp,
                compute_proof_g_C_g_precomp,
                compute_proof_g_K_precomp,
                compute_proof_g_H_precomp,

                compute_proof_g_B_g_precomp,

                check_kc_A_valid,
                check_kc_B_valid,
                check_kc_C_valid,
                check_QAP_valid,
                check_CC_valid,

                kc_A_valid,
                kc_B_valid,
                kc_C_valid,
                QAP_valid,
                CC_valid,

                all_test_results,
                all_tests_pass,
            },
        )
    }
}
impl<ppT: ppTConfig> r1cs_ppzksnark_online_verifier_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        PROFILE_CONSTRAINTS(&self.pb, "accumulate verifier input");
        {
            print_indent();
            print!(
                "* Number of bits as an input to verifier gadget: {}\n",
                self.t.input.len()
            );
            self.t.accumulate_input.borrow().generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTS(&self.pb, "rest of the verifier");
        {
            self.t
                .compute_proof_g_A_g_acc
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_A_g_acc_C
                .borrow()
                .generate_r1cs_constraints();

            self.t
                .compute_proof_g_A_g_acc_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_A_g_acc_C_precomp
                .borrow()
                .generate_r1cs_constraints();

            self.t
                .compute_proof_g_A_h_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_A_g_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_B_h_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_C_h_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_C_g_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_H_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_K_precomp
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .compute_proof_g_B_g_precomp
                .borrow()
                .generate_r1cs_constraints();

            self.t.check_kc_A_valid.borrow().generate_r1cs_constraints();
            self.t.check_kc_B_valid.borrow().generate_r1cs_constraints();
            self.t.check_kc_C_valid.borrow().generate_r1cs_constraints();
            self.t.check_QAP_valid.borrow().generate_r1cs_constraints();
            self.t.check_CC_valid.borrow().generate_r1cs_constraints();

            self.t.all_tests_pass.borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.accumulate_input.borrow().generate_r1cs_witness();

        self.t
            .compute_proof_g_A_g_acc
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_A_g_acc_C
            .borrow()
            .generate_r1cs_witness();

        self.t
            .compute_proof_g_A_g_acc_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_A_g_acc_C_precomp
            .borrow()
            .generate_r1cs_witness();

        self.t
            .compute_proof_g_A_h_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_A_g_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_B_h_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_C_h_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_C_g_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_H_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_K_precomp
            .borrow()
            .generate_r1cs_witness();
        self.t
            .compute_proof_g_B_g_precomp
            .borrow()
            .generate_r1cs_witness();

        self.t.check_kc_A_valid.borrow().generate_r1cs_witness();
        self.t.check_kc_B_valid.borrow().generate_r1cs_witness();
        self.t.check_kc_C_valid.borrow().generate_r1cs_witness();
        self.t.check_QAP_valid.borrow().generate_r1cs_witness();
        self.t.check_CC_valid.borrow().generate_r1cs_witness();

        self.t.all_tests_pass.borrow().generate_r1cs_witness();
    }
}

pub type r1cs_ppzksnark_verifier_gadgets<ppT> =
    gadget<<ppT as ppTConfig>::FieldT, <ppT as ppTConfig>::PB, r1cs_ppzksnark_verifier_gadget<ppT>>;
impl<ppT: ppTConfig> r1cs_ppzksnark_verifier_gadget<ppT> {
    pub fn new(
        pb: RcCell<protoboard<ppT::FieldT, ppT::PB>>,
        vk: r1cs_ppzksnark_verification_key_variables<ppT>,
        input: pb_variable_array<ppT::FieldT, ppT::PB>,
        elt_size: usize,
        proof: r1cs_ppzksnark_proof_variables<ppT>,
        result: variable<ppT::FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> r1cs_ppzksnark_verifier_gadgets<ppT> {
        let pvk = RcCell::new(
            r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variables::<ppT>::default(),
        );
        let compute_pvk = RcCell::new(r1cs_ppzksnark_verifier_process_vk_gadget::<ppT>::new(
            pb.clone(),
            vk.clone(),
            pvk.borrow().clone(),
            prefix_format!(annotation_prefix, " compute_pvk"),
        ));
        let online_verifier = RcCell::new(r1cs_ppzksnark_online_verifier_gadget::<ppT>::new(
            pb.clone(),
            pvk.borrow().clone(),
            input.clone(),
            elt_size,
            proof.clone(),
            result.clone(),
            prefix_format!(annotation_prefix, " online_verifier"),
        ));
        gadget::<ppT::FieldT, ppT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                pvk,
                compute_pvk,
                online_verifier,
            },
        )
    }
}
impl<ppT: ppTConfig> r1cs_ppzksnark_verifier_gadgets<ppT> {
    pub fn generate_r1cs_constraints(&self) {
        PROFILE_CONSTRAINTS(&self.pb, "precompute pvk");
        {
            self.t.compute_pvk.borrow().generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTS(&self.pb, "online verifier");
        {
            self.t.online_verifier.borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_pvk.borrow().generate_r1cs_witness();
        self.t.online_verifier.borrow().generate_r1cs_witness();
    }
}
