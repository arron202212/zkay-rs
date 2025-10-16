// /** @file
//  *****************************************************************************
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
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_GADGET_HPP_
// // #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_GADGET_HPP_

// use  <vector>

// use crate::gadgetlib2::gadgetMacros;
// use crate::gadgetlib2::protoboard;
// use crate::gadgetlib2::variable;

// namespace gadgetlib2 {

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                         class Gadget                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /**
//  Gadget class, representing the constraints and witness generation for a logical task.

//  Gadget hierarchy:
//  (Here and elsewhere: R1P = Rank 1 constraints over a prime-characteristic field.)
//  Gadgets have a somewhat cumbersome class hierarchy, for the sake of clean gadget construction.
//  (1) A field agnostic, concrete (as opposed to interface) gadget will derive from Gadget. For
//      instance NAND needs only AND and NOT and does not care about the field, thus it derives from
//      Gadget.
//  (2) Field specific interface class R1P_Gadget derives from Gadget using virtual
//      inheritance, in order to avoid the Dreaded Diamond problem (see
//      http://stackoverflow.com/a/21607/1756254 for more info)
//  (3) Functional interface classes such as LooseMUX_GadgetBase virtually derive from Gadget and
//      define special gadget functionality. For gadgets with no special interfaces we use the macro
//      CREATE_GADGET_BASE_CLASS() for the sake of code consistency (these gadgets can work the same
//      without this base class). This is an interface only and the implementation of AND_Gadget is
//      field specific.
//  (4) These field specific gadgets will have a factory class with static method create, such as
//      AND_Gadget::create(...) in order to agnostically create this gadget for use by a field
//      agnostic gadget.
//  (5) Concrete field dependent gadgets derive via multiple inheritance from two interfaces.
//      e.g. R1P_AND_Gadget derives from both AND_Gadget and R1P_Gadget. This was done to allow usage
//      of AND_Gadget's field agnostic create() method and R1P_Gadget's field specific val() method.
// */
// class Gadget {
// private:
//     DISALLOW_COPY_AND_ASSIGN(Gadget);
// protected:
//     ProtoboardPtr pb_;
// public:
//     Gadget(ProtoboardPtr pb);
//     virtual void init() = 0;
//     /* generate constraints must have this interface, however generateWitness for some gadgets
//        (like CTime) will take auxiliary information (like memory contents). We do not want to force
//        the interface for generateWitness but do want to make sure it is never invoked from base
//        class.
//     */
//     virtual void generateConstraints() = 0;
//     virtual void generateWitness(); // Not abstract as this method may have different signatures.
//     void addUnaryConstraint(const LinearCombination& a, const ::std::string& name);
//     void addRank1Constraint(const LinearCombination& a,
//                             const LinearCombination& b,
//                             const LinearCombination& c,
//                             const ::std::string& name);
//     void enforceBooleanity(const Variable& var) {pb_->enforceBooleanity(var);}
//     FElem& val(const Variable& var) {return pb_->val(var);}
//     FElem val(const LinearCombination& lc) {return pb_->val(lc);}
//     FieldType fieldType() const {return pb_->fieldType_;}
//     bool flagIsSet(const FlagVariable& flag) const {return pb_->flagIsSet(flag);}
// };

// type ::std::shared_ptr<Gadget> GadgetPtr; // Not a unique_ptr because sometimes we need to cast
//                                              // these pointers for specific gadget operations.
// /***********************************/
// /***   END OF CLASS DEFINITION   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                      Gadget Interfaces                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//    We use multiple inheritance in order to use much needed syntactic sugar. We want val() to be
//    able to return different types depending on the field so we need to differentiate the interfaces
//    between R1P and other fields. We also want the interfaces of specific logical gadgets
//    (for instance AND_Gadget which has n inputs and 1 output) in order to construct higher level
//    gadgets without specific knowledge of the underlying field. Both interfaces (for instance
//    R1P_gadget and AND_Gadget) inherit from Gadget using virtual inheritance (this means only one
//    instance of Gadget will be created. For a more thorough discussion on virtual inheritance see
//    http://www.phpcompiler.org/articles/virtualinheritance.html
//  */

// class R1P_Gadget : virtual public Gadget {
// public:
//     R1P_Gadget(ProtoboardPtr pb) : Gadget(pb) {}
//     virtual ~R1P_Gadget() = 0;

//     virtual void addRank1Constraint(const LinearCombination& a,
//                                     const LinearCombination& b,
//                                     const LinearCombination& c,
//                                     const ::std::string& name);
// private:
//     virtual void init() = 0; // private in order to force programmer to invoke from a Gadget* only
//     DISALLOW_COPY_AND_ASSIGN(R1P_Gadget);
// }; // class R1P_Gadget

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                     AND_Gadget classes                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(AND_GadgetBase);

// /// Specific case for and AND with two inputs. Field agnostic
// class BinaryAND_Gadget : public AND_GadgetBase {
// private:
//     BinaryAND_Gadget(ProtoboardPtr pb,
//                      const LinearCombination& input1,
//                      const LinearCombination& input2,
//                      const Variable& result);
//     void init();
//     void generateConstraints();
//     void generateWitness();
// public:
//     friend class AND_Gadget;
// private:
//     //external variables
//     const LinearCombination input1_;
//     const LinearCombination input2_;
//     const Variable result_;

//     DISALLOW_COPY_AND_ASSIGN(BinaryAND_Gadget);
// }; // class BinaryAND_Gadget

// class R1P_AND_Gadget : public AND_GadgetBase, public R1P_Gadget {
// private:
//     R1P_AND_Gadget(ProtoboardPtr pb, const VariableArray& input, const Variable& result);
//     virtual void init();
// public:
//     void generateConstraints();
//     void generateWitness();
//     friend class AND_Gadget;
// private:
//     //external variables
//     const VariableArray input_;
//     const Variable result_;
//     //internal variables
//     LinearCombination sum_;
//     Variable sumInverse_;

//     DISALLOW_COPY_AND_ASSIGN(R1P_AND_Gadget);
// };


// class AND_Gadget {
// public:
//     static GadgetPtr create(ProtoboardPtr pb, const VariableArray& input, const Variable& result);
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const LinearCombination& input1,
//                             const LinearCombination& input2,
//                             const Variable& result);
// private:
//     DISALLOW_CONSTRUCTION(AND_Gadget);
//     DISALLOW_COPY_AND_ASSIGN(AND_Gadget);
// }; // class GadgetType


// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                     OR_Gadget classes                      ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(OR_GadgetBase);

