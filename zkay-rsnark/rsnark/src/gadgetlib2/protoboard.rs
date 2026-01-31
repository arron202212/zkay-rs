//  Definition of Protoboard, a "memory manager" for building arithmetic constraints
use crate::gadgetlib2::constraint::{
    ConstraintSystem, PolynomialConstraint, PrintOptions, Rank1Constraint,
};
use crate::gadgetlib2::infrastructure::Log2ceil;
use crate::gadgetlib2::variable::{
    DualWord, FElem, FElemInterface, FieldType, FlagVariable, LinearCombination, MultiPackedWord,
    Polynomial, ProtoboardPtr, SubVariableArrayConfig, UnpackedWord, Variable, VariableArray,
    VariableArrayConfig, VariableAssignment,
};
use rccell::RcCell;

// #define ASSERT_CONSTRAINTS_SATISFIED(pb) \
// ASSERT_TRUE(pb->isSatisfied(PrintOptions::DBG_PRINT_IF_NOT_SATISFIED))

// #define ASSERT_CONSTRAINTS_NOT_SATISFIED(pb) \
// ASSERT_FALSE(pb->isSatisfied(PrintOptions::NO_DBG_PRINT))

// pub struct ProtoboardParams; // Forward declaration
pub type ParamsCPtr = Option<RcCell<ProtoboardParams>>;

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                       pub struct Protoboard                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
pub struct Protoboard {
    pub assignment_: VariableAssignment,
    pub constraintSystem_: ConstraintSystem,
    pub numInputs_: usize,
    pub pParams_: ParamsCPtr,
    pub fieldType_: FieldType,
    // TODO try to refactor this out and use inheritance for different types
    // of protoboards, for instance TinyRAMProtoboard : public Protoboard
    // This may not be trivial because of Gadget multiple inheritance scheme
}
//     Protoboard(fieldType:FieldType&, ParamsCPtr pParams);
//
impl Protoboard {
    pub fn create(fieldType: FieldType, pParams: ParamsCPtr) -> ProtoboardPtr {
        Some(RcCell::new(Protoboard::new(fieldType, pParams)))
    }
    pub fn numVars(&self) -> usize {
        self.assignment_.len()
    } // TODO change to take num from constraintSys_
    pub fn numVarss(&self) -> usize {
        self.constraintSystem_.getUsedVariables().len()
    } // TODO change to take num from constraintSys_

    pub fn numInputs(&self) -> usize {
        self.numInputs_
    } // TODO Madars How do we book keep this?
    pub fn params(&self) -> &ParamsCPtr {
        &self.pParams_
    }
    //     FElem& val(var:&Variable);
    //     FElem val(lc:&LinearCombination) const;
    //     pub fn  setValuesAsBitArray(varArray:&VariableArray, srcValue:usize);
    //     pub fn  setDualWordValue(dualWord:&DualWord, srcValue:usize);
    //     pub fn  setMultipackedWordValue(multipackedWord:&MultiPackedWord, srcValue:usize);

