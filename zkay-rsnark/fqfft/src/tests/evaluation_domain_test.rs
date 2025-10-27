/**
 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#include <memory>
//#include <vector>

//#include <gtest/gtest.h>
use crate::algebra::curves::mnt::mnt4::mnt4_pp;
//#include <stdint.h>

use crate::evaluation_domain::domains::arithmetic_sequence_domain;
use crate::evaluation_domain::domains::basic_radix2_domain;
use crate::evaluation_domain::domains::extended_radix2_domain;
use crate::evaluation_domain::domains::geometric_sequence_domain;
use crate::evaluation_domain::domains::step_radix2_domain;
use crate::polynomial_arithmetic::naive_evaluate;
use crate::tools::exceptions;

//namespace libfqfft {

  /**
   * Note: Templatized type referenced with TypeParam (instead of canonical FieldT)
   * https://github.com/google/googletest/blob/master/googletest/docs/AdvancedGuide.md#typed-tests
   */
  template <T>
  pub struct EvaluationDomainTest {//::testing::Test
    
      virtual pub fn  SetUp() {
        mnt4_pp::init_public_params();
      }
  };

  type FieldT=::testing::Types<Fr<mnt4_pp>, Double>; /* List Extend Here */
  TYPED_TEST_CASE(EvaluationDomainTest, FieldT);

  TYPED_TEST(EvaluationDomainTest, FFT) {

    let m = 4;
    Vec<TypeParam> f = { 2, 5, 3, 8 };

    RcCell<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(basic_radix2_domain::<TypeParam>::new(m);
        else if key == 1) domain.reset(extended_radix2_domain::<TypeParam>::new(m);
        else if key == 2) domain.reset(step_radix2_domain::<TypeParam>::new(m);
        else if key == 3) domain.reset(geometric_sequence_domain::<TypeParam>::new(m);
        else if key == 4) domain.reset(arithmetic_sequence_domain::<TypeParam>::new(m);

        Vec<TypeParam> a(f);
        domain->FFT(a);

        Vec<TypeParam> idx(m);
        for i in 0..m
        {
          idx[i] = domain->get_domain_element(i);
        }

        for i in 0..m
        {
          TypeParam e = evaluate_polynomial(m, f, idx[i]);
          EXPECT_TRUE(e == a[i]);
        }
      }
      catch(DomainSizeException &e)
      {
        print!("{} - skipping\n", e.what());
      }
      catch(InvalidSizeException &e)
      {
        print!("{} - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, InverseFFTofFFT) {

    let m = 4;
    Vec<TypeParam> f = { 2, 5, 3, 8 };

    RcCell<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(basic_radix2_domain::<TypeParam>::new(m);
        else if key == 1) domain.reset(extended_radix2_domain::<TypeParam>::new(m);
        else if key == 2) domain.reset(step_radix2_domain::<TypeParam>::new(m);
        else if key == 3) domain.reset(geometric_sequence_domain::<TypeParam>::new(m);
        else if key == 4) domain.reset(arithmetic_sequence_domain::<TypeParam>::new(m);

        Vec<TypeParam> a(f);
        domain->FFT(a);
        domain->iFFT(a);

        for i in 0..m
        {
          EXPECT_TRUE(f[i] == a[i]);
        }
      }
      catch(e:&DomainSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
      catch(e:&InvalidSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, InverseCosetFFTofCosetFFT) {

    let m = 4;
    Vec<TypeParam> f = { 2, 5, 3, 8 };

    TypeParam coset = TypeParam::multiplicative_generator;

    RcCell<evaluation_domain<TypeParam> > domain;
    for key in 0..3
    {
      try
      {
        if key == 0) domain.reset(basic_radix2_domain::<TypeParam>::new(m);
        else if key == 1) domain.reset(extended_radix2_domain::<TypeParam>::new(m);
        else if key == 2) domain.reset(step_radix2_domain::<TypeParam>::new(m);
        else if key == 3) domain.reset(geometric_sequence_domain::<TypeParam>::new(m);
        else if key == 4) domain.reset(arithmetic_sequence_domain::<TypeParam>::new(m);

        Vec<TypeParam> a(f);
        domain->cosetFFT(a, coset);
        domain->icosetFFT(a, coset);

        for i in 0..m
        {
          EXPECT_TRUE(f[i] == a[i]);
        }
      }
      catch(e:&DomainSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
      catch(e:&InvalidSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, LagrangeCoefficients) {

    let m = 8;
    TypeParam t = TypeParam(10);

    RcCell<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {

      try
      {
        if key == 0) domain.reset(basic_radix2_domain::<TypeParam>::new(m);
        else if key == 1) domain.reset(extended_radix2_domain::<TypeParam>::new(m);
        else if key == 2) domain.reset(step_radix2_domain::<TypeParam>::new(m);
        else if key == 3) domain.reset(geometric_sequence_domain::<TypeParam>::new(m);
        else if key == 4) domain.reset(arithmetic_sequence_domain::<TypeParam>::new(m);

        Vec<TypeParam> a;
        a = domain->evaluate_all_lagrange_polynomials(t);

        Vec<TypeParam> d(m);
        for i in 0..m
        {
          d[i] = domain->get_domain_element(i);
        }

        for i in 0..m
        {
          TypeParam e = evaluate_lagrange_polynomial(m, d, t, i);
          print!("%ld == %ld\n", e.as_ulong(), a[i].as_ulong());
          EXPECT_TRUE(e == a[i]);
        }
      }
      catch(e:&DomainSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
      catch(e:&InvalidSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, ComputeZ) {

    let m = 8;
    TypeParam t = TypeParam(10);

    RcCell<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(basic_radix2_domain::<TypeParam>::new(m);
        else if key == 1) domain.reset(extended_radix2_domain::<TypeParam>::new(m);
        else if key == 2) domain.reset(step_radix2_domain::<TypeParam>::new(m);
        else if key == 3) domain.reset(geometric_sequence_domain::<TypeParam>::new(m);
        else if key == 4) domain.reset(arithmetic_sequence_domain::<TypeParam>::new(m);

        TypeParam a;
        a = domain->compute_vanishing_polynomial(t);

        TypeParam Z = TypeParam::one();
        for i in 0..m
        {
          Z *= (t - domain->get_domain_element(i));
        }

        EXPECT_TRUE(Z == a);
      }
      catch(e:&DomainSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
      catch(e:&InvalidSizeException)
      {
        print!("{} - skipping\n", e.what());
      }
    }
  }

//} // libfqfft
