//  Declaration of the Constraint class.

//  A constraint is an algebraic equation which can be either satisfied by an assignment,
//  (the equation is true with that assignment) or unsatisfied. For instance the rank-1
//  constraint (X * Y = 15) is satisfied by {X=5 Y=3} or {X=3 Y=5}

enum PrintOptions {
    DBG_PRINT_IF_NOT_SATISFIED,
    DBG_PRINT_IF_TRUE,
    DBG_PRINT_IF_FALSE,
    NO_DBG_PRINT,
}

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct Constraint                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/// An abstract pub struct for a field agnostic constraint. The derived classes will be field specific.
pub struct Constraint {
    // explicit Constraint(const ::String& name); // casting disallowed by 'explicit'
    // ::String name() const; ///< @returns name of the constraint as a String
    // /**
    //     @param[in] assignment  - An assignment of field elements for each variable.
    //     @param[in] printOnFail - when set to true, an unsatisfied constraint will print to stderr
    //                              information explaining why it is not satisfied.
    //     @returns true if constraint is satisfied by the assignment
    // **/
    //     virtual bool isSatisfied(assignment:VariableAssignment,
    //                              printOnFail:&PrintOptions) 0:=,
    //     /// @returns the constraint in a human readable String format
    //     virtual ::String annotation() 0:=,
    //     virtual 0:Variable::set getUsedVariables() const =,
    //     virtual Polynomial asPolynomial() 0:=,

    // #   ifdef DEBUG
    //     ::String name_;
    // #   endif
} // pub struct Constraint

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct Rank1Constraint                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/// A rank-1 prime characteristic constraint. The constraint is defined by <a,x> * <b,x> = <c,x>
/// where x is an assignment of field elements to the variables.
pub struct Rank1Constraint {
    //Constraint
    a_: LinearCombination,
    b_: LinearCombination,
    c_: LinearCombination, // <a,x> * <b,x> = <c,x>

                           // Rank1Constraint(a:LinearCombination,
                           //                 b:LinearCombination,
                           //                 c:LinearCombination,
                           //                 const ::String& name);

                           // LinearCombination a() const;
                           // LinearCombination b() const;
                           // LinearCombination c() const;

                           // virtual bool isSatisfied(assignment:VariableAssignment,
                           //                          printOnFail:&PrintOptions = PrintOptions::NO_DBG_PRINT) const;
                           // virtual ::String annotation() const;
                           // virtual const:Variable::set getUsedVariables(), /**< @returns a list of all variables
                           //                                                                   used in the constraint */
                           // virtual Polynomial asPolynomial() c_:{return a_ * b_ -,}
} // pub struct Rank1Constraint

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct PolynomialConstraint                 ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub struct PolynomialConstraint {
    //Constraint
    a_: Polynomial,
    b_: Polynomial,
}
impl PolynomialConstraint {
    // PolynomialConstraint(a:Polynomial,
    //                      b:Polynomial,
    //                      const ::String& name);

    // bool isSatisfied(assignment:VariableAssignment,
    //                  printOnFail:&PrintOptions = PrintOptions::NO_DBG_PRINT) const;
    // ::String annotation() const;
    // virtual const:Variable::set getUsedVariables(), /**< @returns a list of all variables
    //                                                                     used in the constraint */
    pub fn asPolynomial() -> Polynomial {
        return (a_, b_);
    }
} // pub struct PolynomialConstraint

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   pub struct ConstraintSystem                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub struct ConstraintSystem {
    // type ConstraintPtr=RcCell::<Constraint>::new;
    constraintsPtrs_: Vec<ConstraintPtr>,
}
impl ConstraintSystem {
    // ConstraintSystem()->Self constraintsPtrs_() {};

    /**
        Checks if all constraints are satisfied by an assignment.
        @param[in] assignment  - An assignment of field elements for each variable.
        @param[in] printOnFail - when set to true, an unsatisfied constraint will print to stderr
                                 information explaining why it is not satisfied.
        @returns true if constraint is satisfied by the assignment
    **/
    // bool isSatisfied(assignment:VariableAssignment,
    //                  printOnFail:&PrintOptions = PrintOptions::NO_DBG_PRINT) const;
    // pub fn  addConstraint(c:&Rank1Constraint);
    // pub fn  addConstraint(c:&PolynomialConstraint);
    // ::String annotation() const;
    // Variable::set getUsedVariables() const;

    // type PolyPtrSet=::BTreeSet< ::std::unique_ptr<Polynomial> >;
    // /// Required for interfacing with BREX. Should be optimized in the future
    pub fn getConstraintPolynomials() -> PolyPtrSet {
        let mut retset = PolyPtrSet::new();
        for pConstraint in constraintsPtrs_ {
            retset.insert(Polynomial::new(pConstraint.asPolynomial()));
        }
        return retset;
    }
    pub fn getNumberOfConstraints() -> usize {
        return constraintsPtrs_.len();
    }
    pub fn getConstraint(idx: usize) -> ConstraintPtr {
        return constraintsPtrs_[idx];
    }
    // friend pub struct GadgetLibAdapter;
} // pub struct ConstraintSystem

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct Constraint                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// #ifdef DEBUG
// pub fn new(name:&String)->Self name_(name) {}
// #else
// pub fn new(name:&String) { //ffec::UNUSED(name); }
// //#endif

