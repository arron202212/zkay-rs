// Declaration of interfaces for the "step radix-2" evaluation domain.
// Roughly, the domain has size m = 2^k + 2^r and consists of
// "the 2^k-th roots of unity" union "a coset of 2^r-th roots of unity".

use crate::evaluation_domain::domains::basic_radix2_domain_aux::{
    _basic_radix2_FFT, _basic_radix2_evaluate_all_lagrange_polynomials, _multiply_by_coset,
};
use crate::evaluation_domain::evaluation_domain::{EvaluationDomainConfig, evaluation_domain};
use ffec::FieldTConfig;
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
use ffec::common::utils::log2;

#[derive(Default, Clone)]
pub struct step_radix2_domain<FieldT: FieldTConfig> {
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

// std::default::Default
//         + std::cmp::PartialEq
//         + std::convert::From<usize>
//         + std::ops::SubAssign
//         + std::ops::BitXor<Output = FieldT>
//         + std::ops::AddAssign
//         + std::ops::MulAssign
//         + std::ops::Add<Output = FieldT>
//         + std::ops::Sub<Output = FieldT>
//         + num_traits::Zero
//         + num_traits::One
//         + Clone,

pub type step_radix2_domains<FieldT> =
    evaluation_domain<step_radix2_domain<FieldT>>;
impl<FieldT: FieldTConfig> step_radix2_domain<FieldT> {
    pub fn new(m: usize) -> eyre::Result<step_radix2_domains<FieldT> > {
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
        Ok(evaluation_domain::<Self>::new(
            m,Self {
            big_m: 0,
            small_m: 0,
            m: 0,
            omega,
            big_omega,
            small_omega: get_root_of_unity_is_same_double::<FieldT>(small_m),
        }))
        // catch (const std::invalid_argument& e) { throw DomainSizeException(e.what()); }
    }
}

impl<FieldT: FieldTConfig> EvaluationDomainConfig<FieldT> for step_radix2_domains<FieldT> {
    fn FFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("step_radix2: expected a.len() == self.m");
        }

        let mut c = vec![FieldT::zero(); self.t.big_m];
        let mut d = vec![FieldT::zero(); self.t.big_m];

        let mut omega_i = FieldT::one();
        for i in 0..self.t.big_m {
            c[i] = if i < self.t.small_m {
                a[i].clone() + a[i + self.t.big_m].clone()
            } else {
                a[i].clone()
            };
            d[i] = omega_i.clone()
                * (if i < self.t.small_m {
                    a[i].clone() - a[i + self.t.big_m].clone()
                } else {
                    a[i].clone()
                });
            omega_i *= self.t.omega.clone();
        }

        let mut e = vec![FieldT::zero(); self.t.small_m];
        let compr = 1usize << (log2(self.t.big_m) - log2(self.t.small_m));
        for i in 0..self.t.small_m {
            for j in 0..compr {
                e[i] += d[i + j * self.t.small_m].clone();
            }
        }

        _basic_radix2_FFT(&mut c, &self.t.omega); //self.t.omega.squared()
        let sm: FieldT = get_root_of_unity_is_same_double::<FieldT>(self.t.small_m);
        _basic_radix2_FFT(&mut e, &sm);

        for i in 0..self.t.big_m {
            a[i] = c[i].clone();
        }

