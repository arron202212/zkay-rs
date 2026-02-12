use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTS};
use crate::gadgetlib1::gadgets::curves::weierstrass_g1_gadget::G1_variable;
use crate::gadgetlib1::gadgets::curves::weierstrass_g2_gadget::{
    G2_variable, test_G2_checker_gadget,
};
use crate::gadgetlib1::gadgets::fields::exponentiation_gadget::test_exponentiation_gadget;
use crate::gadgetlib1::gadgets::fields::fp2_gadgets::{
    Fp2_mul_gadget, Fp2_sqr_gadget, Fp2_variable, Fp2TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp3_gadgets::{
    Fp3_mul_gadget, Fp3_sqr_gadget, Fp3_variable, Fp3TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp4_gadgets::{
    Fp4_cyclotomic_sqr_gadget, Fp4_mul_gadget, Fp4_sqr_gadget, Fp4_variable, Fp4TConfig,
};
use crate::gadgetlib1::gadgets::fields::fp6_gadgets::{
    Fp6_cyclotomic_sqr_gadget, Fp6_mul_gadget, Fp6_sqr_gadget, Fp6_variable, Fp6TConfig,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::pairing_selector;
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputations, G2_precomputations,
};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::knowledge_commitment::knowledge_commitment::knowledge_commitment;
use crate::zk_proof_systems::ppzksnark::ram_ppzksnark::ram_ppzksnark_params::RamPptConfig;
use ffec::One;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector1;
use std::fmt::Debug;

use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fqk_variable, MulTConfig, SqrTConfig, VariableTConfig, final_exp_gadget, other_curve, ppTConfig,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop::{
    mnt_miller_loop_gadget, test_mnt_e_over_e_miller_loop, test_mnt_e_times_e_over_e_miller_loop,
    test_mnt_miller_loop,
};
use crate::gadgetlib1::gadgets::pairing::weierstrass_precomputation::{
    G1_precomputation, G2_precomputation, precompute_G1_gadget, precompute_G2_gadget,
    test_G1_variable_precomp, test_G2_variable_precomp,
};
use crate::gadgetlib1::gadgets::verifiers::r1cs_ppzksnark_verifier_gadget::{
    r1cs_ppzksnark_online_verifier_gadget,
    r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable,
    r1cs_ppzksnark_proof_variable, r1cs_ppzksnark_verification_key_variable,
    r1cs_ppzksnark_verifier_gadget,
};
use crate::gadgetlib1::pb_variable::pb_linear_combination;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::generate_r1cs_example_with_field_input;
use crate::relations::variable::variable;
use crate::zk_proof_systems::ppzksnark::r1cs_ppzksnark::r1cs_ppzksnark::{
    r1cs_ppzksnark_generator, r1cs_ppzksnark_prover, r1cs_ppzksnark_verifier_strong_IC,
};
use ff_curves::{Fqk, Fr, G1, G2, PublicParams};
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::common::profiling::start_profiling;
use ffec::field_utils::bigint::bigint;
use rccell::RcCell;
use std::marker::PhantomData;
use std::ops::Mul;

pub fn dump_constraints<FieldT: FieldTConfig, PB: PBConfig>(pb: RcCell<protoboard<FieldT, PB>>) {
    // #ifdef DEBUG
    for s in &pb.borrow().constraint_system.constraint_annotations {
        print!("constraint: {}\n", s.1);
    }
}
struct test_verifierTester<ppT_A: ppTConfig, ppT_B: ppTConfig, T>(PhantomData<(ppT_A, ppT_B, T)>);
trait test_verifierConfig<
    ppT_A: ppTConfig<FieldT = Self::FieldT_A>,
    ppT_B: ppTConfig<FieldT = Self::FieldT_B, P = ppT_B> + pairing_selector<other_curve_type = ppT_A>,
>
{
    // type BP: pairing_selector<other_curve_type = ppT_A>;
    type FieldT_A; //Fr<ppT_A>;
    type FieldT_B; //Fr<ppT_B>;
    fn test_verifier<PB: PBConfig>(annotation_A: &str, annotation_B: &str)
    where
        knowledge_commitment<
            <ppT_A as ff_curves::PublicParams>::G1,
            <ppT_A as ff_curves::PublicParams>::G1,
        >: Mul<
                <ppT_A as ff_curves::PublicParams>::Fr,
                Output = knowledge_commitment<
                    <ppT_A as ff_curves::PublicParams>::G1,
                    <ppT_A as ff_curves::PublicParams>::G1,
                >,
            >,
        knowledge_commitment<
            <ppT_A as ff_curves::PublicParams>::G2,
            <ppT_A as ff_curves::PublicParams>::G1,
        >: Mul<
                <ppT_A as ff_curves::PublicParams>::Fr,
                Output = knowledge_commitment<
                    <ppT_A as ff_curves::PublicParams>::G2,
                    <ppT_A as ff_curves::PublicParams>::G1,
                >,
            >,
        <Self as test_verifierConfig<ppT_A, ppT_B>>::FieldT_A: FieldTConfig,
        <Self as test_verifierConfig<ppT_A, ppT_B>>::FieldT_B: FieldTConfig,
    {
        let num_constraints = 50;
        let primary_input_size = 3;

        let example = generate_r1cs_example_with_field_input::<
            Self::FieldT_A,
            pb_variable,
            pb_linear_combination,
        >(num_constraints, primary_input_size);
        assert!(example.primary_input.len() == primary_input_size);

        assert!(
            example
                .constraint_system
                .is_satisfied(&example.primary_input, &example.auxiliary_input)
        );
        let keypair = r1cs_ppzksnark_generator::<ppT_A>(&example.constraint_system);
        let pi = r1cs_ppzksnark_prover::<ppT_A>(
            &keypair.pk,
            &example.primary_input,
            &example.auxiliary_input,
        );
        let bit =
            r1cs_ppzksnark_verifier_strong_IC::<ppT_A>(&keypair.vk, &example.primary_input, &pi);
        assert!(bit);

        let elt_size = Self::FieldT_A::size_in_bits();
        let primary_input_size_in_bits = elt_size * primary_input_size;
        let vk_size_in_bits =
            r1cs_ppzksnark_verification_key_variable::<ppT_B>::size_in_bits(primary_input_size);

        let mut pb = RcCell::new(protoboard::<Self::FieldT_B, <ppT_B as ppTConfig>::PB>::default());
        let mut vk_bits = pb_variable_array::<Self::FieldT_B, <ppT_B as ppTConfig>::PB>::default();
        vk_bits.allocate(&pb, vk_size_in_bits, "vk_bits");

        let mut primary_input_bits =
            pb_variable_array::<Self::FieldT_B, <ppT_B as ppTConfig>::PB>::default();
        primary_input_bits.allocate(&pb, primary_input_size_in_bits, "primary_input_bits");

        let mut proof = r1cs_ppzksnark_proof_variable::<ppT_B>::new(pb.clone(), "proof".to_owned());

        let mut vk = r1cs_ppzksnark_verification_key_variable::<ppT_B>::new(
            pb.clone(),
            vk_bits,
            primary_input_size,
            "vk".to_owned(),
        );

        let mut result = variable::<Self::FieldT_B, pb_variable>::default();
        result.allocate(&pb, "result");

        let mut verifier = r1cs_ppzksnark_verifier_gadget::<ppT_B>::new(
            pb.clone(),
            vk.clone(),
            primary_input_bits.clone(),
            elt_size,
            proof.clone(),
            result.clone(),
            "verifier".to_owned(),
        );

        PROFILE_CONSTRAINTS(&pb, "check that proofs lies on the curve");
        {
            proof.generate_r1cs_constraints();
        }
        verifier.generate_r1cs_constraints();

        let mut input_as_bits = vec![];
        for el in &example.primary_input {
            let v = convert_field_element_to_bit_vector1::<Self::FieldT_A>(el, elt_size);
            input_as_bits.extend(v);
        }

        primary_input_bits.fill_with_bits(&pb, &input_as_bits);

        vk.generate_r1cs_witness(&keypair.vk);
        proof.generate_r1cs_witness(&pi);
        verifier.generate_r1cs_witness();
        *pb.borrow_mut().val_ref(&result) = Self::FieldT_B::one();

        print!("positive test:\n");
        assert!(pb.borrow().is_satisfied());

        *pb.borrow_mut().val_ref(&primary_input_bits[0]) =
            Self::FieldT_B::one() - pb.borrow().val(&primary_input_bits[0]);
        verifier.generate_r1cs_witness();
        *pb.borrow_mut().val_ref(&result) = Self::FieldT_B::one();

        print!("negative test:\n");
        assert!(!pb.borrow().is_satisfied());
        PRINT_CONSTRAINT_PROFILING();
        print!(
            "number of constraints for verifier: {} (verifier is implemented in {} constraints and verifies {} proofs))\n",
            pb.borrow().num_constraints(),
            annotation_B,
            annotation_A
        );
    }
}
struct test_hardcoded_verifierTester<ppT_A: ppTConfig, ppT_B: ppTConfig, T>(
    PhantomData<(ppT_A, ppT_B, T)>,
);
trait test_hardcoded_verifierConfig<
    ppT_A: ppTConfig<FieldT = Self::FieldT_A>,
    ppT_B: ppTConfig<FieldT = Self::FieldT_B, P = ppT_B> + pairing_selector<other_curve_type = ppT_A>,
>
{
    // type BP: pairing_selector<other_curve_type = ppT_A>;
    type FieldT_A: FieldTConfig; // = Fr<ppT_A>;
    type FieldT_B: FieldTConfig; //= Fr<ppT_B>;
    fn test_hardcoded_verifier<PB: PBConfig>(annotation_A: &str, annotation_B: &str)
    where
        knowledge_commitment<
            <ppT_A as ff_curves::PublicParams>::G1,
            <ppT_A as ff_curves::PublicParams>::G1,
        >: Mul<
                <ppT_A as ff_curves::PublicParams>::Fr,
                Output = knowledge_commitment<
                    <ppT_A as ff_curves::PublicParams>::G1,
                    <ppT_A as ff_curves::PublicParams>::G1,
                >,
            >,
        knowledge_commitment<
            <ppT_A as ff_curves::PublicParams>::G2,
            <ppT_A as ff_curves::PublicParams>::G1,
        >: Mul<
                <ppT_A as ff_curves::PublicParams>::Fr,
                Output = knowledge_commitment<
                    <ppT_A as ff_curves::PublicParams>::G2,
                    <ppT_A as ff_curves::PublicParams>::G1,
                >,
            >,
        <Self as test_hardcoded_verifierConfig<ppT_A, ppT_B>>::FieldT_A: FieldTConfig,
    {
        let num_constraints = 50;
        let primary_input_size = 3;

        let mut example = generate_r1cs_example_with_field_input::<
            Self::FieldT_A,
            pb_variable,
            pb_linear_combination,
        >(num_constraints, primary_input_size);
        assert!(example.primary_input.len() == primary_input_size);

        assert!(
            example
                .constraint_system
                .is_satisfied(&example.primary_input, &example.auxiliary_input)
        );
        let keypair = r1cs_ppzksnark_generator::<ppT_A>(&example.constraint_system);
        let pi = r1cs_ppzksnark_prover::<ppT_A>(
            &keypair.pk,
            &example.primary_input,
            &example.auxiliary_input,
        );
        let bit =
            r1cs_ppzksnark_verifier_strong_IC::<ppT_A>(&keypair.vk, &example.primary_input, &pi);
        assert!(bit);

        let elt_size = Self::FieldT_A::size_in_bits();
        let primary_input_size_in_bits = elt_size * primary_input_size;

        let mut pb = RcCell::new(protoboard::<Self::FieldT_B, <ppT_B as ppTConfig>::PB>::default());
        let mut hardcoded_vk = r1cs_ppzksnark_preprocessed_r1cs_ppzksnark_verification_key_variable::<
            ppT_B,
        >::new(pb.clone(), keypair.vk, "hardcoded_vk".to_owned());
        let mut primary_input_bits =
            pb_variable_array::<Self::FieldT_B, <ppT_B as ppTConfig>::PB>::default();
        primary_input_bits.allocate(&pb, primary_input_size_in_bits, "primary_input_bits");

        let proof = r1cs_ppzksnark_proof_variable::<ppT_B>::new(pb.clone(), "proof".to_owned());

        let mut result = variable::<Self::FieldT_B, pb_variable>::default();
        result.allocate(&pb, "result");

        let mut online_verifier = r1cs_ppzksnark_online_verifier_gadget::<ppT_B>::new(
            pb.clone(),
            hardcoded_vk,
            primary_input_bits.clone(),
            elt_size,
            proof.clone(),
            result.clone(),
            "online_verifier".to_owned(),
        );

        PROFILE_CONSTRAINTS(&pb, "check that proofs lies on the curve");
        {
            proof.generate_r1cs_constraints();
        }
        online_verifier.generate_r1cs_constraints();

        let mut input_as_bits = vec![];
        for el in &example.primary_input {
            let v = convert_field_element_to_bit_vector1::<Self::FieldT_A>(el, elt_size);
            input_as_bits.extend(v);
        }

        primary_input_bits.fill_with_bits(&pb, &input_as_bits);

        proof.generate_r1cs_witness(&pi);
        online_verifier.generate_r1cs_witness();
        *pb.borrow_mut().val_ref(&result) = Self::FieldT_B::one();

        print!("positive test:\n");
        assert!(pb.borrow().is_satisfied());

        *pb.borrow_mut().val_ref(&primary_input_bits[0]) =
            Self::FieldT_B::one() - pb.borrow().val(&primary_input_bits[0]);
        online_verifier.generate_r1cs_witness();
        *pb.borrow_mut().val_ref(&result) = Self::FieldT_B::one();

        print!("negative test:\n");
        assert!(!pb.borrow().is_satisfied());
        PRINT_CONSTRAINT_PROFILING();
        print!(
            "number of constraints for verifier: {} (verifier is implemented in {} constraints and verifies {} proofs))\n",
            pb.borrow().num_constraints(),
            annotation_B,
            annotation_A
        );
    }
}

struct test_mulTester<
    FpExtT: ppTConfig<my_Fp = VarT::FieldT>,
    VarT: VariableTConfig,
    MulT: MulTConfig<FieldT = VarT::FieldT, PB = VarT::PB>,
    Fp6T,
    FieldT: FieldTConfig,
    PB: PBConfig,
    T,
>(PhantomData<(FpExtT, VarT, MulT, Fp6T, FieldT, PB, T)>);
trait test_mulConfig<
    VarT: VariableTConfig<FieldT = Self::FieldT>,
    MulT: MulTConfig<Fpk_variableT = VarT, FieldT = VarT::FieldT, PB = VarT::PB>,
    FpExtT: ppTConfig,
>
{
    type FieldT: FieldTConfig = FpExtT::my_Fp;
    fn test_mul<PB: PBConfig>(annotation: &str)
    where
        <VarT as VariableTConfig>::FpkT: PartialEq<FpExtT>,
    {
        let mut pb = RcCell::new(protoboard::<
            <VarT as VariableTConfig>::FieldT,
            <VarT as VariableTConfig>::PB,
        >::default());
        let mut x = VarT::new(pb.clone(), "x".to_owned());
        let mut y = VarT::new(pb.clone(), "y".to_owned());
        let mut xy = VarT::new(pb.clone(), "xy".to_owned());
        let mut mul = MulT::new(
            pb.clone(),
            x.clone(),
            y.clone(),
            xy.clone(),
            "mul".to_owned(),
        );
        mul.generate_r1cs_constraints();

        for i in 0..10 {
            let x_val = FpExtT::random_element();
            let y_val = FpExtT::random_element();
            x.generate_r1cs_witness(&x_val);
            y.generate_r1cs_witness(&y_val);
            mul.generate_r1cs_witness();
            let res = xy.get_element();
            assert!(res == x_val * y_val);
            assert!(pb.borrow().is_satisfied());
        }
        print!(
            "number of constraints for {}_mul = {}\n",
            annotation,
            pb.borrow().num_constraints()
        );
    }
}

struct test_sqrTester<
    FpExtT: ppTConfig,
    VarT: VariableTConfig,
    SqrT: SqrTConfig<FieldT = VarT::FieldT, PB = VarT::PB>,
    Fp4T,
    FieldT: FieldTConfig,
    PB: PBConfig,
    T,
>(PhantomData<(FpExtT, VarT, SqrT, Fp4T, FieldT, PB, T)>);
trait test_sqrConfig<
    VarT: VariableTConfig,
    SqrT: SqrTConfig<Fpk_variableT = VarT, FieldT = VarT::FieldT, PB = VarT::PB>,
    FpExtT: ppTConfig,
>
{
    type FieldT: FieldTConfig = <FpExtT as ppTConfig>::my_Fp;
    fn test_sqr<PB: PBConfig>(annotation: &str)
    where
        <VarT as VariableTConfig>::FpkT: PartialEq<FpExtT>,
    {
        let mut pb = RcCell::new(protoboard::<
            <VarT as VariableTConfig>::FieldT,
            <VarT as VariableTConfig>::PB,
        >::default());
        let mut x = VarT::new(pb.clone(), "x".to_owned());
        let mut xsq = VarT::new(pb.clone(), "xsq".to_owned());
        let mut sqr = SqrT::new(
            pb.clone(),
            RcCell::new(x.clone()),
            xsq.clone(),
            "sqr".to_owned(),
        );
        sqr.generate_r1cs_constraints();

        for i in 0..10 {
            let mut x_val = FpExtT::random_element();
            x.generate_r1cs_witness(&x_val);
            sqr.generate_r1cs_witness();
            let res = xsq.get_element();
            assert!(res == x_val.squared());
            assert!(pb.borrow().is_satisfied());
        }
        print!(
            "number of constraints for {}_sqr = {}\n",
            annotation,
            pb.borrow().num_constraints()
        );
    }
}
trait FpExtTConfig: ppTConfig {
    type my_Fp: FieldTConfig;
}
struct test_cyclotomic_sqrTester<
    ppT: ppTConfig,
    VarT: VariableTConfig,
    CycloSqrT,
    Fp4T,
    FieldT: FieldTConfig,
    PB: PBConfig,
    T,
>(PhantomData<(ppT, VarT, CycloSqrT, Fp4T, FieldT, PB, T)>);
trait test_cyclotomic_sqrConfig<
    ppT: ppTConfig + FpExtTConfig,
    VarT: VariableTConfig,
    CycloSqrT: SqrTConfig<Fpk_variableT = VarT, FieldT = VarT::FieldT, PB = VarT::PB>,
>
{
    type FpExtT: FpExtTConfig = ppT; // = Fqk<ppT>;
    type FieldT: FieldTConfig = <Self::FpExtT as FpExtTConfig>::my_Fp;
    fn test_cyclotomic_sqr<PB: PBConfig>(annotation: &str)
    where
        <VarT as VariableTConfig>::FpkT:
            PartialEq<<Self as test_cyclotomic_sqrConfig<ppT, VarT, CycloSqrT>>::FpExtT>,
        <VarT as VariableTConfig>::FpkT: Debug,
        <Self as test_cyclotomic_sqrConfig<ppT, VarT, CycloSqrT>>::FpExtT:
            From<<ppT as ppTConfig>::FieldT>,
    {
        let mut pb = RcCell::new(protoboard::<
            <VarT as VariableTConfig>::FieldT,
            <VarT as VariableTConfig>::PB,
        >::default());
        let mut x = VarT::new(pb.clone(), "x".to_owned());
        let mut xsq = VarT::new(pb.clone(), "xsq".to_owned());
        let mut sqr = CycloSqrT::new(
            pb.clone(),
            RcCell::new(x.clone()),
            xsq.clone(),
            "sqr".to_owned(),
        );
        sqr.generate_r1cs_constraints();

        for i in 0..10 {
            let mut x_val = <ppT as ppTConfig>::FieldT::random_element();
            x_val = ppT::final_exponentiation(&x_val);

            x.generate_r1cs_witness(&x_val);
            sqr.generate_r1cs_witness();
            let res = xsq.get_element();
            assert_eq!(res, Self::FpExtT::from(x_val.squared()));
            assert!(pb.borrow().is_satisfied());
        }
        print!(
            "number of constraints for {}_cyclotomic_sqr = {}\n",
            annotation,
            pb.borrow().num_constraints()
        );
    }
}

struct test_FrobeniusTester<
    FpExtT: ppTConfig + From<FieldT>,
    VarT: VariableTConfig,
    FieldT: FieldTConfig,
    PB: PBConfig,
    T,
>(PhantomData<(FpExtT, VarT, FieldT, PB, T)>);
trait test_FrobeniusConfig<FpExtT: ppTConfig, VarT: VariableTConfig> {
    type FieldT: FieldTConfig = FpExtT::my_Fp;
    fn test_Frobenius<PB: PBConfig>(annotation: &str)
    where
        <VarT as VariableTConfig>::FpkT: PartialEq<FpExtT>,
    {
        for i in 0..100 {
            let mut pb = RcCell::new(protoboard::<
                <VarT as VariableTConfig>::FieldT,
                <VarT as VariableTConfig>::PB,
            >::default());
            let mut x = VarT::new(pb.clone(), "x".to_owned());
            let mut x_frob = x.Frobenius_map(i);

            let x_val = FpExtT::random_element();
            x.generate_r1cs_witness(&x_val);
            x_frob.evaluate();
            let res = x_frob.get_element();
            assert!(res == x_val.Frobenius_map(i));
            assert!(pb.borrow().is_satisfied());
        }

        print!("Frobenius map for {} correct\n", annotation);
    }
}
struct test_full_pairingTester<ppT: ppTConfig, T>(PhantomData<(ppT, T)>);
trait test_full_pairingConfig<ppT: ppTConfig> {
    type FieldT: FieldTConfig = Fr<ppT>;
    fn test_full_pairing<PB: PBConfig>(annotation: &str) {
        let mut pb = RcCell::new(protoboard::<
            <ppT as ppTConfig>::FieldT,
            <ppT as ppTConfig>::PB,
        >::default());
        let mut P_val = Fr::<other_curve<ppT>>::random_element() * G1::<other_curve<ppT>>::one();
        let mut Q_val = Fr::<other_curve<ppT>>::random_element() * G2::<other_curve<ppT>>::one();

        let mut P = G1_variable::<ppT>::new(pb.clone(), "P".to_owned());
        let mut Q = G2_variable::<ppT>::new(pb.clone(), "Q".to_owned());
        let mut prec_P = G1_precomputations::<ppT>::default();
        let mut prec_Q = G2_precomputations::<ppT>::default();

        let mut compute_prec_P = precompute_G1_gadget::<ppT>::new(
            pb.clone(),
            P.clone(),
            prec_P.clone(),
            "compute_prec_P".to_owned(),
        );
        let mut compute_prec_Q = precompute_G2_gadget::<ppT>::new(
            pb.clone(),
            Q.clone(),
            prec_Q.clone(),
            "compute_prec_Q".to_owned(),
        );

        let mut miller_result = Fqk_variable::<ppT>::new(pb.clone(), "miller_result".to_owned());
        let mut miller = mnt_miller_loop_gadget::<ppT>::new(
            pb.clone(),
            prec_P.clone(),
            prec_Q.clone(),
            miller_result.clone(),
            "miller".to_owned(),
        );
        let mut result_is_one = variable::<<ppT as ppTConfig>::FieldT, pb_variable>::default();
        result_is_one.allocate(&pb, "result_is_one");
        let mut finexp = final_exp_gadget::<ppT>::new(
            pb.clone(),
            miller_result,
            result_is_one,
            "finexp".to_owned(),
        );

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

        P.generate_r1cs_witness(&P_val);
        compute_prec_P.generate_r1cs_witness();
        Q.generate_r1cs_witness(&Q_val);
        compute_prec_Q.generate_r1cs_witness();
        miller.generate_r1cs_witness();
        finexp.generate_r1cs_witness();
        assert!(pb.borrow().is_satisfied());

        let mut native_prec_P = other_curve::<ppT>::affine_ate_precompute_G1(&P_val);
        let mut native_prec_Q = other_curve::<ppT>::affine_ate_precompute_G2(&Q_val);
        let mut native_miller_result =
            other_curve::<ppT>::affine_ate_miller_loop(&native_prec_P, &native_prec_Q);

        let mut native_finexp_result =
            other_curve::<ppT>::final_exponentiation(&native_miller_result);
        print!("Must match:\n");
        finexp.t.result.borrow().get_element().print();
        native_finexp_result.print();

        assert_eq!(finexp.t.result.borrow().get_element(), native_finexp_result);

        print!(
            "number of constraints for full pairing (Fr is {})  = {}\n",
            annotation,
            pb.borrow().num_constraints()
        );
    }
}
struct test_full_precomputed_pairingTester<ppT: ppTConfig, T>(PhantomData<(ppT, T)>);
trait test_full_precomputed_pairingConfig<ppT: ppTConfig> {
    type FieldT: FieldTConfig = Fr<ppT>;
    fn test_full_precomputed_pairing<PB: PBConfig>(annotation: &str) {
        let mut pb = RcCell::new(protoboard::<
            <ppT as ppTConfig>::FieldT,
            <ppT as ppTConfig>::PB,
        >::default());
        let mut P_val = Fr::<other_curve<ppT>>::random_element() * G1::<other_curve<ppT>>::one();
        let mut Q_val = Fr::<other_curve<ppT>>::random_element() * G2::<other_curve<ppT>>::one();

        let mut prec_P =
            G1_precomputation::<ppT>::new(pb.clone(), P_val.clone(), "prec_P".to_owned());
        let mut prec_Q =
            G2_precomputation::<ppT>::new(pb.clone(), Q_val.clone(), "prec_Q".to_owned());

        let mut miller_result = Fqk_variable::<ppT>::new(pb.clone(), "miller_result".to_owned());
        let mut miller = mnt_miller_loop_gadget::<ppT>::new(
            pb.clone(),
            prec_P.clone(),
            prec_Q.clone(),
            miller_result.clone(),
            "miller".to_owned(),
        );
        let mut result_is_one = variable::<<ppT as ppTConfig>::FieldT, pb_variable>::default();
        result_is_one.allocate(&pb, "result_is_one");
        let mut finexp = final_exp_gadget::<ppT>::new(
            pb.clone(),
            miller_result.clone(),
            result_is_one.clone(),
            "finexp".to_owned(),
        );

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
        assert!(pb.borrow().is_satisfied());

        let mut native_prec_P = other_curve::<ppT>::affine_ate_precompute_G1(&P_val);
        let mut native_prec_Q = other_curve::<ppT>::affine_ate_precompute_G2(&Q_val);
        let mut native_miller_result =
            other_curve::<ppT>::affine_ate_miller_loop(&native_prec_P, &native_prec_Q);

        let mut native_finexp_result =
            other_curve::<ppT>::final_exponentiation(&native_miller_result);
        print!("Must match:\n");
        finexp.t.result.borrow().get_element().print();
        native_finexp_result.print();

        assert_eq!(finexp.t.result.borrow().get_element(), native_finexp_result);

        print!(
            "number of constraints for full precomputed pairing (Fr is {})  = {}\n",
            annotation,
            pb.borrow().num_constraints()
        );
    }
}
trait Fp2_variableConfig<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    VariableTConfig
{
}
trait Fp2_mul_gadgetConfig<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    MulTConfig
{
}

impl<
    mnt4_Fq2: ppTConfig<my_Fp = Fp2V::FieldT>,
    Fp2T: Fp2TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp2V: Fp2_variableConfig<Fp2T, FieldT, PB>,
    Fp2M: Fp2_mul_gadgetConfig<
            Fp2T,
            FieldT,
            PB,
            Fpk_variableT = Fp2V,
            FieldT = Fp2V::FieldT,
            PB = Fp2V::PB,
        >,
> test_mulConfig<Fp2V, Fp2M, mnt4_Fq2>
    for test_mulTester<mnt4_Fq2, Fp2V, Fp2M, Fp2T, FieldT, PB, mnt4_Fq2S>
{
}

trait Fp4_variableConfig<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    VariableTConfig
{
}
trait Fp4_mul_gadgetConfig<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    MulTConfig
{
}
impl<
    mnt4_Fq4: ppTConfig<my_Fp = Fp4V::FieldT>,
    Fp4T: Fp4TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp4V: Fp4_variableConfig<Fp4T, FieldT, PB>,
    Fp4M: Fp4_mul_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
> test_mulConfig<Fp4V, Fp4M, mnt4_Fq4>
    for test_mulTester<mnt4_Fq4, Fp4V, Fp4M, Fp4T, FieldT, PB, mnt4_Fq4S>
{
}

trait Fp3_variableConfig<Fp3T: Fp3TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    VariableTConfig
{
}
trait Fp3_mul_gadgetConfig<Fp3T: Fp3TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    MulTConfig
{
}
impl<
    mnt6_Fq3: ppTConfig<my_Fp = Fp3V::FieldT>,
    Fp3T: Fp3TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp3V: Fp3_variableConfig<Fp3T, FieldT, PB>,
    Fp3M: Fp3_mul_gadgetConfig<
            Fp3T,
            FieldT,
            PB,
            Fpk_variableT = Fp3V,
            FieldT = Fp3V::FieldT,
            PB = Fp3V::PB,
        >,
> test_mulConfig<Fp3V, Fp3M, mnt6_Fq3>
    for test_mulTester<mnt6_Fq3, Fp3V, Fp3M, Fp3T, FieldT, PB, mnt6_Fq3S>
{
}

trait Fp6_variableConfig<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    VariableTConfig
{
}
trait Fp6_mul_gadgetConfig<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    MulTConfig
{
}
impl<
    mnt6_Fq6: ppTConfig<my_Fp = Fp6V::FieldT>,
    Fp6T: Fp6TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp6V: Fp6_variableConfig<Fp6T, FieldT, PB>,
    Fp6M: Fp6_mul_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
> test_mulConfig<Fp6V, Fp6M, mnt6_Fq6>
    for test_mulTester<mnt6_Fq6, Fp6V, Fp6M, Fp6T, FieldT, PB, mnt6_Fq6S>
{
}

trait Fp2_sqr_gadgetConfig<Fp2T: Fp2TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}

impl<
    mnt4_Fq2: ppTConfig,
    Fp2T: Fp2TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp2V: Fp2_variableConfig<Fp2T, FieldT, PB>,
    Fp2S: Fp2_sqr_gadgetConfig<
            Fp2T,
            FieldT,
            PB,
            Fpk_variableT = Fp2V,
            FieldT = Fp2V::FieldT,
            PB = Fp2V::PB,
        >,
> test_sqrConfig<Fp2V, Fp2S, mnt4_Fq2>
    for test_sqrTester<mnt4_Fq2, Fp2V, Fp2S, Fp2T, FieldT, PB, mnt4_Fq2S>
{
}

trait Fp4_sqr_gadgetConfig<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}
impl<
    mnt4_Fq4: ppTConfig,
    Fp4T: Fp4TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp4V: Fp4_variableConfig<Fp4T, FieldT, PB>,
    Fp4S: Fp4_sqr_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
