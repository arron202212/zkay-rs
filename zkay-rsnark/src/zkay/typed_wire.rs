

use circuit::auxiliary::long_element;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use circuit::structure::wire_array;
use examples::gadgets::math::long_integer_floor_div_gadget;
use examples::gadgets::math::long_integer_mod_gadget;

use  zkay::zkay_circuit_base::negate;
use  zkay::zkay_type::*;

pub struct TypedWire {
    pub   wire:Wire,
    pub  type:ZkayType,
    pub  name:String,
}
impl TypedWire {
    pub  new(wire:Wire , type:ZkayType , name:String , restrict:Vec<bool>)->Self {
        assert!(wire.is_some() && type.is_some(),"Arguments cannot be null");
        
        if (restrict.length > 0 && restrict[0]) || ZkayUtil.ZKAY_RESTRICT_EVERYTHING {
            wire.restrictBitLength(type.bitwidth);
        }
        self.wire = wire;
        self.type = type;
        self.name = name;
    }

    /** ARITH OPS **/

    pub  fn plus(rhs:TypedWire )->  TypedWire {
        let  resultType = checkType(self.type, rhs.type);
        let  op = self.name + " + " + rhs.name;
        return handle_overflow(self.wire.add(rhs.wire, op), resultType, false, op);
    }

    pub  fn minus(rhs:TypedWire )->  TypedWire {
        let resultType = checkType(self.type, rhs.type);
        let op = self.name + " - " + rhs.name;
        let ret = self.wire.add(negate(rhs).wire, op);
        return handle_overflow(ret, resultType, false, op);
    }

    pub  fn times(rhs:TypedWire )->  TypedWire {
        let resultType = checkType(self.type, rhs.type);
        let op = self.name + " * " + rhs.name;
        if resultType.bitwidth == 256 {
            // Don't care about overflow with uint type
            return TypedWire::new(self.wire.mul(rhs.wire, op), resultType, op);
        }
         if resultType.bitwidth <= 120 {
            // Result always fits into 240 < 253 bits
            return handle_overflow(self.wire.mul(rhs.wire, op), resultType, true, op);
        }
       
            // Result could overflow 253 bits -> do it in two halves to get correct overflow behavior
            let LhsLoHi = self.wire.getBitWires(resultType.bitwidth).packBitsIntoWords(124);
            let RhsLoHi = rhs.wire.getBitWires(resultType.bitwidth).packBitsIntoWords(124);

            // https://www.codeproject.com/Tips/618570/UInt-Multiplication-Squaring, BSD license
            let ansLoHi = LhsLoHi[0].mul(RhsLoHi[0], op + "[lo*lo]").getBitWires(resultType.bitwidth).packBitsIntoWords(124);
            let hiLoMul = handle_overflow(LhsLoHi[1].mul(RhsLoHi[0], op + "[hi*lo]"), Zk124, true, op + "[hi*lo]").wire;
            let loHiMul = handle_overflow(LhsLoHi[0].mul(RhsLoHi[1], op + "[lo*hi]"), Zk124, true, op + "[lo*hi]").wire;
            let hiLoPlusloHi = handle_overflow(hiLoMul.add(loHiMul, op + "[hi*lo + lo*hi]"), Zk124, false, op + "[hi*lo + lo*hi]").wire;
            ansLoHi[1] = handle_overflow(ansLoHi[1].add(hiLoPlusloHi, op + "[anshi + hi*lo + lo*hi]"), Zk124, false, op + "[anshi + hi*lo + lo*hi]").wire;

            let ans = WireArray::new(ansLoHi).getBits(124).packBitsIntoWords(resultType.bitwidth, op + "[combine hi and lo]");
                assert!(ans.length == 1,"Multiplication ans array has wrong length");
            return TypedWire::new(ans[0], resultType, op);
        
    }