// /// Specific case for and OR with two inputs. Field agnostic
// class BinaryOR_Gadget : public OR_GadgetBase {
// private:
//     BinaryOR_Gadget(ProtoboardPtr pb,
//                     const LinearCombination& input1,
//                     const LinearCombination& input2,
//                     const Variable& result);
//     void init();
//     void generateConstraints();
//     void generateWitness();
// public:
//     friend class OR_Gadget;
// private:
//     //external variables
//     const LinearCombination input1_;
//     const LinearCombination input2_;
//     const Variable result_;

//     DISALLOW_COPY_AND_ASSIGN(BinaryOR_Gadget);
// }; // class BinaryOR_Gadget

// class R1P_OR_Gadget : public OR_GadgetBase, public R1P_Gadget {
// private:
//     LinearCombination sum_;
//     Variable sumInverse_;
//     R1P_OR_Gadget(ProtoboardPtr pb, const VariableArray& input, const Variable& result);
//     virtual void init();
// public:
//     const VariableArray input_;
//     const Variable result_;
//     void generateConstraints();
//     void generateWitness();
//     friend class OR_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_OR_Gadget);
// };

// class OR_Gadget {
// public:
//     static GadgetPtr create(ProtoboardPtr pb, const VariableArray& input, const Variable& result);
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const LinearCombination& input1,
//                             const LinearCombination& input2,
//                             const Variable& result);
// private:
//     DISALLOW_CONSTRUCTION(OR_Gadget);
//     DISALLOW_COPY_AND_ASSIGN(OR_Gadget);
// }; // class GadgetType

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************               InnerProduct_Gadget classes                  ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(InnerProduct_GadgetBase);

// class R1P_InnerProduct_Gadget : public InnerProduct_GadgetBase, public R1P_Gadget {
// private:
//     VariableArray partialSums_;
//     R1P_InnerProduct_Gadget(ProtoboardPtr pb,
//                             const VariableArray& A,
//                             const VariableArray& B,
//                             const Variable& result);
//     virtual void init();
// public:
//     const VariableArray A_, B_;
//     const Variable result_;
//     void generateConstraints();
//     void generateWitness();
//     friend class InnerProduct_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_InnerProduct_Gadget);
// };

// CREATE_GADGET_FACTORY_CLASS_3(InnerProduct_Gadget, VariableArray, A,
//                                                    VariableArray, B,
//                                                    Variable, result);

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                LooseMUX_Gadget classes                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     Loose Multiplexer (MUX):
//     Multiplexes one Variable
//     index not in bounds -> success_flag = 0
//     index in bounds && success_flag = 1 -> result is correct
//     index is in bounds, we can also set success_flag to 0 -> result will be forced to 0
// */

// class LooseMUX_GadgetBase : virtual public Gadget {
// protected:
//     LooseMUX_GadgetBase(ProtoboardPtr pb) : Gadget(pb) {}
// public:
//     virtual ~LooseMUX_GadgetBase() = 0;
//     virtual VariableArray indicatorVariables() const = 0;
// private:
//     virtual void init() = 0;
//     DISALLOW_COPY_AND_ASSIGN(LooseMUX_GadgetBase);
// }; // class LooseMUX_GadgetBase


// class R1P_LooseMUX_Gadget : public LooseMUX_GadgetBase, public R1P_Gadget {
// private:
//     VariableArray indicators_;
//     ::std::vector<GadgetPtr> computeResult_; // Inner product gadgets
//     R1P_LooseMUX_Gadget(ProtoboardPtr pb,
//                         const MultiPackedWordArray& inputs,
//                         const Variable& index,
//                         const VariableArray& output,
//                         const Variable& successFlag);
//     virtual void init();
// public:
//     MultiPackedWordArray inputs_;
//     const Variable index_;
//     const VariableArray output_;
//     const Variable successFlag_;
//     void generateConstraints();
//     void generateWitness();
//     virtual VariableArray indicatorVariables() const;
//     friend class LooseMUX_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_LooseMUX_Gadget);
// };

// class LooseMUX_Gadget {
// public:
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const MultiPackedWordArray& inputs,
//                             const Variable& index,
//                             const VariableArray& output,
//                             const Variable& successFlag);
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const VariableArray& inputs,
//                             const Variable& index,
//                             const Variable& output,
//                             const Variable& successFlag);
// private:
//     DISALLOW_CONSTRUCTION(LooseMUX_Gadget);
//     DISALLOW_COPY_AND_ASSIGN(LooseMUX_Gadget);
// }; // class GadgetType


// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************            CompressionPacking_Gadget classes               ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// // TODO change class name to bitpacking
// enum class PackingMode : bool {PACK, UNPACK};

// CREATE_GADGET_BASE_CLASS(CompressionPacking_GadgetBase);

// class R1P_CompressionPacking_Gadget : public CompressionPacking_GadgetBase, public R1P_Gadget {
// private:
//     PackingMode packingMode_;
//     R1P_CompressionPacking_Gadget(ProtoboardPtr pb,
//                                   const VariableArray& unpacked,
//                                   const VariableArray& packed,
//                                   PackingMode packingMode);
//     virtual void init();
// public:
//     const VariableArray unpacked_;
//     const VariableArray packed_;
//     void generateConstraints();
//     void generateWitness();
//     friend class CompressionPacking_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_CompressionPacking_Gadget);
// };

// CREATE_GADGET_FACTORY_CLASS_3(CompressionPacking_Gadget, VariableArray, unpacked, VariableArray,
//                               packed, PackingMode, packingMode);


// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************            IntegerPacking_Gadget classes                ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// CREATE_GADGET_BASE_CLASS(IntegerPacking_GadgetBase);

// // In R1P compression and arithmetic packing are implemented the same, hence this gadget simply
// // instantiates an R1P_CompressionPacking_Gadget
// class R1P_IntegerPacking_Gadget : public IntegerPacking_GadgetBase, public R1P_Gadget {
// private:
//     PackingMode packingMode_;
//     GadgetPtr compressionPackingGadget_;
//     R1P_IntegerPacking_Gadget(ProtoboardPtr pb,
//                               const VariableArray& unpacked,
//                               const VariableArray& packed,
//                               PackingMode packingMode);
//     virtual void init();
// public:
//     const VariableArray unpacked_;
//     const VariableArray packed_;
//     void generateConstraints();
//     void generateWitness();
//     friend class IntegerPacking_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_IntegerPacking_Gadget);
// };

// CREATE_GADGET_FACTORY_CLASS_3(IntegerPacking_Gadget, VariableArray, unpacked, VariableArray,
//                               packed, PackingMode, packingMode);

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 EqualsConst_Gadget classes                 ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     Gadgets recieve a constant field element n, and an input.
//     input == n ==> result = 1
//     input != n ==> result = 0
// */

// // TODO change to take LinearCombination as input and change AND/OR to use this
// CREATE_GADGET_BASE_CLASS(EqualsConst_GadgetBase);

