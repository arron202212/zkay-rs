/** @file
 *****************************************************************************
 Declaration of PublicParams for Fp field arithmetic
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_
#define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_

use  <memory>
use  <vector>

use  <libff/common/default_types/ec_pp.hpp>

namespace gadgetlib2 {

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        R1P World                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/* curve-specific public parameters */
type libff::Fr<libff::default_ec_pp> Fp;

type std::vector<Fp> FpVector;

class PublicParams {
public:
    size_t log_p;
    PublicParams(const std::size_t log_p);
    Fp getFp(long x) const; // to_support changes later
    ~PublicParams();
};

PublicParams initPublicParamsFromDefaultPp();

} // namespace gadgetlib2
#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PP_HPP_
/** @file
 *****************************************************************************
 Implementation of PublicParams for Fp field arithmetic
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cassert>
use  <vector>

use  <libsnark/gadgetlib2/pp.hpp>

namespace gadgetlib2 {

PublicParams::PublicParams(const std::size_t log_p) : log_p(log_p) {}

Fp PublicParams::getFp(long x) const {
    return Fp(x);
}

PublicParams::~PublicParams() {}

PublicParams initPublicParamsFromDefaultPp() {
    libff::default_ec_pp::init_public_params();
    const std::size_t log_p = libff::Fr<libff::default_ec_pp>::size_in_bits();
    return PublicParams(log_p);
}

} // namespace gadgetlib2