> test_sqrConfig<Fp4V, Fp4S, mnt4_Fq4>
    for test_sqrTester<mnt4_Fq4, Fp4V, Fp4S, Fp4T, FieldT, PB, mnt4_Fq4S>
{
}
trait Fp3_sqr_gadgetConfig<Fp3T: Fp3TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}
impl<
    mnt6_Fq3: ppTConfig,
    Fp3T: Fp3TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp3V: Fp3_variableConfig<Fp3T, FieldT, PB>,
    Fp3S: Fp3_sqr_gadgetConfig<
            Fp3T,
            FieldT,
            PB,
            Fpk_variableT = Fp3V,
            FieldT = Fp3V::FieldT,
            PB = Fp3V::PB,
        >,
> test_sqrConfig<Fp3V, Fp3S, mnt6_Fq3>
    for test_sqrTester<mnt6_Fq3, Fp3V, Fp3S, Fp3T, FieldT, PB, mnt6_Fq3S>
{
}

trait Fp6_sqr_gadgetConfig<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}
impl<
    mnt6_Fq6: ppTConfig,
    Fp6T: Fp6TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp6V: Fp6_variableConfig<Fp6T, FieldT, PB>,
    Fp6S: Fp6_sqr_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
> test_sqrConfig<Fp6V, Fp6S, mnt6_Fq6>
    for test_sqrTester<mnt6_Fq6, Fp6V, Fp6S, Fp6T, FieldT, PB, mnt6_Fq6S>
{
}

