use crate::evaluation_domain::domains::arithmetic_sequence_domain::arithmetic_sequence_domain;
use crate::evaluation_domain::domains::basic_radix2_domain::basic_radix2_domain;
use crate::evaluation_domain::domains::extended_radix2_domain::extended_radix2_domain;
use crate::evaluation_domain::domains::geometric_sequence_domain::geometric_sequence_domain;
use crate::evaluation_domain::domains::step_radix2_domain::step_radix2_domain;
use crate::evaluation_domain::evaluation_domain::EvaluationDomainConfig;
use crate::evaluation_domain::evaluation_domain::EvaluationDomainType;
use crate::polynomial_arithmetic::naive_evaluate::evaluate_lagrange_polynomial;
use crate::polynomial_arithmetic::naive_evaluate::evaluate_polynomial;
use ffec::common::double::Double;
use rccell::RcCell;
use crate::dbl_vec;



/**
 * Note: Templatized type referenced with TypeParam (instead of canonical FieldT)
// https://github.com/google/googletest/blob/main/docs/advanced.md#type-parameterized-tests
 */
#[cfg(test)]
mod test {
    use super::*;
    pub struct EvaluationDomainTest {
        // <T>
        //::testing::Test

        //    pub fn  SetUp() {
        //     // mnt4_pp::init_public_params();
        //   }
    }

    //   type FieldT=::testing::Types<Fr<mnt4_pp>, Double>; /* List Extend Here */
    //   TYPED_TEST_CASE(EvaluationDomainTest, FieldT);
    type TypeParam = Double;
    #[test]
    pub fn FFT() {
        let m = 4;
        let f = dbl_vec![2, 5, 3, 8];

        let mut domain = RcCell::new(EvaluationDomainType::<TypeParam>::default());
        for key in 0..5 {
            if key == 0 {
                domain = RcCell::new(basic_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 1 {
                domain = RcCell::new(extended_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 2 {
                domain = RcCell::new(step_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 3 {
                domain = RcCell::new(
                    geometric_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            } else if key == 4 {
                domain = RcCell::new(
                    arithmetic_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            }

            let mut a = f.clone();
            domain.borrow_mut().FFT(&mut a);

            let mut idx = vec![TypeParam::zero(); m];
            for i in 0..m {
                idx[i] = domain.borrow_mut().get_domain_element(i);
            }

            for i in 0..m {
                let e = evaluate_polynomial(m, &f, &idx[i]).unwrap();
                assert_eq!(e, a[i]);
            }
            //   }
            //   catch(DomainSizeException &e)
            //   {
            //     print!("{} - skipping\n", e.what());
            //   }
            //   catch(InvalidSizeException &e)
            //   {
            //     print!("{} - skipping\n", e.what());
            //   }
        }
    }

    #[test]
    pub fn InverseFFTofFFT() {
        let m = 4;
        let f = dbl_vec![2, 5, 3, 8];

        let mut domain = RcCell::new(EvaluationDomainType::<TypeParam>::default());
        for key in 0..5 {
            if key == 0 {
                domain = RcCell::new(basic_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 1 {
                domain = RcCell::new(extended_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 2 {
                domain = RcCell::new(step_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 3 {
                domain = RcCell::new(
                    geometric_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            } else if key == 4 {
                domain = RcCell::new(
                    arithmetic_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            }

            let mut a = f.clone();
            domain.borrow_mut().FFT(&mut a);
            domain.borrow_mut().iFFT(&mut a);

            for i in 0..m {
                assert_eq!(f[i], a[i]);
            }
        }
    }

    #[test]
    pub fn InverseCosetFFTofCosetFFT() {
        let m = 4;
        let f = dbl_vec![2, 5, 3, 8];

        let coset = TypeParam::multiplicative_generator();

        let mut domain = RcCell::new(EvaluationDomainType::<TypeParam>::default());
        for key in 0..3 {
            if key == 0 {
                domain = RcCell::new(basic_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 1 {
                domain = RcCell::new(extended_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 2 {
                domain = RcCell::new(step_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 3 {
                domain = RcCell::new(
                    geometric_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            } else if key == 4 {
                domain = RcCell::new(
                    arithmetic_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            }

            let mut a = f.clone();
            domain.borrow_mut().cosetFFT(&mut a, &coset);
            domain.borrow_mut().icosetFFT(&mut a, &coset);

            for i in 0..m {
                assert_eq!(f[i], a[i]);
            }
        }
    }

    #[test]
    pub fn LagrangeCoefficients() {
        let m = 8;
        let t = TypeParam::from(10);

        let mut domain = RcCell::new(EvaluationDomainType::<TypeParam>::default());
        for key in 0..5 {
            if key == 0 {
                domain = RcCell::new(basic_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 1 {
                domain = RcCell::new(extended_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 2 {
                domain = RcCell::new(step_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 3 {
                domain = RcCell::new(
                    geometric_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            } else if key == 4 {
                domain = RcCell::new(
                    arithmetic_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            }

            let a = domain.borrow_mut().evaluate_all_lagrange_polynomials(&t);

            let mut d = vec![TypeParam::zero(); m];
            for i in 0..m {
                d[i] = domain.borrow_mut().get_domain_element(i);
            }

            for i in 0..m {
                let e: TypeParam = evaluate_lagrange_polynomial(m, &d, &t, i).unwrap().into();
                print!("{} == {}\n", e.as_ulong(), a[i].as_ulong());
                assert_eq!(e, a[i]);
            }
        }
    }

    #[test]
    pub fn ComputeZ() {
        let m = 8;
        let t = TypeParam::from(10);

        let mut domain = RcCell::new(EvaluationDomainType::<TypeParam>::default());
        for key in 0..5 {
            if key == 0 {
                domain = RcCell::new(basic_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 1 {
                domain = RcCell::new(extended_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 2 {
                domain = RcCell::new(step_radix2_domain::<TypeParam>::new(m).unwrap().into());
            } else if key == 3 {
                domain = RcCell::new(
                    geometric_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            } else if key == 4 {
                domain = RcCell::new(
                    arithmetic_sequence_domain::<TypeParam>::new(m)
                        .unwrap()
                        .into(),
                );
            }

            let a = domain.borrow_mut().compute_vanishing_polynomial(&t);

            let mut Z = TypeParam::one();
            for i in 0..m {
                Z *= (t.clone() - domain.borrow_mut().get_domain_element(i));
            }

            assert_eq!(Z, a);
        }
    }
}
