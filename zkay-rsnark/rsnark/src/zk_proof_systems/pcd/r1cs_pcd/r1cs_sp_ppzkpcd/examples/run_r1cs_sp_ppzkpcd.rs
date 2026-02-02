// Declaration of functionality that runs the R1CS single-predicate ppzkPCD
// for a compliance predicate example.

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_local_data;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_message;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp::TallyCPHConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp::tally_pcd_message;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp::{
    tally_cp_handler, tally_pcd_local_data,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::{
    r1cs_pcd_compliance_predicate_auxiliary_input, r1cs_pcd_compliance_predicate_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd::{
    r1cs_sp_ppzkpcd_generator, r1cs_sp_ppzkpcd_online_verifier, r1cs_sp_ppzkpcd_process_vk,
    r1cs_sp_ppzkpcd_processed_verification_key, r1cs_sp_ppzkpcd_proof, r1cs_sp_ppzkpcd_prover,
    r1cs_sp_ppzkpcd_proving_key, r1cs_sp_ppzkpcd_verification_key, r1cs_sp_ppzkpcd_verifier,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd_params::r1cs_sp_ppzkpcd_primary_input;
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::common::serialization::reserialize;
use rccell::RcCell;
use std::collections::BTreeSet;
use std::ops::Mul;
/**
 * Runs the single-predicate ppzkPCD (generator, prover, and verifier) for the
 * "tally compliance predicate", of a given wordsize, arity, and depth.
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
//
// bool run_r1cs_sp_ppzkpcd_tally_example(wordsize:usize,
//                                        arity:usize,
//                                        depth:usize,
//                                        test_serialization:bool);

type FieldT<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>;
pub fn run_r1cs_sp_ppzkpcd_tally_example<
    PCD_ppT: PcdPptConfig<curve_A_pp = PCD_ppT>
        + TallyCPHConfig<
            protoboard_type = protoboard<
                <PCD_ppT as ppTConfig>::FieldT,
                <PCD_ppT as ppTConfig>::PB,
            >,
        >,
>(
    wordsize: usize,
    arity: usize,
    depth: usize,
    test_serialization: bool,
) -> bool
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
    enter_block("Call to run_r1cs_sp_ppzkpcd_tally_example", false);

    let mut all_accept = true;

    enter_block("Generate all messages", false);
    let mut tree_size = 0;
    let mut nodes_in_layer = 1;
    for layer in 0..=depth {
        tree_size += nodes_in_layer;
        nodes_in_layer *= arity;
    }
    let mut tree_elems = vec![0; tree_size];
    for i in 0..tree_size {
        tree_elems[i] = rand::random::<usize>() % 10;
        print!("tree_elems[{}] = {}\n", i, tree_elems[i]);
    }
    leave_block("Generate all messages", false);

    let mut tree_proofs = vec![r1cs_sp_ppzkpcd_proof::<PCD_ppT>::default(); tree_size];
    let mut tree_messages =
        vec![r1cs_pcd_message::<FieldT<PCD_ppT>, PCD_ppT::M>::default(); tree_size];

    enter_block("Generate compliance predicate", false);
    let types = 1;
    let mut tally =
        tally_cp_handler::<PCD_ppT>::new(types, arity, wordsize, false, BTreeSet::new());
    tally.generate_r1cs_constraints();
    let mut tally_cp = tally.get_compliance_predicate();
    leave_block("Generate compliance predicate", false);

    println!("R1CS ppzkPCD Generator");
    let mut keypair = r1cs_sp_ppzkpcd_generator::<PCD_ppT>(&tally_cp);

    println!("Process verification key");
    let mut pvk = r1cs_sp_ppzkpcd_process_vk::<PCD_ppT>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<r1cs_sp_ppzkpcd_proving_key<PCD_ppT>>(&keypair.pk);
        keypair.vk = reserialize::<r1cs_sp_ppzkpcd_verification_key<PCD_ppT>>(&keypair.vk);
        pvk = reserialize::<r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    let mut base_msg = tally.get_base_case_message();
    nodes_in_layer /= arity;
    for layer in (0..=depth).rev() {
        for i in 0..nodes_in_layer {
            let cur_idx = (nodes_in_layer - 1) / (arity - 1) + i;

            let mut msgs = vec![base_msg.clone(); arity];
            let mut proofs = vec![r1cs_sp_ppzkpcd_proof::<PCD_ppT>::default(); arity];

            let base_case = (arity * cur_idx + arity >= tree_size);

            if !base_case {
                for i in 0..arity {
                    msgs[i] = RcCell::new(tree_messages[arity * cur_idx + i + 1].clone());
                    proofs[i] = tree_proofs[arity * cur_idx + i + 1].clone();
                }
            }

            let mut ld = RcCell::new(tally_pcd_local_data::<FieldT<PCD_ppT>>::new(
                tree_elems[cur_idx],
            ));
            tally.generate_r1cs_witness(&msgs, &ld);

            let mut tally_primary_input = r1cs_pcd_compliance_predicate_primary_input::<
                FieldT<PCD_ppT>,
                PCD_ppT::M,
            >::from(tally.get_outgoing_message());
            let mut tally_auxiliary_input =
                r1cs_pcd_compliance_predicate_auxiliary_input::<
                    FieldT<PCD_ppT>,
                    PCD_ppT::M,
                    PCD_ppT::LD,
                >::new(msgs.clone(), ld, tally.get_witness());

            println!("R1CS ppzkPCD Prover");
            let mut proof = r1cs_sp_ppzkpcd_prover::<PCD_ppT>(
                &keypair.pk,
                &tally_primary_input,
                &tally_auxiliary_input,
                &proofs,
            );

            if test_serialization {
                enter_block("Test serialization of proof", false);
                proof = reserialize::<r1cs_sp_ppzkpcd_proof<PCD_ppT>>(&proof);
                leave_block("Test serialization of proof", false);
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = tally.get_outgoing_message().borrow().clone();

            println!("R1CS ppzkPCD Verifier");
            let pcd_verifier_input = r1cs_sp_ppzkpcd_primary_input::<PCD_ppT>::from(RcCell::new(
                tree_messages[cur_idx].clone(),
            ));
            let ans = r1cs_sp_ppzkpcd_verifier::<PCD_ppT>(
                &keypair.vk,
                &pcd_verifier_input,
                &tree_proofs[cur_idx],
            );

            println!("R1CS ppzkPCD Online Verifier");
            let ans2 = r1cs_sp_ppzkpcd_online_verifier::<PCD_ppT>(
                &pvk,
                &pcd_verifier_input,
                &tree_proofs[cur_idx],
            );
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..arity {
                print!("Message {} was:\n", i);
                msgs[i].borrow().print();
            }
            print!("Summand at this node:\n{}\n", tree_elems[cur_idx]);
            print!("Outgoing message is:\n");
            tree_messages[cur_idx].print();
            print!("\n");
            print!(
                "Current node = {}. Current proof verifies = {}\n",
                cur_idx,
                if ans { "YES" } else { "NO" }
            );
            print!(
                "\n\n\n ================================================================================\n\n\n"
            );
        }
        nodes_in_layer /= arity;
    }

    leave_block("Call to run_r1cs_sp_ppzkpcd_tally_example", false);

    all_accept
}
