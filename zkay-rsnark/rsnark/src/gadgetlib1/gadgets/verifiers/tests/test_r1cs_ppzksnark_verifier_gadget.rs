/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use ff_curves::algebra::curves::mnt::mnt4::mnt4_pp;
use ff_curves::algebra::curves::mnt::mnt6::mnt6_pp;
use ffec::algebra::field_utils::field_utils;

use crate::gadgetlib1::gadgets::fields::fp2_gadgets;
use crate::gadgetlib1::gadgets::fields::fp3_gadgets;
use crate::gadgetlib1::gadgets::fields::fp4_gadgets;
use crate::gadgetlib1::gadgets::fields::fp6_gadgets;
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark;




pub fn  dump_constraints( pb:RcCell<protoboard<FieldT>> )
{
// #ifdef DEBUG
    for s in &pb.constraint_system.constraint_annotations
    {
        print!("constraint: {}\n", s.second);
    }
//#endif
}


pub fn  test_verifier(annotation_A:&String, annotation_B:&String)
{
    type FieldT_A=ffec::Fr<ppT_A>;
    type FieldT_B=ffec::Fr<ppT_B>;

    let num_constraints = 50;
    let primary_input_size = 3;

    let  example = generate_r1cs_example_with_field_input::<FieldT_A>(num_constraints, primary_input_size);
    assert!(example.primary_input.len() == primary_input_size);

    assert!(example.constraint_system.is_satisfied(example.primary_input, example.auxiliary_input));
    let  keypair = r1cs_ppzksnark_generator::<ppT_A>(example.constraint_system);
    let  pi = r1cs_ppzksnark_prover::<ppT_A>(keypair.pk, example.primary_input, example.auxiliary_input);
    let  bit = r1cs_ppzksnark_verifier_strong_IC::<ppT_A>(keypair.vk, example.primary_input, pi);
    assert!(bit);

    let elt_size = FieldT_A::size_in_bits();
    let primary_input_size_in_bits = elt_size * primary_input_size;
    let vk_size_in_bits = r1cs_ppzksnark_verification_key_variable::<ppT_B>::size_in_bits(primary_input_size);

    let mut  pb=protoboard::<FieldT_B>::new();
    let mut  vk_bits=pb_variable_array::<FieldT_B>::new();
    vk_bits.allocate(&pb, vk_size_in_bits, "vk_bits");

    let mut  primary_input_bits=pb_variable_array::<FieldT_B>::new();
    primary_input_bits.allocate(&pb, primary_input_size_in_bits, "primary_input_bits");

    let mut proof=r1cs_ppzksnark_proof_variable::<ppT_B> ::new(pb, "proof");

   let mut  vk=r1cs_ppzksnark_verification_key_variable::<ppT_B> ::new(pb, vk_bits, primary_input_size, "vk");

    let mut result=pb_variable::<FieldT_B> ::new();
    result.allocate(&pb, "result");

    let mut  verifier=r1cs_ppzksnark_verifier_gadget::<ppT_B>::new(pb, vk, primary_input_bits, elt_size, proof, result, "verifier");

    PROFILE_CONSTRAINTS(&pb, "check that proofs lies on the curve");
    {
        proof.generate_r1cs_constraints();
    }
    verifier.generate_r1cs_constraints();

    let mut  input_as_bits=vec![];
    for el in &example.primary_input
    {
        let  v = ffec::convert_field_element_to_bit_vector::<FieldT_A>(el, elt_size);
        input_as_bits.insert(input_as_bits.end(), v.begin(), v.end());
    }

    primary_input_bits.fill_with_bits(&pb, input_as_bits);

    vk.generate_r1cs_witness(keypair.vk);
    proof.generate_r1cs_witness(pi);
    verifier.generate_r1cs_witness();
    pb.borrow().val(&result) = FieldT_B::one();

    print!("positive test:\n");
    assert!(pb.is_satisfied());

    pb.borrow().val(&primary_input_bits[0]) = FieldT_B::one() - pb.borrow().val(&primary_input_bits[0]);
    verifier.generate_r1cs_witness();
    pb.borrow().val(&result) = FieldT_B::one();

    print!("negative test:\n");
    assert!(!pb.is_satisfied());
    PRINT_CONSTRAINT_PROFILING();
    print!("number of constraints for verifier: {} (verifier is implemented in {} constraints and verifies {} proofs))\n",
           pb.num_constraints(), annotation_B, annotation_A);
}


