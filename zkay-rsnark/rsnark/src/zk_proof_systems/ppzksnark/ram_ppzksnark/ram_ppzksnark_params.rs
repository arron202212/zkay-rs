// Declaration of public-parameter selector for the RAM ppzkSNARK.

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_boot_trace, ram_input_tape, ram_params_type,
};

/**
 * The interfaces of the RAM ppzkSNARK are templatized via the parameter
 * ram_ppzksnark_ppT. When used, the interfaces must be invoked with
 * a particular parameter choice; let 'my_ram_ppzksnark_pp' denote this choice.
 *
 * my_ram_ppzksnark_pp needs to contain typedefs for the s
 * - snark_pp, and
 * - machine_pp.
 * as well as a method with the following signature:
 * - static pub fn  init_public_params();
 *
 * For example, if you want to use the types my_snark_pp and my_machine_pp,
 * then you could declare my_ram_ppzksnark_pp as follows:
 *
 *   pub struct my_ram_ppzksnark_pp {
 *   
 *       pub type snark_pp=my_snark_pp;
 *       pub type machine_pp=my_machine_pp;
 *       static pub fn  init_public params()
 *       {
 *           snark_pp::init_public_params(); // and additional initialization if needed
 *       }
 *   };
 *
 * Having done the above, my_ram_ppzksnark_pp can be used as a template parameter.
 *
 * Look for for default_tinyram_ppzksnark_pp in the file
 *
 *   common/default_types/ram_ppzksnark_pp.hpp
 *
 * for an example of the above steps for the case of "RAM=TinyRAM".
 *
 */

/**
 * Below are various template aliases (used for convenience).
 */

pub trait RamPptConfig: Default + Clone {
    type snark_pp: ppTConfig<FieldT = <Self::machine_pp as ram_params_type>::base_field_type>;
    type machine_pp: ram_params_type;

    fn init_public_params() {}
}

pub type ram_ppzksnark_snark_pp<ram_ppzksnark_ppT> = <ram_ppzksnark_ppT as RamPptConfig>::snark_pp;

pub type ram_ppzksnark_machine_pp<ram_ppzksnark_ppT> =
    <ram_ppzksnark_ppT as RamPptConfig>::machine_pp;

pub type ram_ppzksnark_architecture_params<ram_ppzksnark_ppT> =
    ram_architecture_params<ram_ppzksnark_machine_pp<ram_ppzksnark_ppT>>;

pub type ram_ppzksnark_primary_input = ram_boot_trace;

pub type ram_ppzksnark_auxiliary_input = ram_input_tape;
