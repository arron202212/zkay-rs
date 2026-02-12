//  Unit tests for gadgetlib2 - test rank

use crate::gadgetlib2::constraint::{ConstraintConfig, PrintOptions, Rank1Constraint};
use crate::gadgetlib2::gadget::{
    ConditionalFlag_Gadget, EqualsConst_Gadget, LogicImplication_Gadget,
};
use crate::gadgetlib2::pp::{Fp, initPublicParamsFromDefaultPp};

use crate::gadgetlib2::variable::{
    FElem, LinearCombination, VariableArray, VariableArrayBase, VariableAssignment,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Rank1Constraint() {
        initPublicParamsFromDefaultPp();
        let x = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut assignment = VariableAssignment::default();
        for i in 0..10 {
            assignment.insert(x[i].clone(), FElem::froms(Fp::from(i)));
        }
        let a: LinearCombination = x[0].clone() + &x[1] + 2; // <a,assignment> = 0+1+2=3
        let b: LinearCombination = x[2].clone() * 2 - &(x[3].clone() * 3) + 4; // <b,assignment> = 2*2-3*3+4=-1
        let c: LinearCombination = x[5].clone().into(); // <c,assignment> = 5
        let c1 = Rank1Constraint::new(a.clone(), b.clone(), c.clone(), "c1".to_owned());
        assert_eq!(c1.t.a().eval(&assignment), a.eval(&assignment));
        assert_eq!(c1.t.b().eval(&assignment), b.eval(&assignment));
        assert_eq!(c1.t.c().eval(&assignment), c.eval(&assignment));
        assert!(!c1.isSatisfied(&assignment, &PrintOptions::NO_DBG_PRINT));
        assert!(!c1.isSatisfied(&assignment, &PrintOptions::NO_DBG_PRINT));
        assignment.insert(x[5].clone(), (-3).into());
        assert!(c1.isSatisfied(&assignment, &PrintOptions::NO_DBG_PRINT));
        assert!(c1.isSatisfied(&assignment, &PrintOptions::NO_DBG_PRINT));
        let varSet = c1.getUsedVariables();
        assert_eq!(varSet.len(), 5usize);
        assert!(varSet.contains(&x[0]));
        assert!(varSet.contains(&x[1]));
        assert!(varSet.contains(&x[2]));
        assert!(varSet.contains(&x[3]));
        assert!(!varSet.contains(&x[4]));
        assert!(varSet.contains(&x[5]));
    }
}
