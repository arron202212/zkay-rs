// /** @file
//  *****************************************************************************

//  Declaration of interfaces for auxiliary functions for the "basic radix-2" evaluation domain.

//  These functions compute the radix-2 FFT (in single- or multi-thread mode) and,
//  also compute Lagrange coefficients.

//  *****************************************************************************
//  * @author     This file is part of libfqfft, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef BASIC_RADIX2_DOMAIN_AUX_HPP_
// #define BASIC_RADIX2_DOMAIN_AUX_HPP_

//#include <vector>

// //namespace libfqfft {

// /**
//  * Compute the radix-2 FFT of the vector a over the set S={omega^{0},...,omega^{m-1}}.
//  */
// 
// pub fn _basic_radix2_FFT(a:Vec<FieldT>, omega:&FieldT);

// /**
//  * A multi-thread version of _basic_radix2_FFT.
//  */
// 
// pub fn _parallel_basic_radix2_FFT(a:Vec<FieldT>, omega:&FieldT);

// /**
//  * Translate the vector a to a coset defined by g.
//  */
// 
// pub fn _multiply_by_coset(a:Vec<FieldT>, g:&FieldT);

// /**
//  * Compute the m Lagrange coefficients, relative to the set S={omega^{0},...,omega^{m-1}}, at the field element t.
//  */
// 
// Vec<FieldT> _basic_radix2_evaluate_all_lagrange_polynomials(m:usize, t:&FieldT);

// //} // libfqfft

// use crate::evaluation_domain::domains::basic_radix2_domain_aux.tcc;

//#endif // BASIC_RADIX2_DOMAIN_AUX_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for auxiliary functions for the "basic radix-2" evaluation domain.

 See basic_radix2_domain_aux.hpp .

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_RADIX2_DOMAIN_AUX_TCC_
// #define BASIC_RADIX2_DOMAIN_AUX_TCC_

//#include <algorithm>
//#include <vector>

// #ifdef MULTICORE
//#include <omp.h>
//#endif

use ffec::algebra::field_utils::field_utils;
use ffec::common::utils::bitreverse;
 use ffec::common::utils::log2;
use crate::tools::exceptions;
use ffec::common::profiling::print_indent;
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
 use std::ops::BitXor;
use std::ops::MulAssign;
use num_traits::One;
// #ifdef DEBUG
use ffec::common::profiling;
//#endif

// //namespace libfqfft {

// #ifdef MULTICORE
// #define _basic_radix2_FFT _basic_parallel_radix2_FFT
// #else
// #define _basic_radix2_FFT _basic_serial_radix2_FFT
//#endif

#[inline]
pub fn _basic_radix2_FFT<FieldT:num_traits::One+ Clone+std::ops::BitXor<Output = FieldT> + std::convert::From<usize>+ std::ops::MulAssign + std::ops::AddAssign+ std::ops::Sub<Output = FieldT>>(a:&mut Vec<FieldT>, omega:&FieldT)->eyre::Result<()>
 {
    _basic_serial_radix2_FFT(a,omega)
}

/*
 Below we make use of pseudocode from [CLRS 2n Ed, pp. 864].
 Also, note that it's the caller's responsibility to multiply by 1/N.
 */

pub fn _basic_serial_radix2_FFT<FieldT:num_traits::One+ std::ops::MulAssign+Clone+ BitXor<Output = FieldT>+std::convert::From<usize>+std::ops::AddAssign  + std::ops::Sub<Output = FieldT>>(a:&mut Vec<FieldT>, omega:&FieldT)->eyre::Result<()>
{
    let  n = a.len();let  logn = log2(n);
    if n != (1usize << logn){ eyre::bail!("expected n == (1usize << logn)");}

    /* swapping in place (from Storer's book) */
    for k in 0..n
    {
        let  rk = bitreverse(k, logn);
        if k < rk{
            a.swap(k,rk);
        }
        
    }
    let omega:FieldT=omega.clone();
    let mut m = 1; // invariant: m = 2^{s-1}
    for s in 1..=logn
    {
        let nm:FieldT=FieldT::from(n/(2*m));
        // w_m is 2^s-th root of unity now
        let  w_m:FieldT = omega.clone()^nm;

        // asm volatile  ("/* pre-inner */");
        for k in 0..n
        {
             let mut   w = FieldT::one();
            for j in 0..m
            {
                let  t = w.clone() * a[k+j+m].clone();
                a[k+j+m] = a[k+j].clone() - t.clone();
                a[k+j] += t.clone();
                w *= w_m.clone();
            }
        }
        // asm volatile ("/* post-inner */");
        m *= 2;
    }
    Ok(())
}


