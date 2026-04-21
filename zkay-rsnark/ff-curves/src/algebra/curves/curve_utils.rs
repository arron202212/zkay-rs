use ffec::PpConfig;
use ffec::algebra::field_utils::bigint::bigint;
// GroupT scalar_mul(base:&GroupT, scalar:&bigint<m>);

pub fn scalar_mul<GroupT: PpConfig, const M: usize>(base: &GroupT, scalar: &bigint<M>) -> GroupT {
    let mut result = GroupT::zero();

    let mut found_one = false;
    for i in (0..=(scalar.max_bits() - 1)).rev() {
        if found_one {
            result = result.dbl();
        }

        if scalar.test_bit(i) {
            found_one = true;
            result = result + base.clone();
        }
    }

    result
}
