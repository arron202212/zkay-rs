use crate::polynomial_arithmetic::basic_operations::{
    _polynomial_addition, _polynomial_division, _polynomial_multiplication,
    _polynomial_multiplication_on_kronecker, _polynomial_subtraction,
};
use crate::polynomial_arithmetic::xgcd::_polynomial_xgcd;
use ffec::common::double::Double;
use crate::dbl_vec;



#[cfg(test)]
mod test {
    use super::*;
    //   template <T>
    pub struct PolynomialArithmeticTest; //::testing::Test};
    //   type FieldT=::testing::Types<Double>; /* List Extend Here */
    //   TYPED_TEST_CASE(PolynomialArithmeticTest, FieldT);
    type TypeParam = Double;
    #[test]
    pub fn PolynomialAdditionSame() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![9, 3, 11, 14, 7, 1, 5, 8];
        let mut c = vec![TypeParam::zero()];

        _polynomial_addition(&mut c, &a, &b);

        let c_ans = dbl_vec![10, 6, 15, 39, 13, 8, 12, 10];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialAdditionBiggerA() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![9, 3, 11, 14, 7];
        let mut c = vec![TypeParam::zero()];

        _polynomial_addition(&mut c, &a, &b);

        let c_ans = dbl_vec![10, 6, 15, 39, 13, 7, 7, 2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialAdditionBiggerB() {
        let a = dbl_vec![1, 3, 4, 25, 6];
        let b = dbl_vec![9, 3, 11, 14, 7, 1, 5, 8];
        let mut c = vec![TypeParam::zero()];

        _polynomial_addition(&mut c, &a, &b);

        let c_ans = dbl_vec![10, 6, 15, 39, 13, 1, 5, 8];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialAdditionZeroA() {
        let a = dbl_vec![0, 0, 0];
        let b = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let mut c = vec![TypeParam::zero()];

        _polynomial_addition(&mut c, &a, &b);

        let c_ans = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialAdditionZeroB() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![0, 0, 0];
        let mut c = vec![TypeParam::zero()];

        _polynomial_addition(&mut c, &a, &b);

        let c_ans = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialSubtractionSame() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![9, 3, 11, 14, 7, 1, 5, 8];
        let mut c = vec![TypeParam::zero()];

        _polynomial_subtraction(&mut c, &a, &b);

        let c_ans = dbl_vec![-8, 0, -7, 11, -1, 6, 2, -6];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialSubtractionBiggerA() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![9, 3, 11, 14, 7];
        let mut c = vec![TypeParam::zero()];

        _polynomial_subtraction(&mut c, &a, &b);

        let c_ans = dbl_vec![-8, 0, -7, 11, -1, 7, 7, 2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialSubtractionBiggerB() {
        let a = dbl_vec![1, 3, 4, 25, 6];
        let b = dbl_vec![9, 3, 11, 14, 7, 1, 5, 8];
        let mut c = vec![TypeParam::zero()];

        _polynomial_subtraction(&mut c, &a, &b);

        let c_ans = dbl_vec![-8, 0, -7, 11, -1, -1, -5, -8];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialSubtractionZeroA() {
        let a = dbl_vec![0, 0, 0];
        let b = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let mut c = vec![TypeParam::zero()];

        _polynomial_subtraction(&mut c, &a, &b);

        let c_ans = dbl_vec![-1, -3, -4, -25, -6, -7, -7, -2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialSubtractionZeroB() {
        let a = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];
        let b = dbl_vec![0, 0, 0];
        let mut c = vec![TypeParam::zero()];

        _polynomial_subtraction(&mut c, &a, &b);

        let c_ans = dbl_vec![1, 3, 4, 25, 6, 7, 7, 2];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialMultiplicationBasic() {
        let a = dbl_vec![5, 0, 0, 13, 0, 1];
        let b = dbl_vec![13, 0, 1];
        let mut c = vec![TypeParam::zero()];

        _polynomial_multiplication(&mut c, &a, &b);

        let c_ans = dbl_vec![65, 0, 5, 169, 0, 26, 0, 1];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialMultiplicationZero() {
        let a = dbl_vec![5, 0, 0, 13, 0, 1];
        let b = dbl_vec![0];
        let mut c = vec![TypeParam::zero()];

        _polynomial_multiplication(&mut c, &a, &b);

        let c_ans = dbl_vec![0];

        for i in 0..c.len() {
            assert_eq!((c_ans[i]), c[i]);
        }
    }

    #[test]
    pub fn PolynomialDivision() {
        let a = dbl_vec![5, 0, 0, 13, 0, 1];
        let b = dbl_vec![13, 0, 1];

        let mut Q = vec![TypeParam::zero()];
        let mut R = vec![TypeParam::zero()];

        _polynomial_division(&mut Q, &mut R, &a, &b);

        let Q_ans = dbl_vec![0, 0, 0, 1];
        let R_ans = dbl_vec![5];

        for i in 0..Q.len() {
            assert_eq!((Q_ans[i]), Q[i]);
        }
        for i in 0..R.len() {
            assert_eq!((R_ans[i]), R[i]);
        }
    }

    #[test]
    pub fn ExtendedGCD() {
        let a = dbl_vec![0, 0, 0, 0, 1];
        let b = dbl_vec![1, -6, 11, -6];

        let mut pg = vec![TypeParam::zero()];
        let mut pu = vec![TypeParam::zero()];
        let mut pv = vec![TypeParam::zero()];

        _polynomial_xgcd(&a, &b, &mut pg, &mut pu, &mut pv);

        let pv_ans = dbl_vec![1, 6, 25, 90];

        for i in 0..pv.len() {
            assert_eq!((pv_ans[i]), pv[i]);
        }
    }
}
