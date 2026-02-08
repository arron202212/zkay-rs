//  Declaration of interfaces for basic polynomial operation routines.

// /**
//  * Returns true if polynomial A is a zero polynomial.
//  */
//
// bool _is_zero(a:&Vec<FieldT>);

// /**
//  * Removes extraneous zero entries from in vector representation of polynomial.
//  * Example - Degree-4 Polynomial: [0, 1, 2, 3, 4, 0, 0, 0, 0] -> [0, 1, 2, 3, 4]
//  * Note: Simplest condensed form is a zero polynomial of vector form: [0]
//  */
//
// pub fn  _condense(a:&Vec<FieldT>);

// /**
//  * Compute the reverse polynomial up to vector size n (degree n-1).
//  * Below we make use of the reversal endomorphism definition from
//  * [Bostan, Lecerf, & Schost, 2003. Tellegen's Principle in Practice, on page 38].
//  */
//
// pub fn  _reverse(a:&Vec<FieldT>,  usize n);

// /**
//  * Computes the standard polynomial addition, polynomial A + polynomial B, and stores result in polynomial C.
//  */
//
// pub fn  _polynomial_addition(c:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

// /**
//  * Computes the standard polynomial subtraction, polynomial A - polynomial B, and stores result in polynomial C.
//  */
//
// pub fn  _polynomial_subtraction(c:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

// /**
//  * Perform the multiplication of two polynomials, polynomial A * polynomial B, and stores result in polynomial C.
//  */
//
// pub fn  _polynomial_multiplication(c:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

// /**
//  * Perform the multiplication of two polynomials, polynomial A * polynomial B, using FFT, and stores result in polynomial C.
//  */
//
// pub fn  _polynomial_multiplication_on_fft(c:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

// /**
//  * Perform the multiplication of two polynomials, polynomial A * polynomial B, using Kronecker Substitution, and stores result in polynomial C.
//  */
//
// pub fn  _polynomial_multiplication_on_kronecker(c:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

// /**
//  * Compute the transposed, polynomial multiplication of vector a and vector b.
//  * Below we make use of the transposed multiplication definition from
//  * [Bostan, Lecerf, & Schost, 2003. Tellegen's Principle in Practice, on page 39].
//  */
//
// Vec<FieldT> _polynomial_multiplication_transpose( usize &n, a:&Vec<FieldT>,  c:&Vec<FieldT>);

// /**
//  * Perform the standard Euclidean Division algorithm.
//  * Input: Polynomial A, Polynomial B, where A / B
//  * Output: Polynomial Q, Polynomial R, such that A = (Q * B) + R.
//  */
//
// pub fn  _polynomial_division(q:&Vec<FieldT>, r:&Vec<FieldT>, a:&Vec<FieldT>,  b:&Vec<FieldT>);

use crate::evaluation_domain::domains::basic_radix2_domain_aux;
use crate::kronecker_substitution::kronecker_substitution;
use crate::polynomial_arithmetic::basic_operations::kronecker_substitution::kronecker_substitution;
use crate::polynomial_arithmetic::xgcd::_polynomial_xgcd;
use crate::tools::exceptions;
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
use ffec::common::utils::get_power_of_two;
use ffec::common::utils::log2;
// #ifdef MULTICORE
//#include <omp.h>
//#endif

//
pub fn _is_zero<FieldT: num_traits::Zero + std::cmp::PartialEq>(a: &Vec<FieldT>) -> bool {
    a.iter().all(|i| i == &FieldT::zero())
    // return std::all_of(a.begin(), a.end(), [](FieldT i) { return i == FieldT::zero(); });
}

//
pub fn _condense<FieldT: num_traits::Zero + std::cmp::PartialEq>(a: &mut Vec<FieldT>) {
    while a.last().is_some_and(|v| v == &FieldT::zero()) {
        a.pop();
    }
}

//
pub fn _reverse<FieldT: num_traits::Zero + Clone>(a: &mut Vec<FieldT>, n: usize) {
    // std::reverse(a.begin(), a.end());
    a.reverse();
    a.resize(n, FieldT::zero());
}

