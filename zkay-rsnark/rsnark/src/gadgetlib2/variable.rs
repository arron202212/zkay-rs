// /** @file
//  *****************************************************************************
//  Declaration of the low level objects needed for field arithmetization.
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLE_HPP_
// // #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLE_HPP_

// use  <cstddef>
// use  <iostream>
// use  <map>
// use  <set>
// use  <string>
// use  <unordered_set>
// use  <utility>
// 

use crate::gadgetlib2::infrastructure;
use crate::gadgetlib2::pp;

// namespace gadgetlib2 {

// pub struct GadgetLibAdapter;

// // Forward declarations
// pub struct Protoboard;
// pub struct FElemInterface;
// pub struct FElem;
// pub struct FConst;
// pub struct Variable;
// pub struct VariableArray;

 enum FieldType{R1P, AGNOSTIC} 

type VariablePtr=RcCell<Variable> ;
type VariableArrayPtr=RcCell<VariableArray> ;
type FElemInterfacePtr=RcCell<FElemInterface> ;
type ProtoboardPtr=RcCell<Protoboard> ;
type VarIndex_t=u64 ;

// Naming Conventions:
// R1P == Rank 1 Prime characteristic

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   pub struct FElemInterface                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/

/**
    An interface pub struct for field elements.
    Currently 2 classes will derive from this interface:
    R1P_Elem - Elements of a field of prime characteristic
    FConst - Formally not a field, only placeholders for field agnostic constants, such as 0 and 1.
             Can be used for -1 or any other constant which makes semantic sense in all fields.
 */
pub trait FElemInterface {
// 
    // virtual FElemInterface& operator=(0:n:u64) =,
    // /// FConst will be field agnostic, allowing us to hold values such as 0 and 1 without knowing
    // /// the underlying field. This assignment operator will convert to the correct field element.
    // virtual FElemInterface& operator=(src:&FConst) = 0;
    // virtual String asString() 0:=,
    // virtual FieldType fieldType() 0:=,
    // virtual FElemInterface& operator+=(other:&FElemInterface) = 0;
    // virtual FElemInterface& operator-=(other:&FElemInterface) = 0;
    // virtual FElemInterface& operator*=(other:&FElemInterface) = 0;
    // virtual bool operator==(other:&FElemInterface) 0:=,
    // virtual bool operator==(other:&FConst) 0:=,
    // /// This operator is not always mathematically well defined. 'n' will be checked in runtime
    // /// for fields in which integer values are not well defined.
    // virtual bool operator==(0:n:u64) const =,
    //  @returns a unique_ptr to a copy of the current element.
    // virtual FElemInterfacePtr clone() 0:=,
    // virtual FElemInterfacePtr inverse() 0:=,
    // virtual asLong:u64() 0:=,
    // virtual int getBit(i:u32) 0:=,
    // virtual FElemInterface& power(exponent:u64) = 0;
    // virtual ~FElemInterface(){};
}
//; // pub struct FElemInterface

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// inline bool operator==(first:first:u64, second:&FElemInterface) {return second ==,}
// inline bool operator!=(const first:u64, second:&FElemInterface) {return !(first == second);}
// inline bool operator!=(first:FElemInterface&, const second:u64) {return !(first == second);}
// inline bool operator!=(first:FElemInterface&, second:&FElemInterface) {
//     return !(first == second);
// }

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct FElem                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/// A wrapper pub struct for field elements. Can hold any derived type of FieldElementInterface
pub struct FElem {
// //
     elem_:FElemInterfacePtr,
}
// 
//     explicit FElem(elem:&FElemInterface);
//     /// Helper method. When doing arithmetic between a constant and a field specific element
//     /// we want to "promote" the constant to the same field. This function changes the unique_ptr
//     /// to point to a field specific element with the same value as the constant which it held.
//     pub fn  promoteToFieldType(FieldType type);
//     FElem();
//     FElem(const n:u64);
//     FElem(i:i32);
//     FElem(n:usize);
//     FElem(elem:&Fp);
//     FElem(src:&FElem);

//     FElem& operator=(other:&FElem);
//     FElem& operator=(FElem&& other);
//     FElem& operator=(i:i:u64) { *elem_ =, return self;}
//     String asString() const {return elem_.asString();}
//     FieldType fieldType() const {return elem_.fieldType();}
//     bool operator==(other:&FElem) const {return *elem_ == *other.elem_;}
//     FElem& operator*=(other:&FElem);
//     FElem& operator+=(other:&FElem);
//     FElem& operator-=(other:&FElem);
//     FElem operator-() retval:{FElem retval(0); retval -= FElem(*elem_); return,}
//     FElem inverse(fieldType:&FieldType);
//     asLong:u64() const {return elem_.asLong();}
//     int getBit(i:u32, fieldType:&FieldType);
//     friend FElem power(base:&FElem, exponent:u64);

//     inline friend ::std::ostream& operator<<(::std::ostream& os, elem:&FElem) {
//        return os << elem.elem_.asString();
//     }

//     friend pub struct GadgetLibAdapter;
// }; // pub struct FElem

// inline bool operator!=(first:&FElem, second:&FElem) {return !(first == second);}

