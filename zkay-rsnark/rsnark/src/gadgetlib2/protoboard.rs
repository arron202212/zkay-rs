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

use  <string>

use libsnark/gadgetlib2/constraint;
use libsnark/gadgetlib2/pp;
use libsnark/gadgetlib2/variable;

// #define ASSERT_CONSTRAINTS_SATISFIED(pb) \
    ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED))

// #define ASSERT_CONSTRAINTS_NOT_SATISFIED(pb) \
    ASSERT_FALSE(pb->isSatisfied(PrintOptions::NO_DBG_PRINT))

namespace gadgetlib2 {

class ProtoboardParams; // Forward declaration
type ::std::shared_ptr<const ProtoboardParams> ParamsCPtr;

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       class Protoboard                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
class Protoboard {
protected:
    VariableAssignment assignment_;
    ConstraintSystem constraintSystem_;
    size_t numInputs_;
    ParamsCPtr pParams_; // TODO try to refactor this out and use inheritance for different types
                         // of protoboards, for instance TinyRAMProtoboard : public Protoboard
                         // This may not be trivial because of Gadget multiple inheritance scheme

    Protoboard(const FieldType& fieldType, ParamsCPtr pParams);
public:
    const FieldType fieldType_;
    static ProtoboardPtr create(const FieldType& fieldType, ParamsCPtr pParams = NULL) {
        return ProtoboardPtr(new Protoboard(fieldType, pParams));
    }
    size_t numVars() const {return assignment_.size();} // TODO change to take num from constraintSys_
    //size_t numVars() const {return constraintSystem_.getUsedVariables().size();} // TODO change to take num from constraintSys_

    size_t numInputs() const {return numInputs_;} // TODO Madars How do we book keep this?
    ParamsCPtr params() const {return pParams_;}
    FElem& val(const Variable& var);
    FElem val(const LinearCombination& lc) const;
    void setValuesAsBitArray(const VariableArray& varArray, const size_t srcValue);
    void setDualWordValue(const DualWord& dualWord, const size_t srcValue);
    void setMultipackedWordValue(const MultiPackedWord& multipackedWord, const size_t srcValue);

    // The following 3 methods are purposely not overloaded to the same name in order to reduce
    // programmer error. We want the programmer to explicitly code what type of constraint
    // she wants.
    void addRank1Constraint(const LinearCombination& a,
                            const LinearCombination& b,
                            const LinearCombination& c,
                            const ::std::string& name);
    void addGeneralConstraint(const Polynomial& a,
                              const Polynomial& b,
                              const ::std::string& name);
    /// adds a constraint of the form (a == 0)
    void addUnaryConstraint(const LinearCombination& a, const ::std::string& name);
    bool isSatisfied(const PrintOptions& printOnFail = PrintOptions::NO_DBG_PRINT);
    bool flagIsSet(const FlagVariable& flag) const {return val(flag) == 1;}
    void setFlag(const FlagVariable& flag, bool newFlagState = true);
    void clearFlag(const FlagVariable& flag) {val(flag) = 0;}
    void flipFlag(const FlagVariable& flag) {val(flag) = 1 - val(flag);}
    void enforceBooleanity(const Variable& var);
    ::std::string annotation() const;
    ConstraintSystem constraintSystem() const {return constraintSystem_;}
    VariableAssignment assignment() const {return assignment_;}
    bool dualWordAssignmentEqualsValue(
            const DualWord& dualWord,
            const size_t expectedValue,
            const PrintOptions& printOption = PrintOptions::NO_DBG_PRINT) const;
    bool multipackedWordAssignmentEqualsValue(
            const MultiPackedWord& multipackedWord,
            const size_t expectedValue,
            const PrintOptions& printOption = PrintOptions::NO_DBG_PRINT) const;
    bool unpackedWordAssignmentEqualsValue(
            const UnpackedWord& unpackedWord,
            const size_t expectedValue,
            const PrintOptions& printOption = PrintOptions::NO_DBG_PRINT) const;
};
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
class ProtoboardParams {
public:
    virtual ~ProtoboardParams() = 0;
};

} // namespace gadgetlib2

