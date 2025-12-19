//  Interfaces and basic gadgets for R1P (Rank 1 prime characteristic)
//  constraint systems.

//  These interfaces have been designed to allow later adding other fields or constraint
//  structures while allowing high level design to stay put.

//  A gadget represents (and generates) the constraints, constraint "wiring", and
//  witness for a logical task. This is best explained using the physical design of a printed
//  circuit. The Protoboard is the board on which we will "solder" our circuit. The wires
//  (implemented by Variables) can hold any element of the underlying field. Each constraint
//  enforces a relation between wires. These can be thought of as gates.

//  The delegation of tasks is as follows:

//  -   Constructor - Allocates all Variables to a Protoboard. Creates all sub-gadgets
//      that will be needed and wires their inputs and outputs.
//      generateConstraints - Generates the constraints which define the
//      necessary relations between the previously allocated Variables.

//  -   generateWitness - Generates an assignment for all non-input Variables which is
//      consistent with the assignment of the input Variables and satisfies
//      all of the constraints. In essence, this computes the logical
//      function of the Gadget.

//  -   create - A static factory method used for construction of the Gadget. This is
//      used in order to create a Gadget without explicit knowledge of the
//      underlying algebraic field.

// use crate::gadgetlib2::gadgetMacros;
// use crate::gadgetlib2::protoboard;
// use crate::gadgetlib2::variable;

// namespace gadgetlib2 {

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                         pub struct Gadget                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// /**
//  Gadget class, representing the constraints and witness generation for a logical task.

//  Gadget hierarchy:
//  (Here and elsewhere: R1P = Rank 1 constraints over a prime-characteristic field.)
//  Gadgets have a somewhat cumbersome pub struct hierarchy, for the sake of clean gadget construction.
//  (1) A field agnostic, concrete (as opposed to interface) gadget will derive from Gadget. For
//      instance NAND needs only AND and NOT and does not care about the field, thus it derives from
//      Gadget.
//  (2) Field specific interface pub struct R1P_Gadget derives from Gadget using virtual
//      inheritance, in order to avoid the Dreaded Diamond problem (see
//      http://stackoverflow.com/a/21607/1756254 for more info)
//  (3) Functional interface classes such as LooseMUX_GadgetBase virtually derive from Gadget and
//      define special gadget functionality. For gadgets with no special interfaces we use the macro
//      CREATE_GADGET_BASE_CLASS() for the sake of code consistency (these gadgets can work the same
//      without this base class). This is an interface only and the implementation of AND_Gadget is
//      field specific.
//  (4) These field specific gadgets will have a factory pub struct with static method create, such as
//      AND_Gadget::create(...) in order to agnostically create this gadget for use by a field
//      agnostic gadget.
//  (5) Concrete field dependent gadgets derive via multiple inheritance from two interfaces.
//      e.g. R1P_AND_Gadget derives from both AND_Gadget and R1P_Gadget. This was done to allow usage
//      of AND_Gadget's field agnostic create() method and R1P_Gadget's field specific val() method.
// */

pub struct Gadget {
    // DISALLOW_COPY_AND_ASSIGN(Gadget);
    pb_: ProtoboardPtr,
    // Gadget(pb:ProtoboardPtr);
    // virtual pub fn  init() = 0;
    // /* generate constraints must have this interface, however generateWitness for some gadgets
    //    (like CTime) will take auxiliary information (like memory contents). We do not want to force
    //    the interface for generateWitness but do want to make sure it is never invoked from base
    //    class.
    // */
    // virtual pub fn  generateConstraints() = 0;
    // virtual pub fn  generateWitness(); // Not abstract as this method may have different signatures.
    // pub fn String& name);
    // pub fn  addRank1Constraint(a:LinearCombination,
    //                         b:LinearCombination,
    //                         c:LinearCombination,
    //                         name:&str);
    // pub fn  enforceBooleanity(var:&Variable) {pb_.enforceBooleanity(var);}
    // FElem& val(var:&Variable) {return pb_.val(var);}
    // FElem val(lc:&LinearCombination) {return pb_.val(lc);}
    // FieldType fieldType() const {return pb_.fieldType_;}
    // bool flagIsSet(flag:&FlagVariable) const {return pb_.flagIsSet(flag);}
}

