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


use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::{ram_zksnark_auxiliary_input,ram_zksnark_architecture_params,ram_zksnark_PCD_pp};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate::ram_compliance_predicate_handler;
use crate::relations::ram_computations::memory::delegated_ra_memory::delegated_ra_memory;
 use crate::gadgetlib1::gadgets::hashes::crh_gadget::CRH_with_bit_out_gadget;
use ffec::common::profiling::{enter_block,leave_block};
 use ffec::log2;



/******************************** Proving key ********************************/

/**
 * A proving key for the RAM zkSNARK.
 */

#[derive(Default, Clone)]
pub struct ram_zksnark_proving_key<ram_zksnark_ppT> {
    pub ap: ram_zksnark_architecture_params<ram_zksnark_ppT>,
    pub pcd_pk: r1cs_sp_ppzkpcd_proving_key<ram_zksnark_PCD_pp<ram_zksnark_ppT>>,
}
impl<ram_zksnark_ppT> ram_zksnark_proving_key<ram_zksnark_ppT> {
    pub fn new(
        ap: ram_zksnark_architecture_params<ram_zksnark_ppT>,
        pcd_pk: r1cs_sp_ppzkpcd_proving_key<ram_zksnark_PCD_pp<ram_zksnark_ppT>>,
    ) -> Self {
        Self { ap, pcd_pk }
    }
}

/******************************* Verification key ****************************/

/**
 * A verification key for the RAM zkSNARK.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_verification_key<ram_zksnark_ppT> {
    pub ap: ram_zksnark_architecture_params<ram_zksnark_ppT>,
    pub pcd_vk: r1cs_sp_ppzkpcd_verification_key<ram_zksnark_PCD_pp<ram_zksnark_ppT>>,
}
impl<ram_zksnark_ppT> ram_zksnark_verification_key<ram_zksnark_ppT> {
    pub fn new(
        ap: ram_zksnark_architecture_params<ram_zksnark_ppT>,
        pcd_vk: r1cs_sp_ppzkpcd_verification_key<ram_zksnark_PCD_pp<ram_zksnark_ppT>>,
    ) -> Self {
        Self { ap, pcd_vk }
    }
}

/********************************** Key pair *********************************/

/**
 * A key pair for the RAM zkSNARK, which consists of a proving key and a verification key.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_keypair<ram_zksnark_ppT> {
    pub pk: ram_zksnark_proving_key<ram_zksnark_ppT>,
    pub vk: ram_zksnark_verification_key<ram_zksnark_ppT>,
}
impl<ram_zksnark_ppT> ram_zksnark_keypair<ram_zksnark_ppT> {
    pub fn new(
        pk: ram_zksnark_proving_key<ram_zksnark_ppT>,
        vk: ram_zksnark_verification_key<ram_zksnark_ppT>,
    ) -> Self {
        Self { pk, vk }
    }
}

/*********************************** Proof ***********************************/

/**
 * A proof for the RAM zkSNARK.
 */
#[derive(Default, Clone)]
pub struct ram_zksnark_proof<ram_zksnark_ppT> {
    pub PCD_proof: r1cs_sp_ppzkpcd_proof<ram_zksnark_PCD_pp<ram_zksnark_ppT>>,
}

impl<ram_zksnark_ppT> ram_zksnark_proof<ram_zksnark_ppT> {
    pub fn new(PCD_proof: r1cs_sp_ppzkpcd_proof<ram_zksnark_PCD_pp<ram_zksnark_ppT>>) -> Self {
        Self { PCD_proof }
    }

