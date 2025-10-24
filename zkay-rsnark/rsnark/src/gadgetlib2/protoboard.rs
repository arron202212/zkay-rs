/** @file
 *****************************************************************************
 Definition of Protoboard, a "memory manager" for building arithmetic constraints
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PROTOBOARD_HPP_
// #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PROTOBOARD_HPP_

// use  <string>

use crate::gadgetlib2::constraint;
use crate::gadgetlib2::pp;
use crate::gadgetlib2::variable;

// #define ASSERT_CONSTRAINTS_SATISFIED(pb) \
    // ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED))

// #define ASSERT_CONSTRAINTS_NOT_SATISFIED(pb) \
    // ASSERT_FALSE(pb->isSatisfied(PrintOptions::NO_DBG_PRINT))

// namespace gadgetlib2 {

// class ProtoboardParams; // Forward declaration
// type ::std::shared_ptr<const ProtoboardParams> ParamsCPtr;

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                       class Protoboard                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
pub struct Protoboard {
// protected:
    assignment_:VariableAssignment,
    constraintSystem_:ConstraintSystem,
    numInputs_:size_t,
    pParams_:ParamsCPtr, 
fieldType_:FieldType,
    // TODO try to refactor this out and use inheritance for different types
                         // of protoboards, for instance TinyRAMProtoboard : public Protoboard
                         // This may not be trivial because of Gadget multiple inheritance scheme
}
//     Protoboard(const FieldType& fieldType, ParamsCPtr pParams);
// 
//    
//     static ProtoboardPtr create(const FieldType& fieldType, ParamsCPtr pParams = NULL) {
//         return ProtoboardPtr(new Protoboard(fieldType, pParams));
//     }
//     size_t numVars() const {return assignment_.len();} // TODO change to take num from constraintSys_
//     //size_t numVars() const {return constraintSystem_.getUsedVariables().len();} // TODO change to take num from constraintSys_

//     size_t numInputs() const {return numInputs_;} // TODO Madars How do we book keep this?
//     ParamsCPtr params() const {return pParams_;}
//     FElem& val(var:&Variable);
//     FElem val(lc:&LinearCombination) const;
//     void setValuesAsBitArray(varArray:&VariableArray, srcValue:usize);
//     void setDualWordValue(dualWord:&DualWord, srcValue:usize);
//     void setMultipackedWordValue(multipackedWord:&MultiPackedWord, srcValue:usize);

//     // The following 3 methods are purposely not overloaded to the same name in order to reduce
//     // programmer error. We want the programmer to explicitly code what type of constraint
//     // she wants.
//     void addRank1Constraint(a:&LinearCombination,
//                             b:&LinearCombination,
//                             c:&LinearCombination,
//                             name:&String);
//     void addGeneralConstraint(a:&Polynomial,
//                               b:&Polynomial,
//                               name:&String);
//     /// adds a constraint of the form (a == 0)
//     void addUnaryConstraint(a:&LinearCombination, name:&String);
//     bool isSatisfied(printOnFail:PrintOptions = PrintOptions::NO_DBG_PRINT);
//     bool flagIsSet(flag:&FlagVariable) const {return val(flag) == 1;}
//     void setFlag(flag:&FlagVariable, bool newFlagState = true);
//     void clearFlag(flag:&FlagVariable) {val(flag) = 0;}
//     void flipFlag(flag:&FlagVariable) {val(flag) = 1 - val(flag);}
//     void enforceBooleanity(var:&Variable);
//     ::std::string annotation() const;
//     ConstraintSystem constraintSystem() const {return constraintSystem_;}
//     VariableAssignment assignment() const {return assignment_;}
//     bool dualWordAssignmentEqualsValue(
//             dualWord:&DualWord,
//             expectedValue:usize,
//             printOption:PrintOptions = PrintOptions::NO_DBG_PRINT) const;
//     bool multipackedWordAssignmentEqualsValue(
//             multipackedWord:&MultiPackedWord,
//             expectedValue:usize,
//             printOption:PrintOptions = PrintOptions::NO_DBG_PRINT) const;
//     bool unpackedWordAssignmentEqualsValue(
//             unpackedWord:&UnpackedWord,
//             expectedValue:usize,
//             printOption:PrintOptions = PrintOptions::NO_DBG_PRINT) const;
// };
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     class ProtoboardParams                 ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/*
    An abstract class to hold any additional information needed by a specific Protoboard. For
    example a Protoboard specific to TinyRAM will have a class ArchParams which will inherit from
    this class.
*/
// class ProtoboardParams {
// 
//     virtual ~ProtoboardParams() = 0;
// };

