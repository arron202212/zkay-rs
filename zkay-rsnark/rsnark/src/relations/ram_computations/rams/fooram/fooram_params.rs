/** @file
 *****************************************************************************

 Declaration of public parameters for FOORAM.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_PARAMS_HPP_
// #define FOORAM_PARAMS_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::fooram::fooram_cpu_checker;
use crate::relations::ram_computations::rams::fooram::fooram_aux;
use crate::relations::ram_computations::rams::ram_params;



// template<typename FieldT>
pub trait ram_fooram<FieldT>{
// 
    type base_field_type=FieldT;
    type protoboard_type=fooram_protoboard<FieldT>;
    type gadget_base_type=fooram_gadget<FieldT>;
    type cpu_checker_type=fooram_cpu_checker<FieldT>;
    type architecture_params_type=fooram_architecture_params;

    const timestamp_length:usize=300;
}

// template<typename FieldT>
// size_t ram_fooram<FieldT>::timestamp_length = 300;



//#endif // FOORAM_PARAMS_HPP_
