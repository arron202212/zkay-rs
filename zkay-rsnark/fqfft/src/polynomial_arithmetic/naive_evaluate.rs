/** @file
 *****************************************************************************

 Declaration of interfaces for naive evaluation routines.

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef NAIVE_EVALUATE_HPP_
// #define NAIVE_EVALUATE_HPP_

//#include <vector>

use crate::tools::exceptions;

//namespace libfqfft {

/**
 * Naive evaluation of a *single* polynomial, used for testing purposes.
 *
 * The inputs are:
 * - an integer m
 * - a vector coeff representing monomial P of size m
 * - a field element element t
 * The output is the polynomial P(x) evaluated at x = t.
 */
// 
// FieldT evaluate_polynomial(m:usize, coeff:&Vec<FieldT>, t:&FieldT);

/**
 * Naive evaluation of a *single* Lagrange polynomial, used for testing purposes.
 *
 * The inputs are:
 * - an integer m
 * - a domain S = (a_{0},...,a_{m-1}) of size m
 * - a field element element t
 * - an index idx in {0,...,m-1}
 * The output is the polynomial L_{idx,S}(z) evaluated at z = t.
 */
// 
// FieldT evaluate_lagrange_polynomial(m:usize, domain:&Vec<FieldT>, t:&FieldT, idx:usize);

//} // libfqfft

// use crate::polynomial_arithmetic::naive_evaluate.tcc;

//#endif // NAIVE_EVALUATE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for naive evaluation routines.

 See naive_evaluate.hpp .

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef NAIVE_EVALUATE_TCC_
// #define NAIVE_EVALUATE_TCC_

//#include <algorithm>

//namespace libfqfft {

// 
 pub fn evaluate_polynomial<FieldT: num_traits::Zero+ num_traits::One+Clone>(m:usize, coeff:&Vec<FieldT>, t:&FieldT)->eyre::Result<FieldT>
{
    if m != coeff.len(){eyre::bail!("expected m == coeff.len()");}

    let mut  result = FieldT::zero();

    /* NB: unsigned reverse iteration: cannot do i >= 0, but can do i < m
       because unsigned integers are guaranteed to wrap around */
    for i in (0..m).rev()
    {
        result = (result * t.clone()) + coeff[i].clone();
    }

    return Ok(result);
}

// 
 pub fn evaluate_lagrange_polynomial<FieldT: Clone+std::ops::Sub<Output = FieldT>+num_traits::One+num_traits::Zero+ std::ops::MulAssign>(m:usize, domain:&Vec<FieldT>, t:&FieldT, idx:usize)->eyre::Result<FieldT>
{
    if m != domain.len(){eyre::bail!("expected m == domain.len()");}
    if idx >= m{eyre::bail!("expected idx < m");}

    let mut num = FieldT::one();
    let mut denom = FieldT::one();

    for k in 0..m
    {
        if k == idx
        {
            continue;
        }

        num *= t.clone() - domain[k].clone();
        denom *= domain[idx].clone() - domain[k].clone();
    }

    return Ok(num) ;//* denom.inverse();
}

//} // libfqfft

//#endif // NAIVE_EVALUATE_TCC_
