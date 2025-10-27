/** @file
 *****************************************************************************

 Declaration of public-parameter selector for RAMs.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_PARAMS_HPP_
// #define RAM_PARAMS_HPP_

// 

use ffec::common::utils;

use crate::relations::ram_computations::memory::memory_store_trace;



/*
  When declaring a new ramT one should do a make it a pub struct that declares typedefs for:

  base_field_type
  ram_cpu_checker_type
  architecture_params_type

  For ram_to_r1cs reduction currently the following are also necessary:
  protoboard_type (e.g. tinyram_protoboard<FieldT>)
  gadget_base_type (e.g. tinyram_gadget<FieldT>)
  cpu_state_variable_type (must have pb_variable_array<FieldT> all_vars)

  The ramT pub struct must also have a static usize variable
  timestamp_length, which specifies the zk-SNARK reduction timestamp
  length.
*/


type  ram_base_field<ramT> =  ramT::base_field_type;


type ram_cpu_state=bit_vector;


type ram_boot_trace =memory_store_trace;


type  ram_protoboard<ramT> =  ramT::protoboard_type;


type  ram_gadget_base<ramT> =  ramT::gadget_base_type;


type  ram_cpu_checker<ramT> =  ramT::cpu_checker_type;


type  ram_architecture_params<ramT> =  ramT::architecture_params_type;


type  ram_input_tape = Vec<usize>;

/*
  One should also make the following methods for ram_architecture_params

  (We are not yet making a ram_architecture_params base class, as it
  would require base pub struct for ram_program

  TODO: make this base class)

  usize address_size();
  usize value_size();
  usize cpu_state_size();
  usize initial_pc_addr();
  bit_vector initial_cpu_state();
*/



//#endif // RAM_PARAMS_HPP_
