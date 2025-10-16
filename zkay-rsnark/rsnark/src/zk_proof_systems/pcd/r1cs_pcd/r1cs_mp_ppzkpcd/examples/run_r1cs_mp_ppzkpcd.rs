/** @file
 *****************************************************************************

 Declaration of functionality that runs the R1CS multi-predicate ppzkPCD
 for a compliance predicate example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_MP_PPZKPCD_HPP_
// #define RUN_R1CS_MP_PPZKPCD_HPP_

use  <cstddef>



/**
 * Runs the multi-predicate ppzkPCD (generator, prover, and verifier) for the
 * "tally compliance predicate", of a given wordsize, arity, and depth.
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 *
 * Optionally, also test the case of compliance predicates with different types.
 */
template<typename PCD_ppT>
bool run_r1cs_mp_ppzkpcd_tally_example(const size_t wordsize,
                                       const size_t max_arity,
                                       const size_t depth,
                                       const bool test_serialization,
                                       const bool test_multi_type,
                                       const bool test_same_type_optimization);



use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/examples/run_r1cs_mp_ppzkpcd;

//#endif // RUN_R1CS_MP_PPZKPCD_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the R1CS multi-predicate ppzkPCD
 for a compliance predicate example.

 See run_r1cs_mp_ppzkpcd.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_MP_PPZKPCD_TCC_
// #define RUN_R1CS_MP_PPZKPCD_TCC_

use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/examples/tally_cp;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_mp_ppzkpcd/r1cs_mp_ppzkpcd;



