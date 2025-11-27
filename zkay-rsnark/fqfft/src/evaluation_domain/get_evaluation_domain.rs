/** @file
*****************************************************************************

A convenience method for choosing an evaluation domain

Returns an evaluation domain object in which the domain S has size
|S| >= min_size.
The function chooses from different supported domains, depending on min_size.

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef GET_EVALUATION_DOMAIN_HPP_
// #define GET_EVALUATION_DOMAIN_HPP_

//#include <memory>

// use crate::evaluation_domain::evaluation_domain;

// //namespace libfqfft {

//
// RcCell<evaluation_domain<FieldT> > get_evaluation_domain(min_size:usize);

// //} // libfqfft

// use crate::evaluation_domain::get_evaluation_domain.tcc;

//#endif // GET_EVALUATION_DOMAIN_HPP_

/** @file
*****************************************************************************

Imeplementation of interfaces for evaluation domains.

See evaluation_domain.hpp .

We currently implement, and select among, three types of domains:
- "basic radix-2": the domain has size m = 2^k and consists of the m-th roots of unity
- "extended radix-2": the domain has size m = 2^{k+1} and consists of "the m-th roots of unity" union "a coset"
- "step radix-2": the domain has size m = 2^k + 2^r and consists of "the 2^k-th roots of unity" union "a coset of 2^r-th roots of unity"

*****************************************************************************
* @author     This file is part of libfqfft, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef GET_EVALUATION_DOMAIN_TCC_
// #define GET_EVALUATION_DOMAIN_TCC_
use crate::evaluation_domain::domains::arithmetic_sequence_domain::arithmetic_sequence_domain;
use crate::evaluation_domain::domains::basic_radix2_domain::basic_radix2_domain;
use crate::evaluation_domain::domains::extended_radix2_domain::extended_radix2_domain;
use crate::evaluation_domain::domains::geometric_sequence_domain::geometric_sequence_domain;
use crate::evaluation_domain::domains::step_radix2_domain::step_radix2_domain;
use crate::evaluation_domain::evaluation_domain::evaluation_domain;
use crate::polynomial_arithmetic::basic_operations::_polynomial_multiplication;
use crate::polynomial_arithmetic::basis_change::compute_subproduct_tree;
use crate::tools::exceptions;
use ffec::common::utils::log2;
use rccell::RcCell;
// //namespace libfqfft {

pub fn get_evaluation_domain<FieldT, ED: evaluation_domain<FieldT>>(
    min_size: usize,
) -> eyre::Result<RcCell<ED>> {
    // let mut  result; //RcCell::new( );//evaluation_domain::<FieldT>

    let big = 1usize << (log2(min_size) - 1);
    let small = min_size - big;
    let rounded_small = (1usize << log2(small));

    //         result=basic_radix2_domain::<FieldT>::new(min_size)).map_err(||
    //         result=extended_radix2_domain::<FieldT>::new(min_size)) .map_err(||
    //         result=step_radix2_domain::<FieldT>::new(min_size)).map_err(||
    // result=basic_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    // result=extended_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    // result=step_radix2_domain::<FieldT>::new(big + rounded_small)).map_err(||
    //  result=geometric_sequence_domain::<FieldT>::new(min_size)).map_err(||
    //         result=arithmetic_sequence_domain::<FieldT>::new(min_size)).map_err(||
    //      eyre::bail!("get_evaluation_domain: no matching domain")))))))))
    eyre::bail!("get_evaluation_domain: no matching domain")
}

// //} // libfqfft

//#endif // GET_EVALUATION_DOMAIN_TCC_
