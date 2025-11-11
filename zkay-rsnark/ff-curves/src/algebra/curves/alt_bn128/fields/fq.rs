use ffec::algebra::fields::{*,prime_base::fp::Fp256, prime_base::MontBackend};
use ffec::algebra::fields::prime_base::MontConfig;
use ffec::algebra::fields::field::AdditiveGroup;
// use ff_macros::MontConfig;
#[derive(ff_macros::MontConfig)]
#[modulus = "21888242871839275222246405745257275088696311157297823662689037894645226208583"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
