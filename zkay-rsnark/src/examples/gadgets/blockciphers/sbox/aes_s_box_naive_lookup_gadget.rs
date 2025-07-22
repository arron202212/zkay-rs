#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::blockciphers::aes128_cipher_gadget;

pub struct AESSBoxNaiveLookupGadget {
    input: WireType,
    output: WireType,
}
impl AESSBoxNaiveLookupGadget {
    const SBox: Vec<i32> = AES128CipherGadget.SBox;
    pub fn new(input: WireType, desc: &Option<String>) -> Self {
        super(desc);
        self.input = input;
        buildCircuit();
    }
}
impl Gadget for AESSBoxNaiveLookupGadget {
    fn buildCircuit() {
        output = generator.get_zero_wire();
        for i in 0..256 {
            output = output.add(input.isEqualTo(i).mul(SBox[i]));
        }
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        vec![output]
    }
}
