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
use crate::polynomial_arithmetic::xgcd;

//namespace libfqfft {

  template <T>
  pub struct PolynomialArithmeticTest {//::testing::Test};
  type FieldT=::testing::Types<Double>; /* List Extend Here */
  TYPED_TEST_CASE(PolynomialArithmeticTest, FieldT);

  TYPED_TEST(PolynomialArithmeticTest, PolynomialAdditionSame) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7, 1, 5, 8 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_addition(c, a, b);

    Vec<TypeParam> c_ans = { 10, 6, 15, 39, 13, 8, 12, 10 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialAdditionBiggerA) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_addition(c, a, b);

    Vec<TypeParam> c_ans = { 10, 6, 15, 39, 13, 7, 7, 2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialAdditionBiggerB) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7, 1, 5, 8 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_addition(c, a, b);

    Vec<TypeParam> c_ans = { 10, 6, 15, 39, 13, 1, 5, 8 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialAdditionZeroA) {

    Vec<TypeParam> a = { 0, 0, 0 };
    Vec<TypeParam> b = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_addition(c, a, b);

    Vec<TypeParam> c_ans = { 1, 3, 4, 25, 6, 7, 7, 2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialAdditionZeroB) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 0, 0, 0 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_addition(c, a, b);

    Vec<TypeParam> c_ans = { 1, 3, 4, 25, 6, 7, 7, 2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialSubtractionSame) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7, 1, 5, 8 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_subtraction(c, a, b);

    Vec<TypeParam> c_ans = { -8, 0, -7, 11, -1, 6, 2, -6 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialSubtractionBiggerA) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_subtraction(c, a, b);

    Vec<TypeParam> c_ans = { -8, 0, -7, 11, -1, 7, 7, 2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialSubtractionBiggerB) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6 };
    Vec<TypeParam> b = { 9, 3, 11, 14, 7, 1, 5, 8 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_subtraction(c, a, b);

    Vec<TypeParam> c_ans = { -8, 0, -7, 11, -1, -1, -5, -8 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialSubtractionZeroA) {

    Vec<TypeParam> a = { 0, 0, 0 };
    Vec<TypeParam> b = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_subtraction(c, a, b);

    Vec<TypeParam> c_ans = { -1, -3, -4, -25, -6, -7, -7, -2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialSubtractionZeroB) {

    Vec<TypeParam> a = { 1, 3, 4, 25, 6, 7, 7, 2 };
    Vec<TypeParam> b = { 0, 0, 0 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_subtraction(c, a, b);

    Vec<TypeParam> c_ans = { 1, 3, 4, 25, 6, 7, 7, 2 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialMultiplicationBasic) {

    Vec<TypeParam> a = { 5, 0, 0, 13, 0, 1 };
    Vec<TypeParam> b = { 13, 0, 1 };
    Vec<TypeParam> c(1, TypeParam::zero());
    
    _polynomial_multiplication(c, a, b);

    Vec<TypeParam> c_ans = { 65, 0, 5, 169, 0, 26, 0, 1 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialMultiplicationZero) {

    Vec<TypeParam> a = { 5, 0, 0, 13, 0, 1 };
    Vec<TypeParam> b = { 0 };
    Vec<TypeParam> c(1, TypeParam::zero());

    _polynomial_multiplication(c, a, b);

    Vec<TypeParam> c_ans = { 0 };

    for i in 0..c.len()
    {
      EXPECT_TRUE(c_ans[i] == c[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, PolynomialDivision) {

    Vec<TypeParam> a = { 5, 0, 0, 13, 0, 1 };
    Vec<TypeParam> b = { 13, 0, 1 };

    Vec<TypeParam> Q(1, TypeParam::zero());
    Vec<TypeParam> R(1, TypeParam::zero());

    _polynomial_division(Q, R, a, b);

    Vec<TypeParam> Q_ans = { 0, 0, 0, 1 };
    Vec<TypeParam> R_ans = { 5 };

    for i in 0..Q.len()
    {
      EXPECT_TRUE(Q_ans[i] == Q[i]);
    }
    for i in 0..R.len()
    {
      EXPECT_TRUE(R_ans[i] == R[i]);
    }
  }

  TYPED_TEST(PolynomialArithmeticTest, ExtendedGCD) {

    Vec<TypeParam> a = { 0, 0, 0, 0, 1 };
    Vec<TypeParam> b = { 1, -6, 11, -6 };

    Vec<TypeParam> pg(1, TypeParam::zero());
    Vec<TypeParam> pu(1, TypeParam::zero());
    Vec<TypeParam> pv(1, TypeParam::zero());

    _polynomial_xgcd(a, b, pg, pu, pv);

    Vec<TypeParam> pv_ans = { 1, 6, 25, 90 };

    for i in 0..pv.len()
    {
      EXPECT_TRUE(pv_ans[i] == pv[i]);
    }
  }

//} // libfqfft
