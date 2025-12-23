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
use crate::gadgetlib2::examples::tutorial::{
    HashDifficultyEnforcer_Gadget, NAND_Gadget, R1P_VerifyTransactionAmounts_Gadget,
};
use crate::gadgetlib2::infrastructure::POW2;
use crate::gadgetlib2::pp::Fp;
use crate::gadgetlib2::variable::{
    DualWord, DualWordArray, FElem, FElemInterface, FieldType, FlagVariable, LinearCombination,
    MultiPackedWord, MultiPackedWordArray, PackedWord, ProtoboardPtr, R1P_Elem, UnpackedWord,
    Variable, VariableArray, VariableArrayBase, VariableArrayConfig, VariableArrayType, negate,
    sum,
};
use enum_dispatch::enum_dispatch;
use rccell::RcCell;

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
//      of AND_Gadget's field agnostic create() method and R1P_Gadget's field specific self.val(& ) method.
// */
#[derive(Clone, Default)]
pub struct Gadget<T: Default + Clone> {
    // DISALLOW_COPY_AND_ASSIGN(Gadget);
    pub pb_: ProtoboardPtr,
    pub t: T,
    // Gadget(pb:ProtoboardPtr);
    // virtual pub fn  init() = 0;
    // /* generate constraints must have this interface, however generateWitness for some gadgets
    //    (like CTime) will take auxiliary information (like memory contents). We do not want to force
    //    the interface for generateWitness but do want to make sure it is never invoked from base
    //    class.
    // */
    // virtual pub fn  generateConstraints() = 0;
    // virtual pub fn  generateWitness(); // Not abstract as this method may have different signatures.
    // pub fn addUnaryConstraint(const LinearCombination& a, const ::std::string& name);
    // pub fn  addRank1Constraint(a:LinearCombination,
    //                         b:LinearCombination,
    //                         c:LinearCombination,
    //                         name:&str);
    // pub fn  enforceBooleanity(var:&Variable) {pb_.enforceBooleanity(var);}
    // FElem& self.val(& var:&Variable) {return pb_.self.val(& var);}
    // FElem self.val(& lc:&LinearCombination) {return pb_.self.val(& lc);}
    // FieldType fieldType() const {return pb_.fieldType_;}
    // bool flagIsSet(flag:&FlagVariable) const {return pb_.flagIsSet(flag);}
}
use strum_macros::{EnumIs, EnumTryAs};
#[enum_dispatch(FElemInterface)]
#[enum_dispatch]
pub trait GadgetConfig {
    fn init(&mut self) {}
    fn generateConstraints(&self) {
        panic!("Attempted to generate witness for an incomplete Gadget type.");
    }
    fn generateWitness(&self) {}
}
#[enum_dispatch(GadgetConfig)]
#[derive(EnumIs, EnumTryAs, Clone)]
pub enum GadgetType {
    BinaryAnd(Gadget<BinaryAND_Gadget>),
    R1PAnd(Gadget<R1P_AND_Gadget>),
    BinaryOr(Gadget<BinaryOR_Gadget>),
    R1POr(Gadget<R1P_OR_Gadget>),
    R1PInnerProduct(Gadget<R1P_InnerProduct_Gadget>),
    R1PLooseMux(Gadget<R1P_LooseMUX_Gadget>),
    R1PCompressionPacking(Gadget<R1P_CompressionPacking_Gadget>),
    R1PIntegerPacking(Gadget<R1P_IntegerPacking_Gadget>),
    R1PEqualsConst(Gadget<R1P_EqualsConst_Gadget>),
    DualWord(Gadget<DualWord_Gadget>),
    DualWordArray(Gadget<DualWordArray_Gadget>),
    Toggle(Gadget<Toggle_Gadget>),
    ConditionalFlag(Gadget<ConditionalFlag_Gadget>),
    LogicImplication(Gadget<LogicImplication_Gadget>),
    R1PComparison(Gadget<R1P_Comparison_Gadget>),
    NAND(Gadget<NAND_Gadget>),
    HashDifficultyEnforcer(Gadget<HashDifficultyEnforcer_Gadget>),
    R1PVerifyTransactionAmounts(Gadget<R1P_VerifyTransactionAmounts_Gadget>),
}
impl Default for GadgetType {
    fn default() -> Self {
        Self::BinaryAnd(Gadget::<BinaryAND_Gadget>::default())
    }
}
pub type GadgetPtr = RcCell<GadgetType>; // Not a unique_ptr because sometimes we need to cast
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
  We use multiple inheritance in order to use much needed syntactic sugar. We want self.val(& ) to be
  able to return different types depending on the field so we need to differentiate the interfaces
  between R1P and other fields. We also want the interfaces of specific logical gadgets
  (for instance AND_Gadget which has n inputs and 1 output) in order to construct higher level
  gadgets without specific knowledge of the underlying field. Both interfaces (for instance
  R1P_gadget and AND_Gadget) inherit from Gadget using virtual inheritance (this means only one
  instance of Gadget will be created. For a more thorough discussion on virtual inheritance see
  http://www.phpcompiler.org/articles/virtualinheritance.html
*/

pub trait R1P_Gadget {
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
pub trait AND_GadgetBase {}
impl AND_GadgetBase for BinaryAND_Gadget {}
/// Specific case for and AND with two inputs. Field agnostic
#[derive(Default, Clone)]
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

impl AND_GadgetBase for R1P_AND_Gadget {}
impl R1P_Gadget for R1P_AND_Gadget {}
#[derive(Default, Clone)]
pub struct R1P_AND_Gadget {
    //AND_GadgetBase, public R1P_Gadget

    // R1P_AND_Gadget(pb:ProtoboardPtr, input:VariableArrayType, result:Variable);
    // virtual pub fn  init();

    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct AND_Gadget;

    //external variables
    input_: VariableArrayType,
    result_: Variable,
    //internal variables
    sum_: LinearCombination,
    sumInverse_: Variable,
    // DISALLOW_COPY_AND_ASSIGN(R1P_AND_Gadget);
}

pub struct AND_Gadget;
//  {
// static GadgetPtr create(pb:ProtoboardPtr, input:VariableArrayType, result:Variable);
// static GadgetPtr create(pb:ProtoboardPtr,
//                         input1:LinearCombination,
//                         input2:LinearCombination,
//                         result:Variable);

// DISALLOW_CONSTRUCTION(AND_Gadget);
// DISALLOW_COPY_AND_ASSIGN(AND_Gadget);
// } // pub struct GadgetType

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
pub trait OR_GadgetBase {}
impl OR_GadgetBase for BinaryOR_Gadget {}
/// Specific case for and OR with two inputs. Field agnostic
#[derive(Default, Clone)]
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
impl OR_GadgetBase for R1P_OR_Gadget {}
impl R1P_Gadget for R1P_OR_Gadget {}
#[derive(Default, Clone)]
pub struct R1P_OR_Gadget {
    //OR_GadgetBase, public R1P_Gadget
    sum_: LinearCombination,
    sumInverse_: Variable,
    // R1P_OR_Gadget(pb:ProtoboardPtr, input:VariableArrayType, result:Variable);
    // virtual pub fn  init();
    input_: VariableArrayType,
    result_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct OR_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_OR_Gadget);
}

pub struct OR_Gadget;
//  {
// static GadgetPtr create(pb:ProtoboardPtr, input:VariableArrayType, result:Variable);
// static GadgetPtr create(pb:ProtoboardPtr,
//                         input1:LinearCombination,
//                         input2:LinearCombination,
//                         result:Variable);

// DISALLOW_CONSTRUCTION(OR_Gadget);
// DISALLOW_COPY_AND_ASSIGN(OR_Gadget);
// } // pub struct GadgetType

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
pub trait InnerProduct_GadgetBase {}
impl InnerProduct_GadgetBase for R1P_InnerProduct_Gadget {}
impl R1P_Gadget for R1P_InnerProduct_Gadget {}
#[derive(Default, Clone)]
pub struct R1P_InnerProduct_Gadget {
    //InnerProduct_GadgetBase, public R1P_Gadget
    partialSums_: VariableArrayType,
    // R1P_InnerProduct_Gadget(pb:ProtoboardPtr,
    //                         A:VariableArrayType,
    //                         B:VariableArrayType,
    //                         result:Variable);
    // virtual pub fn  init();
    A_: VariableArrayType,
    B_: VariableArrayType,
    result_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct InnerProduct_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_InnerProduct_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(InnerProduct_Gadget, VariableArrayType, A,
//    VariableArrayType, B,
//    Variable, result);
pub struct InnerProduct_Gadget;
impl InnerProduct_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        A: VariableArrayType,
        B: VariableArrayType,
        result: Variable,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_InnerProduct_Gadget::new(pb, A, B, result);

        pGadget.init();
        RcCell::new(GadgetType::R1PInnerProduct(pGadget))
    }
}

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

pub trait LooseMUX_GadgetBase {
    // : virtual public Gadget
    // LooseMUX_GadgetBase(pb:ProtoboardPtr)->Self Gadget(pb) {}

    // virtual ~LooseMUX_GadgetBase() = 0;
    fn indicatorVariables(&self) -> &VariableArrayType;

    // virtual pub fn  init() = 0;
    // DISALLOW_COPY_AND_ASSIGN(LooseMUX_GadgetBase);
} // pub struct LooseMUX_GadgetBase
// impl LooseMUX_GadgetBase for R1P_LooseMUX_Gadget {
// }
impl R1P_Gadget for R1P_LooseMUX_Gadget {}
#[derive(Default, Clone)]
pub struct R1P_LooseMUX_Gadget {
    //LooseMUX_GadgetBase, public R1P_Gadget
    indicators_: VariableArrayType,
    computeResult_: Vec<GadgetPtr>, // Inner product gadgets
    // R1P_LooseMUX_Gadget(pb:ProtoboardPtr,
    //                     inputs:MultiPackedWordArray,
    //                     index:Variable,
    //                     output:VariableArrayType,
    //                     successFlag:&Variable);
    // virtual pub fn  init();
    inputs_: MultiPackedWordArray,
    index_: Variable,
    output_: VariableArrayType,
    successFlag_: Variable,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // virtual VariableArrayType indicatorVariables() const;
    // friend pub struct LooseMUX_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_LooseMUX_Gadget);
}

pub struct LooseMUX_Gadget;
//  {
// static GadgetPtr create(pb:ProtoboardPtr,
//                         inputs:MultiPackedWordArray,
//                         index:Variable,
//                         output:VariableArrayType,
//                         successFlag:&Variable);
// static GadgetPtr create(pb:ProtoboardPtr,
//                         inputs:VariableArrayType,
//                         index:Variable,
//                         output:Variable,
//                         successFlag:&Variable);

// DISALLOW_CONSTRUCTION(LooseMUX_Gadget);
// DISALLOW_COPY_AND_ASSIGN(LooseMUX_Gadget);
// } // pub struct GadgetType

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
#[derive(Default, Clone, PartialEq)]
pub enum PackingMode {
    #[default]
    PACK,
    UNPACK,
}

// CREATE_GADGET_BASE_CLASS(CompressionPacking_GadgetBase);
pub trait CompressionPacking_GadgetBase {}
impl CompressionPacking_GadgetBase for R1P_CompressionPacking_Gadget {}
impl R1P_Gadget for R1P_CompressionPacking_Gadget {}
#[derive(Default, Clone)]
pub struct R1P_CompressionPacking_Gadget {
    //CompressionPacking_GadgetBase, public R1P_Gadget
    packingMode_: PackingMode,
    // R1P_CompressionPacking_Gadget(pb:ProtoboardPtr,
    //                               unpacked:VariableArrayType,
    //                               packed:VariableArrayType,
    //                               packingMode:PackingMode);
    // virtual pub fn  init();
    unpacked_: VariableArrayType,
    packed_: VariableArrayType,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct CompressionPacking_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_CompressionPacking_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(CompressionPacking_Gadget, VariableArrayType, unpacked, VariableArrayType,
//                               packed, PackingMode, packingMode);
pub struct CompressionPacking_Gadget;
impl CompressionPacking_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        unpacked: VariableArrayType,
        packed: VariableArrayType,
        packingMode: PackingMode,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_CompressionPacking_Gadget::new(pb, unpacked, packed, packingMode);

        pGadget.init();
        RcCell::new(GadgetType::R1PCompressionPacking(pGadget))
    }
}

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
pub trait IntegerPacking_GadgetBase {}
impl IntegerPacking_GadgetBase for R1P_IntegerPacking_Gadget {}
impl R1P_Gadget for R1P_IntegerPacking_Gadget {}
// In R1P compression and arithmetic packing are implemented the same, hence this gadget simply
// instantiates an R1P_CompressionPacking_Gadget
#[derive(Default, Clone)]
pub struct R1P_IntegerPacking_Gadget {
    //IntegerPacking_GadgetBase, public R1P_Gadget
    packingMode_: PackingMode,
    compressionPackingGadget_: GadgetPtr,
    // R1P_IntegerPacking_Gadget(pb:ProtoboardPtr,
    //                           unpacked:VariableArrayType,
    //                           packed:VariableArrayType,
    //                           packingMode:PackingMode);
    // virtual pub fn  init();
    unpacked_: VariableArrayType,
    packed_: VariableArrayType,
    // pub fn  generateConstraints();
    // pub fn  generateWitness();
    // friend pub struct IntegerPacking_Gadget;

    // DISALLOW_COPY_AND_ASSIGN(R1P_IntegerPacking_Gadget);
}

// CREATE_GADGET_FACTORY_CLASS_3(IntegerPacking_Gadget, VariableArrayType, unpacked, VariableArrayType,
//                               packed, PackingMode, packingMode);
pub struct IntegerPacking_Gadget;
impl IntegerPacking_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        unpacked: VariableArrayType,
        packed: VariableArrayType,
        packingMode: PackingMode,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_IntegerPacking_Gadget::new(pb, unpacked, packed, packingMode);

        pGadget.init();
        RcCell::new(GadgetType::R1PIntegerPacking(pGadget))
    }
}

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
pub trait EqualsConst_GadgetBase {}
impl EqualsConst_GadgetBase for R1P_EqualsConst_Gadget {}
impl R1P_Gadget for R1P_EqualsConst_Gadget {}
#[derive(Default, Clone)]
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
pub struct EqualsConst_Gadget;
impl EqualsConst_Gadget {
    pub fn create(
        pb: ProtoboardPtr,
        n: FElem,
        input: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_EqualsConst_Gadget::new(pb, n, input, result);

        pGadget.init();
        RcCell::new(GadgetType::R1PEqualsConst(pGadget))
    }
}
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
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
#[derive(Default, Clone)]
pub struct R1P_Comparison_Gadget {
    //Comparison_GadgetBase, public R1P_Gadget
    wordBitSize_: usize,
    lhs_: PackedWord,
    rhs_: PackedWord,
    less_: FlagVariable,
    lessOrEqual_: FlagVariable,
    alpha_p_: PackedWord,
    alpha_u_: VariableArray<UnpackedWord>,
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

// Declarations of the interfaces and basic gadgets for R1P (Rank 1 prime characteristic)
// constraint systems.

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
impl<T: Default + Clone> Gadget<T> {
    pub fn new(pb: ProtoboardPtr, t: T) -> Self {
        //  pb_(pb)
        assert!(
            pb.is_some(),
            "Attempted to create gadget with uninitialized Protoboard."
        );
        Self { pb_: pb, t }
    }