// class R1P_EqualsConst_Gadget : public EqualsConst_GadgetBase, public R1P_Gadget {
// private:
//     const FElem n_;
//     Variable aux_;
//     R1P_EqualsConst_Gadget(ProtoboardPtr pb,
//                            const FElem& n,
//                            const LinearCombination& input,
//                            const Variable& result);
//     virtual void init();
// public:
//     const LinearCombination input_;
//     const Variable result_;
//     void generateConstraints();
//     void generateWitness();
//     friend class EqualsConst_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_EqualsConst_Gadget);
// };

// CREATE_GADGET_FACTORY_CLASS_3(EqualsConst_Gadget, FElem, n, LinearCombination, input,
//                               Variable, result);

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   DualWord_Gadget                      ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// //TODO add test

// class DualWord_Gadget : public Gadget {

// private:
//     const DualWord var_;
//     const PackingMode packingMode_;

//     GadgetPtr packingGadget_;

//     DualWord_Gadget(ProtoboardPtr pb, const DualWord& var, PackingMode packingMode);
//     virtual void init();
//     DISALLOW_COPY_AND_ASSIGN(DualWord_Gadget);
// public:
//     static GadgetPtr create(ProtoboardPtr pb, const DualWord& var, PackingMode packingMode);
//     void generateConstraints();
//     void generateWitness();
// };

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 DualWordArray_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// //TODO add test

// class DualWordArray_Gadget : public Gadget {

// private:
//     const DualWordArray vars_;
//     const PackingMode packingMode_;

//     ::std::vector<GadgetPtr> packingGadgets_;

//     DualWordArray_Gadget(ProtoboardPtr pb,
//                              const DualWordArray& vars,
//                              PackingMode packingMode);
//     virtual void init();
//     DISALLOW_COPY_AND_ASSIGN(DualWordArray_Gadget);
// public:
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const DualWordArray& vars,
//                             PackingMode packingMode);
//     void generateConstraints();
//     void generateWitness();
// };

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                        Toggle_Gadget                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// //TODO add test

// /// A gadget for the following semantics:
// /// If toggle is 0, zeroValue --> result
// /// If toggle is 1, oneValue --> result
// /// Uses 1 constraint

// class Toggle_Gadget : public Gadget {
// private:
//     FlagVariable toggle_;
//     LinearCombination zeroValue_;
//     LinearCombination oneValue_;
//     Variable result_;

//     Toggle_Gadget(ProtoboardPtr pb,
//                   const FlagVariable& toggle,
//                   const LinearCombination& zeroValue,
//                   const LinearCombination& oneValue,
//                   const Variable& result);

//     virtual void init() {}
//     DISALLOW_COPY_AND_ASSIGN(Toggle_Gadget);
// public:
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const FlagVariable& toggle,
//                             const LinearCombination& zeroValue,
//                             const LinearCombination& oneValue,
//                             const Variable& result);

//     void generateConstraints();
//     void generateWitness();
// };

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/


// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   ConditionalFlag_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /// A gadget for the following semantics:
// /// condition != 0  --> flag = 1
// /// condition == 0 --> flag = 0
// /// Uses 2 constraints

// class ConditionalFlag_Gadget : public Gadget {
// private:
//     FlagVariable flag_;
//     LinearCombination condition_;
//     Variable auxConditionInverse_;

//     ConditionalFlag_Gadget(ProtoboardPtr pb,
//                            const LinearCombination& condition,
//                            const FlagVariable& flag);

//     virtual void init() {}
//     DISALLOW_COPY_AND_ASSIGN(ConditionalFlag_Gadget);
// public:
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const LinearCombination& condition,
//                             const FlagVariable& flag);

//     void generateConstraints();
//     void generateWitness();
// };

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                  LogicImplication_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /// A gadget for the following semantics:
// /// condition == 1 --> flag = 1
// /// Uses 1 constraint

// class LogicImplication_Gadget : public Gadget {
// private:
//     FlagVariable flag_;
//     LinearCombination condition_;

//     LogicImplication_Gadget(ProtoboardPtr pb,
//                             const LinearCombination& condition,
//                             const FlagVariable& flag);

//     virtual void init() {}
//     DISALLOW_COPY_AND_ASSIGN(LogicImplication_Gadget);
// public:
//     static GadgetPtr create(ProtoboardPtr pb,
//                             const LinearCombination& condition,
//                             const FlagVariable& flag);

//     void generateConstraints();
//     void generateWitness();
// };

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                        Compare_Gadget                      ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// // TODO create unit test
// CREATE_GADGET_BASE_CLASS(Comparison_GadgetBase);

// class R1P_Comparison_Gadget : public Comparison_GadgetBase, public R1P_Gadget {
// private:
//     const size_t wordBitSize_;
//     const PackedWord lhs_;
//     const PackedWord rhs_;
//     const FlagVariable less_;
//     const FlagVariable lessOrEqual_;
// 	const PackedWord alpha_p_;
// 	UnpackedWord alpha_u_;
//     const FlagVariable notAllZeroes_;
//     GadgetPtr allZeroesTest_;
//     GadgetPtr alphaDualVariablePacker_;

//     R1P_Comparison_Gadget(ProtoboardPtr pb,
//                           const size_t& wordBitSize,
//                           const PackedWord& lhs,
//                           const PackedWord& rhs,
//                           const FlagVariable& less,
//                           const FlagVariable& lessOrEqual);
//     virtual void init();
// public:

// 	static GadgetPtr create(ProtoboardPtr pb,
// 							const size_t& wordBitSize,
// 							const PackedWord& lhs,
// 							const PackedWord& rhs,
// 							const FlagVariable& less,
// 							const FlagVariable& lessOrEqual);

//     void generateConstraints();
//     void generateWitness();
//     friend class Comparison_Gadget;
// private:
//     DISALLOW_COPY_AND_ASSIGN(R1P_Comparison_Gadget);
// };

// CREATE_GADGET_FACTORY_CLASS_5(Comparison_Gadget, // TODO uncomment this
//                               size_t, wordBitSize,
//                               PackedWord, lhs,
//                               PackedWord, rhs,
//                               FlagVariable, less,
//                               FlagVariable, lessOrEqual);

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// } // namespace gadgetlib2

// //#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_GADGET_HPP_
// /** @file
//  *****************************************************************************
//  Declarations of the interfaces and basic gadgets for R1P (Rank 1 prime characteristic)
//  constraint systems.

//  See details in gadget.hpp .
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// use  <cmath>
// use  <memory>

// use crate::gadgetlib2::gadget;

// using ::std::shared_ptr;
// using ::std::string;
// using ::std::vector;
// using ::std::cout;
// using ::std::cerr;
// using ::std::endl;

