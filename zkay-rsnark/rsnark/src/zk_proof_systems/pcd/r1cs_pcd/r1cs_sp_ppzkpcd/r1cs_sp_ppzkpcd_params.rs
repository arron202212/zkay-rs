// Parameters for *single-predicate* ppzkPCD for R1CS.

use crate::gadgetlib1::gadgets::pairing::pairing_params::pairing_selector;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    r1cs_pcd_compliance_predicate, r1cs_pcd_local_data, r1cs_pcd_message,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdConfigPptConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_mp_ppzkpcd::r1cs_mp_ppzkpcd_params::r1cs_mp_ppzkpcd_primary_input;
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::{
    r1cs_pcd_compliance_predicate_auxiliary_input, r1cs_pcd_compliance_predicate_primary_input,
};
use ff_curves::Fr;

pub type r1cs_sp_ppzkpcd_compliance_predicate<PCD_ppT> = r1cs_pcd_compliance_predicate<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <PCD_ppT as PcdConfigPptConfig>::curve_A_pp,
>;

pub type r1cs_sp_ppzkpcd_message<PCD_ppT> = r1cs_pcd_message<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::M,
>;

pub type r1cs_sp_ppzkpcd_local_data<PCD_ppT> = r1cs_pcd_local_data<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::LD,
>;

pub type r1cs_sp_ppzkpcd_primary_input<PCD_ppT> = r1cs_pcd_compliance_predicate_primary_input<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::M,
>;

pub type r1cs_sp_ppzkpcd_auxiliary_input<PCD_ppT> = r1cs_pcd_compliance_predicate_auxiliary_input<
    Fr<<PCD_ppT as PcdConfigPptConfig>::curve_A_pp>,
    <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::M,
    <<PCD_ppT as PcdConfigPptConfig>::curve_A_pp as ppTConfig>::LD,
>;
