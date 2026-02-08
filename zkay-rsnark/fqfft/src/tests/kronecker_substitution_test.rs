use crate::polynomial_arithmetic::basic_operations::{
    _polynomial_addition, _polynomial_division, _polynomial_multiplication,
    _polynomial_multiplication_on_kronecker, _polynomial_subtraction,
};
use ffec::common::double::Double;
use crate::dbl_vec;




#[cfg(test)]
mod test {
    use super::*;

    //   template <T>
    pub struct KroneckerSubstitutionTest; //::testing::Test};
    //   type FieldT=::testing::Types<Double>; /* List Extend Here */
    //   TYPED_TEST_CASE(KroneckerSubstitutionTest, FieldT);
    type TypeParam = Double;

    #[test]
    pub fn StandardPolynomialMultiplication() {
        let a = dbl_vec![1, 2, 3, 1];
        let b = dbl_vec![1, 2, 1, 1];
        let mut c = vec![TypeParam::zero()];

        _polynomial_multiplication_on_kronecker(&mut c, &a, &b);

        let mut c_answer = vec![TypeParam::zero()];
        _polynomial_multiplication(&mut c_answer, &a, &b);

        for i in 0..c_answer.len() {
            assert!(c_answer[i] == c[i]);
        }
    }

    #[test]
    pub fn SquaredPolynomialMultiplication() {
        let a = dbl_vec![1, 2, 3, 1];
        let b = a.clone();
        let mut c = vec![TypeParam::zero()];

        _polynomial_multiplication_on_kronecker(&mut c, &a, &b);

        let mut c_answer = vec![TypeParam::zero()];
        _polynomial_multiplication(&mut c_answer, &a, &b);

        for i in 0..c_answer.len() {
            assert!(c_answer[i] == c[i]);
        }
    }
}