type GadgetPtr = RcCell<Gadget>; // Not a unique_ptr because sometimes we need to cast
// these pointers for specific gadget operations.
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      Gadget Interfaces                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
  We use multiple inheritance in order to use much needed syntactic sugar. We want val() to be
  able to return different types depending on the field so we need to differentiate the interfaces
  between R1P and other fields. We also want the interfaces of specific logical gadgets
  (for instance AND_Gadget which has n inputs and 1 output) in order to construct higher level
  gadgets without specific knowledge of the underlying field. Both interfaces (for instance
  R1P_gadget and AND_Gadget) inherit from Gadget using virtual inheritance (this means only one
  instance of Gadget will be created. For a more thorough discussion on virtual inheritance see
  http://www.phpcompiler.org/articles/virtualinheritance.html
*/

pub struct R1P_Gadget {
    // : virtual public Gadget
    // R1P_Gadget(pb:ProtoboardPtr)->Self Gadget(pb) {}
    // virtual ~R1P_Gadget() = 0;

    // virtual pub fn  addRank1Constraint(a:LinearCombination,
    //                                 b:LinearCombination,
    //                                 c:LinearCombination,
    //                                 name:&str);

    // virtual pub fn  init() = 0; // private in order to force programmer to invoke from a Gadget* only
    // DISALLOW_COPY_AND_ASSIGN(R1P_Gadget);
} // pub struct R1P_Gadget

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     AND_Gadget classes                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(AND_GadgetBase);

/// Specific case for and AND with two inputs. Field agnostic
pub struct BinaryAND_Gadget {
    //AND_GadgetBase

    // BinaryAND_Gadget(pb:ProtoboardPtr,
    //                  input1:LinearCombination,
    //                  input2:LinearCombination,
    //                  result:Variable);
    // pub fn  init();
    // pub fn  generateConstraints();
    // pub fn  generateWitness();

    // friend pub struct AND_Gadget;

    //external variables
    input1_: LinearCombination,
    input2_: LinearCombination,
    result_: Variable,
    // DISALLOW_COPY_AND_ASSIGN(BinaryAND_Gadget);
} // pub struct BinaryAND_Gadget

pub struct R1P_AND_Gadget {
    //AND_GadgetBase, public R1P_Gadget

    // R1P_AND_Gadget(pb:ProtoboardPtr, input:VariableArray, result:Variable);
    // virtual pub fn  init();

    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct AND_Gadget;

    //external variables
    input_: VariableArray,
    result_: Variable,
    //internal variables
    sum_: LinearCombination,
    sumInverse_: Variable,
    // DISALLOW_COPY_AND_ASSIGN(R1P_AND_Gadget);
}

pub struct AND_Gadget {
    // static GadgetPtr create(pb:ProtoboardPtr, input:VariableArray, result:Variable);
    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         input1:LinearCombination,
    //                         input2:LinearCombination,
    //                         result:Variable);

    // DISALLOW_CONSTRUCTION(AND_Gadget);
    // DISALLOW_COPY_AND_ASSIGN(AND_Gadget);
} // pub struct GadgetType

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     OR_Gadget classes                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(OR_GadgetBase);

/// Specific case for and OR with two inputs. Field agnostic
pub struct BinaryOR_Gadget {
    //OR_GadgetBase

    // BinaryOR_Gadget(pb:ProtoboardPtr,
    //                 input1:LinearCombination,
    //                 input2:LinearCombination,
    //                 result:Variable);
    // pub fn  init();
    // pub fn  generateConstraints();
    // pub fn  generateWitness();

    // friend pub struct OR_Gadget;

    //external variables
    input1_: LinearCombination,
    input2_: LinearCombination,
    result_: Variable,
    // DISALLOW_COPY_AND_ASSIGN(BinaryOR_Gadget);
} // pub struct BinaryOR_Gadget

pub struct R1P_OR_Gadget {
    //OR_GadgetBase, public R1P_Gadget
    sum_: LinearCombination,
    sumInverse_: Variable,
    // R1P_OR_Gadget(pb:ProtoboardPtr, input:VariableArray, result:Variable);
    // virtual pub fn  init();
    input_: VariableArray,
    result_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct OR_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_OR_Gadget);
}

pub struct OR_Gadget {
    // static GadgetPtr create(pb:ProtoboardPtr, input:VariableArray, result:Variable);
    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         input1:LinearCombination,
    //                         input2:LinearCombination,
    //                         result:Variable);

    // DISALLOW_CONSTRUCTION(OR_Gadget);
    // DISALLOW_COPY_AND_ASSIGN(OR_Gadget);
} // pub struct GadgetType

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************               InnerProduct_Gadget classes                  ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(InnerProduct_GadgetBase);

