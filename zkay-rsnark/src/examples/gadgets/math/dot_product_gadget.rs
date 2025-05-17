use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

pub struct DotProductGadget {
    a: Vec<WireType>,
    b: Vec<WireType>,
    output: WireType,
}
impl DotProductGadget {
    pub fn new(a: Vec<WireType>, b: Vec<WireType>, desc: Vec<String>) -> Self {
        super(desc);
        assert!(a.len() == b.len());
        self.a = a;
        self.b = b;
        buildCircuit();
    }
}
impl Gadget for DotProductGadget {
    fn buildCircuit() {
        output = generator.getZeroWire();
        for i in 0..a.len() {
            let product = a[i].mul(b[i], format!("Multiply elements # {i}"));
            output = output.add(product);
        }
    }

    pub fn getOutputWires() -> Vec<WireType> {
        return vec![output];
    }
}
