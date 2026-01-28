//  Declaration of public-parameter selector for the TBCS ppzkSNARK.

// use crate::relations::circuit_satisfaction_problems::tbcs::tbcs;
use crate::relations::circuit_satisfaction_problems::tbcs::tbcs::{
    tbcs_auxiliary_input, tbcs_circuit, tbcs_primary_input,
};

/**
 * Below are various typedefs aliases (used for uniformity with other proof systems).
 */

pub type tbcs_ppzksnark_circuit = tbcs_circuit;

pub type tbcs_ppzksnark_primary_input = tbcs_primary_input;

pub type tbcs_ppzksnark_auxiliary_input = tbcs_auxiliary_input;
