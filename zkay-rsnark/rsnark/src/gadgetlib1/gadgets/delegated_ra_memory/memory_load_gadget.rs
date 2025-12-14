//  Declaration of interfaces for the memory load gadget.
//  The gadget can be used to verify a memory load from a "delegated memory".

use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_read_gadget::{
    merkle_tree_check_read_gadget, merkle_tree_check_read_gadgets,
};

pub type memory_load_gadget<FieldT, PB, HashT> = merkle_tree_check_read_gadget<FieldT, PB, HashT>;
pub type memory_load_gadgets<FieldT, PB, HashT> = merkle_tree_check_read_gadgets<FieldT, PB, HashT>;
