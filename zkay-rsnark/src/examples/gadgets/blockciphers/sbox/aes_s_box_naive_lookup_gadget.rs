use circuit::operations::gadget;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;

pub struct AESSBoxNaiveLookupGadget {
    input: Wire,
    output: Wire,
}
impl AESSBoxNaiveLookupGadget {
    const SBox: Vec<i32> = AES128CipherGadget.SBox;
    pub fn new(input: Wire, desc: Vec<String>) -> Self {
        super(desc);
        self.input = input;
        buildCircuit();
    }
}
impl Gadget for AESSBoxNaiveLookupGadget {
    fn buildCircuit() {
        output = generator.getZeroWire();
        for i in 0..256 {
            output = output.add(input.isEqualTo(i).mul(SBox[i]));
        }
    }

    pub fn getOutputWires() -> Vec<Wire> {
        return vec![output];
    }
}
