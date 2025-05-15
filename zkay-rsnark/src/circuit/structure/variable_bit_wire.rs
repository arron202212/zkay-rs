pub struct VariableBitWire;
impl VariableBitWire {
    // pub fn  new(wireId:i32) {
    // 	super(wireId);
    // }

    pub fn getBitWires(&self) -> WireArray {
        WireArray::new(vec![self.clone()])
    }
}
