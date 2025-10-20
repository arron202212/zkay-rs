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
  template <typename T>
  class EvaluationDomainTest : public ::testing::Test {
    protected:
      virtual void SetUp() {
        mnt4_pp::init_public_params();
      }
  };

  typedef ::testing::Types<Fr<mnt4_pp>, Double> FieldT; /* List Extend Here */
  TYPED_TEST_CASE(EvaluationDomainTest, FieldT);

  TYPED_TEST(EvaluationDomainTest, FFT) {

    const size_t m = 4;
    std::vector<TypeParam> f = { 2, 5, 3, 8 };

    std::shared_ptr<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(new basic_radix2_domain<TypeParam>(m);
        else if key == 1) domain.reset(new extended_radix2_domain<TypeParam>(m);
        else if key == 2) domain.reset(new step_radix2_domain<TypeParam>(m);
        else if key == 3) domain.reset(new geometric_sequence_domain<TypeParam>(m);
        else if key == 4) domain.reset(new arithmetic_sequence_domain<TypeParam>(m);

        std::vector<TypeParam> a(f);
        domain->FFT(a);

        std::vector<TypeParam> idx(m);
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
        print!("%s - skipping\n", e.what());
      }
      catch(InvalidSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, InverseFFTofFFT) {

    const size_t m = 4;
    std::vector<TypeParam> f = { 2, 5, 3, 8 };

    std::shared_ptr<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(new basic_radix2_domain<TypeParam>(m);
        else if key == 1) domain.reset(new extended_radix2_domain<TypeParam>(m);
        else if key == 2) domain.reset(new step_radix2_domain<TypeParam>(m);
        else if key == 3) domain.reset(new geometric_sequence_domain<TypeParam>(m);
        else if key == 4) domain.reset(new arithmetic_sequence_domain<TypeParam>(m);

        std::vector<TypeParam> a(f);
        domain->FFT(a);
        domain->iFFT(a);

        for i in 0..m
        {
          EXPECT_TRUE(f[i] == a[i]);
        }
      }
      catch(const DomainSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
      catch(const InvalidSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, InverseCosetFFTofCosetFFT) {

    const size_t m = 4;
    std::vector<TypeParam> f = { 2, 5, 3, 8 };

    TypeParam coset = TypeParam::multiplicative_generator;

    std::shared_ptr<evaluation_domain<TypeParam> > domain;
    for key in 0..3
    {
      try
      {
        if key == 0) domain.reset(new basic_radix2_domain<TypeParam>(m);
        else if key == 1) domain.reset(new extended_radix2_domain<TypeParam>(m);
        else if key == 2) domain.reset(new step_radix2_domain<TypeParam>(m);
        else if key == 3) domain.reset(new geometric_sequence_domain<TypeParam>(m);
        else if key == 4) domain.reset(new arithmetic_sequence_domain<TypeParam>(m);

        std::vector<TypeParam> a(f);
        domain->cosetFFT(a, coset);
        domain->icosetFFT(a, coset);

        for i in 0..m
        {
          EXPECT_TRUE(f[i] == a[i]);
        }
      }
      catch(const DomainSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
      catch(const InvalidSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, LagrangeCoefficients) {

    const size_t m = 8;
    TypeParam t = TypeParam(10);

    std::shared_ptr<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {

      try
      {
        if key == 0) domain.reset(new basic_radix2_domain<TypeParam>(m);
        else if key == 1) domain.reset(new extended_radix2_domain<TypeParam>(m);
        else if key == 2) domain.reset(new step_radix2_domain<TypeParam>(m);
        else if key == 3) domain.reset(new geometric_sequence_domain<TypeParam>(m);
        else if key == 4) domain.reset(new arithmetic_sequence_domain<TypeParam>(m);

        std::vector<TypeParam> a;
        a = domain->evaluate_all_lagrange_polynomials(t);

        std::vector<TypeParam> d(m);
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
      catch(const DomainSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
      catch(const InvalidSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
    }
  }

  TYPED_TEST(EvaluationDomainTest, ComputeZ) {

    const size_t m = 8;
    TypeParam t = TypeParam(10);

    std::shared_ptr<evaluation_domain<TypeParam> > domain;
    for key in 0..5
    {
      try
      {
        if key == 0) domain.reset(new basic_radix2_domain<TypeParam>(m);
        else if key == 1) domain.reset(new extended_radix2_domain<TypeParam>(m);
        else if key == 2) domain.reset(new step_radix2_domain<TypeParam>(m);
        else if key == 3) domain.reset(new geometric_sequence_domain<TypeParam>(m);
        else if key == 4) domain.reset(new arithmetic_sequence_domain<TypeParam>(m);

        TypeParam a;
        a = domain->compute_vanishing_polynomial(t);

        TypeParam Z = TypeParam::one();
        for i in 0..m
        {
          Z *= (t - domain->get_domain_element(i));
        }

        EXPECT_TRUE(Z == a);
      }
      catch(const DomainSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
      catch(const InvalidSizeException &e)
      {
        print!("%s - skipping\n", e.what());
      }
    }
  }

//} // libfqfft
