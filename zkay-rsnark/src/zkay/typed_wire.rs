#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::{
    circuit::{
        auxiliary::long_element::LongElement,
        structure::{
            circuit_generator::{
                CGConfig, CGConfigFields, CircuitGenerator, CircuitGeneratorExtend,
                add_to_evaluation_queue, get_active_circuit_generator,
            },
            wire::WireConfig,
            wire_array::WireArray,
            wire_type::WireType,
        },
    },
    examples::gadgets::math::{
        long_integer_division::LongIntegerDivisionConfig,
        long_integer_floor_div_gadget::LongIntegerFloorDivGadget,
        long_integer_mod_gadget::LongIntegerModGadget,
    },
    zkay::{
        zkay_circuit_base::s_negate,
        zkay_type::{ZkayType, zk124, zkbool},
        zkay_util::ZkayUtil,
    },
};

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
            wire.restrict_bit_length(zkay_type.bitwidth as u64, &None);
        }
        Self {
            wire,
            zkay_type,
            name,
            generator,
        }
    }

    //  ARITH OPS

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
        let ret = self
            .wire
            .addw(&s_negate(rhs, &self.generator).wire, &Some(op.clone()));
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
            .get_bit_wiresi(resultType.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);
        let RhsLoHi = rhs
            .wire
            .get_bit_wiresi(resultType.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);

        // https://www.codeproject.com/Tips/618570/UInt-Multiplication-Squaring, BSD license
        let mut ansLoHi = LhsLoHi[0]
            .as_ref()
            .unwrap()
            .mulw(RhsLoHi[0].as_ref().unwrap(), &Some(op.clone() + "[lo*lo]"))
            .get_bit_wiresi(resultType.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);
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
                    .mulw(RhsLoHi[1].as_ref().unwrap(), &Some(op.clone() + "[lo*hi]")),
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
        ansLoHi[1] = Some(
            self.handle_overflow(
                &ansLoHi[1]
                    .as_ref()
                    .unwrap()
                    .addw(&hiLoPlusloHi, &Some(op.clone() + "[anshi + hi*lo + lo*hi]")),
                zk124(),
                false,
                &(op.clone() + "[anshi + hi*lo + lo*hi]"),
            )
            .wire
            .clone(),
        );

        let ans = WireArray::new(ansLoHi, self.generator.clone().downgrade())
            .get_bits(124, &None)
            .pack_bits_into_words(
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

    pub fn divideBy(&self, rhs: &TypedWire, generator: RcCell<CircuitGenerator>) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " / " + &rhs.name;
        // let mut generator = CircuitGenerator.get_active_circuit_generator();
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &rhs.wire.check_non_zero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let mut resultSign = generator.get_zero_wire().unwrap();
        let mut lhsWire = self.wire.clone();
        let mut rhsWire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhsSign = lhsWire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None)
                [self.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            lhsWire = lhsSign.mux(&s_negate(self, &self.generator).wire, &lhsWire);
            resultSign = resultSign.xor_bitwise(&lhsSign, 1, &None);
        }
        if rhs.zkay_type.signed {
            let rhsSign = rhsWire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None)
                [rhs.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            rhsWire = rhsSign.mux(&s_negate(rhs, &self.generator).wire, &rhsWire);
            resultSign = resultSign.xor_bitwise(&rhsSign, 1, &None);
        }

        // Need to operate on integers:long , regular div / mod gadget only works for <= 126 bits
        let lhsLong = LongElement::newa(
            lhsWire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let rhsLong = LongElement::newa(
            rhsWire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let mut q = LongIntegerFloorDivGadget::new(
            lhsLong,
            rhsLong,
            0,
            &Some(op.clone()),
            self.generator.clone(),
        )
        .getQuotient()
        .clone();
        let resAbs = q
            .get_bitsi(resultType.bitwidth)
            .pack_bits_into_words(resultType.bitwidth as usize, &None)[0]
            .clone()
            .unwrap();

        let resPos = TypedWire::new(
            resAbs,
            resultType.clone(),
            op.clone(),
            &vec![],
            self.generator.clone(),
        );
        let resNeg = s_negate(&resPos, &self.generator);
        TypedWire::new(
            resultSign.mux(&resNeg.wire, &resPos.wire),
            resultType,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn modulo(&self, rhs: &TypedWire, generator: RcCell<CircuitGenerator>) -> TypedWire {
        let resultType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " % " + &rhs.name;
        // let mut generator = CircuitGenerator.get_active_circuit_generator();
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &rhs.wire.check_non_zero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let mut resultSign = generator.get_zero_wire().unwrap();
        let mut lhsWire = self.wire.clone();
        let mut rhsWire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhsSign = lhsWire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None)
                [self.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            lhsWire = lhsSign.mux(&s_negate(self, &self.generator).wire, &lhsWire);
            resultSign = lhsSign;
        }
        if rhs.zkay_type.signed {
            let rhsSign = rhsWire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None)
                [rhs.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            rhsWire = rhsSign.mux(&s_negate(rhs, &self.generator).wire, &rhsWire);
        }

        // Need to operate on long integers, regular div / mod gadget only works for <= 126 bits
        let lhsLong = LongElement::newa(
            lhsWire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let rhsLong = LongElement::newa(
            rhsWire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let mut r = LongIntegerModGadget::new(
            lhsLong,
            rhsLong,
            0,
            true,
            &Some(op.clone()),
            self.generator.clone(),
        )
        .getRemainder()
        .clone();
        let resAbs = r
            .get_bitsi(resultType.bitwidth)
            .pack_bits_into_words(resultType.bitwidth as usize, &None)[0]
            .clone();

        let resPos = TypedWire::new(
            resAbs.unwrap(),
            resultType.clone(),
            op.clone(),
            &vec![],
            self.generator.clone(),
        );
        let resNeg = s_negate(&resPos, &self.generator);
        TypedWire::new(
            resultSign.mux(&resNeg.wire, &resPos.wire),
            resultType,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    //  BIT OPS

    pub fn bitOr(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " | " + &rhs.name;
        let res = self
            .wire
            .or_bitwises(&rhs.wire, resultType.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn bitAnd(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " & " + &rhs.name;
        let res = self
            .wire
            .and_bitwise(&rhs.wire, resultType.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn bitXor(&self, rhs: &TypedWire) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " ^ " + &rhs.name;
        let res = self
            .wire
            .xor_bitwise(&rhs.wire, resultType.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    //  SHIFT OPS

    pub fn shift_leftBy(&self, amount: i32) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &self.zkay_type, false);
        let op = self.name.clone() + " << " + &amount.to_string();
        let res = self.wire.shift_left(
            resultType.bitwidth as usize,
            amount as usize,
            &Some(op.clone()),
        );
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    pub fn shift_rightBy(&self, amount: i32) -> TypedWire {
        let resultType = ZkayType::checkTypeb(&self.zkay_type, &self.zkay_type, false);
        let res;
        let op = self.name.clone() + " >> " + &amount.to_string();
        if resultType.signed {
            res = self.wire.shift_arith_right(
                resultType.bitwidth as usize,
                amount.min(resultType.bitwidth) as usize,
                &Some(op.clone()),
            );
        } else {
            res = self.wire.shift_right(
                resultType.bitwidth as usize,
                amount as usize,
                &Some(op.clone()),
            );
        }
        TypedWire::new(res, resultType, op, &vec![], self.generator.clone())
    }

    //  EQ OPS

    pub fn is_equal_tos(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " == " + &rhs.name;
        TypedWire::new(
            self.wire.is_equal_tos(&rhs.wire, &Some(op.clone())),
            zkbool().clone(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn isNotEqualTo(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " != " + &rhs.name;
        TypedWire::new(
            self.wire
                .subw(&rhs.wire, &Some(op.clone()))
                .check_non_zero(&Some(op.clone())),
            zkbool().clone(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    //  INEQ OPS

    pub fn is_less_thans(&self, rhs: &TypedWire) -> TypedWire {
        let commonType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " < " + &rhs.name;
        if commonType.signed {
            let lhsSign = self.wire.get_bit_wiresi(commonType.bitwidth as u64, &None)
                [commonType.bitwidth as usize - 1]
                .clone()
                .unwrap();
            let rhsSign = rhs.wire.get_bit_wiresi(commonType.bitwidth as u64, &None)
                [commonType.bitwidth as usize - 1]
                .clone()
                .unwrap();

            let alwaysLt = lhsSign.is_greater_thans(&rhsSign, 1, &None);
            let sameSign = lhsSign.is_equal_tos(&rhsSign, &None);
            let lhsLess = self
                .wire
                .is_less_thans(&rhs.wire, commonType.bitwidth, &None);
            let isLt = alwaysLt.orw(&sameSign.and(&lhsLess, &None), &Some(op.clone()));
            TypedWire::new(isLt, zkbool().clone(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire
                    .is_less_thans(&rhs.wire, commonType.bitwidth.min(253), &Some(op.clone())),
                zkbool().clone(),
                op,
                &vec![],
                self.generator.clone(),
            )
        }
    }

    pub fn is_less_than_or_equals(&self, rhs: &TypedWire) -> TypedWire {
        let commonType = ZkayType::checkType(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " <= " + &rhs.name;
        if commonType.signed {
            let lhsSign = self.wire.get_bit_wiresi(commonType.bitwidth as u64, &None)
                [commonType.bitwidth as usize - 1]
                .clone()
                .unwrap();
            let rhsSign = rhs.wire.get_bit_wiresi(commonType.bitwidth as u64, &None)
                [commonType.bitwidth as usize - 1]
                .clone()
                .unwrap();

            let alwaysLt = lhsSign.is_greater_thans(&rhsSign, 1, &None);
            let sameSign = lhsSign.is_equal_tos(&rhsSign, &None);
            let lhsLessEq = self
                .wire
                .is_less_than_or_equals(&rhs.wire, commonType.bitwidth, &None);
            let isLt = alwaysLt.orw(&sameSign.and(&lhsLessEq, &None), &Some(op.clone()));
            TypedWire::new(isLt, zkbool().clone(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire.is_less_than_or_equals(
                    &rhs.wire,
                    commonType.bitwidth.min(253),
                    &Some(op.clone()),
                ),
                zkbool().clone(),
                op,
                &vec![],
                self.generator.clone(),
            )
        }
    }

    pub fn is_greater_thans(&self, rhs: &TypedWire) -> TypedWire {
        rhs.is_less_thans(self)
    }

    pub fn is_greater_than_or_equals(&self, rhs: &TypedWire) -> TypedWire {
        rhs.is_less_than_or_equals(self)
    }

    //  BOOL OPS

    pub fn and(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::checkType(zkbool(), &self.zkay_type);
        ZkayType::checkType(zkbool(), &rhs.zkay_type);
        let op = self.name.clone() + " && " + &rhs.name;
        TypedWire::new(
            self.wire.and(&rhs.wire, &Some(op.clone())),
            zkbool().clone(),
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
            self.wire.orw(&rhs.wire, &Some(op.clone())),
            zkbool().clone(),
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
        let mut w = w.clone();
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
            w = w.trim_bits(
                from_bits,
                targetType.bitwidth,
                &Some(format!("% 2^{}", targetType.bitwidth)),
            );
        }
        TypedWire::new(
            w,
            targetType.clone(),
            format!("{}({})", targetType, name),
            &vec![],
            self.generator.clone(),
        )
    }
}