// } // namespace gadgetlib2

//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PROTOBOARD_HPP_
/** @file
 *****************************************************************************
 Implementation of Protoboard, a "memory manager" for building arithmetic constraints
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <cstdio>

// use crate::gadgetlib2::protoboard;

// using ::std::string;
// using ::std::cout;
// using ::std::endl;

// namespace gadgetlib2 {

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       class Protoboard                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl Protoboard
{
pub fn new(fieldType:&FieldType,  pParams:ParamsCPtr)
    ->Self {
Self{numInputs_:0, pParams_:pParams, fieldType_:fieldType}
}


 pub fn val(&mut self,var:&Variable)->&mut FElem{
    let retval = &assignment_[var];
    assert!(retval.fieldType() == fieldType_ || retval.fieldType() == AGNOSTIC,
                    "Assigned field element of incorrect field type in Variable \"{}\"",
                        var.name());
    return retval;
}

 pub fn val(lc:&LinearCombination) ->FElem {
    return lc.eval(assignment_);
}

pub fn  setValuesAsBitArray(varArray:&VariableArray, srcValue:usize) {
    assert!(varArray.len() >= Log2ceil(srcValue),
                 "Variable array of size {} too small to hold value {}. Array must be of size at least {}", varArray.len(), srcValue, Log2ceil(srcValue));
    let  i = 0;
    for i in 0.. Log2ceil(srcValue) {
        val(varArray[i]) = if srcValue & (1usize<<i)  {1} else{ 0} ;
    }
    for j in i.. varArray.len() {
        val(varArray[j]) = 0 ;
    }
}

pub fn  setDualWordValue(dualWord:&DualWord, srcValue:usize) {
    setMultipackedWordValue(dualWord.multipacked(), srcValue);
    setValuesAsBitArray(dualWord.unpacked(), srcValue);
}

pub fn  setMultipackedWordValue(multipackedWord:&MultiPackedWord,
                                         srcValue:usize)->eyre::Result<()> {
    if fieldType_ == R1P {
        assert!(multipackedWord.len() == 1, "Multipacked word size mismatch in R1P");
        val(multipackedWord[0]) = srcValue;
    } else {
        eyre::bail!("Unknown protoboard type in pub fn setMultipackedWordValue");
    }
    Ok(())
}

// The following 3 methods are purposely not overloaded to the same name in order to reduce
// programmer error. We want the programmer to explicitly code what type of constraint
// she wants.
pub fn  addRank1Constraint(a:&LinearCombination,
                                    b:&LinearCombination,
                                    c:&LinearCombination,
                                    name:&String) {
    constraintSystem_.addConstraint(Rank1Constraint(a,b,c,name));
}

pub fn  addGeneralConstraint(a:&Polynomial,
                                      b:&Polynomial,
                                      name:&String) {
    constraintSystem_.addConstraint(PolynomialConstraint(a,b,name));
}

pub fn  addUnaryConstraint(a:&LinearCombination, name:&String) {
    addRank1Constraint(a, 1, 0, name);
}

 pub fn isSatisfied(printOnFail:PrintOptions)->bool {
    return constraintSystem_.isSatisfied(assignment_, printOnFail);
}

pub fn  setFlag(flag:&FlagVariable,  newFlagState:bool) {
    val(flag) = if newFlagState { 1} else {0};
}

pub fn  enforceBooleanity(var:&Variable) {
    addRank1Constraint(var , var - 1, 0 , format!("enforceBooleanity({})",var.name()));
}

 pub fn annotation() ->String {
// #   ifdef DEBUG
//         string retVal = constraintSystem_.annotation();
//         retVal += "Variable Assignments:\n";
//         for(const auto& assignmentPair : assignment_) {
//             const string varName = assignmentPair.first.name();
//             const string varAssignedValue = assignmentPair.second.asString();
//             retVal +=  varName + ": " + varAssignedValue + "\n";
//         }
//         return retVal;
// #   else // not DEBUG
        return "".to_owned();
// #   endif
}

 pub fn dualWordAssignmentEqualsValue(dualWord:&DualWord,
                                               expectedValue:usize,
                                               printOption:PrintOptions) ->bool {
    let  multipackedEqualsValue = multipackedWordAssignmentEqualsValue(dualWord.multipacked(),
                                                                       expectedValue,
                                                                       printOption);
    let unpackedEqualsValue = unpackedWordAssignmentEqualsValue(dualWord.unpacked(),
                                                                 expectedValue,
                                                                 printOption);
    if multipackedAndUnpackedValuesDisagree(multipackedEqualsValue, unpackedEqualsValue) {
        printInformativeNoticeMessage(multipackedEqualsValue, unpackedEqualsValue);
    }
    return multipackedEqualsValue && unpackedEqualsValue;
}

 pub fn multipackedWordAssignmentEqualsValue(multipackedWord:&MultiPackedWord,
                                                      expectedValue:usize,
                                                      printOption:PrintOptions) ->eyre::Result<bool> {
    let mut  retval = true;
    if fieldType_ == R1P {
        assert!(multipackedWord.len() == 1, "R1P multipacked size mismatch");
        if val(multipackedWord[0]) == expectedValue {
            retval = true;
        } else {
            retval = false;
        }
        if expectedToPrintValues(retval, printOption) {
            cout << "Expected value for multipacked word \"" << multipackedWord.name()
                 << "\" is: " << expectedValue << endl;
            cout << "Actual value is: " << val(multipackedWord[0]) << endl;
        }
    } else {
        eyre::bail!("Unknown field type in pub fn multipackedWordAssignmentEqualsValue(...)");
    }
    return Ok(retval);
}

 pub fn unpackedWordAssignmentEqualsValue(unpackedWord:&UnpackedWord,
                                                   expectedValue:usize,
                                                   printOption:PrintOptions) ->bool {
    let  retval = true;
    let  expectedValueCopy = expectedValue;
    for  i in  0.. unpackedWord.len() {
        if val(unpackedWord[i]) != (expectedValueCopy & 1usize) {
            retval = false;
            break;
        }
        expectedValueCopy >>= 1;
    }
    if expectedValueCopy != 0 {
        retval = false;
    }
    if expectedToPrintValues(retval, printOption) {
        println!("Expected value for unpacked word \"{}\" {expectedValue}",unpackedWord.name() );
        println!("Actual values are: ");
        for i in  0..unpackedWord.len() {
           println!("bit {i} : {}" , val(unpackedWord[i]));
        }
    }
    return retval;
}
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// ProtoboardParams::~ProtoboardParams() {}

// } // namespace gadgetlib2

 pub fn multipackedAndUnpackedValuesDisagree(multipackedEqualsValue:bool,
                                          unpackedEqualsValue:bool)->bool {
    return multipackedEqualsValue != unpackedEqualsValue;
}

 pub fn printInformativeNoticeMessage(multipackedEqualsValue:bool,
                                   unpackedEqualsValue:bool) {
    if multipackedEqualsValue  && !unpackedEqualsValue  {
        println!("NOTE: multipacked value equals expected value but unpacked value does not!");
    } else {
        assert!(!multipackedEqualsValue  && unpackedEqualsValue ,
                     "printInformativeNoticeMessage(...) has been called incorrectly");
       println!( "NOTE: unpacked value equals expected value but multipacked value does not!");
            
    }
}

pub fn  expectedToPrintValues(boolValue:bool, printOption:PrintOptions)->bool {
    return ((boolValue  && printOption == PrintOptions::DBG_PRINT_IF_TRUE) ||
            (!boolValue  && printOption == PrintOptions::DBG_PRINT_IF_FALSE));
}
