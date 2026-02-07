//  Unit tests for gadgetlib2

// use  <gtest/gtest.h>

// use crate::gadgetlib2::adapters;
// use crate::gadgetlib2::pp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn LinearTerm() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        adapter.resetVariableIndex();
        let x = Variable::from("x");
        let lt = 5 * x;
        let new_lt = adapter.convert(lt);
        assert_eq!(new_lt.first, 0usize);
        assert_eq!(new_lt.second, Fp::from(5));
    }

    #[test]
    fn LinearCombination() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let lc = 5 * x + 3 * y + 42;
        let new_lc = adapter.convert(lc);
        assert_eq!(new_lc.second, Fp::from(42));
        assert_eq!(new_lc.first.len(), 2usize);
        assert_eq!(new_lc.first[0], adapter.convert(5 * x));
        assert_eq!(new_lc.first[1], adapter.convert(3 * y));
    }

    #[test]
    fn Constraint() {
        // using ::std::get;
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let constraint = Rank1Constraint::new(x + y, 5 * x, 0, "(x + y) * (5 * x) == 0");
        let new_constraint = adapter.convert(constraint);
        assert_eq!(new_constraint.0, adapter.convert(x + y));
        assert_eq!(new_constraint.1, adapter.convert(5 * x + 0));
        assert_eq!(new_constraint.2, adapter.convert(LinearCombination(0)));
    }

    #[test]
    fn ConstraintSystem() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        let x = Variable::from("x");
        let y = Variable::from("y");
        let constraint0 = Rank1Constraint::new(x + y, 5 * x, 0, "(x + y) * (5*x) == 0");
        let constraint1 = Rank1Constraint::new(x, y, 3, "x * y == 3");
        let mut system = ConstraintSystem::default();
        system.addConstraint(constraint0);
        system.addConstraint(constraint1);
        let new_constraint_sys = adapter.convert(system);
        assert_eq!(new_constraint_sys.len(), 2usize);
        assert_eq!(new_constraint_sys.at(0), adapter.convert(constraint0));
        assert_eq!(new_constraint_sys.at(1), adapter.convert(constraint1));
    }

    #[test]
    fn VariableAssignment() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        adapter.resetVariableIndex();
        let varArray = VariableArray::new(10, "x");
        let mut assignment = VariableAssignment::default();
        for i in 0..varArray.len() {
            assignment[varArray[i]] = i;
        }
        let new_assignment = adapter.convert(assignment);
        assert_eq!(assignment.len(), new_assignment.len());
        for i in 0..new_assignment.len() {
            let var = i as GadgetLibAdapter::variable_index_t;
            assert_eq!(new_assignment.at(var), Fp::from(i));
        }
    }

    #[test]
    fn Protoboard() {
        initPublicParamsFromDefaultPp();
        let adapter = GLA::default(); //GadgetLibAdapter
        adapter.resetVariableIndex();
        let x = Variable::from("x");
        let y = Variable::from("y");
        let pb = Protoboard::create(R1P);
        pb.addRank1Constraint(x + y, 5 * x, 0, "(x + y) * (5*x) == 0");
        pb.addRank1Constraint(x, y, 3, "x * y == 3");
        pb.val(x) = 1;
        pb.val(y) = 2;
        let new_pb = adapter.convert(*pb);
        assert_eq!(new_pb.first, adapter.convert(pb.constraintSystem()));
        assert_eq!(new_pb.second, adapter.convert(pb.assignment()));
    }
}