trait Fp4_cyclotomic_sqr_gadgetConfig<Fp4T: Fp4TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}
impl<
    mnt4_pp: ppTConfig + FpExtTConfig,
    Fp4T: Fp4TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp4V: Fp4_variableConfig<Fp4T, FieldT, PB>,
    Fp4CS: Fp4_cyclotomic_sqr_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
> test_cyclotomic_sqrConfig<mnt4_pp, Fp4V, Fp4CS>
    for test_cyclotomic_sqrTester<mnt4_pp, Fp4V, Fp4CS, Fp4T, FieldT, PB, mnt4_Fq4S>
{
}
trait Fp6_cyclotomic_sqr_gadgetConfig<Fp6T: Fp6TConfig<FieldT>, FieldT: FieldTConfig, PB: PBConfig>:
    SqrTConfig
{
}
impl<
    mnt6_pp: ppTConfig + FpExtTConfig,
    Fp6T: Fp6TConfig<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp6V: Fp6_variableConfig<Fp6T, FieldT, PB>,
    Fp6CS: Fp6_cyclotomic_sqr_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
> test_cyclotomic_sqrConfig<mnt6_pp, Fp6V, Fp6CS>
    for test_cyclotomic_sqrTester<mnt6_pp, Fp6V, Fp6CS, Fp6T, FieldT, PB, mnt6_Fq6S>
{
}
impl<
    Fp4T: Fp4TConfig<FieldT> + ppTConfig + std::convert::From<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp4V: Fp4_variableConfig<Fp4T, FieldT, PB>,
