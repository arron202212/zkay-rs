/** @file
*****************************************************************************

Declaration of interfaces for the "extended radix-2" evaluation domain.

Roughly, the domain has size m = 2^{k+1} and consists of
"the m-th roots of unity" union "a coset of these roots".

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef EXTENDED_RADIX2_DOMAIN_HPP_
// #define EXTENDED_RADIX2_DOMAIN_HPP_
use crate::evaluation_domain::evaluation_domain;

//namespace libfqfft {

//
pub struct extended_radix2_domain<FieldT> {
    // : public evaluation_domain<FieldT>
    small_m: usize,
    omega: FieldT,
    shift: FieldT,
    m: usize,
}

//     extended_radix2_domain(m:usize);

//     pub fn  FFT(a:Vec<FieldT>);
//     pub fn  iFFT(a:Vec<FieldT>);
//     pub fn  cosetFFT(a:Vec<FieldT>, g:&FieldT);
//     pub fn  icosetFFT(a:Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(idx:usize);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     pub fn  add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>);
//     pub fn  divide_by_Z_on_coset(P:&Vec<FieldT>);

// };

//} // libfqfft

// use crate::evaluation_domain::domains::extended_radix2_domain.tcc;

//#endif // EXTENDED_RADIX2_DOMAIN_HPP_

/** @file
*****************************************************************************

Implementation of interfaces for the "extended radix-2" evaluation domain.

See extended_radix2_domain.hpp .

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef EXTENDED_RADIX2_DOMAIN_TCC_
use crate::evaluation_domain::domains::basic_radix2_domain_aux::{
    _basic_radix2_FFT, _basic_radix2_evaluate_all_lagrange_polynomials, _multiply_by_coset,
};
use ffec::algebra::field_utils::bigint::bigint;
use ffec::algebra::field_utils::field_utils::{coset_shift, get_root_of_unity_is_same_double};
use ffec::common::utils::log2;
use std::ops::{BitXor, Mul};

//namespace libfqfft {
impl<
    FieldT: std::default::Default
        + std::cmp::PartialEq
        + std::ops::SubAssign
        + std::ops::AddAssign
        + std::ops::Sub<Output = FieldT>
        + std::ops::MulAssign
        + num_traits::Zero
        + std::convert::From<usize>
        + std::convert::From<bigint<1>>
        + Clone
        + std::ops::Add
        + num_traits::One
        + std::ops::BitXor<Output = FieldT>,
> extended_radix2_domain<FieldT>
{
    pub fn new(m: usize) -> eyre::Result<Self> {
        // : evaluation_domain<FieldT>(m)
        if m <= 1 {
            eyre::bail!("extended_radix2(): expected m > 1");
        }

        if "FieldT" != "Double" {
            let logm = log2(m);
            // if logm != (FieldT::s + 1){eyre::bail!("extended_radix2(): expected logm == FieldT::s + 1");}
        }

        let small_m = m / 2;

        Ok(Self {
            small_m,
            omega: get_root_of_unity_is_same_double::<FieldT>(small_m),
            shift: coset_shift::<FieldT>(),
            m,
        })
        // catch (const std::invalid_argument& e) { throw DomainSizeException(e.what()); }

        // shift = coset_shift<FieldT>();
    }

    pub fn FFT(&self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("extended_radix2: expected a.len() == self.m");
        }

        let mut a0 = vec![FieldT::zero(); self.small_m];
        let mut a1 = vec![FieldT::zero(); self.small_m];

        let shift_to_small_m =
            self.shift.clone() ^ FieldT::from(bigint::<1>::new(self.small_m as u64));

        let mut shift_i = FieldT::one();
        for i in 0..self.small_m {
            a0[i] = a[i].clone() + a[self.small_m + i].clone();
            // a1[i] = shift_i * (a[i] + shift_to_small_m * a[self.small_m + i]);

            shift_i *= self.shift.clone();
        }

        _basic_radix2_FFT(&mut a0, &self.omega);
        _basic_radix2_FFT(&mut a1, &self.omega);

        for i in 0..self.small_m {
            a[i] = a0[i].clone();
            a[i + self.small_m] = a1[i].clone();
        }
        Ok(())
    }

    pub fn iFFT(&self, a: Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("extended_radix2: expected a.len() == self.m");
        }

        // note: this is not in-place
        let a0 = a[..self.small_m].to_vec();
        let a1 = a[self.small_m..].to_vec();

        // let omega_inverse = self.omega.inverse();
        // _basic_radix2_FFT(a0, omega_inverse);
        // _basic_radix2_FFT(a1, omega_inverse);

        let shift_to_small_m =
            self.shift.clone() ^ FieldT::from(bigint::<1>::new(self.small_m as u64));
        // let sconst = (FieldT::from(self.small_m) * (FieldT::one()-shift_to_small_m)).inverse();

        // let shift_inverse = self.shift.inverse();
        let shift_inverse_i = FieldT::one();

        for i in 0..self.small_m {
            // a[i] = sconst * (-shift_to_small_m * a0[i] + shift_inverse_i * a1[i]);
            // a[i+self.small_m] = sconst * (a0[i] - shift_inverse_i * a1[i]);

            // shift_inverse_i *= shift_inverse;
        }
        Ok(())
    }

    pub fn cosetFFT(&self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        _multiply_by_coset(a, &g);
        self.FFT(a)
    }

    pub fn icosetFFT(&self, a: Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        self.iFFT(a);
        // _multiply_by_coset(a, g.inverse());
        Ok(())
    }

    pub fn evaluate_all_lagrange_polynomials(&self, t: &FieldT) -> Vec<FieldT> {
        let T0 = _basic_radix2_evaluate_all_lagrange_polynomials(self.small_m, t);
        // let  T1 = _basic_radix2_evaluate_all_lagrange_polynomials(self.small_m, t * self.shift.inverse());

        let result = vec![FieldT::zero(); self.m];
        let tt: FieldT = t.clone();
        let t_to_small_m = tt ^ FieldT::from(bigint::<1>::new(self.small_m as u64));
        let shift_to_small_m =
            self.shift.clone() ^ FieldT::from(bigint::<1>::new(self.small_m as u64));
        // let  one_over_denom = (shift_to_small_m - FieldT::one()).inverse();
        // let  T0_coeff = (t_to_small_m - shift_to_small_m) * (-one_over_denom);
        // let  T1_coeff = (t_to_small_m - FieldT::one()) * one_over_denom;
        for i in 0..self.small_m {
            // result[i] = T0[i] * T0_coeff;
            // result[i+self.small_m] = T1[i] * T1_coeff;
        }

        return result;
    }

    pub fn get_domain_element(&self, idx: usize) -> FieldT {
        if idx < self.small_m {
            return self.omega.clone() ^ FieldT::from(idx);
        } else {
            return self.shift.clone() * (self.omega.clone() ^ FieldT::from(idx - self.small_m));
        }
    }

    pub fn compute_vanishing_polynomial(&self, t: &FieldT) -> FieldT {
        let tt: FieldT = t.clone();
        let mm = FieldT::from(self.small_m);
        return ((tt.clone() ^ mm.clone()) - FieldT::one())
            * ((tt.clone() ^ mm.clone()) - (self.shift.clone() ^ mm.clone()));
    }

    pub fn add_poly_Z(&self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()> {
        if H.len() != self.m + 1 {
            eyre::bail!("extended_radix2: expected H.len() == self.m+1");
        }

        let shift_to_small_m: FieldT = self.shift.clone() ^ FieldT::from(self.small_m);
        let coeff_clone: FieldT = coeff.clone();
        H[self.m] += coeff_clone.clone();
        H[self.small_m] -= coeff_clone.clone() * (shift_to_small_m.clone() + FieldT::one());
        H[0] += coeff_clone.clone() * shift_to_small_m.clone();
        Ok(())
    }

    pub fn divide_by_Z_on_coset(&self, P: &Vec<FieldT>) {
        let coset = FieldT::zero(); //multiplicative_generator;

        let coset_to_small_m = coset.clone() ^ FieldT::from(self.small_m);
        let shift_to_small_m = self.shift.clone() ^ FieldT::from(self.small_m);

        let Z0 = (coset_to_small_m.clone() - FieldT::one())
            * (coset_to_small_m.clone() - shift_to_small_m.clone());
        let Z1 = (coset_to_small_m.clone() * shift_to_small_m.clone() - FieldT::one())
            * (coset_to_small_m.clone() * shift_to_small_m.clone() - shift_to_small_m.clone());

        // let  Z0_inverse = Z0.inverse();
        // let  Z1_inverse = Z1.inverse();

        for i in 0..self.small_m {
            // P[i] *= Z0_inverse;
            // P[i+self.small_m] *= Z1_inverse;
        }
    }
}
//} // libfqfft

//#endif // EXTENDED_RADIX2_DOMAIN_TCC_
