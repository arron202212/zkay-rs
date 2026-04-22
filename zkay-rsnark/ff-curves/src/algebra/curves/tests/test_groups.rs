// use crate::algebra::curves::edwards::edwards_pp;
// use crate::algebra::curves::mnt::mnt4::mnt4_pp;
// use crate::algebra::curves::mnt::mnt6::mnt6_pp;
// use crate::common::profiling;
// use crate::common::utils;
// use crate::algebra::curves::bn128::bn128_pp;



// use crate::algebra::curves::alt_bn128::alt_bn128_pp;
// use crate::algebra::curves::bls12_381/bls12_381_pp;


pub struct CurveGroupsTest{//::testing::Test

    pub fn init()
    {
        edwards_pp::init_public_params();
        mnt4_pp::init_public_params();
        mnt6_pp::init_public_params();
        alt_bn128_pp::init_public_params();
        bls12_381_pp::init_public_params();
// #ifdef CURVE_BN128 // BN128 has fancy dependencies so it may be disabled
        bn128_pp::init_public_params();

    }
}


pub fn  test_mixed_add()
{
    let  (mut base,mut  el)=(GroupT::zero(),GroupT::zero());
    el.to_special();
    let mut result = base.mixed_add(el);
    assert_eq!(result, base + el);

    base = GroupT::zero();
    el = GroupT::random_element();
    el.to_special();
    result = base.mixed_add(el);
    assert_eq!(result, base + el);

    base = GroupT::random_element();
    el = GroupT::zero();
    el.to_special();
    result = base.mixed_add(el);
    assert_eq!(result, base + el);

    base = GroupT::random_element();
    el = GroupT::random_element();
    el.to_special();
    result = base.mixed_add(el);
    assert_eq!(result, base + el);

    base = GroupT::random_element();
    el = base;
    el.to_special();
    result = base.mixed_add(el);
    assert_eq!(result, base.dbl());
}


pub fn  test_group()
{
    let mut  rand1 = bigint::<1>::from("76749407");
    let mut  rand2 = bigint::<1>::from("44410867");
    let mut  randsum = bigint::<1>::from("121160274");

    let mut zero = GroupT::zero();
    assert_eq!(zero, zero);
    let mut one = GroupT::one();
    assert_eq!(one, one);
    let mut two = bigint::<1>::from(2L) * GroupT::one();
    assert_eq!(two, two);
    let mut five = bigint::<1>::from(5L) * GroupT::one();

    let mut three = bigint::<1>::from(3L) * GroupT::one();
    let mut four = bigint::<1>::from(4L) * GroupT::one();

    assert_eq!(two+five, three+four);

    let mut a = random_element_non_zero_one::<GroupT>();
    let mut b = random_element_non_zero_one::<GroupT>();

    asset_ne!(one, zero);
    asset_ne!(a, zero);
    asset_ne!(a, one);
    asset_ne!(b, zero);
    asset_ne!(b, one);

    assert_eq!(a.dbl(), a + a);
    assert_eq!(b.dbl(), b + b);
    assert_eq!(one.add(two), three);
    assert_eq!(two.add(one), three);
    assert_eq!(a + b, b + a);
    assert_eq!(a - a, zero);
    assert_eq!(a - b, a + (-b));
    assert_eq!(a - b, (-b) + a);

    // handle special cases
    assert_eq!(zero + (-a), -a);
    assert_eq!(zero - a, -a);
    assert_eq!(a - zero, a);
    assert_eq!(a + zero, a);
    assert_eq!(zero + a, a);

    assert_eq!((a + b).dbl(), (a + b) + (b + a));
    assert_eq!(bigint::<1>::from("2") * (a + b), (a + b) + (b + a));

    assert_eq!(rand1 * a + rand2 * a, randsum * a);

    assert_eq!(GroupT::order() * a, zero);
    assert_eq!(GroupT::order() * one, zero);
    asset_ne!(GroupT::order() * a - a, zero);
    asset_ne!(GroupT::order() * one - one, zero);

    test_mixed_add::<GroupT>();
}


pub fn  test_mul_by_q()
{
    let mut a = GroupT::random_element();
    assert_eq!(GroupT::field_char() * a, a.mul_by_q());
}


pub fn  test_output()
{
    let mut g = GroupT::zero();

    // /* ate-pairing contained optimizations specific to the original curve that were breaking
    //    point addition with extremely small probability, so this code was run for 1000 times
    //    in case there was a missing carry. Since no problems were found, this is now reduced
    //    to only 10 times for quick testing. */
    for i in 0..10
    {
        let mut g_ser = reserialize(g);
        assert_eq!(g, g_ser);
        // Use a random point in next iteration
        g = GroupT::random_element();
    }
}

#[test]
 pub fn GroupTest()
{
    test_group::<G1<edwards_pp> >();
    test_group::<G2<edwards_pp> >();

    test_group::<G1<mnt4_pp> >();
    test_group::<G2<mnt4_pp> >();

    test_group::<G1<mnt6_pp> >();
    test_group::<G2<mnt6_pp> >();

    test_group::<G1<alt_bn128_pp> >();
    test_group::<G2<alt_bn128_pp> >();

    test_group::<G1<bls12_381_pp> >();
    test_group::<G2<bls12_381_pp> >();

// #ifdef CURVE_BN128       // BN128 has fancy dependencies so it may be disabled
    test_group::<G1<bn128_pp> >();
    test_group::<G2<bn128_pp> >();

}

#[test]
 pub fn OutputTest()
{
    test_output::<G1<edwards_pp> >();
    test_output::<G2<edwards_pp> >();

    test_output::<G1<mnt4_pp> >();
    test_output::<G2<mnt4_pp> >();

    test_output::<G1<mnt6_pp> >();
    test_output::<G2<mnt6_pp> >();

    test_output::<G1<alt_bn128_pp> >();
    test_output::<G2<alt_bn128_pp> >();

    test_output::<G1<bls12_381_pp> >();
    test_output::<G2<bls12_381_pp> >();

// #ifdef CURVE_BN128       // BN128 has fancy dependencies so it may be disabled
    test_output::<G1<bn128_pp> >();
    test_output::<G2<bn128_pp> >();

}

#[test]
 pub fn MulByQTest()
{
    test_mul_by_q::<G2<edwards_pp> >();
    test_mul_by_q::<G2<mnt4_pp> >();
    test_mul_by_q::<G2<mnt6_pp> >();
    test_mul_by_q::<G2<alt_bn128_pp> >();
    test_mul_by_q::<G2<bls12_381_pp> >();
}
