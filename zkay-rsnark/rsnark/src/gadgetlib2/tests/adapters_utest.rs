//  Unit tests for gadgetlib2

use crate::gadgetlib2::adapters::{ConvertConfig, GLA, GadgetLibAdapter, variable_index_t};
use crate::gadgetlib2::constraint::{ConstraintSystem, Rank1Constraint};
use crate::gadgetlib2::gadget::{
    ConditionalFlag_Gadget, EqualsConst_Gadget, LogicImplication_Gadget,
};
use crate::gadgetlib2::pp::{Fp, initPublicParamsFromDefaultPp};
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{
    FieldType, LinearCombination, Variable, VariableArray, VariableArrayBase, VariableArrayConfig,
    VariableAssignment,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_LinearTerm() {
        initPublicParamsFromDefaultPp();
        // let adapter = GLA; //GadgetLibAdapter
        GLA::resetVariableIndex();
        let x = Variable::from("x");
        let lt = x * 5;
        let new_lt = GLA::convert(&lt);
        assert_eq!(new_lt.0, 0u64);
        assert_eq!(new_lt.1, Fp::from(5));
    }

    #[test]
    fn test_LinearCombination() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA; //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let lc = x.clone() * 5 + &(y.clone() * 3) + 42;
        let new_lc = GLA::convert(&lc);
        assert_eq!(new_lc.1, Fp::from(42));
        assert_eq!(new_lc.0.len(), 2usize);
        assert_eq!(new_lc.0[0], GLA::convert(&(x * 5)));
        assert_eq!(new_lc.0[1], GLA::convert(&(y * 3)));
    }

    #[test]
    fn test_Constraint() {
        // using ::std::get;
        initPublicParamsFromDefaultPp();
        let adapter = GLA; //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let constraint = Rank1Constraint::new(
            x.clone() + &y,
            (x.clone() * 5).into(),
            0.into(),
            "(x + y) * (5 * x) == 0".to_owned(),
        );
        let new_constraint = GLA::convert(&constraint);
        assert_eq!(new_constraint.0, GLA::convert(&(x.clone() + &y)));
        assert_eq!(new_constraint.1, GLA::convert(&(x.clone() * 5 + 0)));
        assert_eq!(new_constraint.2, GLA::convert(&LinearCombination::from(0)));
    }

    #[test]
    fn test_constraint_system() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA; //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let constraint0 = Rank1Constraint::new(
            x.clone() + &y,
            (x.clone() * 5).into(),
            0.into(),
            "(x + y) * (5*x) == 0".to_owned(),
        );
        let constraint1 = Rank1Constraint::new(
            x.clone().into(),
            y.clone().into(),
            3.into(),
            "x * y == 3".to_owned(),
        );
        let mut system = ConstraintSystem::default();
        system.addConstraint1(constraint0.clone());
        system.addConstraint1(constraint1.clone());
        let new_constraint_sys = GLA::convert(&system);
        assert_eq!(new_constraint_sys.len(), 2usize);
        assert_eq!(new_constraint_sys[0], GLA::convert(&constraint0));
        assert_eq!(new_constraint_sys[1], GLA::convert(&constraint1));
    }

    #[test]
    fn test_VariableAssignment() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA; //GadgetLibAdapter
        GLA::resetVariableIndex;
        let varArray = VariableArray::new(10, "x".to_owned(), VariableArrayBase);
        let mut assignment = VariableAssignment::default();
        for i in 0..varArray.len() {
            assignment.insert(varArray[i].clone(), i.into());
        }
        let new_assignment = GLA::convert(&assignment);
        assert_eq!(assignment.len(), new_assignment.len());
        for i in 0..new_assignment.len() {
            let var = i as variable_index_t;
            assert_eq!(new_assignment[&var], Fp::from(i));
        }
    }

    #[test]
    fn test_Protoboard() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA; //GadgetLibAdapter
        GLA::resetVariableIndex;
        let x = Variable::from("x");
        let y = Variable::from("y");
        let pb = Protoboard::create(FieldType::R1P, None).unwrap();
        pb.borrow_mut().addRank1Constraint(
            x.clone() + &y,
            (x.clone() * 5).into(),
            0.into(),
            "(x + y) * (5*x) == 0",
        );
        pb.borrow_mut().addRank1Constraint(
            x.clone().into(),
            y.clone().into(),
            3.into(),
            "x * y == 3",
        );
        *pb.borrow_mut().val(&x) = 1.into();
        *pb.borrow_mut().val(&y) = 2.into();
        let new_pb = GLA::convert(&*pb.borrow());
        assert_eq!(new_pb.0, GLA::convert(pb.borrow().constraintSystem()));
        assert_eq!(new_pb.1, GLA::convert(pb.borrow().assignment()));
    }
}
