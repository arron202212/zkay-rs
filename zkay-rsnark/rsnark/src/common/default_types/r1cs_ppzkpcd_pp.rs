/** @file
 *****************************************************************************

 This file defines the default PCD cycle.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef R1CS_PPZKPCD_PP_HPP_
#define R1CS_PPZKPCD_PP_HPP_

/*********************** Define default PCD cycle ***************************/

use  <libff/algebra/curves/mnt/mnt4/mnt4_pp.hpp>
use  <libff/algebra/curves/mnt/mnt6/mnt6_pp.hpp>

namespace libsnark {

class default_r1cs_ppzkpcd_pp {
public:
    type libff::mnt4_pp curve_A_pp;
    type libff::mnt6_pp curve_B_pp;

    type libff::Fr<curve_A_pp> scalar_field_A;
    type libff::Fr<curve_B_pp> scalar_field_B;

    static void init_public_params();
};

} // libsnark

#endif // R1CS_PPZKPCD_PP_HPP_
/** @file
 *****************************************************************************

 This file provides the initialization methods for the default PCD cycle.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <libsnark/common/default_types/r1cs_ppzkpcd_pp.hpp>

namespace libsnark {

void default_r1cs_ppzkpcd_pp::init_public_params()
{
    curve_A_pp::init_public_params();
    curve_B_pp::init_public_params();
}

} // libsnark