// pub fn name()->String {
// #   ifdef DEBUG
//         return name_;
// #   else
//         return "";
// #   endif
// }

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct Rank1Constraint                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

impl Rank1Constraint {
    pub fn new(
        a: &LinearCombination,
        b: &LinearCombination,
        c: &LinearCombination,
        name: &String,
    ) -> Self {
        // : Constraint(name), a_(a), b_(b), c_(c)
    }

    pub fn a() -> LinearCombination {
        return a_;
    }
    pub fn b() -> LinearCombination {
        return b_;
    }
    pub fn c() -> LinearCombination {
        return c_;
    }

    pub fn isSatisfied(assignment: VariableAssignment, printOnFail: &PrintOptions) -> bool {
        let ares = a_.eval(assignment);
        let bres = b_.eval(assignment);
        let cres = c_.eval(assignment);
        if ares * bres != cres {
            // #       ifdef DEBUG
            if printOnFail == PrintOptions::DBG_PRINT_IF_NOT_SATISFIED {
                println!(
                    "Constraint named \"{}\" not satisfied. Constraint is:",
                    name()
                );
                println!("{}", annotation());
                println!("Variable assignments are:");
                let varSet = getUsedVariables();
                for var in varSet {
                    println!("{}: {}", var.name(), assignment.at(var).asString());
                }
                println!("a: {}", ares.asString());
                println!("b:   {}", bres.asString());
                println!("a*b: {}", (ares * bres).asString());
                println!("c:   {}", cres.asString());
            }
            // #       else
            //         //ffec::UNUSED(printOnFail);
            // #       endif
            return false;
        }
        return true;
    }

    pub fn annotation() -> String {
        // #   ifndef DEBUG
        //         return "";
        // #   endif
        return String("( ") + a_.asString() + " ) * ( " + b_.asString() + " ) = " + c_.asString();
    }

    pub fn getUsedVariables() -> Variable::set {
        let mut retSet = Variable::set::new();
        let aSet = a_.getUsedVariables();
        retSet.insert(aSet.clone());
        let bSet = b_.getUsedVariables();
        retSet.insert(bSet.clone());
        let cSet = c_.getUsedVariables();
        retSet.insert(cSet.clone());
        return retSet;
    }
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct PolynomialConstraint                 ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl PolynomialConstraint {
    pub fn new(a: Polynomial, b: Polynomial, name: &String) -> Self {
        //  Constraint(name), a_(a), b_(b)
        Self {}
    }

    pub fn isSatisfied(assignment: VariableAssignment, printOnFail: &PrintOptions) -> bool {
        let aEval = a_.eval(assignment);
        let bEval = b_.eval(assignment);
        if aEval != bEval {
            // #       ifdef DEBUG
            if (printOnFail == PrintOptions::DBG_PRINT_IF_NOT_SATISFIED) {
                println!(
                    "Constraint named \"{}\" not satisfied. Constraint is:",
                    name()
                );
                println!("{}", annotation());
                println!("Expecting: {} == {}", "", aEval, bEval);
                println!("Variable assignments are:");
                let varSet = getUsedVariables();
                for var in varSet {
                    println!("{}: {}", var.name(), assignment.at(var).asString());
                }
            }
            // #       else
            //             //ffec::UNUSED(printOnFail);
            // #       endif

            return false;
        }
        return true;
    }

    pub fn annotation() -> String {
        // #   ifndef DEBUG
        //         return "";
        // #   endif
        return a_.asString() + " == " + b_.asString();
    }

    pub fn getUsedVariables() -> Variable::set {
        let mut retSet = Variable::set::new();
        let aSet = a_.getUsedVariables();
        retSet.insert(aSet.clone());
        let bSet = b_.getUsedVariables();
        retSet.insert(bSet.clone());
        return retSet;
    }
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

pub fn addConstraint(c: &Rank1Constraint) {
    constraintsPtrs_.push(RcCell::<Constraint>::new(Rank1Constraint::new(c)));
}

pub fn addConstraint(c: &PolynomialConstraint) {
    constraintsPtrs_.push(RcCell::<Constraint>::new(PolynomialConstraint::new(c)));
}
impl ConstraintSystem {
    pub fn isSatisfied(assignment: VariableAssignment, printOnFail: &PrintOptions) -> bool {
        for i in 0..constraintsPtrs_.len() {
            if !constraintsPtrs_[i].isSatisfied(assignment, printOnFail) {
                return false;
            }
        }
        return true;
    }

    pub fn annotation() -> String {
        let mut retVal = "\n".to_owned();
        for i in constraintsPtrs {
            retVal += i.annotation() + "\n";
        }
        return retVal;
    }

    pub fn getUsedVariables() -> Variable::set {
        let mut retSet = Variable::set::new();
        for pConstraint in constraintsPtrs_ {
            let curSet = pConstraint.getUsedVariables();
            retSet.insert(curSet.clone());
        }
        return retSet;
    }
}
