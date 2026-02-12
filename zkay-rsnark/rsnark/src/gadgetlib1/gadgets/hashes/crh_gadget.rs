// #define CRH_GADGET_HPP_
use crate::gadgetlib1::gadgets::hashes::knapsack::knapsack_gadget::{
    knapsack_CRH_with_bit_out_gadget, knapsack_CRH_with_bit_out_gadgets,
    knapsack_CRH_with_field_out_gadget, knapsack_CRH_with_field_out_gadgets,
};

// for now all CRH gadgets are knapsack CRH's; can be easily extended
// later to more expressive selector types.

pub type CRH_with_field_out_gadget<FieldT, PB> = knapsack_CRH_with_field_out_gadget<FieldT, PB>;
pub type CRH_with_field_out_gadgets<FieldT, PB> = knapsack_CRH_with_field_out_gadgets<FieldT, PB>;
pub type CRH_with_bit_out_gadget<FieldT, PB> = knapsack_CRH_with_bit_out_gadget<FieldT, PB>;
pub type CRH_with_bit_out_gadgets<FieldT, PB> = knapsack_CRH_with_bit_out_gadgets<FieldT, PB>;
