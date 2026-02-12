//  Unit tests for gadgetlib2 variables

use crate::gadgetlib2::constraint::ConstraintConfig;
use crate::gadgetlib2::pp::{Fp, initPublicParamsFromDefaultPp};
use crate::gadgetlib2::variable::{
    DualWord, FElem, FElemInterface, FieldType, LinearCombination, LinearTerm, Monomial,
    MultiPackedWord, Polynomial, UnpackedWord, Variable, VariableArray, VariableArrayBase,
    VariableArrayConfig, VariableAssignment, VariableSet,
};
use ffec::Field;

#[cfg(test)]
mod tests {
    use super::*;

    fn orderFunc(a: &Variable, b: &Variable) -> bool {
        a.index_ < b.index_
    }

    #[test]
    fn test_VariableNaming() {
        let mut v1 = Variable::default();
        assert_eq!(v1.name(), "");
        let mut v2 = Variable::from("foo");
        // #   ifdef DEBUG
        assert_eq!(v2.name(), "foo");
        // #   endif
        v2 = v1.clone();
        assert_eq!(v2.name(), "");
    }

    #[test]
    fn test_VariableStrictOrdering() {
        let mut v1 = Variable::default();
        let mut v2 = Variable::default();
        // Variable::VariableStrictOrder orderFunc;
        assert!(orderFunc(&v1, &v2) || orderFunc(&v2, &v1)); // check strict ordering
        v2 = v1.clone();
        assert!(!orderFunc(&v1, &v2) || orderFunc(&v2, &v1));
    }

    #[test]
    fn test_VariableSet() {
        let mut v1 = Variable::default();
        let mut s1 = VariableSet::default();
        s1.insert(v1.clone());
        assert_eq!(s1.len(), 1usize);
        let mut v2 = Variable::default();
        v2 = v1.clone();
        s1.insert(v2);
        assert_eq!(s1.len(), 1usize);
        let mut v3 = Variable::default();
        s1.insert(v3);
        assert_eq!(s1.len(), 2usize);
        let mut v4 = Variable::default();
        s1.remove(&v4);
        assert_eq!(s1.len(), 2usize);
        v4 = v1.clone();
        s1.remove(&v4);
        assert_eq!(s1.len(), 1usize);
    }

    #[test]
    fn test_VariableArray() {
        let v1 = Variable::default();
        let v2 = Variable::from("v2");
        let mut vArr = VariableArray::<VariableArrayBase>::default();
        vArr.contents.push(v1);
        vArr.contents.push(v2);
        assert_eq!(vArr.len(), 2usize);
        assert!(orderFunc(&vArr[0], &vArr[1]) || orderFunc(&vArr[1], &vArr[0])); // check strict ordering
        vArr[1] = vArr[0].clone();
        assert!(!orderFunc(&vArr[0], &vArr[1]) || orderFunc(&vArr[1], &vArr[0])); // check strict ordering
        assert!(vArr.contents.get(2).is_none()); // = v1, ::std::out_of_range);
    }

