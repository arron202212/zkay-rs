use ffec::algebra::fields::field::AdditiveGroup;
use ffec::algebra::fields::prime_base::MontConfig;
use ffec::algebra::fields::prime_base::{
    fp::Fp384,
    {MontBackend, MontFp},
};
// use ff_macros::MontConfig;
#[derive(ff_macros::MontConfig)]
#[modulus = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787"]
#[generator = "2"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
pub struct FqConfig;
pub type Fq = Fp384<MontBackend<FqConfig, 6>>;

pub const FQ_ONE: Fq = MontFp!("1");
pub const FQ_ZERO: Fq = MontFp!("0");
