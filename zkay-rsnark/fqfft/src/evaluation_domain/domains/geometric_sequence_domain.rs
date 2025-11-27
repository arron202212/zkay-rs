/** @file
*****************************************************************************

Declaration of interfaces for the "geometric sequence" evaluation domain.

These functions use a geometric sequence of size m to perform evaluation.

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef GEOMETRIC_SEQUENCE_DOMAIN_HPP
// #define GEOMETRIC_SEQUENCE_DOMAIN_HPP
use crate::evaluation_domain::evaluation_domain;

//namespace libfqfft {

//
pub struct geometric_sequence_domain<FieldT> {
    //   : public evaluation_domain<FieldT>
    precomputation_sentinel: bool,
    geometric_sequence: Vec<FieldT>,
    geometric_triangular_sequence: Vec<FieldT>,
    m: usize,
}

//     pub fn  do_precomputation();

//     geometric_sequence_domain(m:usize);

//     pub fn  FFT(a:&Vec<FieldT>);
//     pub fn  iFFT(a:&Vec<FieldT>);
//     pub fn  cosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     pub fn  icosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(idx:usize);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     pub fn  add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>);
//     pub fn  divide_by_Z_on_coset(P:&Vec<FieldT>);

//   };

//} // libfqfft

// use crate::evaluation_domain::domains::geometric_sequence_domain.tcc;

//#endif // GEOMETRIC_SEQUENCE_DOMAIN_HPP

/** @file
*****************************************************************************

Implementation of interfaces for the "geometric sequence" evaluation domain.

See geometric_sequence_domain.hpp .

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef GEOMETRIC_SEQUENCE_DOMAIN_TCC_
// #define GEOMETRIC_SEQUENCE_DOMAIN_TCC_
use crate::evaluation_domain::domains::basic_radix2_domain_aux::_multiply_by_coset;
use crate::polynomial_arithmetic::basic_operations::_polynomial_multiplication;
use crate::polynomial_arithmetic::basis_change;
use crate::polynomial_arithmetic::basis_change::monomial_to_newton_basis_geometric;
use crate::polynomial_arithmetic::basis_change::newton_to_monomial_basis_geometric;
// #ifdef MULTICORE
//#include <omp.h>
//#endif

//namespace libfqfft {
impl<
    FieldT: num_traits::Zero
        + Clone
        + num_traits::One
        + std::default::Default
        + std::convert::From<usize>
        + std::ops::MulAssign
        + std::ops::AddAssign
        + std::ops::Sub<Output = FieldT>
        + std::cmp::PartialEq
        + std::ops::Neg<Output = FieldT>,
> geometric_sequence_domain<FieldT>
{
    pub fn new(m: usize) -> eyre::Result<Self> {
        //: evaluation_domain<FieldT>(m)
        if m <= 1 {
            eyre::bail!("geometric(): expected m > 1");
        }
        //   if FieldT::geometric_generator() == FieldT::zero()
        //     {eyre::bail!("geometric(): expected FieldT::geometric_generator() != FieldT::zero()");}

        Ok(Self {
            precomputation_sentinel: false,
            geometric_sequence: vec![],
            geometric_triangular_sequence: vec![],
            m,
        })
    }

    pub fn FFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("geometric: expected a.len() == self.m");
        }

        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        monomial_to_newton_basis_geometric(
            a,
            &self.geometric_sequence,
            &self.geometric_triangular_sequence,
            self.m,
        );

        /* Newton to Evaluation */
        let mut T = vec![FieldT::zero(); self.m];
        T[0] = FieldT::one();

        let mut g = vec![FieldT::zero(); self.m];
        g[0] = a[0].clone();

        for i in 1..self.m {
            // T[i] = T[i-1] * (self.geometric_sequence[i] - FieldT::one()).inverse();
            g[i] = self.geometric_triangular_sequence[i].clone() * a[i].clone();
        }

        _polynomial_multiplication(a, &g, &T);
        a.resize(self.m, FieldT::zero());

        // #ifdef MULTICORE
        //#pragma omp parallel for
        //#endif
        for i in 0..self.m {
            // a[i] *= T[i].inverse();
        }
        Ok(())
    }

    pub fn iFFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("geometric: expected a.len() == self.m");
        }

        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        /* Interpolation to Newton */
        let mut T = vec![FieldT::zero(); self.m];
        T[0] = FieldT::one();

        let mut W = vec![FieldT::zero(); self.m];
        W[0] = a[0].clone() * T[0].clone();

        let mut prev_T = T[0].clone();
        for i in 1..self.m {
            // prev_T *= (self.geometric_sequence[i].clone() - FieldT::one()).inverse();

            // W[i] = a[i].clone() * prev_T.clone();
            T[i] = self.geometric_triangular_sequence[i].clone() * prev_T.clone();
            if i % 2 == 1 {
                T[i] = -T[i].clone();
            }
        }

        _polynomial_multiplication(a, &W, &T);
        a.resize(self.m, FieldT::zero());

        // #ifdef MULTICORE
        //#pragma omp parallel for
        //#endif
        for i in 0..self.m {
            // a[i] *= self.geometric_triangular_sequence[i].inverse();
        }

        newton_to_monomial_basis_geometric(
            a,
            &self.geometric_sequence,
            &self.geometric_triangular_sequence,
            self.m,
        );
        Ok(())
    }

    pub fn cosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        _multiply_by_coset(a, g);
        self.FFT(a)
    }

    pub fn icosetFFT(&mut self, a: &mut Vec<FieldT>, g: &FieldT) -> eyre::Result<()> {
        self.iFFT(a);
        //   _multiply_by_coset(a, g.inverse());
        Ok(())
    }

    pub fn evaluate_all_lagrange_polynomials(&mut self, t: &FieldT) -> Vec<FieldT> {
        /* Compute Lagrange polynomial of size m, with m+1 points (x_0, y_0), ... ,(x_m, y_m) */
        /* Evaluate for x = t */
        /* Return coeffs for each l_j(x) = (l / l_i[j]) * w[j] */

        /* for all i: w[i] = (1 / r) * w[i-1] * (1 - a[i]^m-i+1) / (1 - a[i]^-i) */

        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        /**
         * If t equals one of the geometric progression values,
         * then output 1 at the right place, and 0 elsewhere.
         */
        for i in 0..self.m {
            if &self.geometric_sequence[i] == t
            // i.e., t equals a[i]
            {
                let mut res = vec![FieldT::zero(); self.m];
                res[i] = FieldT::one();
                return res;
            }
        }

        /**
         * Otherwise, if t does not equal any of the geometric progression values,
         * then compute each Lagrange coefficient.
         */
        let mut l = vec![FieldT::zero(); self.m];
        let tt: FieldT = t.clone();
        l[0] = tt.clone() - self.geometric_sequence[0].clone();

        let mut g = vec![FieldT::zero(); self.m];
        g[0] = FieldT::zero();

        let mut l_vanish = l[0].clone();
        let mut g_vanish = FieldT::one();
        for i in 1..self.m {
            l[i] = tt.clone() - self.geometric_sequence[i].clone();
            g[i] = FieldT::one() - self.geometric_sequence[i].clone();

            l_vanish *= l[i].clone();
            g_vanish *= g[i].clone();
        }

        //   let mut  r = self.geometric_sequence[self.m-1].inverse();
        //   let mut  r_i = r.clone();

        let mut g_i = vec![FieldT::zero(); self.m];
        //   g_i[0] = g_vanish.inverse();

        //   l[0] = l_vanish * l[0].clone().inverse() * g_i[0].clone();
        for i in 1..self.m {
            // g_i[i] = g_i[i-1].clone() * g[self.m-i].clone() * -g[i].clone().inverse() * self.geometric_sequence[i].clone();
            // l[i] = l_vanish * r_i * l[i].clone().inverse() * g_i[i].clone();
            // r_i *= r;
        }

        return l;
    }

    pub fn get_domain_element(&mut self, idx: usize) -> FieldT {
        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        return self.geometric_sequence[idx].clone();
    }

    pub fn compute_vanishing_polynomial(&mut self, t: &FieldT) -> FieldT {
        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        /* Notes: Z = prod_{i = 0 to m} (t - a[i]) */
        /* Better approach: Montgomery Trick + Divide&Conquer/FFT */
        let mut Z = FieldT::one();
        let tt: FieldT = t.clone();
        for i in 0..self.m {
            Z *= (tt.clone() - self.geometric_sequence[i].clone());
        }
        return Z;
    }

    pub fn add_poly_Z(&mut self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()> {
        if H.len() != self.m + 1 {
            eyre::bail!("geometric: expected H.len() == self.m+1");
        }

        if !self.precomputation_sentinel {
            self.do_precomputation();
        }

        let mut x = vec![-self.geometric_sequence[0].clone(), FieldT::one()];

        let mut t = vec![FieldT::zero(); 2];

        for i in 1..self.m + 1 {
            t[0] = -self.geometric_sequence[i].clone();
            t[1] = FieldT::one();
            let xx = x.clone();
            _polynomial_multiplication(&mut x, &xx, &t);
        }

        // #ifdef MULTICORE
        //#pragma omp parallel for
        //#endif
        for i in 0..self.m + 1 {
            H[i] += (x[i].clone() * coeff.clone());
        }
        Ok(())
    }

    pub fn divide_by_Z_on_coset(&self, P: &Vec<FieldT>) {
        let coset = FieldT::one(); //multiplicative_generator.clone(); /* coset in geometric sequence? */
        //   let  Z_inverse_at_coset = self.compute_vanishing_polynomial(coset).inverse();
        for i in 0..self.m {
            // P[i] *= Z_inverse_at_coset;
        }
    }

    pub fn do_precomputation(&mut self) {
        self.geometric_sequence = vec![FieldT::zero(); self.m];
        self.geometric_sequence[0] = FieldT::one();

        self.geometric_triangular_sequence = vec![FieldT::zero(); self.m];
        self.geometric_triangular_sequence[0] = FieldT::one();

        for i in 1..self.m {
            self.geometric_sequence[i] = self.geometric_sequence[i - 1].clone() * FieldT::one(); //geometric_generator();
            self.geometric_triangular_sequence[i] = self.geometric_triangular_sequence[i - 1]
                .clone()
                * self.geometric_sequence[i - 1].clone();
        }

        self.precomputation_sentinel = true;
    }
}
//} // libfqfft

//#endif // GEOMETRIC_SEQUENCE_DOMAIN_TCC_
