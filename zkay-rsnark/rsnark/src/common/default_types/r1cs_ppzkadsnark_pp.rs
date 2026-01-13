// This file defines default_r1cs_ppzkadsnark_pp based on the elliptic curve
// choice selected in ec_pp.hpp.

use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::prf::aes_ctr_prf::aesPrfKeyT;
 use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::ppzkadsnarkConfig;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::signature::ed25519_signature::{ed25519_sigT,ed25519_skT,ed25519_vkT};
use ffec::PpConfig;
 use ffec::field_utils::bigint::bigint;
use ff_curves::PublicParams;

#[derive(Default, Clone)]
pub struct default_r1cs_ppzkadsnark_pp;

#[derive(Default, Clone)]
pub struct pp;
impl PpConfig for pp {
    type T = bigint<4>;
}
impl ppTConfig for pp {}

impl evaluation_domain for pp {}
impl ppzkadsnarkConfig for default_r1cs_ppzkadsnark_pp {
    type snark_pp = default_r1cs_ppzksnark_pp;
    type skT = ed25519_skT;
    type vkT = ed25519_vkT;
    type sigT = ed25519_sigT;
    type prfKeyT = aesPrfKeyT;
    type ppT = pp;
    type Sig = Ed25519<Self>;
    type Prf = PrfAdsnark<Self>;
    type ED = pp;

    fn init_public_params() {
        Self::snark_pp::init_public_params();
    }
}
