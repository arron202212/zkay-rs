// Template aliasing for prettifying R1CS PCD interfaces.
use crate::gadgetlib2::pp::Fr;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    r1cs_pcd_compliance_predicate, r1cs_pcd_local_data, r1cs_pcd_message,
};
/* template aliasing for R1CS (multi-predicate) ppzkPCD: */
pub trait PcdConfigPptConfig {
    type curve_A_pp;
    type SV;
    type SLC;
}
type r1cs_mp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <PCD_ppT as PcdConfigPptConfig>::SV,
    <PCD_ppT as PcdConfigPptConfig>::SLC,
>;

type r1cs_mp_ppzkpcd_message<PCD_ppT> =
    r1cs_pcd_message<Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>;

type r1cs_mp_ppzkpcd_local_data<PCD_ppT> =
    r1cs_pcd_local_data<Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>;

type r1cs_mp_ppzkpcd_variable_assignment<PCD_ppT> =
    r1cs_variable_assignment<Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>>;