    //     // The following 3 methods are purposely not overloaded to the same name in order to reduce
    //     // programmer error. We want the programmer to explicitly code what type of constraint
    //     // she wants.
    //     pub fn  addRank1Constraint(a:&LinearCombination,
    //                             b:&LinearCombination,
    //                             c:&LinearCombination,
    //                             name:&String);
    //     pub fn  addGeneralConstraint(a:&Polynomial,
    //                               b:&Polynomial,
    //                               name:&String);
    //     /// adds a constraint of the form (a == 0)
    //     pub fn  addUnaryConstraint(a:&LinearCombination, name:&String);
    //     bool isSatisfied(printOnFail:PrintOptions = PrintOptions::NO_DBG_PRINT);
    pub fn flagIsSet(&mut self, flag: &FlagVariable) -> bool {
        *self.val(flag) == 1
    }
    // pub fn  setFlag(flag:&FlagVariable, bool newFlagState = true);
    pub fn clearFlag(&mut self, flag: &FlagVariable) {
        *self.val(flag) = FElem::from(0);
    }
    pub fn flipFlag(&mut self, flag: &FlagVariable) {
        *self.val(flag) = FElem::from(1) - &*self.val(flag);
    }
    //     pub fn  enforceBooleanity(var:&Variable);
    //     ::String annotation() const;
    pub fn constraintSystem(&self) -> &ConstraintSystem {
        &self.constraintSystem_
    }
    pub fn assignment(&self) -> &VariableAssignment {
        &self.assignment_
    }
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
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     pub struct ProtoboardParams                 ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/*
    An abstract pub struct to hold any additional information needed by a specific Protoboard. For
    example a Protoboard specific to TinyRAM will have a pub struct ArchParams which will inherit from
    this class.
*/
pub struct ProtoboardParams;
//  {
//
//     virtual ~ProtoboardParams() = 0;
// };

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       pub struct Protoboard                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl Protoboard {
    pub fn new(fieldType: FieldType, pParams: ParamsCPtr) -> Self {
        Self {
            assignment_: VariableAssignment::default(),
            constraintSystem_: ConstraintSystem::default(),
            numInputs_: 0,
            pParams_: pParams,
            fieldType_: fieldType,
        }
    }

    pub fn val(&mut self, var: &Variable) -> &mut FElem {
        let retval = self.assignment_.get_mut(var).unwrap();
        assert!(
            retval.fieldType() == self.fieldType_ || retval.fieldType() == FieldType::AGNOSTIC,
            "Assigned field element of incorrect field type in Variable \"{}\"",
            var.name()
        );
        return retval;
    }

    pub fn val_lc(&self, lc: &LinearCombination) -> FElem {
        lc.eval(&self.assignment_)
    }

    pub fn setValuesAsBitArray<T: SubVariableArrayConfig>(
        &mut self,
        varArray: &VariableArray<T>,
        srcValue: usize,
    ) {
        assert!(
            varArray.len() >= Log2ceil(srcValue as u64) as usize,
            "Variable array of size {} too small to hold value {}. Array must be of size at least {}",
            varArray.len(),
            srcValue,
            Log2ceil(srcValue as u64)
        );
        let i = 0;
        for i in 0..Log2ceil(srcValue as u64) as usize {
            *self.val(&varArray[i]) =
                FElem::from(if srcValue & (1usize << i) != 0 { 1 } else { 0 });
        }
        for j in i..varArray.len() {
            *self.val(&varArray[j]) = FElem::from(0);
        }
    }

    pub fn setDualWordValue(&mut self, dualWord: &DualWord, srcValue: usize) {
        self.setMultipackedWordValue(&dualWord.multipacked(), srcValue);
        self.setValuesAsBitArray(&dualWord.unpacked(), srcValue);
    }

    pub fn setMultipackedWordValue(
        &mut self,
        multipackedWord: &VariableArray<MultiPackedWord>,
        srcValue: usize,
    ) {
        assert!(
            self.fieldType_ == FieldType::R1P,
            "Unknown protoboard type in pub fn setMultipackedWordValue"
        );

        assert!(
            multipackedWord.len() == 1,
            "Multipacked word size mismatch in R1P"
        );
        *self.val(&multipackedWord[0]) = FElem::from(srcValue);
    }

    // The following 3 methods are purposely not overloaded to the same name in order to reduce
    // programmer error. We want the programmer to explicitly code what type of constraint
    // she wants.
    pub fn addRank1Constraint(
        &mut self,
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
        name: &str,
    ) {
        self.constraintSystem_
            .addConstraint1(Rank1Constraint::new(a, b, c, name.to_owned()));
    }

    pub fn addGeneralConstraint(&mut self, a: Polynomial, b: Polynomial, name: String) {
        self.constraintSystem_
            .addConstraint(PolynomialConstraint::new(a, b, name));
    }

    pub fn addUnaryConstraint(&mut self, a: LinearCombination, name: &str) {
        self.addRank1Constraint(a, 1.into(), 0.into(), name);
    }

    pub fn isSatisfied(&self, printOnFail: &PrintOptions) -> bool {
        self.constraintSystem_
            .isSatisfied(&self.assignment_, printOnFail)
    }

    pub fn setFlag(&mut self, flag: &FlagVariable, newFlagState: bool) {
        *self.val(flag) = FElem::from(if newFlagState { 1 } else { 0 });
    }

    pub fn enforceBooleanity(&mut self, var: &Variable) {
        self.addRank1Constraint(
            var.clone().into(),
            var.clone() - 1,
            0.into(),
            &format!("enforceBooleanity({})", var.name()),
        );
    }

    pub fn annotation(&self) -> String {
        let mut retVal = self.constraintSystem_.annotation();
        retVal += "Variable Assignments:\n";
        for assignmentPair in &self.assignment_ {
            let varName = assignmentPair.0.name();
            let varAssignedValue = assignmentPair.1.asString();
            retVal += &format!("{varName} : {varAssignedValue} \n");
        }
        retVal
    }

    pub fn dualWordAssignmentEqualsValue(
        &mut self,
        dualWord: &DualWord,
        expectedValue: usize,
        printOption: &PrintOptions,
    ) -> bool {
        let multipackedEqualsValue = self.multipackedWordAssignmentEqualsValue(
            &dualWord.multipacked(),
            expectedValue,
            printOption,
        );
        let unpackedEqualsValue = self.unpackedWordAssignmentEqualsValue(
            &dualWord.unpacked(),
            expectedValue,
            printOption,
        );
        if multipackedAndUnpackedValuesDisagree(multipackedEqualsValue, unpackedEqualsValue) {
            printInformativeNoticeMessage(multipackedEqualsValue, unpackedEqualsValue);
        }
        return multipackedEqualsValue && unpackedEqualsValue;
    }

    pub fn multipackedWordAssignmentEqualsValue(
        &mut self,
        multipackedWord: &VariableArray<MultiPackedWord>,
        expectedValue: usize,
        printOption: &PrintOptions,
    ) -> bool {
        let mut retval = true;
        assert!(
            self.fieldType_ == FieldType::R1P,
            "Unknown field type in pub fn multipackedWordAssignmentEqualsValue(...)"
        );
        assert!(multipackedWord.len() == 1, "R1P multipacked size mismatch");
        if *self.val(&multipackedWord[0]) == FElem::from(expectedValue) {
            retval = true;
        } else {
            retval = false;
        }
        if expectedToPrintValues(retval, printOption) {
            println!(
                "Expected value for multipacked word \"{}\" is: {}\nActual value is: {}",
                multipackedWord.name(),
                expectedValue,
                *self.val(&multipackedWord[0])
            );
        }

        retval
    }

    pub fn unpackedWordAssignmentEqualsValue(
        &mut self,
        unpackedWord: &VariableArray<UnpackedWord>,
        expectedValue: usize,
        printOption: &PrintOptions,
    ) -> bool {
        let mut retval = true;
        let mut expectedValueCopy = expectedValue;
        for i in 0..unpackedWord.len() {
            if *self.val(&unpackedWord[i]) != FElem::from(expectedValueCopy & 1usize) {
                retval = false;
                break;
            }
            expectedValueCopy >>= 1;
        }
        if expectedValueCopy != 0 {
            retval = false;
        }
        if expectedToPrintValues(retval, printOption) {
            println!(
                "Expected value for unpacked word \"{}\" {expectedValue}",
                unpackedWord.name()
            );
            println!("Actual values are: ");
            for i in 0..unpackedWord.len() {
                println!("bit {i} : {}", self.val(&unpackedWord[i]));
            }
        }
        retval
    }
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// ProtoboardParams::~ProtoboardParams() {}

pub fn multipackedAndUnpackedValuesDisagree(
    multipackedEqualsValue: bool,
    unpackedEqualsValue: bool,
) -> bool {
    multipackedEqualsValue != unpackedEqualsValue
}

pub fn printInformativeNoticeMessage(multipackedEqualsValue: bool, unpackedEqualsValue: bool) {
    if multipackedEqualsValue && !unpackedEqualsValue {
        println!("NOTE: multipacked value equals expected value but unpacked value does not!");
    } else {
        assert!(
            !multipackedEqualsValue && unpackedEqualsValue,
            "printInformativeNoticeMessage(...) has been called incorrectly"
        );
        println!("NOTE: unpacked value equals expected value but multipacked value does not!");
    }
}

pub fn expectedToPrintValues(boolValue: bool, printOption: &PrintOptions) -> bool {
    ((boolValue && printOption == &PrintOptions::DBG_PRINT_IF_TRUE)
        || (!boolValue && printOption == &PrintOptions::DBG_PRINT_IF_FALSE))
}