// namespace gadgetlib2 {

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                      Gadget Interfaces                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /***********************************/
// /***          Gadget             ***/
// /***********************************/

// Gadget::Gadget(ProtoboardPtr pb) : pb_(pb) {
//     GADGETLIB_ASSERT(pb != NULL, "Attempted to create gadget with uninitialized Protoboard.");
// }

// void Gadget::generateWitness() {
//     GADGETLIB_FATAL("Attempted to generate witness for an incomplete Gadget type.");
// }

// void Gadget::addUnaryConstraint(const LinearCombination& a, const ::std::string& name) {
//     pb_->addUnaryConstraint(a, name);
// }

// void Gadget::addRank1Constraint(const LinearCombination& a,
//                                 const LinearCombination& b,
//                                 const LinearCombination& c,
//                                 const ::std::string& name) {
//     pb_->addRank1Constraint(a, b, c, name);
// }

// /***********************************/
// /***        R1P_Gadget           ***/
// /***********************************/
// R1P_Gadget::~R1P_Gadget() {};

// void R1P_Gadget::addRank1Constraint(const LinearCombination& a,
//                                     const LinearCombination& b,
//                                     const LinearCombination& c,
//                                     const string& name) {
//     pb_->addRank1Constraint(a,b,c, name);
// }

// /***********************************/
// /***  End of Gadget Interfaces   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                      AND Gadgets                           ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// AND_GadgetBase::~AND_GadgetBase() {};

// /*
//     Constraint breakdown:
//     (1) input1 * input2 = result
// */
// BinaryAND_Gadget::BinaryAND_Gadget(ProtoboardPtr pb,
//                                    const LinearCombination& input1,
//                                    const LinearCombination& input2,
//                                    const Variable& result)
//         : Gadget(pb), AND_GadgetBase(pb), input1_(input1), input2_(input2), result_(result) {}

// void BinaryAND_Gadget::init() {}

// void BinaryAND_Gadget::generateConstraints() {
//     addRank1Constraint(input1_, input2_, result_, "result = AND(input1, input2)");
// }

// void BinaryAND_Gadget::generateWitness() {
//     if val(input1_) == 1 && val(input2_) == 1 {
//         val(result_) = 1;
//     } else {
//         val(result_) = 0;
//     }
// }

// /*
//     Constraint breakdown:

//     (*) sum = sum(input[i]) - n
//     (1) sum * result = 0
//     (2) sum * sumInverse = 1 - result

//     [ AND(inputs) == 1 ] (*)==> [sum == 0] (2)==> [result == 1]
//     [ AND(inputs) == 0 ] (*)==> [sum != 0] (1)==> [result == 0]
// */

// R1P_AND_Gadget::R1P_AND_Gadget(ProtoboardPtr pb,
//                                const VariableArray &input,
//                                const Variable &result)
//     : Gadget(pb), AND_GadgetBase(pb), R1P_Gadget(pb), input_(input), result_(result),
//       sumInverse_("sumInverse") {
//     GADGETLIB_ASSERT(input.size() > 0, "Attempted to create an R1P_AND_Gadget with 0 inputs.");
//     GADGETLIB_ASSERT(input.size() <= Fp(-1).as_ulong(), "Attempted to create R1P_AND_Gadget with too "
//                                                               "many inputs. Will cause overflow!");
// }

// void R1P_AND_Gadget::init() {
//     const int numInputs = input_.size();
//     sum_ = sum(input_) - numInputs;
// }

// void R1P_AND_Gadget::generateConstraints() {
//     addRank1Constraint(sum_, result_, 0,
//                       "sum * result = 0 | sum == sum(input[i]) - n");
//     addRank1Constraint(sumInverse_, sum_, 1-result_,
//                       "sumInverse * sum = 1-result | sum == sum(input[i]) - n");
// }

// void R1P_AND_Gadget::generateWitness() {
//     FElem sum = 0;
//     for(size_t i = 0; i < input_.size(); ++i) {
//         sum += val(input_[i]);
//     }
//     sum -= input_.size(); // sum(input[i]) - n ==> sum
//     if sum == 0) { // AND(input[0], input[1], ... == 1
//         val(sumInverse_) = 0;
//         val(result_) = 1;
//     } else {                   // AND(input[0], input[1], ...) == 0
//         val(sumInverse_) = sum.inverse(R1P);
//         val(result_) = 0;
//     }
// }

// GadgetPtr AND_Gadget::create(ProtoboardPtr pb, const VariableArray& input, const Variable& result){
//     GadgetPtr pGadget;
//     if pb->fieldType_ == R1P {
//         pGadget.reset(new R1P_AND_Gadget(pb, input, result));
//     } else {
//         GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
//     }
//         pGadget->init();
//     return pGadget;
// }

// GadgetPtr AND_Gadget::create(ProtoboardPtr pb,
//                              const LinearCombination& input1,
//                              const LinearCombination& input2,
//                              const Variable& result) {
//     GadgetPtr pGadget(new BinaryAND_Gadget(pb, input1, input2, result));
//     pGadget->init();
//     return pGadget;
// }

// /***********************************/
// /***     End of AND Gadgets      ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                      OR Gadgets                            ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// OR_GadgetBase::~OR_GadgetBase() {};

// /*
//     Constraint breakdown:
//     (1) result = input1 + input2 - input1 * input2
//         input1 * input2 = input1 + input2 - result
// */
// BinaryOR_Gadget::BinaryOR_Gadget(ProtoboardPtr pb,
//                                  const LinearCombination& input1,
//                                  const LinearCombination& input2,
//                                  const Variable& result)
//         : Gadget(pb), OR_GadgetBase(pb), input1_(input1), input2_(input2), result_(result) {}

// void BinaryOR_Gadget::init() {}

// void BinaryOR_Gadget::generateConstraints() {
//     addRank1Constraint(input1_, input2_, input1_ + input2_ - result_,
//                        "result = OR(input1, input2)");
// }

// void BinaryOR_Gadget::generateWitness() {
//     if val(input1_) == 1 || val(input2_) == 1 {
//         val(result_) = 1;
//     } else {
//         val(result_) = 0;
//     }
// }

// /*
//     Constraint breakdown:

//     (*) sum = sum(input[i])
//     (1) sum * (1 - result) = 0
//     (2) sum * sumInverse = result

//     [ OR(inputs) == 1 ] (*)==> [sum != 0] (1)==> [result == 1]
//     [ OR(inputs) == 0 ] (*)==> [sum == 0] (2)==> [result == 0]
// */

