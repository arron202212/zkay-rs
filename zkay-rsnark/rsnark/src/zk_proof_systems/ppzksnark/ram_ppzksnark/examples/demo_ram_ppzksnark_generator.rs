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

use crate::common::default_types::ram_ppzksnark_pp;
use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark;

//#ifndef MINDEPS
// namespace po = boost::program_options;

// bool process_generator_command_line(argc:int, argv:char**,
//                                     String &architecture_params_fn,
//                                     String &computation_bounds_fn,
//                                     String &proving_key_fn,
//                                     String &verification_key_fn)
// {
//     try
//     {
//         po::options_description desc("Usage");
//         desc.add_options()
//             ("help", "print this help message")
//             ("architecture_params", po::value<String>(&architecture_params_fn)->required())
//             ("computation_bounds", po::value<String>(&computation_bounds_fn)->required())
//             ("proving_key", po::value<String>(&proving_key_fn)->required())
//             ("verification_key", po::value<String>(&verification_key_fn)->required());

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
//     ram_ppzksnark_snark_pp<default_ram_ppzksnark_pp>::init_public_params();
// // #ifdef MINDEPS
//     String architecture_params_fn = "architecture_params.txt";
//     String computation_bounds_fn = "computation_bounds.txt";
//     String proving_key_fn = "proving_key.txt";
//     String verification_key_fn = "verification_key.txt";
// #else
//     String architecture_params_fn;
//     String computation_bounds_fn;
//     String proving_key_fn;
//     String verification_key_fn;

//     if (!process_generator_command_line(argc, argv, architecture_params_fn, computation_bounds_fn,
//                                         proving_key_fn, verification_key_fn))
//     {
//         return 1;
//     }
// //#endif
//     ffec::start_profiling();

//     /* load everything */
//     ram_ppzksnark_architecture_params<default_ram_ppzksnark_pp> ap;
//     std::ifstream f_ap(architecture_params_fn);
//     f_ap >> ap;

//     std::ifstream f_rp(computation_bounds_fn);
//     usize tinyram_input_size_bound, tinyram_program_size_bound, time_bound;
//     f_rp >> tinyram_input_size_bound >> tinyram_program_size_bound >> time_bound;

//     let boot_trace_size_bound = tinyram_program_size_bound + tinyram_input_size_bound;

//     boot_trace_size_bound:ram_ppzksnark_keypair<default_ram_ppzksnark_pp> keypair = ram_ppzksnark_generator<default_ram_ppzksnark_pp>(ap,, time_bound);

//     std::ofstream pk(proving_key_fn);
//     pk << keypair.pk;
//     pk.close();

//     std::ofstream vk(verification_key_fn);
//     vk << keypair.vk;
//     vk.close();
// }
