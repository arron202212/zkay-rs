// This file defines the default choices of TinyRAM zk-SNARK.

// use crate::common::default_types::r1cs_ppzkpcd_pp;
// use crate::relations::ram_computations::rams::tinyram::tinyram_params;
use ffec::FieldTConfig;

pub struct default_tinyram_zksnark_pp;
pub trait default_tinyram_zksnark_ppConfig {
    type PCD_pp; //=default_r1cs_ppzkpcd_pp;
    type FieldT: FieldTConfig; //=PCD_pp::scalar_field_A;
    type machine_pp; //=ram_tinyram<FieldT>;

    fn init_public_params() {
        // PCD_pp::init_public_params();
    }
}

//  This file provides the initialization methods for the default TinyRAM zk-SNARK.

// use crate::common::default_types::tinyram_zksnark_pp;