// R1P_OR_Gadget::R1P_OR_Gadget(ProtoboardPtr pb,
//                              const VariableArray &input,
//                              const Variable &result)
//         : Gadget(pb), OR_GadgetBase(pb), R1P_Gadget(pb), sumInverse_("sumInverse"), input_(input),
//           result_(result) {
//     GADGETLIB_ASSERT(input.size() > 0, "Attempted to create an R1P_OR_Gadget with 0 inputs.");
//     GADGETLIB_ASSERT(input.size() <= Fp(-1).as_ulong(), "Attempted to create R1P_OR_Gadget with too "
//                                                               "many inputs. Will cause overflow!");

//     }

// void R1P_OR_Gadget::init() {
//     sum_ = sum(input_);
// }

// void R1P_OR_Gadget::generateConstraints() {
//     addRank1Constraint(sum_, 1 - result_, 0,
//                        "sum * (1 - result) = 0 | sum == sum(input[i])");
//     addRank1Constraint(sumInverse_, sum_, result_,
//                        "sum * sumInverse = result | sum == sum(input[i])");
// }

// void R1P_OR_Gadget::generateWitness() {
//     FElem sum = 0;
//     for(size_t i = 0; i < input_.size(); ++i) { // sum(input[i]) ==> sum
//         sum += val(input_[i]);
//     }
//     if sum == 0) { // OR(input[0], input[1], ... == 0
//         val(sumInverse_) = 0;
//         val(result_) = 0;
//     } else {                   // OR(input[0], input[1], ...) == 1
//         val(sumInverse_) = sum.inverse(R1P);
//         val(result_) = 1;
//     }
// }

// GadgetPtr OR_Gadget::create(ProtoboardPtr pb, const VariableArray& input, const Variable& result) {
//     GadgetPtr pGadget;
//     if pb->fieldType_ == R1P {
//         pGadget.reset(new R1P_OR_Gadget(pb, input, result));
//     } else {
//         GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
//     }
//         pGadget->init();
//     return pGadget;
// }

// GadgetPtr OR_Gadget::create(ProtoboardPtr pb,
//                             const LinearCombination& input1,
//                             const LinearCombination& input2,
//                             const Variable& result) {
//     GadgetPtr pGadget(new BinaryOR_Gadget(pb, input1, input2, result));
//     pGadget->init();
//     return pGadget;
// }

// /***********************************/
// /***     End of OR Gadgets       ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 InnerProduct Gadgets                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// InnerProduct_GadgetBase::~InnerProduct_GadgetBase() {};

// /*
//     Constraint breakdown:

//     (1) partialSums[0] = A[0] * B[0]
//     (2) partialSums[i] = partialSums[i-1] + A[0] * B[0] ==>                     i = 1..n-2
//         partialSums[i] - partialSums[i-1] = A[i] * B[i]
//     (3) result = partialSums[n-1] = partialSums[n-2] + A[n-1] * B[n-1] ==>
//         result - partialSums[n-2] = A[n-1] * B[n-1]

// */

// R1P_InnerProduct_Gadget::R1P_InnerProduct_Gadget(ProtoboardPtr pb,
//                                                  const VariableArray& A,
//                                                  const VariableArray& B,
//                                                  const Variable& result)
//         : Gadget(pb), InnerProduct_GadgetBase(pb), R1P_Gadget(pb), partialSums_(A.size(),
//           "partialSums"), A_(A), B_(B), result_(result) {
//     GADGETLIB_ASSERT(A.size() > 0, "Attempted to create an R1P_InnerProduct_Gadget with 0 inputs.");
//     GADGETLIB_ASSERT(A.size() == B.size(), GADGETLIB2_FMT("Inner product vector sizes not equal. Sizes are: "
//                                                         "(A) - %u, (B) - %u", A.size(), B.size()));
// }

// void R1P_InnerProduct_Gadget::init() {}

// void R1P_InnerProduct_Gadget::generateConstraints() {
//     const int n = A_.size();
//     if n == 1 {
//         addRank1Constraint(A_[0], B_[0], result_, "A[0] * B[0] = result");
//         return;
//     }
//     // else (n > 1)
//     addRank1Constraint(A_[0], B_[0], partialSums_[0], "A[0] * B[0] = partialSums[0]");
//     for(int i = 1; i <= n-2; ++i) {
//         addRank1Constraint(A_[i], B_[i], partialSums_[i] - partialSums_[i-1],
//             GADGETLIB2_FMT("A[%u] * B[%u] = partialSums[%u] - partialSums[%u]", i, i, i, i-1));
//     }
//     addRank1Constraint(A_[n-1], B_[n-1], result_ - partialSums_[n-2],
//         "A[n-1] * B[n-1] = result - partialSums[n-2]");
// }

// void R1P_InnerProduct_Gadget::generateWitness() {
//     const int n = A_.size();
//     if n == 1 {
//         val(result_) = val(A_[0]) * val(B_[0]);
//         return;
//     }
//     // else (n > 1)
//     val(partialSums_[0]) = val(A_[0]) * val(B_[0]);
//     for(int i = 1; i <= n-2; ++i) {
//         val(partialSums_[i]) = val(partialSums_[i-1]) + val(A_[i]) * val(B_[i]);
//     }
//     val(result_) = val(partialSums_[n-2]) + val(A_[n-1]) * val(B_[n-1]);
// }

// /***********************************/
// /*** End of InnerProduct Gadgets ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   LooseMUX Gadgets                         ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// LooseMUX_GadgetBase::~LooseMUX_GadgetBase() {};

// /*
//     Constraint breakdown:
//     (1) indicators[i] * (index - i) = 0  | i = 0..n-1 ==> only indicators[index] will be non-zero
//     (2) sum(indicators[i]) = successFlag ==> successFlag = indicators[index]
//     (3) successFlag is boolean
//     (4) result[j] = <indicators> * <inputs[index][j]>  |  j = 1..output.size()   ==>
//         result[j] = inputs[index][j]

// */

// R1P_LooseMUX_Gadget::R1P_LooseMUX_Gadget(ProtoboardPtr pb,
//                                          const MultiPackedWordArray& inputs,
//                                          const Variable& index,
//                                          const VariableArray& output,
//                                          const Variable& successFlag)
//         : Gadget(pb), LooseMUX_GadgetBase(pb), R1P_Gadget(pb),
//           indicators_(inputs.size(), "indicators"), inputs_(inputs.size()), index_(index),
//           output_(output), successFlag_(successFlag) {
//     GADGETLIB_ASSERT(inputs.size() <= Fp(-1).as_ulong(), "Attempted to create R1P_LooseMUX_Gadget "
//                                                       "with too many inputs. May cause overflow!");
// //    for(const VariableArray& inpArr : inputs) {
//     for(size_t i = 0; i < inputs.size(); ++i) {
//         GADGETLIB_ASSERT(inputs[i].size() == output.size(), "Input VariableArray is of incorrect size.");
//     }
//     ::std::copy(inputs.begin(), inputs.end(), inputs_.begin()); // change type to R1P_VariableArray
// }

