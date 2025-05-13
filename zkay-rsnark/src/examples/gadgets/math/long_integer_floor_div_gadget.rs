
use circuit::auxiliary::long_element;
use circuit::structure::wire;

/**
 * This gadget provides floor(a / b), when both operands are represented as long
 * elements. You can check the RSA gadgets/circuit generators for an example.
 * Most of the optimizations that reduce the cost of this step are more visible
 * in the LongElement class methods called by this gadget.
 */
pub struct LongIntegerFloorDivGadget extends LongIntegerDivision {

	pub  LongIntegerFloorDivGadget(LongElement a, LongElement b, desc:Vec<String>) {
		super(a, b, true, desc);
	}

	pub  LongIntegerFloorDivGadget(LongElement a, LongElement b, i32 bMinBitwidth, desc:Vec<String>) {
		super(a, b, bMinBitwidth, true, desc);
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return getQuotient().getArray();
	}
}
