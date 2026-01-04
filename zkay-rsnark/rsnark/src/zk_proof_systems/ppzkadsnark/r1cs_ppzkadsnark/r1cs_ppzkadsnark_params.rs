// Declaration of public-parameter selector for the R1CS ppzkADSNARK.

use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::zk_proof_systems::PptConfig;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::PpConfig;

#[derive(Default, Clone)]
pub struct labelT {
    pub label_bytes: Vec<u8>, //[u8; 16],
                              // labelT() {};
}

pub trait SigTConfig {
    fn sig_bytes(&self) -> &[u8];
    fn sig_bytes_mut(&mut self) -> &mut Vec<u8>;
}

pub trait VkTConfig {
    fn vk_bytes(&self) -> &[u8];
}

pub trait r1cs_ppzkadsnark_ppTConfig: Sized + Default + Clone {
    type snark_pp: PublicParams;
    type skT: Default + Clone;
    type vkT: VkTConfig + Default + Clone;
    type sigT: SigTConfig + Default + Clone;
    type prfKeyT: Default + Clone;
    fn init_public_params();
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
