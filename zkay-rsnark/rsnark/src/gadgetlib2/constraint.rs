// /** @file
//  *****************************************************************************
//  Declaration of the Constraint class.

//  A constraint is an algebraic equation which can be either satisfied by an assignment,
//  (the equation is true with that assignment) or unsatisfied. For instance the rank-1
//  constraint (X * Y = 15) is satisfied by {X=5 Y=3} or {X=3 Y=5}
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_CONSTRAINT_HPP_
// // #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_CONSTRAINT_HPP_

// use  <string>
// use  <vector>

// use crate::gadgetlib2::variable;

// namespace gadgetlib2 {

// enum class PrintOptions {
//     DBG_PRINT_IF_NOT_SATISFIED,
//     DBG_PRINT_IF_TRUE,
//     DBG_PRINT_IF_FALSE,
//     NO_DBG_PRINT
// };

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                    class Constraint                        ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /// An abstract class for a field agnostic constraint. The derived classes will be field specific.
// class Constraint {
// public:
//     explicit Constraint(const ::std::string& name); // casting disallowed by 'explicit'
//     ::std::string name() const; ///< @returns name of the constraint as a string
//     /**
//         @param[in] assignment  - An assignment of field elements for each variable.
//         @param[in] printOnFail - when set to true, an unsatisfied constraint will print to stderr
//                                  information explaining why it is not satisfied.
//         @returns true if constraint is satisfied by the assignment
//     **/
//     virtual bool isSatisfied(const VariableAssignment& assignment,
//                              const PrintOptions& printOnFail) const = 0;
//     /// @returns the constraint in a human readable string format
//     virtual ::std::string annotation() const = 0;
//     virtual const Variable::set getUsedVariables() const = 0;
//     virtual Polynomial asPolynomial() const = 0;
// protected:
// #   ifdef DEBUG
//     ::std::string name_;
// #   endif

// }; // class Constraint

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 class Rank1Constraint                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// /// A rank-1 prime characteristic constraint. The constraint is defined by <a,x> * <b,x> = <c,x>
// /// where x is an assignment of field elements to the variables.
// class Rank1Constraint : public Constraint {
// private:
//     LinearCombination a_, b_, c_; // <a,x> * <b,x> = <c,x>
// public:
//     Rank1Constraint(const LinearCombination& a,
//                     const LinearCombination& b,
//                     const LinearCombination& c,
//                     const ::std::string& name);

//     LinearCombination a() const;
//     LinearCombination b() const;
//     LinearCombination c() const;

//     virtual bool isSatisfied(const VariableAssignment& assignment,
//                              const PrintOptions& printOnFail = PrintOptions::NO_DBG_PRINT) const;
//     virtual ::std::string annotation() const;
//     virtual const Variable::set getUsedVariables() const; /**< @returns a list of all variables
//                                                                       used in the constraint */
//     virtual Polynomial asPolynomial() const {return a_ * b_ - c_;}
// }; // class Rank1Constraint

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 class PolynomialConstraint                 ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// class PolynomialConstraint : public Constraint {
// private:
//     Polynomial a_, b_;
// public:
//     PolynomialConstraint(const Polynomial& a,
//                          const Polynomial& b,
//                          const ::std::string& name);

//     bool isSatisfied(const VariableAssignment& assignment,
//                      const PrintOptions& printOnFail = PrintOptions::NO_DBG_PRINT) const;
//     ::std::string annotation() const;
//     virtual const Variable::set getUsedVariables() const; /**< @returns a list of all variables
//                                                                         used in the constraint */
//     virtual Polynomial asPolynomial() const {return a_ - b_;}
// }; // class PolynomialConstraint

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   class ConstraintSystem                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// class ConstraintSystem {
// protected:
//     type ::std::shared_ptr<Constraint> ConstraintPtr;
//     ::std::vector<ConstraintPtr> constraintsPtrs_;
// public:
//     ConstraintSystem() : constraintsPtrs_() {};

