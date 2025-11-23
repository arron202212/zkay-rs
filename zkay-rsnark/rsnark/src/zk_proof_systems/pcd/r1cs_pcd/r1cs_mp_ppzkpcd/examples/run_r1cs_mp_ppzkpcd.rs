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

// use  <cstddef>



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



// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::examples::run_r1cs_mp_ppzkpcd;

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

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::examples::tally_cp;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd;



// 
 pub fn run_r1cs_mp_ppzkpcd_tally_example<PCD_ppT>(wordsize:usize,
                                       max_arity:usize,
                                       depth:usize,
                                       test_serialization:bool,
                                       test_multi_type:bool,
                                       test_same_type_optimization:bool)->bool
{
    ffec::enter_block("Call to run_r1cs_mp_ppzkpcd_tally_example");

    type FieldT=ffec::Fr;//< PCD_ppT::curve_A_pp> ;

    let mut  all_accept = true;

    ffec::enter_block("Generate all messages");
    let mut  tree_size = 0;
    let mut  nodes_in_layer = 1;
    for layer in 0..=depth
    {
        tree_size += nodes_in_layer;
        nodes_in_layer *= max_arity;
    }

    let mut  tree_types=vec![0;tree_size];
    let mut tree_elems=vec![0;tree_size];
    let mut  tree_arity=vec![0;tree_size];

     nodes_in_layer = 1;
    let mut  node_idx = 0;
    for layer in 0..=depth
    {
        for  id_in_layer in  0.. nodes_in_layer
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
                    tree_types[node_idx] = 1 + (rand::random() % 2);
                }
            }

            tree_elems[node_idx] = rand::random() % 100;
            tree_arity[node_idx] = 1 + (rand::random() % max_arity); /* we will just skip below this threshold */
            print!("tree_types[{}] = {}\n", node_idx, tree_types[node_idx]);
            print!("tree_elems[{}] = {}\n", node_idx, tree_elems[node_idx]);
            print!("tree_arity[{}] = {}\n", node_idx, tree_arity[node_idx]);
            node_idx+=1;

        }
        nodes_in_layer *= max_arity;
    }

    ffec::leave_block("Generate all messages");

    let mut  tree_proofs=vec![r1cs_mp_ppzkpcd_proof::<PCD_ppT>::new();tree_size];//Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >
    let mut tree_messages=vec![r1cs_pcd_message::<PCD_ppT>::new();tree_size];

    ffec::enter_block("Generate compliance predicates");
    let ( mut tally_1_accepted_types, mut tally_2_accepted_types)=(BTreeSet::new(),BTreeSet::new());
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

    let mut  tally_1=tally_cp_handler::<FieldT>::new(1, max_arity, wordsize, test_same_type_optimization, tally_1_accepted_types);
   let mut tally_2=tally_cp_handler::<FieldT>::new(2, max_arity, wordsize, test_same_type_optimization, tally_2_accepted_types);
    tally_1.generate_r1cs_constraints();
    tally_2.generate_r1cs_constraints();
    let mut  cp_1 = tally_1.get_compliance_predicate();
    let mut  cp_2 = tally_2.get_compliance_predicate();
    ffec::leave_block("Generate compliance predicates");

    ffec::print_header("R1CS ppzkPCD Generator");
    let mut  keypair = r1cs_mp_ppzkpcd_generator::<PCD_ppT>( [cp_1, cp_2] );

    ffec::print_header("Process verification key");
    let mut  pvk = r1cs_mp_ppzkpcd_process_vk::<PCD_ppT>(keypair.vk);

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize::<r1cs_mp_ppzkpcd_proving_key::<PCD_ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize::<r1cs_mp_ppzkpcd_verification_key::<PCD_ppT> >(keypair.vk);
        pvk = ffec::reserialize::<r1cs_mp_ppzkpcd_processed_verification_key::<PCD_ppT> >(pvk);
        ffec::leave_block("Test serialization of keys");
    }

    let mut  base_msg = tally_1.get_base_case_message(); /* we choose the base to always be tally_1 */
    nodes_in_layer /= max_arity;
    for layer in  (0..=depth).rev()
    {
        for i in 0..nodes_in_layer
        {
            let  cur_idx = (nodes_in_layer - 1) / (max_arity - 1) + i;

            let cur_tally = if tree_types[cur_idx] == 1 {tally_1} else{tally_2};
            let cur_cp = if tree_types[cur_idx] == 1 {cp_1} else{cp_2};

            let  base_case = (max_arity * cur_idx + max_arity >= tree_size);

            let mut msgs=vec![base_msg;max_arity];//Vec<RcCell<r1cs_pcd_message<FieldT> > > 
            let mut  proofs=vec![r1cs_mp_ppzkpcd_proof::<PCD_ppT>::new();max_arity];//Vec<r1cs_mp_ppzkpcd_proof<PCD_ppT> >

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

            let mut ld=r1cs_pcd_local_data::<FieldT>::new();
            ld.reset(tally_pcd_local_data::<FieldT>::new(tree_elems[cur_idx]));
            cur_tally.generate_r1cs_witness(msgs, ld);

            let   tally_primary_input=r1cs_pcd_compliance_predicate_primary_input::<FieldT>::new(cur_tally.get_outgoing_message());
            let  tally_auxiliary_input=r1cs_pcd_compliance_predicate_auxiliary_input::<FieldT>::new(msgs, ld, cur_tally.get_witness());

            ffec::print_header("R1CS ppzkPCD Prover");
            let  mut proof = r1cs_mp_ppzkpcd_prover::<PCD_ppT>(keypair.pk, cur_cp.name, tally_primary_input, tally_auxiliary_input, proofs);

            if test_serialization
            {
                ffec::enter_block("Test serialization of proof");
                proof = ffec::reserialize::<r1cs_mp_ppzkpcd_proof::<PCD_ppT> >(proof);
                ffec::leave_block("Test serialization of proof");
            }

            tree_proofs[cur_idx] = proof;
            tree_messages[cur_idx] = cur_tally.get_outgoing_message();

            ffec::print_header("R1CS ppzkPCD Verifier");
            let   pcd_verifier_input=r1cs_mp_ppzkpcd_primary_input::<PCD_ppT>(tree_messages[cur_idx]);
            let  ans = r1cs_mp_ppzkpcd_verifier::<PCD_ppT>(keypair.vk, pcd_verifier_input, tree_proofs[cur_idx]);

            ffec::print_header("R1CS ppzkPCD Online Verifier");
            let  ans2 = r1cs_mp_ppzkpcd_online_verifier::<PCD_ppT>(pvk, pcd_verifier_input, tree_proofs[cur_idx]);
            assert!(ans == ans2);

            all_accept = all_accept && ans;

            print!("\n");
            for i in 0..msgs.len()
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
        nodes_in_layer /= max_arity;
    }

    ffec::leave_block("Call to run_r1cs_mp_ppzkpcd_tally_example");

    return all_accept;
}



//#endif // RUN_R1CS_MP_PPZKPCD_TCC_