    pub  fn divideBy(rhs:TypedWire )->  TypedWire {
        let  resultType = checkType(self.type, rhs.type);
        let op = self.name + " / " + rhs.name;
        let generator = CircuitGenerator.getActiveCircuitGenerator();
        generator.addOneAssertion(rhs.wire.checkNonZero(), "no div by 0");

        // Sign handling...
        let resultSign = generator.getZeroWire();
        let lhsWire = self.wire;
        let rhsWire = rhs.wire;

        if self.type.signed {
            let lhsSign = lhsWire.getBitWires(self.type.bitwidth).get(self.type.bitwidth - 1);
            lhsWire = lhsSign.mux(negate(this).wire, lhsWire);
            resultSign = resultSign.xorBitwise(lhsSign, 1);
        }
        if rhs.type.signed {
            let rhsSign = rhsWire.getBitWires(rhs.type.bitwidth).get(rhs.type.bitwidth - 1);
            rhsWire = rhsSign.mux(negate(rhs).wire, rhsWire);
            resultSign = resultSign.xorBitwise(rhsSign, 1);
        }

        // Need to operate on integers:long , regular div / mod gadget only works for <= 126 bits
        let lhsLong = LongElement::new(lhsWire.getBitWires(self.type.bitwidth));
        let rhsLong = LongElement::new(rhsWire.getBitWires(rhs.type.bitwidth));
        let q = LongIntegerFloorDivGadget::new(lhsLong, rhsLong, op).getQuotient();
        let resAbs = q.getBits(resultType.bitwidth).packBitsIntoWords(resultType.bitwidth)[0];

        let resPos = TypedWire::new(resAbs, resultType, op);
        let resNeg = negate(resPos);
        return TypedWire::new(resultSign.mux(resNeg.wire, resPos.wire), resultType, op);
    }

    pub  fn modulo(rhs:TypedWire )->  TypedWire {
        let resultType = checkType(self.type, rhs.type);
        let op = self.name + " % " + rhs.name;
        let generator = CircuitGenerator.getActiveCircuitGenerator();
        generator.addOneAssertion(rhs.wire.checkNonZero(), "no div by 0");

        // Sign handling...
        let resultSign = generator.getZeroWire();
        let lhsWire = self.wire;
        let rhsWire = rhs.wire;

        if self.type.signed {
            Wire lhsSign = lhsWire.getBitWires(self.type.bitwidth).get(self.type.bitwidth - 1);
            lhsWire = lhsSign.mux(negate(this).wire, lhsWire);
            resultSign = lhsSign;
        }
        if rhs.type.signed {
            Wire rhsSign = rhsWire.getBitWires(rhs.type.bitwidth).get(rhs.type.bitwidth - 1);
            rhsWire = rhsSign.mux(negate(rhs).wire, rhsWire);
        }

        // Need to operate on long integers, regular div / mod gadget only works for <= 126 bits
let lhsLong =  LongElement::new(lhsWire.getBitWires(self.type.bitwidth));
let rhsLong =  LongElement::new(rhsWire.getBitWires(rhs.type.bitwidth));
let r =  LongIntegerModGadget::new(lhsLong, rhsLong, true, op).getRemainder();
let resAbs =  r.getBits(resultType.bitwidth).packBitsIntoWords(resultType.bitwidth)[0];

let resPos =  TypedWire::new(resAbs, resultType, op);
let resNeg =  negate(resPos);
        return TypedWire::new(resultSign.mux(resNeg.wire, resPos.wire), resultType, op);
    }

    /** BIT OPS */

    pub  fn bitOr(rhs:TypedWire )->  TypedWire {
let resultType =  checkType(self.type, rhs.type, false);
let op =  self.name + " | " + rhs.name;
let res =  self.wire.orBitwise(rhs.wire, resultType.bitwidth, op);
        return TypedWire::new(res, resultType, op);
    }

    pub  fn bitAnd(rhs:TypedWire )->  TypedWire {
let resultType =  checkType(self.type, rhs.type, false);
let op =  self.name + " & " + rhs.name;
let res =  self.wire.andBitwise(rhs.wire, resultType.bitwidth, op);
        return TypedWire::new(res, resultType, op);
    }

    pub  fn bitXor(rhs:TypedWire )->  TypedWire {
let resultType =  checkType(self.type, rhs.type, false);
let op =  self.name + " ^ " + rhs.name;
let res =  self.wire.xorBitwise(rhs.wire, resultType.bitwidth, op);
        return TypedWire::new(res, resultType, op);
    }

    /** SHIFT OPS */

    pub  fn shiftLeftBy(amount:i32 )->  TypedWire {
let resultType =  checkType(self.type, self.type, false);
let op =  self.name + " << " + amount;
let res =  self.wire.shiftLeft(resultType.bitwidth, amount, op);
        return TypedWire::new(res, resultType, op);
    }

    pub  fn shiftRightBy(amount:i32 )->  TypedWire {
let resultType =  checkType(self.type, self.type, false);
        Wire res;
let op =  self.name + " >> " + amount;
        if resultType.signed {
            res = self.wire.shiftArithRight(resultType.bitwidth, std::cmp::min(amount, resultType.bitwidth), op);
        } else {
            res = self.wire.shiftRight(resultType.bitwidth, amount, op);
        }
        return TypedWire::new(res, resultType, op);
    }

