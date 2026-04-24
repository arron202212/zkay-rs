// Declaration of interfaces for a *multi-predicate* ppzkPCD for R1CS.

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
// in \[CTV15]. Thus, PCD is constructed from two "matched" ppzkSNARKs for R1CS.

// Acronyms:

// "R1CS" = "Rank-1 Constraint Systems"
// "ppzkSNARK" = "PreProcessing Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
// "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

// References:

// \[CTV15]:
// "Cluster Computing in Zero Knowledge",
// Alessandro Chiesa, Eran Tromer, Madars Virza,

use crate::common::data_structures::set_commitment::set_commitment_accumulator;
use crate::common::data_structures::set_commitment::{set_commitment, set_membership_proof};
use crate::gadgetlib1::gadgets::hashes::crh_gadget::CRH_with_bit_out_gadget;
use crate::gadgetlib1::gadgets::hashes::crh_gadget::CRH_with_bit_out_gadgets;
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    other_curve, pairing_selector, ppTConfig,
};
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget::r1cs_ppzksnark_verification_key_variable;
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::prefix_format;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::mp_pcd_circuits::{
    get_mp_compliance_step_pcd_circuit_input, get_mp_translation_step_pcd_circuit_input,
    mp_compliance_step_pcd_circuit_maker, mp_translation_step_pcd_circuit_maker,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd_params::r1cs_mp_ppzkpcd_compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd_params::{
    r1cs_mp_ppzkpcd_auxiliary_input, r1cs_mp_ppzkpcd_primary_input,
};
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
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, Mul};
use tracing::{Level, span};

// /**
//  * A proving key for the R1CS (multi-predicate) ppzkPCD.
//  */
type A_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_A_pp;
type B_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_B_pp;

type FieldT_A<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>;
type FieldT_B<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_B_pp>;
type curve_A_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_A_pp;
type curve_B_pp<PCD_ppT> = <PCD_ppT as PcdPptConfig>::curve_B_pp;

