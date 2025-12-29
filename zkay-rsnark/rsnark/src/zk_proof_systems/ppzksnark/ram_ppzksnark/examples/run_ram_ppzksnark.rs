/** @file
*****************************************************************************

Declaration of functionality that runs the RAM ppzkSNARK for
a given RAM example.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RUN_RAM_PPZKSNARK_HPP_
// #define RUN_RAM_PPZKSNARK_HPP_
use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params;

/**
 * Runs the ppzkSNARK (generator, prover, and verifier) for a given
 * RAM example (specified by an architecture, boot trace, auxiliary input, and time bound).
 *
 * Optionally, also test the serialization routines for keys and proofs.
 * (This takes additional time.)
 */
//
// bool run_ram_ppzksnark(example:&ram_example<ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> >,
//                        test_serialization:bool);
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::examples::run_ram_ppzksnark;

//#endif // RUN_RAM_PPZKSNARK_HPP_
/** @file
*****************************************************************************

Implementation of functionality that runs the RAM ppzkSNARK for
a given RAM example.

See run_ram_ppzksnark.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RUN_RAM_PPZKSNARK_TCC_
// #define RUN_RAM_PPZKSNARK_TCC_

// use  <sstream>
use ffec::common::profiling;

use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;

/**
 * The code below provides an example of all stages of running a RAM ppzkSNARK.
 *
 * Of course, in a real-life scenario, we would have three distinct entities,
 * mangled into one in the demonstration below. The three entities are as follows.
 * (1) The "generator", which runs the ppzkSNARK generator on input a given
 *     architecture and bounds on the computation.
 * (2) The "prover", which runs the ppzkSNARK prover on input the proving key,
 *     a boot trace, and an auxiliary input.
 * (3) The "verifier", which runs the ppzkSNARK verifier on input the verification key,
 *     a boot trace, and a proof.
 */
//
pub fn run_ram_ppzksnark<ram_ppzksnark_ppT>(
    example: &ram_example<ram_ppzksnark_machine_pp<ram_ppzksnark_ppT>>,
    test_serialization: bool,
) -> bool {
    ffec::enter_block("Call to run_ram_ppzksnark");

    print!("This run uses an example with the following parameters:\n");
    example.ap.print();
    print!(
        "* Primary input size bound (L): {}\n",
        example.boot_trace_size_bound
    );
    print!("* Time bound (T): {}\n", example.time_bound);
    print!(
        "Hence, ffec::log2(L+2*T) equals {}\n",
        ffec::log2(example.boot_trace_size_bound + 2 * example.time_bound)
    );

    ffec::print_header("RAM ppzkSNARK Generator");
    let keypair = ram_ppzksnark_generator::<ram_ppzksnark_ppT>(
        example.ap,
        example.boot_trace_size_bound,
        example.time_bound,
    );
    print!("\n");
    ffec::print_indent();
    ffec::print_mem("after generator");

    if test_serialization {
        ffec::enter_block("Test serialization of keys");
        keypair.pk = ffec::reserialize::<ram_ppzksnark_proving_key<ram_ppzksnark_ppT>>(keypair.pk);
        keypair.vk =
            ffec::reserialize::<ram_ppzksnark_verification_key<ram_ppzksnark_ppT>>(keypair.vk);
        ffec::leave_block("Test serialization of keys");
    }

    ffec::print_header("RAM ppzkSNARK Prover");
    let proof = ram_ppzksnark_prover::<ram_ppzksnark_ppT>(
        keypair.pk,
        example.boot_trace,
        example.auxiliary_input,
    );
    print!("\n");
    ffec::print_indent();
    ffec::print_mem("after prover");

    if test_serialization {
        ffec::enter_block("Test serialization of proof");
        proof = ffec::reserialize::<ram_ppzksnark_proof<ram_ppzksnark_ppT>>(proof);
        ffec::leave_block("Test serialization of proof");
    }

    ffec::print_header("RAM ppzkSNARK Verifier");
    let ans = ram_ppzksnark_verifier::<ram_ppzksnark_ppT>(keypair.vk, example.boot_trace, proof);
    print!("\n");
    ffec::print_indent();
    ffec::print_mem("after verifier");
    print!(
        "* The verification result is: {}\n",
        if ans { "PASS" } else { "FAIL" }
    );

    ffec::leave_block("Call to run_ram_ppzksnark");

    return ans;
}

//#endif // RUN_RAM_PPZKSNARK_TCC_