//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_PROTOBOARD_HPP_
/** @file
 *****************************************************************************
 Implementation of Protoboard, a "memory manager" for building arithmetic constraints
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cstdio>

use libsnark/gadgetlib2/protoboard;

using ::std::string;
using ::std::cout;
using ::std::endl;

namespace gadgetlib2 {

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       class Protoboard                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

Protoboard::Protoboard(const FieldType& fieldType, ParamsCPtr pParams)
    : numInputs_(0), pParams_(pParams), fieldType_(fieldType) {}


FElem& Protoboard::val(const Variable &var) {
    FElem& retval = assignment_[var];
    GADGETLIB_ASSERT(retval.fieldType() == fieldType_ || retval.fieldType() == AGNOSTIC,
                    GADGETLIB2_FMT("Assigned field element of incorrect field type in Variable \"%s\"",
                        var.name().c_str()));
    return retval;
}

FElem Protoboard::val(const LinearCombination& lc) const {
    return lc.eval(assignment_);
}

void Protoboard::setValuesAsBitArray(const VariableArray& varArray, const size_t srcValue) {
    GADGETLIB_ASSERT(varArray.size() >= Log2ceil(srcValue),
                 GADGETLIB2_FMT("Variable array of size %u too small to hold value %u. Array must be of size "
                     "at least %u", varArray.size(), srcValue, Log2ceil(srcValue)));
    size_t i = 0;
    for(i = 0; i < Log2ceil(srcValue); ++i) {
        val(varArray[i]) = srcValue & (1u<<i) ? 1 : 0 ;
    }
    for(; i < varArray.size(); ++i) {
        val(varArray[i]) = 0 ;
    }
}

void Protoboard::setDualWordValue(const DualWord& dualWord, const size_t srcValue) {
    setMultipackedWordValue(dualWord.multipacked(), srcValue);
    setValuesAsBitArray(dualWord.unpacked(), srcValue);
}

void Protoboard::setMultipackedWordValue(const MultiPackedWord& multipackedWord,
                                         const size_t srcValue) {
    if (fieldType_ == R1P) {
        GADGETLIB_ASSERT(multipackedWord.size() == 1, "Multipacked word size mismatch in R1P");
        val(multipackedWord[0]) = srcValue;
    } else {
        GADGETLIB_FATAL("Unknown protoboard type in Protoboard::setMultipackedWordValue");
    }
}

// The following 3 methods are purposely not overloaded to the same name in order to reduce
// programmer error. We want the programmer to explicitly code what type of constraint
// she wants.
void Protoboard::addRank1Constraint(const LinearCombination& a,
                                    const LinearCombination& b,
                                    const LinearCombination& c,
                                    const ::std::string& name) {
    constraintSystem_.addConstraint(Rank1Constraint(a,b,c,name));
}

void Protoboard::addGeneralConstraint(const Polynomial& a,
                                      const Polynomial& b,
                                      const ::std::string& name) {
    constraintSystem_.addConstraint(PolynomialConstraint(a,b,name));
}

void Protoboard::addUnaryConstraint(const LinearCombination& a, const ::std::string& name) {
    addRank1Constraint(a, 1, 0, name);
}

bool Protoboard::isSatisfied(const PrintOptions& printOnFail) {
    return constraintSystem_.isSatisfied(assignment_, printOnFail);
}

void Protoboard::setFlag(const FlagVariable& flag, bool newFlagState) {
    val(flag) = newFlagState ? 1 : 0;
}

void Protoboard::enforceBooleanity(const Variable& var) {
    addRank1Constraint(var , var - 1, 0 , GADGETLIB2_FMT("enforceBooleanity(%s)",var.name().c_str()));
}

string Protoboard::annotation() const {
#   ifdef DEBUG
        string retVal = constraintSystem_.annotation();
        retVal += "Variable Assignments:\n";
        for(const auto& assignmentPair : assignment_) {
            const string varName = assignmentPair.first.name();
            const string varAssignedValue = assignmentPair.second.asString();
            retVal +=  varName + ": " + varAssignedValue + "\n";
        }
        return retVal;
#   else // not DEBUG
        return "";
#   endif
}

bool multipackedAndUnpackedValuesDisagree(const bool multipackedEqualsValue,
                                          const bool unpackedEqualsValue) {
    return multipackedEqualsValue != unpackedEqualsValue;
}

void printInformativeNoticeMessage(const bool multipackedEqualsValue,
                                   const bool unpackedEqualsValue) {
    if (multipackedEqualsValue == true && unpackedEqualsValue == false) {
        cout << "NOTE: multipacked value equals expected value but unpacked value does not!"
             << endl;
    } else {
        GADGETLIB_ASSERT(multipackedEqualsValue == false && unpackedEqualsValue == true,
                     "printInformativeNoticeMessage(...) has been called incorrectly");
        cout << "NOTE: unpacked value equals expected value but multipacked value does not!"
             << endl;
    }
}

bool Protoboard::dualWordAssignmentEqualsValue(const DualWord& dualWord,
                                               const size_t expectedValue,
                                               const PrintOptions& printOption) const {
    bool multipackedEqualsValue = multipackedWordAssignmentEqualsValue(dualWord.multipacked(),
                                                                       expectedValue,
                                                                       printOption);
    bool unpackedEqualsValue = unpackedWordAssignmentEqualsValue(dualWord.unpacked(),
                                                                 expectedValue,
                                                                 printOption);
    if (multipackedAndUnpackedValuesDisagree(multipackedEqualsValue, unpackedEqualsValue)) {
        printInformativeNoticeMessage(multipackedEqualsValue, unpackedEqualsValue);
    }
    return multipackedEqualsValue && unpackedEqualsValue;
}

bool expectedToPrintValues(const bool boolValue, const PrintOptions& printOption) {
    return ((boolValue == true && printOption == PrintOptions::DBG_PRINT_IF_TRUE) ||
            (boolValue == false && printOption == PrintOptions::DBG_PRINT_IF_FALSE));
}

bool Protoboard::multipackedWordAssignmentEqualsValue(const MultiPackedWord& multipackedWord,
                                                      const size_t expectedValue,
                                                      const PrintOptions& printOption) const {
    bool retval = true;
    if (fieldType_ == R1P) {
        GADGETLIB_ASSERT(multipackedWord.size() == 1, "R1P multipacked size mismatch");
        if (val(multipackedWord[0]) == expectedValue) {
            retval = true;
        } else {
            retval = false;
        }
        if (expectedToPrintValues(retval, printOption)) {
            cout << "Expected value for multipacked word \"" << multipackedWord.name()
                 << "\" is: " << expectedValue << endl;
            cout << "Actual value is: " << val(multipackedWord[0]) << endl;
        }
    } else {
        GADGETLIB_FATAL("Unknown field type in Protoboard::multipackedWordAssignmentEqualsValue(...)");
    }
    return retval;
}

bool Protoboard::unpackedWordAssignmentEqualsValue(const UnpackedWord& unpackedWord,
                                                   const size_t expectedValue,
                                                   const PrintOptions& printOption) const {
    bool retval = true;
    size_t expectedValueCopy = expectedValue;
    for(size_t i = 0; i < unpackedWord.size(); ++i) {
        if (val(unpackedWord[i]) != (expectedValueCopy & 1u)) {
            retval = false;
            break;
        }
        expectedValueCopy >>= 1;
    }
    if (expectedValueCopy != 0) {
        retval = false;
    }
    if (expectedToPrintValues(retval, printOption)) {
        cout << "Expected value for unpacked word \"" << unpackedWord.name()
             << "\" is: " << expectedValue << endl;
        cout << "Actual values are: " << endl;
        for(size_t i = 0; i < unpackedWord.size(); ++i) {
            cout << "bit " << i << ": " << val(unpackedWord[i]) << endl;
        }
    }
    return retval;
}


/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

ProtoboardParams::~ProtoboardParams() {}

} // namespace gadgetlib2
