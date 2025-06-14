use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;

pub struct DotProductGadget {
    a: Vec<Option<WireType>>,
    b: Vec<Option<WireType>>,
    output: WireType,
}
impl DotProductGadget {
    pub fn new(a: Vec<Option<WireType>>, b: Vec<Option<WireType>>, desc: &Option<String>) -> Self {
        super(desc);
        assert!(a.len() == b.len());
        self.a = a;
        self.b = b;
        buildCircuit();
    }
}
impl Gadget for DotProductGadget {
    fn buildCircuit() {
        output = generator.get_zero_wire();
        for i in 0..a.len() {
            let product = a[i].mul(b[i], format!("Multiply elements # {i}"));
            output = output.add(product);
        }
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        vec![output]
    }
}
