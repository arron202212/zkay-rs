/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
// use  <fstream>
// use  <iostream>
//#ifndef MINDEPS
// use boost/program_options;
//#endif
use crate::common::default_types::tinyram_ppzksnark_pp;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;

// //#ifndef MINDEPS
// namespace po = boost::program_options;

// bool process_verifier_command_line(argc:int, argv:char**,
//                                    String &processed_assembly_fn,
//                                    String &verification_key_fn,
//                                    String &primary_input_fn,
//                                    String &proof_fn,
//                                    String &verification_result_fn)
// {
//     try
//     {
//         po::options_description desc("Usage");
//         desc.add_options()
//             ("help", "print this help message")
//             ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
//             ("verification_key", po::value<String>(&verification_key_fn)->required())
//             ("primary_input", po::value<String>(&primary_input_fn)->required())
//             ("proof", po::value<String>(&proof_fn)->required())
//             ("verification_result", po::value<String>(&verification_result_fn)->required());

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
// //#endif

// int main(int argc, const char * argv[])
// {
//     default_tinyram_ppzksnark_pp::init_public_params();

// // #ifdef MINDEPS
//     String processed_assembly_fn = "processed.txt";
//     String verification_key_fn = "verification_key.txt";
//     String proof_fn = "proof.txt";
//     String primary_input_fn = "primary_input.txt";
//     String verification_result_fn = "verification_result.txt";
// #else
//     String processed_assembly_fn;
//     String verification_key_fn;
//     String proof_fn;
//     String primary_input_fn;
//     String verification_result_fn;

//     if !process_verifier_command_line(argc, argv, processed_assembly_fn, verification_key_fn, primary_input_fn, proof_fn, verification_result_fn)
//     {
//         return 1;
//     }
// //#endif
//     ffec::start_profiling();

//     ram_ppzksnark_verification_key<default_tinyram_ppzksnark_pp> vk;
//     std::ifstream vk_file(verification_key_fn);
//     vk_file >> vk;
//     vk_file.close();

//     std::ifstream processed(processed_assembly_fn);
//     tinyram_program program = load_preprocessed_program(vk.ap, processed);

//     std::ifstream f_primary_input(primary_input_fn);
//     tinyram_input_tape primary_input = load_tape(f_primary_input);

//     std::ifstream proof_file(proof_fn);
//     ram_ppzksnark_proof<default_tinyram_ppzksnark_pp> pi;
//     proof_file >> pi;
//     proof_file.close();

//     program:ram_boot_trace<default_tinyram_ppzksnark_pp> boot_trace = tinyram_boot_trace_from_program_and_input(vk.ap, vk.primary_input_size_bound,, primary_input);
//     boot_trace:bool bit = ram_ppzksnark_verifier<default_tinyram_ppzksnark_pp>(vk,, pi);

//     print!("================================================================================\n");
//     print!("The verification result is: {}\n", if bit {"PASS"} else{"FAIL"});
//     print!("================================================================================\n");
//     std::ofstream vr_file(verification_result_fn);
//     vr_file << bit << "\n";
//     vr_file.close();
// }
