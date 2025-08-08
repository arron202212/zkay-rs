#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::circuit::auxiliary::long_element::LongElement;
use crate::circuit::structure::circuit_generator::CGConfigFields;
use crate::circuit::structure::circuit_generator::{
    CGConfig, CircuitGenerator, CircuitGeneratorExtend, addToEvaluationQueue,
    getActiveCircuitGenerator,
};
use crate::circuit::structure::wire::WireConfig;
use crate::circuit::structure::wire_array::WireArray;
use crate::circuit::structure::wire_type::WireType;
use crate::examples::gadgets::math::long_integer_division::LongIntegerDivisionConfig;
use crate::examples::gadgets::math::long_integer_floor_div_gadget::LongIntegerFloorDivGadget;
use crate::examples::gadgets::math::long_integer_mod_gadget::LongIntegerModGadget;
use crate::zkay::zkay_circuit_base::s_negate;
use crate::zkay::zkay_type::zkbool;
use crate::zkay::zkay_type::{ZkayType, zk124};
use crate::zkay::zkay_util::ZkayUtil;
use rccell::RcCell;
use std::ops::{Add, Mul, Sub};
#[derive(Debug, Clone)]
pub struct TypedWire {
    pub wire: WireType,
    pub zkay_type: ZkayType,
    pub name: String,
    pub generator: RcCell<CircuitGenerator>,
}
impl TypedWire {
    pub fn new(
        wire: WireType,
        zkay_type: ZkayType,
        name: String,
        restrict: &Vec<bool>,
        generator: RcCell<CircuitGenerator>,
    ) -> Self {
        // assert!(
        //     wire.is_some() && zkay_type.is_some(),
        //     "Arguments cannot be None"
        // );

        if restrict.get(0).is_some_and(|&v| v) || ZkayUtil::ZKAY_RESTRICT_EVERYTHING {
            wire.restrictBitLength(zkay_type.bitwidth as u64, &None);
        }
        Self {
            wire,
            zkay_type,
            name,
            generator,
        }
    }

    /** ARITH OPS **/

    pub fn plus(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " + " + &rhs.name;
        self.handle_overflow(
            &self.wire.clone().addw(&rhs.wire, &Some(op.clone())),
            &resultType,
            false,
            &op,
        )
    }

    pub fn minus(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " - " + &rhs.name;
        let ret = self.wire.add(s_negate(rhs, &self.generator).wire, op);
        self.handle_overflow(&ret, &resultType, false, &op)
    }

