use crate::circuit::structure::wire_array::WireArray;

 use std::hash::Hash;
 use std::fmt::Debug;
#[derive(Debug,Clone,Hash)]
pub struct VariableBitWire;
impl VariableBitWire {
    // pub fn  new(wireId:i32) {
    // 	super(wireId);
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![self.clone()])
    }
}
