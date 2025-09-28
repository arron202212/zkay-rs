/** @file
 *****************************************************************************

 This file defines the default choices of TinyRAM zk-SNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef TINYRAM_PPZKSNARK_PP_HPP_
#define TINYRAM_PPZKSNARK_PP_HPP_

use  <libsnark/common/default_types/r1cs_ppzkpcd_pp.hpp>
use  <libsnark/relations/ram_computations/rams/tinyram/tinyram_params.hpp>

namespace libsnark {

class default_tinyram_zksnark_pp {
public:
    type default_r1cs_ppzkpcd_pp PCD_pp;
    type typename PCD_pp::scalar_field_A FieldT;
    type ram_tinyram<FieldT> machine_pp;

    static void init_public_params();
};

} // libsnark

#endif // TINYRAM_PPZKSNARK_PP_HPP_
/** @file
 *****************************************************************************
 This file provides the initialization methods for the default TinyRAM zk-SNARK.
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <libsnark/common/default_types/tinyram_zksnark_pp.hpp>

namespace libsnark {

void default_tinyram_zksnark_pp::init_public_params()
{
    PCD_pp::init_public_params();
}

} // libsnark