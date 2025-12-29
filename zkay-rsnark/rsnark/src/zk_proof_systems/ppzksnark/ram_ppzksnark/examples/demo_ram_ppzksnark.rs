/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
// use  <algorithm>
// use  <fstream>
// use  <iostream>
// use  <sstream>
// use  <string>
//#ifndef MINDEPS
// use boost/program_options;
//#endif
use ffec::common::profiling;

use crate::common::default_types::tinyram_ppzksnark_pp;
use crate::reductions::ram_to_r1cs::ram_to_r1cs;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;

//#ifndef MINDEPS
// // namespace po = boost::program_options;

// bool process_demo_command_line(argc:int, argv:char**,
//                                String &assembly_fn,
//                                String &processed_assembly_fn,
//                                String &architecture_params_fn,
//                                String &computation_bounds_fn,
//                                String &primary_input_fn,
//                                String &auxiliary_input_fn)
// {
//     try
//     {
//         po::options_description desc("Usage");
//         desc.add_options()
//             ("help", "print this help message")
//             ("assembly", po::value<String>(&assembly_fn)->required())
//             ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
//             ("architecture_params", po::value<String>(&architecture_params_fn)->required())
//             ("computation_bounds", po::value<String>(&computation_bounds_fn)->required())
//             ("primary_input", po::value<String>(&primary_input_fn)->required())
//             ("auxiliary_input", po::value<String>(&auxiliary_input_fn)->required());

//         po::variables_map vm;
//         po::store(po::parse_command_line(argc, argv, desc), vm);

//         if vm.count("help")
//         {
//             std::cout << desc << "\n";
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
//#endif

// int main(int argc, const char * argv[])
// {
//     default_tinyram_ppzksnark_pp::init_public_params();
// // #ifdef MINDEPS
//     String assembly_fn = "assembly.s";
//     String processed_assembly_fn = "processed.txt";
//     String architecture_params_fn = "architecture_params.txt";
//     String computation_bounds_fn = "computation_bounds.txt";
//     String primary_input_fn = "primary_input.txt";
//     String auxiliary_input_fn = "auxiliary_input.txt";
// #else
//     String assembly_fn;
//     String processed_assembly_fn;
//     String architecture_params_fn;
//     String computation_bounds_fn;
//     String primary_input_fn;
//     String auxiliary_input_fn;

//     if (!process_demo_command_line(argc, argv, assembly_fn, processed_assembly_fn, architecture_params_fn,
//                                    computation_bounds_fn, primary_input_fn, auxiliary_input_fn))
//     {
//         return 1;
//     }
// //#endif
//     ffec::start_profiling();

//     print!("================================================================================\n");
//     print!("TinyRAM example loader\n");
//     print!("================================================================================\n\n");

//     /* load everything */
//     ram_ppzksnark_architecture_params<default_tinyram_ppzksnark_pp> ap;
//     std::ifstream f_ap(architecture_params_fn);
//     f_ap >> ap;

//     print!("Will run on {} register machine (word size = {})\n", ap.k, ap.w);

//     std::ifstream f_rp(computation_bounds_fn);
//     usize tinyram_input_size_bound, tinyram_program_size_bound, time_bound;
//     f_rp >> tinyram_input_size_bound >> tinyram_program_size_bound >> time_bound;

//     std::ifstream processed(processed_assembly_fn);
//     std::ifstream raw(assembly_fn);
//     tinyram_program program = load_preprocessed_program(ap, processed);

//     print!("Program:\n{}\n", String((std::istreambuf_iterator<char>(raw)),
//                                          std::istreambuf_iterator<char>()));

//     std::ifstream f_primary_input(primary_input_fn);
//     std::ifstream f_auxiliary_input(auxiliary_input_fn);

//     ffec::enter_block("Loading primary input");
//     tinyram_input_tape primary_input = load_tape(f_primary_input);
//     ffec::leave_block("Loading primary input");

//     ffec::enter_block("Loading auxiliary input");
//     tinyram_input_tape auxiliary_input = load_tape(f_auxiliary_input);
//     ffec::leave_block("Loading auxiliary input");

//     print!("\nPress enter to continue.\n");
//     std::cin.get();

//     let boot_trace_size_bound = tinyram_program_size_bound + tinyram_input_size_bound;
//     boot_trace_size_bound:ram_boot_trace<default_tinyram_ppzksnark_pp> boot_trace = tinyram_boot_trace_from_program_and_input(ap,, program, primary_input);

//     print!("================================================================================\n");
//     print!("TinyRAM arithmetization test for T = {} time steps\n", time_bound);
//     print!("================================================================================\n\n");

//     type default_ram=ram_ppzksnark_machine_pp<default_tinyram_ppzksnark_pp>;
//     type FieldT=ram_base_field<default_ram>;

//     ram_to_r1cs<default_ram> r(ap, boot_trace_size_bound, time_bound);
//     r.instance_map();

//     boot_trace_size_bound:r1cs_primary_input<FieldT> r1cs_primary_input = ram_to_r1cs<default_ram>::primary_input_map(ap,, boot_trace);
//     const r1cs_auxiliary_input<FieldT> r1cs_auxiliary_input = r.auxiliary_input_map(boot_trace, auxiliary_input);
//     const r1cs_constraint_system<FieldT> constraint_system = r.get_constraint_system();

//     r.print_execution_trace();
//     assert!(constraint_system.is_satisfied(r1cs_primary_input, r1cs_auxiliary_input));

//     print!("\nPress enter to continue.\n");
//     std::cin.get();

//     print!("================================================================================\n");
//     print!("TinyRAM ppzkSNARK Key Pair Generator\n");
//     print!("================================================================================\n\n");
//     boot_trace_size_bound:ram_ppzksnark_keypair<default_tinyram_ppzksnark_pp> keypair = ram_ppzksnark_generator<default_tinyram_ppzksnark_pp>(ap,, time_bound);

//     print!("\nPress enter to continue.\n");
//     std::cin.get();

//     print!("================================================================================\n");
//     print!("TinyRAM ppzkSNARK Prover\n");
//     print!("================================================================================\n\n");
//     boot_trace:ram_ppzksnark_proof<default_tinyram_ppzksnark_pp> proof = ram_ppzksnark_prover<default_tinyram_ppzksnark_pp>(keypair.pk,,  auxiliary_input);

//     print!("\nPress enter to continue.\n");
//     std::cin.get();

//     print!("================================================================================\n");
//     print!("TinyRAM ppzkSNARK Verifier\n");
//     print!("================================================================================\n\n");
//     bool bit = ram_ppzksnark_verifier<default_tinyram_ppzksnark_pp>(keypair.vk, boot_trace, proof);

//     print!("================================================================================\n");
//     print!("The verification result is: {}\n", if bit {"PASS"} else{"FAIL"});
//     print!("================================================================================\n");
//     ffec::print_mem();
//     print!("================================================================================\n");
// }