    pub fn size_in_bits() -> usize {
        return PCD_proof.size_in_bits();
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
// ram_zksnark_keypair<ram_zksnark_ppT> ram_zksnark_generator(ap:&ram_zksnark_architecture_params<ram_zksnark_ppT>);

// /**
//  * A prover algorithm for the RAM zkSNARK.
//  *
//  * Given a proving key, primary input X, time bound T, and auxiliary input Y, this algorithm
//  * produces a proof (of knowledge) that attests to the following statement:
//  *               ``there exists Y such that X(Y) accepts within T steps''.
//  */
//
// ram_zksnark_proof<ram_zksnark_ppT> ram_zksnark_prover(pk:&ram_zksnark_proving_key<ram_zksnark_ppT>,
//                                                       primary_input:&ram_zksnark_primary_input<ram_zksnark_ppT>,
//                                                       time_bound:&usize,
//                                                       auxiliary_input:&ram_zksnark_auxiliary_input<ram_zksnark_ppT>);

// /**
//  * A verifier algorithm for the RAM zkSNARK.
//  *
//  * This algorithm is universal in the sense that the verification key
//  * supports proof verification for *any* choice of primary input and time bound.
//  */
//
// bool ram_zksnark_verifier(vk:&ram_zksnark_verification_key<ram_zksnark_ppT>,
//                           primary_input:&ram_zksnark_primary_input<ram_zksnark_ppT>,
//                           time_bound:&usize,
//                           proof:&ram_zksnark_proof<ram_zksnark_ppT>);

// use common::profiling;

//
// bool ram_zksnark_proving_key<ram_zksnark_ppT>::operator==(other:&ram_zksnark_proving_key<ram_zksnark_ppT>) const
// {
//     return (self.ap == other.ap &&
//             self.pcd_pk == other.pcd_pk);
// }

//
// std::ostream& operator<<(std::ostream &out, pk:&ram_zksnark_proving_key<ram_zksnark_ppT>)
// {
//     out << pk.ap;
//     out << pk.pcd_pk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_proving_key<ram_zksnark_ppT> &pk)
// {
//     in >> pk.ap;
//     in >> pk.pcd_pk;

//     return in;
// }

//
// bool ram_zksnark_verification_key<ram_zksnark_ppT>::operator==(other:&ram_zksnark_verification_key<ram_zksnark_ppT>) const
// {
//     return (self.ap == other.ap &&
//             self.pcd_vk == other.pcd_vk);
// }

//
// std::ostream& operator<<(std::ostream &out, vk:&ram_zksnark_verification_key<ram_zksnark_ppT>)
// {
//     out << vk.ap;
//     out << vk.pcd_vk;

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_verification_key<ram_zksnark_ppT> &vk)
// {
//     in >> vk.ap;
//     in >> vk.pcd_vk;

//     return in;
// }

//
// bool ram_zksnark_proof<ram_zksnark_ppT>::operator==(other:&ram_zksnark_proof<ram_zksnark_ppT>) const
// {
//     return (self.PCD_proof == other.PCD_proof);
// }

//
// std::ostream& operator<<(std::ostream &out, proof:&ram_zksnark_proof<ram_zksnark_ppT>)
// {
//     out << proof.PCD_proof;
//     return out;
// }

//
// std::istream& operator>>(std::istream &in, ram_zksnark_proof<ram_zksnark_ppT> &proof)
// {
//     in >> proof.PCD_proof;
//     return in;
// }

impl<ram_zksnark_ppT> ram_zksnark_verification_key<ram_zksnark_ppT> {
    pub fn dummy_verification_key(
        ap: &ram_zksnark_architecture_params<ram_zksnark_ppT>,
    ) -> ram_zksnark_verification_key<ram_zksnark_ppT> {
        // type pcdT = ram_zksnark_PCD_pp<ram_zksnark_ppT>;

        ram_zksnark_verification_key::<ram_zksnark_ppT>(
            ap,
            r1cs_sp_ppzkpcd_verification_key::<ram_zksnark_PCD_pp<ram_zksnark_ppT>>::dummy_verification_key(),
        )
    }
}

pub fn ram_zksnark_generator<ram_zksnark_ppT>(
    ap: &ram_zksnark_architecture_params<ram_zksnark_ppT>,
) -> ram_zksnark_keypair<ram_zksnark_ppT> {
    // type ramT=ram_zksnark_machine_pp<ram_zksnark_ppT>;
    // type pcdT=ram_zksnark_PCD_pp<ram_zksnark_ppT>;
    enter_block("Call to ram_zksnark_generator");

    enter_block("Generate compliance predicate for RAM");
    let mut cp_handler = ram_compliance_predicate_handler::<ramT>::new(ap);
    cp_handler.generate_r1cs_constraints();
    let mut ram_compliance_predicate = cp_handler.get_compliance_predicate();
    leave_block("Generate compliance predicate for RAM");

    enter_block("Generate PCD key pair");
    let mut kp = r1cs_sp_ppzkpcd_generator::<pcdT>(ram_compliance_predicate);
    leave_block("Generate PCD key pair");

    leave_block("Call to ram_zksnark_generator");

    let pk = ram_zksnark_proving_key::<ram_zksnark_ppT>(ap, (kp.pk));
    let vk = ram_zksnark_verification_key::<ram_zksnark_ppT>(ap, (kp.vk));

    return ram_zksnark_keypair::<ram_zksnark_ppT>(pk, vk);
}

pub fn ram_zksnark_prover<ram_zksnark_ppT>(
    pk: &ram_zksnark_proving_key<ram_zksnark_ppT>,
    primary_input: &ram_zksnark_primary_input<ram_zksnark_ppT>,
    time_bound: &usize,
    auxiliary_input: &ram_zksnark_auxiliary_input<ram_zksnark_ppT>,
) -> ram_zksnark_proof<ram_zksnark_ppT> {
    // type ramT=ram_zksnark_machine_pp<ram_zksnark_ppT>;
    // type pcdT=ram_zksnark_PCD_pp<ram_zksnark_ppT>;
    // type FieldT=Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>; // XXX

    assert!(log2(time_bound) <= ram_zksnark_machine_pp::<ram_zksnark_ppT>::timestamp_length);

    enter_block("Call to ram_zksnark_prover");
    enter_block("Generate compliance predicate for RAM");
    let mut cp_handler = ram_compliance_predicate_handler::<ram_zksnark_machine_pp<ram_zksnark_ppT>>::new(pk.ap);
    leave_block("Generate compliance predicate for RAM");

    enter_block("Initialize the RAM computation");
    let mut cur_proof = r1cs_sp_ppzkpcd_proof::<ram_zksnark_PCD_pp<ram_zksnark_ppT>>::default(); // start out with an empty proof

    /* initialize memory with the correct values */
    let num_addresses = 1u64 << pk.ap.address_size();
    let value_size = pk.ap.value_size();

    let mut mem = delegated_ra_memory::<CRH_with_bit_out_gadget<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>>::new(
        num_addresses,
        value_size,
        primary_input.as_memory_contents(),
    );
    let msg = ram_compliance_predicate_handler::<ram_zksnark_machine_pp<ram_zksnark_ppT>>::get_base_case_message(pk.ap, primary_input);

    let aux_it = auxiliary_input.begin();
    leave_block("Initialize the RAM computation");

    enter_block("Execute and prove the computation");
    let mut want_halt = false;
    for step in 1..=time_bound {
        enter_block(FMT("", "Prove step {} out of {}", step, time_bound));

        enter_block("Execute witness map");

        let local_data = r1cs_pcd_local_data::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::new();
        local_data = RcCell::new(ram_pcd_local_data::<ramT>::new(
            want_halt,
            mem,
            aux_it,
            auxiliary_input.end(),
        ));

        cp_handler.generate_r1cs_witness([msg], local_data);

        let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::new(
            cp_handler.get_outgoing_message(),
        );
        let cp_auxiliary_input = r1cs_pcd_compliance_predicate_auxiliary_input::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::new(
            [msg],
            local_data,
            cp_handler.get_witness(),
        );

        // #ifdef DEBUG
        print!("Current state:\n");
        msg.print();
        //#endif

        msg = cp_handler.get_outgoing_message();

        // #ifdef DEBUG
        print!("Next state:\n");
        msg.print();
        //#endif
        leave_block("Execute witness map");

        cur_proof = r1cs_sp_ppzkpcd_prover::<ram_zksnark_PCD_pp<ram_zksnark_ppT>>(
            pk.pcd_pk,
            cp_primary_input,
            cp_auxiliary_input,
            [cur_proof],
        );
        leave_block(FMT("", "Prove step {} out of {}", step, time_bound));
    }
    leave_block("Execute and prove the computation");

    enter_block("Finalize the computation");
    want_halt = true;

    enter_block("Execute witness map");

    let mut local_data = r1cs_pcd_local_data::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::default();
    local_data = RcCell::new(ram_pcd_local_data::<ram_zksnark_machine_pp<ram_zksnark_ppT>>::new(
        want_halt,
        mem,
        aux_it,
        auxiliary_input.end(),
    ));

    cp_handler.generate_r1cs_witness([msg], local_data);

    let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::new(
        cp_handler.get_outgoing_message(),
    );
    let cp_auxiliary_input = r1cs_pcd_compliance_predicate_auxiliary_input::<Fr< ram_zksnark_PCD_pp<ram_zksnark_ppT>::curve_A_pp>>::new(
        [msg],
        local_data,
        cp_handler.get_witness(),
    );
    leave_block("Execute witness map");

    cur_proof = r1cs_sp_ppzkpcd_prover::<ram_zksnark_PCD_pp<ram_zksnark_ppT>>(
        pk.pcd_pk,
        cp_primary_input,
        cp_auxiliary_input,
        [cur_proof],
    );
    leave_block("Finalize the computation");

    leave_block("Call to ram_zksnark_prover");

     cur_proof
}

pub fn ram_zksnark_verifier<ram_zksnark_ppT>(
    vk: &ram_zksnark_verification_key<ram_zksnark_ppT>,
    primary_input: &ram_zksnark_primary_input<ram_zksnark_ppT>,
    time_bound: &usize,
    proof: &ram_zksnark_proof<ram_zksnark_ppT>,
) -> bool {
    // type ramT=ram_zksnark_machine_pp<ram_zksnark_ppT>;
    // type pcdT=ram_zksnark_PCD_pp<ram_zksnark_ppT>;
    // type FieldT=Fr< pcdT::curve_A_pp>; // XXX

    enter_block("Call to ram_zksnark_verifier");
    let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<FieldT>::new(
        ram_compliance_predicate_handler::<ramT>::get_final_case_msg(
            vk.ap,
            primary_input,
            time_bound,
        ),
    );
    let ans = r1cs_sp_ppzkpcd_verifier::<pcdT>(vk.pcd_vk, cp_primary_input, proof.PCD_proof);
    leave_block("Call to ram_zksnark_verifier");

    ans
}
