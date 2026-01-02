// Declaration of public-parameter selector for the R1CS ppzkADSNARK.

// use ff_curves::algebra::curves::public_params;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::zk_proof_systems::PptConfig;
use ff_curves::Fr;

pub struct labelT {
    label_bytes: [u8; 16],
    // labelT() {};
}
pub trait r1cs_ppzkadsnark_ppTConfig: PptConfig + Sized {
    type snark_pp: PptConfig;
    type skT: Default + Clone;
    type vkT: Default + Clone;
    type sigT: Default + Clone;
    type prfKeyT: Default + Clone;
}
/**
 * Below are various template aliases (used for convenience).
 */

pub type snark_pp<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as r1cs_ppzkadsnark_ppTConfig>::snark_pp;

pub type r1cs_ppzkadsnark_constraint_system<r1cs_ppzkadsnark_ppT> =
    r1cs_constraint_system<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>, pb_variable, pb_linear_combination>;

pub type r1cs_ppzkadsnark_primary_input<r1cs_ppzkadsnark_ppT> =
    r1cs_primary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>>;

pub type r1cs_ppzkadsnark_auxiliary_input<r1cs_ppzkadsnark_ppT> =
    r1cs_auxiliary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>>;

pub type r1cs_ppzkadsnark_skT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as r1cs_ppzkadsnark_ppTConfig>::skT;

pub type r1cs_ppzkadsnark_vkT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as r1cs_ppzkadsnark_ppTConfig>::vkT;

pub type r1cs_ppzkadsnark_sigT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as r1cs_ppzkadsnark_ppTConfig>::sigT;

pub type r1cs_ppzkadsnark_prfKeyT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as r1cs_ppzkadsnark_ppTConfig>::prfKeyT;
