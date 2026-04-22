//  Declaration of the Constraint class.

//  A constraint is an algebraic equation which can be either satisfied by an assignment,
//  (the equation is true with that assignment) or unsatisfied. For instance the rank-1
//  constraint (X * Y = 15) is satisfied by {X=5 Y=3} or {X=3 Y=5}
use crate::gadgetlib2::variable::{
    FElemInterface, LinearCombination, Polynomial, Variable, VariableAssignment, VariableSet,
};
use enum_dispatch::enum_dispatch;
use rccell::RcCell;
use std::collections::BTreeSet;
use strum_macros::{EnumIs, EnumTryAs};
#[derive(PartialEq, Clone)]
pub enum PrintOptions {
    DBG_PRINT_IF_NOT_SATISFIED,
    DBG_PRINT_IF_TRUE,
    DBG_PRINT_IF_FALSE,
    NO_DBG_PRINT,
}

#[enum_dispatch(ConstraintConfig)]
#[derive(EnumIs, EnumTryAs)]
pub enum ConstraintType {
    Rank1(Constraint<Rank1Constraint>),
    Polynomial(Constraint<PolynomialConstraint>),
}

#[enum_dispatch]
pub trait ConstraintConfig {
    fn name(&self) -> &String;
    fn isSatisfied(&self, assignment: &VariableAssignment, printOnFail: &PrintOptions) -> bool;
    fn annotation(&self) -> String;
    fn getUsedVariables(&self) -> VariableSet;
    fn asPolynomial(&self) -> Polynomial;
}

/// An abstract struct for a field agnostic constraint. The derived classes will be field specific.
#[derive(Default, Clone)]
pub struct Constraint<T: Default + Clone> {
    
    pub name_: String,
    pub t: T,

}

/// A rank-1 prime characteristic constraint. The constraint is defined by <a,x> * <b,x> = <c,x>
/// where x is an assignment of field elements to the variables.
#[derive(Default, Clone)]
pub struct Rank1Constraint {
    pub a_: LinearCombination,
    pub b_: LinearCombination,
    pub c_: LinearCombination, 
}

#[derive(Default, Clone)]
pub struct PolynomialConstraint {
    pub a_: Polynomial,
    pub b_: Polynomial,
}


pub type ConstraintPtr = RcCell<ConstraintType>;
#[derive(Default, Clone)]
pub struct ConstraintSystem {
    pub constraintsPtrs_: Vec<ConstraintPtr>,
}
pub type PolyPtrSet = BTreeSet<RcCell<Polynomial>>;
impl ConstraintSystem {
   

   
    //
    // /// Required for interfacing with BREX. Should be optimized in the future
    pub fn getConstraintPolynomials(&self) -> PolyPtrSet {
        let mut retset = PolyPtrSet::new();
        for pConstraint in &self.constraintsPtrs_ {
            retset.insert(RcCell::new(pConstraint.borrow().asPolynomial()));
        }
        retset
    }
    pub fn getNumberOfConstraints(&self) -> usize {
        self.constraintsPtrs_.len()
    }
    pub fn getConstraint(&self, idx: usize) -> &ConstraintPtr {
        &self.constraintsPtrs_[idx]
    }
}

impl<T: Default + Clone> Constraint<T> {
    pub fn new(name: String, t: T) -> Self {
        Self { name_: name, t }
    }
    
}

impl Rank1Constraint {
    pub fn new(
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
        name: String,
    ) -> Constraint<Self> {
        Constraint::<Self>::new(
            name,
            Self {
                a_: a,
                b_: b,
                c_: c,
            },
        )
    }

    pub fn a(&self) -> &LinearCombination {
        &self.a_
    }
    pub fn b(&self) -> &LinearCombination {
        &self.b_
    }
    pub fn c(&self) -> &LinearCombination {
        &self.c_
    }
}

impl ConstraintConfig for Constraint<Rank1Constraint> {
    fn name(&self) -> &String {
        &self.name_
    }
    fn isSatisfied(&self, assignment: &VariableAssignment, printOnFail: &PrintOptions) -> bool {
        let ares = self.t.a_.eval(assignment);
        let bres = self.t.b_.eval(assignment);
        let cres = self.t.c_.eval(assignment);
        if ares.clone() * &bres == cres {
            return true;
        }
       
        if printOnFail == &PrintOptions::DBG_PRINT_IF_NOT_SATISFIED {
            println!(
                "Constraint named \"{}\" not satisfied. Constraint is:",
                self.name()
            );
            println!("{}", self.annotation());
            println!("Variable assignments are:");
            let varSet = self.getUsedVariables();
            for var in varSet {
                println!(
                    "{}: {}",
                    var.name(),
                    assignment.get(&var).unwrap().asString()
                );
            }
            println!("a: {}", ares.asString());
            println!("b:   {}", bres.asString());
            println!("a*b: {}", (ares * &bres).asString());
            println!("c:   {}", cres.asString());
        }

        false
    }