//     /**
//         Checks if all constraints are satisfied by an assignment.
//         @param[in] assignment  - An assignment of field elements for each variable.
//         @param[in] printOnFail - when set to true, an unsatisfied constraint will print to stderr
//                                  information explaining why it is not satisfied.
//         @returns true if constraint is satisfied by the assignment
//     **/
//     bool isSatisfied(const VariableAssignment& assignment,
//                      const PrintOptions& printOnFail = PrintOptions::NO_DBG_PRINT) const;
//     void addConstraint(const Rank1Constraint& c);
//     void addConstraint(const PolynomialConstraint& c);
//     ::std::string annotation() const;
//     Variable::set getUsedVariables() const;

//     type ::std::set< ::std::unique_ptr<Polynomial> > PolyPtrSet;
//     /// Required for interfacing with BREX. Should be optimized in the future
//     PolyPtrSet getConstraintPolynomials() const {
//         PolyPtrSet retset;
//         for(const auto& pConstraint : constraintsPtrs_) {
//             retset.insert(::std::unique_ptr<Polynomial>(new Polynomial(pConstraint->asPolynomial())));
//         }
//         return retset;
//     }
//     size_t getNumberOfConstraints() { return constraintsPtrs_.size(); }
//     ConstraintPtr getConstraint(size_t idx){ return constraintsPtrs_[idx];}
//     friend class GadgetLibAdapter;
// }; // class ConstraintSystem

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// } // namespace gadgetlib2

// //#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_CONSTRAINT_HPP_
// /** @file
//  *****************************************************************************
//  Implementation of the Constraint class.
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use  <algorithm>
// use  <cassert>
// use  <iostream>
// use  <memory>
// use  <set>

// use crate::gadgetlib2::constraint;
// use crate::gadgetlib2::variable;

// using ::std::string;
// using ::std::vector;
// using ::std::set;
// using ::std::cout;
// using ::std::cerr;
// using ::std::endl;
// using ::std::shared_ptr;

// namespace gadgetlib2 {

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                    class Constraint                        ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// // #ifdef DEBUG
// Constraint::Constraint(const string& name) : name_(name) {}
// #else
// Constraint::Constraint(const string& name) { ffec::UNUSED(name); }
// //#endif

// string Constraint::name() const {
// #   ifdef DEBUG
//         return name_;
// #   else
//         return "";
// #   endif
// }

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 class Rank1Constraint                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// Rank1Constraint::Rank1Constraint(const LinearCombination &a,
//                                const LinearCombination &b,
//                                const LinearCombination &c,
//                                const string& name)
//     : Constraint(name), a_(a), b_(b), c_(c) {}

// LinearCombination Rank1Constraint::a() const {return a_;}
// LinearCombination Rank1Constraint::b() const {return b_;}
// LinearCombination Rank1Constraint::c() const {return c_;}

// bool Rank1Constraint::isSatisfied(const VariableAssignment& assignment,
//                                   const PrintOptions& printOnFail) const {
//     const FElem ares = a_.eval(assignment);
//     const FElem bres = b_.eval(assignment);
//     const FElem cres = c_.eval(assignment);
//     if ares*bres != cres {
// #       ifdef DEBUG
//         if printOnFail == PrintOptions::DBG_PRINT_IF_NOT_SATISFIED {
//             cerr << GADGETLIB2_FMT("Constraint named \"%s\" not satisfied. Constraint is:",
//                 name().c_str()) << endl;
//             cerr << annotation() << endl;
//             cerr << "Variable assignments are:" << endl;
//             const Variable::set varSet = getUsedVariables();
//             for(const Variable& var : varSet) {
//                 cerr <<  var.name() << ": " << assignment.at(var).asString() << endl;
//             }
//             cerr << "a:   " << ares.asString() << endl;
//             cerr << "b:   " << bres.asString() << endl;
//             cerr << "a*b: " << (ares*bres).asString() << endl;
//             cerr << "c:   " << cres.asString() << endl;
//         }
// #       else
//         ffec::UNUSED(printOnFail);
// #       endif
//         return false;
//     }
//     return true;
// }