pub struct R1P_InnerProduct_Gadget {
    //InnerProduct_GadgetBase, public R1P_Gadget
    partialSums_: VariableArray,
    // R1P_InnerProduct_Gadget(pb:ProtoboardPtr,
    //                         A:VariableArray,
    //                         B:VariableArray,
    //                         result:Variable);
    // virtual pub fn  init();
    A_: VariableArray,
    B_: VariableArray,
    result_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct InnerProduct_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_InnerProduct_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(InnerProduct_Gadget, VariableArray, A,
//    VariableArray, B,
//    Variable, result);

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                LooseMUX_Gadget classes                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
    Loose Multiplexer (MUX):
    Multiplexes one Variable
    index not in bounds -> success_flag = 0
    index in bounds && success_flag = 1 -> result is correct
    index is in bounds, we can also set success_flag to 0 -> result will be forced to 0
*/

pub struct LooseMUX_GadgetBase {
    // : virtual public Gadget
    // LooseMUX_GadgetBase(pb:ProtoboardPtr)->Self Gadget(pb) {}

    // virtual ~LooseMUX_GadgetBase() = 0;
    // virtual VariableArray indicatorVariables() 0:=,

    // virtual pub fn  init() = 0;
    // DISALLOW_COPY_AND_ASSIGN(LooseMUX_GadgetBase);
} // pub struct LooseMUX_GadgetBase

pub struct R1P_LooseMUX_Gadget {
    //LooseMUX_GadgetBase, public R1P_Gadget
    indicators_: VariableArray,
    computeResult_: Vec<GadgetPtr>, // Inner product gadgets
    // R1P_LooseMUX_Gadget(pb:ProtoboardPtr,
    //                     inputs:MultiPackedWordArray,
    //                     index:Variable,
    //                     output:VariableArray,
    //                     successFlag:&Variable);
    // virtual pub fn  init();
    inputs_: MultiPackedWordArray,
    index_: Variable,
    output_: VariableArray,
    successFlag_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // virtual VariableArray indicatorVariables() const;
    // friend pub struct LooseMUX_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_LooseMUX_Gadget);
}

pub struct LooseMUX_Gadget {
    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         inputs:MultiPackedWordArray,
    //                         index:Variable,
    //                         output:VariableArray,
    //                         successFlag:&Variable);
    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         inputs:VariableArray,
    //                         index:Variable,
    //                         output:Variable,
    //                         successFlag:&Variable);

    // DISALLOW_CONSTRUCTION(LooseMUX_Gadget);
    // DISALLOW_COPY_AND_ASSIGN(LooseMUX_Gadget);
} // pub struct GadgetType

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************            CompressionPacking_Gadget classes               ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
// TODO change pub struct name to bitpacking
enum PackingMode {
    PACK,
    UNPACK,
}

// CREATE_GADGET_BASE_CLASS(CompressionPacking_GadgetBase);

pub struct R1P_CompressionPacking_Gadget {
    //CompressionPacking_GadgetBase, public R1P_Gadget
    packingMode_: PackingMode,
    // R1P_CompressionPacking_Gadget(pb:ProtoboardPtr,
    //                               unpacked:VariableArray,
    //                               packed:VariableArray,
    //                               packingMode:PackingMode);
    // virtual pub fn  init();
    unpacked_: VariableArray,
    packed_: VariableArray,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct CompressionPacking_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_CompressionPacking_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(CompressionPacking_Gadget, VariableArray, unpacked, VariableArray,
//                               packed, PackingMode, packingMode);

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************            IntegerPacking_Gadget classes                ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(IntegerPacking_GadgetBase);

// In R1P compression and arithmetic packing are implemented the same, hence this gadget simply
// instantiates an R1P_CompressionPacking_Gadget
pub struct R1P_IntegerPacking_Gadget {
    //IntegerPacking_GadgetBase, public R1P_Gadget
    packingMode_: PackingMode,
    compressionPackingGadget_: GadgetPtr,
    // R1P_IntegerPacking_Gadget(pb:ProtoboardPtr,
    //                           unpacked:VariableArray,
    //                           packed:VariableArray,
    //                           packingMode:PackingMode);
    // virtual pub fn  init();
    unpacked_: VariableArray,
    packed_: VariableArray,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct IntegerPacking_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_IntegerPacking_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(IntegerPacking_Gadget, VariableArray, unpacked, VariableArray,
//                               packed, PackingMode, packingMode);

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 EqualsConst_Gadget classes                 ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
    Gadgets recieve a constant field element n, and an input.
    input == n ==> result = 1
    input != n ==> result = 0
*/

// TODO change to take LinearCombination as input and change AND/OR to use this
// CREATE_GADGET_BASE_CLASS(EqualsConst_GadgetBase);

pub struct R1P_EqualsConst_Gadget {
    //EqualsConst_GadgetBase, public R1P_Gadget
    n_: FElem,
    aux_: Variable,
    // R1P_EqualsConst_Gadget(pb:ProtoboardPtr,
    //                        n:FElem,
    //                        input:LinearCombination,
    //                        result:Variable);
    // virtual pub fn  init();
    input_: LinearCombination,
    result_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct EqualsConst_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_EqualsConst_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(EqualsConst_Gadget, FElem, n, LinearCombination, input,
//                               Variable, result);

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   DualWord_Gadget                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
//TODO add test

pub struct DualWord_Gadget {
    //Gadget
    var_: DualWord,
    packingMode_: PackingMode,

    packingGadget_: GadgetPtr,
    // DualWord_Gadget(pb:ProtoboardPtr, var:DualWord, packingMode:PackingMode);
    // virtual pub fn  init();
    // DISALLOW_COPY_AND_ASSIGN(DualWord_Gadget);

    // static GadgetPtr create(pb:ProtoboardPtr, var:DualWord, packingMode:PackingMode);
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 DualWordArray_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
//TODO add test

pub struct DualWordArray_Gadget {
    //Gadget
    vars_: DualWordArray,
    packingMode_: PackingMode,

    packingGadgets_: Vec<GadgetPtr>,
    // DualWordArray_Gadget(pb:ProtoboardPtr,
    //                          vars:DualWordArray,
    //                          packingMode:PackingMode);
    // virtual pub fn  init();
    // DISALLOW_COPY_AND_ASSIGN(DualWordArray_Gadget);

    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         vars:DualWordArray,
    //                         packingMode:PackingMode);
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        Toggle_Gadget                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

//TODO add test

/// A gadget for the following semantics:
/// If toggle is 0, zeroValue --> result
/// If toggle is 1, oneValue --> result
/// Uses 1 constraint

pub struct Toggle_Gadget {
    //Gadget
    toggle_: FlagVariable,
    zeroValue_: LinearCombination,
    oneValue_: LinearCombination,
    result_: Variable,
    // Toggle_Gadget(pb:ProtoboardPtr,
    //               toggle:FlagVariable,
    //               zeroValue:LinearCombination,
    //               oneValue:LinearCombination,
    //               result:Variable);

    // virtual pub fn  init() {}
    // DISALLOW_COPY_AND_ASSIGN(Toggle_Gadget);

    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         toggle:FlagVariable,
    //                         zeroValue:LinearCombination,
    //                         oneValue:LinearCombination,
    //                         result:Variable);

    // pub fn  generateConstraints();
    // pub fn  generateWitness();
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   ConditionalFlag_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/// A gadget for the following semantics:
/// condition != 0  --> flag = 1
/// condition == 0 --> flag = 0
/// Uses 2 constraints

pub struct ConditionalFlag_Gadget {
    //Gadget
    flag_: FlagVariable,
    condition_: LinearCombination,
    auxConditionInverse_: Variable,
    //     ConditionalFlag_Gadget(pb:ProtoboardPtr,
    //                            condition:LinearCombination,
    //                            flag:&FlagVariable);

    //     virtual pub fn  init() {}
    //     DISALLOW_COPY_AND_ASSIGN(ConditionalFlag_Gadget);

    //     static GadgetPtr create(pb:ProtoboardPtr,
    //                             condition:LinearCombination,
    //                             flag:&FlagVariable);

    //     pub fn  generateConstraints();
    //     pub fn  generateWitness();
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  LogicImplication_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/// A gadget for the following semantics:
/// condition == 1 --> flag = 1
/// Uses 1 constraint

pub struct LogicImplication_Gadget {
    //Gadget
    flag_: FlagVariable,
    condition_: LinearCombination,
    // LogicImplication_Gadget(pb:ProtoboardPtr,
    //                         condition:LinearCombination,
    //                         flag:&FlagVariable);

    // virtual pub fn  init() {}
    // DISALLOW_COPY_AND_ASSIGN(LogicImplication_Gadget);

    // static GadgetPtr create(pb:ProtoboardPtr,
    //                         condition:LinearCombination,
    //                         flag:&FlagVariable);

    // pub fn  generateConstraints();
    // pub fn  generateWitness();
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        Compare_Gadget                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// TODO create unit test
// CREATE_GADGET_BASE_CLASS(Comparison_GadgetBase);

pub struct R1P_Comparison_Gadget {
    //Comparison_GadgetBase, public R1P_Gadget
    wordBitSize_: usize,
    lhs_: PackedWord,
    rhs_: PackedWord,
    less_: FlagVariable,
    lessOrEqual_: FlagVariable,
    alpha_p_: PackedWord,
    alpha_u_: UnpackedWord,
    notAllZeroes_: FlagVariable,
    allZeroesTest_: GadgetPtr,
    alphaDualVariablePacker_: GadgetPtr,
    // R1P_Comparison_Gadget(pb:ProtoboardPtr,
    //                       wordBitSize:usize,
    //                       lhs:PackedWord,
    //                       rhs:PackedWord,
    //                       less:FlagVariable,
    //                       lessOrEqual:&FlagVariable);
    // virtual pub fn  init();

    // static GadgetPtr create(pb:ProtoboardPtr,
    // 						wordBitSize:usize,
    // 						lhs:PackedWord,
    // 						rhs:PackedWord,
    // 						less:FlagVariable,
    // 						lessOrEqual:&FlagVariable);

    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct Comparison_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_Comparison_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_5(Comparison_Gadget, // TODO uncomment this
//                               usize, wordBitSize,
//                               PackedWord, lhs,
//                               PackedWord, rhs,
//                               FlagVariable, less,
//                               FlagVariable, lessOrEqual);

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

// } // namespace gadgetlib2

//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_GADGET_HPP_
/** @file
*****************************************************************************
Declarations of the interfaces and basic gadgets for R1P (Rank 1 prime characteristic)
constraint systems.

See details in gadget.hpp .
*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// use  <cmath>

// use crate::gadgetlib2::gadget;

// using ::RcCell;
// using ::String;
// using ::Vec;
// using ::std::cout;
// using ::std::cerr;
// using ::std::endl;

// namespace gadgetlib2 {

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      Gadget Interfaces                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/***********************************/
/***          Gadget             ***/
/***********************************/
impl Gadget {
    pub fn new(pb: ProtoboardPtr) -> Self {
        //  pb_(pb)
        assert!(
            pb != NULL,
            "Attempted to create gadget with uninitialized Protoboard."
        );
    }

    pub fn generateWitness() {
        GADGETLIB_FATAL("Attempted to generate witness for an incomplete Gadget type.");
    }

    pub fn addUnaryConstraint(a: LinearCombination, name: &str) {
        pb_.addUnaryConstraint(a, name);
    }

    pub fn addRank1Constraint(
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
        name: &str,
    ) {
        pb_.addRank1Constraint(a, b, c, name);
    }
}
/***********************************/
/***        R1P_Gadget           ***/
/***********************************/

impl R1P_Gadget {
    // R1P_Gadget::~R1P_Gadget() {}

    pub fn addRank1Constraint(
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
        name: &string,
    ) {
        pb_.addRank1Constraint(a, b, c, name);
    }
}
/***********************************/
/***  End of Gadget Interfaces   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      AND Gadgets                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
// impl AND_GadgetBase{
// AND_GadgetBase::~AND_GadgetBase() {}
// }

/*
    Constraint breakdown:
    (1) input1 * input2 = result
*/
impl BinaryAND_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> Self {
        // : Gadget(pb), AND_GadgetBase(pb), input1_(input1), input2_(input2), result_(result)
    }

    pub fn init() {}

    pub fn generateConstraints() {
        addRank1Constraint(input1_, input2_, result_, "result = AND(input1, input2)");
    }

    pub fn generateWitness() {
        if val(input1_) == 1 && val(input2_) == 1 {
            val(result_) = 1;
        } else {
            val(result_) = 0;
        }
    }
}

/*
    Constraint breakdown:

    (*) sum = sum(input[i]) - n
    (1) sum * result = 0
    (2) sum * sumInverse = 1 - result

    [ AND(inputs) == 1 ] (*)==> [sum == 0] (2)==> [result == 1]
    [ AND(inputs) == 0 ] (*)==> [sum != 0] (1)==> [result == 0]
*/
impl R1P_AND_Gadget {
    pub fn new(pb: ProtoboardPtr, input: &VariableArray, result: Variable) -> Self {
        // : Gadget(pb), AND_GadgetBase(pb), R1P_Gadget(pb), input_(input), result_(result),
        //       sumInverse_("sumInverse")
        // assert!(input.len() > 0, "Attempted to create an R1P_AND_Gadget with 0 inputs.");
        // assert!(input.len() <= Fp(-1).as_ulong(), "Attempted to create R1P_AND_Gadget with too "
        //                                                           "many inputs. Will cause overflow!");
    }

    pub fn init() {
        let numInputs = input_.len();
        sum_ = sum(input_) - numInputs;
    }

    pub fn generateConstraints() {
        addRank1Constraint(
            sum_,
            result_,
            0,
            "sum * result = 0 | sum == sum(input[i]) - n",
        );
        addRank1Constraint(
            sumInverse_,
            sum_,
            1 - result_,
            "sumInverse * sum = 1-result | sum == sum(input[i]) - n",
        );
    }

    pub fn generateWitness() {
        let mut sum = 0; //FElem
        for i in 0..input_.len() {
            sum += val(input_[i]);
        }
        sum -= input_.len(); // sum(input[i]) - n ==> sum
        if sum == 0 {
            // AND(input[0], input[1], ... == 1
            val(sumInverse_) = 0;
            val(result_) = 1;
        } else {
            // AND(input[0], input[1], ...) == 0
            val(sumInverse_) = sum.inverse(R1P);
            val(result_) = 0;
        }
    }
}
impl AND_Gadget {
    pub fn create(pb: ProtoboardPtr, input: VariableArray, result: Variable) -> GadgetPtr {
        let mut pGadget = GadgetPtr::default();
        if pb.fieldType_ == R1P {
            pGadget = RcCell::new(R1P_AND_Gadget::new(&pb, input, result));
        } else {
            GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
        }
        pGadget.init();
        return pGadget;
    }

    pub fn create(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(BinaryAND_Gadget::new(&pb, input1, input2, result));
        pGadget.init();
        return pGadget;
    }
}

/***********************************/
/***     End of AND Gadgets      ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      OR Gadgets                            ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// OR_GadgetBase::~OR_GadgetBase() {}

/*
    Constraint breakdown:
    (1) result = input1 + input2 - input1 * input2
        input1 * input2 = input1 + input2 - result
*/
impl BinaryOR_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> Self {
        //  : Gadget(pb), OR_GadgetBase(pb), input1_(input1), input2_(input2), result_(result)
    }

    pub fn init() {}

    pub fn generateConstraints() {
        addRank1Constraint(
            input1_,
            input2_,
            input1_ + input2_ - result_,
            "result = OR(input1, input2)",
        );
    }

    pub fn generateWitness() {
        if val(input1_) == 1 || val(input2_) == 1 {
            val(result_) = 1;
        } else {
            val(result_) = 0;
        }
    }
}
/*
    Constraint breakdown:

    (*) sum = sum(input[i])
    (1) sum * (1 - result) = 0
    (2) sum * sumInverse = result

    [ OR(inputs) == 1 ] (*)==> [sum != 0] (1)==> [result == 1]
    [ OR(inputs) == 0 ] (*)==> [sum == 0] (2)==> [result == 0]
*/
impl R1P_OR_Gadget {
    pub fn new(pb: ProtoboardPtr, input: &VariableArray, result: Variable) {
        // : Gadget(pb), OR_GadgetBase(pb), R1P_Gadget(pb), sumInverse_("sumInverse"), input_(input),
        //           result_(result)
        // assert!(input.len() > 0, "Attempted to create an R1P_OR_Gadget with 0 inputs.");
        // assert!(input.len() <= Fp(-1).as_ulong(), "Attempted to create R1P_OR_Gadget with too "
        //                                                           "many inputs. Will cause overflow!");
    }

    pub fn init() {
        sum_ = sum(input_);
    }

    pub fn generateConstraints() {
        addRank1Constraint(
            sum_,
            1 - result_,
            0,
            "sum * (1 - result) = 0 | sum == sum(input[i])",
        );
        addRank1Constraint(
            sumInverse_,
            sum_,
            result_,
            "sum * sumInverse = result | sum == sum(input[i])",
        );
    }

    pub fn generateWitness() {
        let mut sum = 0; //FElem
        for i in 0..input_.len() {
            // sum(input[i]) ==> sum
            sum += val(input_[i]);
        }
        if sum == 0 {
            // OR(input[0], input[1], ... == 0
            val(sumInverse_) = 0;
            val(result_) = 0;
        } else {
            // OR(input[0], input[1], ...) == 1
            val(sumInverse_) = sum.inverse(R1P);
            val(result_) = 1;
        }
    }
}

impl OR_Gadget {
    pub fn create(pb: ProtoboardPtr, input: VariableArray, result: Variable) -> GadgetPtr {
        let mut pGadget = GadgetPtr::default();
        if pb.fieldType_ == R1P {
            pGadget = RcCell::new(R1P_OR_Gadget::new(&pb, input, result));
        } else {
            GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
        }
        pGadget.init();
        return pGadget;
    }

    pub fn create(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(BinaryOR_Gadget::new(&pb, input1, input2, result));
        pGadget.init();
        return pGadget;
    }
}
/***********************************/
/***     End of OR Gadgets       ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 InnerProduct Gadgets                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// InnerProduct_GadgetBase::~InnerProduct_GadgetBase() {}

/*
    Constraint breakdown:

    (1) partialSums[0] = A[0] * B[0]
    (2) partialSums[i] = partialSums[i-1] + A[0] * B[0] ==>                     i = 1..n-2
        partialSums[i] - partialSums[i-1] = A[i] * B[i]
    (3) result = partialSums[n-1] = partialSums[n-2] + A[n-1] * B[n-1] ==>
        result - partialSums[n-2] = A[n-1] * B[n-1]

*/

impl R1P_InnerProduct_Gadget {
    pub fn new(pb: ProtoboardPtr, A: VariableArray, B: VariableArray, result: Variable) -> Self {
        // : Gadget(pb), InnerProduct_GadgetBase(pb), R1P_Gadget(pb), partialSums_(A.len(),
        //           "partialSums"), A_(A), B_(B), result_(result)
        // assert!(A.len() > 0, "Attempted to create an R1P_InnerProduct_Gadget with 0 inputs.");
        // assert!(A.len() == B.len(), GADGETLIB2_FMT("Inner product vector sizes not equal. Sizes are: "
        //                                                     "(A) - %u, (B) - %u", A.len(), B.len()));
    }

    pub fn init() {}

    pub fn generateConstraints() {
        let n = A_.len();
        if n == 1 {
            addRank1Constraint(A_[0], B_[0], result_, "A[0] * B[0] = result");
            return;
        }
        // else (n > 1)
        addRank1Constraint(
            A_[0],
            B_[0],
            partialSums_[0],
            "A[0] * B[0] = partialSums[0]",
        );
        for i in 1..=n - 2 {
            addRank1Constraint(
                A_[i],
                B_[i],
                partialSums_[i] - partialSums_[i - 1],
                GADGETLIB2_FMT(
                    "A[%u] * B[%u] = partialSums[%u] - partialSums[%u]",
                    i,
                    i,
                    i,
                    i - 1,
                ),
            );
        }
        addRank1Constraint(
            A_[n - 1],
            B_[n - 1],
            result_ - partialSums_[n - 2],
            "A[n-1] * B[n-1] = result - partialSums[n-2]",
        );
    }

    pub fn generateWitness() {
        let n = A_.len();
        if n == 1 {
            val(result_) = val(A_[0]) * val(B_[0]);
            return;
        }
        // else (n > 1)
        val(partialSums_[0]) = val(A_[0]) * val(B_[0]);
        for i in 1..=n - 2 {
            val(partialSums_[i]) = val(partialSums_[i - 1]) + val(A_[i]) * val(B_[i]);
        }
        val(result_) = val(partialSums_[n - 2]) + val(A_[n - 1]) * val(B_[n - 1]);
    }
}

/***********************************/
/*** End of InnerProduct Gadgets ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   LooseMUX Gadgets                         ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// LooseMUX_GadgetBase::~LooseMUX_GadgetBase() {}

/*
    Constraint breakdown:
    (1) indicators[i] * (index - i) = 0  | i = 0..n-1 ==> only indicators[index] will be non-zero
    (2) sum(indicators[i]) = successFlag ==> successFlag = indicators[index]
    (3) successFlag is boolean
    (4) result[j] = <indicators> * <inputs[index][j]>  |  j = 1..output.len()   ==>
        result[j] = inputs[index][j]

*/
impl R1P_LooseMUX_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        inputs: MultiPackedWordArray,
        index: Variable,
        output: VariableArray,
        successFlag: &Variable,
    ) {
        // : Gadget(pb), LooseMUX_GadgetBase(pb), R1P_Gadget(pb),
        //           indicators_(inputs.len(), "indicators"), inputs_(inputs.len()), index_(index),
        //           output_(output), successFlag_(successFlag)

        assert!(inputs.len() <= Fp(-1).as_ulong(), "Attempted to create R1P_LooseMUX_Gadget "
                                                      "with too many inputs. May cause overflow!");
        //    for inpArr  inputs) {
        for i in 0..inputs.len() {
            assert!(
                inputs[i].len() == output.len(),
                "Input VariableArray is of incorrect size."
            );
        }
        // ::std::copy(inputs.begin(), inputs.end(), inputs_.begin()); // change type to R1P_VariableArray
        inputs_ = inputs;
    }

    pub fn init() {
        // create inputs for the inner products and initialize them. Each iteration creates a
        // VariableArray for the i'th elements from each of the vector's VariableArrays.
        for i in 0..output_.len() {
            let mut curInput = VariableArray::default();
            for j in 0..inputs_.len() {
                curInput.push(inputs_[j][i]);
            }
            computeResult_.push(InnerProduct_Gadget::create(
                pb_,
                indicators_,
                curInput,
                output_[i],
            ));
        }
    }

    pub fn generateConstraints() {
        let n = inputs_.len();
        for i in 0..n {
            addRank1Constraint(
                indicators_[i],
                (index_ - i),
                0,
                GADGETLIB2_FMT("indicators[%u] * (index - %u) = 0", i, i),
            );
        }
        addRank1Constraint(
            sum(indicators_),
            1,
            successFlag_,
            "sum(indicators) * 1 = successFlag",
        );
        enforceBooleanity(successFlag_);
        for curGadget in computeResult_ {
            curGadget.generateConstraints();
        }
    }

    pub fn generateWitness() {
        let n = inputs_.len();
        /* assumes that idx can be fit in ulong; true for our purposes for now */
        let index = val(index_).asLong();
        let arraySize = n;
        for i in 0..n {
            val(indicators_[i]) = 0; // Redundant, but just in case.
        }
        if index >= n {
            //  || index < 0
            val(successFlag_) = 0;
        } else {
            // index in bounds
            val(indicators_[index]) = 1;
            val(successFlag_) = 1;
        }
        for curGadget in computeResult_ {
            curGadget.generateWitness();
        }
    }

    pub fn indicatorVariables() -> VariableArray {
        return indicators_;
    }
}

impl LooseMUX_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        inputs: MultiPackedWordArray,
        index: Variable,
        output: VariableArray,
        successFlag: &Variable,
    ) -> GadgetPtr {
        let mut pGadget = GadgetPtr::default();
        if pb.fieldType_ == R1P {
            pGadget = RcCell::new(R1P_LooseMUX_Gadget::new(
                &pb,
                inputs,
                index,
                output,
                successFlag,
            ));
        } else {
            GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
        }
        pGadget.init();
        return pGadget;
    }

    /**
        An overload for the private case in which we only want to multiplex one Variable. This is
        usually the case in R1P.
    **/
    pub fn create(
        pb: ProtoboardPtr,
        inputs: VariableArray,
        index: Variable,
        output: Variable,
        successFlag: &Variable,
    ) -> GadgetPtr {
        let mut inpVec = MultiPackedWordArray::default();
        for i in 0..inputs.len() {
            let mut cur = MultiPackedWord::new(pb.fieldType_);
            cur.push(inputs[i]);
            inpVec.push(cur);
        }
        let mut outVec = VariableArray::default();
        outVec.push(output);
        let result = LooseMUX_Gadget::create(&pb, inpVec, index, outVec, successFlag);
        return result;
    }
}
/***********************************/
/***   End of LooseMUX Gadgets   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************               CompressionPacking Gadgets                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
    Compression Packing gadgets have two modes, which differ in the way the witness and constraints
    are created. In PACK mode  gerateWitness() will take the bits and create a packed element (or
    number of elements) while generateConstraints() will not enforce that bits are indeed Boolean.
    In UNPACK mode generateWitness() will take the packed representation and unpack it to bits while
    generateConstraints will in addition enforce that unpacked bits are indeed Boolean.
*/

// CompressionPacking_GadgetBase::~CompressionPacking_GadgetBase() {}

/*
    Constraint breakdown:

    (1) packed = sum(unpacked[i] * 2^i)
    (2) (UNPACK only) unpacked[i] is Boolean.
*/
impl R1P_CompressionPacking_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        unpacked: VariableArray,
        packed: VariableArray,
        packingMode: PackingMode,
    ) -> Self {
        // : Gadget(pb), CompressionPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
        //       unpacked_(unpacked), packed_(packed)
        let n = unpacked.len();
        assert!(n > 0, "Attempted to pack 0 bits in R1P.");
        assert!(
            packed.len() == 1,
            "Attempted to pack into more than 1 Variable in R1P_CompressionPacking_Gadget."
        )
        // TODO add assertion that 'n' bits can fit in the field characteristic
    }

    pub fn init() {}

    pub fn generateConstraints() {
        let n = unpacked_.len();
        let mut packed = LinearCombination::default();
        let mut two_i = FElem::new(R1P_Elem(1)); // Will hold 2^i
        for i in 0..n {
            packed += unpacked_[i] * two_i;
            two_i += two_i;
            if packingMode_ == PackingMode::UNPACK {
                enforceBooleanity(unpacked_[i]);
            }
        }
        addRank1Constraint(packed_[0], 1, packed, "packed[0] = sum(2^i * unpacked[i])");
    }

    pub fn generateWitness() {
        let n = unpacked_.len();
        if packingMode_ == PackingMode::PACK {
            let mut packedVal = 0; //FElem
            let mut two_i = FElem::new(R1P_Elem(1)); // will hold 2^i
            for i in 0..n {
                assert!(
                    val(unpacked_[i]).asLong() == 0 || val(unpacked_[i]).asLong() == 1,
                    GADGETLIB2_FMT(
                        "unpacked[%u]  = %u. Expected a Boolean value.",
                        i,
                        val(unpacked_[i]).asLong()
                    )
                );
                packedVal += two_i * val(unpacked_[i]).asLong();
                two_i += two_i;
            }
            val(packed_[0]) = packedVal;
            return;
        }
        // else (UNPACK)
        assert!(
            packingMode_ == PackingMode::UNPACK,
            "Packing gadget created with unknown packing mode."
        );
        for i in 0..n {
            val(unpacked_[i]) = val(packed_[0]).getBit(i, R1P);
        }
    }
}
/*****************************************/
/*** End of CompressionPacking Gadgets ***/
/*****************************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                IntegerPacking Gadgets                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
    Arithmetic Packing gadgets have two modes, which differ in the way the witness and constraints
    are created. In PACK mode  gerateWitness() will take the bits and create a packed element (or
    number of elements) while generateConstraints() will not enforce that bits are indeed Boolean.
    In UNPACK mode generateWitness() will take the packed representation and unpack it to bits while
    generateConstraints will in addition enforce that unpacked bits are indeed Boolean.
*/

// IntegerPacking_GadgetBase::~IntegerPacking_GadgetBase() {}

/*
    Constraint breakdown:

    (1) packed = sum(unpacked[i] * 2^i)
    (2) (UNPACK only) unpacked[i] is Boolean.
*/
impl R1P_IntegerPacking_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        unpacked: VariableArray,
        packed: VariableArray,
        packingMode: PackingMode,
    ) -> Self {
        // : Gadget(pb), IntegerPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
        //   unpacked_(unpacked), packed_(packed)
        let n = unpacked.len();
        assert!(n > 0, "Attempted to pack 0 bits in R1P.");
        assert!(
            packed.len() == 1,
            "Attempted to pack into more than 1 Variable in R1P_IntegerPacking_Gadget."
        );
    }

    pub fn init() {
        compressionPackingGadget_ =
            CompressionPacking_Gadget::create(pb_, unpacked_, packed_, packingMode_);
    }

    pub fn generateConstraints() {
        compressionPackingGadget_.generateConstraints();
    }

    pub fn generateWitness() {
        compressionPackingGadget_.generateWitness();
    }
}
/*****************************************/
/*** End of IntegerPacking Gadgets  ***/
/*****************************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 EqualsConst Gadgets                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// EqualsConst_GadgetBase::~EqualsConst_GadgetBase() {}

/*
    Constraint breakdown:

    (1) (input - n) * result = 0
    (2) (input - n) * aux = 1 - result

    [ input == n ] (2)==> [result == 1]    (aux can ake any value)
    [ input != n ] (1)==> [result == 0]    (aux == inverse(input - n))
*/
impl R1P_EqualsConst_Gadget {
    pub fn new(pb: ProtoboardPtr, n: FElem, input: &LinearCombination, result: Variable) -> Self {
        // : Gadget(pb), EqualsConst_GadgetBase(pb), R1P_Gadget(pb), n_(n),
        //           aux_("aux (R1P_EqualsConst_Gadget)"), input_(input), result_(result)
    }

    pub fn init() {}

    pub fn generateConstraints() {
        addRank1Constraint(input_ - n_, result_, 0, "(input - n) * result = 0");
        addRank1Constraint(
            input_ - n_,
            aux_,
            1 - result_,
            "(input - n) * aux = 1 - result",
        );
    }

    pub fn generateWitness() {
        val(aux_) = if val(input_) == n_ {
            0
        } else {
            (val(input_) - n_).inverse(R1P)
        };
        val(result_) = if val(input_) == n_ { 1 } else { 0 };
    }
}
/***********************************/
/*** End of EqualsConst Gadgets  ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   DualWord_Gadget                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

impl DualWord_Gadget {
    pub fn new(pb: ProtoboardPtr, var: DualWord, packingMode: PackingMode) -> Self {
        // : Gadget(pb), var_(var), packingMode_(packingMode), packingGadget_()
    }

    pub fn init() {
        packingGadget_ = CompressionPacking_Gadget::create(
            pb_,
            var_.unpacked(),
            var_.multipacked(),
            packingMode_,
        );
    }

    pub fn create(pb: ProtoboardPtr, var: DualWord, packingMode: PackingMode) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(DualWord_Gadget::new(&pb, var, packingMode));
        pGadget.init();
        return pGadget;
    }

    pub fn generateConstraints() {
        packingGadget_.generateConstraints();
    }

    pub fn generateWitness() {
        packingGadget_.generateWitness();
    }
}
/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 DualWordArray_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

impl DualWordArray_Gadget {
    pub fn new(pb: ProtoboardPtr, vars: DualWordArray, packingMode: PackingMode) -> Self {
        //  : Gadget(pb), vars_(vars), packingMode_(packingMode), packingGadgets_()
    }

    pub fn init() {
        let unpacked = vars_.unpacked();
        let packed = vars_.multipacked();
        for i in 0..vars_.len() {
            let curGadget =
                CompressionPacking_Gadget::create(pb_, unpacked[i], packed[i], packingMode_);
            packingGadgets_.push(curGadget);
        }
    }

    pub fn create(pb: ProtoboardPtr, vars: DualWordArray, packingMode: PackingMode) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(DualWordArray_Gadget::new(&pb, vars, packingMode));
        pGadget.init();
        return pGadget;
    }

    pub fn generateConstraints() {
        for gadget in packingGadgets_ {
            gadget.generateConstraints();
        }
    }

    pub fn generateWitness() {
        for gadget in packingGadgets_ {
            gadget.generateWitness();
        }
    }
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        Toggle_Gadget                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
    Constraint breakdown:

    (1) result = (1 - toggle) * zeroValue + toggle * oneValue
        (rank 1 format) ==> toggle * (oneValue - zeroValue) = result - zeroValue

*/
impl Toggle_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        toggle: FlagVariable,
        zeroValue: LinearCombination,
        oneValue: LinearCombination,
        result: Variable,
    ) -> Self {
        // : Gadget(pb), toggle_(toggle), zeroValue_(zeroValue), oneValue_(oneValue),
        //           result_(result)
    }

    pub fn create(
        pb: ProtoboardPtr,
        toggle: FlagVariable,
        zeroValue: LinearCombination,
        oneValue: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget =
            GadgetPtr::new(Toggle_Gadget::new(&pb, toggle, zeroValue, oneValue, result));
        pGadget.init();
        return pGadget;
    }

    pub fn generateConstraints() {
        pb_.addRank1Constraint(
            toggle_,
            oneValue_ - zeroValue_,
            result_ - zeroValue_,
            "result = (1 - toggle) * zeroValue + toggle * oneValue",
        );
    }

    pub fn generateWitness() {
        if val(toggle_) == 0 {
            val(result_) = val(zeroValue_);
        } else if val(toggle_) == 1 {
            val(result_) = val(oneValue_);
        } else {
            GADGETLIB_FATAL("Toggle value must be Boolean.");
        }
    }
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   ConditionalFlag_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
   semantics: condition != 0 --> flag = 1
              condition == 0 --> flag = 0

   Constraint breakdown:
   (1) condition * not(flag) = 0
   (2) condition * auxConditionInverse = flag

*/
impl ConditionalFlag_Gadget {
    pub fn new(pb: ProtoboardPtr, condition: LinearCombination, flag: &FlagVariable) -> Self {
        //  : Gadget(pb), flag_(flag), condition_(condition),
        //           auxConditionInverse_("ConditionalFlag_Gadget::auxConditionInverse_")
    }

    pub fn create(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: &FlagVariable,
    ) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(ConditionalFlag_Gadget::new(&pb, condition, flag));
        pGadget.init();
        return pGadget;
    }

    pub fn generateConstraints() {
        pb_.addRank1Constraint(condition_, negate(flag_), 0, "condition * not(flag) = 0");
        pb_.addRank1Constraint(
            condition_,
            auxConditionInverse_,
            flag_,
            "condition * auxConditionInverse = flag",
        );
    }

    pub fn generateWitness() {
        if val(condition_) == 0 {
            val(flag_) = 0;
            val(auxConditionInverse_) = 0;
        } else {
            val(flag_) = 1;
            val(auxConditionInverse_) = val(condition_).inverse(fieldType());
        }
    }
}
/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  LogicImplication_Gadget                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/*
   semantics: condition == 1 --> flag = 1

   Constraint breakdown:
   (1) condition * (1 - flag) = 0

*/

impl LogicImplication_Gadget {
    pub fn new(pb: ProtoboardPtr, condition: LinearCombination, flag: &FlagVariable) -> Self {
        //  : Gadget(pb), flag_(flag), condition_(condition)
    }

    pub fn create(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: &FlagVariable,
    ) -> GadgetPtr {
        let mut pGadget = GadgetPtr::new(LogicImplication_Gadget::new(&pb, condition, flag));
        pGadget.init();
        return pGadget;
    }

    pub fn generateConstraints() {
        pb_.addRank1Constraint(condition_, negate(flag_), 0, "condition * not(flag) = 0");
    }

    pub fn generateWitness() {
        if val(condition_) == 1 {
            val(flag_) = 1;
        }
    }
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        Compare_Gadget                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// Comparison_GadgetBase::~Comparison_GadgetBase() {}
impl R1P_Comparison_Gadget {
    pub fn new(
        pb: ProtoboardPtr,
        wordBitSize: usize,
        lhs: PackedWord,
        rhs: PackedWord,
        less: FlagVariable,
        lessOrEqual: &FlagVariable,
    ) -> Self {
        // : Gadget(pb), Comparison_GadgetBase(pb), R1P_Gadget(pb), wordBitSize_(wordBitSize),
        //           lhs_(lhs), rhs_(rhs), less_(less), lessOrEqual_(lessOrEqual),
        //           alpha_u_(wordBitSize,  "alpha"), notAllZeroes_("notAllZeroes")
    }

    pub fn init() {
        allZeroesTest_ = OR_Gadget::create(pb_, alpha_u_, notAllZeroes_);
        alpha_u_.push(lessOrEqual_);
        alphaDualVariablePacker_ = CompressionPacking_Gadget::create(
            pb_,
            alpha_u_,
            VariableArray(1, alpha_p_),
            PackingMode::UNPACK,
        );
    }

    /*
        Constraint breakdown:

        for succinctness we shall define:
        (1) wordBitSize == n
        (2) lhs == A
        (3) rhs == B

        packed(alpha) = 2^n + B - A
        not_all_zeros = OR(alpha.unpacked)

        if B - A > 0, then: alpha > 2^n,
        so alpha[n] = 1 and notAllZeroes = 1
        if B - A = 0, then: alpha = 2^n,
        so alpha[n] = 1 and notAllZeroes = 0
        if B - A < 0, then: 0 <= alpha <= 2^n-1
        so alpha[n] = 0

        therefore:
        (1) alpha[n] = lessOrEqual
        (2) alpha[n] * notAllZeroes = less


    */
    pub fn generateConstraints() {
        enforceBooleanity(notAllZeroes_);
        let two_n = long(POW2(wordBitSize_));
        addRank1Constraint(
            1,
            alpha_p_,
            two_n + rhs_ - lhs_,
            "packed(alpha) = 2^n + B - A",
        );
        alphaDualVariablePacker_.generateConstraints();
        allZeroesTest_.generateConstraints();
        addRank1Constraint(
            1,
            alpha_u_[wordBitSize_],
            lessOrEqual_,
            "alpha[n] = lessOrEqual",
        );
        addRank1Constraint(
            alpha_u_[wordBitSize_],
            notAllZeroes_,
            less_,
            "alpha[n] * notAllZeroes = less",
        );
    }

    pub fn generateWitness() {
        let two_n = long(POW2(wordBitSize_));
        val(alpha_p_) = two_n + val(rhs_) - val(lhs_);
        alphaDualVariablePacker_.generateWitness();
        allZeroesTest_.generateWitness();
        val(lessOrEqual_) = val(alpha_u_[wordBitSize_]);
        val(less_) = val(lessOrEqual_) * val(notAllZeroes_);
    }
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/
