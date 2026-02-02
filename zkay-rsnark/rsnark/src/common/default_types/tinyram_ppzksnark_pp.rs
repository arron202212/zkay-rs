// This file defines the default architecture and curve choices for RAM
// ppzk-SNARK.

use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_architecture_params;
use crate::relations::ram_computations::rams::tinyram::tinyram_params::ram_tinyram;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use ff_curves::Fr;
// use crate::common::default_types::r1cs_ppzksnark_pp;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;

#[derive(Default, Clone)]
pub struct default_tinyram_ppzksnark_pp;
pub trait default_tinyram_ppzksnark_ppConfig: RamPptConfig<machine_pp = Self::machine_ppp> {
    type snark_pp; //=default_r1cs_ppzksnark_pp;
    type FieldT; //=Fr<default_r1cs_ppzksnark_pp>;
    type machine_ppp: ram_params_type<architecture_params_type = tinyram_architecture_params>; //=ram_tinyram<FieldT>;
    // type ram_ppt:RamPptConfig<machine_pp=Self::machine_pp>>
    fn init_public_params() {
        // snark_pp::init_public_params();
    }
}

//  This file provides the initialization methods for the default TinyRAM ppzk-SNARK.

// use crate::common::default_types::tinyram_ppzksnark_pp;