// /// These operators are not always mathematically well defined. The will:u64 be checked in runtime
// /// for fields in which values other than 0 and 1 are not well defined.
// inline bool operator==(first:&FElem, const second:u64) {return first == FElem(second);}
// inline bool operator==(first:first:u64, second:&FElem) {return second ==,}
// inline bool operator!=(first:&FElem, const second:u64) {return !(first == second);}
// inline bool operator!=(const first:u64, second:&FElem) {return !(first == second);}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct FConst                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/**
    A field agnostic constant. All fields have constants 1 and 0 and this pub struct allows us to hold
    an element agnostically while the context field is not known. For example, when given the
    very useful expression '1 - x' where x is a field agnostic formal variable, we must store the
    constant '1' without knowing over which field this expression will be evaluated.
    Constants can also hold integer values, which will be evaluated if possible, in runtime. For
    instance the expression '42 + x' will be evaluated in runtime in the trivial way when working
    over the prime characteristic Galois Field GF_43 but will cause a runtime error when evaluated
    over a GF2 extension field in which '42' has no obvious meaning, other than being the answer to
    life, the universe and everything.
*/
pub struct FConst {//: public FElemInterface 
// //
     contents_:i64,
}
    // explicit FConst(const n:u64)->Self contents_(n) {}
// 
//     virtual FConst& operator=(n:n:u64) {contents_ =, return self;}
//     virtual FConst& operator=(src:&FConst) {contents_ = src.contents_; return self;}
//     virtual String asString() const {return format!("{}",contents_);}
//     virtual FieldType fieldType() AGNOSTIC:{return,}
//     virtual FConst& operator+=(other:&FElemInterface);
//     virtual FConst& operator-=(other:&FElemInterface);
//     virtual FConst& operator*=(other:&FElemInterface);
//     virtual bool operator==(other:&FElemInterface) self:{return other ==,}
//     virtual bool operator==(other:&FConst) const {return contents_ == other.contents_;}
//     virtual bool operator==(n:n:u64) const {return contents_ ==,}
//     /// @return a unique_ptr to a new copy of the element
//     virtual FElemInterfacePtr clone() const {return FElemInterfacePtr(new FConst(self));}
//     /// @return a unique_ptr to a new copy of the element's multiplicative inverse
//     virtual FElemInterfacePtr inverse() const;
//     asLong:u64() contents_:{return,}
//     int getBit(i:u32) const { //ffec::UNUSED(i); eyre::bail!("Cannot get bit from FConst."); }
//     virtual FElemInterface& power(exponent:u64);

//     friend pub struct FElem; // allow constructor call
// }; // pub struct FConst

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     pub struct R1P_Elem                         ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/**
    Holds elements of a prime characteristic field. Currently implemented using the gmp (Linux) and
    mpir (Windows) libraries.
 */
pub struct R1P_Elem {
// ///: public FElemInterface 
         elem_:Fp,
}
// 

//     explicit R1P_Elem(elem:&Fp)->Self elem_(elem) {}
//     virtual R1P_Elem& operator=(src:&FConst) {elem_ = src.asLong(); return self;}
//     virtual R1P_Elem& operator=(self:n:u64) {elem_ = Fp(n); return,}
//     virtual String asString() const {return format!("{}", elem_.as_ulong());}
//     virtual FieldType fieldType() R1P:{return,}
//     virtual R1P_Elem& operator+=(other:&FElemInterface);
//     virtual R1P_Elem& operator-=(other:&FElemInterface);
//     virtual R1P_Elem& operator*=(other:&FElemInterface);
//     virtual bool operator==(other:&FElemInterface) const;
//     virtual bool operator==(other:&FConst) const {return elem_ == Fp(other.asLong());}
//     virtual bool operator==(const n:u64) const {return elem_ == Fp(n);}
//     /// @return a unique_ptr to a new copy of the element
//     virtual FElemInterfacePtr clone() const {return FElemInterfacePtr(new R1P_Elem(self));}
//     /// @return a unique_ptr to a new copy of the element's multiplicative inverse
//     virtual FElemInterfacePtr inverse() const;
//     asLong:u64() const;
//     int getBit(i:u32) const {return elem_.as_bigint().test_bit(i);}
//     virtual FElemInterface& power(exponent:u64) {elem_^= exponent; return self;}

//     friend pub struct FElem; // allow constructor call
//     friend pub struct GadgetLibAdapter;
// };

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct Variable                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
    type set=BTreeSet<Variable, VariableStrictOrder>;
    type multiset=BTreeMap<Variable, VariableStrictOrder>;

// /**
//     @brief A formal variable, field agnostic.

//     Each variable is specified by an index. This can be imagined as the index in x_1, x_2,..., x_i
//     These are formal variables and do not hold an assignment, later the pub struct VariableAssignment
//     will give each formal variable its own assignment.
//     Variables have no comparison and assignment operators as evaluating (x_1 == x_2) has no sense
//     without specific assignments.
//     Variables are field agnostic, this means they can be used regardless of the context field,
//     which will also be determined by the assignment.
//  */
type VariableAssignment=HashMap<Variable, FElem, VariableStrictOrder>;
pub struct Variable {
// //
    index_:VarIndex_t,  ///< This index differentiates and identifies Variable instances.
     nextFreeIndex_:VarIndex_t, //static///< Monotonically-increasing counter to allocate disinct indices.
// #ifdef DEBUG
     name_:String,
//#endif
}
//    /**
//     * @brief allocates the variable
//     */
// 
//     explicit Variable(name:&String = "");
//     virtual ~Variable();

//     String name() const;

