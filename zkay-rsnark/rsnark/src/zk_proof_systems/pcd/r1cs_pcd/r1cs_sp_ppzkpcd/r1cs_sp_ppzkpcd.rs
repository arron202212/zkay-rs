// Declaration of interfaces for a *single-predicate* ppzkPCD for R1CS.

// This includes:
// - pub struct for proving key
// - pub struct for verification key
// - pub struct for processed verification key
// - pub struct for key pair (proving key & verification key)
// - pub struct for proof
// - generator algorithm
// - prover algorithm
// - verifier algorithm
// - online verifier algorithm

// The implementation follows, extends, and optimizes the approach described
// in \[BCTV14]. Thus, PCD is constructed from two "matched" ppzkSNARKs for R1CS.

// Acronyms:

// "R1CS" = "Rank-1 Constraint Systems"
// "ppzkSNARK" = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
// "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

// References:

// \[BCTV14]:
// "Scalable Zero Knowledge via Cycles of Elliptic Curves",
// Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
// CRYPTO 2014,
// <http://eprint.iacr.org/2014/595>

use crate::gadgetlib1::gadgets::pairing::pairing_params::pairing_selector;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget::r1cs_ppzksnark_verification_key_variable;
use crate::gadgetlib1::protoboard::PBConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::R1csPcdLocalDataConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::R1csPcdMessageConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdConfigPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_params::{
    r1cs_sp_ppzkpcd_auxiliary_input, r1cs_sp_ppzkpcd_compliance_predicate,
    r1cs_sp_ppzkpcd_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::sp_pcd_circuits::{
    get_sp_compliance_step_pcd_circuit_input, get_sp_translation_step_pcd_circuit_input,
    sp_compliance_step_pcd_circuit_maker, sp_translation_step_pcd_circuit_maker,
};
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark_proof;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_generator, r1cs_ppzksnark_keypair, r1cs_ppzksnark_online_verifier_strong_IC,
    r1cs_ppzksnark_processed_verification_key, r1cs_ppzksnark_prover, r1cs_ppzksnark_proving_key,
    r1cs_ppzksnark_verification_key, r1cs_ppzksnark_verifier_process_vk,
    r1cs_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::bit_vector;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};

/******************************** Proving key ********************************/

/**
 * A proving key for the R1CS (single-predicate) ppzkPCD.
 */
//
type A_pp<PCD_ppT> = <PCD_ppT as PcdConfigPptConfig>::curve_A_pp;
type B_pp<PCD_ppT> = <PCD_ppT as PcdConfigPptConfig>::curve_B_pp;

#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_proving_key<PCD_ppT: PcdConfigPptConfig> {
    pub compliance_predicate: r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,

    pub compliance_step_r1cs_pk: r1cs_ppzksnark_proving_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_pk: r1cs_ppzksnark_proving_key<B_pp<PCD_ppT>>,

    pub compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
}
impl<PCD_ppT: PcdConfigPptConfig> r1cs_sp_ppzkpcd_proving_key<PCD_ppT> {
    pub fn new(
        compliance_predicate: r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,
        compliance_step_r1cs_pk: r1cs_ppzksnark_proving_key<A_pp<PCD_ppT>>,
        translation_step_r1cs_pk: r1cs_ppzksnark_proving_key<B_pp<PCD_ppT>>,
        compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
        translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
    ) -> Self {
        Self {
            compliance_predicate,
            compliance_step_r1cs_pk,
            translation_step_r1cs_pk,
            compliance_step_r1cs_vk,
            translation_step_r1cs_vk,
        }
    }
}

/******************************* Verification key ****************************/

/**
 * A verification key for the R1CS (single-predicate) ppzkPCD.
 */
