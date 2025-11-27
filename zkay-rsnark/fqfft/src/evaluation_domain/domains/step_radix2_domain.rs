/** @file
*****************************************************************************

Declaration of interfaces for the "step radix-2" evaluation domain.

Roughly, the domain has size m = 2^k + 2^r and consists of
"the 2^k-th roots of unity" union "a coset of 2^r-th roots of unity".

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef STEP_RADIX2_DOMAIN_HPP_
// #define STEP_RADIX2_DOMAIN_HPP_
use crate::evaluation_domain::evaluation_domain;

//namespace libfqfft {

//
pub struct step_radix2_domain<FieldT> {
    //: public evaluation_domain<FieldT>
    big_m: usize,
    small_m: usize,
    omega: FieldT,
    big_omega: FieldT,
    small_omega: FieldT,
    m: usize,
}

//     step_radix2_domain(m:usize);

//     pub fn  FFT(a:Vec<FieldT>);
//     pub fn  iFFT(a:Vec<FieldT>);
//     pub fn  cosetFFT(a:Vec<FieldT>, g:&FieldT);
//     pub fn  icosetFFT(a:Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(idx:usize);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     pub fn  add_poly_Z(coeff:&FieldT, H:Vec<FieldT>);
//     pub fn  divide_by_Z_on_coset(P:Vec<FieldT>);

// };

//} // libfqfft

// use crate::evaluation_domain::domains::step_radix2_domain.tcc;

//#endif // STEP_RADIX2_DOMAIN_HPP_

use crate::evaluation_domain::domains::basic_radix2_domain_aux::{
    _basic_radix2_FFT, _basic_radix2_evaluate_all_lagrange_polynomials, _multiply_by_coset,
};
/** @file
*****************************************************************************

Implementation of interfaces for the "step radix-2" evaluation domain.

See step_radix2_domain.hpp .

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef STEP_RADIX2_DOMAIN_TCC_
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
use ffec::common::utils::log2;
//namespace libfqfft {
impl<
    FieldT: std::default::Default
        + std::cmp::PartialEq
        + std::convert::From<usize>
        + std::ops::SubAssign
        + std::ops::BitXor<Output = FieldT>
        + std::ops::AddAssign
        + std::ops::MulAssign
        + std::ops::Add<Output = FieldT>
        + std::ops::Sub<Output = FieldT>
        + num_traits::Zero
        + num_traits::One
        + Clone,
> step_radix2_domain<FieldT>
{
    pub fn new(m: usize) -> eyre::Result<Self> {
        //: evaluation_domain<FieldT>(m)
        if m <= 1 {
            eyre::bail!("step_radix2(): expected m > 1");
        }

        let big_m = 1usize << (log2(m) - 1);
        let small_m = m - big_m;

        if small_m != 1usize << log2(small_m) {
            eyre::bail!("step_radix2(): expected small_m == 1u64<<log2(small_m)");
        }
        let omega = get_root_of_unity_is_same_double::<FieldT>(1usize << log2(m));
        let big_omega = FieldT::one(); //omega.squared();
        Ok(Self {
            big_m: 0,
            small_m: 0,
            m: 0,
            omega,
            big_omega,
            small_omega: get_root_of_unity_is_same_double::<FieldT>(small_m),
        })
        // catch (const std::invalid_argument& e) { throw DomainSizeException(e.what()); }
    }

    pub fn FFT(&self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("step_radix2: expected a.len() == self.m");
        }

        let mut c = vec![FieldT::zero(); self.big_m];
        let mut d = vec![FieldT::zero(); self.big_m];

        let mut omega_i = FieldT::one();
        for i in 0..self.big_m {
            c[i] = if i < self.small_m {
                a[i].clone() + a[i + self.big_m].clone()
            } else {
                a[i].clone()
            };
            d[i] = omega_i.clone()
                * (if i < self.small_m {
                    a[i].clone() - a[i + self.big_m].clone()
                } else {
                    a[i].clone()
                });
            omega_i *= self.omega.clone();
        }

        let mut e = vec![FieldT::zero(); self.small_m];
        let compr = 1usize << (log2(self.big_m) - log2(self.small_m));
        for i in 0..self.small_m {
            for j in 0..compr {
                e[i] += d[i + j * self.small_m].clone();
            }
        }

        _basic_radix2_FFT(&mut c, &self.omega); //self.omega.squared()
        let sm: FieldT = get_root_of_unity_is_same_double::<FieldT>(self.small_m);
        _basic_radix2_FFT(&mut e, &sm);

        for i in 0..self.big_m {
            a[i] = c[i].clone();
        }

        for i in 0..self.small_m {
            a[i + self.big_m] = e[i].clone();
        }
        Ok(())
    }

    pub fn iFFT(&self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("step_radix2: expected a.len() == self.m");
        }

        let mut U0 = a[..self.big_m].to_vec();
        let mut U1 = a[..self.big_m].to_vec();

        // _basic_radix2_FFT(U0, self.omega.squared().inverse());
        // _basic_radix2_FFT(U1, get_root_of_unity_is_same_double::<FieldT>(self.small_m).inverse());

        // let  U0_size_inv = FieldT::from(self.big_m).inverse();
        for i in 0..self.big_m {
            // U0[i] *= U0_size_inv;
        }

        // let  U1_size_inv = FieldT::from(self.small_m).inverse();
        for i in 0..self.small_m {
            // U1[i] *= U1_size_inv;
        }

        let mut tmp = U0.clone();
        let mut omega_i = FieldT::one();
        for i in 0..self.big_m {
            tmp[i] *= omega_i.clone();
            omega_i *= self.omega.clone();
        }

        // save A_suffix
        for i in self.small_m..self.big_m {
            a[i] = U0[i].clone();
        }

        let compr = 1usize << (log2(self.big_m) - log2(self.small_m));
        for i in 0..self.small_m {
            for j in 1..compr {
                U1[i] -= tmp[i + j * self.small_m].clone();
            }
        }

        // let  omega_inv = self.omega.inverse();
        let mut omega_inv_i = FieldT::one();
        for i in 0..self.small_m {
            U1[i] *= omega_inv_i.clone();
            // omega_inv_i *= omega_inv.clone();
        }

        // compute A_prefix
        // let  over_two = FieldT::from(2).inverse();
        for i in 0..self.small_m {
            // a[i] = (U0[i]+U1[i]) * over_two;
        }

        // compute B2
        for i in 0..self.small_m {
            // a[self.big_m + i] = (U0[i]-U1[i]) * over_two;
        }
        Ok(())
    }

    pub fn cosetFFT(&self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        _multiply_by_coset(a, g);
        self.FFT(a)
    }

    pub fn icosetFFT(&self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        self.iFFT(a);
        // _multiply_by_coset(a, g.inverse());
        Ok(())
    }

    pub fn evaluate_all_lagrange_polynomials(&self, t: &FieldT) -> Vec<FieldT> {
        let inner_big = _basic_radix2_evaluate_all_lagrange_polynomials(self.big_m, t).unwrap();
        // let inner_small = _basic_radix2_evaluate_all_lagrange_polynomials(self.small_m, t * self.omega.inverse());

        let mut result = vec![FieldT::zero(); self.m];
        let tt: FieldT = t.clone();
        let sm: FieldT = FieldT::from(self.small_m);
        let L0 = (tt.clone() ^ sm.clone()) - (self.omega.clone() ^ sm.clone());
        let omega_to_small_m = self.omega.clone() ^ sm.clone();
        let big_omega_to_small_m = self.big_omega.clone() ^ sm.clone();
        let mut elt = FieldT::one();
        for i in 0..self.big_m {
            // result[i] = inner_big[i] * L0 * (elt - omega_to_small_m).inverse();
            elt *= big_omega_to_small_m.clone();
        }

        // let  L1 = ((t^self.big_m)-FieldT::one()) * ((self.omega^self.big_m) - FieldT::one()).inverse();

        for i in 0..self.small_m {
            // result[self.big_m + i] = L1 * inner_small[i];
        }

        return result;
    }

    pub fn get_domain_element(&self, idx: usize) -> FieldT {
        if idx < self.big_m {
            return self.big_omega.clone() ^ idx.into();
        } else {
            return self.omega.clone() * (self.small_omega.clone() ^ (idx - self.big_m).into());
        }
    }

    pub fn compute_vanishing_polynomial(&self, t: &FieldT) -> FieldT {
        let tt: FieldT = t.clone();
        let bm: FieldT = FieldT::from(self.big_m);
        let sm: FieldT = FieldT::from(self.small_m);
        return ((tt.clone() ^ bm.clone()) - FieldT::one())
            * ((tt.clone() ^ sm.clone()) - (self.omega.clone() ^ sm.clone()));
    }

    pub fn add_poly_Z(&self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()> {
        if H.len() != self.m + 1 {
            eyre::bail!("step_radix2: expected H.len() == self.m+1");
        }

        let omega_to_small_m = self.omega.clone() ^ FieldT::from(self.small_m);
        let coeff_clone: FieldT = coeff.clone();
        H[self.m] += coeff_clone.clone();
        H[self.big_m] -= coeff_clone.clone() * omega_to_small_m.clone();
        H[self.small_m] -= coeff_clone.clone();
        H[0] += coeff_clone.clone() * omega_to_small_m;
        Ok(())
    }

    pub fn divide_by_Z_on_coset(&self, P: Vec<FieldT>) {
        // (c^{2^k}-1) * (c^{2^r} * w^{2^{r+1}*i) - w^{2^r})
        let coset = FieldT::one(); //multiplicative_generator;
        let sm: FieldT = FieldT::from(self.small_m);
        let bm: FieldT = FieldT::from(self.big_m);
        let Z0 = (coset.clone() ^ bm.clone()) - FieldT::one();
        let coset_to_small_m_times_Z0 = (coset.clone() ^ sm.clone()) * Z0.clone();
        let omega_to_small_m_times_Z0 = (self.omega.clone() ^ sm.clone()) * Z0.clone();
        let omega_to_2small_m = self.omega.clone() ^ FieldT::from(2 * self.small_m);
        let mut elt = FieldT::one();

        for i in 0..self.big_m {
            // P[i] *= (coset_to_small_m_times_Z0 * elt - omega_to_small_m_times_Z0).inverse();
            elt *= omega_to_2small_m.clone();
        }

        // (c^{2^k}*w^{2^k}-1) * (c^{2^k} * w^{2^r} - w^{2^r})

        let Z1 = ((((coset.clone() * self.omega.clone()) ^ bm.clone()) - FieldT::one())
            * (((coset.clone() * self.omega.clone()) ^ sm.clone())
                - (self.omega.clone() ^ sm.clone())));
        // let  Z1_inverse = Z1.inverse();

        for i in 0..self.small_m {
            // P[self.big_m + i] *= Z1_inverse;
        }
    }
}
//} // libfqfft

//#endif // STEP_RADIX2_DOMAIN_TCC_
