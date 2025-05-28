use crate::circuit::auxiliary::long_element;
use crate::circuit::structure::wire_type::WireType;

/**
 * This gadget provides floor(a / b), when both operands are represented as long
 * elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
pub struct LongIntegerFloorDivGadget {}
impl LongIntegerFloorDivGadget {
    pub fn new(a: LongElement, b: LongElement, bMinBitwidth: i32, desc: &Option<String>) -> Self {
        super(a, b, bMinBitwidth, true, desc);
    }
}
impl LongIntegerDivision for LongIntegerFloorDivGadget {
    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return getQuotient().getArray();
    }
}