// string Rank1Constraint::annotation() const {
// #   ifndef DEBUG
//         return "";
// #   endif
//     return string("( ") + a_.asString() + " ) * ( " + b_.asString() + " ) = "+ c_.asString();
// }

// const Variable::set Rank1Constraint::getUsedVariables() const {
//     Variable::set retSet;
//     const Variable::set aSet = a_.getUsedVariables();
//     retSet.insert(aSet.begin(), aSet.end());
//     const Variable::set bSet = b_.getUsedVariables();
//     retSet.insert(bSet.begin(), bSet.end());
//     const Variable::set cSet = c_.getUsedVariables();
//     retSet.insert(cSet.begin(), cSet.end());
//     return retSet;
// }

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 class PolynomialConstraint                 ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// PolynomialConstraint::PolynomialConstraint(const Polynomial& a, const Polynomial& b,
//         const string& name) : Constraint(name), a_(a), b_(b) {}

// bool PolynomialConstraint::isSatisfied(const VariableAssignment& assignment,
//                                        const PrintOptions& printOnFail) const {
//     const FElem aEval = a_.eval(assignment);
//     const FElem bEval = b_.eval(assignment);
//     if aEval != bEval {
// #       ifdef DEBUG
//             if(printOnFail == PrintOptions::DBG_PRINT_IF_NOT_SATISFIED) {
//                 cerr << GADGETLIB2_FMT("Constraint named \"%s\" not satisfied. Constraint is:",
//                     name().c_str()) << endl;
//                 cerr << annotation() << endl;
// 				cerr << "Expecting: " << aEval << " == " << bEval << endl;
//                 cerr << "Variable assignments are:" << endl;
//                 const Variable::set varSet = getUsedVariables();
//                 for(const Variable& var : varSet) {
//                     cerr <<  var.name() << ": " << assignment.at(var).asString() << endl;
//                 }
//             }
// #       else
//             ffec::UNUSED(printOnFail);
// #       endif

//         return false;
//     }
//     return true;
// }

// string PolynomialConstraint::annotation() const {
// #   ifndef DEBUG
//         return "";
// #   endif
//     return a_.asString() + " == " + b_.asString();
// }

// const Variable::set PolynomialConstraint::getUsedVariables() const {
//     Variable::set retSet;
//     const Variable::set aSet = a_.getUsedVariables();
//     retSet.insert(aSet.begin(), aSet.end());
//     const Variable::set bSet = b_.getUsedVariables();
//     retSet.insert(bSet.begin(), bSet.end());
//     return retSet;
// }

// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/


// void ConstraintSystem::addConstraint(const Rank1Constraint& c) {
//     constraintsPtrs_.push(::std::shared_ptr<Constraint>(new Rank1Constraint(c)));
// }

// void ConstraintSystem::addConstraint(const PolynomialConstraint& c) {
//     constraintsPtrs_.push(::std::shared_ptr<Constraint>(new PolynomialConstraint(c)));
// }

// bool ConstraintSystem::isSatisfied(const VariableAssignment& assignment,
//                                    const PrintOptions& printOnFail) const {
//     for(size_t i = 0; i < constraintsPtrs_.size(); ++i) {
//         if !constraintsPtrs_[i]->isSatisfied(assignment, printOnFail){
//             return false;
//         }
//     }
//     return true;
// }

// string ConstraintSystem::annotation() const {
//     string retVal("\n");
//     for(auto i = constraintsPtrs_.begin(); i != constraintsPtrs_.end(); ++i) {
//         retVal += (*i)->annotation() + '\n';
//     }
//     return retVal;
// }

// Variable::set ConstraintSystem::getUsedVariables() const {
//     Variable::set retSet;
//     for(auto& pConstraint : constraintsPtrs_) {
//         const Variable::set curSet = pConstraint->getUsedVariables();
//         retSet.insert(curSet.begin(), curSet.end());
//     }
//     return retSet;
// }

// } // namespace gadgetlib2
