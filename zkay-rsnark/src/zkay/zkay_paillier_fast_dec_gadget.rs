use crate::circuit::auxiliary::long_element;
use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_floor_div_gadget;
use crate::examples::gadgets::math::long_integer_mod_gadget;
use crate::examples::gadgets::math::long_integer_mod_inverse_gadget;
use crate::examples::gadgets::math::long_integer_mod_pow_gadget;

pub struct ZkayPaillierFastDecGadget {
    n: LongElement,
    nSquare: LongElement,
    nBits: i32,
    lambda: LongElement,
    cipher: LongElement,
    plain: Option<LongElement>,
}
impl ZkayPaillierFastDecGadget {
    pub fn new(
        n: LongElement,
        nBits: i32,
        lambda: LongElement,
        cipher: LongElement,
        desc: &Option<String>,
        generator: RcCell<CircuitGenerator>,
    ) -> Gadget<Self> {
        let nSquareMaxBits = 2 * nBits;
        let maxNumChunks =
            (nSquareMaxBits + (LongElement::CHUNK_BITWIDTH - 1)) / LongElement::CHUNK_BITWIDTH;
        let nSquare = n.mul(n).align(maxNumChunks);
        let mut _self = Gadget::<Self> {
            generator,
            description: desc
                .as_ref()
                .map_or_else(|| String::new(), |d| d.to_owned()),
            t: Self {
                n,
                nSquare,
                nBits,
                nSquareMaxBits,
                lambda,
                cipher,
                plain: None,
            },
        };
        _self.buildCircuit();
        _self
    }
}
impl Gadget<ZkayPaillierFastDecGadget> {
    fn buildCircuit(&mut self) {
        let nSquareMinBits = 2 * nBits - 1; // Minimum bit length of n^2
        let lambdaInverse =
            LongIntegerModInverseGadget::new(lambda, n, false, "lambda^(-1)").getResult();

        // plain = L(cipher^lambda mod n^2) / lambda mod n
        let cPowLambda = LongIntegerModPowGadget::new(
            cipher,
            lambda,
            nSquare,
            nSquareMinBits,
            -1,
            &Some("c^lambda".to_owned()),
            self.cg(),
        )
        .getResult();
        let lOutput = LongIntegerFloorDivGadget::new(cPowLambda.sub(1), n, "(c^lambda - 1) / n")
            .getQuotient();
        let timesLambdaInverse = lOutput.mul(lambdaInverse);
        self.t.plain = Some(
            LongIntegerModGadget::new(timesLambdaInverse, n, nBits, true)
                .getRemainder()
                .clone(),
        );
    }

    pub fn getPlaintext(&self) -> &Option<LongElement> {
        &self.t.plain
    }
}
impl GadgetConfig for Gadget<ZkayPaillierFastDecGadget> {
    fn getOutputWires(&self) -> &Vec<Option<WireType>> {
        self.t.plain.getArray()
    }
}