// void R1P_LooseMUX_Gadget::init() {
//     // create inputs for the inner products and initialize them. Each iteration creates a
//     // VariableArray for the i'th elements from each of the vector's VariableArrays.
//     for(size_t i = 0; i < output_.size(); ++i) {
//         VariableArray curInput;
//         for(size_t j = 0; j < inputs_.size(); ++j) {
//             curInput.push_back(inputs_[j][i]);
//         }
//         computeResult_.push_back(InnerProduct_Gadget::create(pb_, indicators_, curInput,
//                                                              output_[i]));
//     }
// }

// void R1P_LooseMUX_Gadget::generateConstraints() {
//     const size_t n = inputs_.size();
//     for(size_t i = 0; i < n; ++i) {
//         addRank1Constraint(indicators_[i], (index_-i), 0,
//             GADGETLIB2_FMT("indicators[%u] * (index - %u) = 0", i, i));
//     }
//     addRank1Constraint(sum(indicators_), 1, successFlag_, "sum(indicators) * 1 = successFlag");
//     enforceBooleanity(successFlag_);
//     for(auto& curGadget : computeResult_) {
//         curGadget->generateConstraints();
//     }
// }

// void R1P_LooseMUX_Gadget::generateWitness() {
//     const size_t n = inputs_.size();
//     /* assumes that idx can be fit in ulong; true for our purposes for now */
//     const size_t index = val(index_).asLong();
//     const FElem arraySize = n;
//     for(size_t i = 0; i < n; ++i) {
//         val(indicators_[i]) = 0; // Redundant, but just in case.
//     }
//     if index >= n { //  || index < 0
//         val(successFlag_) = 0;
//     } else { // index in bounds
//         val(indicators_[index]) = 1;
//         val(successFlag_) = 1;
//     }
//     for(auto& curGadget : computeResult_) {
//         curGadget->generateWitness();
//     }
// }

// VariableArray R1P_LooseMUX_Gadget::indicatorVariables() const {return indicators_;}

// GadgetPtr LooseMUX_Gadget::create(ProtoboardPtr pb,
//                                   const MultiPackedWordArray& inputs,
//                                   const Variable& index,
//                                   const VariableArray& output,
//                                   const Variable& successFlag) {
//     GadgetPtr pGadget;
//     if pb->fieldType_ == R1P {
//         pGadget.reset(new R1P_LooseMUX_Gadget(pb, inputs, index, output, successFlag));
//     } else {
//         GADGETLIB_FATAL("Attempted to create gadget of undefined Protoboard type.");
//     }
//     pGadget->init();
//     return pGadget;
// }

// /**
//     An overload for the private case in which we only want to multiplex one Variable. This is
//     usually the case in R1P.
// **/
// GadgetPtr LooseMUX_Gadget::create(ProtoboardPtr pb,
//                                   const VariableArray& inputs,
//                                   const Variable& index,
//                                   const Variable& output,
//                                   const Variable& successFlag) {
//     MultiPackedWordArray inpVec;
//     for(size_t i = 0; i < inputs.size(); ++i) {
//         MultiPackedWord cur(pb->fieldType_);
//         cur.push_back(inputs[i]);
//         inpVec.push_back(cur);
//     }
//     VariableArray outVec;
//     outVec.push_back(output);
//     auto result = LooseMUX_Gadget::create(pb, inpVec, index, outVec, successFlag);
//     return result;
// }

// /***********************************/
// /***   End of LooseMUX Gadgets   ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************               CompressionPacking Gadgets                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     Compression Packing gadgets have two modes, which differ in the way the witness and constraints
//     are created. In PACK mode  gerateWitness() will take the bits and create a packed element (or
//     number of elements) while generateConstraints() will not enforce that bits are indeed Boolean.
//     In UNPACK mode generateWitness() will take the packed representation and unpack it to bits while
//     generateConstraints will in addition enforce that unpacked bits are indeed Boolean.
// */

// CompressionPacking_GadgetBase::~CompressionPacking_GadgetBase() {};

// /*
//     Constraint breakdown:

//     (1) packed = sum(unpacked[i] * 2^i)
//     (2) (UNPACK only) unpacked[i] is Boolean.
// */

// R1P_CompressionPacking_Gadget::R1P_CompressionPacking_Gadget(ProtoboardPtr pb,
//                                                              const VariableArray& unpacked,
//                                                              const VariableArray& packed,
//                                                              PackingMode packingMode)
//     : Gadget(pb), CompressionPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
//       unpacked_(unpacked), packed_(packed) {
//     const int n = unpacked.size();
//     GADGETLIB_ASSERT(n > 0, "Attempted to pack 0 bits in R1P.")
//     GADGETLIB_ASSERT(packed.size() == 1,
//                  "Attempted to pack into more than 1 Variable in R1P_CompressionPacking_Gadget.")
//     // TODO add assertion that 'n' bits can fit in the field characteristic
// }

// void R1P_CompressionPacking_Gadget::init() {}

// void R1P_CompressionPacking_Gadget::generateConstraints() {
//     const int n = unpacked_.size();
//     LinearCombination packed;
//     FElem two_i(R1P_Elem(1)); // Will hold 2^i
//     for i in 0..n {
//         packed += unpacked_[i]*two_i;
//         two_i += two_i;
//         if packingMode_ == PackingMode::UNPACK) {enforceBooleanity(unpacked_[i];}
//     }
//     addRank1Constraint(packed_[0], 1, packed, "packed[0] = sum(2^i * unpacked[i])");
// }

// void R1P_CompressionPacking_Gadget::generateWitness() {
//     const int n = unpacked_.size();
//     if packingMode_ == PackingMode::PACK {
//         FElem packedVal = 0;
//         FElem two_i(R1P_Elem(1)); // will hold 2^i
//         for(int i = 0; i < n; ++i) {
//             GADGETLIB_ASSERT(val(unpacked_[i]).asLong() == 0 || val(unpacked_[i]).asLong() == 1,
//                          GADGETLIB2_FMT("unpacked[%u]  = %u. Expected a Boolean value.", i,
//                              val(unpacked_[i]).asLong()));
//             packedVal += two_i * val(unpacked_[i]).asLong();
//             two_i += two_i;
//         }
//         val(packed_[0]) = packedVal;
//         return;
//     }
//     // else (UNPACK)
//     GADGETLIB_ASSERT(packingMode_ == PackingMode::UNPACK, "Packing gadget created with unknown packing mode.");
//     for(int i = 0; i < n; ++i) {
//         val(unpacked_[i]) = val(packed_[0]).getBit(i, R1P);
//     }
// }