    fn annotation(&self) -> String {
        format!(
            "( {} )*( {} ) = {}",
            self.t.a_.asString(),
            self.t.b_.asString(),
            self.t.c_.asString()
        )
    }

    fn getUsedVariables(&self) -> VariableSet {
        let mut retSet = VariableSet::new();
        let aSet = self.t.a_.getUsedVariables();
        retSet.append(&mut aSet.clone());
        let bSet = self.t.b_.getUsedVariables();
        retSet.append(&mut bSet.clone());
        let cSet = self.t.c_.getUsedVariables();
        retSet.append(&mut cSet.clone());
        retSet
    }
    fn asPolynomial(&self) -> Polynomial {
        self.t.a_.clone() * &self.t.b_ - &self.t.c_
    }
}

impl PolynomialConstraint {
    pub fn new(a: Polynomial, b: Polynomial, name: String) -> Constraint<Self> {
        Constraint::<Self>::new(name, Self { a_: a, b_: b })
    }
}
impl ConstraintConfig for Constraint<PolynomialConstraint> {
    fn name(&self) -> &String {
        &self.name_
    }
    fn isSatisfied(&self, assignment: &VariableAssignment, printOnFail: &PrintOptions) -> bool {
        let aEval = self.t.a_.eval(assignment);
        let bEval = self.t.b_.eval(assignment);
        if aEval == bEval {
            return true;
        }
        if (printOnFail == &PrintOptions::DBG_PRINT_IF_NOT_SATISFIED) {
            println!(
                "Constraint named \"{}\" not satisfied. Constraint is:",
                self.name()
            );
            println!("{}", self.annotation());
            println!("Expecting: {} == {}", aEval, bEval);
            println!("Variable assignments are:");
            let varSet = self.getUsedVariables();
            for var in varSet {
                println!(
                    "{}: {}",
                    var.name(),
                    assignment.get(&var).unwrap().asString()
                );
            }
        }

        false
    }

    fn annotation(&self) -> String {
        format!("{} == {}", self.t.a_.asString(), self.t.b_.asString())
    }

    fn getUsedVariables(&self) -> VariableSet {
        let mut retSet = VariableSet::new();
        let aSet = self.t.a_.getUsedVariables();
        retSet.append(&mut aSet.keys().cloned().collect::<VariableSet>());
        let bSet = self.t.b_.getUsedVariables();
        retSet.append(&mut bSet.keys().cloned().collect::<VariableSet>());
        retSet
    }
    fn asPolynomial(&self) -> Polynomial {
        self.t.a_.clone() - &self.t.b_
    }
}

impl ConstraintSystem {
    pub fn addConstraint1(&mut self, c: Constraint<Rank1Constraint>) {
        self.constraintsPtrs_
            .push(RcCell::new(ConstraintType::Rank1(c)));
    }

    pub fn addConstraint(&mut self, c: Constraint<PolynomialConstraint>) {
        self.constraintsPtrs_
            .push(RcCell::new(ConstraintType::Polynomial(c)));
    }

    pub fn isSatisfied(&self, assignment: &VariableAssignment, printOnFail: &PrintOptions) -> bool {
        for i in 0..self.constraintsPtrs_.len() {
            if !self.constraintsPtrs_[i]
                .borrow()
                .isSatisfied(assignment, printOnFail)
            {
                return false;
            }
        }
        true
    }

    pub fn annotation(&self) -> String {
        let mut retVal = "\n".to_owned();
        for i in &self.constraintsPtrs_ {
            retVal += &(i.borrow().annotation() + "\n");
        }
        retVal
    }

    pub fn getUsedVariables(&self) -> VariableSet {
        let mut retSet = VariableSet::new();
        for pConstraint in &self.constraintsPtrs_ {
            let curSet = pConstraint.borrow().getUsedVariables();
            retSet.append(&mut curSet.clone());
        }
        retSet
    }
}
