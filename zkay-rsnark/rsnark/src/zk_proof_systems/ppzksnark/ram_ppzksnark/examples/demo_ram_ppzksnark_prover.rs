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

//#ifndef MINDEPS
// namespace po = boost::program_options;

// bool process_prover_command_line(argc:int, argv:char**,
//                                  String &processed_assembly_fn,
//                                  String &proving_key_fn,
//                                  String &primary_input_fn,
//                                  String &auxiliary_input_fn,
//                                  String &proof_fn)
// {
//     try
//     {
//         po::options_description desc("Usage");
//         desc.add_options()
//             ("help", "print this help message")
//             ("processed_assembly", po::value<String>(&processed_assembly_fn)->required())
//             ("proving_key", po::value<String>(&proving_key_fn)->required())
//             ("primary_input", po::value<String>(&primary_input_fn)->required())
//             ("auxiliary_input", po::value<String>(&auxiliary_input_fn)->required())
//             ("proof", po::value<String>(&proof_fn)->required());

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
//     String proving_key_fn = "proving_key.txt";
//     String primary_input_fn = "primary_input.txt";
//     String auxiliary_input_fn = "auxiliary_input.txt";
//     String proof_fn = "proof.txt";
// #else
//     String processed_assembly_fn;
//     String proving_key_fn;
//     String primary_input_fn;
//     String auxiliary_input_fn;
//     String proof_fn;

//     if (!process_prover_command_line(argc, argv, processed_assembly_fn,
//                                      proving_key_fn, primary_input_fn, auxiliary_input_fn, proof_fn))
//     {
//         return 1;
//     }
// //#endif
//     ffec::start_profiling();

//     /* load everything */
//     ram_ppzksnark_proving_key<default_tinyram_ppzksnark_pp> pk;
//     std::ifstream pk_file(proving_key_fn);
//     pk_file >> pk;
//     pk_file.close();

//     std::ifstream processed(processed_assembly_fn);
//     tinyram_program program = load_preprocessed_program(pk.ap, processed);

//     std::ifstream f_primary_input(primary_input_fn);
//     std::ifstream f_auxiliary_input(auxiliary_input_fn);
//     tinyram_input_tape primary_input = load_tape(f_primary_input);
//     tinyram_input_tape auxiliary_input = load_tape(f_auxiliary_input);

//     program:ram_boot_trace<default_tinyram_ppzksnark_pp> boot_trace = tinyram_boot_trace_from_program_and_input(pk.ap, pk.primary_input_size_bound,, primary_input);
//     boot_trace:ram_ppzksnark_proof<default_tinyram_ppzksnark_pp> proof = ram_ppzksnark_prover<default_tinyram_ppzksnark_pp>(pk,,  auxiliary_input);

//     std::ofstream proof_file(proof_fn);
//     proof_file << proof;
//     proof_file.close();
// }