//    type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_verification_key<PCD_ppT: PcdConfigPptConfig> {
    pub compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
}
impl<PCD_ppT: PcdConfigPptConfig> r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn new(
        compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
        translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
    ) -> Self {
        Self {
            compliance_step_r1cs_vk,
            translation_step_r1cs_vk,
        }
    }

    pub fn size_in_bits(&self) -> usize {
        self.compliance_step_r1cs_vk.size_in_bits() + self.translation_step_r1cs_vk.size_in_bits()
    }
}

/************************ Processed verification key *************************/

/**
 * A processed verification key for the R1CS (single-predicate) ppzkPCD.
 *
 * Compared to a (non-processed) verification key, a processed verification key
 * contains a small constant amount of additional pre-computed information that
 * enables a faster verification time.
 */
//  type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT: PcdConfigPptConfig> {
    pub compliance_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<B_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk_bits: bit_vector,
}
impl<PCD_ppT: PcdConfigPptConfig> r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> {
    pub fn new(
        compliance_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<A_pp<PCD_ppT>>,
        translation_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<B_pp<PCD_ppT>>,
        translation_step_r1cs_vk_bits: bit_vector,
    ) -> Self {
        Self {
            compliance_step_r1cs_pvk,
            translation_step_r1cs_pvk,
            translation_step_r1cs_vk_bits,
        }
    }

    pub fn size_in_bits(&self) -> usize {
        self.compliance_step_r1cs_pvk.size_in_bits()
            + self.translation_step_r1cs_pvk.size_in_bits()
            + self.translation_step_r1cs_vk_bits.len()
    }
}

/********************************* Key pair **********************************/

/**
 * A key pair for the R1CS (single-predicate) ppzkPC, which consists of a proving key and a verification key.
 */
//    type A_pp<PCD_ppT>= <PCD_ppT as PcdConfigPptConfig>::curve_A_pp;
//     type B_pp<PCD_ppT>= <PCD_ppT as PcdConfigPptConfig>::curve_B_pp;
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_keypair<PCD_ppT: PcdConfigPptConfig> {
    pub pk: r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
    pub vk: r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
}
impl<PCD_ppT: PcdConfigPptConfig> r1cs_sp_ppzkpcd_keypair<PCD_ppT> {
    pub fn new(
        pk: r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
        vk: r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
    ) -> Self {
        Self { pk, vk }
    }
    pub fn new2(
        kp_A: r1cs_ppzksnark_keypair<A_pp<PCD_ppT>>,
        kp_B: r1cs_ppzksnark_keypair<B_pp<PCD_ppT>>,
    ) -> Self {
        Self {
            pk: r1cs_sp_ppzkpcd_proving_key::<PCD_ppT>::new(
                r1cs_sp_ppzkpcd_compliance_predicate::<PCD_ppT>::default(),
                kp_A.pk,
                kp_B.pk,
                r1cs_ppzksnark_verification_key::<A_pp<PCD_ppT>>::default(),
                r1cs_ppzksnark_verification_key::<B_pp<PCD_ppT>>::default(),
            ),
            vk: r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>::new(kp_A.vk, kp_B.vk),
        }
    }
}

/*********************************** Proof ***********************************/

/**
 * A proof for the R1CS (single-predicate) ppzkPCD.
 */
//
type r1cs_sp_ppzkpcd_proof<PCD_ppT> =
    r1cs_ppzksnark_proof<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>;

// /***************************** Main algorithms *******************************/
// /**
//  * A generator algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a compliance predicate, this algorithm produces proving and verification keys for the predicate.
//  */
//
// r1cs_sp_ppzkpcd_keypair<PCD_ppT> r1cs_sp_ppzkpcd_generator(compliance_predicate:r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>);

// /**
//  * A prover algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a proving key, inputs for the compliance predicate, and proofs for
//  * the predicate's input messages, this algorithm produces a proof (of knowledge)
//  * that attests to the compliance of the output message.
//  */
//
// r1cs_sp_ppzkpcd_proof<PCD_ppT> r1cs_sp_ppzkpcd_prover(pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
//                                                       primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                                                       auxiliary_input:r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT>,
//                                                       incoming_proofs:Vec<r1cs_sp_ppzkpcd_proof<PCD_ppT> >);

