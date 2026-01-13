// Template aliasing for prettifying R1CS PCD interfaces.
use crate::gadgetlib1::gadgets::pairing::pairing_params::{pairing_selector, ppTConfig};
use crate::gadgetlib2::pp::Fr;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    R1csPcdLocalDataConfig, R1csPcdMessageConfig, r1cs_pcd_compliance_predicate,
    r1cs_pcd_local_data, r1cs_pcd_message,
};
use ff_curves::PublicParams;
use ffec::FieldTConfig;
/* template aliasing for R1CS (multi-predicate) ppzkPCD: */
pub trait PcdConfigPptConfig: ppTConfig {
    type curve_A_pp: ppTConfig;
    type curve_B_pp: ppTConfig;
    type AP: pairing_selector<my_ec_pp = Self::curve_A_pp, other_curve_type = Self::curve_B_pp>;
    type BP: pairing_selector<my_ec_pp = Self::curve_B_pp, other_curve_type = Self::curve_A_pp>;
    type FieldT: FieldTConfig;
    type SV;
    type SLC;
    type M: R1csPcdMessageConfig<FieldT = <Self as PcdConfigPptConfig>::FieldT>;
    type LD: R1csPcdLocalDataConfig<FieldT = <Self as PcdConfigPptConfig>::FieldT>;
    const N: usize = 4;
}
type r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<PCD_ppT>;

type r1cs_mp_ppzkpcd_message<PCD_ppT> = r1cs_pcd_message<<PCD_ppT as PcdConfigPptConfig>::M>;

type r1cs_mp_ppzkpcd_local_data<PCD_ppT> = r1cs_pcd_local_data<<PCD_ppT as PcdConfigPptConfig>::LD>;

type r1cs_mp_ppzkpcd_variable_assignment<PCD_ppT> =
    r1cs_variable_assignment<Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>;