pub fn  test_hardcoded_verifier(annotation_A:&String, annotation_B:&String)
{
    type FieldT_A=ffec::Fr<ppT_A>;
    type FieldT_B=ffec::Fr<ppT_B>;

    let num_constraints = 50;
    let primary_input_size = 3;

    let mut  example = generate_r1cs_example_with_field_input::<FieldT_A>(num_constraints, primary_input_size);
    assert!(example.primary_input.len() == primary_input_size);

    assert!(example.constraint_system.is_satisfied(example.primary_input, example.auxiliary_input));
    let  keypair = r1cs_ppzksnark_generator::<ppT_A>(example.constraint_system);
    let pi = r1cs_ppzksnark_prover::<ppT_A>(keypair.pk, example.primary_input, example.auxiliary_input);
    let bit = r1cs_ppzksnark_verifier_strong_IC::<ppT_A>(keypair.vk, example.primary_input, pi);
    assert!(bit);

    let elt_size = FieldT_A::size_in_bits();
    let primary_input_size_in_bits = elt_size * primary_input_size;

    let mut pb=protoboard::<FieldT_B> ::new();
    let mut  hardcoded_vk=r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<ppT_B>::new(pb, keypair.vk, "hardcoded_vk");
    let mut  primary_input_bits=pb_variable_array::<FieldT_B>::new;
    primary_input_bits.allocate(&pb, primary_input_size_in_bits, "primary_input_bits");

    let  proof=r1cs_ppzksnark_proof_variable::<ppT_B>::new(pb, "proof");

    let mut  result=pb_variable::<FieldT_B>::new();
    result.allocate(&pb, "result");

    let mut  online_verifier=r1cs_ppzksnark_online_verifier_gadget::<ppT_B>::new(pb, hardcoded_vk, primary_input_bits, elt_size, proof, result, "online_verifier");

    PROFILE_CONSTRAINTS(&pb, "check that proofs lies on the curve");
    {
        proof.generate_r1cs_constraints();
    }
    online_verifier.generate_r1cs_constraints();

    let mut  input_as_bits=vec![];
    for el in &example.primary_input
    {
        let  v = ffec::convert_field_element_to_bit_vector::<FieldT_A>(el, elt_size);
        input_as_bits.insert(input_as_bits.end(), v.begin(), v.end());
    }

    primary_input_bits.fill_with_bits(&pb, input_as_bits);

    proof.generate_r1cs_witness(pi);
    online_verifier.generate_r1cs_witness();
    pb.borrow().val(&result) = FieldT_B::one();

    print!("positive test:\n");
    assert!(pb.is_satisfied());

    pb.borrow().val(&primary_input_bits[0]) = FieldT_B::one() - pb.borrow().val(&primary_input_bits[0]);
    online_verifier.generate_r1cs_witness();
    pb.borrow().val(&result) = FieldT_B::one();

    print!("negative test:\n");
    assert!(!pb.is_satisfied());
    PRINT_CONSTRAINT_PROFILING();
    print!("number of constraints for verifier: {} (verifier is implemented in {} constraints and verifies {} proofs))\n",
           pb.num_constraints(), annotation_B, annotation_A);
}