> test_FrobeniusConfig<Fp4T, Fp4V> for test_FrobeniusTester<Fp4T, Fp4V, FieldT, PB, mnt4_Fq4S>
{
}

impl<
    Fp6T: Fp6TConfig<FieldT> + ppTConfig + std::convert::From<FieldT>,
    FieldT: FieldTConfig,
    PB: PBConfig,
    Fp6V: Fp6_variableConfig<Fp6T, FieldT, PB>,
> test_FrobeniusConfig<Fp6T, Fp6V> for test_FrobeniusTester<Fp6T, Fp6V, FieldT, PB, mnt6_Fq6S>
{
}
impl<mnt4_pp: ppTConfig> test_full_pairingConfig<mnt4_pp>
    for test_full_pairingTester<mnt4_pp, mnt4_ppS>
{
}
impl<mnt6_pp: ppTConfig> test_full_pairingConfig<mnt6_pp>
    for test_full_pairingTester<mnt6_pp, mnt6_ppS>
{
}
impl<mnt4_pp: ppTConfig> test_full_precomputed_pairingConfig<mnt4_pp>
    for test_full_precomputed_pairingTester<mnt4_pp, mnt4_ppS>
{
}
struct mnt4_Fq2S;
struct mnt4_Fq4S;
struct mnt6_Fq3S;
struct mnt6_Fq6S;
struct mnt4_ppS;
struct mnt6_ppS;
struct mnt64_ppS;
struct mnt46_ppS;
impl<mnt6_pp: ppTConfig> test_full_precomputed_pairingConfig<mnt6_pp>
    for test_full_precomputed_pairingTester<mnt6_pp, mnt6_ppS>
{
}
impl<
    mnt4_pp: ppTConfig<P = mnt4_pp> + pairing_selector<other_curve_type = mnt6_pp>,
    mnt6_pp: ppTConfig<P = mnt6_pp> + pairing_selector<other_curve_type = mnt4_pp>,
