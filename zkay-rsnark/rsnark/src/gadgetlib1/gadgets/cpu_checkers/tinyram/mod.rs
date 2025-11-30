pub mod components;
// pub mod tinyram_cpu_checker;

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    inner_product_gadget, loose_multiplexing_gadget, loose_multiplexing_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::protoboard::protoboard;
pub type tinyram_packing_gadget<FieldT> =
    gadget<FieldT, tinyram_protoboard<FieldT>, packing_gadget<FieldT, tinyram_protoboard<FieldT>>>;
pub type tinyram_loose_multiplexing_gadget<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    loose_multiplexing_gadget<FieldT, tinyram_protoboard<FieldT>>,
>;

pub type tinyram_inner_product_gadget<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    inner_product_gadget<FieldT, tinyram_protoboard<FieldT>>,
>;