pub fn  test_mul(annotation:&String)
{
    type FieldT=FpExtT::my_Fp;

    let mut  pb=protoboard::<FieldT> ::new();
     let mut  x=VarT::<FpExtT>::new(pb, "x");
     let mut  y=VarT::<FpExtT>::new(pb, "y");
     let mut  xy=VarT::<FpExtT>::new(pb, "xy");
     let mut  mul=MulT::<FpExtT>::new(pb, x, y, xy, "mul");
    mul.generate_r1cs_constraints();

    for i in 0..10
    {
        let x_val= FpExtT::random_element();
        let y_val= FpExtT::random_element();
        x.generate_r1cs_witness(x_val);
        y.generate_r1cs_witness(y_val);
        mul.generate_r1cs_witness();
        let res= xy.get_element();
        assert!(res == x_val*y_val);
        assert!(pb.is_satisfied());
    }
    print!("number of constraints for {}_mul = {}\n", annotation, pb.num_constraints());
}


pub fn  test_sqr(annotation:&String)
{
    type FieldT=FpExtT::my_Fp;

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  x=VarT::<FpExtT>::new(pb, "x");
    let mut  xsq=VarT::<FpExtT>::new(pb, "xsq");
    let mut  sqr=SqrT::<FpExtT>::new(pb, x, xsq, "sqr");
    sqr.generate_r1cs_constraints();

    for i in 0..10
    {
        let x_val= FpExtT::random_element();
        x.generate_r1cs_witness(x_val);
        sqr.generate_r1cs_witness();
        let res= xsq.get_element();
        assert!(res == x_val.squared());
        assert!(pb.is_satisfied());
    }
    print!("number of constraints for {}_sqr = {}\n", annotation, pb.num_constraints());
}


pub fn  test_cyclotomic_sqr(annotation:&String)
{
    type FpExtT=ffec::Fqk<ppT>;
    type FieldT=FpExtT::my_Fp;


    let mut  pb=protoboard::<FieldT> ::new();
    let mut x=VarT::<FpExtT>::new(pb, "x");
    let mut xsq=VarT::<FpExtT>::new(pb, "xsq");
    let mut sqr=CycloSqrT::<FpExtT>::new(pb, x, xsq, "sqr");
    sqr.generate_r1cs_constraints();

    for i in 0..10
    {
        let mut  x_val = FpExtT::random_element();
        x_val = ppT::final_exponentiation(x_val);

        x.generate_r1cs_witness(x_val);
        sqr.generate_r1cs_witness();
        let res= xsq.get_element();
        assert!(res == x_val.squared());
        assert!(pb.is_satisfied());
    }
    print!("number of constraints for {}_cyclotomic_sqr = {}\n", annotation, pb.num_constraints());
}


pub fn  test_Frobenius(annotation:&String)
{
    type FieldT=FpExtT::my_Fp;

    for i in 0..100
    {
        let mut  pb=protoboard::<FieldT> ::new();
        let mut x=VarT::<FpExtT>::new(pb, "x");
        let mut  x_frob = x.Frobenius_map(i);

        let x_val= FpExtT::random_element();
        x.generate_r1cs_witness(x_val);
        x_frob.evaluate();
        let res= x_frob.get_element();
        assert!(res == x_val.Frobenius_map(i));
        assert!(pb.is_satisfied());
    }

    print!("Frobenius map for {} correct\n", annotation);
}


