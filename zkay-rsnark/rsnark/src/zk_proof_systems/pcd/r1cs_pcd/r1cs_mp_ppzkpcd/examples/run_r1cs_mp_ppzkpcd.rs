// Declaration of functionality that runs the R1CS multi-predicate ppzkPCD
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
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd::{
    r1cs_mp_ppzkpcd_generator, r1cs_mp_ppzkpcd_online_verifier, r1cs_mp_ppzkpcd_process_vk,
    r1cs_mp_ppzkpcd_processed_verification_key, r1cs_mp_ppzkpcd_proof, r1cs_mp_ppzkpcd_prover,
    r1cs_mp_ppzkpcd_proving_key, r1cs_mp_ppzkpcd_verification_key, r1cs_mp_ppzkpcd_verifier,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd_params::r1cs_mp_ppzkpcd_primary_input;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::{
    r1cs_pcd_compliance_predicate_auxiliary_input, r1cs_pcd_compliance_predicate_primary_input,
};
/**
 * Runs the multi-predicate ppzkPCD (generator, prover, and verifier) for the
 * "tally compliance predicate", of a given wordsize, arity, and depth.
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 *
 * Optionally, also test the case of compliance predicates with different types.
 */
//
// bool run_r1cs_mp_ppzkpcd_tally_example(wordsize:usize,
//                                        max_arity:usize,
//                                        depth:usize,
//                                        test_serialization:bool,
//                                        test_multi_type:bool,
//                                        test_same_type_optimization:bool);

// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp;
// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd;
use ff_curves::Fr;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::common::serialization::reserialize;
use rccell::RcCell;
use std::collections::BTreeSet;
use std::ops::Mul;

type FieldT<PCD_ppT> = Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>;

pub fn run_r1cs_mp_ppzkpcd_tally_example<
    PCD_ppT: PcdPptConfig<curve_A_pp = PCD_ppT>
        + TallyCPHConfig<
            protoboard_type = protoboard<
                <PCD_ppT as ppTConfig>::FieldT,
                <PCD_ppT as ppTConfig>::PB,
            >,
        >,
>(
    wordsize: usize,
    max_arity: usize,
    depth: usize,
    test_serialization: bool,
    test_multi_type: bool,
    test_same_type_optimization: bool,
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
    enter_block("Call to run_r1cs_mp_ppzkpcd_tally_example", false);

    let mut all_accept = true;

    enter_block("Generate all messages", false);
    let mut tree_size = 0;
    let mut nodes_in_layer = 1;
    for layer in 0..=depth {
        tree_size += nodes_in_layer;
        nodes_in_layer *= max_arity;
    }

    let mut tree_types = vec![0; tree_size];
    let mut tree_elems = vec![0; tree_size];
    let mut tree_arity = vec![0; tree_size];

    nodes_in_layer = 1;
    let mut node_idx = 0;
    for layer in 0..=depth {
        for id_in_layer in 0..nodes_in_layer {
            if !test_multi_type {
                tree_types[node_idx] = 1;
            } else {
                if test_same_type_optimization {
                    tree_types[node_idx] = 1 + ((depth - layer) & 1);
                } else {
                    tree_types[node_idx] = 1 + (rand::random::<usize>() % 2);
                }
            }

            tree_elems[node_idx] = rand::random::<usize>() % 100;
            tree_arity[node_idx] = 1 + (rand::random::<usize>() % max_arity); /* we will just skip below this threshold */
            print!("tree_types[{}] = {}\n", node_idx, tree_types[node_idx]);
            print!("tree_elems[{}] = {}\n", node_idx, tree_elems[node_idx]);
            print!("tree_arity[{}] = {}\n", node_idx, tree_arity[node_idx]);
            node_idx += 1;
        }
        nodes_in_layer *= max_arity;
    }

    leave_block("Generate all messages", false);

    let mut tree_proofs = vec![r1cs_mp_ppzkpcd_proof::<PCD_ppT>::default(); tree_size]; //Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >
    let mut tree_messages =
        vec![r1cs_pcd_message::<FieldT<PCD_ppT>, PCD_ppT::M>::default(); tree_size];

    enter_block("Generate compliance predicates", false);
    let (mut tally_1_accepted_types, mut tally_2_accepted_types) =
        (BTreeSet::new(), BTreeSet::new());
    if test_same_type_optimization {
        if !test_multi_type {
            /* only tally 1 is going to be used */
            tally_1_accepted_types.insert(1);
        } else {
            tally_1_accepted_types.insert(2);
            tally_2_accepted_types.insert(1);
        }
    }

    let mut tally_1 = tally_cp_handler::<PCD_ppT>::new(
        1,
        max_arity,
        wordsize,
        test_same_type_optimization,
        tally_1_accepted_types,
    );
    let mut tally_2 = tally_cp_handler::<PCD_ppT>::new(
        2,
        max_arity,
        wordsize,
        test_same_type_optimization,
        tally_2_accepted_types,
    );
    tally_1.generate_r1cs_constraints();
    tally_2.generate_r1cs_constraints();
    let mut cp_1 = tally_1.get_compliance_predicate();
    let mut cp_2 = tally_2.get_compliance_predicate();
    leave_block("Generate compliance predicates", false);

    println!("R1CS ppzkPCD Generator");
    let mut keypair = r1cs_mp_ppzkpcd_generator::<PCD_ppT>(&vec![cp_1.clone(), cp_2.clone()]);

    println!("Process verification key");
    let mut pvk = r1cs_mp_ppzkpcd_process_vk::<PCD_ppT>(&keypair.vk);

    if test_serialization {
        enter_block("Test serialization of keys", false);
        keypair.pk = reserialize::<r1cs_mp_ppzkpcd_proving_key<PCD_ppT>>(&keypair.pk);
        keypair.vk = reserialize::<r1cs_mp_ppzkpcd_verification_key<PCD_ppT>>(&keypair.vk);
        pvk = reserialize::<r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT>>(&pvk);
        leave_block("Test serialization of keys", false);
    }

    let mut base_msg = tally_1.get_base_case_message(); /* we choose the base to always be tally_1 */
    nodes_in_layer /= max_arity;
    for layer in (0..=depth).rev() {
        for i in 0..nodes_in_layer {
            let cur_idx = (nodes_in_layer - 1) / (max_arity - 1) + i;

            let mut cur_tally = if tree_types[cur_idx] == 1 {
                tally_1.clone()
            } else {
                tally_2.clone()
            };
            let cur_cp = if tree_types[cur_idx] == 1 {
                cp_1.clone()
            } else {
                cp_2.clone()
            };

            let base_case = (max_arity * cur_idx + max_arity >= tree_size);

            let mut msgs = vec![base_msg.clone(); max_arity]; //Vec<RcCell<r1cs_pcd_message<FieldT> > > 
            let mut proofs = vec![r1cs_mp_ppzkpcd_proof::<PCD_ppT>::default(); max_arity]; //Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >

            if !base_case {
                for i in 0..max_arity {
                    msgs[i] = RcCell::new(tree_messages[max_arity * cur_idx + i + 1].clone());
                    proofs[i] = tree_proofs[max_arity * cur_idx + i + 1].clone();
                }
            }
            msgs.resize(
                tree_arity[i],
                RcCell::new(r1cs_pcd_message::<
                    <PCD_ppT as ppTConfig>::FieldT,
                    tally_pcd_message<<PCD_ppT as ppTConfig>::FieldT>,
                >::default()),
            );
            proofs.resize(tree_arity[i], r1cs_mp_ppzkpcd_proof::<PCD_ppT>::default());

            let mut ld = RcCell::new(tally_pcd_local_data::<FieldT<PCD_ppT>>::new(
                tree_elems[cur_idx],
            ));
            cur_tally.generate_r1cs_witness(&msgs, &ld);

            let tally_primary_input = r1cs_pcd_compliance_predicate_primary_input::<
                FieldT<PCD_ppT>,
                PCD_ppT::M,
            >::from(cur_tally.get_outgoing_message());
            let tally_auxiliary_input =
                r1cs_pcd_compliance_predicate_auxiliary_input::<
                    FieldT<PCD_ppT>,
                    PCD_ppT::M,
                    PCD_ppT::LD,
                >::new(msgs.clone(), ld, cur_tally.get_witness());

            println!("R1CS ppzkPCD Prover");
            let mut proof = r1cs_mp_ppzkpcd_prover::<PCD_ppT>(
                &keypair.pk,
                cur_cp.name,
                &tally_primary_input,
                &tally_auxiliary_input,
                &proofs,
            );

            if test_serialization {
                enter_block("Test serialization of proof", false);
                proof = reserialize::<r1cs_mp_ppzkpcd_proof<PCD_ppT>>(&proof);
                leave_block("Test serialization of proof", false);
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = cur_tally.get_outgoing_message().borrow().clone();

            println!("R1CS ppzkPCD Verifier");
            let pcd_verifier_input = r1cs_mp_ppzkpcd_primary_input::<PCD_ppT>::from(RcCell::new(
                tree_messages[cur_idx].clone(),
            ));
            let ans = r1cs_mp_ppzkpcd_verifier::<PCD_ppT>(
                &keypair.vk,
                &pcd_verifier_input,
                &tree_proofs[cur_idx],
            );

            println!("R1CS ppzkPCD Online Verifier");
            let ans2 = r1cs_mp_ppzkpcd_online_verifier::<PCD_ppT>(
                &pvk,
                &pcd_verifier_input,
                &tree_proofs[cur_idx],
            );
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..msgs.len() {
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
        nodes_in_layer /= max_arity;
    }

    leave_block("Call to run_r1cs_mp_ppzkpcd_tally_example", false);

    return all_accept;
}