//
pub fn _polynomial_addition<FieldT: num_traits::Zero + Clone + std::cmp::PartialEq>(
    c: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    if _is_zero(a) {
        *c = b.clone();
    } else if _is_zero(b) {
        *c = a.clone();
    } else {
        let a_size = a.len();
        let b_size = b.len();

        if a_size > b_size {
            // c.resize(a_size,FieldT::zero());
            // std::transform(b.begin(), b.end(), a.begin(), c.begin(), std::plus<FieldT>());
            // std::copy(a.begin() + b_size, a.end(), c.begin() + b_size);
            // let mut c:Vec<_>=b.iter().zip(&a).map(|(x,y)| x+y).collect();
            c.resize(a_size, FieldT::zero());
            b.iter().zip(a).enumerate().for_each(|(i, (x, y))| {
                c[i] = x.clone() + y.clone();
            });
            c[b_size..].clone_from_slice(&a[b_size..]);
        } else {
            // c.resize(b_size,FieldT::zero());
            // std::transform(a.begin(), a.end(), b.begin(), c.begin(), std::plus<FieldT>());
            // std::copy(b.begin() + a_size, b.end(), c.begin() + a_size);
            let mut c: Vec<_> = a
                .iter()
                .zip(b)
                .map(|(x, y)| x.clone() + y.clone())
                .collect();
            c.resize(b_size, FieldT::zero());
            a.iter().zip(b).enumerate().for_each(|(i, (x, y))| {
                c[i] = x.clone() + y.clone();
            });
            c[a_size..].clone_from_slice(&b[a_size..]);
        }
    }

    _condense(c);
}

//
pub fn _polynomial_subtraction<
    FieldT: num_traits::Zero
        + Clone
        + std::cmp::PartialEq
        + std::ops::Neg<Output = FieldT>
        + std::ops::Sub<Output = FieldT>,
>(
    c: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    if _is_zero(b) {
        *c = a.clone();
    } else if _is_zero(a) {
        c.resize(b.len(), FieldT::zero());
        // std::transform(b.begin(), b.end(), c.begin(), std::negate<FieldT>());
        b.iter().enumerate().for_each(|(i, v)| {
            c[i] = -v.clone();
        });
    } else {
        let a_size = a.len();
        let b_size = b.len();

        if a_size > b_size {
            c.resize(a_size, FieldT::zero());
            // std::transform(a.begin(), a.begin() + b_size, b.begin(), c.begin(), std::minus<FieldT>());
            // std::copy(a.begin() + b_size, a.end(), c.begin() + b_size);
            a.iter()
                .take(b_size)
                .zip(b)
                .enumerate()
                .for_each(|(i, (x, y))| {
                    c[i] = x.clone() - y.clone();
                });
            c[b_size..].clone_from_slice(&a[b_size..]);
        } else {
            c.resize(b_size, FieldT::zero());
            // std::transform(a.begin(), a.end(), b.begin(), c.begin(), std::minus<FieldT>());
            // std::transform(b.begin() + a_size, b.end(), c.begin() + a_size, std::negate<FieldT>());
            a.iter().zip(b).enumerate().for_each(|(i, (x, y))| {
                c[i] = x.clone() - y.clone();
            });
            b.iter().enumerate().skip(a_size).for_each(|(i, v)| {
                c[i] = -v.clone();
            });
        }
    }

    _condense(c);
}

pub fn _polynomial_multiplication<
    FieldT: Clone
        + num_traits::Zero
        + std::default::Default
        + std::cmp::PartialEq
        + std::ops::Mul<Output = FieldT>,
>(
    c: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    _polynomial_multiplication_on_fft(c, a, b);
}

pub fn _polynomial_multiplication_on_fft<
    FieldT: Clone
        + num_traits::Zero
        + std::default::Default
        + std::cmp::PartialEq
        + std::ops::Mul<Output = FieldT>,
