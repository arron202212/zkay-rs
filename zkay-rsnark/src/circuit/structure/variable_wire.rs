use crate::circuit::structure::wire_array::WireArray;

pub struct VariableWire {
    bitWires: WireArray,
}
impl VariableWire {
    fn getBitWires(&self) -> WireArray {
        self.bitWires.clone()
    }

    fn setBits(&mut self, bitWires: WireArray) {
        self.bitWires = bitWires;
    }
}
