// Declaration of interfaces for the "basic radix-2" evaluation domain.
// Roughly, the domain has size m = 2^k and consists of the m-th roots of unity.

use crate::evaluation_domain::domains::basic_radix2_domain_aux::{
    _basic_radix2_FFT, _basic_radix2_evaluate_all_lagrange_polynomials, _multiply_by_coset,
};
use crate::evaluation_domain::evaluation_domain::{EvaluationDomainConfig, evaluation_domain};
use ffec::FieldTConfig;
use ffec::algebra::field_utils::field_utils;
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
use ffec::common::double;
use ffec::common::utils::log2;
use num_traits::One;
use std::ops::BitXor;
use std::ops::Sub;

#[derive(Default, Clone)]
pub struct basic_radix2_domain<FieldT: FieldTConfig> {
    // : public evaluation_domain<FieldT>
    pub omega: FieldT,
    pub m: usize,
}

//     basic_radix2_domain(m:usize);

//     pub fn  FFT(a:&Vec<FieldT>);
//     pub fn  iFFT(a:&Vec<FieldT>);
//     pub fn  cosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     pub fn  icosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(idx:usize);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     pub fn  add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>);
//     pub fn  divide_by_Z_on_coset(P:&Vec<FieldT>);

// };

//  std::default::Default
//         + std::ops::Sub
//         + One
//         + std::ops::MulAssign
//         + std::cmp::PartialEq
//         + Clone
//         + std::ops::Sub<Output = FieldT>
//         + std::ops::MulAssign
//         + std::ops::BitXor<Output = FieldT>
//         + std::convert::From<usize>
//         + num_traits::Zero
//         + std::ops::SubAssign
//         + std::ops::AddAssign<FieldT>,

pub type basic_radix2_domains<FieldT> = evaluation_domain<basic_radix2_domain<FieldT>>;
impl<FieldT: FieldTConfig> basic_radix2_domain<FieldT> {
    pub fn new(m: usize) -> eyre::Result<basic_radix2_domains<FieldT>> {
        // : evaluation_domain<FieldT>(m)
        if m <= 1 {
            eyre::bail!("basic_radix2(): expected m > 1");
        }

        if "FieldT" != "Double" {
            let logm = log2(m);
            // if logm > (FieldT::s){eyre::bail!("basic_radix2(): expected logm <= FieldT::s");}
        }

        Ok(evaluation_domain::<Self>::new(
            m,
            Self {
                omega: get_root_of_unity_is_same_double::<FieldT>(m),
                m,
            },
        ))
        // catch (const std::invalid_argument& e) { throw DomainSizeException(e.what()); }
    }
}

impl<FieldT: FieldTConfig> EvaluationDomainConfig<FieldT> for basic_radix2_domains<FieldT> {
    fn FFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("basic_radix2: expected a.len() == self.m");
        }

        _basic_radix2_FFT(a, &self.t.omega);
        Ok(())
    }

    fn iFFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("basic_radix2: expected a.len() == self.m");
        }

        // _basic_radix2_FFT(a, self.t.omega.inverse());

        // let  sconst = FieldT::from(a.len()).inverse();
        // for i in 0..a.len()
        // {
        //     a[i] *= sconst;
        // }
        Ok(())
    }

    fn cosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        _multiply_by_coset(a, g);
        self.FFT(a)
    }

    fn icosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        self.iFFT(a);
        // _multiply_by_coset(a, g.inverse());
        Ok(())
    }

    fn evaluate_all_lagrange_polynomials(&mut self, t: &FieldT) -> Vec<FieldT> {
        return _basic_radix2_evaluate_all_lagrange_polynomials(self.m, t).unwrap();
    }

    fn get_domain_element(&mut self, idx: usize) -> FieldT {
        self.t.omega.clone() ^ idx
    }

    fn compute_vanishing_polynomial(&mut self, t: &FieldT) -> FieldT {
        let tt: FieldT = t.clone();
        let tm: FieldT = tt ^ self.m;
        tm - FieldT::one()
    }

    fn add_poly_Z(&mut self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()> {
        if H.len() != self.m + 1 {
            eyre::bail!("basic_radix2: expected H.len() == self.m+1");
        }

        H[self.m] += coeff.clone();
        H[0] -= coeff.clone();
        Ok(())
    }

    fn divide_by_Z_on_coset(&self, P: &mut Vec<FieldT>) {
        // let  coset = FieldT::multiplicative_generator.clone();
        // let  Z_inverse_at_coset = self.compute_vanishing_polynomial(coset).inverse();
        // for i in 0..self.m
        // {
        //     P[i] *= Z_inverse_at_coset;
        // }
    }
}
