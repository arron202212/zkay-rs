use ffec::algebra::fields::{prime_base::fp::Fp256, prime_base::{MontBackend}};
use ffec::algebra::fields::prime_base::MontConfig;
use ffec::algebra::fields::field::AdditiveGroup;
use ffec::algebra::Fp;
// use ff_macros::MontConfig;
#[derive(ff_macros::MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;
