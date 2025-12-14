//  Declaration of interfaces for the memory load&store gadget.

//  The gadget can be used to verify a memory load, followed by a store to the
//  same address, from a "delegated memory".

use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_update_gadget::{
    merkle_tree_check_update_gadget, merkle_tree_check_update_gadgets,
};

pub type memory_load_store_gadget<FieldT, PB, HashT> =
    merkle_tree_check_update_gadget<FieldT, PB, HashT>;
pub type memory_load_store_gadgets<FieldT, PB, HashT> =
    merkle_tree_check_update_gadgets<FieldT, PB, HashT>;