// /*
//  Below are two variants of verifier algorithm for the R1CS (single-predicate) ppzkPCD.

//  These are the two cases that arise from whether the verifier accepts a
//  (non-processed) verification key or, instead, a processed verification key.
//  In the latter case, we call the algorithm an "online verifier".
//  */
// /**
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a non-processed verification key.
//  */
//
// bool r1cs_sp_ppzkpcd_verifier(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
//                               primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                               proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>);

// /**
//  * Convert a (non-processed) verification key into a processed verification key.
//  */
//
// r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_sp_ppzkpcd_process_vk(vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>);

// /**
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a processed verification key.
//  */
//
// bool r1cs_sp_ppzkpcd_online_verifier(pvk:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>,
//                                      primary_input:r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
//                                      proof:r1cs_sp_ppzkpcd_proof<PCD_ppT>);

// use common::profiling;
// use common::utils;

// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::sp_pcd_circuits;

//
// bool r1cs_sp_ppzkpcd_proving_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>) const
// {
//     return (self.compliance_predicate == other.compliance_predicate &&
//             self.compliance_step_r1cs_pk == other.compliance_step_r1cs_pk &&
//             self.translation_step_r1cs_pk == other.translation_step_r1cs_pk &&
//             self.compliance_step_r1cs_vk == other.compliance_step_r1cs_vk &&
//             self.translation_step_r1cs_vk == other.translation_step_r1cs_vk);
// }

//
// std::ostream& operator<<(std::ostream &out, pk:r1cs_sp_ppzkpcd_proving_key<PCD_ppT>)
// {
//     out << pk.compliance_predicate;
//     out << pk.compliance_step_r1cs_pk;
//     out << pk.translation_step_r1cs_pk;
//     out << pk.compliance_step_r1cs_vk;
//     out << pk.translation_step_r1cs_vk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_proving_key<PCD_ppT> &pk)
// {
//     in >> pk.compliance_predicate;
//     in >> pk.compliance_step_r1cs_pk;
//     in >> pk.translation_step_r1cs_pk;
//     in >> pk.compliance_step_r1cs_vk;
//     in >> pk.translation_step_r1cs_vk;

//     return in;
// }

//
// bool r1cs_sp_ppzkpcd_verification_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_vk == other.compliance_step_r1cs_vk &&
//             self.translation_step_r1cs_vk == other.translation_step_r1cs_vk);
// }

//
// std::ostream& operator<<(std::ostream &out, vk:r1cs_sp_ppzkpcd_verification_key<PCD_ppT>)
// {
//     out << vk.compliance_step_r1cs_vk;
//     out << vk.translation_step_r1cs_vk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_verification_key<PCD_ppT> &vk)
// {
//     in >> vk.compliance_step_r1cs_vk;
//     in >> vk.translation_step_r1cs_vk;

//     return in;
// }
impl<PCD_ppT: PcdConfigPptConfig> r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn dummy_verification_key() -> r1cs_sp_ppzkpcd_verification_key<PCD_ppT>
// where <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,Output=<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2>,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,Output=<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1> ,
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp: ppTConfig ,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,Output=<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2>,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,Output=<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>,
// <PCD_ppT as PcdConfigPptConfig>::curve_B_pp: ppTConfig,<PCD_ppT as PcdConfigPptConfig>::curve_A_pp: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,<PCD_ppT as PcdConfigPptConfig>::curve_B_pp: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>
    {
        // type curve_A_pp = PCD_ppT::curve_A_pp;
        // type curve_B_pp = PCD_ppT::curve_B_pp;

        let mut result = r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>::default();
        result.compliance_step_r1cs_vk =
            r1cs_ppzksnark_verification_key::<PCD_ppT::curve_A_pp>::dummy_verification_key(
                sp_compliance_step_pcd_circuit_maker::<PCD_ppT::curve_A_pp, PCD_ppT::curve_A_pp>::input_size_in_elts(
                ),
            );
        result.translation_step_r1cs_vk =
            r1cs_ppzksnark_verification_key::<PCD_ppT::curve_B_pp>::dummy_verification_key(
                sp_translation_step_pcd_circuit_maker::<PCD_ppT::curve_B_pp,PCD_ppT::curve_B_pp>::input_size_in_elts(),
            );

        result
    }
}

