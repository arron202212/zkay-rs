// This file defines default_r1cs_ppzkadsnark_pp based on the elliptic curve
// choice selected in ec_pp.hpp.

use crate::common::default_types::r1cs_ppzksnark_pp::default_r1cs_ppzksnark_pp;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::prf::aes_ctr_prf::{aesPrfKeyT,PrfAdsnark};
 use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::ppzkadsnarkConfig;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::examples::signature::ed25519_signature::{Ed25519,ed25519_sigT,ed25519_skT,ed25519_vkT};
use ffec::PpConfig;
 use ffec::field_utils::bigint::bigint;
use ff_curves::PublicParams;
use ff_curves::algebra::curves::alt_bn128::alt_bn128_fields::Backend;
use std::ops::{Add,Sub,Mul,Neg};
use ffec::{One,Zero};
use std::borrow::Borrow;
 use ffec::{Fp_model,Fp_modelConfig};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct pp;

#[derive(Default, Clone)]
pub struct default_r1cs_ppzkadsnark_pp;

// impl ppzkadsnarkConfig for default_r1cs_ppzkadsnark_pp {
//     type snark_pp = default_r1cs_ppzksnark_pp;
//     type skT = ed25519_skT;
//     type vkT = ed25519_vkT;
//     type sigT = ed25519_sigT;
//     type prfKeyT = aesPrfKeyT;
//     type ppT = pp;
//     type Sig = Ed25519<Self>;
//     type Prf = PrfAdsnark<Self>;
//     type FieldT=pp;

//     fn init_public_params() {
//         Self::snark_pp::init_public_params();
//     }
// }