pub fn  test_full_pairing(annotation:&String)
{
    type FieldT=ffec::Fr<ppT>;

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  P_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
    let mut  Q_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

    let mut P=G1_variable::<ppT,P>::new(pb, "P");
    let mut Q=G2_variable::<ppT>::new(pb, "Q");
    let mut prec_P=G1_precomputation::<ppT>::new();
    let mut prec_Q=G2_precomputation::<ppT>::new();

    let mut compute_prec_P=precompute_G1_gadget::<ppT>::new(pb, P, prec_P, "compute_prec_P");
    let mut compute_prec_Q=precompute_G2_gadget::<ppT>::new(pb, Q, prec_Q, "compute_prec_Q");

    let mut miller_result=Fqk_variable::<ppT>::new(pb, "miller_result");
    let mut miller=mnt_miller_loop_gadget::<ppT>::new(pb, prec_P, prec_Q, miller_result, "miller");
    let mut result_is_one=variable::<FieldT,pb_variable>::new();
    result_is_one.allocate(&pb, "result_is_one");
    let mut finexp=final_exp_gadget::<ppT>::new(pb, miller_result, result_is_one, "finexp");

    PROFILE_CONSTRAINTS(&pb, "precompute P");
    {
        compute_prec_P.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "precompute Q");
    {
        compute_prec_Q.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "Miller loop");
    {
        miller.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "final exp");
    {
        finexp.generate_r1cs_constraints();
    }
    PRINT_CONSTRAINT_PROFILING();

    P.generate_r1cs_witness(P_val);
    compute_prec_P.generate_r1cs_witness();
    Q.generate_r1cs_witness(Q_val);
    compute_prec_Q.generate_r1cs_witness();
    miller.generate_r1cs_witness();
    finexp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let mut  native_prec_P = other_curve::<ppT>::affine_ate_precompute_G1(P_val);
    let mut  native_prec_Q = other_curve::<ppT>::affine_ate_precompute_G2(Q_val);
    let mut  native_miller_result = other_curve::<ppT>::affine_ate_miller_loop(native_prec_P, native_prec_Q);

    let mut  native_finexp_result = other_curve::<ppT>::final_exponentiation(native_miller_result);
    print!("Must match:\n");
    finexp.result.get_element().print();
    native_finexp_result.print();

    assert!(finexp.result.get_element() == native_finexp_result);

    print!("number of constraints for full pairing (Fr is {})  = {}\n", annotation, pb.num_constraints());
}