>(
    c: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    let n = get_power_of_two(a.len() + b.len() - 1);
    let omega = get_root_of_unity_is_same_double::<FieldT>(n);

    let mut u = a.clone();
    let mut v = b.clone();
    u.resize(n, FieldT::zero());
    v.resize(n, FieldT::zero());
    c.resize(n, FieldT::zero());

    // #ifdef MULTICORE
    //     _basic_parallel_radix2_FFT(u, omega);
    //     _basic_parallel_radix2_FFT(v, omega);
    // #else
    //     _basic_serial_radix2_FFT(u, omega);
    //     _basic_serial_radix2_FFT(v, omega);
    //#endif

    // std::transform(u.begin(), u.end(), v.begin(), c.begin(), std::multiplies<FieldT>());
    u.iter().zip(v).enumerate().for_each(|(i, (x, y))| {
        c[i] = x.clone() * y.clone();
    });

    // #ifdef MULTICORE
    //     _basic_parallel_radix2_FFT(c, omega.inverse());
    // #else
    //     _basic_serial_radix2_FFT(c, omega.inverse());
    //#endif

    //  let mut  sconst = FieldT::from(n).inverse();
    // std::transform(c.begin(), c.end(), c.begin(), std::bind(std::multiplies<FieldT>(), sconst, std::placeholders::_1));
    // c.iter_mut().for_each(|v|{
    //             *v*=sconst.clone();
    //     });
    _condense(c);
}

pub fn _polynomial_multiplication_on_kronecker<
    FieldT: std::convert::From<usize>
        + std::ops::Mul<Output = FieldT>
        + std::cmp::Ord
        + std::clone::Clone
        + std::ops::AddAssign
        + num_traits::Zero
        + std::cmp::PartialEq,
>(
    c: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    kronecker_substitution(c, a, b);
}

pub fn _polynomial_multiplication_transpose<
    FieldT: num_traits::Zero
        + std::cmp::PartialEq
        + Clone
        + std::default::Default
        + std::ops::Mul<Output = FieldT>,
>(
    n: usize,
    a: &Vec<FieldT>,
    c: &Vec<FieldT>,
) -> eyre::Result<Vec<FieldT>> {
    let m = a.len();
    if c.len() - 1 > m + n {
        eyre::bail!("expected c.len() - 1 <= m + n");
    }

    let mut r = a.clone();
    _reverse(&mut r, m);
    let rr = r.clone();
    _polynomial_multiplication(&mut r, &rr, c);

    /* Determine Middle Product */
    let mut result = vec![];
    for i in m..n + m {
        result.push(r[i].clone());
    }
    Ok(result)
}

pub fn _polynomial_division<
    FieldT: num_traits::One
        + Clone
        + std::ops::SubAssign
        + std::ops::Sub
        + std::ops::Mul
        + std::ops::AddAssign
        + num_traits::Zero
        + Clone
        + std::cmp::PartialEq,
>(
    q: &mut Vec<FieldT>,
    r: &mut Vec<FieldT>,
    a: &Vec<FieldT>,
    b: &Vec<FieldT>,
) {
    let d = b.len() - 1; /* Degree of B */
    let c = FieldT::one(); //b.last().unwrap().inverse(); /* Inverse of Leading Coefficient of B */
    *r = a.clone();
    *q = vec![FieldT::zero(); r.len()];

    let mut r_deg = r.len() - 1;
    let mut shift;

    while (r_deg >= d && !_is_zero(r)) {
        if r_deg >= d {
            shift = r_deg - d;
        } else {
            shift = 0;
        }

        let mut lead_coeff = r.last().unwrap().clone() * c.clone();

        q[shift] += lead_coeff.clone();

        if b.len() + shift + 1 > r.len() {
            r.resize(b.len() + shift + 1, FieldT::zero());
        }
        let glambda = |x: FieldT, y: FieldT| {
            return y - (x * lead_coeff.clone());
        };
        // std::transform(b.begin(), b.end(), r.begin() + shift, r.begin() + shift, glambda);
        r.iter_mut().skip(shift).zip(b).for_each(|(y, x)| {
            *y -= x.clone() * lead_coeff.clone();
        });
        _condense(r);

        r_deg = r.len() - 1;
    }
    _condense(q);
}
