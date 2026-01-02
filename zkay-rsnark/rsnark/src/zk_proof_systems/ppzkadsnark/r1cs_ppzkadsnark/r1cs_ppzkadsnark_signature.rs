// Generic signature interface for ADSNARK.

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    labelT, r1cs_ppzkadsnark_ppTConfig, r1cs_ppzkadsnark_sigT, r1cs_ppzkadsnark_skT,
    r1cs_ppzkadsnark_vkT, snark_pp,
};
use ff_curves::G2;

#[derive(Default, Clone)]
pub struct kpT<ppT: r1cs_ppzkadsnark_ppTConfig> {
    pub sk: r1cs_ppzkadsnark_skT<ppT>,
    pub vk: r1cs_ppzkadsnark_vkT<ppT>,
}

pub trait SigConfig {
    fn sigGen<ppT: r1cs_ppzkadsnark_ppTConfig>() -> kpT<ppT>;

    fn sigSign<ppT: r1cs_ppzkadsnark_ppTConfig>(
        sk: &r1cs_ppzkadsnark_skT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
    ) -> r1cs_ppzkadsnark_sigT<ppT>;

    fn sigVerif<ppT: r1cs_ppzkadsnark_ppTConfig>(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
        sig: &r1cs_ppzkadsnark_sigT<ppT>,
    ) -> bool;

    fn sigBatchVerif<ppT: r1cs_ppzkadsnark_ppTConfig>(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        labels: &Vec<labelT>,
        Lambdas: &Vec<G2<snark_pp<ppT>>>,
        sigs: &Vec<r1cs_ppzkadsnark_sigT<ppT>>,
    ) -> bool;
}