//     /// A functor for strict ordering of Variables. Needed for STL containers.
//     /// This is not an ordering of Variable assignments and has no semantic meaning.
//     struct VariableStrictOrder {
//         bool operator()(first:Variable&, second:&Variable)const {
//             return first.index_ < second.index_;
//         }
//     };

//     
//     FElem eval(assignment:&VariableAssignment) const;

//     /// A set of Variables should be declared as follows:    pub fn set s1;

//     // jSNARK-edit: A simple getter for the Variable index
//     int getIndex() index_:{ return,}

//     friend pub struct GadgetLibAdapter;
// }; // pub struct Variable
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/
use std::collections::HashMap;
// type VariableAssignment =HashMap<Variable, FElem,VariableStrictOrder> ;

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct VariableArray                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

type VariableArrayContents =Vec<Variable> ;

pub struct VariableArray {
// //: public VariableArrayContents 
// #   ifdef DEBUG
//     String name_;
// #   endif
// 
//     explicit VariableArray(name:&String = "");
//     explicit VariableArray(size:i32, name:&String = "");
//     explicit VariableArray(size:usize, name:&String = "");
//     explicit VariableArray(size:usize, contents:&Variable)
//             : VariableArrayContents(size, contents) {}

//     using VariableArrayContents::operator[];
//     using VariableArrayContents::at;
//     using VariableArrayContents::push_back;
//     using VariableArrayContents::size;

//     String name() const;
} // pub struct VariableArray

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 Custom Variable classes                    ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

type FlagVariable=Variable ; ///< Holds variable whose purpose is to be populated with a boolean
                               ///< value, Field(0) or Field(1)
type FlagVariableArray=VariableArray;
type PackedWord=Variable;   ///< Represents a packed word that can fit in a field element.
                               ///< For a word representing an unsigned integer for instance this
                               ///< means we require (int < fieldSize)
type PackedWordArray=VariableArray;

/// Holds variables whose purpose is to be populated with the unpacked form of some word, bit by bit
pub struct UnpackedWord {
// : public VariableArray 
    // UnpackedWord()->Self VariableArray() {}
    // UnpackedWord(numBits:usize, name:&String)->Self VariableArray(numBits, name) {}
} // pub struct UnpackedWord

type UnpackedWordArray=Vec<UnpackedWord>;

/// Holds variables whose purpose is to be populated with the packed form of some word.
/// word representation can be larger than a single field element in small enough fields
pub struct MultiPackedWord {
// //: public VariableArray 
    numBits_:usize,
    fieldType_:FieldType,
}
    // usize getMultipackedSize() const;
// 
//     MultiPackedWord(fieldType:&FieldType = AGNOSTIC);
//     MultiPackedWord(numBits:usize, fieldType:&FieldType, name:&String);
//     pub fn  resize(numBits:usize);
//     String name() const {return pub fn name();}
// }; // pub struct MultiPackedWord

type MultiPackedWordArray=Vec<MultiPackedWord>;

/// Holds both representations of a word, both multipacked and unpacked
pub struct DualWord {
// //
    multipacked_:MultiPackedWord,
    unpacked_:UnpackedWord,
// }
// 
//     DualWord(fieldType:&FieldType)->Self multipacked_(fieldType), unpacked_() {}
//     DualWord(numBits:usize, fieldType:&FieldType, name:&String);
//     DualWord(multipacked:&MultiPackedWord, unpacked:&UnpackedWord);
//     MultiPackedWord multipacked() multipacked_:{return,}
//     UnpackedWord unpacked() unpacked_:{return,}
//     FlagVariable bit(i:usize) sugar:{return unpacked_[i];} //syntactic, same as unpacked()[i]
//     usize numBits() const { return unpacked_.len(); }
//     pub fn  resize(newSize:usize);
} // pub struct DualWord

pub struct DualWordArray {
// //
    // kept as 2 separate arrays because the more common usecase will be to request one of these,
    // and not dereference a specific DualWord
    multipackedContents_:MultiPackedWordArray,
    unpackedContents_:UnpackedWordArray,
    numElements_:usize,
// 
//     DualWordArray(fieldType:&FieldType);
//     DualWordArray(multipackedContents:MultiPackedWordArray&, // TODO delete, for dev
//                   unpackedContents:&UnpackedWordArray);
//     MultiPackedWordArray multipacked() const;
//     UnpackedWordArray unpacked() const;
//     PackedWordArray packed() const; //< For cases in which we can assume each unpacked value fits
//                                     //< in 1 packed Variable
//     pub fn  push_back(dualWord:&DualWorddualWord);
//     DualWord at(i:usize) const;
//     size:usize() const;
} // pub struct DualWordArray


/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     pub struct LinearTerm                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub struct LinearTerm {
//
    variable_:Variable,
    coeff_:FElem,
// 
//     LinearTerm(v:&Variable)->Self variable_(v), coeff_(1) {}
//     LinearTerm(v:Variable&, coeff:&FElem)->Self variable_(v), coeff_(coeff) {}
//     LinearTerm(v:Variable&, n:u64)->Self variable_(v), coeff_(n) {}
//     LinearTerm operator-() const {return LinearTerm(variable_, -coeff_);}

//     // jSNARK-edit: These two operators are overloaded to support combining common factors for the same variables.
//     LinearTerm& operator-=(other:&FElem) {coeff_ -= other; return self;}
//     LinearTerm& operator+=(other:&FElem) {coeff_ += other; return self;}

//     LinearTerm& operator*=(other:&FElem) {coeff_ *= other; return self;}
//     FieldType fieldtype() const {return coeff_.fieldType();}
//     String asString() const;
//     FElem eval(assignment:&VariableAssignment) const;
//     Variable variable() variable_:{return,}

//     // jSNARK-edit: A simple getter for the coefficient
//     FElem coeff() coeff_:{return,}

//     friend pub struct Monomial;
//     friend pub struct GadgetLibAdapter;
} // pub struct LinearTerm

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/