#[derive(Default, Clone)]
pub struct r1cs_mp_ppzkpcd_proving_key<PCD_ppT: PcdPptConfig> {
    pub compliance_predicates: Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT>>,
    pub compliance_step_r1cs_pks: Vec<r1cs_ppzksnark_proving_key<A_pp<PCD_ppT>>>,
    pub translation_step_r1cs_pks: Vec<r1cs_ppzksnark_proving_key<B_pp<PCD_ppT>>>,
    pub compliance_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>>,
    pub translation_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>>,
    pub commitment_to_translation_step_r1cs_vks: set_commitment,
    pub compliance_step_r1cs_vk_membership_proofs: Vec<set_membership_proof>,
    pub compliance_predicate_name_to_idx: BTreeMap<usize, usize>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_proving_key<PCD_ppT> {
    pub fn new(
        compliance_predicates: Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT>>,
        compliance_step_r1cs_pk: Vec<r1cs_ppzksnark_proving_key<A_pp<PCD_ppT>>>,
        translation_step_r1cs_pk: Vec<r1cs_ppzksnark_proving_key<B_pp<PCD_ppT>>>,
        compliance_step_r1cs_vk: Vec<r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>>,
        translation_step_r1cs_vk: Vec<r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>>,
        commitment_to_translation_step_r1cs_vks: set_commitment,
        compliance_step_r1cs_vk_membership_proofs: Vec<set_membership_proof>,
        compliance_predicate_name_to_idx: BTreeMap<usize, usize>,
    ) -> Self {
        Self {
            compliance_predicates,
            compliance_step_r1cs_pks: vec![],
            translation_step_r1cs_pks: vec![],
            compliance_step_r1cs_vks: vec![],
            translation_step_r1cs_vks: vec![],
            commitment_to_translation_step_r1cs_vks,
            compliance_step_r1cs_vk_membership_proofs,
            compliance_predicate_name_to_idx,
        }
    }
}

// /**
//  * A verification key for the R1CS (multi-predicate) ppzkPCD.
//  */
//
//    type A_pp= PCD_ppT::curve_A_pp ;
//     type B_pp= PCD_ppT::curve_B_pp ;

#[derive(Default, Clone)]
pub struct r1cs_mp_ppzkpcd_verification_key<PCD_ppT: PcdPptConfig> {
    pub compliance_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>>,
    pub translation_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>>,
    pub commitment_to_translation_step_r1cs_vks: set_commitment,
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn new(
        compliance_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<A_pp<PCD_ppT>>>,
        translation_step_r1cs_vks: Vec<r1cs_ppzksnark_verification_key<B_pp<PCD_ppT>>>,
        commitment_to_translation_step_r1cs_vks: set_commitment,
    ) -> Self {
        Self {
            compliance_step_r1cs_vks,
            translation_step_r1cs_vks,
            commitment_to_translation_step_r1cs_vks,
        }
    }
}

// /**
//  * A processed verification key for the R1CS (multi-predicate) ppzkPCD.
//  *
//  * Compared to a (non-processed) verification key, a processed verification key
//  * contains a small constant amount of additional pre-computed information that
//  * enables a faster verification time.
//  */
//
// type A_pp= PCD_ppT::curve_A_pp ;
//     type B_pp= PCD_ppT::curve_B_pp ;

#[derive(Default, Clone)]
pub struct r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT: PcdPptConfig> {
    pub compliance_step_r1cs_pvks: Vec<r1cs_ppzksnark_processed_verification_key<A_pp<PCD_ppT>>>,
    pub translation_step_r1cs_pvks: Vec<r1cs_ppzksnark_processed_verification_key<B_pp<PCD_ppT>>>,
    pub commitment_to_translation_step_r1cs_vks: set_commitment,
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> {
    pub fn new(
        compliance_step_r1cs_pvks: Vec<r1cs_ppzksnark_processed_verification_key<A_pp<PCD_ppT>>>,
        translation_step_r1cs_pvks: Vec<r1cs_ppzksnark_processed_verification_key<B_pp<PCD_ppT>>>,
        commitment_to_translation_step_r1cs_vks: set_commitment,
    ) -> Self {
        Self {
            compliance_step_r1cs_pvks,
            translation_step_r1cs_pvks,
            commitment_to_translation_step_r1cs_vks,
        }
    }
}

// /**
//  * A key pair for the R1CS (multi-predicate) ppzkPC, which consists of a proving key and a verification key.
//  */
#[derive(Default, Clone)]
pub struct r1cs_mp_ppzkpcd_keypair<PCD_ppT: PcdPptConfig> {
    pub pk: r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
    pub vk: r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_keypair<PCD_ppT> {
    pub fn new(
        pk: r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
        vk: r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
    ) -> Self {
        Self { pk, vk }
    }
}

// /**
//  * A proof for the R1CS (multi-predicate) ppzkPCD.
//  */
#[derive(Default, Clone)]
pub struct r1cs_mp_ppzkpcd_proof<PCD_ppT: PcdPptConfig> {
    pub compliance_predicate_idx: usize,
    pub r1cs_proof: r1cs_ppzksnark_proof<PCD_ppT::curve_B_pp>,
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_proof<PCD_ppT> {
    pub fn new(
        compliance_predicate_idx: usize,
        r1cs_proof: r1cs_ppzksnark_proof<PCD_ppT::curve_B_pp>,
    ) -> Self {
        Self {
            compliance_predicate_idx,
            r1cs_proof,
        }
    }
}

// /**
//  * A generator algorithm for the R1CS (multi-predicate) ppzkPCD.
//  *
//  * Given a vector of compliance predicates, this algorithm produces proving and verification keys for the vector.
//  */
//
// r1cs_mp_ppzkpcd_keypair<PCD_ppT> r1cs_mp_ppzkpcd_generator(compliance_predicates:Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> >);

// /**
//  * A prover algorithm for the R1CS (multi-predicate) ppzkPCD.
//  *
//  * Given a proving key, name of chosen compliance predicate, inputs for the
//  * compliance predicate, and proofs for the predicate's input messages, this
//  * algorithm produces a proof (of knowledge) that attests to the compliance of
//  * the output message.
//  */
//
// r1cs_mp_ppzkpcd_proof<PCD_ppT> r1cs_mp_ppzkpcd_prover(pk:r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
//                                                       compliance_predicate_name:usize ,
//                                                       primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                                                       auxiliary_input:r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT>,
//                                                       incoming_proofs:Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >);

// /*
//   Below are two variants of verifier algorithm for the R1CS (multi-predicate) ppzkPCD.

//   These are the two cases that arise from whether the verifier accepts a
//   (non-processed) verification key or, instead, a processed verification key.
//   In the latter case, we call the algorithm an "online verifier".
// */
// /**
//  * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
//  * accepts a non-processed verification key.
//  */
//
// bool r1cs_mp_ppzkpcd_verifier(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
//                               primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                               proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>);

// /**
//  * Convert a (non-processed) verification key into a processed verification key.
//  */
//
// r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> r1cs_mp_ppzkpcd_process_vk(vk:r1cs_mp_ppzkpcd_verification_key<PCD_ppT>);

// /**
//  * A verifier algorithm for the R1CS (multi-predicate) ppzkPCD that
//  * accepts a processed verification key.
//  */
//
// bool r1cs_mp_ppzkpcd_online_verifier(pvk:r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>,
//                                      primary_input:r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
//                                      proof:r1cs_mp_ppzkpcd_proof<PCD_ppT>);

// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd;

// use common::profiling;
// use common::utils;

// use crate::common::libsnark_serialization;
// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::mp_pcd_circuits;

impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_proving_key<PCD_ppT> {
    pub fn size_in_bits(&self) -> usize {
        let num_predicates = self.compliance_predicates.len();

        let mut result = 0;
        for i in 0..num_predicates {
            result += (self.compliance_predicates[i].size_in_bits()
                + self.compliance_step_r1cs_pks[i].size_in_bits()
                + self.translation_step_r1cs_pks[i].size_in_bits()
                + self.compliance_step_r1cs_vks[i].size_in_bits()
                + self.translation_step_r1cs_vks[i].size_in_bits()
                + self.compliance_step_r1cs_vk_membership_proofs[i].size_in_bits());
        }
        result += self.commitment_to_translation_step_r1cs_vks.len();

        result
    }

    pub fn is_well_formed(&self) -> bool {
        let num_predicates = self.compliance_predicates.len();

        let mut result = false;
        result = result && (self.compliance_step_r1cs_pks.len() == num_predicates);
        result = result && (self.translation_step_r1cs_pks.len() == num_predicates);
        result = result && (self.compliance_step_r1cs_vks.len() == num_predicates);
        result = result && (self.translation_step_r1cs_vks.len() == num_predicates);
        result = result && (self.compliance_step_r1cs_vk_membership_proofs.len() == num_predicates);

        result
    }
}
impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_verification_key<PCD_ppT> {
    pub fn size_in_bits(&self) -> usize {
        let num_predicates = self.compliance_step_r1cs_vks.len();

        let mut result = 0;
        for i in 0..num_predicates {
            result += (self.compliance_step_r1cs_vks[i].size_in_bits()
                + self.translation_step_r1cs_vks[i].size_in_bits());
        }

        result += self.commitment_to_translation_step_r1cs_vks.len();

        result
    }
}

impl<PCD_ppT: PcdPptConfig> r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> {
    pub fn size_in_bits(&self) -> usize {
        let num_predicates = self.compliance_step_r1cs_pvks.len();

        let mut result = 0;
        for i in 0..num_predicates {
            result += (self.compliance_step_r1cs_pvks[i].size_in_bits()
                + self.translation_step_r1cs_pvks[i].size_in_bits());
        }

        result += self.commitment_to_translation_step_r1cs_vks.len();

        result
    }
}

pub fn r1cs_mp_ppzkpcd_generator<PCD_ppT: PcdPptConfig>(
    compliance_predicates: &Vec<r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT>>,
) -> r1cs_mp_ppzkpcd_keypair<PCD_ppT> {
    let span0 = span!(Level::TRACE, "Call to r1cs_mp_ppzkpcd_generator");
    let _ = span0.enter();

    let mut keypair = r1cs_mp_ppzkpcd_keypair::<PCD_ppT>::default();
    let translation_input_size =
        mp_translation_step_pcd_circuit_maker::<B_pp<PCD_ppT>>::input_size_in_elts();
    let vk_size_in_bits = r1cs_ppzksnark_verification_key_variable::<A_pp<PCD_ppT>>::size_in_bits(
        translation_input_size,
    );
    print!("{} {}\n", translation_input_size, vk_size_in_bits);

    let mut all_translation_vks = set_commitment_accumulator::<
        CRH_with_bit_out_gadgets<FieldT_A<PCD_ppT>, PCD_ppT::PB>,
    >::new(compliance_predicates.len(), vk_size_in_bits.clone());

    let spanp = span!(Level::TRACE, "Perform type checks").entered();
    let mut type_counts = BTreeMap::new();

    for cp in compliance_predicates {
        *type_counts.entry(cp.types).or_insert(0) += 1;
    }

    for cp in compliance_predicates {
        if cp.relies_on_same_type_inputs {
            for types in &cp.accepted_input_types {
                assert!(type_counts[types] == 1); //each of accepted_input_types must be unique
            }
        } else {
            assert!(cp.accepted_input_types.is_empty());
        }
    }
    spanp.exit();

    for i in 0..compliance_predicates.len() {
        let s = prefix_format!(
            "",
            "Process predicate {} (with name {} and type {})",
            i,
            compliance_predicates[i].name,
            compliance_predicates[i].types,
        );
        let span0 = span!(Level::TRACE, "{s}");
        let _ = span0.enter();

        assert!(compliance_predicates[i].is_well_formed());

        let span = span!(Level::TRACE, "Construct compliance step PCD circuit").entered();
        let mut mp_compliance_step_pcd_circuit =
            mp_compliance_step_pcd_circuit_maker::<A_pp<PCD_ppT>>::new(
                compliance_predicates[i].clone(),
                compliance_predicates.len(),
            );
        mp_compliance_step_pcd_circuit.generate_r1cs_constraints();
        let mut mp_compliance_step_pcd_circuit_cs = mp_compliance_step_pcd_circuit.get_circuit();
        span.exit();

        let spang = span!(
            Level::TRACE,
            "Generate key pair for compliance step PCD circuit"
        )
        .entered();
        let mut mp_compliance_step_keypair =
            r1cs_ppzksnark_generator::<A_pp<PCD_ppT>>(&mp_compliance_step_pcd_circuit_cs);
        spang.exit();

        let spanc = span!(Level::TRACE, "Construct translation step PCD circuit").entered();
        let mut mp_translation_step_pcd_circuit =
            mp_translation_step_pcd_circuit_maker::<B_pp<PCD_ppT>>::new(
                mp_compliance_step_keypair.vk.clone(),
            );
        mp_translation_step_pcd_circuit.generate_r1cs_constraints();
        let mp_translation_step_pcd_circuit_cs = mp_translation_step_pcd_circuit.get_circuit();
        spanc.exit();

        let spant = span!(
            Level::TRACE,
            "Generate key pair for translation step PCD circuit"
        )
        .entered();
        let mut mp_translation_step_keypair =
            r1cs_ppzksnark_generator::<B_pp<PCD_ppT>>(&mp_translation_step_pcd_circuit_cs);
        spant.exit();

        let spana = span!(
            Level::TRACE,
            "Augment set of translation step verification keys"
        )
        .entered();
        let vk_bits =
            r1cs_ppzksnark_verification_key_variable::<A_pp<PCD_ppT>>::get_verification_key_bits(
                &mp_translation_step_keypair.vk,
            );
        all_translation_vks.add(&vk_bits);
        spana.exit();

        let spanu = span!(Level::TRACE, "Update r1cs_mp_ppzkpcd keypair").entered();
        keypair
            .pk
            .compliance_predicates
            .push(compliance_predicates[i].clone());
        keypair
            .pk
            .compliance_step_r1cs_pks
            .push(mp_compliance_step_keypair.pk.clone());
        keypair
            .pk
            .translation_step_r1cs_pks
            .push(mp_translation_step_keypair.pk.clone());
        keypair
            .pk
            .compliance_step_r1cs_vks
            .push(mp_compliance_step_keypair.vk.clone());
        keypair
            .pk
            .translation_step_r1cs_vks
            .push(mp_translation_step_keypair.vk.clone());
        let cp_name = compliance_predicates[i].name;
        assert!(
            !keypair
                .pk
                .compliance_predicate_name_to_idx
                .contains_key(&cp_name)
        ); // all names must be distinct
        keypair
            .pk
            .compliance_predicate_name_to_idx
            .insert(cp_name, i);

        keypair
            .vk
            .compliance_step_r1cs_vks
            .push(mp_compliance_step_keypair.vk.clone());
        keypair
            .vk
            .translation_step_r1cs_vks
            .push(mp_translation_step_keypair.vk.clone());
        spanu.exit();
    }
    let spanc = span!(
        Level::TRACE,
        "Compute set commitment and corresponding membership proofs"
    )
    .entered();

    let cm = all_translation_vks.get_commitment();
    keypair.pk.commitment_to_translation_step_r1cs_vks = cm.clone();
    keypair.vk.commitment_to_translation_step_r1cs_vks = cm;
    for i in 0..compliance_predicates.len() {
        let vk_bits =
            r1cs_ppzksnark_verification_key_variable::<A_pp<PCD_ppT>>::get_verification_key_bits(
                &keypair.vk.translation_step_r1cs_vks[i],
            );
        let proof = all_translation_vks.get_membership_proof(&vk_bits);

        keypair
            .pk
            .compliance_step_r1cs_vk_membership_proofs
            .push(proof);
    }
    let _ = spanc.exit();

    print_indent();
    println!("in generator");

    keypair
}

pub fn r1cs_mp_ppzkpcd_prover<PCD_ppT: PcdPptConfig>(
    pk: &r1cs_mp_ppzkpcd_proving_key<PCD_ppT>,
    compliance_predicate_name: usize,
    primary_input: &r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
    auxiliary_input: &r1cs_mp_ppzkpcd_auxiliary_input<PCD_ppT>,
    prev_proofs: &Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT>>,
) -> r1cs_mp_ppzkpcd_proof<PCD_ppT>
where
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
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
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
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<PCD_ppT as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<PCD_ppT as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
{
    let span0 = span!(Level::TRACE, "Call to r1cs_mp_ppzkpcd_prover");
    let _ = span0.enter();

    // #ifdef DEBUG
    print!("Compliance predicate name: {}\n", compliance_predicate_name);

    let mut it = pk
        .compliance_predicate_name_to_idx
        .get(&compliance_predicate_name);
    assert!(it.is_some());
    let compliance_predicate_idx = *it.unwrap();

    // #ifdef DEBUG
    print!("Outgoing message:\n");
    primary_input.outgoing_message.borrow().print();

    let spanp = span!(Level::TRACE, "Prove compliance step").entered();
    assert!(compliance_predicate_idx < pk.compliance_predicates.len());
    assert!(prev_proofs.len() <= pk.compliance_predicates[compliance_predicate_idx].max_arity);

    let arity = prev_proofs.len();
    let max_arity = pk.compliance_predicates[compliance_predicate_idx].max_arity;

    if pk.compliance_predicates[compliance_predicate_idx].relies_on_same_type_inputs {
        let input_predicate_idx = prev_proofs[0].compliance_predicate_idx;
        for i in 1..arity {
            assert!(prev_proofs[i].compliance_predicate_idx == input_predicate_idx);
        }
    }

    let mut padded_proofs = vec![r1cs_ppzksnark_proof::<B_pp<PCD_ppT>>::default(); max_arity];
    for i in 0..arity {
        padded_proofs[i] = prev_proofs[i].r1cs_proof.clone();
    }

    let mut translation_step_vks = vec![];
    let mut membership_proofs = vec![];

    for i in 0..arity {
        let input_predicate_idx = prev_proofs[i].compliance_predicate_idx;
        translation_step_vks.push(pk.translation_step_r1cs_vks[input_predicate_idx].clone());
        membership_proofs
            .push(pk.compliance_step_r1cs_vk_membership_proofs[input_predicate_idx].clone());

        // #ifdef DEBUG
        if auxiliary_input.incoming_messages[i].borrow().types != 0 {
            print!("check proof for message {}\n", i);
            let translated_msg = get_mp_translation_step_pcd_circuit_input::<B_pp<PCD_ppT>>(
                &pk.commitment_to_translation_step_r1cs_vks,
                &(auxiliary_input.incoming_messages[i].clone().into()),
            );
            let bit = r1cs_ppzksnark_verifier_strong_IC::<B_pp<PCD_ppT>>(
                &translation_step_vks[i],
                &translated_msg,
                &padded_proofs[i],
            );
            assert!(bit);
        } else {
            print!("message {} is base case\n", i);
        }
    }

    //pad with dummy vks/membership proofs
    for i in arity..max_arity {
        print!("proof {} will be a dummy\n", arity);
        translation_step_vks.push(pk.translation_step_r1cs_vks[0].clone());
        membership_proofs.push(pk.compliance_step_r1cs_vk_membership_proofs[0].clone());
    }

    let mut mp_compliance_step_pcd_circuit =
        mp_compliance_step_pcd_circuit_maker::<A_pp<PCD_ppT>>::new(
            pk.compliance_predicates[compliance_predicate_idx].clone(),
            pk.compliance_predicates.len(),
        );

    mp_compliance_step_pcd_circuit.generate_r1cs_witness(
        &pk.commitment_to_translation_step_r1cs_vks,
        &translation_step_vks,
        &membership_proofs,
        &primary_input,
        &auxiliary_input,
        &padded_proofs,
    );

    let compliance_step_primary_input = mp_compliance_step_pcd_circuit.get_primary_input();
    let compliance_step_auxiliary_input = mp_compliance_step_pcd_circuit.get_auxiliary_input();
    let compliance_step_proof = r1cs_ppzksnark_prover::<A_pp<PCD_ppT>>(
        &pk.compliance_step_r1cs_pks[compliance_predicate_idx],
        &compliance_step_primary_input,
        &compliance_step_auxiliary_input,
    );
    spanp.exit();

    // #ifdef DEBUG
    let compliance_step_input = get_mp_compliance_step_pcd_circuit_input::<A_pp<PCD_ppT>>(
        &pk.commitment_to_translation_step_r1cs_vks,
        &(primary_input.outgoing_message.clone().into()),
    );
    let compliance_step_ok = r1cs_ppzksnark_verifier_strong_IC::<A_pp<PCD_ppT>>(
        &pk.compliance_step_r1cs_vks[compliance_predicate_idx],
        &compliance_step_input,
        &compliance_step_proof,
    );
    assert!(compliance_step_ok);

    let span = span!(Level::TRACE, "Prove translation step").entered();
    let mut mp_translation_step_pcd_circuit =
        mp_translation_step_pcd_circuit_maker::<B_pp<PCD_ppT>>::new(
            pk.compliance_step_r1cs_vks[compliance_predicate_idx].clone(),
        );

    let translation_step_primary_input = get_mp_translation_step_pcd_circuit_input::<B_pp<PCD_ppT>>(
        &pk.commitment_to_translation_step_r1cs_vks,
        primary_input,
    );
    mp_translation_step_pcd_circuit
        .generate_r1cs_witness(&translation_step_primary_input, &compliance_step_proof);
    let translation_step_auxiliary_input = mp_translation_step_pcd_circuit.get_auxiliary_input();

    let translation_step_proof = r1cs_ppzksnark_prover::<B_pp<PCD_ppT>>(
        &pk.translation_step_r1cs_pks[compliance_predicate_idx],
        &translation_step_primary_input,
        &translation_step_auxiliary_input,
    );

    span.exit();

    let translation_step_ok = r1cs_ppzksnark_verifier_strong_IC::<B_pp<PCD_ppT>>(
        &pk.translation_step_r1cs_vks[compliance_predicate_idx],
        &translation_step_primary_input,
        &translation_step_proof,
    );
    assert!(translation_step_ok);

    print_indent();
    println!("in prover");

    let mut result = r1cs_mp_ppzkpcd_proof::<PCD_ppT>::default();
    result.compliance_predicate_idx = compliance_predicate_idx;
    result.r1cs_proof = translation_step_proof;
    result
}

pub fn r1cs_mp_ppzkpcd_online_verifier<PCD_ppT: PcdPptConfig>(
    pvk: &r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>,
    primary_input: &r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_mp_ppzkpcd_proof<PCD_ppT>,
) -> bool {
    let span0 = span!(Level::TRACE, "Call to r1cs_mp_ppzkpcd_online_verifier");
    let _ = span0.enter();

    let r1cs_input = get_mp_translation_step_pcd_circuit_input::<B_pp<PCD_ppT>>(
        &pvk.commitment_to_translation_step_r1cs_vks,
        primary_input,
    );
    let result = r1cs_ppzksnark_online_verifier_strong_IC(
        &pvk.translation_step_r1cs_pvks[proof.compliance_predicate_idx],
        &r1cs_input,
        &proof.r1cs_proof,
    );

    print_indent();
    println!("in online verifier");

    result
}

pub fn r1cs_mp_ppzkpcd_process_vk<PCD_ppT: PcdPptConfig>(
    vk: &r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
) -> r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> {
    let span = span!(
        Level::TRACE,
        "Call to r1cs_mp_ppzkpcd_processed_verification_key"
    );
    let _ = span.enter();

    let mut result = r1cs_mp_ppzkpcd_processed_verification_key::<PCD_ppT>::default();
    result.commitment_to_translation_step_r1cs_vks =
        vk.commitment_to_translation_step_r1cs_vks.clone();

    for i in 0..vk.compliance_step_r1cs_vks.len() {
        let compliance_step_r1cs_pvk =
            r1cs_ppzksnark_verifier_process_vk::<A_pp<PCD_ppT>>(&vk.compliance_step_r1cs_vks[i]);
        let translation_step_r1cs_pvk =
            r1cs_ppzksnark_verifier_process_vk::<B_pp<PCD_ppT>>(&vk.translation_step_r1cs_vks[i]);

        result
            .compliance_step_r1cs_pvks
            .push(compliance_step_r1cs_pvk);
        result
            .translation_step_r1cs_pvks
            .push(translation_step_r1cs_pvk);
    }

    result
}

pub fn r1cs_mp_ppzkpcd_verifier<PCD_ppT: PcdPptConfig>(
    vk: &r1cs_mp_ppzkpcd_verification_key<PCD_ppT>,
    primary_input: &r1cs_mp_ppzkpcd_primary_input<PCD_ppT>,
    proof: &r1cs_mp_ppzkpcd_proof<PCD_ppT>,
) -> bool {
    let span = span!(Level::TRACE, "Call to r1cs_mp_ppzkpcd_verifier").entered();
    let pvk = r1cs_mp_ppzkpcd_process_vk(vk);
    let result = r1cs_mp_ppzkpcd_online_verifier(&pvk, primary_input, proof);

    print_indent();
    println!("in verifier");
    span.exit();
    result
}
