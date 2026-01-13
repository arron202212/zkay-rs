// Generic signature interface for ADSNARK.

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    labelT, ppzkadsnarkConfig, r1cs_ppzkadsnark_sigT, r1cs_ppzkadsnark_skT, r1cs_ppzkadsnark_vkT,
    snark_pp,
};
use ff_curves::G2;

#[derive(Default, Clone)]
pub struct kpT<ppT: ppzkadsnarkConfig> {
    pub sk: r1cs_ppzkadsnark_skT<ppT>,
    pub vk: r1cs_ppzkadsnark_vkT<ppT>,
}

pub trait SigConfig<ppT: ppzkadsnarkConfig> {
    fn sigGen() -> kpT<ppT>;

    fn sigSign(
        sk: &r1cs_ppzkadsnark_skT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
    ) -> r1cs_ppzkadsnark_sigT<ppT>;

    fn sigVerif(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
        sig: &r1cs_ppzkadsnark_sigT<ppT>,
    ) -> bool;

    fn sigBatchVerif(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        labels: &Vec<labelT>,
        Lambdas: &Vec<G2<snark_pp<ppT>>>,
        sigs: &Vec<r1cs_ppzkadsnark_sigT<ppT>>,
    ) -> bool;
}
