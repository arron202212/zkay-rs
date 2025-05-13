
use circuit::auxiliary::long_element;
use circuit::operations::gadget;
use circuit::structure::wire;


/**
 * This gadget computes the result of the modular exponentiation c = b^e mod m,
 * where c, b, e, and m are LongElements.
 */
pub struct LongIntegerModPowGadget extends Gadget {

	 LongElement b; // base
	 LongElement e; // exponent
	 i32 eMaxBits; // maximum bit length of e
	 LongElement m; // modulus
	 i32 mMinBits; // minimum bit length of m

	 LongElement c; // c = m^e mod m

	pub  LongIntegerModPowGadget(LongElement b, LongElement e, LongElement m, i32 mMinBitLength, desc:Vec<String>) {
		this(b, e, -1, m, mMinBitLength, desc);
	}

	pub  LongIntegerModPowGadget(LongElement b, LongElement e, i32 eMaxBits, LongElement m, i32 mMinBits, desc:Vec<String>) {
		super(desc);
		self.b = b;
		self.e = e;
		self.eMaxBits = eMaxBits;
		self.m = m;
		self.mMinBits = mMinBits;
		buildCircuit();
	}

	  fn buildCircuit() {
		LongElement one = LongElement::new(vec![BigInteger::default();] {BigInteger.ONE});
		Vec<Wire> eBits = e.getBits(eMaxBits).asArray();

		// Start with product = 1
		LongElement product = one;
		// From the most significant to the least significant bit of the exponent, proceed as follow:
		// product = product^2 mod m
		// if eBit == 1) product = (product * base mod m
		for i in (0..=eBits.length - 1).rev()
			LongElement square = product.mul(product);
			LongElement squareModM = LongIntegerModGadget::new(square, m, mMinBits, false, "modPow: prod^2 mod m").getRemainder();
			LongElement squareTimesBase = squareModM.mul(one.muxBit(b, eBits[i]));
			product = LongIntegerModGadget::new(squareTimesBase, m, mMinBits, false, "modPow: prod * base mod m").getRemainder();
		}

		c = LongIntegerModGadget::new(product, m, true, "modPow: prod mod m").getRemainder();
	}

	pub  LongElement getResult() {
		return c;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return c.getArray();
	}
}
