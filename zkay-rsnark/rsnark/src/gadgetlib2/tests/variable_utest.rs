//  Unit tests for gadgetlib2 variables

// use crate::gadgetlib2::pp;
// use crate::gadgetlib2::variable;

#[cfg(test)]
mod tests {
    use super::*;

    fn orderFunc(a: &Variable, b: &Variable) -> bool {
        a.index_ < b.index_
    }

    #[test]
    fn VariableNaming() {
        let mut v1 = Variable::default();
        assert_eq!(v1.name(), "");
        let v2 = Variable::from("foo");
        // #   ifdef DEBUG
        assert_eq!(v2.name(), "foo");
        // #   endif
        v2 = v1;
        assert_eq!(v2.name(), "");
    }

    #[test]
    fn VariableStrictOrdering() {
        let mut v1 = Variable::default();
        let mut v2 = Variable::default();
        // Variable::VariableStrictOrder orderFunc;
        assert!(orderFunc(v1, v2) || orderFunc(v2, v1)); // check strict ordering
        v2 = v1;
        assert!(!orderFunc(v1, v2) || orderFunc(v2, v1));
    }

    #[test]
    fn VariableSet() {
        let mut v1 = Variable::default();
        let mut s1 = Variable::set::default();
        s1.insert(v1);
        assert_eq!(s1.len(), 1u);
        let mut v2 = Variable::default();
        v2 = v1;
        s1.insert(v2);
        assert_eq!(s1.len(), 1u);
        let mut v3 = Variable::default();
        s1.insert(v3);
        assert_eq!(s1.len(), 2u);
        let mut v4 = Variable::default();
        s1.remove(v4);
        assert_eq!(s1.len(), 2u);
        v4 = v1;
        s1.remove(v4);
        assert_eq!(s1.len(), 1u);
    }

    #[test]
    fn VariableArray() {
        let v1 = Variable::default();
        let v2 = Variable::from("v2");
        let mut vArr = VariableArray::default();
        vArr.push(v1);
        vArr.push(v2);
        assert_eq!(vArr.len(), 2u);
        assert!(orderFunc(&vArr[0], &vArr[1]) || orderFunc(&vArr[1], &vArr[0])); // check strict ordering
        vArr[1] = vArr[0];
        assert!(!orderFunc(&vArr[0], &vArr[1]) || orderFunc(&vArr[1], &vArr[0])); // check strict ordering
        assert!(vArr.get(2).is_none()); // = v1, ::std::out_of_range);
    }

