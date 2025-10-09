/** @file
 *****************************************************************************

 This file defines the default architecture and curve choices for RAM
 ppzk-SNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_PPZKSNARK_PP_HPP_
// #define TINYRAM_PPZKSNARK_PP_HPP_

use crate::common::default_types::r1cs_ppzksnark_pp;
use libsnark/relations/ram_computations/rams/tinyram/tinyram_params;



class default_tinyram_ppzksnark_pp {
public:
    type default_r1cs_ppzksnark_pp snark_pp;
    type ffec::Fr<default_r1cs_ppzksnark_pp> FieldT;
    type ram_tinyram<FieldT> machine_pp;

    static void init_public_params();
};



//#endif // TINYRAM_PPZKSNARK_PP_HPP_
/** @file
 *****************************************************************************

 This file provides the initialization methods for the default TinyRAM ppzk-SNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::common::default_types::tinyram_ppzksnark_pp;



void default_tinyram_ppzksnark_pp::init_public_params()
{
    snark_pp::init_public_params();
}


