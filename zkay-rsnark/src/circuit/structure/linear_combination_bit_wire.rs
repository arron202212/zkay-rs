pub struct LinearCombinationBitWire;
impl LinearCombinationBitWire {
    // pub fn new( wireId:i32) {
    // 	super(wireId);
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![self.clone()])
    }
}