> test_verifierConfig<mnt4_pp, mnt6_pp> for test_verifierTester<mnt4_pp, mnt6_pp, mnt46_ppS>
{
    type FieldT_A = Fr<mnt4_pp>;
    type FieldT_B = Fr<mnt6_pp>;
}
impl<
    mnt6_pp: ppTConfig<P = mnt6_pp> + pairing_selector<other_curve_type = mnt4_pp>,
    mnt4_pp: ppTConfig<P = mnt4_pp> + pairing_selector<other_curve_type = mnt6_pp>,
> test_verifierConfig<mnt6_pp, mnt4_pp> for test_verifierTester<mnt6_pp, mnt4_pp, mnt64_ppS>
{
    type FieldT_A = Fr<mnt6_pp>;
    type FieldT_B = Fr<mnt4_pp>;
}

impl<
    mnt4_pp: ppTConfig<P = mnt4_pp> + pairing_selector<other_curve_type = mnt6_pp>,
    mnt6_pp: ppTConfig<P = mnt6_pp> + pairing_selector<other_curve_type = mnt4_pp>,
> test_hardcoded_verifierConfig<mnt4_pp, mnt6_pp>
    for test_hardcoded_verifierTester<mnt4_pp, mnt6_pp, mnt46_ppS>
{
    type FieldT_A = Fr<mnt4_pp>;
    type FieldT_B = Fr<mnt6_pp>;
}
impl<
    mnt6_pp: ppTConfig<P = mnt6_pp> + pairing_selector<other_curve_type = mnt4_pp>,
    mnt4_pp: ppTConfig<P = mnt4_pp> + pairing_selector<other_curve_type = mnt6_pp>,
