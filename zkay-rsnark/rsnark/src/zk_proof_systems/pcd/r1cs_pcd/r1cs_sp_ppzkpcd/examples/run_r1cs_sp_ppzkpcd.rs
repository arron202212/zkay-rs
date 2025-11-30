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

// use  <cstddef>



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



// use crate::zk_proof_systems::pcd::r1cs_sp_ppzkpcd::examples::run_r1cs_sp_ppzkpcd;

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

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd;



// 
 pub fn run_r1cs_sp_ppzkpcd_tally_example<PCD_ppT>(wordsize:usize,
                                       arity:usize,
                                       depth:usize,
                                       test_serialization:bool)->bool
{
    ffec::enter_block("Call to run_r1cs_sp_ppzkpcd_tally_example");

    // type FieldT=ffec::Fr< PCD_ppT::curve_A_pp> ;

    let mut  all_accept = true;

    ffec::enter_block("Generate all messages");
    let mut  tree_size = 0;
    let mut  nodes_in_layer = 1;
    for layer in 0..=depth
    {
        tree_size += nodes_in_layer;
        nodes_in_layer *= arity;
    }
    let mut  tree_elems=vec![0;tree_size];
    for i in 0..tree_size
    {
        tree_elems[i] = rand::random() % 10;
        print!("tree_elems[{}] = {}\n", i, tree_elems[i]);
    }
    ffec::leave_block("Generate all messages");

    let mut tree_proofs=vec![r1cs_sp_ppzkpcd_proof::<PCD_ppT> ::new();tree_size];
    let mut  tree_messages=vec![r1cs_pcd_message::<PCD_ppT> ::new();tree_size];

    ffec::enter_block("Generate compliance predicate");
    let types = 1;
    let mut  tally=tally_cp_handler::<FieldT>::new(types, arity, wordsize);
    tally.generate_r1cs_constraints();
    let mut tally_cp = tally.get_compliance_predicate();
    ffec::leave_block("Generate compliance predicate");

    ffec::print_header("R1CS ppzkPCD Generator");
    let mut keypair = r1cs_sp_ppzkpcd_generator::<PCD_ppT>(tally_cp);

    ffec::print_header("Process verification key");
   let mut  pvk = r1cs_sp_ppzkpcd_process_vk::<PCD_ppT>(keypair.vk);

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize::<r1cs_sp_ppzkpcd_proving_key::<PCD_ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize::<r1cs_sp_ppzkpcd_verification_key::<PCD_ppT> >(keypair.vk);
        pvk = ffec::reserialize::<r1cs_sp_ppzkpcd_processed_verification_key::<PCD_ppT> >(pvk);
        ffec::leave_block("Test serialization of keys");
    }

    let mut  base_msg = tally.get_base_case_message();
    nodes_in_layer /= arity;
    for  layer in  (0..=depth).rev()
    {
        for i in 0..nodes_in_layer
        {
            let cur_idx = (nodes_in_layer - 1) / (arity - 1) + i;

            let mut  msgs=vec![base_msg;arity];
            let mut  proofs=vec![r1cs_sp_ppzkpcd_proof::<PCD_ppT>::new();arity];

            let base_case = (arity * cur_idx + arity >= tree_size);

            if !base_case
            {
                for i in 0..arity
                {
                    msgs[i] = tree_messages[arity*cur_idx + i + 1];
                    proofs[i] = tree_proofs[arity*cur_idx + i + 1];
                }
            }

            let mut ld=r1cs_pcd_local_data::<FieldT>::new();
            ld=RcCell::new(tally_pcd_local_data::<FieldT>::new(tree_elems[cur_idx]));
            tally.generate_r1cs_witness(msgs, ld);

            let mut   tally_primary_input=r1cs_pcd_compliance_predicate_primary_input::<FieldT>::new()(tally.get_outgoing_message());
            let mut   tally_auxiliary_input=r1cs_pcd_compliance_predicate_auxiliary_input::<FieldT>::new()(msgs, ld, tally.get_witness());

            ffec::print_header("R1CS ppzkPCD Prover");
            let  proof = r1cs_sp_ppzkpcd_prover::<PCD_ppT>(keypair.pk, tally_primary_input, tally_auxiliary_input, proofs);

            if test_serialization
            {
                ffec::enter_block("Test serialization of proof");
                proof = ffec::reserialize::<r1cs_sp_ppzkpcd_proof::<PCD_ppT> >(proof);
                ffec::leave_block("Test serialization of proof");
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = tally.get_outgoing_message();

            ffec::print_header("R1CS ppzkPCD Verifier");
            let   pcd_verifier_input=r1cs_sp_ppzkpcd_primary_input::<PCD_ppT>::new(tree_messages[cur_idx]);
            let ans = r1cs_sp_ppzkpcd_verifier::<PCD_ppT>(keypair.vk, pcd_verifier_input, tree_proofs[cur_idx]);

            ffec::print_header("R1CS ppzkPCD Online Verifier");
            let ans2 = r1cs_sp_ppzkpcd_online_verifier::<PCD_ppT>(pvk, pcd_verifier_input, tree_proofs[cur_idx]);
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..arity
            {
                print!("Message {} was:\n", i);
                msgs[i].print();
            }
            print!("Summand at this node:\n{}\n", tree_elems[cur_idx]);
            print!("Outgoing message is:\n");
            tree_messages[cur_idx].print();
            print!("\n");
            print!("Current node = {}. Current proof verifies = {}\n", cur_idx, if ans  {"YES" }else {"NO"});
            print!("\n\n\n ================================================================================\n\n\n");
        }
        nodes_in_layer /= arity;
    }

    ffec::leave_block("Call to run_r1cs_sp_ppzkpcd_tally_example");

    return all_accept;
}



//#endif // RUN_R1CS_SP_PPZKPCD_TCC_
