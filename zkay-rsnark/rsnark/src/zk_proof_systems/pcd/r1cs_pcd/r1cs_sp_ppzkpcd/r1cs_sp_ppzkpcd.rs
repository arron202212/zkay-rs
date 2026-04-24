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

use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    other_curve, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget::r1cs_ppzksnark_verification_key_variable;
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_params::{
    r1cs_sp_ppzkpcd_auxiliary_input, r1cs_sp_ppzkpcd_compliance_predicate,
    r1cs_sp_ppzkpcd_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::sp_pcd_circuits::{
    get_sp_compliance_step_pcd_circuit_input, get_sp_translation_step_pcd_circuit_input,
    sp_compliance_step_pcd_circuit_maker, sp_translation_step_pcd_circuit_maker,
};

use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_generator, r1cs_ppzksnark_keypair, r1cs_ppzksnark_online_verifier_strong_IC,
    r1cs_ppzksnark_processed_verification_key, r1cs_ppzksnark_proof, r1cs_ppzksnark_prover,
    r1cs_ppzksnark_proving_key, r1cs_ppzksnark_verification_key,
    r1cs_ppzksnark_verifier_process_vk, r1cs_ppzksnark_verifier_strong_IC,
};
use ff_curves::Fr;
use ffec::common::profiling::print_indent;
use ffec::{FieldTConfig, PpConfig, bit_vector};
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use std::ops::{Add, Mul};
use tracing::{Level, span};

// /**
//  * A proving key for the R1CS (single-predicate) ppzkPCD.
//  */
//
type A_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_A_pp;
type B_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_B_pp;

