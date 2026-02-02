//  This file defines the default PCD cycle.

// /*********************** Define default PCD cycle ***************************/
// use ff_curves::algebra::curves::mnt::mnt4::mnt4_pp;
// use ff_curves::algebra::curves::mnt::mnt6::mnt6_pp;

pub trait default_r1cs_ppzkpcd_ppConfig {
    type curve_A_pp; //=mnt4_pp;
    type curve_B_pp; //=mnt6_pp;

    type scalar_field_A; //=Fr<curve_A_pp>;
    type scalar_field_B; //=Fr<curve_B_pp>;

    fn init_public_params() {
        // Self::curve_A_pp::init_public_params();
        //  Self::curve_B_pp::init_public_params();
    }
}

// use crate::common::default_types::r1cs_ppzkpcd_pp;