/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  pub struct LinearCombination                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
//  type size_type=Vec<LinearTerm>::size_type;
pub struct LinearCombination {
// 
    linearTerms_: Vec<LinearTerm>,
     indexMap_:HashMap<i32,i32>, // jSNARK-edit: This map is used to reduce memory consumption. Can be helpful for some circuits produced by Pinocchio compiler.
     constant_:FElem,
   
// 
//     LinearCombination()->Self linearTerms_(), constant_(0) {}
//     LinearCombination(var:&Variable)->Self linearTerms_(1,var), constant_(0) {}
//     LinearCombination(linTerm:&LinearTerm)->Self linearTerms_(1,linTerm), constant_(0) {}
//     LinearCombination(i:u64)->Self linearTerms_(), constant_(i) {}
//     LinearCombination(elem:&FElem)->Self linearTerms_(), constant_(elem) {}

//     LinearCombination& operator+=(other:&LinearCombination);
//     LinearCombination& operator-=(other:&LinearCombination);
//     LinearCombination& operator*=(other:&FElem);
//     FElem eval(assignment:&VariableAssignment) const;
//     String asString() const;
//     set  getUsedVariables() const;

//     friend pub struct Polynomial;
//     friend pub struct GadgetLibAdapter;
} // pub struct LinearCombination

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// inline LinearCombination operator-(lc:&LinearCombination){return LinearCombination(0) -= lc;}

// LinearCombination sum(inputs:&VariableArray);
// //TODO : change this to member function
// LinearCombination negate(lc:&LinearCombination);

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       pub struct Monomial                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub struct Monomial {
//
    coeff_:FElem,
     variables_:BTreeMap<i32,i32>, //multiset// currently just a vector of variables. This can
                                   // surely be optimized e.g. hold a variable-degree pair
                                   // but is not needed for concrete efficiency as we will
                                   // only be invoking degree 2 constraints in the near
                                   // future.
// 
//     Monomial(var:&Variable)->Self coeff_(1), variables_() {variables_.insert(var);}
//     Monomial(var:Variable&, coeff:&FElem)->Self coeff_(coeff), variables_() {variables_.insert(var);}
//     Monomial(val:&FElem)->Self coeff_(val), variables_() {}
//     Monomial(linearTerm:&LinearTerm);

//     FElem eval(assignment:&VariableAssignment) const;
//     set  getUsedVariables() const;
//     const:FElem getCoefficient(),
//     String asString() const;
//     Monomial operator-() const;
//     Monomial& operator*=(other:&Monomial);
} // pub struct Monomial

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/


/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct Polynomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub struct Polynomial {
// //
     monomials_:Vec<Monomial>,
     constant_:FElem,
}
// 
//     Polynomial()->Self monomials_(), constant_(0) {}
//     Polynomial(monomial:&Monomial)->Self monomials_(1, monomial), constant_(0) {}
//     Polynomial(var:&Variable)->Self monomials_(1, Monomial(var)), constant_(0) {}
//     Polynomial(val:&FElem)->Self monomials_(), constant_(val) {}
//     Polynomial(linearCombination:&LinearCombination);
//     Polynomial(linearTerm:&LinearTerm)->Self monomials_(1, Monomial(linearTerm)), constant_(0) {}
//     Polynomial(int i)->Self monomials_(), constant_(i) {}

//     FElem eval(assignment:&VariableAssignment) const;
//     set  getUsedVariables() const;
//     const Vec<Monomial>& getMonomials()const;
//     const FElem getConstant()const;
//     String asString() const;
//     Polynomial& operator+=(other:&Polynomial);
//     Polynomial& operator*=(other:&Polynomial);
//     Polynomial& operator-=(other:&Polynomial);
//     Polynomial& operator+=(other:&LinearTerm) {return self += Polynomial(Monomial(other));}
// }; // pub struct Polynomial

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// inline Polynomial operator-(src:&Polynomial) {return Polynomial(FElem(0)) -= src;}

// } // namespace gadgetlib2

use crate::gadgetlib2::variable_operators;

