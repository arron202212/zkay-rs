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

// impl<O: Borrow<Self>> Add<O> for pp {
//     type Output = pp;

//     fn add(self, other: O) -> Self::Output {
//         let mut r = self;
//         // r += *other.borrow();
//         r
//     }
// }

// impl Sub for pp {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         let mut r = self;
//         // r -= other;
//         r
//     }
// }

// impl<const N: usize> Mul<bigint<N>> for pp {
//     type Output = pp;

//     fn mul(self, rhs: bigint<N>) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

// impl<const N: usize, T: Fp_modelConfig<N>> Mul<Fp_model<N, T>> for pp {
//     type Output = pp;

//     fn mul(self, rhs: Fp_model<N, T>) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

// impl<O: Borrow<Self>> Mul<O> for pp {
//     type Output = pp;

//     fn mul(self, rhs: O) -> Self::Output {
//         let mut r = self;
//         // r *= *rhs.borrow();
//         r
//     }
// }

// impl Neg for pp {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         self
//     }
// }

// use std::fmt;
// impl fmt::Display for pp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", Self::one())
//     }
// }

// impl One for pp {
//     fn one() -> Self {
//         Self::one()
//     }
// }

// impl Zero for pp {
//     fn zero() -> Self {
//         Self::zero()
//     }
//     fn is_zero(&self) -> bool {
//         false
//     }
// }

// impl PpConfig for pp{
//     type TT = bigint<1>;
//     // type Fr=Self;
// }

// impl ppTConfig for pp{
//     // type TT = bigint<1>;
//     // type Fr=Self;
// }

// impl FieldTConfig for pp{
//     // type TT = bigint<1>;
//     // type Fr=Self;
// }
// impl PublicParams for pp{
//     // type TT = bigint<1>;
//     // type Fr=Self;
// }
