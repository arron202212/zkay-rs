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
        let result_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " + " + &rhs.name;
        self.handle_overflow(
            &self.wire.clone().addw(&rhs.wire, &Some(op.clone())),
            &result_type,
            false,
            &op,
        )
    }

    pub fn minus(&self, rhs: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " - " + &rhs.name;
        let ret = self
            .wire
            .addw(&s_negate(rhs, &self.generator).wire, &Some(op.clone()));
        self.handle_overflow(&ret, &result_type, false, &op)
    }

    pub fn times(&self, rhs: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " * " + &rhs.name;
        if result_type.bitwidth == 256 {
            // Don't care about overflow with uint zkay_type
            return TypedWire::new(
                self.wire.mulw(&rhs.wire, &Some(op.clone())),
                result_type,
                op.clone(),
                &vec![],
                self.generator.clone(),
            );
        }
        if result_type.bitwidth <= 120 {
            // Result always fits into 240 < 253 bits
            return self.handle_overflow(
                &self.wire.mulw(&rhs.wire, &Some(op.clone())),
                &result_type,
                true,
                &op,
            );
        }

        // Result could overflow 253 bits -> do it in two halves to get correct overflow behavior
        let _lhs_lo_hi = self
            .wire
            .get_bit_wiresi(result_type.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);
        let _rhs_lo_hi = rhs
            .wire
            .get_bit_wiresi(result_type.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);

        // https://www.codeproject.com/Tips/618570/UInt-Multiplication-Squaring, BSD license
        let mut ans_lo_hi = _lhs_lo_hi[0]
            .as_ref()
            .unwrap()
            .mulw(
                _rhs_lo_hi[0].as_ref().unwrap(),
                &Some(op.clone() + "[lo*lo]"),
            )
            .get_bit_wiresi(result_type.bitwidth as u64, &None)
            .pack_bits_into_words(124, &None);
        let hi_lo_mul = self
            .handle_overflow(
                &_lhs_lo_hi[1].as_ref().unwrap().mulw(
                    _rhs_lo_hi[0].as_ref().unwrap(),
                    &Some(op.clone() + "[hi*lo]"),
                ),
                zk124(),
                true,
                &(op.clone() + "[hi*lo]"),
            )
            .wire
            .clone();
        let lo_hi_mul = self
            .handle_overflow(
                &_lhs_lo_hi[0].as_ref().unwrap().mulw(
                    _rhs_lo_hi[1].as_ref().unwrap(),
                    &Some(op.clone() + "[lo*hi]"),
                ),
                zk124(),
                true,
                &(op.clone() + "[lo*hi]"),
            )
            .wire
            .clone();
        let hi_lo_pluslo_hi = self
            .handle_overflow(
                &hi_lo_mul.addw(&lo_hi_mul, &Some(op.clone() + "[hi*lo + lo*hi]")),
                zk124(),
                false,
                &(op.clone() + "[hi*lo + lo*hi]"),
            )
            .wire
            .clone();
        ans_lo_hi[1] = Some(
            self.handle_overflow(
                &ans_lo_hi[1].as_ref().unwrap().addw(
                    &hi_lo_pluslo_hi,
                    &Some(op.clone() + "[anshi + hi*lo + lo*hi]"),
                ),
                zk124(),
                false,
                &(op.clone() + "[anshi + hi*lo + lo*hi]"),
            )
            .wire
            .clone(),
        );

        let ans = WireArray::new(ans_lo_hi, self.generator.clone().downgrade())
            .get_bits(124, &None)
            .pack_bits_into_words(
                result_type.bitwidth as usize,
                &Some(op.clone() + "[combine hi and lo]"),
            );
        assert!(ans.len() == 1, "Multiplication ans array has wrong length");
        TypedWire::new(
            ans[0].clone().unwrap(),
            result_type,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn divide_by(&self, rhs: &TypedWire, generator: RcCell<CircuitGenerator>) -> TypedWire {
        let result_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " / " + &rhs.name;
        // let mut generator = CircuitGenerator.get_active_circuit_generator();
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &rhs.wire.check_non_zero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let mut result_sign = generator.get_zero_wire().unwrap();
        let mut lhs_wire = self.wire.clone();
        let mut rhs_wire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhs_sign = lhs_wire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None)
                [self.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            lhs_wire = lhs_sign.mux(&s_negate(self, &self.generator).wire, &lhs_wire);
            result_sign = result_sign.xor_bitwises(&lhs_sign, 1, &None);
        }
        if rhs.zkay_type.signed {
            let rhs_sign = rhs_wire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None)
                [rhs.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            rhs_wire = rhs_sign.mux(&s_negate(rhs, &self.generator).wire, &rhs_wire);
            result_sign = result_sign.xor_bitwises(&rhs_sign, 1, &None);
        }

        // Need to operate on integers:long , regular div / mod gadget only works for <= 126 bits
        let lhs_long = LongElement::newa(
            lhs_wire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let rhs_long = LongElement::newa(
            rhs_wire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let mut q = LongIntegerFloorDivGadget::new(
            lhs_long,
            rhs_long,
            0,
            &Some(op.clone()),
            self.generator.clone(),
        )
        .get_quotient()
        .clone();
        let res_abs = q
            .get_bitsi(result_type.bitwidth)
            .pack_bits_into_words(result_type.bitwidth as usize, &None)[0]
            .clone()
            .unwrap();

        let res_pos = TypedWire::new(
            res_abs,
            result_type.clone(),
            op.clone(),
            &vec![],
            self.generator.clone(),
        );
        let res_neg = s_negate(&res_pos, &self.generator);
        TypedWire::new(
            result_sign.mux(&res_neg.wire, &res_pos.wire),
            result_type,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn modulo(&self, rhs: &TypedWire, generator: RcCell<CircuitGenerator>) -> TypedWire {
        let result_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " % " + &rhs.name;
        // let mut generator = CircuitGenerator.get_active_circuit_generator();
        CircuitGenerator::add_one_assertion(
            generator.clone(),
            &rhs.wire.check_non_zero(&None),
            &Some("no div by 0".to_owned()),
        );

        // Sign handling...
        let mut result_sign = generator.get_zero_wire().unwrap();
        let mut lhs_wire = self.wire.clone();
        let mut rhs_wire = rhs.wire.clone();

        if self.zkay_type.signed {
            let lhs_sign = lhs_wire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None)
                [self.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            lhs_wire = lhs_sign.mux(&s_negate(self, &self.generator).wire, &lhs_wire);
            result_sign = lhs_sign;
        }
        if rhs.zkay_type.signed {
            let rhs_sign = rhs_wire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None)
                [rhs.zkay_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            rhs_wire = rhs_sign.mux(&s_negate(rhs, &self.generator).wire, &rhs_wire);
        }

        // Need to operate on long integers, regular div / mod gadget only works for <= 126 bits
        let lhs_long = LongElement::newa(
            lhs_wire.get_bit_wiresi(self.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let rhs_long = LongElement::newa(
            rhs_wire.get_bit_wiresi(rhs.zkay_type.bitwidth as u64, &None),
            self.generator.clone().downgrade(),
        );
        let mut r = LongIntegerModGadget::new(
            lhs_long,
            rhs_long,
            0,
            true,
            &Some(op.clone()),
            self.generator.clone(),
        )
        .get_remainder()
        .clone();
        let res_abs = r
            .get_bitsi(result_type.bitwidth)
            .pack_bits_into_words(result_type.bitwidth as usize, &None)[0]
            .clone();

        let res_pos = TypedWire::new(
            res_abs.unwrap(),
            result_type.clone(),
            op.clone(),
            &vec![],
            self.generator.clone(),
        );
        let res_neg = s_negate(&res_pos, &self.generator);
        TypedWire::new(
            result_sign.mux(&res_neg.wire, &res_pos.wire),
            result_type,
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    //  BIT OPS

    pub fn bit_or(&self, rhs: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_typeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " | " + &rhs.name;
        let res = self
            .wire
            .or_bitwises(&rhs.wire, result_type.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, result_type, op, &vec![], self.generator.clone())
    }

    pub fn bit_and(&self, rhs: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_typeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " & " + &rhs.name;
        let res = self
            .wire
            .and_bitwises(&rhs.wire, result_type.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, result_type, op, &vec![], self.generator.clone())
    }

    pub fn bit_xor(&self, rhs: &TypedWire) -> TypedWire {
        let result_type = ZkayType::check_typeb(&self.zkay_type, &rhs.zkay_type, false);
        let op = self.name.clone() + " ^ " + &rhs.name;
        let res = self
            .wire
            .xor_bitwises(&rhs.wire, result_type.bitwidth as u64, &Some(op.clone()));
        TypedWire::new(res, result_type, op, &vec![], self.generator.clone())
    }

    //  SHIFT OPS

    pub fn shift_left_by(&self, amount: i32) -> TypedWire {
        let result_type = ZkayType::check_typeb(&self.zkay_type, &self.zkay_type, false);
        let op = self.name.clone() + " << " + &amount.to_string();
        let res = self.wire.shift_left(
            result_type.bitwidth as usize,
            amount as usize,
            &Some(op.clone()),
        );
        TypedWire::new(res, result_type, op, &vec![], self.generator.clone())
    }

    pub fn shift_right_by(&self, amount: i32) -> TypedWire {
        let result_type = ZkayType::check_typeb(&self.zkay_type, &self.zkay_type, false);
        let res;
        let op = self.name.clone() + " >> " + &amount.to_string();
        if result_type.signed {
            res = self.wire.shift_arith_right(
                result_type.bitwidth as usize,
                amount.min(result_type.bitwidth) as usize,
                &Some(op.clone()),
            );
        } else {
            res = self.wire.shift_right(
                result_type.bitwidth as usize,
                amount as usize,
                &Some(op.clone()),
            );
        }
        TypedWire::new(res, result_type, op, &vec![], self.generator.clone())
    }

    //  EQ OPS

    pub fn is_equal_tos(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " == " + &rhs.name;
        TypedWire::new(
            self.wire.is_equal_tos(&rhs.wire, &Some(op.clone())),
            zkbool().clone(),
            op,
            &vec![],
            self.generator.clone(),
        )
    }

    pub fn is_not_equal_to(&self, rhs: &TypedWire) -> TypedWire {
        ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
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
        let common_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " < " + &rhs.name;
        if common_type.signed {
            let lhs_sign = self.wire.get_bit_wiresi(common_type.bitwidth as u64, &None)
                [common_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            let rhs_sign = rhs.wire.get_bit_wiresi(common_type.bitwidth as u64, &None)
                [common_type.bitwidth as usize - 1]
                .clone()
                .unwrap();

            let always_lt = lhs_sign.is_greater_thans(&rhs_sign, 1, &None);
            let same_sign = lhs_sign.is_equal_tos(&rhs_sign, &None);
            let lhs_less = self
                .wire
                .is_less_thans(&rhs.wire, common_type.bitwidth, &None);
            let is_lt = always_lt.orw(&same_sign.and(&lhs_less, &None), &Some(op.clone()));
            TypedWire::new(is_lt, zkbool().clone(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire.is_less_thans(
                    &rhs.wire,
                    common_type.bitwidth.min(253),
                    &Some(op.clone()),
                ),
                zkbool().clone(),
                op,
                &vec![],
                self.generator.clone(),
            )
        }
    }

    pub fn is_less_than_or_equals(&self, rhs: &TypedWire) -> TypedWire {
        let common_type = ZkayType::check_type(&self.zkay_type, &rhs.zkay_type);
        let op = self.name.clone() + " <= " + &rhs.name;
        if common_type.signed {
            let lhs_sign = self.wire.get_bit_wiresi(common_type.bitwidth as u64, &None)
                [common_type.bitwidth as usize - 1]
                .clone()
                .unwrap();
            let rhs_sign = rhs.wire.get_bit_wiresi(common_type.bitwidth as u64, &None)
                [common_type.bitwidth as usize - 1]
                .clone()
                .unwrap();

            let always_lt = lhs_sign.is_greater_thans(&rhs_sign, 1, &None);
            let same_sign = lhs_sign.is_equal_tos(&rhs_sign, &None);
            let lhs_less_eq =
                self.wire
                    .is_less_than_or_equals(&rhs.wire, common_type.bitwidth, &None);
            let is_lt = always_lt.orw(&same_sign.and(&lhs_less_eq, &None), &Some(op.clone()));
            TypedWire::new(is_lt, zkbool().clone(), op, &vec![], self.generator.clone())
        } else {
            // Note: breaks if value > 253 bit
            TypedWire::new(
                self.wire.is_less_than_or_equals(
                    &rhs.wire,
                    common_type.bitwidth.min(253),
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
        ZkayType::check_type(zkbool(), &self.zkay_type);
        ZkayType::check_type(zkbool(), &rhs.zkay_type);
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
        ZkayType::check_type(zkbool(), &self.zkay_type);
        ZkayType::check_type(zkbool(), &rhs.zkay_type);
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
        target_type: &ZkayType,
        was_mul: bool,
        name: &String,
    ) -> TypedWire {
        let mut w = w.clone();
        if target_type.bitwidth < 256 {
            // Downcast or result with overflow modulo < field prime -> modulo/mask lower bits
            let from_bits = std::cmp::min(
                256,
                if was_mul {
                    target_type.bitwidth * 2
                } else {
                    target_type.bitwidth + 1
                },
            );
            w = w.trim_bits(
                from_bits,
                target_type.bitwidth,
                &Some(format!("% 2^{}", target_type.bitwidth)),
            );
        }
        TypedWire::new(
            w,
            target_type.clone(),
            format!("{}({})", target_type, name),
            &vec![],
            self.generator.clone(),
        )
    }
}
