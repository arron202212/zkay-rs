/** @file
 *****************************************************************************

 Declaration of interfaces for extended GCD routines.

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef XGCD_HPP_
// #define XGCD_HPP_

//#include <vector>

//namespace libfqfft {

/**
 * Perform the standard Extended Euclidean Division algorithm.
 * Input: Polynomial A, Polynomial B.
 * Output: Polynomial G, Polynomial U, Polynomial V, such that G = (A * U) + (B * V).
 */
// template<typename FieldT>
// void _polynomial_xgcd(a:&Vec<FieldT>, b:&Vec<FieldT>, g:&Vec<FieldT>, u:&Vec<FieldT>, v:&Vec<FieldT>);

//} // libfqfft

// use crate::polynomial_arithmetic::xgcd.tcc;

//#endif // XGCD_HPP_



/** @file
 *****************************************************************************

 Implementation of interfaces for extended GCD routines.

 See xgcd.hpp .
 
 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef XGCD_TCC_
// #define XGCD_TCC_

//#include <algorithm>

use crate::evaluation_domain::domains::basic_radix2_domain_aux;
use crate::polynomial_arithmetic::basic_operations;
use crate::polynomial_arithmetic::basic_operations::{_polynomial_subtraction,_is_zero,_polynomial_division,_polynomial_multiplication};

//namespace libfqfft {

// template<typename FieldT>
pub fn  _polynomial_xgcd<FieldT: num_traits::Zero+ std::ops::Neg<Output = FieldT>+ std::ops::Sub<Output = FieldT>+std::ops::AddAssign+std::cmp::PartialEq+std::cmp::PartialEq+ std::ops::SubAssign+ num_traits::One+Clone+ std::default::Default>(a:&Vec<FieldT>, b:&Vec<FieldT>, g:&mut Vec<FieldT>, u:&mut Vec<FieldT>, v:&mut Vec<FieldT>)
{
    if _is_zero(b)
    {
        *g = a.clone();
        *u = vec![FieldT::one()];
        *v = vec![FieldT::zero()];
        return;
    }

    let mut  U=vec![FieldT::one()];
    let mut  V1= vec![FieldT::zero()];
    let mut  G=a.clone();
    let mut  V3=b.clone();

    let mut  Q= vec![FieldT::zero()];
    let mut  R= vec![FieldT::zero()];
    let mut  T= vec![FieldT::zero()];

    while !_is_zero(&V3)
    {
        _polynomial_division(&mut Q,&mut R, &G, &V3);
        _polynomial_multiplication(&mut G, &V1, &Q);
        _polynomial_subtraction(&mut T, &U, &G);

        U = V1;
        G = V3;
        V1 = T.clone();
        V3 = R.clone();
    }

    _polynomial_multiplication(&mut V3, a, &U);
    let vv3=V3.clone();
    _polynomial_subtraction(&mut V3, &G, &vv3);
    _polynomial_division(&mut V1, &mut R, &V3, b);

    // let  lead_coeff = G.last().unwrap().inverse();
    // std::transform(G.begin(), G.end(), G.begin(), std::bind(std::multiplies<FieldT>(), lead_coeff, std::placeholders::_1));
    // std::transform(U.begin(), U.end(), U.begin(), std::bind(std::multiplies<FieldT>(), lead_coeff, std::placeholders::_1));
    // std::transform(V1.begin(), V1.end(), V1.begin(), std::bind(std::multiplies<FieldT>(), lead_coeff, std::placeholders::_1));
    G.iter_mut().for_each(|x|{
        // *x*=lead_coeff.clone();
    });
    U.iter_mut().for_each(|x|{
        // *x*=lead_coeff.clone();
    });
    V1.iter_mut().for_each(|x|{
        // *x*=lead_coeff.clone();
    });
    *g = G;
    *u = U;
    *v = V1;
}

//} // libfqfft

//#endif // XGCD_TCC_
