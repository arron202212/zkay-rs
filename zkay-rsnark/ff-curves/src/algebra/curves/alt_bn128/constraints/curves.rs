use ark_r1cs_std::groups::curves::short_weierstrass::ProjectiveVar;

use super::super::curves::g1::Config;
use super::FBaseVar;

/// A group element in the Bn254 prime-order group.
pub type GVar = ProjectiveVar<Config, FBaseVar>;

#[test]
fn test() {
    ark_curve_constraint_tests::curves::sw_test::<Config, GVar>().unwrap();
}
