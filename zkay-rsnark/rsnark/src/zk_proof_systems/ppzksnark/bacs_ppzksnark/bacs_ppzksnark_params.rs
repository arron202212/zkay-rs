//  Declaration of public-parameter selector for the BACS ppzkSNARK.

// use ff_curves::algebra::curves::public_params;
// use crate::relations::circuit_satisfaction_problems::bacs::bacs;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_primary_input,
};
use ff_curves::Fr;

/**
 * Below are various template aliases (used for convenience).
 */

pub type bacs_ppzksnark_circuit<ppT> = bacs_circuit<Fr<ppT>, pb_variable, pb_linear_combination>;

pub type bacs_ppzksnark_primary_input<ppT> = bacs_primary_input<Fr<ppT>>;

pub type bacs_ppzksnark_auxiliary_input<ppT> = bacs_auxiliary_input<Fr<ppT>>;
