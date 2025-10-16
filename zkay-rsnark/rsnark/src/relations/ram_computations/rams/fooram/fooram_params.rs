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

use crate::gadgetlib1::gadgets/cpu_checkers/fooram/fooram_cpu_checker;
use crate::relations::ram_computations::rams::fooram::fooram_aux;
use crate::relations::ram_computations/rams/ram_params;



template<typename FieldT>
class ram_fooram {
public:
    type FieldT base_field_type;
    type fooram_protoboard<FieldT> protoboard_type;
    type fooram_gadget<FieldT> gadget_base_type;
    type fooram_cpu_checker<FieldT> cpu_checker_type;
    type fooram_architecture_params architecture_params_type;

    static size_t timestamp_length;
};

template<typename FieldT>
size_t ram_fooram<FieldT>::timestamp_length = 300;



//#endif // FOORAM_PARAMS_HPP_
