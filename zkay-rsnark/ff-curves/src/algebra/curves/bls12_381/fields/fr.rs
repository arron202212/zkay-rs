use ffec::algebra::fields::prime_base::{MontBackend, fp::Fp256};
// use ff_macros::MontConfig;
use ffec::algebra::fields::field::AdditiveGroup;
use ffec::algebra::fields::prime_base::MontConfig;
#[derive(ff_macros::MontConfig)]
#[modulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[generator = "7"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;
