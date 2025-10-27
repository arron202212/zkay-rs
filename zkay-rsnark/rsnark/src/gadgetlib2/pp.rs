/** @file
 *****************************************************************************
 Declaration of PublicParams for Fp field arithmetic
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_
// #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_

// 
// 

use ffec::common::default_types::ec_pp;

// namespace gadgetlib2 {


// /*******************                        R1P World                           ******************/


/* curve-specific public parameters */
type Fp=Fr<default_ec_pp> ;

type FpVector=Vec<Fp> ;

pub struct PublicParams {
// 
     log_p:usize,
}
//     PublicParams(const std::usize log_p);
//     Fp getFp(long x) const; // to_support changes later
//     ~PublicParams();
// };

// PublicParams initPublicParamsFromDefaultPp();

// } // namespace gadgetlib2
//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_
/** @file
 *****************************************************************************
 Implementation of PublicParams for Fp field arithmetic
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <cassert>
// 

use crate::gadgetlib2::pp;

impl PublicParams {

pub fn new(log_p:usize) ->Self  {
Self{log_p}}

 pub fn getFp( x:i64) ->Fp {
    return Fp(x);
}





} 

 pub fn initPublicParamsFromDefaultPp()->PublicParams {
    default_ec_pp::init_public_params();
    let  log_p = Fr::<default_ec_pp>::size_in_bits();
    return PublicParams::new(log_p);
}