pub fn _basic_parallel_radix2_FFT_inner<FieldT:num_traits::Zero+BitXor<Output = FieldT>+ std::ops::Sub+std::convert::From<usize>+ std::ops::Sub<Output = FieldT>+One+ std::ops::MulAssign+Clone+ std::ops::AddAssign>(a:&mut Vec<FieldT>, omega:&FieldT, log_cpus:usize)->eyre::Result<()>
{
    let  num_cpus = 1usize<<log_cpus;

    let m = a.len();
    let log_m = log2(m);
    if m != 1usize<<log_m {eyre::bail!("expected m == 1u64<<log_m");}

    if log_m < log_cpus
    {
        _basic_serial_radix2_FFT(a, omega);
        return Ok(());
    }

    let mut tmp=vec![vec![];num_cpus];
    for j in 0..num_cpus
    {
        tmp[j].resize(1usize<<(log_m-log_cpus), FieldT::zero());
    }
    let omega_clone:FieldT=omega.clone();
// #ifdef MULTICORE
    // //#pragma omp parallel for
//#endif
    for j in 0..num_cpus
    {
        let  omega_j = omega_clone.clone()^FieldT::from(j);
        let omega_step = omega_clone.clone()^FieldT::from(j<<(log_m - log_cpus));

        let mut elt = FieldT::one();
        for i in 0..1usize<<(log_m - log_cpus)
        {
            for s in 0..num_cpus
            {
                // invariant: elt is omega^(j*idx)
                let idx = (i + (s<<(log_m - log_cpus))) % (1usize << log_m);
                tmp[j][i] += a[idx].clone() * elt.clone();
                elt *= omega_step.clone();
            }
            elt *= omega_j.clone();
        }
    }

    let omega_num_cpus = omega_clone^FieldT::from(num_cpus);

// #ifdef MULTICORE
    //#pragma omp parallel for
//#endif
    for j in 0..num_cpus
    {
        _basic_serial_radix2_FFT(&mut tmp[j], &omega_num_cpus);
    }

// #ifdef MULTICORE
    //#pragma omp parallel for
//#endif
    for i in 0..num_cpus
    {
        for j in 0..1usize<<(log_m - log_cpus)
        {
            // now: i = idx >> (log_m - log_cpus) and j = idx % (1u32 << (log_m - log_cpus)), for idx = ((i<<(log_m-log_cpus))+j) % (1u32 << log_m)
            a[(j<<log_cpus) + i] = tmp[i][j].clone();
        }
    }
    Ok(())
}


pub fn _basic_parallel_radix2_FFT<FieldT: num_traits::Zero+BitXor<Output = FieldT>+ std::convert::From<usize>+ std::ops::Sub<Output = FieldT>+ std::ops::MulAssign+ Clone+num_traits::One+std::ops::AddAssign>(a:&mut Vec<FieldT>, omega:&FieldT)
{
// #ifdef MULTICORE
//     let num_cpus = omp_get_max_threads();
// #else
     const  num_cpus:usize =1;
//#endif
    let log_cpus = if num_cpus & (num_cpus - 1) == 0  {log2(num_cpus)} else {log2(num_cpus) - 1};

// #ifdef DEBUG
    print_indent(); print!("* Invoking parallel FFT on 2^{} CPUs (omp_get_max_threads = {})\n", log_cpus, num_cpus);
//#endif

    if log_cpus == 0
    {
        _basic_serial_radix2_FFT( a, omega);
    }
    else
    {
        _basic_parallel_radix2_FFT_inner(a, omega, log_cpus);
    }
}


pub fn _multiply_by_coset<FieldT: Clone+std::ops::MulAssign+std::convert::From<usize>>(a:&mut Vec<FieldT>, g:&FieldT)
{
    let mut  u:FieldT = g.clone();
    for i in 1..a.len()
    {
        a[i] *= u.clone();
        u *= g.clone();
    }
}


 pub fn _basic_radix2_evaluate_all_lagrange_polynomials<FieldT:One+Clone+ BitXor<Output =FieldT>+std::cmp::PartialEq+ std::ops::MulAssign+ std::ops::Sub<Output = FieldT>+  std::convert::From<usize>+ num_traits::Zero+std::default::Default>( m:usize, t:&FieldT)->eyre::Result<Vec<FieldT>>
{
    if m == 1
    {
        return  Ok(vec![FieldT::one()]);
    }

    if m != (1usize << log2(m)){ eyre::bail!("expected m == (1u32 << log2(m))");}

   let  omega = get_root_of_unity_is_same_double::<FieldT>(m);

    let mut  u=vec![FieldT::zero();m];

    /*
     If t equals one of the roots of unity in S={omega^{0},...,omega^{m-1}}
     then output 1 at the right place, and 0 elsewhere
     */
    let tt:FieldT=t.clone();
    let fm:FieldT=FieldT::from(m);
    let tm:FieldT=tt.clone()^fm.clone();
    if tm == FieldT::one()
    {
        let mut  omega_i = FieldT::one();
        for i in 0..m
        {
            if &omega_i == t // i.e., t equals omega^i
            {
                u[i] = FieldT::one();
                return Ok(u);
            }

            omega_i *= omega.clone();
        }
    }

    /*
     Otherwise, if t does not equal any of the roots of unity in S,
     then compute each L_{i,S}(t) as Z_{S}(t) * v_i / (t-\omega^i)
     where:
     - Z_{S}(t) = \prod_{j} (t-\omega^j) = (t^m-1), and
     - v_{i} = 1 / \prod_{j \neq i} (\omega^i-\omega^j).
     Below we use the fact that v_{0} = 1/m and v_{i+1} = \omega * v_{i}.
     */

    let  Z = tt.clone()^FieldT::from(m)-FieldT::one();
    // let l = Z * FieldT::from(m).inverse();
    let mut r = FieldT::one();
    for i in 0..m
    {
        // u[i] = l * (t - r).inverse();
        // l *= omega;
        r *= omega.clone();
    }

    return Ok(u);
}

// //} // libfqfft

//#endif // BASIC_RADIX2_DOMAIN_AUX_TCC_