    #[test]
    fn test_VariableEval() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut ass = VariableAssignment::default();
        ass.insert(x[0].clone(), FElem::froms(Fp::from(42)));
        assert_eq!(x[0].eval(&ass), 42);
        assert_ne!(x[0].eval(&ass), 17);
    }

    #[test]
    fn test_FElem_FConst_fromLong() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(0i64);
        assert!(e0 == 0);
    }

    #[test]
    fn test_FElem_FConst_fromInt() {
        initPublicParamsFromDefaultPp();
        let e1 = FElem::from(1);
        assert!(e1 == 1);
        let e2 = FElem::from(2);
        assert_eq!(e2, FElem::from(2));
    }

    #[test]
    fn test_FElem_FConst_copy() {
        initPublicParamsFromDefaultPp();
        let mut e0 = FElem::from(0i64);
        let mut e1 = FElem::from(1);
        let mut e3 = FElem::from(e1.clone());
        assert!(e3 == 1);
        e3 = 0.into();
        assert!(e3 == 0);
        assert_eq!(e1, 1);
        e0 = e1.clone();
        assert_eq!(e0, e1);
        e0 = 0.into();
        assert_ne!(e0, e1);
    }

    #[test]
    fn test_FElem_FConst_move() {
        initPublicParamsFromDefaultPp();
        let e4 = FElem::from(4);
        assert_eq!(e4, FElem::from(4));
    }

    #[test]
    fn test_FElem_FConst_assignment() {
        initPublicParamsFromDefaultPp();
        let mut e0 = FElem::from(0);
        let mut e1 = FElem::from(0);
        e1 = 42.into();
        e0 = e1.clone();
        assert_eq!(e1, FElem::from(42));
        assert_eq!(e0, e1);
    }

    #[test]
    fn test_FElem_FConst_asString() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(42);
        // #ifdef DEBUG
        assert_eq!(e0.asString(), "42");
        // #else
        //     assert_eq!(e0.asString(), "");
    }

    #[test]
    fn test_FElem_FConst_fieldType() {
        initPublicParamsFromDefaultPp();
        let mut e0 = FElem::from(42);
        assert_eq!(e0.fieldType(), FieldType::AGNOSTIC);
        e0 = FElem::froms(Fp::from(42));
        assert_ne!(e0.fieldType(), FieldType::AGNOSTIC);
    }

    #[test]
    fn test_FElem_FConst_operatorEquals() {
        initPublicParamsFromDefaultPp();
        let mut e0 = FElem::from(0i64);
        let mut e1 = FElem::from(1);
        let mut e2 = FElem::from(FElem::from(2));
        e1 = 42.into();
        e0 = e1.clone();
        //bool operator==(other:&FElem) const {return *elem_ == *other.elem_;}
        assert!(e1 == e0);
        assert!(!(e1 == e2));
        let eR1P: FElem = FElem::froms(Fp::from(42));
        assert!(e1 == eR1P);
        assert!(eR1P == e1);
        //bool operator==(first:FElem&, const long second);
        let e3 = FElem::from(4);
        assert!(e3 == 4);
        //bool operator==(first:long, second:&FElem);
        // assert!(4 == e3);
    }

    #[test]
    fn test_FElem_FConst_operatorPlus() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let mut e0 = FElem::from(0);
        let mut e1 = FElem::from(0);
        e1 = 42.into();
        e0 = e1.clone();
        e0 += &e1;
        e1 = e0.clone();
        assert_eq!(e0, FElem::from(84));
        assert!(e1 == 84);
    }

    #[test]
    fn test_FElem_FConst_operatorMinus() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let mut e0 = FElem::from(0);
        let mut e1 = FElem::from(0);
        e1 = 42.into();
        e0 = e1.clone();
        e0 -= &e1;
        e1 = e0.clone();
        assert!(e0 == 0);
        assert!(e1 == 0);
        e0 = 21.into();
        e1 = 2.into();
        assert_eq!(e0, FElem::from(21));
        assert!(e1 == 2);
    }

    #[test]
    fn test_FElem_FConst_operatorTimes() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let mut e0: FElem = 21.into();
        let mut e1: FElem = 2.into();
        e0 *= &e1;
        e1 = e0.clone();
        assert!(e0 == 42);
        assert!(e1 == 42);
        assert!(e0 == e1);
        assert!(e0 == 42);
        // assert!(42 == e0);
    }

    #[test]
    fn test_FElem_FConst_operatorUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let e4 = FElem::from(4);
        assert_eq!(-e4, FElem::from(-4));
    }

    #[test]
    fn test_FElem_FConst_operatorNotEquals() {
        initPublicParamsFromDefaultPp();
        let e0: FElem = 21.into();
        let e4 = FElem::from(FElem::from(4));
        //bool operator!=(first:FElem&, second:&FElem);
        assert!(e4 != e0);
        //bool operator!=(first:FElem&, const long second);
        assert!(e4 != 5);
        //bool operator!=(first:long, second:&FElem);
        // assert!(5 != e4);
    }

    #[test]
    fn test_FElem_FConst_inverse() {
        initPublicParamsFromDefaultPp();
        let e4: FElem = 4.into();
        let eInv: FElem = e4.inverses(&FieldType::R1P);
        assert_eq!(eInv, FElem::froms(Fp::from(e4.asLong()).inverse()));
    }

    #[test]
    fn test_FElem_R1P_Elem_constructor() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::froms(Fp::from(0));
        assert_eq!(e0, 0);
        assert_ne!(e0, 1);
    }

    #[test]
    fn test_FElem_R1P_Elem_copy() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::froms(Fp::from(0));
        let e1 = FElem::from(e0);
        assert_eq!(e1, 0);
    }

    #[test]
    fn test_FElem_R1P_Elem_assignment() {
        initPublicParamsFromDefaultPp();
        initPublicParamsFromDefaultPp();
        let mut e0 = FElem::froms(Fp::from(0));
        let mut e1 = FElem::from(e0);
        let mut e2: FElem = FElem::froms(Fp::from(2));
        e1 = e2.clone();
        assert_eq!(e1, 2);
        let mut e3: FElem = 3.into();
        e1 = e3.clone();
        assert_eq!(e1, 3);
    }

    #[test]
    fn test_FElem_R1P_Elem_move() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = 1.into();
        e1 = FElem::froms(Fp::from(2));
        assert_eq!(e1, 2);
        e1 = FElem::from(1);
        assert_eq!(e1, 1);
    }

    #[test]
    fn test_FElem_R1P_Elem_assignFromLong() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = FElem::from(1);
        e1 = 42i64.into();
        assert_eq!(e1, 42);
    }

    #[test]
    fn test_FElem_R1P_Elem_asString() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = 42i64.into();
        // #ifdef DEBUG
        assert_eq!(e1.asString(), "42");
        // #else
        //     assert_eq!(e1.asString(), "");
    }

    #[test]
    fn test_FElem_R1P_Elem_fieldType() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = FElem::froms(Fp::from(42));
        assert_eq!(e1.fieldType(), FieldType::R1P);
    }

    #[test]
    fn test_FElem_R1P_Elem_operatorEquals() {
        initPublicParamsFromDefaultPp();
        let e0: FElem = 42.into();
        let e1: FElem = 42i64.into();
        let e2: FElem = FElem::froms(Fp::from(2));
        assert!(e0 == e1);
        assert!(!(e0 == e2));
        assert!(!(e0 != e1));
        assert!(e0 == 42);
        assert!(!(e0 == 41));
        assert!(e0 != 41);
        // assert!(42 == e0);
        // assert!(41 != e0);
    }

    #[test]
    fn test_FElem_R1P_Elem_negativeNums() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = 42i64.into();
        let mut e2: FElem = FElem::froms(Fp::from(2));
        e1 = (-42).into();
        let e0: FElem = (-42).into();
        assert!(e0 == e1);
        assert!(!(e0 == e2));
        assert!(!(e0 != e1));
        assert!(e0 == -42);
        assert!(!(e0 == -41));
        assert!(e0 != -41);
        // assert!(-42 == e0);
        // assert!(-41 != e0);
    }

    #[test]
    fn test_FElem_R1P_Elem_operatorTimes() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = FElem::froms(Fp::from(1));
        let mut e2: FElem = FElem::froms(Fp::from(2));
        let mut e3: FElem = FElem::froms(Fp::from(3));
        assert!(e1.fieldType() == FieldType::R1P && e2.fieldType() == FieldType::R1P);
        e2 *= &e3;
        e1 = e2.clone();
        assert_eq!(e1, 6);
        assert_eq!(e2, 6);
        assert_eq!(e3, 3);
    }

    #[test]
    fn test_FElem_R1P_Elem_operatorPlus() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = FElem::froms(Fp::from(6));
        let mut e2: FElem = FElem::froms(Fp::from(6));
        let mut e3: FElem = FElem::froms(Fp::from(3));
        e2 += &e3;
        e1 = e2.clone();
        assert_eq!(e1, 9);
        assert_eq!(e2, 9);
        assert_eq!(e3, 3);
    }

    #[test]
    fn test_FElem_R1P_Elem_operatorMinus() {
        initPublicParamsFromDefaultPp();
        let mut e1: FElem = FElem::froms(Fp::from(9));
        let mut e2: FElem = FElem::froms(Fp::from(9));
        let mut e3: FElem = FElem::froms(Fp::from(3));
        e2 -= &e3;
        e1 = e2.clone();
        assert_eq!(e1, 6);
        assert_eq!(e2, 6);
        assert_eq!(e3, 3);
    }

    #[test]
    fn test_FElem_R1P_Elem_operatorUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let mut e2: FElem = FElem::froms(Fp::from(6));
        let mut e3: FElem = 3.into();
        e3 = -e2.clone();
        assert_eq!(e2, 6);
        assert_eq!(e3, -6);
        assert!(e3.fieldType() == FieldType::R1P);
    }

    #[test]
    fn test_FElem_R1P_Elem_inverse() {
        initPublicParamsFromDefaultPp();
        let e42: FElem = FElem::froms(Fp::from(42));
        assert_eq!(
            e42.inverses(&FieldType::R1P),
            FElem::froms(Fp::from(42).inverse())
        );
    }

    #[test]
    fn test_LinearTermConstructors() {
        initPublicParamsFromDefaultPp();
        //LinearTerm(v:&Variable)->Self variable_(v), coeff_(1) {}
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut lt0 = LinearTerm::from(x[0].clone());
        let mut ass = VariableAssignment::default();
        ass.insert(x[0].clone(), FElem::froms(Fp::from(42)));
        assert_eq!(lt0.eval(&ass), 42);
        assert_ne!(lt0.eval(&ass), 17);
        ass.insert(x[0].clone(), FElem::froms(Fp::from(2)));
        assert_eq!(lt0.eval(&ass), 2);
        let mut lt2 = LinearTerm::from(x[2].clone());
        ass.insert(x[2].clone(), 24.into());
        assert_eq!(lt2.eval(&ass), 24);
        //LinearTerm(v:Variable&, coeff:&FElem)->Self variable_(v), coeff_(coeff) {}
        let mut lt3 = LinearTerm::new(x[3].clone(), FElem::froms(Fp::from(3)));
        ass.insert(x[3].clone(), FElem::froms(Fp::from(4)));
        assert_eq!(lt3.eval(&ass), 3 * 4);
        //LinearTerm(v:Variable&, long n)->Self variable_(v), coeff_(n) {}
        let mut lt5 = LinearTerm::new(x[5].clone(), 2i64.into());
        ass.insert(x[5].clone(), 5.into());
        assert_eq!(lt5.eval(&ass), 5 * 2);
        let mut lt6 = LinearTerm::new(x[6].clone(), 2.into());
        ass.insert(x[6].clone(), 6.into());
        assert_eq!(lt6.eval(&ass), 6 * 2);
    }

    #[test]
    fn test_LinearTermUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut lt6 = LinearTerm::new(x[6].clone(), 2.into());
        let lt7: LinearTerm = -lt6;
        let mut ass = VariableAssignment::default();
        ass.insert(x[6].clone(), 6.into());
        assert_eq!(lt7.eval(&ass), -6 * 2);
    }

    #[test]
    fn test_LinearTermFieldType() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut lt3 = LinearTerm::new(x[3].clone(), FElem::froms(Fp::from(3)));
        let mut lt6 = LinearTerm::new(x[6].clone(), 2.into());
        let mut ass = VariableAssignment::default();
        ass.insert(x[3].clone(), FElem::froms(Fp::from(4)));
        ass.insert(x[6].clone(), 6.into());
        assert_eq!(lt6.fieldtype(), FieldType::AGNOSTIC);
        assert_eq!(lt3.fieldtype(), FieldType::R1P);
    }

    #[test]
    fn test_LinearTermAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut ass = VariableAssignment::default();
        // #ifdef DEBUG
        // FieldType::R1P
        let mut lt10 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(-1)));
        assert_eq!(lt10.asString(), "-1 * x[0]");
        let mut lt11 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(0)));
        assert_eq!(lt11.asString(), "0 * x[0]");
        let mut lt12 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(1)));
        assert_eq!(lt12.asString(), "x[0]");
        let mut lt13 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(2)));
        assert_eq!(lt13.asString(), "2 * x[0]");
        // FieldType::AGNOSTIC
        let mut lt30 = LinearTerm::new(x[0].clone(), (-1).into());
        assert_eq!(lt30.asString(), "-1 * x[0]");
        let mut lt31 = LinearTerm::new(x[0].clone(), 0.into());
        assert_eq!(lt31.asString(), "0 * x[0]");
        let mut lt32 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(1)));
        assert_eq!(lt32.asString(), "x[0]");
        let mut lt33 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(2)));
        assert_eq!(lt33.asString(), "2 * x[0]");
    }

    #[test]
    fn test_LinearTermOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut ass = VariableAssignment::default();
        ass.insert(x[0].clone(), FElem::froms(Fp::from(2)));
        let mut lt42 = LinearTerm::new(x[0].clone(), FElem::froms(Fp::from(1)));
        let mut lt43 = LinearTerm::new(x[0].clone().into(), FElem::froms(Fp::from(2)));
        lt43 *= &FElem::from(4);
        lt42 = lt43.clone();
        assert_eq!(lt42.eval(&ass), 8 * 2);
        assert_eq!(lt43.eval(&ass), 8 * 2);
    }

    // TODO refactor this test
    #[test]
    fn test_LinearCombination() {
        initPublicParamsFromDefaultPp();
        //    LinearCombination()->Self linearTerms_(), constant_(0) {}
        let lc0: LinearCombination = LinearCombination::default();
        let mut assignment = VariableAssignment::default();
        assert_eq!(lc0.eval(&assignment), 0);
        //    LinearCombination(var:&Variable)->Self linearTerms_(1,var), constant_(0) {}
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut lc1: LinearCombination = LinearCombination::from(x[1].clone());
        assignment.insert(x[1].clone(), 42.into());
        assert_eq!(lc1.eval(&assignment), 42);
        //    LinearCombination(linTerm:&LinearTerm)->Self linearTerms_(1,linTerm), constant_(0) {}
        let lt = LinearTerm::new(x[2].clone(), FElem::froms(Fp::from(2)));
        let mut lc2: LinearCombination = lt.clone().into();
        assignment.insert(x[2].clone(), 2.into());
        assert_eq!(lc2.eval(&assignment), 4);
        //    LinearCombination(long i)->Self linearTerms_(), constant_(i) {}
        let mut lc3: LinearCombination = 3.into();
        assert_eq!(lc3.eval(&assignment), 3);
        //    LinearCombination(elem:&FElem)->Self linearTerms_(), constant_(elem) {}
        let elem: FElem = FElem::froms(Fp::from(4));
        let mut lc4: LinearCombination = elem.clone().into();
        assert_eq!(lc4.eval(&assignment), 4);
        //    LinearCombination& operator+=(other:&LinearCombination);
        lc4 += &lc2;
        lc1 = lc4.clone();
        assert_eq!(lc4.eval(&assignment), 4 + 4);
        assert_eq!(lc1.eval(&assignment), 4 + 4);
        assert_eq!(lc2.eval(&assignment), 4);
        //    LinearCombination& operator-=(other:&LinearCombination);
        lc4 -= &lc3;
        lc1 = lc4.clone();
        assert_eq!(lc4.eval(&assignment), 4 + 4 - 3);
        assert_eq!(lc1.eval(&assignment), 4 + 4 - 3);
        assert_eq!(lc3.eval(&assignment), 3);
        //    ::String asString() const;
        // #   ifdef DEBUG
        assert_eq!(lc1.asString(), "2 * x[2] + 1");
        // #   else // ifdef DEBUG
        //     assert_eq!(lc1.asString(), "");
        // // #   endif // ifdef DEBUG
        //    Variable::set getUsedVariables() const;
        let sVar = lc1.getUsedVariables();
        assert_eq!(sVar.len(), 1usize);
        assignment.insert(x[2].clone(), 83.into());
        assert_eq!(assignment[&*sVar.first().unwrap()], FElem::from(83));
        assignment.insert(x[2].clone(), 2.into());
        //  LinearCombination operator-(lc:&LinearCombination);
        lc2 = -lc1.clone();
        assert_eq!(lc2.eval(&assignment), -5);
        lc1 *= &FElem::from(4);
        lc2 = lc1.clone();
        assert_eq!(lc1.eval(&assignment), 5 * 4);
        assert_eq!(lc2.eval(&assignment), 5 * 4);
    }

    #[test]
    fn test_MonomialConstructors() {
        initPublicParamsFromDefaultPp();
        //Monomial(var:&Variable)->Self coeff_(1), variables_(1, var) {}
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let m0: Monomial = x[0].clone().into();
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 42.into());
        assert_eq!(m0.eval(&assignment), 42);
        //Monomial(var:Variable&, coeff:&FElem)->Self coeff_(coeff), variables_(1, var) {}
        let m1 = Monomial::new(x[1].clone(), FElem::froms(Fp::from(3)));
        assignment.insert(x[1].clone(), 2.into());
        assert_eq!(m1.eval(&assignment), 6);
        //Monomial(linearTerm:&LinearTerm);
        let lt = LinearTerm::new(x[3].clone(), 3.into());
        let m3: Monomial = lt.into();
        assignment.insert(x[3].clone(), 3.into());
        assert_eq!(m3.eval(&assignment), 9);
    }

    #[test]
    fn test_MonomialUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = Variable::from("x");
        let m3: Monomial = (x.clone() * 3).into();
        let m4: Monomial = -m3;
        let mut assignment = VariableAssignment::default();
        assignment.insert(x, 3.into());
        assert_eq!(m4.eval(&assignment), -9);
    }

    #[test]
    fn test_MonomialOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let m0: Monomial = x[0].clone().into();
        let mut m4: Monomial = (x[3].clone() * -3).into();
        m4 *= &(m0.clone().into());
        let m3: Monomial = m4.clone();
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 42.into());
        assignment.insert(x[3].clone(), 3.into());
        assert_eq!(m3.eval(&assignment), -9 * 42);
        assert_eq!(m4.eval(&assignment), -9 * 42);
        assert_eq!(m0.eval(&assignment), 42);
    }

    #[test]
    fn test_MonomialUsedVariables() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let m0: Monomial = x[0].clone().into();
        let mut m4: Monomial = (x[3].clone() * (-3)).into();
        m4 *= &(m0.clone().into());
        let m3: Monomial = m4.clone();
        let varSet = m3.getUsedVariables();
        assert_eq!(varSet.len(), 2usize);
        assert!(varSet.contains_key(&x[0]));
        assert!(varSet.contains_key(&x[3]));
        assert!(!varSet.contains_key(&x[4]));
    }

    #[test]
    fn test_MonomialAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let m0: Monomial = x[0].clone().into();
        let mut m4: Monomial = (x[3].clone() * -3).into();
        m4 *= &(m0.clone().into());
        let m3: Monomial = m4.clone();
        // #   ifdef DEBUG
        assert_eq!(m3.asString(), "-3*x[0]*x[3]");
        // #   else
        //         assert_eq!(m3.asString(), "");
        // #   endif
    }

    #[test]
    fn test_PolynomialConstructors() {
        initPublicParamsFromDefaultPp();
        //Polynomial();
        let mut p0 = Polynomial::default();
        let mut assignment = VariableAssignment::default();
        assert_eq!(p0.eval(&assignment), 0);
        //Polynomial(monomial:&Monomial);
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let m0 = Monomial::new(x[0].clone(), 3.into());
        let p1: Polynomial = m0.clone().into();
        assignment.insert(x[0].clone(), 2.into());
        assert_eq!(p1.eval(&assignment), 6);
        //Polynomial(var:&Variable);
        let p2: Polynomial = x[2].clone().into();
        assignment.insert(x[2].clone(), 2.into());
        assert_eq!(p2.eval(&assignment), 2);
        //Polynomial(val:&FElem);
        let p3: Polynomial = FElem::froms(Fp::from(3)).into();
        assert_eq!(p3.eval(&assignment), 3);
        //Polynomial(linearCombination:&LinearCombination);
        let mut lc = LinearCombination::from(x[0].clone());
        lc += &(x[2].clone().into());
        let p4: Polynomial = lc.clone().into();
        assert_eq!(p4.eval(&assignment), 4);
        //Polynomial(linearTerm:&LinearTerm);
        let lt5: Polynomial = (x[5].clone() * 5).into();
        let p5: Polynomial = lt5.clone();
        assignment.insert(x[5].clone(), 5.into());
        assert_eq!(p5.eval(&assignment), 25);
    }

    #[test]
    fn test_PolynomialUsedVariables() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let p4: Polynomial = (x[0].clone() + &x[2]).into();
        let varSet = p4.getUsedVariables();
        assert_eq!(varSet.len(), 2usize);
        assert!(varSet.contains_key(&x[0]));
        assert!(varSet.contains_key(&x[2]));
    }

    #[test]
    fn test_PolynomialOperatorPlus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let p3: Polynomial = FElem::froms(Fp::from(3)).into();
        let mut p4: Polynomial = (x[0].clone() + &x[2]).into();
        p4 += &p3;
        let p5: Polynomial = p4.clone();
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 2.into());
        assignment.insert(x[2].clone(), 2.into());
        assert_eq!(p5.eval(&assignment), 7);
    }

    #[test]
    fn test_PolynomialAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut p0 = Polynomial::default();
        let p1: Polynomial = (x[0].clone() * 3).into();
        let p2: Polynomial = x[2].clone().into();
        let p3: Polynomial = FElem::froms(Fp::from(3)).into();
        let mut p4: Polynomial = (x[0].clone() + &x[2]).into();
        p4 += &p3;
        let p5: Polynomial = p4.clone();
        // #   ifdef DEBUG
        assert_eq!(p0.asString(), "0");
        assert_eq!(p1.asString(), "3*x[0]");
        assert_eq!(p2.asString(), "x[2]");
        assert_eq!(p3.asString(), "3");
        assert_eq!(p4.asString(), "x[0] + x[2] + 3");
        assert_eq!(p5.asString(), "x[0] + x[2] + 3");
        // #   else // DEBUG
        //         assert_eq!(p0.asString(), "");
        //         assert_eq!(p1.asString(), "");
        //         assert_eq!(p2.asString(), "");
        //         assert_eq!(p3.asString(), "");
        //         assert_eq!(p4.asString(), "");
        //         assert_eq!(p5.asString(), "");
        // #   endif // DEBUG
    }

    #[test]
    fn test_PolynomialOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 2.into());
        assignment.insert(x[2].clone(), 2.into());
        let mut p4: Polynomial = (x[0].clone() + &x[2]).into();
        p4 += &Polynomial::from(3);
        let p5: Polynomial = p4.clone();
        p4 *= &p5;
        let p0: Polynomial = p4.clone();
        assert_eq!(p0.eval(&assignment), 7 * 7);
        assert_eq!(p4.eval(&assignment), 7 * 7);
        assert_eq!(p5.eval(&assignment), 7);
    }

    #[test]
    fn test_PolynomialOperatorMinus() {
        initPublicParamsFromDefaultPp();
        let mut x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut p0: Polynomial = x[0].clone().into();
        let mut p1: Polynomial = x[1].clone().into();
        let mut p2: Polynomial = (x[2].clone() * 2).into();
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 0.into());
        assignment.insert(x[1].clone(), 1.into());
        assignment.insert(x[2].clone(), 2.into());
        p1 -= &p2; // = x[1] - 2 * x[2] = 1 - 2 * 2
        p0 = p1.clone();
        assert_eq!(p0.eval(&assignment), 1 - 2 * 2);
        assert_eq!(p1.eval(&assignment), 1 - 2 * 2);
        assert_eq!(p2.eval(&assignment), 2 * 2);
    }

    #[test]
    fn test_PolynomialUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut p0: Polynomial = x[0].clone().into();
        let mut p1: Polynomial = x[1].clone().into();
        let mut assignment = VariableAssignment::default();
        assignment.insert(x[0].clone(), 0.into());
        assignment.insert(x[1].clone(), 1.into());
        p0 = (-p1.clone()).into();
        assert_eq!(p0.eval(&assignment), -1);
        assert_eq!(p1.eval(&assignment), 1);
    }
}