// /*****************************************/
// /*** End of CompressionPacking Gadgets ***/
// /*****************************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                IntegerPacking Gadgets                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     Arithmetic Packing gadgets have two modes, which differ in the way the witness and constraints
//     are created. In PACK mode  gerateWitness() will take the bits and create a packed element (or
//     number of elements) while generateConstraints() will not enforce that bits are indeed Boolean.
//     In UNPACK mode generateWitness() will take the packed representation and unpack it to bits while
//     generateConstraints will in addition enforce that unpacked bits are indeed Boolean.
// */

// IntegerPacking_GadgetBase::~IntegerPacking_GadgetBase() {};

// /*
//     Constraint breakdown:

//     (1) packed = sum(unpacked[i] * 2^i)
//     (2) (UNPACK only) unpacked[i] is Boolean.
// */

// R1P_IntegerPacking_Gadget::R1P_IntegerPacking_Gadget(ProtoboardPtr pb,
//                                                            const VariableArray& unpacked,
//                                                            const VariableArray& packed,
//                                                            PackingMode packingMode)
//     : Gadget(pb), IntegerPacking_GadgetBase(pb), R1P_Gadget(pb), packingMode_(packingMode),
//       unpacked_(unpacked), packed_(packed) {
//     const int n = unpacked.size();
//     GADGETLIB_ASSERT(n > 0, "Attempted to pack 0 bits in R1P.")
//     GADGETLIB_ASSERT(packed.size() == 1,
//                  "Attempted to pack into more than 1 Variable in R1P_IntegerPacking_Gadget.")
// }

// void R1P_IntegerPacking_Gadget::init() {
//     compressionPackingGadget_ = CompressionPacking_Gadget::create(pb_, unpacked_, packed_,
//                                                                   packingMode_);
// }

// void R1P_IntegerPacking_Gadget::generateConstraints() {
//     compressionPackingGadget_->generateConstraints();
// }

// void R1P_IntegerPacking_Gadget::generateWitness() {
//     compressionPackingGadget_->generateWitness();
// }


// /*****************************************/
// /*** End of IntegerPacking Gadgets  ***/
// /*****************************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 EqualsConst Gadgets                        ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// EqualsConst_GadgetBase::~EqualsConst_GadgetBase() {};

// /*
//     Constraint breakdown:

//     (1) (input - n) * result = 0
//     (2) (input - n) * aux = 1 - result

//     [ input == n ] (2)==> [result == 1]    (aux can ake any value)
//     [ input != n ] (1)==> [result == 0]    (aux == inverse(input - n))
// */

// R1P_EqualsConst_Gadget::R1P_EqualsConst_Gadget(ProtoboardPtr pb,
//                                                const FElem& n,
//                                                const LinearCombination &input,
//                                                const Variable &result)
//         : Gadget(pb), EqualsConst_GadgetBase(pb), R1P_Gadget(pb), n_(n),
//           aux_("aux (R1P_EqualsConst_Gadget)"), input_(input), result_(result) {}

// void R1P_EqualsConst_Gadget::init() {}

// void R1P_EqualsConst_Gadget::generateConstraints() {
//     addRank1Constraint(input_ - n_, result_, 0, "(input - n) * result = 0");
//     addRank1Constraint(input_ - n_, aux_, 1 - result_, "(input - n) * aux = 1 - result");
// }

// void R1P_EqualsConst_Gadget::generateWitness() {
//     val(aux_) = val(input_) == n_ ? 0 : (val(input_) - n_).inverse(R1P) ;
//     val(result_) = val(input_) == n_ ? 1 : 0 ;
// }

// /***********************************/
// /*** End of EqualsConst Gadgets  ***/
// /***********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   DualWord_Gadget                      ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// DualWord_Gadget::DualWord_Gadget(ProtoboardPtr pb,
//                                          const DualWord& var,
//                                          PackingMode packingMode)
//         : Gadget(pb), var_(var), packingMode_(packingMode), packingGadget_() {}

// void DualWord_Gadget::init() {
//     packingGadget_ = CompressionPacking_Gadget::create(pb_, var_.unpacked(), var_.multipacked(),
//                                                         packingMode_);
// }

// GadgetPtr DualWord_Gadget::create(ProtoboardPtr pb,
//                                       const DualWord& var,
//                                       PackingMode packingMode) {
//     GadgetPtr pGadget(new DualWord_Gadget(pb, var, packingMode));
//     pGadget->init();
//     return pGadget;
// }

// void DualWord_Gadget::generateConstraints() {
//     packingGadget_->generateConstraints();
// }

// void DualWord_Gadget::generateWitness() {
//     packingGadget_->generateWitness();
// }

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                 DualWordArray_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
// DualWordArray_Gadget::DualWordArray_Gadget(ProtoboardPtr pb,
//                                            const DualWordArray& vars,
//                                            PackingMode packingMode)
//         : Gadget(pb), vars_(vars), packingMode_(packingMode), packingGadgets_() {}

// void DualWordArray_Gadget::init() {
//     const UnpackedWordArray unpacked = vars_.unpacked();
//     const MultiPackedWordArray packed = vars_.multipacked();
//     for(size_t i = 0; i < vars_.size(); ++i) {
//         const auto curGadget = CompressionPacking_Gadget::create(pb_, unpacked[i], packed[i],
//                                                                  packingMode_);
//         packingGadgets_.push_back(curGadget);
//     }
// }

// GadgetPtr DualWordArray_Gadget::create(ProtoboardPtr pb,
//                                            const DualWordArray& vars,
//                                            PackingMode packingMode) {
//     GadgetPtr pGadget(new DualWordArray_Gadget(pb, vars, packingMode));
//     pGadget->init();
//     return pGadget;
// }

// void DualWordArray_Gadget::generateConstraints() {
//     for(auto& gadget : packingGadgets_) {
//         gadget->generateConstraints();
//     }
// }

// void DualWordArray_Gadget::generateWitness() {
//     for(auto& gadget : packingGadgets_) {
//         gadget->generateWitness();
//     }
// }

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                        Toggle_Gadget                       ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     Constraint breakdown:

//     (1) result = (1 - toggle) * zeroValue + toggle * oneValue
//         (rank 1 format) ==> toggle * (oneValue - zeroValue) = result - zeroValue

// */

// Toggle_Gadget::Toggle_Gadget(ProtoboardPtr pb,
//                              const FlagVariable& toggle,
//                              const LinearCombination& zeroValue,
//                              const LinearCombination& oneValue,
//                              const Variable& result)
//         : Gadget(pb), toggle_(toggle), zeroValue_(zeroValue), oneValue_(oneValue),
//           result_(result) {}