    pub fn addUnaryConstraint(&self, a: LinearCombination, name: &str) {
        self.pb_
            .as_ref()
            .unwrap()
            .borrow_mut()
            .addUnaryConstraint(a, name.to_owned());
    }

    pub fn addRank1Constraint(
        &self,
        a: LinearCombination,
        b: LinearCombination,
        c: LinearCombination,
        name: &str,
    ) {
        self.pb_
            .as_ref()
            .unwrap()
            .borrow_mut()
            .addRank1Constraint(a, b, c, name.to_owned());
    }

    pub fn enforceBooleanity(&self, var: &Variable) {
        self.pb_
            .as_ref()
            .unwrap()
            .borrow_mut()
            .enforceBooleanity(var);
    }
    pub fn val_set(&self, var: &Variable, i: i32) {
        *self.pb_.as_ref().unwrap().borrow_mut().val(&var) = i.into();
    }
    pub fn val_set_v(&self, var: &Variable, v: FElem) {
        *self.pb_.as_ref().unwrap().borrow_mut().val(&var) = v;
    }
    pub fn val(&self, var: &Variable) -> FElem {
        self.pb_.as_ref().unwrap().borrow_mut().val(&var).clone()
    }
    pub fn val_lc(&self, lc: &LinearCombination) -> FElem {
        self.pb_.as_ref().unwrap().borrow().val_lc(&lc)
    }
    pub fn fieldType(&self) -> FieldType {
        self.pb_.as_ref().unwrap().borrow().fieldType_.clone()
    }
    pub fn flagIsSet(&self, flag: &FlagVariable) -> bool {
        self.pb_.as_ref().unwrap().borrow_mut().flagIsSet(flag)
    }
}
/***********************************/
/***        R1P_Gadget           ***/
/***********************************/

// impl R1P_Gadget {
//     // R1P_Gadget::~R1P_Gadget() {}

//     pub fn addRank1Constraint(
//         a: LinearCombination,
//         b: LinearCombination,
//         c: LinearCombination,
//         name: &string,
//     ) {
//         pb_.addRank1Constraint(a, b, c, name);
//     }
// }
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
    ) -> Gadget<Self> {
        // : Gadget(pb), AND_GadgetBase(pb), input1_(input1), input2_(input2), result_(result)
        Gadget::<Self>::new(
            pb,
            Self {
                input1_: input1,
                input2_: input2,
                result_: result,
            },
        )
    }
}