//
// bool r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>::operator==(other:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>) const
// {
//     return (self.compliance_step_r1cs_pvk == other.compliance_step_r1cs_pvk &&
//             self.translation_step_r1cs_pvk == other.translation_step_r1cs_pvk &&
//             self.translation_step_r1cs_vk_bits == other.translation_step_r1cs_vk_bits);
// }

//
// std::ostream& operator<<(std::ostream &out, pvk:r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>)
// {
//     out << pvk.compliance_step_r1cs_pvk;
//     out << pvk.translation_step_r1cs_pvk;
//     serialize_bit_vector(out, pvk.translation_step_r1cs_vk_bits);

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> &pvk)
// {
//     in >> pvk.compliance_step_r1cs_pvk;
//     in >> pvk.translation_step_r1cs_pvk;
//     deserialize_bit_vector(in, pvk.translation_step_r1cs_vk_bits);

//     return in;
// }

pub fn r1cs_sp_ppzkpcd_generator<PCD_ppT: PcdConfigPptConfig>(
    compliance_predicate: &r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,
) -> r1cs_sp_ppzkpcd_keypair<PCD_ppT>
// where
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp: ppTConfig,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2>,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//         >,
//     <PCD_ppT as PcdConfigPptConfig>::curve_B_pp: ppTConfig,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2>,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//         >,
//     PCD_ppT::ED: evaluation_domain<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr,
//     >,
//     P: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>, <<P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type as ff_curves::PublicParams>::Fr: Mul<<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr,Output=<<P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type as ff_curves::PublicParams>::Fr>,
//     PCD_ppT::ED:evaluation_domain<<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::FieldT>,
//  PCD_ppT::ED:evaluation_domain<ppT::FieldT>,
// PCD_ppT::ED:evaluation_domain<<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ppTConfig>::FieldT>
{
    // assert!(Fr::< PCD_ppT::curve_A_pp>::modulo == Fq::< PCD_ppT::curve_B_pp>::modulo);
    // assert!(Fq::< PCD_ppT::curve_A_pp>::modulo == Fr::< PCD_ppT::curve_B_pp>::modulo);

    // type FieldT_A=Fr::< PCD_ppT::curve_A_pp>;
    // type FieldT_B=Fr::< PCD_ppT::curve_B_pp>;

    // type curve_A_pp= PCD_ppT::curve_A_pp;
    // type curve_B_pp= PCD_ppT::curve_B_pp;

    enter_block("Call to r1cs_sp_ppzkpcd_generator", false);

    assert!(compliance_predicate.is_well_formed());

    enter_block("Construct compliance step PCD circuit", false);
    let mut compliance_step_pcd_circuit =
        sp_compliance_step_pcd_circuit_maker::<PCD_ppT::curve_A_pp>::new(
            compliance_predicate.clone(),
        );
    compliance_step_pcd_circuit.generate_r1cs_constraints();
    let compliance_step_pcd_circuit_cs = compliance_step_pcd_circuit.get_circuit();
    compliance_step_pcd_circuit_cs.report_linear_constraint_statistics();
    leave_block("Construct compliance step PCD circuit", false);

    enter_block("Generate key pair for compliance step PCD circuit", false);
    let mut compliance_step_keypair =
        r1cs_ppzksnark_generator::<PCD_ppT>(&compliance_step_pcd_circuit_cs);
    leave_block("Generate key pair for compliance step PCD circuit", false);

    enter_block("Construct translation step PCD circuit", false);
    let mut translation_step_pcd_circuit = sp_translation_step_pcd_circuit_maker::<
        PCD_ppT::curve_B_pp,
    >::new(compliance_step_keypair.vk.clone());
    translation_step_pcd_circuit.generate_r1cs_constraints();
    let translation_step_pcd_circuit_cs = translation_step_pcd_circuit.get_circuit();
    translation_step_pcd_circuit_cs.report_linear_constraint_statistics();
    leave_block("Construct translation step PCD circuit", false);

    enter_block("Generate key pair for translation step PCD circuit", false);
    let translation_step_keypair =
        r1cs_ppzksnark_generator::<PCD_ppT>(&translation_step_pcd_circuit_cs);
    leave_block("Generate key pair for translation step PCD circuit", false);

    print_indent();
    println!("in generator");
    leave_block("Call to r1cs_sp_ppzkpcd_generator", false);

    r1cs_sp_ppzkpcd_keypair::<PCD_ppT>::new(
        r1cs_sp_ppzkpcd_proving_key::<PCD_ppT>::new(
            compliance_predicate.clone(),
            compliance_step_keypair.pk.clone(),
            translation_step_keypair.pk.clone(),
            compliance_step_keypair.vk.clone(),
            translation_step_keypair.vk.clone(),
        ),
        r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>::new(
            compliance_step_keypair.vk.clone(),
            translation_step_keypair.vk.clone(),
        ),
    )
}

