// Declaration of public parameters for FOORAM.

// use crate::gadgetlib1::gadgets::cpu_checkers::fooram::fooram_cpu_checker;
use crate::relations::ram_computations::rams::fooram::fooram_aux::fooram_architecture_params;
use crate::relations::ram_computations::rams::ram_params;
use crate::relations::ram_computations::rams::ram_params::ram_params_type;
use ffec::FieldTConfig;
use std::marker::PhantomData;
#[derive(Default)]
pub struct ram_fooram<FieldT: FieldTConfig>(PhantomData<FieldT>);

// impl<FieldT: FieldTConfig> ram_params_type for ram_fooram<FieldT> {
//     //
//     type base_field_type = FieldT;
//     type protoboard_type = (); //fooram_protoboard<FieldT>;
//     type gadget_base_type = (); //fooram_gadget<FieldT>;
//     type cpu_checker_type = (); //fooram_cpu_checker<FieldT>;
//     type architecture_params_type = fooram_architecture_params;

//     const timestamp_length: usize = 300;
// }

//
// usize ram_fooram<FieldT>::timestamp_length = 300;