// GadgetPtr Toggle_Gadget::create(ProtoboardPtr pb,
//                                 const FlagVariable& toggle,
//                                 const LinearCombination& zeroValue,
//                                 const LinearCombination& oneValue,
//                                 const Variable& result) {
//     GadgetPtr pGadget(new Toggle_Gadget(pb, toggle, zeroValue, oneValue, result));
//     pGadget->init();
//     return pGadget;
// }

// void Toggle_Gadget::generateConstraints() {
//     pb_->addRank1Constraint(toggle_, oneValue_ - zeroValue_, result_ - zeroValue_,
//                             "result = (1 - toggle) * zeroValue + toggle * oneValue");
// }

// void Toggle_Gadget::generateWitness() {
//     if val(toggle_) == 0 {
//         val(result_) = val(zeroValue_);
//     } else if val(toggle_) == 1 {
//         val(result_) = val(oneValue_);
//     } else {
//         GADGETLIB_FATAL("Toggle value must be Boolean.");
//     }
// }


// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   ConditionalFlag_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     semantics: condition != 0 --> flag = 1
//                condition == 0 --> flag = 0

//     Constraint breakdown:
//     (1) condition * not(flag) = 0
//     (2) condition * auxConditionInverse = flag

//  */

// ConditionalFlag_Gadget::ConditionalFlag_Gadget(ProtoboardPtr pb,
//                                                const LinearCombination& condition,
//                                                const FlagVariable& flag)
//         : Gadget(pb), flag_(flag), condition_(condition),
//           auxConditionInverse_("ConditionalFlag_Gadget::auxConditionInverse_") {}

// GadgetPtr ConditionalFlag_Gadget::create(ProtoboardPtr pb,
//                                          const LinearCombination& condition,
//                                          const FlagVariable& flag) {
//     GadgetPtr pGadget(new ConditionalFlag_Gadget(pb, condition, flag));
//     pGadget->init();
//     return pGadget;
// }

// void ConditionalFlag_Gadget::generateConstraints() {
//     pb_->addRank1Constraint(condition_, negate(flag_), 0, "condition * not(flag) = 0");
//     pb_->addRank1Constraint(condition_, auxConditionInverse_, flag_,
//                             "condition * auxConditionInverse = flag");
// }

// void ConditionalFlag_Gadget::generateWitness() {
//     if val(condition_) == 0 {
//         val(flag_) = 0;
//         val(auxConditionInverse_) = 0;
//     } else {
//         val(flag_) = 1;
//         val(auxConditionInverse_) = val(condition_).inverse(fieldType());
//     }
// }

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                  LogicImplication_Gadget                   ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// /*
//     semantics: condition == 1 --> flag = 1

//     Constraint breakdown:
//     (1) condition * (1 - flag) = 0

//  */

// LogicImplication_Gadget::LogicImplication_Gadget(ProtoboardPtr pb,
//                                                  const LinearCombination& condition,
//                                                  const FlagVariable& flag)
//     : Gadget(pb), flag_(flag), condition_(condition) {}

// GadgetPtr LogicImplication_Gadget::create(ProtoboardPtr pb,
//                                           const LinearCombination& condition,
//                                           const FlagVariable& flag) {
//     GadgetPtr pGadget(new LogicImplication_Gadget(pb, condition, flag));
//     pGadget->init();
//     return pGadget;
// }

// void LogicImplication_Gadget::generateConstraints() {
//     pb_->addRank1Constraint(condition_, negate(flag_), 0, "condition * not(flag) = 0");
// }

// void LogicImplication_Gadget::generateWitness() {
//     if val(condition_) == 1 {
//         val(flag_) = 1;
//     }
// }

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                        Compare_Gadget                      ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

// Comparison_GadgetBase::~Comparison_GadgetBase() {}

// R1P_Comparison_Gadget::R1P_Comparison_Gadget(ProtoboardPtr pb,
//                                              const size_t& wordBitSize,
//                                              const PackedWord& lhs,
//                                              const PackedWord& rhs,
//                                              const FlagVariable& less,
//                                              const FlagVariable& lessOrEqual)
//         : Gadget(pb), Comparison_GadgetBase(pb), R1P_Gadget(pb), wordBitSize_(wordBitSize),
//           lhs_(lhs), rhs_(rhs), less_(less), lessOrEqual_(lessOrEqual),
//           alpha_u_(wordBitSize,  "alpha"), notAllZeroes_("notAllZeroes") {}

// void R1P_Comparison_Gadget::init() {
//     allZeroesTest_ = OR_Gadget::create(pb_, alpha_u_, notAllZeroes_);
// 	alpha_u_.push(lessOrEqual_);
// 	alphaDualVariablePacker_ = CompressionPacking_Gadget::create(pb_, alpha_u_,VariableArray(1,alpha_p_), PackingMode::UNPACK);
// }
// /*
//     Constraint breakdown:

//     for succinctness we shall define:
//     (1) wordBitSize == n
//     (2) lhs == A
//     (3) rhs == B

//     packed(alpha) = 2^n + B - A
//     not_all_zeros = OR(alpha.unpacked)

//     if B - A > 0, then: alpha > 2^n,
//     so alpha[n] = 1 and notAllZeroes = 1
//     if B - A = 0, then: alpha = 2^n,
//     so alpha[n] = 1 and notAllZeroes = 0
//     if B - A < 0, then: 0 <= alpha <= 2^n-1
//     so alpha[n] = 0

//     therefore:
//     (1) alpha[n] = lessOrEqual
//     (2) alpha[n] * notAllZeroes = less


// */
// void R1P_Comparison_Gadget::generateConstraints() {
//     enforceBooleanity(notAllZeroes_);
//     const FElem two_n = long(POW2(wordBitSize_));
//     addRank1Constraint(1, alpha_p_, two_n + rhs_ - lhs_,
// 							 "packed(alpha) = 2^n + B - A");
//     alphaDualVariablePacker_->generateConstraints();
//     allZeroesTest_->generateConstraints();
//     addRank1Constraint(1, alpha_u_[wordBitSize_], lessOrEqual_, "alpha[n] = lessOrEqual");
//     addRank1Constraint(alpha_u_[wordBitSize_], notAllZeroes_, less_,
//                        "alpha[n] * notAllZeroes = less");
// }

// void R1P_Comparison_Gadget::generateWitness() {
//     const FElem two_n = long(POW2(wordBitSize_));
//     val(alpha_p_) = two_n + val(rhs_) - val(lhs_);
//     alphaDualVariablePacker_->generateWitness();
//     allZeroesTest_->generateWitness();
//     val(lessOrEqual_) = val(alpha_u_[wordBitSize_]);
//     val(less_) = val(lessOrEqual_) * val(notAllZeroes_);
// }

// /*********************************/
// /***       END OF Gadget       ***/
// /*********************************/

// } // namespace gadgetlib2
