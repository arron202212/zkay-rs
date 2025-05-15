pub struct LinearCombinationWire {
    bitWires: WireArray,
}
impl LinearCombinationWire {
    // pub fn new ( wireId:i32) {
    // 	super(wireId);
    // }

    // pub  LinearCombinationWire(WireArray bits) {
    // 	super(bits);
    // }

    fn getBitWires(&self) -> WireArray {
        self.bitWires.clone()
    }

    fn setBits(&mut self, bitWires: WireArray) {
        self.bitWires = bitWires;
    }
}
