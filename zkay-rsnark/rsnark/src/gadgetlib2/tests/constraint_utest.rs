//  Unit tests for gadgetlib2 - test rank

// use  "depends/gtest/googletest/include/gtest/gtest.h"

// use crate::gadgetlib2::constraint;
// use crate::gadgetlib2::pp;

// using ::BTreeSet;
// using namespace gadgetlib2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Rank1Constraint() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x");
        let mut assignment = VariableAssignment::default();
        for i in 0..10 {
            assignment[x[i]] = Fp(i);
        }
        let a: LinearCombination = x[0] + x[1] + 2; // <a,assignment> = 0+1+2=3
        let b: LinearCombination = 2 * x[2] - 3 * x[3] + 4; // <b,assignment> = 2*2-3*3+4=-1
        let c: LinearCombination = x[5]; // <c,assignment> = 5
        let c1 = Rank1Constraint::new(a, b, c, "c1");
        assert_eq!(c1.a().eval(assignment), a.eval(assignment));
        assert_eq!(c1.b().eval(assignment), b.eval(assignment));
        assert_eq!(c1.c().eval(assignment), c.eval(assignment));
        assert!(!c1.isSatisfied(assignment));
        assert!(!c1.isSatisfied(assignment, PrintOptions::NO_DBG_PRINT));
        assignment[x[5]] = -3;
        assert!(c1.isSatisfied(assignment));
        assert!(c1.isSatisfied(assignment, PrintOptions::NO_DBG_PRINT));
        let varSet = c1.getUsedVariables();
        assert_eq!(varSet.len(), 5u);
        assert!(varSet.find(x[0]) != varSet.end());
        assert!(varSet.find(x[1]) != varSet.end());
        assert!(varSet.find(x[2]) != varSet.end());
        assert!(varSet.find(x[3]) != varSet.end());
        assert!(varSet.find(x[4]) == varSet.end());
        assert!(varSet.find(x[5]) != varSet.end());
    }
}
