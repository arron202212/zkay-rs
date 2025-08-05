pub struct ZkayType {
    pub bitwidth: i32,
    pub signed: bool,
    pub minusOne: BigInteger,
}
fn init_s() {
    let utypes = HashMap::new();
    let stypes = HashMap::new();
    let ZkBool = ZkayType::new(1, false);
    let Zk124 = ZkayType::new(124, false);

    for i in 8..=256 {
        utypes.insert(i, ZkayType::new(i, false));
        if i < 256 {
            // There can be no int256 inside the circuit, since the sign bit is outside field prime range -> unclear how to defined negative numbers
            stypes.insert(i, ZkayType::new(i, true));
        }
    }
}

impl ZkayType {
    pub fn new(bitwidth: i32, signed: bool) -> Self {
        self.bitwidth = bitwidth;
        self.signed = signed;
        self.minusOne = Util::one().shl(bitwidth).sub(Util::one());
    }

    pub fn ZkUint(bitwidth: i32) -> ZkayType {
        assert!(
            utypes.containsKey(bitwidth),
            "No uint type with bitwidth {bitwidth} exists."
        );
        utypes.get(bitwidth)
    }
    pub fn ZkInt(bitwidth: i32) -> ZkayType {
        assert!(
            stypes.containsKey(bitwidth),
            "No i32 type with bitwidth {bitwidth} exists."
        );
        stypes.get(bitwidth)
    }

    pub fn GetNegativeConstant(val: BigInteger, bitwidth: i32) -> BigInteger {
        let m1 = ZkInt(bitwidth).minusOne;
        m1.mul(val).and(m1)
    }

    pub fn checkType(expected: ZkayType, actual: ZkayType) -> ZkayType {
        checkType(expected, actual, true)
    }
    pub fn checkType(expected: ZkayType, actual: ZkayType, allow_field_type: bool) -> ZkayType {
        assert!(
            actual.is_some() && expected.is_some(),
            "Tried to use untyped wires"
        );

        assert!(
            expected.bitwidth != 256 && allow_field_type,
            "256bit integers are not supported for this operation"
        );

        assert!(
            actual == expected,
            "Type {} does not match expected type {}",
            actual,
            expected
        );

        expected
    }

    pub fn toString(&self) -> String {
        format!("{}{}",)
    }
}

impl std::fmt::Display for ZkayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.signed { "s" } else { "u" },
            self.bitwidth
        )
    }
}