impl GadgetConfig for Gadget<BinaryAND_Gadget> {
    fn init(&mut self) {}

    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.input1_.clone(),
            self.t.input2_.clone(),
            self.t.result_.clone().into(),
            "result = AND(input1, input2)",
        );
    }

    fn generateWitness(&self) {
        if self.val_lc(&self.t.input1_) == 1 && self.val_lc(&self.t.input2_) == 1 {
            self.val_set(&self.t.result_, 1);
        } else {
            self.val_set(&self.t.result_, 0);
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
    pub fn new(pb: ProtoboardPtr, input: VariableArrayType, result: Variable) -> Gadget<Self> {
        // : Gadget(pb), AND_GadgetBase(pb), R1P_Gadget(pb), input_(input), result_(result),
        //       sumInverse_("sumInverse")
        // assert!(input.len() > 0, "Attempted to create an R1P_AND_Gadget with 0 inputs.");
        // assert!(input.len() <= Fp(-1).as_ulong(), "Attempted to create R1P_AND_Gadget with too "
        //                                                           "many inputs. Will cause overflow!");
        Gadget::<Self>::new(
            pb,
            Self {
                input_: input,
                result_: result,
                sum_: LinearCombination::default(),
                sumInverse_: Variable::new("sumInverse"),
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_AND_Gadget> {
    fn init(&mut self) {
        let numInputs = self.t.input_.len() as i32;
        self.t.sum_ = sum(&self.t.input_) - numInputs;
    }

    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.sum_.clone().into(),
            self.t.result_.clone().into(),
            0.into(),
            "sum * result = 0 | sum == sum(input[i]) - n",
        );
        self.addRank1Constraint(
            self.t.sumInverse_.clone().into(),
            self.t.sum_.clone().into(),
            LinearCombination::from(1) - &self.t.result_,
            "sumInverse * sum = 1-result | sum == sum(input[i]) - n",
        );
    }

    fn generateWitness(&self) {
        let mut sum = FElem::from(0); //FElem
        for i in 0..self.t.input_.len() {
            sum += &self.val(&self.t.input_[i]);
        }
        sum -= &FElem::from(self.t.input_.len() as i32); // sum(input[i]) - n ==> sum
        if sum == FElem::from(0) {
            // AND(input[0], input[1], ... == 1
            self.val_set(&self.t.sumInverse_, 0);
            self.val_set(&self.t.result_, 1);
        } else {
            // AND(input[0], input[1], ...) == 0
            self.val_set_v(&self.t.sumInverse_, sum.inverses(&FieldType::R1P));
            self.val_set(&self.t.result_, 0);
        }
    }
}
impl AND_Gadget {
    pub fn create_r1p(pb: ProtoboardPtr, input: VariableArrayType, result: Variable) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_AND_Gadget::new(pb, input, result);

        pGadget.init();
        RcCell::new(GadgetType::R1PAnd(pGadget))
    }

    pub fn create(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget = BinaryAND_Gadget::new(pb, input1, input2, result);
        pGadget.init();
        RcCell::new(GadgetType::BinaryAnd(pGadget))
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
    ) -> Gadget<Self> {
        //  : Gadget(pb), OR_GadgetBase(pb), input1_(input1), input2_(input2), result_(result)
        Gadget::<Self>::new(
            pb,
            Self {
                input1_: input1,
                input2_: input2,
                result_: result,
            },
        )
    }
}

impl GadgetConfig for Gadget<BinaryOR_Gadget> {
    fn init(&mut self) {}

    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.input1_.clone(),
            self.t.input2_.clone(),
            self.t.input1_.clone() + &self.t.input2_ - &self.t.result_,
            "result = OR(input1, input2)",
        );
    }

    fn generateWitness(&self) {
        if self.val_lc(&self.t.input1_) == FElem::from(1)
            || self.val_lc(&self.t.input2_) == FElem::from(1)
        {
            self.val_set(&self.t.result_, 1);
        } else {
            self.val_set(&self.t.result_, 0);
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
    pub fn new(pb: ProtoboardPtr, input: VariableArrayType, result: Variable) -> Gadget<Self> {
        // : Gadget(pb), OR_GadgetBase(pb), R1P_Gadget(pb), sumInverse_("sumInverse"), input_(input),
        //           result_(result)
        // assert!(input.len() > 0, "Attempted to create an R1P_OR_Gadget with 0 inputs.");
        // assert!(input.len() <= Fp(-1).as_ulong(), "Attempted to create R1P_OR_Gadget with too "
        //                                                           "many inputs. Will cause overflow!");
        Gadget::<Self>::new(
            pb,
            Self {
                input_: input,
                result_: result,
                sum_: LinearCombination::default(),
                sumInverse_: Variable::new("sumInverse"),
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_OR_Gadget> {
    fn init(&mut self) {
        self.t.sum_ = sum(&self.t.input_);
    }

    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.sum_.clone(),
            LinearCombination::from(1) - &self.t.result_,
            0.into(),
            "sum * (1 - result) = 0 | sum == sum(input[i])",
        );
        self.addRank1Constraint(
            self.t.sumInverse_.clone().into(),
            self.t.sum_.clone(),
            self.t.result_.clone().into(),
            "sum * sumInverse = result | sum == sum(input[i])",
        );
    }

    fn generateWitness(&self) {
        let mut sum = FElem::from(0); //FElem
        for i in 0..self.t.input_.len() {
            // sum(input[i]) ==> sum
            sum += &self.val(&self.t.input_[i]);
        }
        if sum == 0 {
            // OR(input[0], input[1], ... == 0
            self.val_set(&self.t.sumInverse_, 0);
            self.val_set(&self.t.result_, 0);
        } else {
            // OR(input[0], input[1], ...) == 1
            self.val_set_v(&self.t.sumInverse_, sum.inverses(&FieldType::R1P));
            self.val_set(&self.t.result_, 1);
        }
    }
}

impl OR_Gadget {
    pub fn create_r1p(pb: ProtoboardPtr, input: VariableArrayType, result: Variable) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_OR_Gadget::new(pb, input, result);
        pGadget.init();
        RcCell::new(GadgetType::R1POr(pGadget))
    }

    pub fn create(
        pb: ProtoboardPtr,
        input1: LinearCombination,
        input2: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget = BinaryOR_Gadget::new(pb, input1, input2, result);
        pGadget.init();
        RcCell::new(GadgetType::BinaryOr(pGadget))
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
    pub fn new(
        pb: ProtoboardPtr,
        A: VariableArrayType,
        B: VariableArrayType,
        result: Variable,
    ) -> Gadget<Self> {
        // : Gadget(pb), InnerProduct_GadgetBase(pb), R1P_Gadget(pb), partialSums_(A.len(),
        //           "partialSums"), A_(A), B_(B), result_(result)
        // assert!(A.len() > 0, "Attempted to create an R1P_InnerProduct_Gadget with 0 inputs.");
        // assert!(A.len() == B.len(), format!("Inner product vector sizes not equal. Sizes are: "
        //                                                     "(A) - {}, (B) - {}", A.len(), B.len()));
        Gadget::<Self>::new(
            pb,
            Self {
                partialSums_: VariableArrayType::Base(VariableArray::<VariableArrayBase>::new(
                    A.len(),
                    "partialSums".to_owned(),
                    VariableArrayBase,
                )),
                A_: A,
                B_: B,
                result_: result,
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_InnerProduct_Gadget> {
    fn init(&mut self) {}

    fn generateConstraints(&self) {
        let n = self.t.A_.len();
        if n == 1 {
            self.addRank1Constraint(
                self.t.A_[0].clone().into(),
                self.t.B_[0].clone().into(),
                self.t.result_.clone().into(),
                "A[0] * B[0] = result",
            );
            return;
        }
        // else (n > 1)
        self.addRank1Constraint(
            self.t.A_[0].clone().into(),
            self.t.B_[0].clone().into(),
            self.t.partialSums_[0].clone().into(),
            "A[0] * B[0] = partialSums[0]",
        );
        for i in 1..=n - 2 {
            self.addRank1Constraint(
                self.t.A_[i].clone().into(),
                self.t.B_[i].clone().into(),
                self.t.partialSums_[i].clone() - &self.t.partialSums_[i - 1],
                &format!(
                    "A[{}] * B[{}] = partialSums[{}] - partialSums[{}]",
                    i,
                    i,
                    i,
                    i - 1,
                ),
            );
        }
        self.addRank1Constraint(
            self.t.A_[n - 1].clone().into(),
            self.t.B_[n - 1].clone().into(),
            self.t.result_.clone() - &self.t.partialSums_[n - 2],
            "A[n-1] * B[n-1] = result - partialSums[n-2]",
        );
    }

    fn generateWitness(&self) {
        let n = self.t.A_.len();
        if n == 1 {
            self.val_set_v(
                &self.t.result_,
                self.val(&self.t.A_[0]) * &self.val(&self.t.B_[0]),
            );
            return;
        }
        // else (n > 1)
        self.val_set_v(
            &self.t.partialSums_[0],
            self.val(&self.t.A_[0]) * &self.val(&self.t.B_[0]),
        );
        for i in 1..=n - 2 {
            self.val_set_v(
                &self.t.partialSums_[i],
                self.val(&self.t.partialSums_[i - 1])
                    + &(self.val(&self.t.A_[i]) * &self.val(&self.t.B_[i])),
            );
        }
        self.val_set_v(
            &self.t.result_,
            self.val(&self.t.partialSums_[n - 2])
                + &(self.val(&self.t.A_[n - 1]) * &self.val(&self.t.B_[n - 1])),
        );
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
        output: VariableArrayType,
        successFlag: Variable,
    ) -> Gadget<Self> {
        // : Gadget(pb), LooseMUX_GadgetBase(pb), R1P_Gadget(pb),
        //           indicators_(inputs.len(), "indicators"), inputs_(inputs.len()), index_(index),
        //           output_(output), successFlag_(successFlag)

        assert!(
            inputs.len() <= Fp::from(-1i64).as_ulong() as usize,
            "Attempted to create R1P_LooseMUX_Gadget with too many inputs. May cause overflow!"
        );
        //    for inpArr  inputs) {
        for i in 0..inputs.len() {
            assert!(
                inputs[i].len() == output.len(),
                "Input VariableArrayType is of incorrect size."
            );
        }
        // ::std::copy(inputs.begin(), inputs.end(), inputs_.begin()); // change type to R1P_VariableArray
        Gadget::<Self>::new(
            pb,
            Self {
                indicators_: VariableArrayType::Base(VariableArray::<VariableArrayBase>::new(
                    inputs.len(),
                    "indicators".to_owned(),
                    VariableArrayBase,
                )),
                inputs_: inputs,
                index_: index,
                output_: output,
                successFlag_: successFlag,
                computeResult_: vec![],
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_LooseMUX_Gadget> {
    fn init(&mut self) {
        // create inputs for the inner products and initialize them. Each iteration creates a
        // VariableArrayType for the i'th elements from each of the vector's VariableArrays.
        for i in 0..self.t.output_.len() {
            let mut curInput = VariableArrayType::default();
            for j in 0..self.t.inputs_.len() {
                curInput.push(self.t.inputs_[j][i].clone());
            }
            self.t.computeResult_.push(InnerProduct_Gadget::create(
                self.pb_.clone(),
                self.t.indicators_.clone(),
                curInput,
                self.t.output_[i].clone(),
            ));
        }
    }

    fn generateConstraints(&self) {
        let n = self.t.inputs_.len();
        for i in 0..n {
            self.addRank1Constraint(
                self.t.indicators_[i].clone().into(),
                (self.t.index_.clone() - i as i32).into(),
                0.into(),
                &format!("indicators[{}] * (index - {}) = 0", i, i),
            );
        }
        self.addRank1Constraint(
            sum(&self.t.indicators_),
            1.into(),
            self.t.successFlag_.clone().into(),
            "sum(indicators) * 1 = successFlag",
        );
        self.enforceBooleanity(&self.t.successFlag_);
        for curGadget in &self.t.computeResult_ {
            curGadget.borrow().generateConstraints();
        }
    }

    fn generateWitness(&self) {
        let n = self.t.inputs_.len();
        /* assumes that idx can be fit in ulong; true for our purposes for now */
        let index = self.val(&self.t.index_).asLong() as usize;
        let arraySize = n;
        for i in 0..n {
            self.val_set(&self.t.indicators_[i], 0); // Redundant, but just in case.
        }
        if index >= n {
            //  || index < 0
            self.val_set(&self.t.successFlag_, 0);
        } else {
            // index in bounds
            self.val_set(&self.t.indicators_[index], 1);
            self.val_set(&self.t.successFlag_, 1);
        }
        for curGadget in &self.t.computeResult_ {
            curGadget.borrow().generateWitness();
        }
    }
}
impl LooseMUX_GadgetBase for Gadget<R1P_LooseMUX_Gadget> {
    fn indicatorVariables(&self) -> &VariableArrayType {
        &self.t.indicators_
    }
}

impl LooseMUX_Gadget {
    pub fn create_r1p(
        pb: ProtoboardPtr,
        inputs: MultiPackedWordArray,
        index: Variable,
        output: VariableArrayType,
        successFlag: Variable,
    ) -> GadgetPtr {
        assert!(
            pb.as_ref().unwrap().borrow().fieldType_ == FieldType::R1P,
            "Attempted to create gadget of undefined Protoboard type."
        );
        let mut pGadget = R1P_LooseMUX_Gadget::new(pb, inputs, index, output, successFlag);

        pGadget.init();
        RcCell::new(GadgetType::R1PLooseMux(pGadget))
    }

    /**
        An overload for the private case in which we only want to multiplex one Variable. This is
        usually the case in R1P.
    **/
    pub fn create(
        pb: ProtoboardPtr,
        inputs: VariableArrayType,
        index: Variable,
        output: Variable,
        successFlag: Variable,
    ) -> GadgetPtr {
        let mut inpVec = MultiPackedWordArray::default();
        for i in 0..inputs.len() {
            let mut cur = VariableArray::<MultiPackedWord>::from(
                pb.as_ref().unwrap().borrow().fieldType_.clone(),
            );
            cur.push(inputs[i].clone());
            inpVec.push(cur);
        }
        let mut outVec = VariableArrayType::default();
        outVec.push(output);
        let result = LooseMUX_Gadget::create_r1p(pb, inpVec, index, outVec, successFlag);
        result
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
        unpacked: VariableArrayType,
        packed: VariableArrayType,
        packingMode: PackingMode,
    ) -> Gadget<Self> {
        // : Gadget(pb), CompressionPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
        //       unpacked_(unpacked), packed_(packed)
        let n = unpacked.len();
        assert!(n > 0, "Attempted to pack 0 bits in R1P.");
        assert!(
            packed.len() == 1,
            "Attempted to pack into more than 1 Variable in R1P_CompressionPacking_Gadget."
        );
        // TODO add assertion that 'n' bits can fit in the field characteristic
        Gadget::<Self>::new(
            pb,
            Self {
                packingMode_: packingMode,
                unpacked_: unpacked,
                packed_: packed,
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_CompressionPacking_Gadget> {
    fn init(&mut self) {}

    fn generateConstraints(&self) {
        let n = self.t.unpacked_.len();
        let mut packed = LinearCombination::default();
        let mut two_i = FElem::from(1); // Will hold 2^i
        for i in 0..n {
            packed += &((self.t.unpacked_[i].clone() * &two_i).into());
            two_i += &two_i.clone();
            if self.t.packingMode_ == PackingMode::UNPACK {
                self.enforceBooleanity(&self.t.unpacked_[i]);
            }
        }
        self.addRank1Constraint(
            self.t.packed_[0].clone().into(),
            1.into(),
            packed,
            "packed[0] = sum(2^i * unpacked[i])",
        );
    }

    fn generateWitness(&self) {
        let n = self.t.unpacked_.len();
        if self.t.packingMode_ == PackingMode::PACK {
            let mut packedVal = FElem::from(0); //
            let mut two_i = FElem::from(R1P_Elem::from(1i64)); // will hold 2^i
            for i in 0..n {
                assert!(
                    self.val(&self.t.unpacked_[i]).asLong() == 0
                        || self.val(&self.t.unpacked_[i]).asLong() == 1,
                    "unpacked[{}]  = {}. Expected a Boolean value.",
                    i,
                    self.val(&self.t.unpacked_[i]).asLong()
                );
                packedVal +=
                    &((two_i.clone() * self.val(&self.t.unpacked_[i]).asLong() as i32).into());
                two_i += &(two_i.clone().into());
            }
            self.val_set_v(&self.t.packed_[0], packedVal);
            return;
        }
        // else (UNPACK)
        assert!(
            self.t.packingMode_ == PackingMode::UNPACK,
            "Packing gadget created with unknown packing mode."
        );
        for i in 0..n {
            self.val_set(
                &self.t.unpacked_[i],
                self.val(&self.t.packed_[0])
                    .getBits(i as u32, &FieldType::R1P),
            );
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
        unpacked: VariableArrayType,
        packed: VariableArrayType,
        packingMode: PackingMode,
    ) -> Gadget<Self> {
        // : Gadget(pb), IntegerPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
        //   unpacked_(unpacked), packed_(packed)
        let n = unpacked.len();
        assert!(n > 0, "Attempted to pack 0 bits in R1P.");
        assert!(
            packed.len() == 1,
            "Attempted to pack into more than 1 Variable in R1P_IntegerPacking_Gadget."
        );
        Gadget::<Self>::new(
            pb,
            Self {
                packingMode_: packingMode,
                unpacked_: unpacked,
                packed_: packed,
                compressionPackingGadget_: RcCell::new(GadgetType::default()),
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_IntegerPacking_Gadget> {
    fn init(&mut self) {
        self.t.compressionPackingGadget_ = CompressionPacking_Gadget::create(
            self.pb_.clone().clone(),
            self.t.unpacked_.clone(),
            self.t.packed_.clone(),
            self.t.packingMode_.clone(),
        );
    }

    fn generateConstraints(&self) {
        self.t
            .compressionPackingGadget_
            .borrow()
            .generateConstraints();
    }

    fn generateWitness(&self) {
        self.t.compressionPackingGadget_.borrow().generateWitness();
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
    pub fn new(
        pb: ProtoboardPtr,
        n: FElem,
        input: LinearCombination,
        result: Variable,
    ) -> Gadget<Self> {
        // : Gadget(pb), EqualsConst_GadgetBase(pb), R1P_Gadget(pb), n_(n),
        //           aux_("aux (R1P_EqualsConst_Gadget)"), input_(input), result_(result)
        Gadget::<Self>::new(
            pb,
            Self {
                n_: n,
                aux_: Variable::new("aux :R1P_EqualsConst_Gadget"),
                input_: input,
                result_: result,
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_EqualsConst_Gadget> {
    fn init(&mut self) {}

    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.input_.clone() - &self.t.n_,
            self.t.result_.clone().into(),
            0.into(),
            "(input - n) * result = 0",
        );
        self.addRank1Constraint(
            self.t.input_.clone() - &self.t.n_,
            self.t.aux_.clone().into(),
            LinearCombination::from(1) - &self.t.result_,
            "(input - n) * aux = 1 - result",
        );
    }

    fn generateWitness(&self) {
        self.val_set_v(
            &self.t.aux_,
            if self.val_lc(&self.t.input_) == self.t.n_ {
                0.into()
            } else {
                (self.val_lc(&self.t.input_) - &self.t.n_).inverses(&FieldType::R1P)
            },
        );
        self.val_set(
            &self.t.result_,
            (if self.val_lc(&self.t.input_) == self.t.n_ {
                1
            } else {
                0
            }),
        );
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
    pub fn new(pb: ProtoboardPtr, var: DualWord, packingMode: PackingMode) -> Gadget<Self> {
        // : Gadget(pb), var_(var), packingMode_(packingMode), packingGadget_()
        Gadget::<Self>::new(
            pb,
            Self {
                packingMode_: packingMode,
                var_: var,
                packingGadget_: RcCell::new(GadgetType::default()),
            },
        )
    }
    pub fn create(pb: ProtoboardPtr, var: DualWord, packingMode: PackingMode) -> GadgetPtr {
        let mut pGadget = DualWord_Gadget::new(pb, var, packingMode);
        pGadget.init();
        RcCell::new(GadgetType::DualWord(pGadget))
    }
}

impl GadgetConfig for Gadget<DualWord_Gadget> {
    fn init(&mut self) {
        self.t.packingGadget_ = CompressionPacking_Gadget::create(
            self.pb_.clone(),
            self.t.var_.unpacked().into(),
            self.t.var_.multipacked().into(),
            self.t.packingMode_.clone(),
        );
    }

    fn generateConstraints(&self) {
        self.t.packingGadget_.borrow().generateConstraints();
    }

    fn generateWitness(&self) {
        self.t.packingGadget_.borrow().generateWitness();
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
    pub fn new(pb: ProtoboardPtr, vars: DualWordArray, packingMode: PackingMode) -> Gadget<Self> {
        //  : Gadget(pb), vars_(vars), packingMode_(packingMode), packingGadgets_()
        Gadget::<Self>::new(
            pb,
            Self {
                packingMode_: packingMode,
                vars_: vars,
                packingGadgets_: vec![],
            },
        )
    }
    pub fn create(pb: ProtoboardPtr, vars: DualWordArray, packingMode: PackingMode) -> GadgetPtr {
        let mut pGadget = DualWordArray_Gadget::new(pb, vars, packingMode);
        pGadget.init();
        RcCell::new(GadgetType::DualWordArray(pGadget))
    }
}

impl GadgetConfig for Gadget<DualWordArray_Gadget> {
    fn init(&mut self) {
        let unpacked = self.t.vars_.unpacked();
        let packed = self.t.vars_.multipacked();
        for i in 0..self.t.vars_.len() {
            let curGadget = CompressionPacking_Gadget::create(
                self.pb_.clone(),
                unpacked[i].clone().into(),
                packed[i].clone().into(),
                self.t.packingMode_.clone(),
            );
            self.t.packingGadgets_.push(curGadget);
        }
    }

    fn generateConstraints(&self) {
        for gadget in &self.t.packingGadgets_ {
            gadget.borrow().generateConstraints();
        }
    }

    fn generateWitness(&self) {
        for gadget in &self.t.packingGadgets_ {
            gadget.borrow().generateWitness();
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
    ) -> Gadget<Self> {
        // : Gadget(pb), toggle_(toggle), zeroValue_(zeroValue), oneValue_(oneValue),
        //           result_(result)
        Gadget::<Self>::new(
            pb,
            Self {
                toggle_: toggle,
                zeroValue_: zeroValue,
                oneValue_: oneValue,
                result_: result,
            },
        )
    }

    pub fn create(
        pb: ProtoboardPtr,
        toggle: FlagVariable,
        zeroValue: LinearCombination,
        oneValue: LinearCombination,
        result: Variable,
    ) -> GadgetPtr {
        let mut pGadget = Toggle_Gadget::new(pb, toggle, zeroValue, oneValue, result);
        pGadget.init();
        RcCell::new(GadgetType::Toggle(pGadget))
    }
}

impl GadgetConfig for Gadget<Toggle_Gadget> {
    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.toggle_.clone().into(),
            self.t.oneValue_.clone() - &self.t.zeroValue_,
            self.t.result_.clone() - &self.t.zeroValue_,
            "result = (1 - toggle) * zeroValue + toggle * oneValue",
        );
    }

    fn generateWitness(&self) {
        if self.val(&self.t.toggle_) == 0 {
            self.val_set_v(&self.t.result_, self.val_lc(&self.t.zeroValue_));
        } else if self.val(&self.t.toggle_) == 1 {
            self.val_set_v(&self.t.result_, self.val_lc(&self.t.oneValue_));
        } else {
            panic!("Toggle value must be Boolean.");
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
    pub fn new(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: FlagVariable,
    ) -> Gadget<Self> {
        //  : Gadget(pb), flag_(flag), condition_(condition),
        //           auxConditionInverse_("ConditionalFlag_Gadget::auxConditionInverse_")
        Gadget::<Self>::new(
            pb,
            Self {
                flag_: flag,
                condition_: condition,
                auxConditionInverse_: Variable::new("ConditionalFlag_Gadget::auxConditionInverse_"),
            },
        )
    }

    pub fn create(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: FlagVariable,
    ) -> GadgetPtr {
        let mut pGadget = ConditionalFlag_Gadget::new(pb, condition, flag);
        pGadget.init();
        RcCell::new(GadgetType::ConditionalFlag(pGadget))
    }
}

impl GadgetConfig for Gadget<ConditionalFlag_Gadget> {
    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.condition_.clone(),
            negate(&(self.t.flag_.clone().into())),
            0.into(),
            "condition * not(flag) = 0",
        );
        self.addRank1Constraint(
            self.t.condition_.clone(),
            self.t.auxConditionInverse_.clone().into(),
            self.t.flag_.clone().into(),
            "condition * auxConditionInverse = flag",
        );
    }

    fn generateWitness(&self) {
        if self.val_lc(&self.t.condition_) == 0 {
            self.val_set(&self.t.flag_, 0);
            self.val_set(&self.t.auxConditionInverse_, 0);
        } else {
            self.val_set(&self.t.flag_, 1);
            self.val_set_v(
                &self.t.auxConditionInverse_,
                self.val_lc(&self.t.condition_).inverses(&self.fieldType()),
            );
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
    pub fn new(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: FlagVariable,
    ) -> Gadget<Self> {
        //  : Gadget(pb), flag_(flag), condition_(condition)
        Gadget::<Self>::new(
            pb,
            Self {
                flag_: flag,
                condition_: condition,
            },
        )
    }

    pub fn create(
        pb: ProtoboardPtr,
        condition: LinearCombination,
        flag: FlagVariable,
    ) -> GadgetPtr {
        let mut pGadget = LogicImplication_Gadget::new(pb, condition, flag);
        pGadget.init();
        RcCell::new(GadgetType::LogicImplication(pGadget))
    }
}

impl GadgetConfig for Gadget<LogicImplication_Gadget> {
    fn generateConstraints(&self) {
        self.addRank1Constraint(
            self.t.condition_.clone().into(),
            negate(&(self.t.flag_.clone().into())),
            0.into(),
            "condition * not(flag) = 0",
        );
    }

    fn generateWitness(&self) {
        if self.val_lc(&self.t.condition_) == 1 {
            self.val_set(&self.t.flag_, 1);
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
        lessOrEqual: FlagVariable,
    ) -> Gadget<Self> {
        // : Gadget(pb), Comparison_GadgetBase(pb), R1P_Gadget(pb), wordBitSize_(wordBitSize),
        //           lhs_(lhs), rhs_(rhs), less_(less), lessOrEqual_(lessOrEqual),
        //           alpha_u_(wordBitSize,  "alpha"), notAllZeroes_("notAllZeroes")
        Gadget::<Self>::new(
            pb,
            Self {
                wordBitSize_: wordBitSize,
                lhs_: lhs,
                rhs_: rhs,
                less_: less,
                lessOrEqual_: lessOrEqual,
                alpha_u_: UnpackedWord::new(wordBitSize, "alpha"),
                notAllZeroes_: FlagVariable::new("notAllZeroes"),
                allZeroesTest_: RcCell::new(GadgetType::default()),
                alphaDualVariablePacker_: RcCell::new(GadgetType::default()),
                alpha_p_: PackedWord::default(),
            },
        )
    }
}

impl GadgetConfig for Gadget<R1P_Comparison_Gadget> {
    fn init(&mut self) {
        self.t.allZeroesTest_ = OR_Gadget::create_r1p(
            self.pb_.clone(),
            self.t.alpha_u_.clone().into(),
            self.t.notAllZeroes_.clone(),
        );
        self.t.alpha_u_.push(self.t.lessOrEqual_.clone());
        self.t.alphaDualVariablePacker_ = CompressionPacking_Gadget::create(
            self.pb_.clone(),
            self.t.alpha_u_.clone().into(),
            VariableArrayType::Base(VariableArray::<VariableArrayBase>::new_with_variable(
                1,
                self.t.alpha_p_.clone(),
                VariableArrayBase,
            )),
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
    fn generateConstraints(&self) {
        self.enforceBooleanity(&self.t.notAllZeroes_);
        let two_n = POW2(self.t.wordBitSize_ as i32) as i64;
        self.addRank1Constraint(
            1.into(),
            self.t.alpha_p_.clone().into(),
            LinearCombination::from(two_n as u64) + &self.t.rhs_ - &self.t.lhs_,
            "packed(alpha) = 2^n + B - A",
        );
        self.t
            .alphaDualVariablePacker_
            .borrow()
            .generateConstraints();
        self.t.allZeroesTest_.borrow().generateConstraints();
        self.addRank1Constraint(
            1.into(),
            self.t.alpha_u_[self.t.wordBitSize_].clone().into(),
            self.t.lessOrEqual_.clone().into(),
            "alpha[n] = lessOrEqual",
        );
        self.addRank1Constraint(
            self.t.alpha_u_[self.t.wordBitSize_].clone().into(),
            self.t.notAllZeroes_.clone().into(),
            self.t.less_.clone().into(),
            "alpha[n] * notAllZeroes = less",
        );
    }

    fn generateWitness(&self) {
        let two_n = POW2(self.t.wordBitSize_ as i32) as i64;
        self.val_set_v(
            &self.t.alpha_p_,
            FElem::from(two_n) + &self.val(&self.t.rhs_) - &self.val(&self.t.lhs_),
        );
        self.t.alphaDualVariablePacker_.borrow().generateWitness();
        self.t.allZeroesTest_.borrow().generateWitness();
        self.val_set_v(
            &self.t.lessOrEqual_,
            self.val(&self.t.alpha_u_[self.t.wordBitSize_]),
        );
        self.val_set_v(
            &self.t.less_,
            self.val(&self.t.lessOrEqual_) * &self.val(&self.t.notAllZeroes_),
        );
    }
}

/*********************************/
/***       END OF Gadget       ***/
/*********************************/