//#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLE_HPP_
/** @file
 *****************************************************************************
 Implementation of the low level objects needed for field arithmetization.
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  <climits>
// use  <iostream>
// use  <set>
// use  <stdexcept>
// 

// use crate::gadgetlib2::infrastructure;
// use crate::gadgetlib2::pp;
use crate::gadgetlib2::variable;

// using String;
// using ::std::stringstream;
// using BTreeSet;
// using Vec;
// using RcCell;
// using ::std::cout;
// using ::std::endl;
// using ::std::dynamic_pointer_cast;

// namespace gadgetlib2 {

// Optimization: In the future we may want to port most of the member functions  from this file to
// the .hpp files in order to allow for compiler inlining. As inlining has tradeoffs this should be
// profiled before doing so.

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct FElem                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// FElem& pub fn operator=(other:&FElem) {
// 	if fieldType() == other.fieldType() || fieldType() == AGNOSTIC {
// 		elem_ = other.elem_.clone();
// 	} else if other.fieldType() != AGNOSTIC {
// 		eyre::bail!("Attempted to assign field element of incorrect type");
// 	} else {
// 		*elem_ = dynamic_cast<FConst*>(other.elem_.get())->asLong();
// 	}
// 	return self;
// }

// FElem& pub fn operator=(FElem&& other) {
// 	if fieldType() == other.fieldType() || fieldType() == AGNOSTIC {
// 		elem_ = ::(other.elem_);
// 	} else if other.elem_.fieldType() != AGNOSTIC {
// 		eyre::bail!(
// 				"Attempted to move assign field element of incorrect type");
// 	} else {
// 		*elem_ = dynamic_cast<FConst*>(other.elem_.get())->asLong();
// 	}
// 	return self;
// // }

// FElem& pub fn operator*=(other:&FElem) {
// 	promoteToFieldType(other.fieldType());
// 	*elem_ *= *other.elem_;
// 	return self;
// }

// FElem& pub fn operator+=(other:&FElem) {
// 	promoteToFieldType(other.fieldType());
// 	*elem_ += *other.elem_;
// 	return self;
// }

// FElem& pub fn operator-=(other:&FElem) {
// 	promoteToFieldType(other.fieldType());
// 	*elem_ -= *other.elem_;
// 	return self;
// }

impl FElem{

// pub fn FElem(elem:&FElemInterface)->Self
// 		elem_(elem.clone()) {
// }
// pub fn FElem()->Self
// 		elem_(new FConst(0)) {
// }
// pub fn FElem(const n:u64)->Self
// 		elem_(new FConst(n)) {
// }
// pub fn FElem(i:i32)->Self
// 		elem_(new FConst(i)) {
// }
// pub fn FElem(n:usize)->Self
// 		elem_(new FConst(n)) {
// }
// pub fn FElem(elem:&Fp)->Self
// 		elem_(new R1P_Elem(elem)) {
// }
// pub fn FElem(src:&FElem)->Self
// 		elem_(src.elem_.clone()) {
// }


 pub fn fieldMustBePromotedForArithmetic(lhsField:&FieldType,
		rhsField:&FieldType)->bool {
	if lhsField == rhsField
		{return false;}
	if rhsField == AGNOSTIC
		{return false;}
	return true;
}

pub fn  promoteToFieldType( types:FieldType) {
	if !fieldMustBePromotedForArithmetic(self.fieldType(), types) {
		return;
	}
	if types == R1P {
		let  fConst = elem_.get();
		assert!(fConst != None,
				"Cannot convert between specialized field types.");
		elem_.reset( R1P_Elem::new(fConst.asLong()));
	} else {
		eyre::bail!("Attempted to promote to unknown field type");
	}
}

pub fn inverse(fieldType:&FieldType)->FElem{
	let promoteToFieldType=fieldType.clone();
	return FElem::new((elem_.inverse()));
}

pub fn getBit(i:u32, fieldType:&FieldType)->int{
    promoteToFieldType=fieldType.clone();
    if self.fieldType() == fieldType {
        return elem_.getBit(i);
    } else {
        eyre::bail!("Attempted to extract bits from incompatible field type.");
    }
}

 pub fn power(base:&FElem, exponent:u64)->FElem { // TODO .cpp
	let  retval=base.clone();
	retval.elem_.power(exponent);
	return retval;
}
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct FConst                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
// FConst& pub fn operator+=(other:&FElemInterface) {
// 	contents_ += dynamic_cast<const FConst&>(other).contents_;
// 	return self;
// }

// FConst& pub fn operator-=(other:&FElemInterface) {
// 	contents_ -= dynamic_cast<const FConst&>(other).contents_;
// 	return self;
// }

// FConst& pub fn operator*=(other:&FElemInterface) {
// 	contents_ *= dynamic_cast<const FConst&>(other).contents_;
// 	return self;
// }
impl FConst{


pub fn inverse() ->FElemInterfacePtr{
	eyre::bail!("Attempted to invert an FConst element.");
}

pub fn power(&self,exponent:u64)->&FElemInterface{
	let contents_ = 0.5 + ::std::pow(double(contents_), double(exponent));
	self
}
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     pub struct R1P_Elem                         ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// R1P_Elem& pub fn operator+=(other:&FElemInterface) {
// 	if other.fieldType() == R1P {
// 		elem_ += dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() == AGNOSTIC {
// 		elem_ += dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible type to R1P_Elem.");
// 	}
// 	return self;
// }

// R1P_Elem& pub fn operator-=(other:&FElemInterface) {
// 	if other.fieldType() == R1P {
// 		elem_ -= dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() == AGNOSTIC {
// 		elem_ -= dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible type to R1P_Elem.");
// 	}
// 	return self;
// }

// R1P_Elem& pub fn operator*=(other:&FElemInterface) {
// 	if other.fieldType() == R1P {
// 		elem_ *= dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() == AGNOSTIC {
// 		elem_ *= dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible type to R1P_Elem.");
// 	}
// 	return self;
// }

// bool pub fn operator==(other:&FElemInterface) const {
// 	const R1P_Elem* pOther = dynamic_cast<const R1P_Elem*>(&other);
// 	if pOther {
// 		return elem_ == pOther->elem_;
// 	}
// 	const FConst* pConst = dynamic_cast<const FConst*>(&other);
// 	if pConst {
// 		return self == *pConst;
// 	}
// 	eyre::bail!("Attempted to Compare R1P_Elem with incompatible type.");
// }
impl R1P_Elem{

pub fn inverse() ->FElemInterfacePtr{
	return FElemInterfacePtr(R1P_Elem::new(elem_.inverse()));
}

pub fn asLong() ->u64{
	//assert!(elem_.as_ulong() <= LONG_MAX, "overflow:u64 occured.");
	return elem_.as_ulong() as u64;
}
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct Variable                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
// nextFreeIndex_:VarIndex_t = 0;
impl Variable{

// #ifdef DEBUG
// pub fn Variable(name:&String)->Self index_(nextFreeIndex_++), name_(name) {
// 	assert!(nextFreeIndex_ > 0, format!("Variable index overflow has occured, maximum number of "
// 					"Variables is {}", ULONG_MAX));
// }
// #else
pub fn Variable(name:&String)->Self{
    let index_=nextFreeIndex_;
        nextFreeIndex_+=1;
    // //ffec::UNUSED(name);
    assert!(nextFreeIndex_ > 0, "Variable index overflow has occured, maximum number of Variables is {}", u64::MAX);
    
}
//#endif

// pub fn ~Variable() {
// }
// ;

pub fn name() ->string{
// #    ifdef DEBUG
// 	return name_;
// #    else
	return "".to_owned();
// #    endif
}

pub fn eval(assignment:&VariableAssignment) ->FElem{
	// try {
		return assignment.at(self);
	// } catch (::std::out_of_range) {
		// eyre::bail!(
		// 		format!(
		// 				"Attempted to evaluate unassigned Variable \"{}\", idx:{}",
		// 				name(), index_));
	// }
}
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct VariableArray                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl VariableArray{
// #ifdef DEBUG
// pub fn new(name:&String) ->Self{ VariableArrayContents(), name_(name) }
// pub fn new2(size:i32, name:&String)->Self {
//     // : VariableArrayContents() 
//     for i in 0..size {
//         push_back(Variable(format!("{}[{}]", name, i)));
//     }
// }

// pub fn new3(size:usize, name:&String)->Self {// : VariableArrayContents()
//     for i in 0..size {
//         push_back(Variable(format!("{}[{}]", name, i)));
//     }
// }
// pub fn name() ->String{
// 	return name_;
// }

// #else
pub fn name() ->String{
	return "".to_owned();
}


pub fn new(name:&String)->Self { 
// //ffec::UNUSED(name); : VariableArrayContents() 
}
pub fn new(size:i32, name:&String)
    ->Self { VariableArrayContents(size) }
pub fn new(size:usize, name:&String) ->Self
    {  VariableArrayContents(size) }
//#endif
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 Custom Variable classes                    ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl MultiPackedWord{
pub fn new(fieldType:&FieldType) ->Self{
		// {VariableArray(), numBits_(0), fieldType_(fieldType) 
}

pub fn new2(numBits:usize,
		fieldType:&FieldType, name:&String) ->Self
	{	
    // VariableArray(), numBits_(numBits), fieldType_(fieldType) 
	let  packedSize = getMultipackedSize();
	let  varArray=VariableArray::new(packedSize, name);
	Self::swap(varArray)
}

pub fn  resize(numBits:usize) {
	let numBits_ = numBits;
	let packedSize = getMultipackedSize();
	// Self::resize(packedSize);
}

pub fn getMultipackedSize() ->usize{
	let  mut packedSize = 0;
	if fieldType_ == R1P {
		packedSize = 1; // TODO add assertion that numBits can fit in the field characteristic
	} else {
		eyre::bail!("Unknown field type for packed variable.");
	}
	return packedSize;
}
}
impl DualWord{
pub fn new(numBits:usize, fieldType:&FieldType,
		name:&String) ->Self
		 {
Self{multipacked_:MultiPackedWord::new(numBits, fieldType, name + "_p"), unpacked_:UnpackedWord::new(numBits,
				name + "_u")}
}

pub fn new(multipacked:&MultiPackedWord,
		unpacked:&UnpackedWord) ->Self{
		Self{multipacked_:MultiPackedWord::new(multipacked), unpacked_:UnpackedWord::new(unpacked) }
}

pub fn  resize(newSize:usize) {
	multipacked_.resize(newSize);
	unpacked_.resize(newSize);
}
}
impl DualWordArray{
pub fn DualWordArray(fieldType:&FieldType) ->Self
		 {
Self{multipackedContents_:vec![0, MultiPackedWord::new(fieldType)], unpackedContents_:
				0, numElements_:0}
}

pub fn DualWordArray(  multipackedContents:&MultiPackedWordArray, // TODO delete, for dev
		 unpackedContents:&UnpackedWordArray) ->Self
		 {
	assert!(multipackedContents_.len() == numElements_,
			"Dual Variable multipacked contents size mismatch");
	assert!(unpackedContents_.len() == numElements_,
			"Dual Variable packed contents size mismatch");
Self{
multipackedContents_:multipackedContents, unpackedContents_:
				unpackedContents, numElements_:multipackedContents_.len()}
}

pub fn multipacked() ->MultiPackedWordArray{
	return multipackedContents_;
}
pub fn unpacked() ->UnpackedWordArray{
	return unpackedContents_;
}
 pub fn packed() ->PackedWordArray {
	assert!(numElements_ == multipackedContents_.len(),
			"multipacked contents size mismatch");
	let  retval=PackedWordArray(numElements_);
	for i in 0..numElements_ {
		let element = multipackedContents_[i];
		assert!(element.len() == 1,
				"Cannot convert from multipacked to packed");
		retval[i] = element[0];
	}
	return retval;
}

pub fn  push_back(dualWord:&DualWorddualWord) {
	multipackedContents_.push_back(dualWord.multipacked());
	unpackedContents_.push_back(dualWord.unpacked());
	numElements_+=1;
}

pub fn at(i:usize) ->DualWord{
	//let multipackedRep= multipacked()[i];
	//let unpackedRep= unpacked()[i];
	//const DualWord retval(multipackedRep, unpackedRep);
	//return retval;
	return DualWord::new(multipacked()[i], unpacked()[i]);
}

pub fn size() ->usize{
	assert!(multipackedContents_.len() == numElements_,
			"Dual Variable multipacked contents size mismatch");
	assert!(unpackedContents_.len() == numElements_,
			"Dual Variable packed contents size mismatch");
	return numElements_;
}
}
/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct LinearTerm                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

impl LinearTerm{
pub fn asString() ->String{
	if coeff_ == 1 {
		return variable_.name();
	} else if coeff_ == -1 {
		return format!("-1 * {}", variable_.name());
	} else if coeff_ == 0 {
		return format!("0 * {}", variable_.name());
	} else {
		return format!("{} * {}", coeff_.asString(),
				variable_.name());
	}
}

pub fn eval(assignment:&VariableAssignment) ->FElem{
	return FElem::new(coeff_) *= variable_.eval(assignment);
}
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  pub struct LinearCombination                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// LinearCombination& pub fn operator+=(
// 		other:&LinearCombination) {

// 	// jSNARK-edit: This method is modified in order to reduce memory consumption when the same variable is
// 	// being added to a linear combination object multiple times.
// 	// This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.

// 	if indexMap_.len() == 0 {
// 		linearTerms_.insert(linearTerms_.end(), other.linearTerms_.cbegin(),
// 				other.linearTerms_.cend());
// 		constant_ += other.constant_;
// 	} else {
// 		for lt in &other.linearTerms_ {
// 			if indexMap_.find(lt.variable().getIndex()) != indexMap_.end() {
// 				linearTerms_[indexMap_[lt.variable().getIndex()]] += lt.coeff();
// 			} else {
// 				linearTerms_.push_back(lt);
// 				int k = indexMap_.len();
// 				indexMap_[lt.variable().getIndex()] = k;
// 			}
// 		}
// 		constant_ += other.constant_;
// 	}

// 	// heuristic threshold
// 	if linearTerms_.len() > 10 && indexMap_.len() == 0 {
// 		int i = 0;
// 		Vec<LinearTerm> newVec;
// 		Vec<LinearTerm>::iterator lt = (linearTerms_.begin());
// 		while (lt != linearTerms_.end()) {

// 			if indexMap_.find(lt->variable().getIndex()) != indexMap_.end() {
// 				newVec[indexMap_[lt->variable().getIndex()]] += lt->coeff();
// 			} else {
// 				newVec.push_back(*lt);
// 				indexMap_[lt->variable().getIndex()] = i++;

// 			}
// 			lt+=1;
// 		}
// 		linearTerms_ = newVec;
// 	}

// 	return self;
// }

// LinearCombination& pub fn operator-=(
// 		other:&LinearCombination) {

// 	// jSNARK-edit: This method is rewritten in order to reduce memory consumption when the same variable is
// 	// being added to a linear combination object multiple times.
// 	// This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.
// 	if indexMap_.len() == 0 {
// 		for lt in &other.linearTerms_ {
// 			linearTerms_.push_back(-lt);
// 		}
// 		constant_ -= other.constant_;
// 	} else {
// 		for lt in &other.linearTerms_ {
// 			if indexMap_.find(lt.variable().getIndex()) != indexMap_.end() {
// 				linearTerms_[indexMap_[lt.variable().getIndex()]] -= lt.coeff();
// 			} else {
// 				linearTerms_.push_back(-lt);
// 				int k = indexMap_.len();
// 				indexMap_[lt.variable().getIndex()] = k;
// 			}
// 		}
// 		constant_ -= other.constant_;
// 	}

// 	// heuristic threshold
// 	if linearTerms_.len() > 10 && indexMap_.len() == 0 {
// 		int i = 0;
// 		Vec<LinearTerm> newVec;
// 		Vec<LinearTerm>::iterator lt = (linearTerms_.begin());

// 		while (lt != linearTerms_.end()) {

// 			if indexMap_.find(lt->variable().getIndex()) != indexMap_.end() {
// 				newVec[indexMap_[lt->variable().getIndex()]] += lt->coeff();
// 			} else {
// 				newVec.push_back(*lt);
// 				indexMap_[lt->variable().getIndex()] = i++;
// 			}
// 			lt+=1;
// 		}
// 		linearTerms_ = newVec;
// 	}

// 	return self;

// }

// LinearCombination& pub fn operator*=(other:&FElem) {
// 	constant_ *= other;
// 	for lt in &linearTerms_ {
// 		lt *= other;
// 	}
// 	return self;
// }

impl LinearCombination{
pub fn eval(assignment:&VariableAssignment) ->FElem{
	let mut  evaluation = constant_;
	for lt in &linearTerms_ {
		evaluation += lt.eval(assignment);
	}
	return evaluation;
}

pub fn asString() ->String{
// #ifdef DEBUG
// 	let mut  retval;
// 	let mut  it = linearTerms_.iter();
// 	if it == linearTerms_.end() {
// 		return constant_.asString();
// 	} else {
// 		retval += it->asString();
// 	}
// 	for (it+=1; it != linearTerms_.end(); ++it) {
// 		retval += " + " + it->asString();
// 	}
// 	if constant_ != 0 {
// 		retval += " + " + constant_.asString();
// 	}
// 	return retval;
// #else // ifdef DEBUG
	return "".to_owned();
//#endif // ifdef DEBUG
}

 pub fn getUsedVariables()-> BTreeSet<i32> {
	let mut  retSet=BTreeSet::new();
	for lt in &linearTerms_ {
		retSet.insert(lt.variable());
	}
	return retSet;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

 pub fn sum(inputs:&VariableArray)->LinearCombination {
	let mut  retval=LinearCombination::new(0);
	for var in &inputs {
		retval += var;
	}
	return retval;
}

 pub fn negate(lc:&LinearCombination)->LinearCombination {
	return (1 - lc);
}
}
/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        pub struct Monomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl Monomial{
pub fn new(linearTerm:&LinearTerm) ->Self
		{
    //coeff_(linearTerm.coeff_), variables_() 
	variables_.insert(linearTerm.variable_);
}

pub fn eval(assignment:&VariableAssignment) ->FElem{
	let mut  retval = coeff_;
	for var in &variables_ {
		retval *= var.eval(assignment);
	}
	return retval;
}

pub fn getUsedVariables() ->set{
	return set::new(variables_);
}

 pub fn getCoefficient() ->FElem{
	return coeff_;
}

pub fn asString() ->String{
// #ifdef DEBUG
// 	if variables_.len() == 0 {
// 		return coeff_.asString();
// 	}
// 	string retval;
// 	if coeff_ != 1 {
// 		retval += coeff_.asString() + "*";
// 	}
// 	auto iter = variables_.begin();
// 	retval += iter->name();
// 	for(iter+=1; iter != variables_.end(); ++iter) {
// 		retval += "*" + iter->name();
// 	}
// 	return retval;
// #else // ifdef DEBUG
	return "".to_owned();
//#endif // ifdef DEBUG
}

}


// Monomial pub fn operator-() const {
// 	Monomial retval = self;
// 	retval.coeff_ = -retval.coeff_;
// 	return retval;
// }

// Monomial& pub fn operator*=(other:&Monomial) {
// 	coeff_ *= other.coeff_;
// 	variables_.insert(other.variables_.begin(), other.variables_.end());
// 	return self;
// }
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      pub struct Polynomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl Polynomial{
pub fn new(linearCombination:&LinearCombination) ->Self
		 {
        let mut monomials_=vec![];
	for linearTerm in &linearCombination.linearTerms_ {
		monomials_.push_back(Monomial(linearTerm));
	}
Self{monomials_, constant_:linearCombination.constant_.clone()}
}

pub fn eval(assignment:&VariableAssignment) ->FElem{
	let mut  retval = constant_;
	for monomial in &monomials_ {
		retval += monomial.eval(assignment);
	}
	return retval;
}

  pub fn getUsedVariables() ->set {
	let mut   retset=set::new();
	for monomial in &monomials_ {
		let   curSet = monomial.getUsedVariables();
		retset.insert(curSet.begin(), curSet.end());
	}
	return retset;
}

pub fn getMonomials() ->Vec<Monomial>  {
	return monomials_;
}

 pub fn getConstant() ->FElem{
	return constant_;
}

pub fn asString() ->String{
// #   ifndef DEBUG
	return "".to_owned();
// #   endif
	// if monomials_.len() == 0 {
	// 	return constant_.asString();
	// }
	// string retval;
	// auto iter = monomials_.begin();
	// retval += iter->asString();
	// for (iter+=1; iter != monomials_.end(); ++iter) {
	// 	retval += " + " + iter->asString();
	// }
	// if constant_ != 0 {
	// 	retval += " + " + constant_.asString();
	// }
	// return retval;
}

}

// Polynomial& pub fn operator+=(other:&Polynomial) {
// 	constant_ += other.constant_;
// 	monomials_.insert(monomials_.end(), other.monomials_.begin(),
// 			other.monomials_.end());
// 	return self;
// }

// Polynomial& pub fn operator*=(other:&Polynomial) {
// 	vector<Monomial> newMonomials;
// 	for thisMonomial in &monomials_ {
// 		for otherMonomial in &other.monomials_ {
// 			newMonomials.push_back(thisMonomial * otherMonomial);
// 		}
// 		newMonomials.push_back(thisMonomial * other.constant_);
// 	}
// 	for otherMonomial in &other.monomials_ {
// 		newMonomials.push_back(otherMonomial * self.constant_);
// 	}
// 	constant_ *= other.constant_;
// 	monomials_ = ::(newMonomials);
// 	return self;
// }

// Polynomial& pub fn operator-=(other:&Polynomial) {
// 	constant_ -= other.constant_;
// 	for otherMonomial in &other.monomials_ {
// 		monomials_.push_back(-otherMonomial);
// 	}
// 	return self;
// }
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// } // namespace gadgetlib2