> test_hardcoded_verifierConfig<mnt6_pp, mnt4_pp>
    for test_hardcoded_verifierTester<mnt6_pp, mnt4_pp, mnt64_ppS>
{
    type FieldT_A = Fr<mnt6_pp>;
    type FieldT_B = Fr<mnt4_pp>;
}

pub fn main<
    mnt4_pp: RamPptConfig
        + pairing_selector<other_curve_type = mnt6_pp>
        + ppTConfig<P = mnt4_pp>
        + FpExtTConfig
        + std::cmp::PartialEq<<mnt4_pp as ppTConfig>::FieldT>
        + std::cmp::PartialEq<<<mnt4_pp as ppTConfig>::P as pairing_selector>::Fqk_variable_type>
        + std::convert::From<<mnt4_pp as ppTConfig>::FieldT>,
    mnt6_pp: RamPptConfig
        + pairing_selector<other_curve_type = mnt4_pp>
        + ppTConfig<P = mnt6_pp>
        + FpExtTConfig
        + std::cmp::PartialEq<<mnt6_pp as ppTConfig>::FieldT>
        + std::cmp::PartialEq<<<mnt6_pp as ppTConfig>::P as pairing_selector>::Fqk_variable_type>
        + std::convert::From<<mnt6_pp as ppTConfig>::FieldT>,
    OCT6: pairing_selector<other_curve_type = mnt6_pp>,
    OCT4: pairing_selector<other_curve_type = mnt4_pp>,
    mnt4_Fq2: ppTConfig<my_Fp = Fp2V::FieldT>,
    mnt4_Fq4: Fp4TConfig<Fp6T>
        + ppTConfig<my_Fp = Fp4V::FieldT>
        + Fp4TConfig<FieldT>
        + std::convert::From<FieldT>,
    mnt6_Fq3: ppTConfig<my_Fp = Fp3V::FieldT>,
    mnt6_Fq6: Fp6TConfig<Fp6T>
        + ppTConfig<my_Fp = Fp6V::FieldT>
        + Fp6TConfig<FieldT>
        + std::convert::From<Fp6T>,
    const mnt4_q_limbs: usize,
    const mnt6_q_limbs: usize,
    PB: PBConfig,
    FieldT: FieldTConfig,
    Fp6T: Fp6TConfig<FieldT>,
    Fp4T: Fp4TConfig<FieldT>,
    Fp2T: Fp2TConfig<FieldT>,
    Fp3T: Fp3TConfig<FieldT>,
    Fp2V: Fp2_variableConfig<Fp2T, FieldT, PB>,
    Fp2M: Fp2_mul_gadgetConfig<
            Fp2T,
            FieldT,
            PB,
            Fpk_variableT = Fp2V,
            FieldT = Fp2V::FieldT,
            PB = Fp2V::PB,
        >,
    Fp2S: Fp2_sqr_gadgetConfig<
            Fp2T,
            FieldT,
            PB,
            Fpk_variableT = Fp2V,
            FieldT = Fp2V::FieldT,
            PB = Fp2V::PB,
        >,
    Fp6V: Fp6_variableConfig<Fp6T, FieldT, PB> + Fp6_variableConfig<mnt6_Fq6, Fp6T, PB>,
    Fp6M: Fp6_mul_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
    Fp6S: Fp6_sqr_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
    Fp6CS: Fp6_cyclotomic_sqr_gadgetConfig<
            Fp6T,
            FieldT,
            PB,
            Fpk_variableT = Fp6V,
            FieldT = Fp6V::FieldT,
            PB = Fp6V::PB,
        >,
    Fp4V: Fp4_variableConfig<Fp4T, FieldT, PB> + Fp4_variableConfig<mnt4_Fq4, FieldT, PB>,
    Fp4M: Fp4_mul_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
    Fp4S: Fp4_sqr_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
    Fp4CS: Fp4_cyclotomic_sqr_gadgetConfig<
            Fp4T,
            FieldT,
            PB,
            Fpk_variableT = Fp4V,
            FieldT = Fp4V::FieldT,
            PB = Fp4V::PB,
        >,
    Fp3V: Fp3_variableConfig<Fp3T, FieldT, PB>,
    Fp3M: Fp3_mul_gadgetConfig<
            Fp3T,
            FieldT,
            PB,
            Fpk_variableT = Fp3V,
            FieldT = Fp3V::FieldT,
            PB = Fp3V::PB,
        >,
    Fp3S: Fp3_sqr_gadgetConfig<
            Fp3T,
            FieldT,
            PB,
            Fpk_variableT = Fp3V,
            FieldT = Fp3V::FieldT,
            PB = Fp3V::PB,
        >,