type FieldT_A<PCD_ppT> = Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>;
type FieldT_B<PCD_ppT> = Fr<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>;
type curve_A_pp<PCD_ppT> = <PCD_ppT as PcdConfigPptConfig>::curve_A_pp;
type curve_B_pp<PCD_ppT> = <PCD_ppT as PcdConfigPptConfig>::curve_B_pp;
// pub trait sp_ppzkpcdConfig {
//     type PCD_ppT: PcdConfigPptConfig<AP = Self::P>;
//     type PB: PBConfig;
//     type M: R1csPcdMessageConfig;
//     type LD: R1csPcdLocalDataConfig;
//     type ED: evaluation_domain<Self::PCD_ppT::curve_B_pp::FieldT>;
//     type P: pairing_selector;
//     const N: usize;
// }
pub fn r1cs_sp_ppzkpcd_prover<PCD_ppT: PcdConfigPptConfig>(
    pk: &r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    auxiliary_input: &r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT>,
    incoming_proofs: &Vec<r1cs_sp_ppzkpcd_proof<PCD_ppT>>,
) -> r1cs_sp_ppzkpcd_proof<PCD_ppT>
// where
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp: ppTConfig,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<
//             knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             >,
//             Output = knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             >,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr: Mul<
//             knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             >,
//             Output = knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
//             >,
//         >,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1>,
//     <PCD_ppT as PcdConfigPptConfig>::curve_B_pp: ppTConfig,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<
//             knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             >,
//             Output = knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             >,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<
//             <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//         >,
//     <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr: Mul<
//             knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             >,
//             Output = knowledge_commitment<
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
//                 <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
//             >,
//         >,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>,
//     <P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type:
//         ff_curves::PublicParams,
//     PCD_ppT::M: R1csPcdMessageConfig<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr,
//     >,
//     PCD_ppT::ED: evaluation_domain<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr,
//     >,
//     P: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
//     <PCD_ppT as PcdConfigPptConfig>::curve_B_pp:
//         pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>,
//     PCD_ppT::M:R1csPcdMessageConfig<<<PCD_ppT as PcdConfigPptConfig>::AP as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>::FieldT>,
//     LD:R1csPcdLocalDataConfig<<<PCD_ppT as PcdConfigPptConfig>::AP as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>::FieldT>,
//     PCD_ppT::M:R1csPcdMessageConfig<<<P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type as ff_curves::PublicParams>::Fr>,
//   PCD_ppT::ED:evaluation_domain<ppT::FieldT>
{
    // type FieldT_A=Fr< PCD_ppT::curve_A_pp>;
    // type FieldT_B=Fr< PCD_ppT::curve_B_pp>;

    // type curve_A_pp= PCD_ppT::curve_A_pp;
    // type curve_B_pp= PCD_ppT::curve_B_pp;

    enter_block("Call to r1cs_sp_ppzkpcd_prover", false);

    let translation_step_r1cs_vk_bits =
        r1cs_ppzksnark_verification_key_variable::<PCD_ppT::curve_A_pp>::get_verification_key_bits(
            &pk.translation_step_r1cs_vk,
        );
    // #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message.borrow().print();
    //#endif

    enter_block("Prove compliance step", false);
    let mut compliance_step_pcd_circuit =
        sp_compliance_step_pcd_circuit_maker::<PCD_ppT::curve_A_pp>::new(pk.compliance_predicate);
    compliance_step_pcd_circuit.generate_r1cs_witness(
        &pk.translation_step_r1cs_vk,
        &primary_input,
        &auxiliary_input,
        incoming_proofs,
    );

    let compliance_step_primary_input = compliance_step_pcd_circuit.get_primary_input();
    let compliance_step_auxiliary_input = compliance_step_pcd_circuit.get_auxiliary_input();

    let compliance_step_proof = r1cs_ppzksnark_prover::<PCD_ppT::curve_A_pp>(
        &pk.compliance_step_r1cs_pk,
        &compliance_step_primary_input,
        &compliance_step_auxiliary_input,
    );
    leave_block("Prove compliance step", false);

    // #ifdef DEBUG
    let compliance_step_input = get_sp_compliance_step_pcd_circuit_input::<PCD_ppT::curve_A_pp>(
        &translation_step_r1cs_vk_bits,
        &primary_input,
    );
    let compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC::<PCD_ppT::curve_A_pp>(
        &pk.compliance_step_r1cs_vk,
        &compliance_step_input,
        &compliance_step_proof,
    );
    assert!(compliance_step_ok);
    //#endif

    enter_block("Prove translation step", false);
    let translation_step_pcd_circuit =
        sp_translation_step_pcd_circuit_maker::<PCD_ppT::curve_B_pp>::new(
            pk.compliance_step_r1cs_vk,
        );

    let translation_step_primary_input = get_sp_translation_step_pcd_circuit_input::<
        PCD_ppT::curve_B_pp,
    >(&translation_step_r1cs_vk_bits, primary_input);
    translation_step_pcd_circuit
        .generate_r1cs_witness(&translation_step_primary_input, &compliance_step_proof); // TODO: potential for better naming

    let translation_step_auxiliary_input = translation_step_pcd_circuit.get_auxiliary_input();
    let translation_step_proof = r1cs_ppzksnark_prover::<PCD_ppT::curve_B_pp>(
        &pk.translation_step_r1cs_pk,
        &translation_step_primary_input,
        &translation_step_auxiliary_input,
    );
    leave_block("Prove translation step", false);

    // #ifdef DEBUG
    let translation_step_ok = r1cs_ppzksnark_verifier_strong_IC::<PCD_ppT::curve_B_pp>(
        &pk.translation_step_r1cs_vk,
        &translation_step_primary_input,
        &translation_step_proof,
    );
    assert!(translation_step_ok);
    //#endif

    print_indent();
    println!("in prover");
    leave_block("Call to r1cs_sp_ppzkpcd_prover", false);

    translation_step_proof
}

