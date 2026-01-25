// Template aliasing for prettifying R1CS PCD interfaces.
use crate::gadgetlib1::gadgets::pairing::pairing_params::{pairing_selector, ppTConfig};
use crate::gadgetlib2::pp::Fr;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig, r1cs_pcd_compliance_predicate, r1cs_pcd_local_data,
    r1cs_pcd_message,
};
use ff_curves::PublicParams;
use ffec::FieldTConfig;
/* template aliasing for R1CS (multi-predicate) ppzkPCD: */
pub trait PcdPptConfig: ppTConfig {
    type curve_A_pp: ppTConfig<
            P = Self::AP,
            FieldT = <Self as ppTConfig>::FieldT,
            M = <Self as ppTConfig>::M,
            LD = <Self as ppTConfig>::LD,
        >;
    type curve_B_pp: ppTConfig<
            P = Self::BP,
            FieldT = <Self as ppTConfig>::FieldT,
            M = <Self as ppTConfig>::M,
            LD = <Self as ppTConfig>::LD,
        >;
    type AP: pairing_selector<my_ec_pp = Self::curve_A_pp, other_curve_type = Self::curve_B_pp>;
    type BP: pairing_selector<my_ec_pp = Self::curve_B_pp, other_curve_type = Self::curve_A_pp>;
    const N: usize = 4;
}
pub type r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<
    Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>,
    <PCD_ppT as PcdPptConfig>::curve_A_pp,
>;

// pub type r1cs_mp_ppzkpcd_message<PCD_ppT> =
//     r1cs_pcd_message<Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>, <<PCD_ppT as PcdPptConfig>::curve_A_pp> as ppTConfig>::M>;

// pub type r1cs_mp_ppzkpcd_local_data<PCD_ppT> = r1cs_pcd_local_data<
//     Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>,
//     <<PCD_ppT as PcdPptConfig>::curve_A_pp> as ppTConfig>::LD,
// >;

pub type r1cs_mp_ppzkpcd_variable_assignment<PCD_ppT> =
    r1cs_variable_assignment<Fr<<PCD_ppT as PcdPptConfig>::curve_A_pp>>;
