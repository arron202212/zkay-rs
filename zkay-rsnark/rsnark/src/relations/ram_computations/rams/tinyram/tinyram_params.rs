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
// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::tinyram_cpu_checker;
use crate::relations::ram_computations::rams::ram_params;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_architecture_params;
use ffec::FieldTConfig;
use std::marker::PhantomData;
#[derive(Default)]
pub struct ram_tinyram<FieldT: FieldTConfig>(PhantomData<FieldT>);

impl<FieldT: FieldTConfig> ram_params_type for ram_tinyram<FieldT> {
    const timestamp_length: usize = 300;

    type base_field_type = FieldT;
    type protoboard_type = (); //tinyram_protoboard<FieldT>;
    type gadget_base_type = (); //tinyram_gadget<FieldT>;
    type cpu_checker_type = (); //tinyram_cpu_checker<FieldT>;
    type architecture_params_type = tinyram_architecture_params;
}

//
// usize ram_tinyram<FieldT>::timestamp_length = 300;

//#endif // TINYRAM_PARAMS_HPP_
