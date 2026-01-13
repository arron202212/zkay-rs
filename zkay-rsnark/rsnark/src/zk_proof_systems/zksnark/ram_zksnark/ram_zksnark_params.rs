// Declaration of public-parameter selector for the RAM zkSNARK.

// use crate::relations::ram_computations::rams::ram_params;

/**
 * The interfaces of the RAM zkSNARK are templatized via the parameter
 * ram_zksnark_ppT. When used, the interfaces must be invoked with
 * a particular parameter choice; let 'my_ram_zksnark_pp' denote this choice.
 *
 * The pub struct my_ram_zksnark_pp must contain typedefs for the typenames
 * - PCD_pp, and
 * - machine_pp.
 *
 * As well as a method with type signature:
 *  static pub fn  init_public_params()
 *
 * For example, if you want to use the types my_PCD_pp and my_machine_pp,
 * then you would declare my_ram_zksnark_pp as follows:
 *
 *   pub struct my_ram_zksnark_pp {
 *   
 *       type PCD_pp=my_PCD_pp;
 *       type machine_pp=my_machine_pp;
 *       static pub fn  init_public_params()
 *       {
 *           PCD_pp::init_public_params(); // plus other necessary initialization
 *       }
 *   };
 *
 * Having done the above, my_ram_zksnark_pp can be used as a template parameter.
 *
 * See default_tinyram_zksnark_pp in the file
 *
 *   common/default_types/tinyram_zksnark_pp.hpp
 *
 * for an example of the above steps for the case of "RAM=TinyRAM".
 *
 */

/*
 * Below are various template aliases (used for convenience).
 */
pub trait ram_zksnark_ppTConfig {
    type PCD_pp;
    type machine_pp;
}
pub type ram_zksnark_PCD_pp<ram_zksnark_ppT> = <ram_zksnark_ppT as ram_zksnark_ppTConfig>::PCD_pp;

pub type ram_zksnark_machine_pp<ram_zksnark_ppT> =
    <ram_zksnark_ppT as ram_zksnark_ppTConfig>::machine_pp;

pub type ram_zksnark_architecture_params<ram_zksnark_ppT> =
    ram_architecture_params<ram_zksnark_machine_pp<ram_zksnark_ppT>>;

pub type ram_zksnark_primary_input<ram_zksnark_ppT> =
    ram_boot_trace<ram_zksnark_machine_pp<ram_zksnark_ppT>>;

pub type ram_zksnark_auxiliary_input<ram_zksnark_ppT> =
    ram_input_tape<ram_zksnark_machine_pp<ram_zksnark_ppT>>;
