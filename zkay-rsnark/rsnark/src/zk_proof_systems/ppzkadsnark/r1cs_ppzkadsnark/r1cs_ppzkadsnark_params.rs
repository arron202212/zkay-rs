// Declaration of public-parameter selector for the R1CS ppzkADSNARK.

use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint_system, r1cs_primary_input,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf::PrfConfig;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature::SigConfig;
use ff_curves::Fr;
use ff_curves::PublicParams;
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::scalar_multiplication::multiexp::KCConfig;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;

#[derive(Default, Clone)]
pub struct labelT {
    pub label_bytes: Vec<u8>, //[u8; 16],
                              // labelT() {};
}

pub trait SigTConfig: Default + Clone {
    fn sig_bytes(&self) -> &[u8];
    fn sig_bytes_mut(&mut self) -> &mut Vec<u8>;
}

pub trait VkTConfig: Default + Clone {
    fn vk_bytes(&self) -> &[u8];
}

pub trait ppzkadsnarkConfig: ppTConfig + Sized + Default + Clone {
    type Sig: SigConfig<Self>;
    type Prf: PrfConfig<Self>;
    type snark_pp: ppTConfig;
    type skT: Default + Clone;
    type vkT: VkTConfig;
    type sigT: SigTConfig;
    type prfKeyT: Default + Clone;
    const NN: usize = 4;

    fn init_public_params();
}

/**
 * Below are various template aliases (used for convenience).
 */

pub type snark_pp<r1cs_ppzkadsnark_ppT> = <r1cs_ppzkadsnark_ppT as ppzkadsnarkConfig>::snark_pp;

pub type r1cs_ppzkadsnark_constraint_system<r1cs_ppzkadsnark_ppT> =
    r1cs_constraint_system<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>, pb_variable, pb_linear_combination>;

pub type r1cs_ppzkadsnark_primary_input<r1cs_ppzkadsnark_ppT> =
    r1cs_primary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>>;

pub type r1cs_ppzkadsnark_auxiliary_input<r1cs_ppzkadsnark_ppT> =
    r1cs_auxiliary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>>;

pub type r1cs_ppzkadsnark_skT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as ppzkadsnarkConfig>::skT;

pub type r1cs_ppzkadsnark_vkT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as ppzkadsnarkConfig>::vkT;

pub type r1cs_ppzkadsnark_sigT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as ppzkadsnarkConfig>::sigT;

pub type r1cs_ppzkadsnark_prfKeyT<r1cs_ppzkadsnark_ppT> =
    <r1cs_ppzkadsnark_ppT as ppzkadsnarkConfig>::prfKeyT;