template<typename PCD_ppT>
bool run_r1cs_mp_ppzkpcd_tally_example(const size_t wordsize,
                                       const size_t max_arity,
                                       const size_t depth,
                                       const bool test_serialization,
                                       const bool test_multi_type,
                                       const bool test_same_type_optimization)
{
    ffec::enter_block("Call to run_r1cs_mp_ppzkpcd_tally_example");

    type ffec::Fr<typename PCD_ppT::curve_A_pp> FieldT;

    bool all_accept = true;

    ffec::enter_block("Generate all messages");
    size_t tree_size = 0;
    size_t nodes_in_layer = 1;
    for layer in 0..=depth
    {
        tree_size += nodes_in_layer;
        nodes_in_layer *= max_arity;
    }

    std::vector<size_t> tree_types(tree_size);
    std::vector<size_t> tree_elems(tree_size);
    std::vector<size_t> tree_arity(tree_size);

    nodes_in_layer = 1;
    size_t node_idx = 0;
    for layer in 0..=depth
    {
        for (size_t id_in_layer = 0; id_in_layer < nodes_in_layer; ++id_in_layer, ++node_idx)
        {
            if !test_multi_type
            {
                tree_types[node_idx] = 1;
            }
            else
            {
                if test_same_type_optimization
                {
                    tree_types[node_idx] = 1 + ((depth-layer) & 1);
                }
                else
                {
                    tree_types[node_idx] = 1 + (std::rand() % 2);
                }
            }

            tree_elems[node_idx] = std::rand() % 100;
            tree_arity[node_idx] = 1 + (std::rand() % max_arity); /* we will just skip below this threshold */
            print!("tree_types[{}] = {}\n", node_idx, tree_types[node_idx]);
            print!("tree_elems[{}] = {}\n", node_idx, tree_elems[node_idx]);
            print!("tree_arity[{}] = {}\n", node_idx, tree_arity[node_idx]);

        }
        nodes_in_layer *= max_arity;
    }

    ffec::leave_block("Generate all messages");

    std::vector<r1cs_mp_ppzkpcd_proof<PCD_ppT> > tree_proofs(tree_size);
    std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > tree_messages(tree_size);

    ffec::enter_block("Generate compliance predicates");
    std::set<size_t> tally_1_accepted_types, tally_2_accepted_types;
    if test_same_type_optimization
    {
        if !test_multi_type
        {
            /* only tally 1 is going to be used */
            tally_1_accepted_types.insert(1);
        }
        else
        {
            tally_1_accepted_types.insert(2);
            tally_2_accepted_types.insert(1);
        }
    }

    tally_cp_handler<FieldT> tally_1(1, max_arity, wordsize, test_same_type_optimization, tally_1_accepted_types);
    tally_cp_handler<FieldT> tally_2(2, max_arity, wordsize, test_same_type_optimization, tally_2_accepted_types);
    tally_1.generate_r1cs_constraints();
    tally_2.generate_r1cs_constraints();
    r1cs_pcd_compliance_predicate<FieldT> cp_1 = tally_1.get_compliance_predicate();
    r1cs_pcd_compliance_predicate<FieldT> cp_2 = tally_2.get_compliance_predicate();
    ffec::leave_block("Generate compliance predicates");

    ffec::print_header("R1CS ppzkPCD Generator");
    r1cs_mp_ppzkpcd_keypair<PCD_ppT> keypair = r1cs_mp_ppzkpcd_generator<PCD_ppT>({ cp_1, cp_2 });

    ffec::print_header("Process verification key");
    r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> pvk = r1cs_mp_ppzkpcd_process_vk<PCD_ppT>(keypair.vk);

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize<r1cs_mp_ppzkpcd_proving_key<PCD_ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize<r1cs_mp_ppzkpcd_verification_key<PCD_ppT> >(keypair.vk);
        pvk = ffec::reserialize<r1cs_mp_ppzkpcd_processed_verification_key<PCD_ppT> >(pvk);
        ffec::leave_block("Test serialization of keys");
    }

    std::shared_ptr<r1cs_pcd_message<FieldT> > base_msg = tally_1.get_base_case_message(); /* we choose the base to always be tally_1 */
    nodes_in_layer /= max_arity;
    for (long layer = depth; layer >= 0; --layer, nodes_in_layer /= max_arity)
    {
        for i in 0..nodes_in_layer
        {
            const size_t cur_idx = (nodes_in_layer - 1) / (max_arity - 1) + i;

            tally_cp_handler<FieldT> &cur_tally = (tree_types[cur_idx] == 1 ? tally_1 : tally_2);
            r1cs_pcd_compliance_predicate<FieldT> &cur_cp = (tree_types[cur_idx] == 1 ? cp_1 : cp_2);

            const bool base_case = (max_arity * cur_idx + max_arity >= tree_size);

            std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > msgs(max_arity, base_msg);
            std::vector<r1cs_mp_ppzkpcd_proof<PCD_ppT> > proofs(max_arity);

            if !base_case
            {
                for i in 0..max_arity
                {
                    msgs[i] = tree_messages[max_arity*cur_idx + i + 1];
                    proofs[i] = tree_proofs[max_arity*cur_idx + i + 1];
                }
            }
            msgs.resize(tree_arity[i]);
            proofs.resize(tree_arity[i]);

            std::shared_ptr<r1cs_pcd_local_data<FieldT> > ld;
            ld.reset(new tally_pcd_local_data<FieldT>(tree_elems[cur_idx]));
            cur_tally.generate_r1cs_witness(msgs, ld);

            const r1cs_pcd_compliance_predicate_primary_input<FieldT> tally_primary_input(cur_tally.get_outgoing_message());
            const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> tally_auxiliary_input(msgs, ld, cur_tally.get_witness());

            ffec::print_header("R1CS ppzkPCD Prover");
            r1cs_mp_ppzkpcd_proof<PCD_ppT> proof = r1cs_mp_ppzkpcd_prover<PCD_ppT>(keypair.pk, cur_cp.name, tally_primary_input, tally_auxiliary_input, proofs);

            if test_serialization
            {
                ffec::enter_block("Test serialization of proof");
                proof = ffec::reserialize<r1cs_mp_ppzkpcd_proof<PCD_ppT> >(proof);
                ffec::leave_block("Test serialization of proof");
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = cur_tally.get_outgoing_message();

            ffec::print_header("R1CS ppzkPCD Verifier");
            const r1cs_mp_ppzkpcd_primary_input<PCD_ppT> pcd_verifier_input(tree_messages[cur_idx]);
            const bool ans = r1cs_mp_ppzkpcd_verifier<PCD_ppT>(keypair.vk, pcd_verifier_input, tree_proofs[cur_idx]);

            ffec::print_header("R1CS ppzkPCD Online Verifier");
            const bool ans2 = r1cs_mp_ppzkpcd_online_verifier<PCD_ppT>(pvk, pcd_verifier_input, tree_proofs[cur_idx]);
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..msgs.size()
            {
                print!("Message {} was:\n", i);
                msgs[i]->print();
            }
            print!("Summand at this node:\n{}\n", tree_elems[cur_idx]);
            print!("Outgoing message is:\n");
            tree_messages[cur_idx]->print();
            print!("\n");
            print!("Current node = {}. Current proof verifies = %s\n", cur_idx, ans ? "YES" : "NO");
            print!("\n\n\n ================================================================================\n\n\n");
        }
    }

    ffec::leave_block("Call to run_r1cs_mp_ppzkpcd_tally_example");

    return all_accept;
}



//#endif // RUN_R1CS_MP_PPZKPCD_TCC_
