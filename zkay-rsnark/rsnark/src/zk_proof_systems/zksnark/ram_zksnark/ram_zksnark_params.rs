// Declaration of public-parameter selector for the RAM zkSNARK.

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_boot_trace, ram_input_tape,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdPptConfig;
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate::{
    ram_pcd_local_data, ram_pcd_message,
};

/**
 * The interfaces of the RAM zkSNARK are templatized via the parameter
 * RamT. When used, the interfaces must be invoked with
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
pub trait RamConfig:
    ppTConfig<FieldT = <Self::machine_pp as ram_params_type>::base_field_type>
{
    type PCD_pp: PcdPptConfig<
            curve_A_pp = Self::machine_pp,
            FieldT = <Self as ppTConfig>::FieldT,
            LD = ram_pcd_local_data<Self::machine_pp>,
            M = ram_pcd_message<Self::machine_pp>,
        >;
    type machine_pp: ram_params_type<
            CPH = <Self::PCD_pp as PcdPptConfig>::curve_A_pp,
            M = ram_pcd_message<Self::machine_pp>,
            LD = ram_pcd_local_data<Self::machine_pp>,
        >;
}
pub type ram_zksnark_PCD_pp<RamT> = <RamT as RamConfig>::PCD_pp;

pub type ram_zksnark_machine_pp<RamT> = <RamT as RamConfig>::machine_pp;

pub type ram_zksnark_architecture_params<RamT> =
    ram_architecture_params<ram_zksnark_machine_pp<RamT>>;

pub type ram_zksnark_primary_input = ram_boot_trace;

pub type ram_zksnark_auxiliary_input = ram_input_tape;