    pub fn times(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " * " + &rhs.name;
        if resultType.bitwidth == 256 {
            // Don't care about overflow with uint zkay_type
            return TypedWire::new(
                self.wire.mulw(&rhs.wire, &Some(op.clone())),
                resultType,
                op.clone(),
                &vec![],
                self.generator.clone(),
            );
        }
        if resultType.bitwidth <= 120 {
            // Result always fits into 240 < 253 bits
            return self.handle_overflow(
                &self.wire.mulw(&rhs.wire, &Some(op.clone())),
                &resultType,
                true,
                &op,
            );
        }

        // Result could overflow 253 bits -> do it in two halves to get correct overflow behavior
        let LhsLoHi = self
            .wire
            .getBitWiresi(resultType.bitwidth as u64, &None)
            .packBitsIntoWords(124, &None);
        let RhsLoHi = rhs
            .wire
            .getBitWiresi(resultType.bitwidth as u64, &None)
            .packBitsIntoWords(124, &None);

        // https://www.codeproject.com/Tips/618570/UInt-Multiplication-Squaring, BSD license
        let ansLoHi = LhsLoHi[0]
            .as_ref()
            .unwrap()
            .mulw(RhsLoHi[0].as_ref().unwrap(), &Some(op.clone() + "[lo*lo]"))
            .getBitWiresi(resultType.bitwidth as u64, &None)
            .packBitsIntoWords(124, &None);
        let hiLoMul = self
            .handle_overflow(
                &LhsLoHi[1]
                    .as_ref()
                    .unwrap()
                    .mulw(RhsLoHi[0].as_ref().unwrap(), &Some(op.clone() + "[hi*lo]")),
                zk124(),
                true,
                &(op.clone() + "[hi*lo]"),
            )
            .wire
            .clone();
        let loHiMul = self
            .handle_overflow(
                &LhsLoHi[0]
                    .as_ref()
                    .unwrap()
                    .mul(RhsLoHi[1].as_ref().unwrap(), &Some(op.clone() + "[lo*hi]")),
                zk124(),
                true,
                &(op.clone() + "[lo*hi]"),
            )
            .wire
            .clone();
        let hiLoPlusloHi = self
            .handle_overflow(
                &hiLoMul.addw(&loHiMul, &Some(op.clone() + "[hi*lo + lo*hi]")),
                zk124(),
                false,
                &(op.clone() + "[hi*lo + lo*hi]"),
            )
            .wire
            .clone();
        ansLoHi[1] = self
            .handle_overflow(
                ansLoHi[1].add(hiLoPlusloHi, op.clone() + "[anshi + hi*lo + lo*hi]"),
                zk124(),
                false,
                &(op.clone() + "[anshi + hi*lo + lo*hi]"),
            )
            .wire
            .clone();

        let ans = WireArray::new(ansLoHi, self.generator.clone().downgrade())
            .getBits(124, &None)
            .packBitsIntoWords(
                resultType.bitwidth as usize,
                &Some(op.clone() + "[combine hi and lo]"),
            );
        assert!(ans.len() == 1, "Multiplication ans array has wrong length");
        TypedWire::new(
            ans[0].clone().unwrap(),
            resultType,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn divideBy(&self, rhs: &TypedWire, generator: &CircuitGenerator) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " / " + &rhs.name;
        // let mut generator = CircuitGenerator.getActiveCircuitGenerator();
        generator.addOneAssertion(
            &rhs.wire.checkNonZero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let resultSign = generator.get_zero_wire();
        let lhsWire = self.wire.clone();
        let rhsWire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhsSign = lhsWire.getBitWiresi(self.zkay_type.bitwidth as usize, &None)
                [self.zkay_type.bitwidth as usize - 1];
            lhsWire = lhsSign.mux(s_negate(self, &self.generator).wire, lhsWire);
            resultSign = resultSign.as_ref().unwrap().xorBitwise(&lhsSign, 1);
        }
        if rhs.zkay_type.signed {
            let rhsSign = rhsWire.getBitWiresi(rhs.zkay_type.bitwidth as usize, &None)
                [rhs.zkay_type.bitwidth as usize - 1];
            rhsWire = rhsSign.mux(s_negate(rhs, &self.generator).wire, rhsWire);
            resultSign = resultSign.xorBitwise(rhsSign, 1);
        }

        // Need to operate on integers:long , regular div / mod gadget only works for <= 126 bits
        let lhsLong = LongElement::newa(
            lhsWire.getBitWiresi(self.zkay_type.bitwidth as usize, &None),
            self.generator.clone().downgrade(),
        );
        let rhsLong = LongElement::newa(
            rhsWire.getBitWiresi(rhs.zkay_type.bitwidth as usize, &None),
            self.generator.clone().downgrade(),
        );
        let q = LongIntegerFloorDivGadget::new(lhsLong, rhsLong, op, &None, self.generator.clone())
            .getQuotient();
        let resAbs = q
            .getBitsi(resultType.bitwidth as u64, &None)
            .packBitsIntoWords(resultType.bitwidth as usize, &None)[0];

        let resPos = TypedWire::new(resAbs, resultType, op, &vec![], self.generator.clone());
        let resNeg = s_negate(&resPos, &self.generator);
        TypedWire::new(
            resultSign.mux(resNeg.wire, resPos.wire),
            resultType,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn modulo(&self, rhs: &TypedWire, generator: &CircuitGenerator) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " % " + &rhs.name;
        // let mut generator = CircuitGenerator.getActiveCircuitGenerator();
        generator.addOneAssertion(
            rhs.wire.checkNonZero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let resultSign = generator.get_zero_wire();
        let lhsWire = self.wire.clone();
        let rhsWire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhsSign = lhsWire.getBitWiresi(self.zkay_type.bitwidth as usize, &None)
                [self.zkay_type.bitwidth as usize - 1];
            lhsWire = lhsSign.mux(s_negate(self, &self.generator).wire, lhsWire);
            resultSign = lhsSign;
        }
        if rhs.zkay_type.signed {
            let rhsSign = rhsWire.getBitWiresi(rhs.zkay_type.bitwidth as usize, &None)
                [rhs.zkay_type.bitwidth as usize - 1];
            rhsWire = rhsSign.mux(s_negate(rhs, &self.generator).wire, rhsWire);
        }

        // Need to operate on long integers, regular div / mod gadget only works for <= 126 bits
        let lhsLong = LongElement::new(
            lhsWire.getBitWiresi(self.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let rhsLong = LongElement::new(
            rhsWire.getBitWiresi(rhs.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let r =
            LongIntegerModGadget::new(lhsLong, rhsLong, true, op, &None, self.generator.clone())
                .getRemainder();
        let resAbs = r
            .getBitsi(resultType.bitwidth)
            .packBitsIntoWords(resultType.bitwidth as usize, &None)[0];

        let resPos = TypedWire::new(resAbs, resultType, op, &vec![], self.generator.clone());
        let resNeg = s_negate(resPos, &self.generator);
        TypedWire::new(
            resultSign.mux(resNeg.wire, resPos.wire),
            resultType,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    /** BIT OPS */

    pub fn bitOr(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " | " + &rhs.name;
        let res = self.wire.orBitwise(rhs.wire, resultType.bitwidth, op);
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn bitAnd(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " & " + &rhs.name;
        let res = self.wire.andBitwise(rhs.wire, resultType.bitwidth, op);
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn bitXor(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " ^ " + &rhs.name;
        let res = self.wire.xorBitwise(rhs.wire, resultType.bitwidth, op);
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    /** SHIFT OPS */

    pub fn shiftLeftBy(&self, amount: i32) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &self.zkay_type, false);
        let op = self.name.clone() + " << " + amount;
        let res = self.wire.shl(resultType.bitwidth, amount, op);
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn shiftRightBy(&self, amount: i32) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &self.zkay_type, false);
        let res;
        let op = self.name.clone() + " >> " + amount;
        if resultType.signed {
            res = self.wire.shiftArithRight(
                resultType.bitwidth,
                std::cmp::min(amount, resultType.bitwidth),
                op,
            );
        } else {
            res = self.wire.shiftRight(resultType.bitwidth, amount, op);
        }
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    /** EQ OPS **/

    pub fn isEqualTo(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " == " + &rhs.name;
        TypedWire::new(
            self.wire.isEqualTo(rhs.wire, op),
            zkbool(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn isNotEqualTo(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " != " + &rhs.name;
        TypedWire::new(
            self.wire.sub(rhs.wire, op).checkNonZero(op),
            zkbool(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    /** INEQ OPS **/

    pub fn isLessThan(&self, rhs: &TypedWire) -> TypedWire {
        let commonType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " < " + &rhs.name;
        if commonType.signed {
            let lhsSign = self
                .wire
                .getBitWires(commonType.bitwidth)
                .get(commonType.bitwidth - 1);
            let rhsSign = rhs
                .wire
                .getBitWires(commonType.bitwidth)
                .get(commonType.bitwidth - 1);

            let alwaysLt = lhsSign.isGreaterThan(rhsSign, 1);
            let sameSign = lhsSign.isEqualTo(rhsSign);
            let lhsLess = self.wire.isLessThan(rhs.wire, commonType.bitwidth);
            let isLt = alwaysLt.or(sameSign.and(lhsLess), op);
            TypedWire::new(isLt, zkbool(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire
                    .isLessThan(rhs.wire, std::cmp::min(253, commonType.bitwidth), op),
                zkbool(),
                op,
                &vec![],
                self.generator.clone(),
            )
        }
    }

    pub fn isLessThanOrEqual(&self, rhs: &TypedWire) -> TypedWire {
        let commonType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " <= " + &rhs.name;
        if commonType.signed {
            let lhsSign = self
                .wire
                .getBitWires(commonType.bitwidth)
                .get(commonType.bitwidth - 1);
            let rhsSign = rhs
                .wire
                .getBitWires(commonType.bitwidth)
                .get(commonType.bitwidth - 1);

            let alwaysLt = lhsSign.isGreaterThan(rhsSign, 1);
            let sameSign = lhsSign.isEqualTo(rhsSign);
            let lhsLessEq = self.wire.isLessThanOrEqual(rhs.wire, commonType.bitwidth);
            let isLt = alwaysLt.or(sameSign.and(lhsLessEq), op);
            TypedWire::new(isLt, zkbool(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire
                    .isLessThanOrEqual(rhs.wire, std::cmp::min(253, commonType.bitwidth), op),
                zkbool(),
                op,
                &vec![],
                self.generator.clone(),
            )
        }
    }

    pub fn isGreaterThan(&self, rhs: &TypedWire) -> TypedWire {
        rhs.isLessThan(self)
    }

    pub fn isGreaterThanOrEqual(&self, rhs: &TypedWire) -> TypedWire {
        rhs.isLessThanOrEqual(self)
    }

    /** BOOL OPS */

    pub fn and(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), &self.zkay_type);
        ZkayType::checkType(zkbool(), &rhs.zkay_type);
        let op = self.name.clone() + " && " + &rhs.name;
        TypedWire::new(
            self.wire.and(rhs.wire, op),
            zkbool(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn or(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), &self.zkay_type);
        ZkayType::checkType(zkbool(), &rhs.zkay_type);
        let op = self.name.clone() + " || " + &rhs.name;
        TypedWire::new(
            self.wire.or(rhs.wire, op),
            zkbool(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    fn handle_overflow(
        &self,
        w: &WireType,
        targetType: &ZkayType,
        was_mul: bool,
        name: &String,
    ) -> TypedWire {
        if targetType.bitwidth < 256 {
            // Downcast or result with overflow modulo < field prime -> modulo/mask lower bits
            let from_bits = std::cmp::min(
                256,
                if was_mul {
                    targetType.bitwidth * 2
                } else {
                    targetType.bitwidth + 1
                },
            );
            w = w.trimBits(from_bits, targetType.bitwidth, "% 2^" + targetType.bitwidth);
        }
        TypedWire::new(
            w,
            targetType,
            targetType.toString() + "(" + name + ")",
            &vec![],
            self.generator.clone(),
        )
    }
}
