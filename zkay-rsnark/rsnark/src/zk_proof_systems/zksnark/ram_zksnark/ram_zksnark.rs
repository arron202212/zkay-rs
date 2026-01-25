// Declaration of interfaces for a zkSNARK for RAM.

// This includes:
// - the pub struct for a proving key;
// - the pub struct for a verification key;
// - the pub struct for a key pair (proving key & verification key);
// - the pub struct for a proof;
// - the generator algorithm;
// - the prover algorithm;
// - the verifier algorithm.

// The implementation follows, extends, and optimizes the approach described
// in \[BCTV14]. Thus, the zkSNARK is constructed from a ppzkPCD for R1CS.

// Acronyms:

// "R1CS" = "Rank-1 Constraint Systems"
// "RAM" = "Random-Access Machines"
// "zkSNARK" = "Zero-Knowledge Succinct Non-interactive ARgument of Knowledge"
// "ppzkPCD" = "Pre-Processing Zero-Knowledge Proof-Carrying Data"

// References:

// \[BCTV14]:
// "Scalable Zero Knowledge via Cycles of Elliptic Curves",
// Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
// CRYPTO 2014,
// <http://eprint.iacr.org/2014/595>

use crate::gadgetlib1::gadgets::hashes::crh_gadget::{
    CRH_with_bit_out_gadget, CRH_with_bit_out_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::prefix_format;
use crate::relations::ram_computations::memory::delegated_ra_memory::delegated_ra_memory;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_boot_trace, ram_input_tape, ram_params_type,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_local_data;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::CPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::{
    r1cs_pcd_compliance_predicate_auxiliary_input, r1cs_pcd_compliance_predicate_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_generator;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_prover;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd::{
    r1cs_sp_ppzkpcd_proof, r1cs_sp_ppzkpcd_proving_key, r1cs_sp_ppzkpcd_verification_key,
    r1cs_sp_ppzkpcd_verifier,
};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate::ram_compliance_predicate_handler;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate::ram_compliance_predicate_handlers;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate::ram_pcd_local_data;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::RamConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::{
    ram_zksnark_PCD_pp, ram_zksnark_architecture_params, ram_zksnark_auxiliary_input,
    ram_zksnark_machine_pp, ram_zksnark_primary_input,
};
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::log2;
use rccell::RcCell;
use std::ops::Mul;
/******************************** Proving key ********************************/

/**
 * A proving key for the RAM zkSNARK.
 */

#[derive(Default, Clone)]
pub struct ram_zksnark_proving_key<RamPpt: RamConfig> {
    pub ap: ram_zksnark_architecture_params<RamPpt>,
    pub pcd_pk: r1cs_sp_ppzkpcd_proving_key<pcdT<RamPpt>>,
}
impl<RamPpt: RamConfig> ram_zksnark_proving_key<RamPpt> {
    pub fn new(
        ap: ram_zksnark_architecture_params<RamPpt>,
        pcd_pk: r1cs_sp_ppzkpcd_proving_key<pcdT<RamPpt>>,
    ) -> Self {
        Self { ap, pcd_pk }
    }
}

/******************************* Verification key ****************************/

/**
 * A verification key for the RAM zkSNARK.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_verification_key<RamPpt: RamConfig> {
    pub ap: ram_zksnark_architecture_params<RamPpt>,
    pub pcd_vk: r1cs_sp_ppzkpcd_verification_key<pcdT<RamPpt>>,
}
impl<RamPpt: RamConfig> ram_zksnark_verification_key<RamPpt> {
    pub fn new(
        ap: ram_zksnark_architecture_params<RamPpt>,
        pcd_vk: r1cs_sp_ppzkpcd_verification_key<pcdT<RamPpt>>,
    ) -> Self {
        Self { ap, pcd_vk }
    }
}

/********************************** Key pair *********************************/

/**
 * A key pair for the RAM zkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_keypair<RamPpt: RamConfig> {
    pub pk: ram_zksnark_proving_key<RamPpt>,
    pub vk: ram_zksnark_verification_key<RamPpt>,
}
impl<RamPpt: RamConfig> ram_zksnark_keypair<RamPpt> {
    pub fn new(
        pk: ram_zksnark_proving_key<RamPpt>,
        vk: ram_zksnark_verification_key<RamPpt>,
    ) -> Self {
        Self { pk, vk }
    }
}

/*********************************** Proof ***********************************/

/**
 * A proof for the RAM zkSNARK.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_proof<RamPpt: RamConfig> {
    pub PCD_proof: r1cs_sp_ppzkpcd_proof<pcdT<RamPpt>>,
}

impl<RamPpt: RamConfig> From<r1cs_sp_ppzkpcd_proof<pcdT<RamPpt>>> for ram_zksnark_proof<RamPpt> {
    fn from(PCD_proof: r1cs_sp_ppzkpcd_proof<pcdT<RamPpt>>) -> Self {
        Self { PCD_proof }
    }
}

impl<RamPpt: RamConfig> ram_zksnark_proof<RamPpt> {
    pub fn size_in_bits() -> usize {
        r1cs_sp_ppzkpcd_proof::<pcdT<RamPpt>>::size_in_bits()
    }
}

/***************************** Main algorithms *******************************/

// /**
//  * A generator algorithm for the RAM zkSNARK.
//  *
//  * Given a choice of architecture parameters, this algorithm produces proving
//  * and verification keys for all computations that respect this choice.
//  */
//
// ram_zksnark_keypair<RamPpt> ram_zksnark_generator(ap:&ram_zksnark_architecture_params<RamPpt>);

// /**
//  * A prover algorithm for the RAM zkSNARK.
//  *
//  * Given a proving key, primary input X, time bound T, and auxiliary input Y, this algorithm
//  * produces a proof (of knowledge) that attests to the following statement:
//  *               ``there exists Y such that X(Y) accepts within T steps''.
//  */
//
// ram_zksnark_proof<RamPpt> ram_zksnark_prover(pk:&ram_zksnark_proving_key<RamPpt>,
//                                                       primary_input:&ram_zksnark_primary_input,
//                                                       time_bound:&usize,
//                                                       auxiliary_input:&ram_zksnark_auxiliary_input);

// /**
//  * A verifier algorithm for the RAM zkSNARK.
//  *
//  * This algorithm is universal in the sense that the verification key
//  * supports proof verification for *any* choice of primary input and time bound.
//  */
//
// bool ram_zksnark_verifier(vk:&ram_zksnark_verification_key<RamPpt>,
//                           primary_input:&ram_zksnark_primary_input,
//                           time_bound:&usize,
//                           proof:&ram_zksnark_proof<RamPpt>);

// use common::profiling;

//
// bool ram_zksnark_proving_key<RamPpt>::operator==(other:&ram_zksnark_proving_key<RamPpt>) const
// {
//     return (self.ap == other.ap &&
//             self.pcd_pk == other.pcd_pk);
// }

//
// std::ostream& operator<<(std::ostream &out, pk:&ram_zksnark_proving_key<RamPpt>)
// {
//     out << pk.ap;
//     out << pk.pcd_pk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_proving_key<RamPpt> &pk)
// {
//     in >> pk.ap;
//     in >> pk.pcd_pk;

//     return in;
// }

//
// bool ram_zksnark_verification_key<RamPpt>::operator==(other:&ram_zksnark_verification_key<RamPpt>) const
// {
//     return (self.ap == other.ap &&
//             self.pcd_vk == other.pcd_vk);
// }

//
// std::ostream& operator<<(std::ostream &out, vk:&ram_zksnark_verification_key<RamPpt>)
// {
//     out << vk.ap;
//     out << vk.pcd_vk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_verification_key<RamPpt> &vk)
// {
//     in >> vk.ap;
//     in >> vk.pcd_vk;

//     return in;
// }

//
// bool ram_zksnark_proof<RamPpt>::operator==(other:&ram_zksnark_proof<RamPpt>) const
// {
//     return (self.PCD_proof == other.PCD_proof);
// }

//
// std::ostream& operator<<(std::ostream &out, proof:&ram_zksnark_proof<RamPpt>)
// {
//     out << proof.PCD_proof;
//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_proof<RamPpt> &proof)
// {
//     in >> proof.PCD_proof;
//     return in;
// }

impl<RamPpt: RamConfig> ram_zksnark_verification_key<RamPpt> {
    pub fn dummy_verification_key(
        ap: &ram_zksnark_architecture_params<RamPpt>,
    ) -> ram_zksnark_verification_key<RamPpt> {
        // type pcdT = ram_zksnark_PCD_pp<RamPpt>;

        ram_zksnark_verification_key::<RamPpt>::new(
            ap.clone(),
            r1cs_sp_ppzkpcd_verification_key::<pcdT<RamPpt>>::dummy_verification_key(),
        )
    }
}

type RamT<RamPpt> = ram_zksnark_machine_pp<RamPpt>;
type pcdT<RamPpt> = ram_zksnark_PCD_pp<RamPpt>;
type A_pp<RamPpt> = <pcdT<RamPpt> as PcdPptConfig>::curve_A_pp;
type FieldT<RamPpt> = Fr<<pcdT<RamPpt> as PcdPptConfig>::curve_A_pp>; // XXX

pub fn ram_zksnark_generator<RamPpt: RamConfig>(
    ap: &ram_zksnark_architecture_params<RamPpt>,
) -> ram_zksnark_keypair<RamPpt>
where
    <<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp: CPHConfig,
{
    // type RamPpt=ram_zksnark_machine_pp<RamPpt>;
    // type pcdT=ram_zksnark_PCD_pp<RamPpt>;
    enter_block("Call to ram_zksnark_generator", false);

    enter_block("Generate compliance predicate for RAM", false);
    let mut cp_handler = ram_compliance_predicate_handler::<RamT<RamPpt>>::new(ap.clone());
    cp_handler.generate_r1cs_constraints();
    let mut ram_compliance_predicate = cp_handler.get_compliance_predicate();
    leave_block("Generate compliance predicate for RAM", false);

    enter_block("Generate PCD key pair", false);
    let mut kp = r1cs_sp_ppzkpcd_generator::<pcdT<RamPpt>>(&ram_compliance_predicate);
    leave_block("Generate PCD key pair", false);

    leave_block("Call to ram_zksnark_generator", false);

    let pk = ram_zksnark_proving_key::<RamPpt>::new(ap.clone(), kp.pk);
    let vk = ram_zksnark_verification_key::<RamPpt>::new(ap.clone(), kp.vk);

    ram_zksnark_keypair::<RamPpt>::new(pk, vk)
}

pub fn ram_zksnark_prover<RamPpt: RamConfig>(
    pk: &ram_zksnark_proving_key<RamPpt>,
    primary_input: &ram_zksnark_primary_input,
    time_bound: usize,
    auxiliary_input: &ram_zksnark_auxiliary_input,
) -> ram_zksnark_proof<RamPpt>
where
    knowledge_commitment<
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G2,
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_B_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
        <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::FieldT,
            Output = knowledge_commitment<
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G2,
                <<RamPpt::PCD_pp as PcdPptConfig>::curve_A_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    <<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp: CPHConfig,
{
    assert!(log2(time_bound) <= RamT::<RamPpt>::timestamp_length);

    enter_block("Call to ram_zksnark_prover", false);
    enter_block("Generate compliance predicate for RAM", false);
    let mut cp_handler = ram_compliance_predicate_handler::<RamT<RamPpt>>::new(pk.ap.clone());
    leave_block("Generate compliance predicate for RAM", false);

    enter_block("Initialize the RAM computation", false);
    let mut cur_proof = r1cs_sp_ppzkpcd_proof::<pcdT<RamPpt>>::default(); // start out with an empty proof

    /* initialize memory with the correct values */
    let num_addresses = 1usize << pk.ap.address_size();
    let value_size = pk.ap.value_size();

    let mut mem = delegated_ra_memory::<
        CRH_with_bit_out_gadgets<
            FieldT<RamPpt>,
            <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::PB,
        >,
    >::new3(
        num_addresses,
        value_size,
        primary_input.as_memory_contents(),
    );
    let mut msg = ram_compliance_predicate_handlers::<RamT<RamPpt>>::get_base_case_message(
        &pk.ap,
        &primary_input,
    );

    // let aux_it = auxiliary_input.begin();
    leave_block("Initialize the RAM computation", false);

    enter_block("Execute and prove the computation", false);
    let mut want_halt = false;
    for step in 1..=time_bound {
        enter_block(
            &prefix_format!("", "Prove step {} out of {}", step, time_bound),
            false,
        );

        enter_block("Execute witness map", false);

        let local_data = RcCell::new(ram_pcd_local_data::<RamT<RamPpt>>::new(
            want_halt,
            mem.clone(),
            auxiliary_input.clone(),
        ));

        cp_handler.generate_r1cs_witness(&vec![msg.clone()], &local_data);

        let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<
            FieldT<RamPpt>,
            <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::M,
        >::from(cp_handler.get_outgoing_message());
        let cp_auxiliary_input =
            r1cs_pcd_compliance_predicate_auxiliary_input::<
                FieldT<RamPpt>,
                <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::M,
                <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::LD,
            >::new(vec![msg.clone()], local_data, cp_handler.get_witness());

        // #ifdef DEBUG
        print!("Current state:\n");
        msg.borrow().print();
        //#endif

        msg = cp_handler.get_outgoing_message();

        // #ifdef DEBUG
        print!("Next state:\n");
        msg.borrow().print();
        //#endif
        leave_block("Execute witness map", false);

        cur_proof = r1cs_sp_ppzkpcd_prover::<pcdT<RamPpt>>(
            &pk.pcd_pk,
            &cp_primary_input,
            &cp_auxiliary_input,
            &vec![cur_proof],
        );
        leave_block(
            &prefix_format!("", "Prove step {} out of {}", step, time_bound),
            false,
        );
    }
    leave_block("Execute and prove the computation", false);

    enter_block("Finalize the computation", false);
    want_halt = true;

    enter_block("Execute witness map", false);

    let mut local_data = RcCell::new(ram_pcd_local_data::<RamT<RamPpt>>::new(
        want_halt,
        mem,
        auxiliary_input.clone(),
    ));

    cp_handler.generate_r1cs_witness(&vec![msg.clone()], &local_data);

    let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<
        FieldT<RamPpt>,
        <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::M,
    >::from(cp_handler.get_outgoing_message());
    let cp_auxiliary_input = r1cs_pcd_compliance_predicate_auxiliary_input::<
        FieldT<RamPpt>,
        <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::M,
        <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::LD,
    >::new(vec![msg.clone()], local_data, cp_handler.get_witness());
    leave_block("Execute witness map", false);

    cur_proof = r1cs_sp_ppzkpcd_prover::<pcdT<RamPpt>>(
        &pk.pcd_pk,
        &cp_primary_input,
        &cp_auxiliary_input,
        &vec![cur_proof],
    );
    leave_block("Finalize the computation", false);

    leave_block("Call to ram_zksnark_prover", false);

    cur_proof.into()
}

pub fn ram_zksnark_verifier<RamPpt: RamConfig>(
    vk: &ram_zksnark_verification_key<RamPpt>,
    primary_input: &ram_zksnark_primary_input,
    time_bound: usize,
    proof: &ram_zksnark_proof<RamPpt>,
) -> bool {
    // type RamT=ram_zksnark_machine_pp<RamPpt>;
    // type pcdT=ram_zksnark_PCD_pp<RamT>;
    // type FieldT=Fr< pcdT::curve_A_pp>; // XXX

    enter_block("Call to ram_zksnark_verifier", false);
    let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<
        FieldT<RamPpt>,
        <<<RamPpt as RamConfig>::PCD_pp as PcdPptConfig>::curve_A_pp as ppTConfig>::M,
    >::from(
        ram_compliance_predicate_handler::<RamT<RamPpt>>::get_final_case_msg(
            &vk.ap,
            primary_input,
            time_bound,
        ),
    );
    let ans =
        r1cs_sp_ppzkpcd_verifier::<pcdT<RamPpt>>(&vk.pcd_vk, &cp_primary_input, &proof.PCD_proof);
    leave_block("Call to ram_zksnark_verifier", false);

    ans
}