pub fn r1cs_sp_ppzkpcd_online_verifier<PCD_ppT: PcdConfigPptConfig>(
    pvk: &r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_sp_ppzkpcd_proof<PCD_ppT>,
) -> bool
// where
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>,
//     <P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type:
//         ff_curves::PublicParams,
//     <PCD_ppT as PcdConfigPptConfig>::curve_B_pp:
//         pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>,
//     PCD_ppT::M: R1csPcdMessageConfig<
//         <<PCD_ppT as PcdConfigPptConfig>::AP as pairing_selector<
//             <PCD_ppT as PcdConfigPptConfig>::curve_A_pp,
//         >>::FieldT,
//     >,
//     PCD_ppT::M: R1csPcdMessageConfig<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::Fr,
//     >,
{
    // type curve_B_pp= PCD_ppT::curve_B_pp;

    enter_block("Call to r1cs_sp_ppzkpcd_online_verifier", false);
    let r1cs_input = get_sp_translation_step_pcd_circuit_input::<PCD_ppT>(
        &pvk.translation_step_r1cs_vk_bits,
        primary_input,
    );
    let result = r1cs_ppzksnark_online_verifier_strong_IC(
        &pvk.translation_step_r1cs_pvk,
        &r1cs_input,
        &proof,
    );
    print_indent();
    println!("in online verifier");
    leave_block("Call to r1cs_sp_ppzkpcd_online_verifier", false);

    result
}

