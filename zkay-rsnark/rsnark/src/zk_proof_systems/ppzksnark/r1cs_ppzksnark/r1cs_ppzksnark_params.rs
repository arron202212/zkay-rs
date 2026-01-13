// Declaration of public-parameter selector for the R1CS ppzkSNARK.

use crate::gadgetlib1::gadgets::pairing::pairing_params::{pairing_selector, ppTConfig};
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::ppzkpcd_compliance_predicate::PcdConfigPptConfig;
use ff_curves::Fr;

/**
 * Below are various template aliases (used for convenience).
 */

pub type r1cs_ppzksnark_constraint_system<ppT> =
    r1cs_constraint_system<<ppT as ppTConfig>::FieldT, pb_variable, pb_linear_combination>;

pub type r1cs_ppzksnark_primary_input<ppT> = r1cs_primary_input<<ppT as ppTConfig>::FieldT>;

pub type r1cs_ppzksnark_auxiliary_input<ppT> = r1cs_auxiliary_input<<ppT as ppTConfig>::FieldT>;
