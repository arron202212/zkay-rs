



// use crate::algebra::curves::edwards::edwards_pp;
// use crate::common::profiling;
// use crate::algebra::curves::bn128::bn128_pp;
// use crate::algebra::curves::bn128::bn128_pp;

// use crate::algebra::curves::alt_bn128::alt_bn128_pp;
// use crate::algebra::curves::bls12_381/bls12_381_pp;
// use crate::algebra::curves::mnt::mnt4::mnt4_pp;
// use crate::algebra::curves::mnt::mnt6::mnt6_pp;



pub struct CurveBilinearityTest{//::testing::Test

    pub fn init()
    {
        start_profiling();
        edwards_pp::init_public_params();
        mnt4_pp::init_public_params();
        mnt6_pp::init_public_params();
        alt_bn128_pp::init_public_params();
        bls12_381_pp::init_public_params();
// #ifdef CURVE_BN128 // BN128 has fancy dependencies so it may be disabled
        bn128_pp::init_public_params();

    }
}


pub fn  pairing_test()
{
let mut GT_one:    GT<ppT>= GT<ppT>::one();

    print!("Running bilinearity tests:\n");
let mut P:    G1<ppT>= (Fr<ppT>::random_element()) * G1<ppT>::one();
let mut P:    //G1<ppT>= Fr<ppT>("2") * G1<ppT>::one();
let mut Q:    G2<ppT>= (Fr<ppT>::random_element()) * G2<ppT>::one();
let mut Q:    //G2<ppT>= Fr<ppT>("3") * G2<ppT>::one();

    print!("P:\n");
    P.print();
    P.print_coordinates();
    print!("Q:\n");
    Q.print();
    Q.print_coordinates();
    print!("\n\n");

let mut s:    Fr<ppT>= Fr<ppT>::random_element();
// let mut s:    Fr<ppT>= Fr<ppT>("2");
let mut sP:    G1<ppT>= s * P;
let mut sQ:    G2<ppT>= s * Q;

    print!("Pairing bilinearity tests (three must match):\n");
let mut ans1:    GT<ppT>= ppT::reduced_pairing(sP, Q);
let mut ans2:    GT<ppT>= ppT::reduced_pairing(P, sQ);
let mut ans3:    GT<ppT>= ppT::reduced_pairing(P, Q)^s;
    ans1.print();
    ans2.print();
    ans3.print();
    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, GT_one);
    assert_eq!(ans1^Fr<ppT>::field_char(), GT_one);
    print!("\n\n");
}


pub fn  double_miller_loop_test()
{
    let P1:G1<ppT>= (Fr<ppT>::random_element()) * G1<ppT>::one();
    let P2:G1<ppT>= (Fr<ppT>::random_element()) * G1<ppT>::one();
    let Q1:G2<ppT>= (Fr<ppT>::random_element()) * G2<ppT>::one();
    let Q2:G2<ppT>= (Fr<ppT>::random_element()) * G2<ppT>::one();

    let prec_P1:G1_precomp<ppT>= ppT::precompute_G1(P1);
    let prec_P2:G1_precomp<ppT>= ppT::precompute_G1(P2);
    let prec_Q1:G2_precomp<ppT>= ppT::precompute_G2(Q1);
    let prec_Q2:G2_precomp<ppT>= ppT::precompute_G2(Q2);

    let ans_1:Fqk<ppT>= ppT::miller_loop(prec_P1, prec_Q1);
    let ans_2:Fqk<ppT>= ppT::miller_loop(prec_P2, prec_Q2);
    let ans_12:Fqk<ppT>  = ppT::double_miller_loop(prec_P1,prec_Q1, prec_P2, prec_Q2);
    assert_eq!(ans_1 * ans_2, ans_12);
}


pub fn  affine_pairing_test()
{
let mut GT_one:    GT<ppT>= GT<ppT>::one();

    print!("Running bilinearity tests:\n");
let mut P:    G1<ppT>= (Fr<ppT>::random_element()) * G1<ppT>::one();
let mut Q:    G2<ppT>= (Fr<ppT>::random_element()) * G2<ppT>::one();

    print!("P:\n");
    P.print();
    print!("Q:\n");
    Q.print();
    print!("\n\n");

let mut s:    Fr<ppT>= Fr<ppT>::random_element();
let mut sP:    G1<ppT>= s * P;
let mut sQ:    G2<ppT>= s * Q;

    print!("Pairing bilinearity tests (three must match):\n");
let mut ans1:    GT<ppT>= ppT::affine_reduced_pairing(sP, Q);
let mut ans2:    GT<ppT>= ppT::affine_reduced_pairing(P, sQ);
let mut ans3:    GT<ppT>= ppT::affine_reduced_pairing(P, Q)^s;
    ans1.print();
    ans2.print();
    ans3.print();
    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, GT_one);
    assert_eq!(ans1^Fr<ppT>::field_char(), GT_one);
    print!("\n\n");
}

#[test]
 pub fn PairingTest()
{
    pairing_test::<edwards_pp>();
    pairing_test::<mnt6_pp>();
    pairing_test::<mnt4_pp>();
    pairing_test::<alt_bn128_pp>();
    pairing_test::<bls12_381_pp>();
// #ifdef CURVE_BN128       // BN128 has fancy dependencies so it may be disabled
    pairing_test::<bn128_pp>();

}

#[test]
 pub fn DoubleMillerLoopTest()
{
    double_miller_loop_test::<edwards_pp>();
    double_miller_loop_test::<mnt6_pp>();
    double_miller_loop_test::<mnt4_pp>();
    double_miller_loop_test::<alt_bn128_pp>();
    double_miller_loop_test::<bls12_381_pp>();
// #ifdef CURVE_BN128       // BN128 has fancy dependencies so it may be disabled
    double_miller_loop_test::<bn128_pp>();

}

#[test]
 pub fn AffinePairingTest()
{
    affine_pairing_test::<mnt6_pp>();
    affine_pairing_test::<mnt4_pp>();
}
