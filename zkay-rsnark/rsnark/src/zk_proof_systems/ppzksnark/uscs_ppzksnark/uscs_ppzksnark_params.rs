//  Declaration of public-parameter selector for the USCS ppzkSNARK.

// use ff_curves::algebra::curves::public_params;
// use crate::relations::constraint_satisfaction_problems/uscs/uscs;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::uscs::uscs::{
    uscs_auxiliary_input, uscs_constraint_system, uscs_primary_input,
};
use ff_curves::Fr;

/**
 * Below are various template aliases (used for convenience).
 */

pub type uscs_ppzksnark_constraint_system<ppT> =
    uscs_constraint_system<Fr<ppT>, pb_variable, pb_linear_combination>;

pub type uscs_ppzksnark_primary_input<ppT> = uscs_primary_input<Fr<ppT>>;

pub type uscs_ppzksnark_auxiliary_input<ppT> = uscs_auxiliary_input<Fr<ppT>>;