pub fn  test_full_precomputed_pairing(annotation:&String)
{
    type FieldT=ffec::Fr<ppT>;

    let mut  pb=protoboard::<FieldT> ::new();
    let mut  P_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G1::<other_curve::<ppT> >::one();
    let mut  Q_val = ffec::Fr::<other_curve::<ppT> >::random_element() * ffec::G2::<other_curve::<ppT> >::one();

    let mut prec_P=G1_precomputation::<ppT>::new(pb, P_val, "prec_P");
    let mut prec_Q=G2_precomputation::<ppT>::new(pb, Q_val, "prec_Q");

    let mut miller_result=Fqk_variable::<ppT>::new(pb, "miller_result");
    let mut miller=mnt_miller_loop_gadget::<ppT>::new(pb, prec_P, prec_Q, miller_result, "miller");
    let mut result_is_one=variable::<FieldT,pb_variable>::new();
    result_is_one.allocate(&pb, "result_is_one");
    let mut finexp=final_exp_gadget::<ppT>::new(pb, miller_result, result_is_one, "finexp");

    PROFILE_CONSTRAINTS(&pb, "Miller loop");
    {
        miller.generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(&pb, "final exp");
    {
        finexp.generate_r1cs_constraints();
    }
    PRINT_CONSTRAINT_PROFILING();

    miller.generate_r1cs_witness();
    finexp.generate_r1cs_witness();
    assert!(pb.is_satisfied());

    let mut  native_prec_P = other_curve::<ppT>::affine_ate_precompute_G1(P_val);
    let mut  native_prec_Q = other_curve::<ppT>::affine_ate_precompute_G2(Q_val);
    let mut  native_miller_result = other_curve::<ppT>::affine_ate_miller_loop(native_prec_P, native_prec_Q);

    let mut  native_finexp_result = other_curve::<ppT>::final_exponentiation(native_miller_result);
    print!("Must match:\n");
    finexp.result.get_element().print();
    native_finexp_result.print();

    assert!(finexp.result.get_element() == native_finexp_result);

    print!("number of constraints for full precomputed pairing (Fr is {})  = {}\n", annotation, pb.num_constraints());
}

pub fn main()->i32
{
    ffec::start_profiling();
    ffec::mnt4_pp::init_public_params();
    ffec::mnt6_pp::init_public_params();

    test_mul::<ffec::mnt4_Fq2, Fp2_variable, Fp2_mul_gadget>("mnt4_Fp2");
    test_sqr::<ffec::mnt4_Fq2, Fp2_variable, Fp2_sqr_gadget>("mnt4_Fp2");

    test_mul::<ffec::mnt4_Fq4, Fp4_variable, Fp4_mul_gadget>("mnt4_Fp4");
    test_sqr::<ffec::mnt4_Fq4, Fp4_variable, Fp4_sqr_gadget>("mnt4_Fp4");
    test_cyclotomic_sqr::<ffec::mnt4_pp, Fp4_variable, Fp4_cyclotomic_sqr_gadget>("mnt4_Fp4");
    test_exponentiation_gadget::<ffec::mnt4_Fq4, Fp4_variable, Fp4_mul_gadget, Fp4_sqr_gadget, ffec::mnt4_q_limbs>(ffec::mnt4_final_exponent_last_chunk_abs_of_w0, "mnt4_Fq4");
    test_Frobenius::<ffec::mnt4_Fq4, Fp4_variable>("mnt4_Fq4");

    test_mul::<ffec::mnt6_Fq3, Fp3_variable, Fp3_mul_gadget>("mnt6_Fp3");
    test_sqr::<ffec::mnt6_Fq3, Fp3_variable, Fp3_sqr_gadget>("mnt6_Fp3");

    test_mul::<ffec::mnt6_Fq6, Fp6_variable, Fp6_mul_gadget>("mnt6_Fp6");
    test_sqr::<ffec::mnt6_Fq6, Fp6_variable, Fp6_sqr_gadget>("mnt6_Fp6");
    test_cyclotomic_sqr::<ffec::mnt6_pp, Fp6_variable, Fp6_cyclotomic_sqr_gadget>("mnt6_Fp6");
    test_exponentiation_gadget::<ffec::mnt6_Fq6, Fp6_variable, Fp6_mul_gadget, Fp6_sqr_gadget, ffec::mnt6_q_limbs>(ffec::mnt6_final_exponent_last_chunk_abs_of_w0, "mnt6_Fq6");
    test_Frobenius::<ffec::mnt6_Fq6, Fp6_variable>("mnt6_Fq6");

    test_G2_checker_gadget::<ffec::mnt4_pp>("mnt4");
    test_G2_checker_gadget::<ffec::mnt6_pp>("mnt6");

    test_G1_variable_precomp::<ffec::mnt4_pp>("mnt4");
    test_G1_variable_precomp::<ffec::mnt6_pp>("mnt6");

    test_G2_variable_precomp::<ffec::mnt4_pp>("mnt4");
    test_G2_variable_precomp::<ffec::mnt6_pp>("mnt6");

    test_mnt_miller_loop::<ffec::mnt4_pp>("mnt4");
    test_mnt_miller_loop::<ffec::mnt6_pp>("mnt6");

    test_mnt_e_over_e_miller_loop::<ffec::mnt4_pp>("mnt4");
    test_mnt_e_over_e_miller_loop::<ffec::mnt6_pp>("mnt6");

    test_mnt_e_times_e_over_e_miller_loop::<ffec::mnt4_pp>("mnt4");
    test_mnt_e_times_e_over_e_miller_loop::<ffec::mnt6_pp>("mnt6");

    test_full_pairing::<ffec::mnt4_pp>("mnt4");
    test_full_pairing::<ffec::mnt6_pp>("mnt6");

    test_full_precomputed_pairing::<ffec::mnt4_pp>("mnt4");
    test_full_precomputed_pairing::<ffec::mnt6_pp>("mnt6");

    test_verifier::<ffec::mnt4_pp, ffec::mnt6_pp>("mnt4", "mnt6");
    test_verifier::<ffec::mnt6_pp, ffec::mnt4_pp>("mnt6", "mnt4");

    test_hardcoded_verifier::<ffec::mnt4_pp, ffec::mnt6_pp>("mnt4", "mnt6");
    test_hardcoded_verifier::<ffec::mnt6_pp, ffec::mnt4_pp>("mnt6", "mnt4");
}