pub fn r1cs_sp_ppzkpcd_process_vk<PCD_ppT: PcdConfigPptConfig>(
    vk: &r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
) -> r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>
// where
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp: ppTConfig,
//     PCD_ppT::ED: evaluation_domain<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr,
//     >,
//     P: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp:
//         pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
//     P: pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>,
{
    // type curve_A_pp= PCD_ppT::curve_A_pp;
    // type curve_B_pp= PCD_ppT::curve_B_pp;

    enter_block("Call to r1cs_sp_ppzkpcd_processed_verification_key", false);
    let compliance_step_r1cs_pvk =
        r1cs_ppzksnark_verifier_process_vk::<PCD_ppT::curve_A_pp>(&vk.compliance_step_r1cs_vk);
    let translation_step_r1cs_pvk =
        r1cs_ppzksnark_verifier_process_vk::<PCD_ppT::curve_B_pp>(&vk.translation_step_r1cs_vk);
    let translation_step_r1cs_vk_bits =
        r1cs_ppzksnark_verification_key_variable::<PCD_ppT::curve_A_pp>::get_verification_key_bits(
            &vk.translation_step_r1cs_vk,
        );
    leave_block("Call to r1cs_sp_ppzkpcd_processed_verification_key", false);

    r1cs_sp_ppzkpcd_processed_verification_key::<PCD_ppT>::new(
        (compliance_step_r1cs_pvk),
        (translation_step_r1cs_pvk),
        translation_step_r1cs_vk_bits,
    )
}

pub fn r1cs_sp_ppzkpcd_verifier<PCD_ppT: PcdConfigPptConfig>(
    vk: &r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_sp_ppzkpcd_proof<PCD_ppT>,
) -> bool
// where
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp: ppTConfig,
//     for<'a> &'a <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1:
//         Add<Output = <<PCD_ppT as PcdConfigPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1>,
//     <P as pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>>::other_curve_type:
//         ff_curves::PublicParams,
//     PCD_ppT::ED: evaluation_domain<
//         <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ff_curves::PublicParams>::Fr,
//     >,
//     <PCD_ppT as PcdConfigPptConfig>::curve_B_pp:
//         pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_B_pp>,
//     <PCD_ppT as PcdConfigPptConfig>::curve_A_pp:
//         pairing_selector<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
//     PCD_ppT::M: R1csPcdMessageConfig<
//         <<PCD_ppT as PcdConfigPptConfig>::AP as pairing_selector<
//             <PCD_ppT as PcdConfigPptConfig>::curve_A_pp,
//         >>::FieldT,
//     >,
{
    enter_block("Call to r1cs_sp_ppzkpcd_verifier", false);
    let pvk = r1cs_sp_ppzkpcd_process_vk::<PCD_ppT>(&vk);
    let result = r1cs_sp_ppzkpcd_online_verifier::<PCD_ppT>(&pvk, primary_input, proof);
    print_indent();
    println!("in verifier");
    leave_block("Call to r1cs_sp_ppzkpcd_verifier", false);

    result
}
