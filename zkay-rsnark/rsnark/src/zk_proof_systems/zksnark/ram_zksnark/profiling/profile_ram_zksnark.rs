/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
// use boost/program_options;
use ffec::common::profiling;

use crate::common::default_types::ram_zksnark_pp;
use crate::relations::ram_computations::memory::examples::memory_contents_examples;
use crate::relations::ram_computations::rams::examples::ram_examples;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::zksnark::ram_zksnark::examples::run_ram_zksnark;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark;

pub fn simulate_random_memory_contents<FieldT>(
    ap: tinyram_architecture_params,
    input_size: usize,
    program_size: usize,
) {
    let num_addresses = 1u64 << ap.dwaddr_len();
    let value_size = 2 * ap.w;
    let init_random = random_memory_contents(
        num_addresses,
        value_size,
        program_size + (input_size + 1) / 2,
    );

    ffec::enter_block("Initialize random delegated memory");
    let dm_random = delegated_ra_memory::<FieldT>::new(num_addresses, value_size, init_random);
    ffec::leave_block("Initialize random delegated memory");
}

pub fn profile_ram_zksnark_verifier<ppT>(
    ap: tinyram_architecture_params,
    input_size: usize,
    program_size: usize,
) {
    // type ramT=ram_zksnark_machine_pp<ppT> ;
    let time_bound = 10;

    let boot_trace_size_bound = program_size + input_size;
    let example = gen_ram_example_complex::<ramT>(ap, boot_trace_size_bound, time_bound, true);

    let mut pi = ram_zksnark_proof::<ppT>::new();
    let vk = ram_zksnark_verification_key::<ppT>::dummy_verification_key(ap);

    ffec::enter_block("Verify fake proof");
    ram_zksnark_verifier::<ppT>(vk, example.boot_trace, time_bound, pi);
    ffec::leave_block("Verify fake proof");
}

pub fn print_ram_zksnark_verifier_profiling<ppT>() {
    // ffec::inhibit_profiling_info = true;
    for w in &[16, 32] {
        let k = 16;

        for input_size in &[0, 10, 100] {
            let mut program_size = 10;
            while program_size <= 10000 {
                let ap = tinyram_architecture_params::new(w, k);

                profile_ram_zksnark_verifier::<ppT>(ap, input_size, program_size);

                let input_map = ffec::last_times["Call to ram_zksnark_verifier_input_map"];
                let preprocessing = ffec::last_times["Call to r1cs_ppzksnark_verifier_process_vk"];
                let accumulate = ffec::last_times["Call to r1cs_ppzksnark_IC_query::accumulate"];
                let pairings = ffec::last_times["Online pairing computations"];
                let total = ffec::last_times["Call to ram_zksnark_verifier"];
                let rest = total - (input_map + preprocessing + accumulate + pairings);

                let delegated_ra_memory_init =
                    ffec::last_times["Construct delegated_ra_memory from memory map"];
                simulate_random_memory_contents::<ffec::Fr<ppT::curve_A_pp>>(
                    ap,
                    input_size,
                    program_size,
                );
                let delegated_ra_memory_init_random =
                    ffec::last_times["Initialize random delegated memory"];
                let input_map_random =
                    input_map - delegated_ra_memory_init + delegated_ra_memory_init_random;
                let total_random =
                    total - delegated_ra_memory_init + delegated_ra_memory_init_random;

                print!(
                    "w = {}, k = {}, program_size = {}, input_size = {}, input_map = {:.2}ms, preprocessing = {:.2}ms, accumulate = {:.2}ms, pairings = {:.2}ms, rest = {:.2}ms, total = {:.2}ms (input_map_random = {:.2}ms, total_random = {:.2}ms)\n",
                    w,
                    k,
                    program_size,
                    input_size,
                    input_map * 1e-6,
                    preprocessing * 1e-6,
                    accumulate * 1e-6,
                    pairings * 1e-6,
                    rest * 1e-6,
                    total * 1e-6,
                    input_map_random * 1e-6,
                    total_random * 1e-6
                );
                program_size *= 10;
            }
        }
    }
}

pub fn profile_ram_zksnark<ppT>(
    ap: tinyram_architecture_params,
    program_size: usize,
    input_size: usize,
    time_bound: usize,
) {
    // type ramT=ram_zksnark_machine_pp<ppT> ;

    let boot_trace_size_bound = program_size + input_size;
    let example = gen_ram_example_complex::<ramT>(ap, boot_trace_size_bound, time_bound, true);
    let test_serialization = true;
    let bit = run_ram_zksnark::<ppT>(example, test_serialization);
    assert!(bit);
}

// namespace po = boost::program_options;

// bool process_command_line(argc:int, argv:char**,
//                           bool &profile_gp,
//                           usize &w,
//                           usize &k,
//                           bool &profile_v,
//                           usize &l)
// {
//     try
//     {
//         po::options_description desc("Usage");
//         desc.add_options()
//             ("help", "print this help message")
//             ("profile_gp", "profile generator and prover")
//             ("w", po::value<usize>(&w)->default_value(16), "word size")
//             ("k", po::value<usize>(&k)->default_value(16), "register count")
//             ("profile_v", "profile verifier")
//             ("v", "print version info")
//             ("l", po::value<usize>(&l)->default_value(10), "program length");

//         po::variables_map vm;
//         po::store(po::parse_command_line(argc, argv, desc), vm);

//         if vm.count("v")
//         {
//             ffec::print_compilation_info();
//             exit(0);
//         }

//         if vm.count("help")
//         {
//             std::cout << desc << "\n";
//             return false;
//         }

//         profile_gp = vm.count("profile_gp");
//         profile_v = vm.count("profile_v");

//         if !(vm.count("profile_gp") ^ vm.count("profile_v"))
//         {
//             std::cout << "Must choose between profiling generator/prover and profiling verifier (see --help)\n";
//             return false;
//         }

//         po::notify(vm);
//     }
//     catch(std::exception& e)
//     {
//         std::cerr << "Error: " << e.what() << "\n";
//         return false;
//     }

//     return true;
// }

// int main(int argc, const char* argv[])
// {
//     ffec::start_profiling();
//     ram_zksnark_PCD_pp<default_ram_zksnark_pp>::init_public_params();

//     bool profile_gp;
//     usize w;
//     usize k;
//     bool profile_v;
//     usize l;

//     if !process_command_line(argc, argv, profile_gp, w, k, profile_v, l)
//     {
//         return 1;
//     }

//     tinyram_architecture_params ap(w, k);

//     if profile_gp
//     {
//         profile_ram_zksnark<default_ram_zksnark_pp>(ap, 100, 100, 10); // w, k, l, n, T
//     }

//     if profile_v
//     {
//         profile_ram_zksnark_verifier<default_ram_zksnark_pp>(ap, l/2, l/2);
//     }
// }
