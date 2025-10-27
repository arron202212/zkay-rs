#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
/** @file
 *****************************************************************************

 Declaration of interfaces for evaluation domains.

 Roughly, given a desired size m for the domain, the constructor selects
 a choice of domain S with size ~m that has been selected so to optimize
 - computations of Lagrange polynomials, and
 - FFT/iFFT computations.
 An evaluation domain also provides other other functions, e.g., accessing
 individual elements in S or evaluating its vanishing polynomial.

 The descriptions below make use of the definition of a *Lagrange polynomial*,
 which we recall. Given a field F, a subset S=(a_i)_i of F, and an index idx
 in {0,...,|S-1|}, the idx-th Lagrange polynomial (wrt to subset S) is defined to be
 \f[   L_{idx,S}(z)->Self= prod_{k \neq idx} (z - a_k) / prod_{k \neq idx} (a_{idx} - a_k)   \f]
 Note that, by construction:
 \f[   \forall j \neq idx: L_{idx,S}(a_{idx}) = 1  \text{ and }  L_{idx,S}(a_j) = 0   \f]

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EVALUATION_DOMAIN_HPP_
// #define EVALUATION_DOMAIN_HPP_

//#include <vector>

// //namespace libfqfft {

/**
 * An evaluation domain.
 */
// 
pub trait evaluation_domain<FieldT> {


    const M:usize;

    /**
     * Construct an evaluation domain S of size m, if possible.
     *
     * (See the function get_evaluation_domain below.)
     */
    // evaluation_domain(m:usize)->Self m(m) {};

    /**
     * Get the idx-th element in S.
     */
    fn   get_domain_element( idx:usize) ->FieldT;

    /**
     * Compute the FFT, over the domain S, of the vector a.
     */
    fn   FFT(a:&Vec<FieldT>) ;

    /**
     * Compute the inverse FFT, over the domain S, of the vector a.
     */
    fn   iFFT(a:&Vec<FieldT>) ;

    /**
     * Compute the FFT, over the domain g*S, of the vector a.
     */
    fn   cosetFFT(a:&Vec<FieldT>, g:&FieldT) ;

    /**
     * Compute the inverse FFT, over the domain g*S, of the vector a.
     */
    fn   icosetFFT(a:&Vec<FieldT>, g:&FieldT) ;

    /**
     * Evaluate all Lagrange polynomials.
     *
     * The inputs are:
     * - an integer m
     * - an element t
     * The output is a vector (b_{0},...,b_{m-1})
     * where b_{i} is the evaluation of L_{i,S}(z) at z = t.
     */
    fn  evaluate_all_lagrange_polynomials(t:&FieldT)->Vec<FieldT>  ;

    /**
     * Evaluate the vanishing polynomial of S at the field element t.
     */
    fn   compute_vanishing_polynomial(t:&FieldT)->FieldT ;

    /**
     * Add the coefficients of the vanishing polynomial of S to the coefficients of the polynomial H.
     */
    fn   add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>) ;

    /**
     * Multiply by the evaluation, on a coset of S, of the inverse of the vanishing polynomial of S.
     */
    fn   divide_by_Z_on_coset(P:&Vec<FieldT>) ;
}

// //} // libfqfft

//#endif // EVALUATION_DOMAIN_HPP_
