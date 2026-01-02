// Generic PRF interface for ADSNARK.
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    labelT, r1cs_ppzkadsnark_ppTConfig, r1cs_ppzkadsnark_prfKeyT, snark_pp,
};
use ff_curves::Fr;

pub trait PrfConfig {
    fn prfGen<ppT: r1cs_ppzkadsnark_ppTConfig>() -> r1cs_ppzkadsnark_prfKeyT<ppT>;

    fn prfCompute<ppT: r1cs_ppzkadsnark_ppTConfig>(
        key: &r1cs_ppzkadsnark_prfKeyT<ppT>,
        label: &labelT,
    ) -> Fr<snark_pp<ppT>>;
}
