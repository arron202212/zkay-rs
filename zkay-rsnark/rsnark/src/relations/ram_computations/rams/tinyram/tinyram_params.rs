/** @file
 *****************************************************************************

 Declaration of public parameters for TinyRAM.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_PARAMS_HPP_
// #define TINYRAM_PARAMS_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::tinyram_cpu_checker;
use crate::relations::ram_computations::rams::ram_params;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux;



// template<typename FieldT>
pub trait  ram_tinyram<FieldT> {
// public:
    const  timestamp_length:usize=300;

    type base_field_type=FieldT;
    type protoboard_type=tinyram_protoboard<FieldT>;
    type gadget_base_type=tinyram_gadget<FieldT>;
    type cpu_checker_type=tinyram_cpu_checker<FieldT>;
    type architecture_params_type=tinyram_architecture_params;
}

// template<typename FieldT>
// size_t ram_tinyram<FieldT>::timestamp_length = 300;



//#endif // TINYRAM_PARAMS_HPP_