    #[test]
    fn VariableEval() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut ass = VariableAssignment::default();
        ass[x[0]] = Fp::from(42);
        assert_eq!(x[0].eval(ass), 42);
        assert_ne!(x[0].eval(ass), 17);
    }

    #[test]
    fn FElem_FConst_fromLong() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(0i64);
        assert!(e0 == 0);
    }

    #[test]
    fn FElem_FConst_fromInt() {
        initPublicParamsFromDefaultPp();
        let e1 = FElem::from(1);
        assert!(e1 == 1);
        let e2 = FElem::from(2);
        assert_eq!(e2, FElem(2));
    }

    #[test]
    fn FElem_FConst_copy() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(0i64);
        let e1 = FElem::from(1);
        let e3 = FElem::from(e1);
        assert!(e3 == 1);
        e3 = 0;
        assert!(e3 == 0);
        assert_eq!(e1, 1);
        e0 = e1;
        assert_eq!(e0, e1);
        e0 = 0;
        assert_ne!(e0, e1);
    }

    #[test]
    fn FElem_FConst_move() {
        initPublicParamsFromDefaultPp();
        let e4 = FElem::from(FElem(4));
        assert_eq!(e4, FElem(4));
    }

    #[test]
    fn FElem_FConst_assignment() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(0);
        let e1 = FElem::from(0);
        e0 = e1 = 42;
        assert_eq!(e1, FElem(42));
        assert_eq!(e0, e1);
    }

    #[test]
    fn FElem_FConst_asString() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(42);
        // #ifdef DEBUG
        assert_eq!(e0.asString(), "42");
        // #else
        //     assert_eq!(e0.asString(), "");
        //#endif
    }

    #[test]
    fn FElem_FConst_fieldType() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(42);
        assert_eq!(e0.fieldType(), AGNOSTIC);
        e0 = Fp::from(42);
        assert_ne!(e0.fieldType(), AGNOSTIC);
    }

    #[test]
    fn FElem_FConst_operatorEquals() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(0i64);
        let e1 = FElem::from(1);
        let e2 = FElem::from(FElem(2));
        e0 = e1 = 42;
        //bool operator==(other:&FElem) const {return *elem_ == *other.elem_;}
        assert!(e1 == e0);
        assert!(!e1 == e2);
        let eR1P: FElem = Fp::from(42);
        assert!(e1 == eR1P);
        assert!(eR1P == e1);
        //bool operator==(first:FElem&, const long second);
        let e3 = FElem::from(FElem(4));
        assert!(e3 == 4);
        //bool operator==(first:long, second:&FElem);
        assert!(4 == e3);
    }

    #[test]
    fn FElem_FConst_operatorPlus() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let e0 = FElem::from(0);
        let e1 = FElem::from(0);
        e0 = e1 = 42;
        e1 = e0 += e1;
        assert_eq!(e0, FElem(84));
        assert!(e1 == 84);
    }

    #[test]
    fn FElem_FConst_operatorMinus() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let e0 = FElem::from(0);
        let e1 = FElem::from(0);
        e0 = e1 = 42;
        e1 = e0 -= e1;
        assert!(e0 == 0);
        assert!(e1 == 0);
        e0 = 21;
        e1 = 2;
        assert_eq!(e0, FElem(21));
        assert!(e1 == 2);
    }

    #[test]
    fn FElem_FConst_operatorTimes() {
        initPublicParamsFromDefaultPp();
        //FElem& operator+=(other:&FElem) {*elem_ += *other.elem_; return *this;}
        let e0: FElem = 21;
        let e1: FElem = 2;
        e1 = e0 *= e1;
        assert!(e0 == 42);
        assert!(e1 == 42);
        assert!(e0 == e1);
        assert!(e0 == 42);
        assert!(42 == e0);
    }

    #[test]
    fn FElem_FConst_operatorUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let e4 = FElem::from(FElem(4));
        assert_eq!(-e4, FElem(-4));
    }

    #[test]
    fn FElem_FConst_operatorNotEquals() {
        initPublicParamsFromDefaultPp();
        let e0: FElem = 21;
        let e4 = FElem::from(FElem(4));
        //bool operator!=(first:FElem&, second:&FElem);
        assert!(e4 != e0);
        //bool operator!=(first:FElem&, const long second);
        assert!(e4 != 5);
        //bool operator!=(first:long, second:&FElem);
        assert!(5 != e4);
    }

    #[test]
    fn FElem_FConst_inverse() {
        initPublicParamsFromDefaultPp();
        let e4: FElem = 4;
        let eInv: FElem = e4.inverse(R1P);
        assert_eq!(eInv, FElem(Fp::from(e4.asLong()).inverse()));
    }

    #[test]
    fn FElem_R1P_Elem_constructor() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(Fp::from(0));
        assert_eq!(e0, 0);
        assert_ne!(e0, 1);
    }

    #[test]
    fn FElem_R1P_Elem_copy() {
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(Fp::from(0));
        let e1 = FElem::from(e0);
        assert_eq!(e1, 0);
    }

    #[test]
    fn FElem_R1P_Elem_assignment() {
        initPublicParamsFromDefaultPp();
        initPublicParamsFromDefaultPp();
        let e0 = FElem::from(Fp::from(0));
        let e1 = FElem::from(e0);
        let e2: FElem = Fp::from(2);
        e1 = e2;
        assert_eq!(e1, 2);
        let e3: FElem = 3;
        e1 = e3;
        assert_eq!(e1, 3);
    }

    #[test]
    fn FElem_R1P_Elem_move() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = 1;
        e1 = FElem(Fp::from(2));
        assert_eq!(e1, 2);
        e1 = FElem(1);
        assert_eq!(e1, 1);
    }

    #[test]
    fn FElem_R1P_Elem_assignFromLong() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = FElem(1);
        e1 = 42i64;
        assert_eq!(e1, 42);
    }

    #[test]
    fn FElem_R1P_Elem_asString() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = 42i64;
        // #ifdef DEBUG
        assert_eq!(e1.asString(), "42");
        // #else
        //     assert_eq!(e1.asString(), "");
        //#endif
    }

    #[test]
    fn FElem_R1P_Elem_fieldType() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = Fp::from(42);
        assert_eq!(e1.fieldType(), R1P);
    }

    #[test]
    fn FElem_R1P_Elem_operatorEquals() {
        initPublicParamsFromDefaultPp();
        let e0: FElem = 42;
        let e1: FElem = 42i64;
        let e2: FElem = Fp::from(2);
        assert!(e0 == e1);
        assert!(!e0 == e2);
        assert!(!e0 != e1);
        assert!(e0 == 42);
        assert!(!e0 == 41);
        assert!(e0 != 41);
        assert!(42 == e0);
        assert!(41 != e0);
    }

    #[test]
    fn FElem_R1P_Elem_negativeNums() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = 42i64;
        let e2: FElem = Fp::from(2);
        let e0: FElem = e1 = -42;
        assert!(e0 == e1);
        assert!(!e0 == e2);
        assert!(!e0 != e1);
        assert!(e0 == -42);
        assert!(!e0 == -41);
        assert!(e0 != -41);
        assert!(-42 == e0);
        assert!(-41 != e0);
    }

    #[test]
    fn FElem_R1P_Elem_operatorTimes() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = Fp::from(1);
        let e2: FElem = Fp::from(2);
        let e3: FElem = Fp::from(3);
        assert!(e1.fieldType() == R1P && e2.fieldType() == R1P);
        e1 = e2 *= e3;
        assert_eq!(e1, 6);
        assert_eq!(e2, 6);
        assert_eq!(e3, 3);
    }

    #[test]
    fn FElem_R1P_Elem_operatorPlus() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = Fp::from(6);
        let e2: FElem = Fp::from(6);
        let e3: FElem = Fp::from(3);
        e1 = e2 += e3;
        assert_eq!(e1, 9);
        assert_eq!(e2, 9);
        assert_eq!(e3, 3);
    }

    #[test]
    fn FElem_R1P_Elem_operatorMinus() {
        initPublicParamsFromDefaultPp();
        let e1: FElem = Fp::from(9);
        let e2: FElem = Fp::from(9);
        let e3: FElem = Fp::from(3);
        e1 = e2 -= e3;
        assert_eq!(e1, 6);
        assert_eq!(e2, 6);
        assert_eq!(e3, 3);
    }

    #[test]
    fn FElem_R1P_Elem_operatorUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let e2: FElem = Fp::from(6);
        let e3: FElem = 3;
        e3 = -e2;
        assert_eq!(e2, 6);
        assert_eq!(e3, -6);
        assert!(e3.fieldType() == R1P);
    }

    #[test]
    fn FElem_R1P_Elem_inverse() {
        initPublicParamsFromDefaultPp();
        let e42: FElem = Fp::from(42);
        assert_eq!(e42.inverse(R1P), Fp::from(42).inverse());
    }

    #[test]
    fn LinearTermConstructors() {
        initPublicParamsFromDefaultPp();
        //LinearTerm(v:&Variable)->Self variable_(v), coeff_(1) {}
        let x = VariableArray::new(10, "x");
        let mut lt0 = LinearTerm::new([0]);
        let mut ass = VariableAssignment::default();
        ass[x[0]] = Fp::from(42);
        assert_eq!(lt0.eval(ass), 42);
        assert_ne!(lt0.eval(ass), 17);
        ass[x[0]] = Fp::from(2);
        assert_eq!(lt0.eval(ass), 2);
        let mut lt2 = LinearTerm::new([2]);
        ass[x[2]] = 24;
        assert_eq!(lt2.eval(ass), 24);
        //LinearTerm(v:Variable&, coeff:&FElem)->Self variable_(v), coeff_(coeff) {}
        let mut lt3 = LinearTerm::new([3], Fp::from(3));
        ass[x[3]] = Fp::from(4);
        assert_eq!(lt3.eval(ass), 3 * 4);
        //LinearTerm(v:Variable&, long n)->Self variable_(v), coeff_(n) {}
        let mut lt5 = LinearTerm::new([5], 2i64);
        ass[x[5]] = 5;
        assert_eq!(lt5.eval(ass), 5 * 2);
        let mut lt6 = LinearTerm::new([6], 2);
        ass[x[6]] = 6;
        assert_eq!(lt6.eval(ass), 6 * 2);
    }

    #[test]
    fn LinearTermUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut lt6 = LinearTerm::new([6], 2);
        let lt7: LinearTerm = -lt6;
        let mut ass = VariableAssignment::default();
        ass[x[6]] = 6;
        assert_eq!(lt7.eval(ass), -6 * 2);
    }

    #[test]
    fn LinearTermFieldType() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut lt3 = LinearTerm::new([3], Fp::from(3));
        let mut lt6 = LinearTerm::new([6], 2);
        let mut ass = VariableAssignment::default();
        ass[x[3]] = Fp::from(4);
        ass[x[6]] = 6;
        assert_eq!(lt6.fieldtype(), AGNOSTIC);
        assert_eq!(lt3.fieldtype(), R1P);
    }

    #[test]
    fn LinearTermAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut ass = VariableAssignment::default();
        // #ifdef DEBUG
        // R1P
        let mut lt10 = LinearTerm::new([0], Fp::from(-1));
        assert_eq!(lt10.asString(), "-1 * x[0]");
        let mut lt11 = LinearTerm::new([0], Fp::from(0));
        assert_eq!(lt11.asString(), "0 * x[0]");
        let mut lt12 = LinearTerm::new([0], Fp::from(1));
        assert_eq!(lt12.asString(), "x[0]");
        let mut lt13 = LinearTerm::new([0], Fp::from(2));
        assert_eq!(lt13.asString(), "2 * x[0]");
        // AGNOSTIC
        let mut lt30 = LinearTerm::new([0], -1);
        assert_eq!(lt30.asString(), "-1 * x[0]");
        let mut lt31 = LinearTerm::new([0], 0);
        assert_eq!(lt31.asString(), "0 * x[0]");
        let mut lt32 = LinearTerm::new([0], Fp::from(1));
        assert_eq!(lt32.asString(), "x[0]");
        let mut lt33 = LinearTerm::new([0], Fp::from(2));
        assert_eq!(lt33.asString(), "2 * x[0]");
        //#endif // DEBUG
    }

    #[test]
    fn LinearTermOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut ass = VariableAssignment::default();
        ass[x[0]] = Fp::from(2);
        let mut lt42 = LinearTerm::new([0], Fp::from(1));
        let mut lt43 = LinearTerm::new([0], Fp::from(2));
        lt43 *= FElem(4);
        lt42 = lt43;
        assert_eq!(lt42.eval(ass), 8 * 2);
        assert_eq!(lt43.eval(ass), 8 * 2);
    }

    // TODO refactor this test
    #[test]
    fn LinearCombination() {
        initPublicParamsFromDefaultPp();
        //    LinearCombination()->Self linearTerms_(), constant_(0) {}
        let lc0 = LinearCombination::default();
        let mut assignment = VariableAssignment::default();
        assert_eq!(lc0.eval(assignment), 0);
        //    LinearCombination(var:&Variable)->Self linearTerms_(1,var), constant_(0) {}
        let x = VariableArray::new(10, "x");
        let lc1 = LinearCombination::from(x[1]);
        assignment[x[1]] = 42;
        assert_eq!(lc1.eval(assignment), 42);
        //    LinearCombination(linTerm:&LinearTerm)->Self linearTerms_(1,linTerm), constant_(0) {}
        let lt = LinearTerm::new(x[2], Fp::from(2));
        let lc2: LinearCombination = lt;
        assignment[x[2]] = 2;
        assert_eq!(lc2.eval(assignment), 4);
        //    LinearCombination(long i)->Self linearTerms_(), constant_(i) {}
        let lc3: LinearCombination = 3;
        assert_eq!(lc3.eval(assignment), 3);
        //    LinearCombination(elem:&FElem)->Self linearTerms_(), constant_(elem) {}
        let elem: FElem = Fp::from(4);
        let lc4 = elem;
        assert_eq!(lc4.eval(assignment), 4);
        //    LinearCombination& operator+=(other:&LinearCombination);
        lc1 = lc4 += lc2;
        assert_eq!(lc4.eval(assignment), 4 + 4);
        assert_eq!(lc1.eval(assignment), 4 + 4);
        assert_eq!(lc2.eval(assignment), 4);
        //    LinearCombination& operator-=(other:&LinearCombination);
        lc1 = lc4 -= lc3;
        assert_eq!(lc4.eval(assignment), 4 + 4 - 3);
        assert_eq!(lc1.eval(assignment), 4 + 4 - 3);
        assert_eq!(lc3.eval(assignment), 3);
        //    ::String asString() const;
        // #   ifdef DEBUG
        assert_eq!(lc1.asString(), "2 * x[2] + 1");
        // #   else // ifdef DEBUG
        //     assert_eq!(lc1.asString(), "");
        // // #   endif // ifdef DEBUG
        //    Variable::set getUsedVariables() const;
        let sVar = lc1.getUsedVariables();
        assert_eq!(sVar.len(), 1u);
        assignment[x[2]] = 83;
        assert_eq!(assignment[*sVar.first().unwrap()], 83);
        assignment[x[2]] = 2;
        //  LinearCombination operator-(lc:&LinearCombination);
        lc2 = -lc1;
        assert_eq!(lc2.eval(assignment), -5);
        lc2 = lc1 *= FElem(4);
        assert_eq!(lc1.eval(assignment), 5 * 4);
        assert_eq!(lc2.eval(assignment), 5 * 4);
    }

    #[test]
    fn MonomialConstructors() {
        initPublicParamsFromDefaultPp();
        //Monomial(var:&Variable)->Self coeff_(1), variables_(1, var) {}
        let x = VariableArray::new(10, "x");
        let m0 = x[0];
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 42;
        assert_eq!(m0.eval(assignment), 42);
        //Monomial(var:Variable&, coeff:&FElem)->Self coeff_(coeff), variables_(1, var) {}
        let m1 = Monomial::new(x[1], Fp::from(3));
        assignment[x[1]] = 2;
        assert_eq!(m1.eval(assignment), 6);
        //Monomial(linearTerm:&LinearTerm);
        let lt = LinearTerm::new(x[3], 3);
        let m3 = lt;
        assignment[x[3]] = 3;
        assert_eq!(m3.eval(assignment), 9);
    }

    #[test]
    fn MonomialUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = Variable::from("x");
        let m3 = 3 * x;
        let m4 = -m3;
        let mut assignment = VariableAssignment::default();
        assignment[x] = 3;
        assert_eq!(m4.eval(assignment), -9);
    }

    #[test]
    fn MonomialOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let m0 = x[0];
        let mut m4 = -3 * x[3];
        m4 *= m0;
        let m3 = m4;
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 42;
        assignment[x[3]] = 3;
        assert_eq!(m3.eval(assignment), -9 * 42);
        assert_eq!(m4.eval(assignment), -9 * 42);
        assert_eq!(m0.eval(assignment), 42);
    }

    #[test]
    fn MonomialUsedVariables() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let m0 = x[0];
        let mut m4 = -3 * x[3];
        m4 *= m0;
        let m3 = m4;
        let varSet = m3.getUsedVariables();
        assert_eq!(varSet.len(), 2u);
        assert!(varSet.contains(&x[0]));
        assert!(varSet.contains(&x[3]));
        assert!(!varSet.contains(&x[4]));
    }

    #[test]
    fn MonomialAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let m0 = x[0];
        let mut m4 = x[3] * -3;
        m4 *= m0;
        let m3 = m4;
        // #   ifdef DEBUG
        assert_eq!(m3.asString(), "-3*x[0]*x[3]");
        // #   else
        //         assert_eq!(m3.asString(), "");
        // #   endif
    }

    #[test]
    fn PolynomialConstructors() {
        initPublicParamsFromDefaultPp();
        //Polynomial();
        let p0;
        let mut assignment = VariableAssignment::default();
        assert_eq!(p0.eval(assignment), 0);
        //Polynomial(monomial:&Monomial);
        let x = VariableArray::new(10, "x");
        let m0 = Monomial::new(x[0], 3);
        let p1 = m0;
        assignment[x[0]] = 2;
        assert_eq!(p1.eval(assignment), 6);
        //Polynomial(var:&Variable);
        let p2 = x[2];
        assignment[x[2]] = 2;
        assert_eq!(p2.eval(assignment), 2);
        //Polynomial(val:&FElem);
        let p3 = FElem(Fp::from(3));
        assert_eq!(p3.eval(assignment), 3);
        //Polynomial(linearCombination:&LinearCombination);
        let mut lc = LinearCombination::from(x[0]);
        lc += x[2];
        let p4 = lc;
        assert_eq!(p4.eval(assignment), 4);
        //Polynomial(linearTerm:&LinearTerm);
        let lt5 = 5 * x[5];
        let p5 = lt5;
        assignment[x[5]] = 5;
        assert_eq!(p5.eval(assignment), 25);
    }

    #[test]
    fn PolynomialUsedVariables() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let p4 = x[0] + x[2];
        let varSet = p4.getUsedVariables();
        assert_eq!(varSet.len(), 2u);
        assert!(varSet.find(x[0]) != varSet.end());
        assert!(varSet.find(x[2]) != varSet.end());
    }

    #[test]
    fn PolynomialOperatorPlus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let p3 = FElem(Fp::from(3));
        let p4 = x[0] + x[2];
        let p5 = p4 += p3;
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 2;
        assignment[x[2]] = 2;
        assert_eq!(p5.eval(assignment), 7);
    }

    #[test]
    fn PolynomialAsString() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let p0;
        let p1 = 3 * x[0];
        let p2 = x[2];
        let p3 = FElem(Fp::from(3));
        let p4 = x[0] + x[2];
        let p5 = p4 += p3;
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
    fn PolynomialOperatorTimes() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 2;
        assignment[x[2]] = 2;
        let p4 = x[0] + x[2];
        let p5 = p4 += 3;
        let p0 = p4 *= p5;
        assert_eq!(p0.eval(assignment), 7 * 7);
        assert_eq!(p4.eval(assignment), 7 * 7);
        assert_eq!(p5.eval(assignment), 7);
    }

    #[test]
    fn PolynomialOperatorMinus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let p0 = x[0];
        let p1 = x[1];
        let p2 = 2 * x[2];
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 0;
        assignment[x[1]] = 1;
        assignment[x[2]] = 2;
        p0 = p1 -= p2; // = x[1] - 2 * x[2] = 1 - 2 * 2
        assert_eq!(p0.eval(assignment), 1 - 2 * 2);
        assert_eq!(p1.eval(assignment), 1 - 2 * 2);
        assert_eq!(p2.eval(assignment), 2 * 2);
    }

    #[test]
    fn PolynomialUnaryMinus() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let p0 = x[0];
        let p1 = x[1];
        let mut assignment = VariableAssignment::default();
        assignment[x[0]] = 0;
        assignment[x[1]] = 1;
        p0 = -p1;
        assert_eq!(p0.eval(assignment), -1);
        assert_eq!(p1.eval(assignment), 1);
    }
}
