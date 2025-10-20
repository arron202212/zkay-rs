/** @file
 *****************************************************************************

 Declaration of functionality that runs the R1CS single-predicate ppzkPCD
 for a compliance predicate example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_SP_PPZKPCD_HPP_
// #define RUN_R1CS_SP_PPZKPCD_HPP_

use  <cstddef>



/**
 * Runs the single-predicate ppzkPCD (generator, prover, and verifier) for the
 * "tally compliance predicate", of a given wordsize, arity, and depth.
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
template<typename PCD_ppT>
bool run_r1cs_sp_ppzkpcd_tally_example(const size_t wordsize,
                                       const size_t arity,
                                       const size_t depth,
                                       const bool test_serialization);



use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/examples/run_r1cs_sp_ppzkpcd;

//#endif // RUN_R1CS_SP_PPZKPCD_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the R1CS single-predicate ppzkPCD
 for a compliance predicate example.

 See run_r1cs_sp_ppzkpcd.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_R1CS_SP_PPZKPCD_TCC_
// #define RUN_R1CS_SP_PPZKPCD_TCC_

use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/examples/tally_cp;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_sp_ppzkpcd/r1cs_sp_ppzkpcd;



template<typename PCD_ppT>
bool run_r1cs_sp_ppzkpcd_tally_example(const size_t wordsize,
                                       const size_t arity,
                                       const size_t depth,
                                       const bool test_serialization)
{
    ffec::enter_block("Call to run_r1cs_sp_ppzkpcd_tally_example");

    type ffec::Fr<typename PCD_ppT::curve_A_pp> FieldT;

    bool all_accept = true;

    ffec::enter_block("Generate all messages");
    size_t tree_size = 0;
    size_t nodes_in_layer = 1;
    for layer in 0..=depth
    {
        tree_size += nodes_in_layer;
        nodes_in_layer *= arity;
    }
    std::vector<size_t> tree_elems(tree_size);
    for i in 0..tree_size
    {
        tree_elems[i] = std::rand() % 10;
        print!("tree_elems[{}] = {}\n", i, tree_elems[i]);
    }
    ffec::leave_block("Generate all messages");

    std::vector<r1cs_sp_ppzkpcd_proof<PCD_ppT> > tree_proofs(tree_size);
    std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > tree_messages(tree_size);

    ffec::enter_block("Generate compliance predicate");
    const size_t type = 1;
    tally_cp_handler<FieldT> tally(type, arity, wordsize);
    tally.generate_r1cs_constraints();
    r1cs_pcd_compliance_predicate<FieldT> tally_cp = tally.get_compliance_predicate();
    ffec::leave_block("Generate compliance predicate");

    ffec::print_header("R1CS ppzkPCD Generator");
    r1cs_sp_ppzkpcd_keypair<PCD_ppT> keypair = r1cs_sp_ppzkpcd_generator<PCD_ppT>(tally_cp);

    ffec::print_header("Process verification key");
    r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> pvk = r1cs_sp_ppzkpcd_process_vk<PCD_ppT>(keypair.vk);

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize<r1cs_sp_ppzkpcd_proving_key<PCD_ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize<r1cs_sp_ppzkpcd_verification_key<PCD_ppT> >(keypair.vk);
        pvk = ffec::reserialize<r1cs_sp_ppzkpcd_processed_verification_key<PCD_ppT> >(pvk);
        ffec::leave_block("Test serialization of keys");
    }

    std::shared_ptr<r1cs_pcd_message<FieldT> > base_msg = tally.get_base_case_message();
    nodes_in_layer /= arity;
    for (long layer = depth; layer >= 0; --layer, nodes_in_layer /= arity)
    {
        for i in 0..nodes_in_layer
        {
            const size_t cur_idx = (nodes_in_layer - 1) / (arity - 1) + i;

            std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > msgs(arity, base_msg);
            std::vector<r1cs_sp_ppzkpcd_proof<PCD_ppT> > proofs(arity);

            const bool base_case = (arity * cur_idx + arity >= tree_size);

            if !base_case
            {
                for i in 0..arity
                {
                    msgs[i] = tree_messages[arity*cur_idx + i + 1];
                    proofs[i] = tree_proofs[arity*cur_idx + i + 1];
                }
            }

            std::shared_ptr<r1cs_pcd_local_data<FieldT> > ld;
            ld.reset(new tally_pcd_local_data<FieldT>(tree_elems[cur_idx]));
            tally.generate_r1cs_witness(msgs, ld);

            const r1cs_pcd_compliance_predicate_primary_input<FieldT> tally_primary_input(tally.get_outgoing_message());
            const r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> tally_auxiliary_input(msgs, ld, tally.get_witness());

            ffec::print_header("R1CS ppzkPCD Prover");
            r1cs_sp_ppzkpcd_proof<PCD_ppT> proof = r1cs_sp_ppzkpcd_prover<PCD_ppT>(keypair.pk, tally_primary_input, tally_auxiliary_input, proofs);

            if test_serialization
            {
                ffec::enter_block("Test serialization of proof");
                proof = ffec::reserialize<r1cs_sp_ppzkpcd_proof<PCD_ppT> >(proof);
                ffec::leave_block("Test serialization of proof");
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = tally.get_outgoing_message();

            ffec::print_header("R1CS ppzkPCD Verifier");
            const r1cs_sp_ppzkpcd_primary_input<PCD_ppT> pcd_verifier_input(tree_messages[cur_idx]);
            const bool ans = r1cs_sp_ppzkpcd_verifier<PCD_ppT>(keypair.vk, pcd_verifier_input, tree_proofs[cur_idx]);

            ffec::print_header("R1CS ppzkPCD Online Verifier");
            const bool ans2 = r1cs_sp_ppzkpcd_online_verifier<PCD_ppT>(pvk, pcd_verifier_input, tree_proofs[cur_idx]);
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..arity
            {
                print!("Message {} was:\n", i);
                msgs[i]->print();
            }
            print!("Summand at this node:\n{}\n", tree_elems[cur_idx]);
            print!("Outgoing message is:\n");
            tree_messages[cur_idx]->print();
            print!("\n");
            print!("Current node = {}. Current proof verifies = %s\n", cur_idx, if ans  {"YES" }else {"NO"});
            print!("\n\n\n ================================================================================\n\n\n");
        }
    }

    ffec::leave_block("Call to run_r1cs_sp_ppzkpcd_tally_example");

    return all_accept;
}



//#endif // RUN_R1CS_SP_PPZKPCD_TCC_