#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_proving_key<PCD_ppT: PcdPptConfig> {
    pub compliance_predicate: r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,
    pub compliance_step_r1cs_pk: r1cs_ppzksnark_proving_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_pk: r1cs_ppzksnark_proving_key<B_pp<PCD_ppT>>,
    pub compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_sp_ppzkpcd_proving_key<PCD_ppT> {
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

// /**
//  * A verification key for the R1CS (single-predicate) ppzkPCD.
//  */
//    type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_verification_key<PCD_ppT: PcdPptConfig> {
    pub compliance_step_r1cs_vk: r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk: r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
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

// /**
//  * A processed verification key for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Compared to a (non-processed) verification key, a processed verification key
//  * contains a small constant amount of additional pre-computed information that
//  * enables a faster verification time.
//  */
//  type A_pp= PCD_ppT::curve_A_pp;
//     type B_pp= PCD_ppT::curve_B_pp;
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT: PcdPptConfig> {
    pub compliance_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<A_pp<PCD_ppT>>,
    pub translation_step_r1cs_pvk: r1cs_ppzksnark_processed_verification_key<B_pp<PCD_ppT>>,
    pub translation_step_r1cs_vk_bits: bit_vector,
}
impl<PCD_ppT: PcdPptConfig> r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> {
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

// /**
//  * A key pair for the R1CS (single-predicate) ppzkPC, which consists of a proving key and a verification key.
//  */
#[derive(Default, Clone)]
pub struct r1cs_sp_ppzkpcd_keypair<PCD_ppT: PcdPptConfig> {
    pub pk: r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
    pub vk: r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_sp_ppzkpcd_keypair<PCD_ppT> {
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

// /**
//  * A proof for the R1CS (single-predicate) ppzkPCD.
//  */
//
pub type r1cs_sp_ppzkpcd_proof<PCD_ppT> =
    r1cs_ppzksnark_proof<<PCD_ppT as PcdPptConfig>::curve_B_pp>;

impl<PCD_ppT: PcdPptConfig> r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn dummy_verification_key() -> r1cs_sp_ppzkpcd_verification_key<PCD_ppT> {
        let mut result = r1cs_sp_ppzkpcd_verification_key::<PCD_ppT>::default();
        result.compliance_step_r1cs_vk =
            r1cs_ppzksnark_verification_key::<PCD_ppT::curve_A_pp>::dummy_verification_key(
                sp_compliance_step_pcd_circuit_maker::<PCD_ppT>::input_size_in_elts(),
            );
        result.translation_step_r1cs_vk =
            r1cs_ppzksnark_verification_key::<PCD_ppT::curve_B_pp>::dummy_verification_key(
                sp_translation_step_pcd_circuit_maker::<PCD_ppT>::input_size_in_elts(),
            );

        result
    }
}

// /**
//  * A generator algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a compliance predicate, this algorithm produces proving and verification keys for the predicate.
//  */
pub fn r1cs_sp_ppzkpcd_generator<PCD_ppT: PcdPptConfig>(
    compliance_predicate: &r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT>,
) -> r1cs_sp_ppzkpcd_keypair<PCD_ppT> {
    // assert!(Fr::< PCD_ppT::curve_A_pp>::modulo == Fq::< PCD_ppT::curve_B_pp>::modulo);
    // assert!(Fq::< PCD_ppT::curve_A_pp>::modulo == Fr::< PCD_ppT::curve_B_pp>::modulo);

    let span0 = span!(Level::TRACE, "Call to r1cs_sp_ppzkpcd_generator");
    let _=span0.enter();
    assert!(compliance_predicate.is_well_formed());

    let spancc = span!(Level::TRACE, "Construct compliance step PCD circuit").entered();
    let mut compliance_step_pcd_circuit =
        sp_compliance_step_pcd_circuit_maker::<A_pp<PCD_ppT>>::new(compliance_predicate.clone());
    compliance_step_pcd_circuit.generate_r1cs_constraints();
    let compliance_step_pcd_circuit_cs = compliance_step_pcd_circuit.get_circuit();
    compliance_step_pcd_circuit_cs.report_linear_constraint_statistics();
    spancc.exit();

    let spanc = span!(
        Level::TRACE,
        "Generate key pair for compliance step PCD circuit"
    )
    .entered();
    let mut compliance_step_keypair =
        r1cs_ppzksnark_generator::<A_pp<PCD_ppT>>(&compliance_step_pcd_circuit_cs);
    spanc.exit();

    let span = span!(Level::TRACE, "Construct translation step PCD circuit").entered();
    let mut translation_step_pcd_circuit =
        sp_translation_step_pcd_circuit_maker::<B_pp<PCD_ppT>>::new(
            compliance_step_keypair.vk.clone(),
        );
    translation_step_pcd_circuit.generate_r1cs_constraints();
    let translation_step_pcd_circuit_cs = translation_step_pcd_circuit.get_circuit();
    translation_step_pcd_circuit_cs.report_linear_constraint_statistics();
    span.exit();

    let spang = span!(
        Level::TRACE,
        "Generate key pair for translation step PCD circuit"
    )
    .entered();
    let translation_step_keypair =
        r1cs_ppzksnark_generator::<B_pp<PCD_ppT>>(&translation_step_pcd_circuit_cs);
    spang.exit();

    print_indent();
    println!("in generator");
   

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

type FieldT_A<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>;
type FieldT_B<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_B_pp>;
type curve_A_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_A_pp;
type curve_B_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_B_pp;
// /**
//  * A prover algorithm for the R1CS (single-predicate) ppzkPCD.
//  *
//  * Given a proving key, inputs for the compliance predicate, and proofs for
//  * the predicate's input messages, this algorithm produces a proof (of knowledge)
//  * that attests to the compliance of the output message.
//  */
pub fn r1cs_sp_ppzkpcd_prover<PCD_ppT: PcdPptConfig>(
    pk: &r1cs_sp_ppzkpcd_proving_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    auxiliary_input: &r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT>,
    incoming_proofs: &Vec<r1cs_sp_ppzkpcd_proof<PCD_ppT>>,
) -> r1cs_sp_ppzkpcd_proof<PCD_ppT>
where
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    let span0 = span!(Level::TRACE, "Call to r1cs_sp_ppzkpcd_prover");
    let _=span0.enter();
    let translation_step_r1cs_vk_bits =
        r1cs_ppzksnark_verification_key_variable::<PCD_ppT::curve_A_pp>::get_verification_key_bits(
            &pk.translation_step_r1cs_vk,
        );
    // #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message.borrow().print();

    let spanp = span!(Level::TRACE, "Prove compliance step").entered();
    let mut compliance_step_pcd_circuit =
        sp_compliance_step_pcd_circuit_maker::<A_pp<PCD_ppT>>::new(pk.compliance_predicate.clone());
    compliance_step_pcd_circuit.generate_r1cs_witness(
        &pk.translation_step_r1cs_vk,
        primary_input,
        auxiliary_input,
        incoming_proofs,
    );

    let compliance_step_primary_input = compliance_step_pcd_circuit.get_primary_input();
    let compliance_step_auxiliary_input = compliance_step_pcd_circuit.get_auxiliary_input();

    let compliance_step_proof = r1cs_ppzksnark_prover::<PCD_ppT::curve_A_pp>(
        &pk.compliance_step_r1cs_pk,
        &compliance_step_primary_input,
        &compliance_step_auxiliary_input,
    );
    spanp.exit();

   
    let compliance_step_input = get_sp_compliance_step_pcd_circuit_input::<PCD_ppT::curve_A_pp>(
        &translation_step_r1cs_vk_bits,
        primary_input,
    );
    let compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC::<PCD_ppT::curve_A_pp>(
        &pk.compliance_step_r1cs_vk,
        &compliance_step_input,
        &compliance_step_proof,
    );
    assert!(compliance_step_ok);

    let spans = span!(Level::TRACE, "Prove translation step").entered();
    let translation_step_pcd_circuit =
        sp_translation_step_pcd_circuit_maker::<PCD_ppT::curve_B_pp>::new(
            pk.compliance_step_r1cs_vk.clone(),
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
    spans.exit();

    
    let translation_step_ok = r1cs_ppzksnark_verifier_strong_IC::<PCD_ppT::curve_B_pp>(
        &pk.translation_step_r1cs_vk,
        &translation_step_primary_input,
        &translation_step_proof,
    );
    assert!(translation_step_ok);

    print_indent();
    println!("in prover");
  

    translation_step_proof
}
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a processed verification key.
pub fn r1cs_sp_ppzkpcd_online_verifier<PCD_ppT: PcdPptConfig>(
    pvk: &r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_sp_ppzkpcd_proof<PCD_ppT>,
) -> bool {
    let span = span!(Level::TRACE, "Call to r1cs_sp_ppzkpcd_online_verifier").entered();
    let r1cs_input = get_sp_translation_step_pcd_circuit_input::<B_pp<PCD_ppT>>(
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
    span.exit();

    result
}
//  * Convert a (non-processed) verification key into a processed verification key.
pub fn r1cs_sp_ppzkpcd_process_vk<PCD_ppT: PcdPptConfig>(
    vk: &r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
) -> r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> {
    let span = span!(
        Level::TRACE,
        "Call to r1cs_sp_ppzkpcd_processed_verification_key"
    )
    .entered();
    let compliance_step_r1cs_pvk =
        r1cs_ppzksnark_verifier_process_vk::<PCD_ppT::curve_A_pp>(&vk.compliance_step_r1cs_vk);
    let translation_step_r1cs_pvk =
        r1cs_ppzksnark_verifier_process_vk::<PCD_ppT::curve_B_pp>(&vk.translation_step_r1cs_vk);
    let translation_step_r1cs_vk_bits =
        r1cs_ppzksnark_verification_key_variable::<PCD_ppT::curve_A_pp>::get_verification_key_bits(
            &vk.translation_step_r1cs_vk,
        );
    span.exit();

    r1cs_sp_ppzkpcd_processed_verification_key::<PCD_ppT>::new(
        (compliance_step_r1cs_pvk),
        (translation_step_r1cs_pvk),
        translation_step_r1cs_vk_bits,
    )
}
//  Below are two variants of verifier algorithm for the R1CS (single-predicate) ppzkPCD.

//  These are the two cases that arise from whether the verifier accepts a
//  (non-processed) verification key or, instead, a processed verification key.
//  In the latter case, we call the algorithm an "online verifier".
//  */
// /**
//  * A verifier algorithm for the R1CS (single-predicate) ppzkPCD that
//  * accepts a non-processed verification key.
//  */
pub fn r1cs_sp_ppzkpcd_verifier<PCD_ppT: PcdPptConfig>(
    vk: &r1cs_sp_ppzkpcd_verification_key<PCD_ppT>,
    primary_input: &r1cs_sp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_sp_ppzkpcd_proof<PCD_ppT>,
) -> bool {
    let span = span!(Level::TRACE, "Call to r1cs_sp_ppzkpcd_verifier").entered();
    let pvk = r1cs_sp_ppzkpcd_process_vk::<PCD_ppT>(&vk);
    let result = r1cs_sp_ppzkpcd_online_verifier::<PCD_ppT>(&pvk, primary_input, proof);
    print_indent();
    println!("in verifier");
    span.exit();

    result
}