    /** EQ OPS **/

    pub  fn isEqualTo(rhs:TypedWire )->  TypedWire {
        checkType(self.type, rhs.type);
       let op =  self.name + " == " + rhs.name;
        return TypedWire::new(self.wire.isEqualTo(rhs.wire, op), ZkBool, op);
    }

    pub  fn isNotEqualTo(rhs:TypedWire )->  TypedWire {
        checkType(self.type, rhs.type);
       let op =  self.name + " != " + rhs.name;
        return TypedWire::new(self.wire.sub(rhs.wire, op).checkNonZero(op), ZkBool, op);
    }

    /** INEQ OPS **/

    pub  fn isLessThan(rhs:TypedWire )->  TypedWire {
       let commonType =  checkType(self.type, rhs.type);
       let op =  self.name + " < " + rhs.name;
        if commonType.signed {
           let lhsSign =  self.wire.getBitWires(commonType.bitwidth).get(commonType.bitwidth-1);
           let rhsSign =  rhs.wire.getBitWires(commonType.bitwidth).get(commonType.bitwidth-1);

           let alwaysLt =  lhsSign.isGreaterThan(rhsSign, 1);
           let sameSign =  lhsSign.isEqualTo(rhsSign);
           let lhsLess =  self.wire.isLessThan(rhs.wire, commonType.bitwidth);
           let isLt =  alwaysLt.or(sameSign.and(lhsLess), op);
            return TypedWire::new(isLt, ZkBool, op);
        } else {
            // Note: breaks if value > 253 bit
            return TypedWire::new(self.wire.isLessThan(rhs.wire, std::cmp::min(253, commonType.bitwidth), op), ZkBool, op);
        }
    }

    pub  fn isLessThanOrEqual(rhs:TypedWire )->  TypedWire {
       let commonType =  checkType(self.type, rhs.type);
       let op =  self.name + " <= " + rhs.name;
        if commonType.signed {
           let lhsSign =  self.wire.getBitWires(commonType.bitwidth).get(commonType.bitwidth-1);
           let rhsSign =  rhs.wire.getBitWires(commonType.bitwidth).get(commonType.bitwidth-1);

           let alwaysLt =  lhsSign.isGreaterThan(rhsSign, 1);
           let sameSign =  lhsSign.isEqualTo(rhsSign);
           let lhsLessEq =  self.wire.isLessThanOrEqual(rhs.wire, commonType.bitwidth);
           let isLt =  alwaysLt.or(sameSign.and(lhsLessEq), op);
            return TypedWire::new(isLt, ZkBool, op);
        } else {
            // Note: breaks if value > 253 bit
            return TypedWire::new(self.wire.isLessThanOrEqual(rhs.wire, std::cmp::min(253, commonType.bitwidth), op), ZkBool, op);
        }
    }

    pub  fn isGreaterThan(rhs:TypedWire )->  TypedWire {
        return rhs.isLessThan(this);
    }

    pub  fn isGreaterThanOrEqual(rhs:TypedWire )->  TypedWire {
        return rhs.isLessThanOrEqual(this);
    }

    /** BOOL OPS */

    pub  fn and(rhs:TypedWire )->  TypedWire {
        checkType(ZkBool, self.type);
        checkType(ZkBool, rhs.type);
       let op =  self.name + " && " + rhs.name;
        return TypedWire::new(self.wire.and(rhs.wire, op), ZkBool, op);
    }

    pub  fn or(rhs:TypedWire )->  TypedWire {
        checkType(ZkBool, self.type);
        checkType(ZkBool, rhs.type);
       let op =  self.name + " || " + rhs.name;
        return TypedWire::new(self.wire.or(rhs.wire, op), ZkBool, op);
    }

      TypedWire handle_overflow(w:Wire , targetType:ZkayType , was_mul:bool , name:String ) {
        if targetType.bitwidth < 256 {
            // Downcast or result with overflow modulo < field prime -> modulo/mask lower bits
           let from_bits =  std::cmp::min(256, was_mul  { targetType.bitwidth * 2 }else { targetType.bitwidth + 1});
            w = w.trimBits(from_bits, targetType.bitwidth, "% 2^" + targetType.bitwidth);
        }
        return TypedWire::new(w, targetType, targetType.toString() + "(" + name + ")");
    }
}