>() -> i32
where
    knowledge_commitment<
        <mnt6_pp as ff_curves::PublicParams>::G1,
        <mnt6_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <mnt6_pp as ff_curves::PublicParams>::Fr,
            Output = knowledge_commitment<
                <mnt6_pp as ff_curves::PublicParams>::G1,
                <mnt6_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <mnt6_pp as ff_curves::PublicParams>::G2,
        <mnt6_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <mnt6_pp as ff_curves::PublicParams>::Fr,
            Output = knowledge_commitment<
                <mnt6_pp as ff_curves::PublicParams>::G2,
                <mnt6_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <mnt4_pp as ff_curves::PublicParams>::G1,
        <mnt4_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <mnt4_pp as ff_curves::PublicParams>::Fr,
            Output = knowledge_commitment<
                <mnt4_pp as ff_curves::PublicParams>::G1,
                <mnt4_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    knowledge_commitment<
        <mnt4_pp as ff_curves::PublicParams>::G2,
        <mnt4_pp as ff_curves::PublicParams>::G1,
    >: Mul<
            <mnt4_pp as ff_curves::PublicParams>::Fr,
            Output = knowledge_commitment<
                <mnt4_pp as ff_curves::PublicParams>::G2,
                <mnt4_pp as ff_curves::PublicParams>::G1,
            >,
        >,
    <Fp6V as VariableTConfig>::FpkT: PartialEq<mnt6_Fq6>,
    <Fp3V as VariableTConfig>::FpkT: PartialEq<mnt6_Fq3>,
    <Fp4V as VariableTConfig>::FpkT: PartialEq<mnt4_Fq4>,
    <Fp2V as VariableTConfig>::FpkT: PartialEq<mnt4_Fq2>,
    <Fp6V as VariableTConfig>::FpkT: Debug,
    <Fp6V as VariableTConfig>::FpkT: PartialEq<mnt6_pp>,
    <Fp4V as VariableTConfig>::FpkT: Debug,
    <Fp4V as VariableTConfig>::FpkT: PartialEq<mnt4_pp>,
{
    start_profiling();
    <mnt4_pp as RamPptConfig>::init_public_params();
    <mnt6_pp as RamPptConfig>::init_public_params();

    test_mulTester::<mnt4_Fq2, Fp2V, Fp2M, Fp2T, FieldT, PB, mnt4_Fq2S>::test_mul::<PB>("mnt4_Fp2");

    test_sqrTester::<mnt4_Fq2, Fp2V, Fp2S, Fp2T, FieldT, PB, mnt4_Fq2S>::test_sqr::<PB>("mnt4_Fp2");

    test_mulTester::<mnt4_Fq4, Fp4V, Fp4M, Fp4T, FieldT, PB, mnt4_Fq4S>::test_mul::<PB>("mnt4_Fp4");

    test_sqrTester::<mnt4_Fq4, Fp4V, Fp4S, Fp4T, FieldT, PB, mnt4_Fq4S>::test_sqr::<PB>("mnt4_Fp4");

    test_cyclotomic_sqrTester::<
        mnt4_pp,
        Fp4V,
        Fp4CS,Fp4T,FieldT, PB,
        mnt4_Fq4S,
    >::test_cyclotomic_sqr::<PB>("mnt4_Fp4");
    let mnt4_final_exponent_last_chunk_abs_of_w0 = bigint::<mnt4_q_limbs>::default();
    test_exponentiation_gadget::<
        mnt4_Fq4,
        // Fp4_variable,
        // Fp4_mul_gadget,
        // Fp4_sqr_gadget,
        mnt4_q_limbs,
    >(&mnt4_final_exponent_last_chunk_abs_of_w0, "mnt4_Fq4");

    test_FrobeniusTester::<mnt4_Fq4, Fp4V, FieldT, PB, mnt4_Fq4S>::test_Frobenius::<PB>("mnt4_Fq4");

    test_mulTester::<mnt6_Fq3, Fp3V, Fp3M, Fp3T, FieldT, PB, mnt6_Fq3S>::test_mul::<PB>("mnt6_Fp3");

    test_sqrTester::<mnt6_Fq3, Fp3V, Fp3S, Fp3T, FieldT, PB, mnt6_Fq3S>::test_sqr::<PB>("mnt6_Fp3");

    test_mulTester::<mnt6_Fq6, Fp6V, Fp6M, Fp6T, FieldT, PB, mnt6_Fq6S>::test_mul::<PB>("mnt6_Fp6");

    test_sqrTester::<mnt6_Fq6, Fp6V, Fp6S, Fp6T, FieldT, PB, mnt6_Fq6S>::test_sqr::<PB>("mnt6_Fp6");

    test_cyclotomic_sqrTester::<
        mnt6_pp,
        Fp6V,
        Fp6CS,Fp6T,FieldT,PB,
        mnt6_Fq6S,
    >::test_cyclotomic_sqr::<PB>("mnt6_Fp6");
    let mnt6_final_exponent_last_chunk_abs_of_w0 = bigint::<mnt6_q_limbs>::default();
    test_exponentiation_gadget::<
        mnt6_Fq6,
        // Fp6_variable,
        // Fp6_mul_gadget,
        // Fp6_sqr_gadget,
        mnt6_q_limbs,
    >(&mnt6_final_exponent_last_chunk_abs_of_w0, "mnt6_Fq6");

    test_FrobeniusTester::<mnt6_Fq6, Fp6V, Fp6T, PB, mnt6_Fq6S>::test_Frobenius::<PB>("mnt6_Fq6");

    test_G2_checker_gadget::<mnt4_pp>("mnt4");
    test_G2_checker_gadget::<mnt6_pp>("mnt6");

    test_G1_variable_precomp::<mnt4_pp>("mnt4");
    test_G1_variable_precomp::<mnt6_pp>("mnt6");

    test_G2_variable_precomp::<mnt4_pp>("mnt4");
    test_G2_variable_precomp::<mnt6_pp>("mnt6");

    test_mnt_miller_loop::<mnt4_pp>("mnt4");
    test_mnt_miller_loop::<mnt6_pp>("mnt6");

    test_mnt_e_over_e_miller_loop::<mnt4_pp>("mnt4");
    test_mnt_e_over_e_miller_loop::<mnt6_pp>("mnt6");

    test_mnt_e_times_e_over_e_miller_loop::<mnt4_pp>("mnt4");
    test_mnt_e_times_e_over_e_miller_loop::<mnt6_pp>("mnt6");

    test_full_pairingTester::<mnt4_pp, mnt4_ppS>::test_full_pairing::<PB>("mnt4");
    test_full_pairingTester::<mnt6_pp, mnt6_ppS>::test_full_pairing::<PB>("mnt6");

    test_full_precomputed_pairingTester::<mnt4_pp, mnt4_ppS>::test_full_precomputed_pairing::<PB>(
        "mnt4",
    );
    test_full_precomputed_pairingTester::<mnt6_pp, mnt6_ppS>::test_full_precomputed_pairing::<PB>(
        "mnt6",
    );

    test_verifierTester::<mnt4_pp, mnt6_pp, mnt46_ppS>::test_verifier::<PB>("mnt4", "mnt6");
    test_verifierTester::<mnt6_pp, mnt4_pp, mnt64_ppS>::test_verifier::<PB>("mnt6", "mnt4");

    test_hardcoded_verifierTester::<mnt4_pp, mnt6_pp, mnt46_ppS>::test_hardcoded_verifier::<PB>(
        "mnt4", "mnt6",
    );
    test_hardcoded_verifierTester::<mnt6_pp, mnt4_pp, mnt64_ppS>::test_hardcoded_verifier::<PB>(
        "mnt6", "mnt4",
    );
    0
}
