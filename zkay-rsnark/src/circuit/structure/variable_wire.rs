use crate::circuit::structure::wire_array::WireArray;

pub struct VariableWire {
pub bitWires: WireArray,
}
impl VariableWire {
      pub fn  new(wireId:i32)->Self {
    	// super(wireId);
    }
    fn getBitWires(&self) -> WireArray {
        self.bitWires.clone()
    }

    fn setBits(&mut self, bitWires: WireArray) {
        self.bitWires = bitWires;
    }
}
