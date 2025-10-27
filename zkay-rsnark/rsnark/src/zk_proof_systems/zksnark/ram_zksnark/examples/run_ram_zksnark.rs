/** @file
 *****************************************************************************

 Declaration of functionality that runs the RAM zkSNARK for
 a given RAM example.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_RAM_ZKSNARK_HPP_
// #define RUN_RAM_ZKSNARK_HPP_

use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params;



/**
 * Runs the zkSNARK (generator, prover, and verifier) for a given
 * RAM example (specified by an architecture, boot trace, auxiliary input, and time bound).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
// 
// bool run_ram_zksnark(example:&ram_example<ram_zksnark_machine_pp<ram_zksnark_ppT> >,
//                      test_serialization:bool);



use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;

//#endif // RUN_RAM_ZKSNARK_HPP_
/** @file
 *****************************************************************************

 Implementation of functionality that runs the RAM zkSNARK for
 a given RAM example.

 See run_ram_zksnark.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RUN_RAM_ZKSNARK_TCC_
// #define RUN_RAM_ZKSNARK_TCC_

// use  <sstream>

use ffec::common::profiling;

use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark;



/**
 * The code below provides an example of all stages of running a RAM zkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the zkSNARK generator on input a given
 *     architecture.
 * (2) The "prover", which runs the zkSNARK prover on input the proving key,
 *     a boot trace, and an auxiliary input.
 * (3) The "verifier", which runs the zkSNARK verifier on input the verification key,
 *     a boot trace, a time bound, and a proof.
 */
// 
 pub fn run_ram_zksnark<ram_zksnark_ppT>(example:&ram_example<ram_zksnark_machine_pp<ram_zksnark_ppT> > ,
                      test_serialization:bool)->bool
{
    ffec::enter_block("Call to run_ram_zksnark");

    print!("This run uses an example with the following parameters:\n");
    example.ap.print();
    print!("* Time bound (T): {}\n", example.time_bound);

    ffec::print_header("RAM zkSNARK Generator");
    let mut  keypair = ram_zksnark_generator::<ram_zksnark_ppT>(example.ap);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after generator");

    if test_serialization
    {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize::<ram_zksnark_proving_key::<ram_zksnark_ppT> >(keypair.pk);
        keypair.vk = ffec::reserialize::<ram_zksnark_verification_key::<ram_zksnark_ppT> >(keypair.vk);
        ffec::leave_block("Test serialization of keys");
    }

    ffec::print_header("RAM zkSNARK Prover");
    let  proof = ram_zksnark_prover::<ram_zksnark_ppT>(keypair.pk, example.boot_trace, example.time_bound, example.auxiliary_input);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after prover");

    if test_serialization
    {
        ffec::enter_block("Test serialization of proof");
        proof = ffec::reserialize::<ram_zksnark_proof::<ram_zksnark_ppT> >(proof);
        ffec::leave_block("Test serialization of proof");
    }

    ffec::print_header("RAM zkSNARK Verifier");
    let  ans = ram_zksnark_verifier::<ram_zksnark_ppT>(keypair.vk, example.boot_trace, example.time_bound, proof);
    print!("\n"); ffec::print_indent(); ffec::print_mem("after verifier");
    print!("* The verification result is: {}\n", if ans {"PASS"} else{"FAIL"});

    ffec::leave_block("Call to run_ram_zksnark");

    return ans;
}



//#endif // RUN_RAM_ZKSNARK_TCC_
