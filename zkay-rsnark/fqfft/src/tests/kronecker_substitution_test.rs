/**
 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
 
//#include <vector>

//#include <gtest/gtest.h>
//#include <stdint.h>

use crate::polynomial_arithmetic::basic_operations;

//namespace libfqfft {

  template <T>
  pub struct KroneckerSubstitutionTest {//::testing::Test};
  type FieldT=::testing::Types<Double>; /* List Extend Here */
  TYPED_TEST_CASE(KroneckerSubstitutionTest, FieldT);

  TYPED_TEST(KroneckerSubstitutionTest, StandardPolynomialMultiplication) {

    Vec<TypeParam> a = { 1, 2, 3, 1 };
    Vec<TypeParam> b = { 1, 2, 1, 1 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_multiplication_on_kronecker(c, a, b);

    Vec<TypeParam> c_answer(1, TypeParam::zero());
    _polynomial_multiplication(c_answer, a, b);

    for i in 0..c_answer.len()
    {
      EXPECT_TRUE(c_answer[i] == c[i]);
    }
  }

  TYPED_TEST(KroneckerSubstitutionTest, SquaredPolynomialMultiplication) {
    
    Vec<TypeParam> a = { 1, 2, 3, 1 };
    Vec<TypeParam> b = a;
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_multiplication_on_kronecker(c, a, b);
    
    Vec<TypeParam> c_answer(1, TypeParam::zero());
    _polynomial_multiplication(c_answer, a, b);

    for i in 0..c_answer.len()
    {
      EXPECT_TRUE(c_answer[i] == c[i]);
    }
  }

//} // libfqfft