        for i in 0..self.t.small_m {
            a[i + self.t.big_m] = e[i].clone();
        }
        Ok(())
    }

    fn iFFT(&mut self, a: &mut Vec<FieldT>) -> eyre::Result<()> {
        if a.len() != self.m {
            eyre::bail!("step_radix2: expected a.len() == self.m");
        }

        let mut U0 = a[..self.t.big_m].to_vec();
        let mut U1 = a[..self.t.big_m].to_vec();

        // _basic_radix2_FFT(U0, self.t.omega.squared().inverse());
        // _basic_radix2_FFT(U1, get_root_of_unity_is_same_double::<FieldT>(self.t.small_m).inverse());

        // let  U0_size_inv = FieldT::from(self.t.big_m).inverse();
        for i in 0..self.t.big_m {
            // U0[i] *= U0_size_inv;
        }

        // let  U1_size_inv = FieldT::from(self.t.small_m).inverse();
        for i in 0..self.t.small_m {
            // U1[i] *= U1_size_inv;
        }

        let mut tmp = U0.clone();
        let mut omega_i = FieldT::one();
        for i in 0..self.t.big_m {
            tmp[i] *= omega_i.clone();
            omega_i *= self.t.omega.clone();
        }

        // save A_suffix
        for i in self.t.small_m..self.t.big_m {
            a[i] = U0[i].clone();
        }

        let compr = 1usize << (log2(self.t.big_m) - log2(self.t.small_m));
        for i in 0..self.t.small_m {
            for j in 1..compr {
                U1[i] -= tmp[i + j * self.t.small_m].clone();
            }
        }

        // let  omega_inv = self.t.omega.inverse();
        let mut omega_inv_i = FieldT::one();
        for i in 0..self.t.small_m {
            U1[i] *= omega_inv_i.clone();
            // omega_inv_i *= omega_inv.clone();
        }

        // compute A_prefix
        // let  over_two = FieldT::from(2).inverse();
        for i in 0..self.t.small_m {
            // a[i] = (U0[i]+U1[i]) * over_two;
        }

        // compute B2
        for i in 0..self.t.small_m {
            // a[self.t.big_m + i] = (U0[i]-U1[i]) * over_two;
        }
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
        let inner_big = _basic_radix2_evaluate_all_lagrange_polynomials(self.t.big_m, t).unwrap();
        // let inner_small = _basic_radix2_evaluate_all_lagrange_polynomials(self.t.small_m, t * self.t.omega.inverse());

        let mut result = vec![FieldT::zero(); self.m];
        let tt: FieldT = t.clone();
        let sm = (self.t.small_m);
        let L0 = (tt.clone() ^ sm.clone()) - (self.t.omega.clone() ^ sm.clone());
        let omega_to_small_m = self.t.omega.clone() ^ sm.clone();
        let big_omega_to_small_m = self.t.big_omega.clone() ^ sm.clone();
        let mut elt = FieldT::one();
        for i in 0..self.t.big_m {
            // result[i] = inner_big[i] * L0 * (elt - omega_to_small_m).inverse();
            elt *= big_omega_to_small_m.clone();
        }

        // let  L1 = ((t^self.t.big_m)-FieldT::one()) * ((self.t.omega^self.t.big_m) - FieldT::one()).inverse();

        for i in 0..self.t.small_m {
            // result[self.t.big_m + i] = L1 * inner_small[i];
        }

         result
    }

    fn get_domain_element(&mut self, idx: usize) -> FieldT {
        if idx < self.t.big_m {
             self.t.big_omega.clone() ^ idx.into()
        } else {
             self.t.omega.clone() * (self.t.small_omega.clone() ^ (idx - self.t.big_m).into())
        }
    }

    fn compute_vanishing_polynomial(&mut self, t: &FieldT) -> FieldT {
        let tt: FieldT = t.clone();
        let bm = (self.t.big_m);
        let sm = (self.t.small_m);
         ((tt.clone() ^ bm.clone()) - FieldT::one())
            * ((tt.clone() ^ sm.clone()) - (self.t.omega.clone() ^ sm.clone()))
    }

    fn add_poly_Z(&mut self, coeff: &FieldT, H: &mut Vec<FieldT>) -> eyre::Result<()> {
        if H.len() != self.m + 1 {
            eyre::bail!("step_radix2: expected H.len() == self.m+1");
        }

        let omega_to_small_m = self.t.omega.clone() ^ (self.t.small_m);
        let coeff_clone: FieldT = coeff.clone();
        H[self.m] += coeff_clone.clone();
        H[self.t.big_m] -= coeff_clone.clone() * omega_to_small_m.clone();
        H[self.t.small_m] -= coeff_clone.clone();
        H[0] += coeff_clone.clone() * omega_to_small_m;
        Ok(())
    }

    fn divide_by_Z_on_coset(&self, P: &mut Vec<FieldT>) {
        // (c^{2^k}-1) * (c^{2^r} * w^{2^{r+1}*i) - w^{2^r})
        let coset = FieldT::one(); //multiplicative_generator;
        let sm = (self.t.small_m);
        let bm =(self.t.big_m);
        let Z0 = (coset.clone() ^ bm.clone()) - FieldT::one();
        let coset_to_small_m_times_Z0 = (coset.clone() ^ sm.clone()) * Z0.clone();
        let omega_to_small_m_times_Z0 = (self.t.omega.clone() ^ sm.clone()) * Z0.clone();
        let omega_to_2small_m = self.t.omega.clone() ^ (2 * self.t.small_m);
        let mut elt = FieldT::one();

        for i in 0..self.t.big_m {
            // P[i] *= (coset_to_small_m_times_Z0 * elt - omega_to_small_m_times_Z0).inverse();
            elt *= omega_to_2small_m.clone();
        }

        // (c^{2^k}*w^{2^k}-1) * (c^{2^k} * w^{2^r} - w^{2^r})

        let Z1 = ((((coset.clone() * self.t.omega.clone()) ^ bm.clone()) - FieldT::one())
            * (((coset.clone() * self.t.omega.clone()) ^ sm.clone())
                - (self.t.omega.clone() ^ sm.clone())));
        // let  Z1_inverse = Z1.inverse();

        for i in 0..self.t.small_m {
            // P[self.t.big_m + i] *= Z1_inverse;
        }
    }
}

