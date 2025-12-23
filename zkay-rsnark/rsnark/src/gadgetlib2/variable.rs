//  Declaration of the low level objects needed for field arithmetization.
use super::pp::Fp;
use super::variable_operators::*;
use crate::gadgetlib2::protoboard::Protoboard;
use enum_dispatch::enum_dispatch;
use rccell::RcCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::{AddAssign, MulAssign, Neg, SubAssign};
use std::ops::{Index, IndexMut};
// pub struct GadgetLibAdapter;

// // Forward declarations
// pub struct Protoboard;
// pub struct FElemInterface;
// pub struct FElem;
// pub struct FConst;
// pub struct Variable;
// pub struct VariableArray;
#[derive(Default, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum FieldType {
    R1P,
    #[default]
    AGNOSTIC,
}
use strum_macros::{EnumIs, EnumTryAs};
#[enum_dispatch(FElemInterface)]
#[derive(Debug, EnumIs, EnumTryAs, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub enum ElemType {
    Const(FConst),
    Elem(R1P_Elem),
}

impl Default for ElemType {
    fn default() -> Self {
        Self::Const(FConst::default())
    }
}

impl AddAssign<&Self> for ElemType {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        match self {
            Self::Const(self_) => {
                *self_ += other;
            }
            Self::Elem(self_) => {
                *self_ += other;
            }
        }
    }
}

impl SubAssign<&Self> for ElemType {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        match self {
            Self::Const(self_) => {
                *self_ -= other;
            }
            Self::Elem(self_) => {
                *self_ -= other;
            }
        }
    }
}

impl MulAssign<&Self> for ElemType {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &Self) {
        match self {
            Self::Const(self_) => {
                *self_ *= other;
            }
            Self::Elem(self_) => {
                *self_ *= other;
            }
        }
    }
}

pub type VariablePtr = RcCell<Variable>;
pub type VariableArrayPtr = RcCell<VariableArrayType>;
pub type FElemInterfacePtr = RcCell<ElemType>;
pub type ProtoboardPtr = Option<RcCell<Protoboard>>;
pub type VarIndex_t = u64;

// Naming Conventions:
//FieldType::R1P == Rank 1 Prime characteristic

// /*************************************************************************************************/
// /*************************************************************************************************/
// /*******************                                                            ******************/
// /*******************                   pub struct FElemInterface                     ******************/
// /*******************                                                            ******************/
// /*************************************************************************************************/
// /*************************************************************************************************/
//    An interface pub struct for field elements.
//    Currently 2 classes will derive from this interface:
//    R1P_Elem - Elements of a field of prime characteristic
//    FConst - Formally not a field, only placeholders for field agnostic constants, such as 0 and 1.
//             Can be used for -1 or any other constant which makes semantic sense in all fields.

#[enum_dispatch]
pub trait FElemInterface: Default + Clone {
    //
    // fn  FElemInterface& operator=(n:u64) =,
    // /// FConst will be field agnostic, allowing us to hold values such as 0 and 1 without knowing
    // /// the underlying field. This assignment operator will convert to the correct field element.
    // fn  FElemInterface& operator=(src:&FConst) = 0;
    fn asString(&self) -> String {
        String::new()
    }
    fn fieldType(&self) -> FieldType {
        FieldType::AGNOSTIC
    }
    // fn  FElemInterface& operator+=(other:&FElemInterface) = 0;
    // fn  FElemInterface& operator-=(other:&FElemInterface) = 0;
    // fn  FElemInterface& operator*=(other:&FElemInterface) = 0;
    // fn  bool operator==(other:&FElemInterface) 0:=,
    // fn  bool operator==(other:&FConst) 0:=,
    // /// This operator is not always mathematically well defined. 'n' will be checked in runtime
    // /// for fields in which integer values are not well defined.
    // fn  bool operator==(0:n:u64) const =,
    //  @returns a unique_ptr to a copy of the current element.
    // fn  FElemInterfacePtr clone() 0:=,
    fn inverse(&self) -> Self {
        panic!("")
    }
    fn asLong(&self) -> i64 {
        0
    }
    fn getBit(&self, i: u32) -> i32 {
        0
    }
    fn power(&self, exponent: u64) -> Self {
        self.clone()
    }
    // fn  ~FElemInterface(){};
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

// inline bool operator==(first:u64, second:&FElemInterface) {return second ==first:,}
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

/// A wrapper pub struct for field elements. Can hold any derived pub type of FieldElementInterface

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FElem {
    pub elem_: RcCell<ElemType>,
}
impl PartialEq<i32> for FElem {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.elem_.borrow().asLong() == *other as i64
    }
}

//
//     explicit FElem(elem:&FElemInterface);
//     /// Helper method. When doing arithmetic between a constant and a field specific element
//     /// we want to "promote" the constant to the same field. This function changes the unique_ptr
//     /// to point to a field specific element with the same value as the constant which it held.
//     pub fn  promoteToFieldType(FieldType pub type);
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
// inline bool operator==(first:u64, second:&FElem) {return second ==first:,}
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
#[derive(Debug, Default, Clone, PartialOrd, Ord, Eq)]
pub struct FConst {
    //: public FElemInterface
    pub contents_: i64,
}
// explicit FConst(const n:u64)->Self contents_(n) {}
//
//     fn  FConst& operator=(n:n:u64) {contents_ =, return self;}
//     fn  FConst& operator=(src:&FConst) {contents_ = src.contents_; return self;}
//     fn  String asString() const {return format!("{}",contents_);}
//     fn  FieldType fieldType()FieldType::AGNOSTIC:{return,}
//     fn  FConst& operator+=(other:&FElemInterface);
//     fn  FConst& operator-=(other:&FElemInterface);
//     fn  FConst& operator*=(other:&FElemInterface);
//     fn  bool operator==(other:&FElemInterface) self:{return other ==,}
//     fn  bool operator==(other:&FConst) const {return contents_ == other.contents_;}
//     fn  bool operator==(n:n:u64) const {return contents_ ==,}
//     /// @return a unique_ptr to a new copy of the element
//     fn  FElemInterfacePtr clone() const {return FElemInterfacePtr(new FConst(self));}
//     /// @return a unique_ptr to a new copy of the element's multiplicative inverse
//     fn  FElemInterfacePtr inverse() const;
//     asLong:u64() contents_:{return,}
//     int getBit(i:u32) const { //ffec::UNUSED(i); eyre::bail!("Cannot get bit from FConst."); }
//     fn  FElemInterface& power(exponent:u64);

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
#[derive(Debug, Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct R1P_Elem {
    // ///: public FElemInterface
    pub elem_: Fp,
}
//

//     explicit R1P_Elem(elem:&Fp)->Self elem_(elem) {}
//     fn  R1P_Elem& operator=(src:&FConst) {elem_ = src.asLong(); return self;}
//     fn  R1P_Elem& operator=(self:n:u64) {elem_ = Fp(n); return,}
//     fn  String asString() const {return format!("{}", elem_.as_ulong());}
//     fn  FieldType fieldType()FieldType::R1P:{return,}
//     fn  R1P_Elem& operator+=(other:&FElemInterface);
//     fn  R1P_Elem& operator-=(other:&FElemInterface);
//     fn  R1P_Elem& operator*=(other:&FElemInterface);
//     fn  bool operator==(other:&FElemInterface) const;
//     fn  bool operator==(other:&FConst) const {return elem_ == Fp(other.asLong());}
//     fn  bool operator==(const n:u64) const {return elem_ == Fp(n);}
//     /// @return a unique_ptr to a new copy of the element
//     fn  FElemInterfacePtr clone() const {return FElemInterfacePtr(new R1P_Elem(self));}
//     /// @return a unique_ptr to a new copy of the element's multiplicative inverse
//     fn  FElemInterfacePtr inverse() const;
//     asLong:u64() const;
//     int getBit(i:u32) const {return elem_.as_bigint().test_bit(i);}
//     fn  FElemInterface& power(exponent:u64) {elem_^= exponent; return self;}

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
pub type VariableSet = BTreeSet<Variable>; //VariableStrictOrder
pub type multiset = BTreeMap<Variable, i32>; //use std::collections::BTreeMap;

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
pub type VariableAssignment = HashMap<Variable, FElem>; //VariableStrictOrder
#[derive(Default, Clone, Hash, Ord, Eq)]
pub struct Variable {
    pub index_: VarIndex_t,
    ///< This index differentiates and identifies Variable instances.
    // nextFreeIndex_: VarIndex_t, //static///< Monotonically-increasing counter to allocate disinct indices.
    // #ifdef DEBUG
    pub name_: String,
    //#endif
}
//    /**
//     * @brief allocates the variable
//     */
//
//     explicit Variable(name:&String = "");
//     fn  ~Variable();

//     String name() const;

//     /// A functor for strict ordering of Variables. Needed for STL containers.
//     /// This is not an ordering of Variable assignments and has no semantic meaning.
//     struct VariableStrictOrder {
//         bool operator()(first:Variable&, second:&Variable)const {
//             return first.index_ < second.index_;
//         }
//     };
impl PartialEq for Variable {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index_ == other.index_
    }
}
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
// pub type VariableAssignment =HashMap<Variable, FElem> ;

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 pub struct VariableArray                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

pub type VariableArrayContents = Vec<Variable>;
pub trait SubVariableArrayConfig: Default + Clone + Ord {
    fn resize(&mut self, numBits: usize) -> usize {
        numBits
    }
}

#[derive(Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct VariableArray<T: SubVariableArrayConfig> {
    // //: public VariableArrayContents
    pub contents: VariableArrayContents,
    pub name_: String,
    pub t: T,
    //     explicit VariableArray(name:&String = "");
    //     explicit VariableArray(size:i32, name:&String = "");
    //     explicit VariableArray(size:usize, name:&String = "");
    //     explicit VariableArray(size:usize, contents:&Variable)
    //             : VariableArrayContents(size, contents) {}

    //     using VariableArrayContents::operator[];
    //     using VariableArrayContents::at;
    //     using VariableArrayContents::push;
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
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableArrayBase;
impl SubVariableArrayConfig for VariableArrayBase {}
pub type FlagVariable = Variable;
///< Holds variable whose purpose is to be populated with a boolean
///< value, Field(0) or Field(1)
pub type FlagVariableArray = VariableArray<VariableArrayBase>;
pub type PackedWord = Variable;
///< Represents a packed word that can fit in a field element.
///< For a word representing an unsigned integer for instance this
///< means we require (int < fieldSize)
pub type PackedWordArray = VariableArray<VariableArrayBase>;
#[enum_dispatch]
pub trait VariableArrayConfig {
    fn iter(&self) -> std::slice::Iter<Variable>;
    fn name(&self) -> &String;
    fn at(&self, i: usize) -> &Variable;
    fn push(&mut self, val: Variable);
    fn len(&self) -> usize;
    fn resize(&mut self, numBits: usize);
}
#[enum_dispatch(VariableArrayConfig)]
#[derive(EnumIs, EnumTryAs, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub enum VariableArrayType {
    Base(VariableArray<VariableArrayBase>),
    UnpackedWord(VariableArray<UnpackedWord>),
    MultiPackedWord(VariableArray<MultiPackedWord>),
}

impl Default for VariableArrayType {
    fn default() -> Self {
        Self::Base(VariableArray::<VariableArrayBase>::default())
    }
}

impl Index<usize> for VariableArrayType {
    type Output = Variable;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Base(_self) => _self.contents.get(index).unwrap(),
            Self::UnpackedWord(_self) => _self.contents.get(index).unwrap(),
            Self::MultiPackedWord(_self) => _self.contents.get(index).unwrap(),
        }
    }
}

impl IndexMut<usize> for VariableArrayType {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Self::Base(_self) => _self.contents.get_mut(index).unwrap(),
            Self::UnpackedWord(_self) => _self.contents.get_mut(index).unwrap(),
            Self::MultiPackedWord(_self) => _self.contents.get_mut(index).unwrap(),
        }
    }
}
impl IntoIterator for VariableArrayType {
    type Item = Variable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Base(_self) => _self.contents.into_iter(),
            Self::UnpackedWord(_self) => _self.contents.into_iter(),
            Self::MultiPackedWord(_self) => _self.contents.into_iter(),
        }
    }
}

/// Holds variables whose purpose is to be populated with the unpacked form of some word, bit by bit

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnpackedWord;
//  {
// : public VariableArray
// UnpackedWord()->Self VariableArray() {}
// UnpackedWord(numBits:usize, name:&String)->Self VariableArray(numBits, name) {}
// } // pub struct UnpackedWord

pub type UnpackedWordArray = Vec<VariableArray<UnpackedWord>>;

/// Holds variables whose purpose is to be populated with the packed form of some word.
/// word representation can be larger than a single field element in small enough fields
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiPackedWord {
    // //: public VariableArray
    pub numBits_: usize,
    pub fieldType_: FieldType,
}
// usize getMultipackedSize() const;
//
//     MultiPackedWord(fieldType:&FieldType =FieldType::AGNOSTIC);
//     MultiPackedWord(numBits:usize, fieldType:&FieldType, name:&String);
//     pub fn  resize(numBits:usize);
//     String name() const {return pub fn name();}
// }; // pub struct MultiPackedWord

pub type MultiPackedWordArray = Vec<VariableArray<MultiPackedWord>>;

/// Holds both representations of a word, both multipacked and unpacked
#[derive(Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct DualWord {
    pub multipacked_: VariableArray<MultiPackedWord>,
    pub unpacked_: VariableArray<UnpackedWord>,
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

#[derive(Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct DualWordArray {
    // kept as 2 separate arrays because the more common usecase will be to request one of these,
    // and not dereference a specific DualWord
    pub multipackedContents_: MultiPackedWordArray,
    pub unpackedContents_: UnpackedWordArray,
    pub numElements_: usize,
    //
    //     DualWordArray(fieldType:&FieldType);
    //     DualWordArray(multipackedContents:MultiPackedWordArray&, // TODO delete, for dev
    //                   unpackedContents:&UnpackedWordArray);
    //     MultiPackedWordArray multipacked() const;
    //     UnpackedWordArray unpacked() const;
    //     PackedWordArray packed() const; //< For cases in which we can assume each unpacked value fits
    //                                     //< in 1 packed Variable
    //     pub fn  push(dualWord:&DualWorddualWord);
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
#[derive(Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct LinearTerm {
    pub variable_: Variable,
    pub coeff_: FElem,
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
//  pub type size_type=Vec<LinearTerm>::size_type;
#[derive(Clone, Eq, PartialEq)]
pub struct LinearCombination {
    pub linearTerms_: Vec<LinearTerm>,
    pub indexMap_: HashMap<i32, i32>, // jSNARK-edit: This map is used to reduce memory consumption. Can be helpful for some circuits produced by Pinocchio compiler.
    pub constant_: FElem,
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
#[derive(Default, Clone, PartialOrd, Ord, Eq, PartialEq)]
pub struct Monomial {
    pub coeff_: FElem,
    pub variables_: BTreeMap<Variable, i32>,
    //multiset// currently just a vector of variables. This can
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
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Polynomial {
    pub monomials_: Vec<Monomial>,
    pub constant_: FElem,
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

// use crate::gadgetlib2::infrastructure;
// use crate::gadgetlib2::pp;
// use crate::gadgetlib2::variable;

// using String;
// using ::std::stringstream;
// using BTreeSet;
// using Vec;
// using RcCell;
// using ::std::cout;
// using ::std::endl;
// using ::std::dynamic_pointer_cast;

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
// 	if fieldType() == other.fieldType() || fieldType() ==FieldType::AGNOSTIC {
// 		elem_ = other.elem_.clone();
// 	} else if other.fieldType() !=FieldType::AGNOSTIC {
// 		eyre::bail!("Attempted to assign field element of incorrect pub type");
// 	} else {
// 		*elem_ = dynamic_cast<FConst*>(other.elem_.get())->asLong();
// 	}
// 	return self;
// }

impl AddAssign<&Self> for FElem {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.promoteToFieldType(&other.fieldType());
        *self.elem_.borrow_mut() += &other.elem_.borrow();
    }
}

impl SubAssign<&Self> for FElem {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        self.promoteToFieldType(&other.fieldType());
        *self.elem_.borrow_mut() -= &other.elem_.borrow();
    }
}

impl MulAssign<&Self> for FElem {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &Self) {
        self.promoteToFieldType(&other.fieldType());
        *self.elem_.borrow_mut() *= &other.elem_.borrow();
    }
}
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
//   bool operator==(other:&FElem) const {return *elem_ == *other.elem_;}

//     FElem operator-() {FElem retval(0); retval -= FElem(*elem_); return retval:,}
impl Neg for FElem {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut retval = FElem::from(0);
        retval -= &FElem::from(self.elem_.borrow().clone());
        retval
    }
}

impl From<ElemType> for FElem {
    fn from(rhs: ElemType) -> Self {
        Self {
            elem_: RcCell::new(rhs),
        }
    }
}

impl Default for FElem {
    fn default() -> Self {
        Self {
            elem_: RcCell::new(ElemType::Const(FConst::from(0))),
        }
    }
}
impl From<u64> for FElem {
    fn from(rhs: u64) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Const(FConst::from(rhs))),
        }
    }
}
impl From<i64> for FElem {
    fn from(rhs: i64) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Const(FConst::from(rhs))),
        }
    }
}
impl From<i32> for FElem {
    fn from(rhs: i32) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Const(FConst::from(rhs))),
        }
    }
}
impl From<usize> for FElem {
    fn from(rhs: usize) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Const(FConst::from(rhs))),
        }
    }
}
impl From<Fp> for FElem {
    fn from(rhs: Fp) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Elem(R1P_Elem::from(rhs))),
        }
    }
}
impl From<R1P_Elem> for FElem {
    fn from(rhs: R1P_Elem) -> Self {
        Self {
            elem_: RcCell::new(ElemType::Elem(rhs)),
        }
    }
}
impl FElemInterface for FElem {
    fn asString(&self) -> String {
        self.elem_.borrow().asString()
    }
    fn fieldType(&self) -> FieldType {
        self.elem_.borrow().fieldType()
    }

    fn asLong(&self) -> i64 {
        self.elem_.borrow().asLong()
    }
    fn inverse(&self) -> Self {
        self.elem_.borrow().inverse().into()
    }
    fn getBit(&self, i: u32) -> i32 {
        self.elem_.borrow().getBit(i)
    }
    fn power(&self, exponent: u64) -> Self {
        self.elem_.borrow().power(exponent).into()
    }
}
use std::fmt;
impl fmt::Display for FElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.elem_.borrow().asString())
    }
}

impl FElem {
    pub fn assignment(&mut self, other: Self) {
        if self.fieldType() == other.fieldType() || self.fieldType() == FieldType::AGNOSTIC {
            self.elem_ = other.elem_.clone();
            return;
        }
        assert!(
            other.fieldType() == FieldType::AGNOSTIC,
            "Attempted to assign field element of incorrect pub type"
        );

        self.elem_ = RcCell::new(ElemType::Const(FConst::from(other.elem_.borrow().asLong())));
    }
    pub fn fieldMustBePromotedForArithmetic(lhsField: &FieldType, rhsField: &FieldType) -> bool {
        lhsField != rhsField && rhsField != &FieldType::AGNOSTIC
    }

    pub fn promoteToFieldType(&self, types: &FieldType) {
        if !Self::fieldMustBePromotedForArithmetic(&self.fieldType(), &types) {
            return;
        }
        assert!(
            types == &FieldType::R1P,
            "Attempted to promote to unknown field pub type"
        );
        let fConst = self.elem_.try_borrow();
        assert!(
            fConst.is_ok(),
            "Cannot convert between specialized field types."
        );
        *self.elem_.borrow_mut() = ElemType::Elem(R1P_Elem::from(fConst.unwrap().asLong()));
    }

    pub fn inverses(&self, fieldType: &FieldType) -> Self {
        self.promoteToFieldType(fieldType);
        FElem::from((self.elem_.borrow().inverse()))
    }

    pub fn getBits(&self, i: u32, fieldType: &FieldType) -> i32 {
        self.promoteToFieldType(fieldType);
        assert!(
            &self.fieldType() == fieldType,
            "Attempted to extract bits from incompatible field pub type."
        );
        self.elem_.borrow().getBit(i)
    }

    pub fn powers(base: &Self, exponent: u64) -> Self {
        // TODO .cpp
        let retval = base.clone();
        retval.elem_.borrow().power(exponent);
        retval
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
impl AddAssign<&ElemType> for FConst {
    #[inline]
    fn add_assign(&mut self, other: &ElemType) {
        self.contents_ += other.try_as_const_ref().unwrap().contents_;
    }
}

impl SubAssign<&ElemType> for FConst {
    #[inline]
    fn sub_assign(&mut self, other: &ElemType) {
        self.contents_ -= other.try_as_const_ref().unwrap().contents_;
    }
}

impl MulAssign<&ElemType> for FConst {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &ElemType) {
        self.contents_ *= other.try_as_const_ref().unwrap().contents_;
    }
}
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
impl From<u64> for FConst {
    fn from(rhs: u64) -> Self {
        Self {
            contents_: rhs as i64,
        }
    }
}
impl From<i64> for FConst {
    fn from(rhs: i64) -> Self {
        Self { contents_: rhs }
    }
}
impl From<i32> for FConst {
    fn from(rhs: i32) -> Self {
        Self {
            contents_: rhs as i64,
        }
    }
}
impl From<usize> for FConst {
    fn from(rhs: usize) -> Self {
        Self {
            contents_: rhs as i64,
        }
    }
}

impl PartialEq for FConst {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.contents_ == other.contents_
    }
}
impl PartialEq<u64> for FConst {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.contents_ == *other as i64
    }
}
impl FElemInterface for FConst {
    fn asString(&self) -> String {
        format!("{}", self.contents_)
    }
    fn fieldType(&self) -> FieldType {
        FieldType::AGNOSTIC
    }
    fn asLong(&self) -> i64 {
        self.contents_
    }
    fn getBit(&self, i: u32) -> i32 {
        panic!("Cannot get bit from FConst.");
    }
    fn inverse(&self) -> Self {
        panic!("Attempted to invert an FConst element.");
    }

    fn power(&self, exponent: u64) -> Self {
        let mut res = self.clone();
        res.contents_ = ((self.contents_ as f64).powf((exponent as f64)) + 0.5) as i64;
        res
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
impl AddAssign<u64> for R1P_Elem {
    #[inline]
    fn add_assign(&mut self, other: u64) {}
}
impl AddAssign<&ElemType> for R1P_Elem {
    #[inline]
    fn add_assign(&mut self, other: &ElemType) {
        if other.fieldType() == FieldType::R1P {
            self.elem_ += &other.try_as_elem_ref().unwrap().elem_;
        } else if other.fieldType() == FieldType::AGNOSTIC {
            self.elem_ += other.try_as_elem_ref().unwrap().asLong();
        } else {
            panic!("Attempted to add incompatible pub type to R1P_Elem.");
        }
    }
}

impl SubAssign<&ElemType> for R1P_Elem {
    #[inline]
    fn sub_assign(&mut self, other: &ElemType) {
        if other.fieldType() == FieldType::R1P {
            self.elem_ -= &other.try_as_elem_ref().unwrap().elem_;
        } else if other.fieldType() == FieldType::AGNOSTIC {
            self.elem_ -= other.try_as_const_ref().unwrap().asLong();
        } else {
            panic!("Attempted to add incompatible pub type to R1P_Elem.");
        }
    }
}

impl MulAssign<&ElemType> for R1P_Elem {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &ElemType) {
        if other.fieldType() == FieldType::R1P {
            self.elem_ *= &other.try_as_elem_ref().unwrap().elem_;
        } else if other.fieldType() == FieldType::AGNOSTIC {
            self.elem_ *= other.try_as_const_ref().unwrap().asLong();
        } else {
            panic!("Attempted to add incompatible pub type to R1P_Elem.");
        }
    }
}
// R1P_Elem& pub fn operator+=(other:&FElemInterface) {
// 	if other.fieldType() ==FieldType::R1P {
// 		elem_ += dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() ==FieldType::AGNOSTIC {
// 		elem_ += dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible pub type to R1P_Elem.");
// 	}
// 	return self;
// }

// R1P_Elem& pub fn operator-=(other:&FElemInterface) {
// 	if other.fieldType() ==FieldType::R1P {
// 		elem_ -= dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() ==FieldType::AGNOSTIC {
// 		elem_ -= dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible pub type to R1P_Elem.");
// 	}
// 	return self;
// }

// R1P_Elem& pub fn operator*=(other:&FElemInterface) {
// 	if other.fieldType() ==FieldType::R1P {
// 		elem_ *= dynamic_cast<const R1P_Elem&>(other).elem_;
// 	} else if other.fieldType() ==FieldType::AGNOSTIC {
// 		elem_ *= dynamic_cast<const FConst&>(other).asLong();
// 	} else {
// 		eyre::bail!("Attempted to add incompatible pub type to R1P_Elem.");
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
// 	eyre::bail!("Attempted to Compare R1P_Elem with incompatible pub type.");
// }

impl From<u64> for R1P_Elem {
    fn from(rhs: u64) -> Self {
        Self {
            elem_: Fp::from(rhs),
        }
    }
}
impl From<i64> for R1P_Elem {
    fn from(rhs: i64) -> Self {
        Self {
            elem_: Fp::from(rhs),
        }
    }
}
impl From<FConst> for R1P_Elem {
    fn from(rhs: FConst) -> Self {
        Self {
            elem_: Fp::from(rhs.asLong()),
        }
    }
}
impl From<Fp> for R1P_Elem {
    fn from(rhs: Fp) -> Self {
        Self { elem_: rhs }
    }
}
impl PartialEq<FConst> for R1P_Elem {
    #[inline]
    fn eq(&self, other: &FConst) -> bool {
        self.elem_ == Fp::from(other.asLong())
    }
}
impl PartialEq<u64> for R1P_Elem {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        self.elem_ == Fp::from(*other)
    }
}
impl FElemInterface for R1P_Elem {
    fn inverse(&self) -> Self {
        R1P_Elem::from(self.elem_.inverse())
    }

    fn asLong(&self) -> i64 {
        //assert!(elem_.as_ulong() <= LONG_MAX, "overflow:u64 occured.");
        self.elem_.as_ulong() as _
    }
    fn asString(&self) -> String {
        format!("{}", self.elem_.as_ulong())
    }
    fn fieldType(&self) -> FieldType {
        FieldType::R1P
    }

    fn getBit(&self, i: u32) -> i32 {
        self.elem_.as_bigint().test_bit(i) as _
    }
    fn power(&self, exponent: u64) -> Self {
        let mut res = self.clone();
        res.elem_ ^= exponent;
        res
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
use std::sync::atomic::{self, AtomicUsize, Ordering};
pub static nextFreeIndex_: AtomicUsize = AtomicUsize::new(0); //VarIndex_t
impl Variable {
    pub fn new(name: &str) -> Self {
        let index_ = nextFreeIndex_.load(Ordering::Relaxed) as u64;
        nextFreeIndex_.fetch_add(1, Ordering::Relaxed);
        assert!(
            nextFreeIndex_.load(Ordering::Relaxed) > 0,
            "Variable index overflow has occured, maximum number of Variables is {}",
            u64::MAX
        );
        Self {
            index_,
            name_: name.to_owned(),
        }
    }

    pub fn name(&self) -> String {
        self.name_.clone()
    }

    pub fn eval(&self, assignment: &VariableAssignment) -> FElem {
        // try {
        assignment
            .get(self)
            .expect(&format!(
                "Attempted to evaluate unassigned Variable \"{}\", idx:{}",
                self.name(),
                self.index_
            ))
            .clone()
        // } catch (::std::out_of_range) {
        // eyre::bail!(
        // 		format!(
        // 				"Attempted to evaluate unassigned Variable \"{}\", idx:{}",
        // 				name(), index_));
        // }
    }
    // jSNARK-edit: A simple getter for the Variable index
    pub fn getIndex(&self) -> VarIndex_t {
        self.index_
    }
}
impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.index_.cmp(&other.index_))
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

impl<T: SubVariableArrayConfig> From<String> for VariableArray<T> {
    fn from(rhs: String) -> Self {
        Self {
            contents: vec![],
            name_: rhs,
            t: T::default(),
        }
    }
}
impl<T: SubVariableArrayConfig> From<i32> for VariableArray<T> {
    fn from(rhs: i32) -> Self {
        Self {
            contents: vec![],
            name_: String::new(),
            t: T::default(),
        }
    }
}
impl<T: SubVariableArrayConfig> From<usize> for VariableArray<T> {
    fn from(rhs: usize) -> Self {
        Self {
            contents: vec![],
            name_: String::new(),
            t: T::default(),
        }
    }
}

impl<T: SubVariableArrayConfig> Index<usize> for VariableArray<T> {
    type Output = Variable;

    fn index(&self, index: usize) -> &Self::Output {
        self.contents.get(index).unwrap()
    }
}

impl<T: SubVariableArrayConfig> IndexMut<usize> for VariableArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.contents.get_mut(index).unwrap()
    }
}
impl<T: SubVariableArrayConfig> IntoIterator for VariableArray<T> {
    type Item = Variable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl<T: SubVariableArrayConfig> VariableArray<T> {
    pub fn new(size: usize, name: String, t: T) -> Self {
        // : VariableArrayContents()
        let mut contents = VariableArrayContents::default();
        for i in 0..size {
            contents.push(Variable::new(&format!("{}[{}]", name, i)));
        }
        Self {
            contents,
            name_: name,
            t,
        }
    }
    pub fn new_with_variable(size: i32, contents: Variable, t: T) -> Self {
        Self {
            contents: vec![contents; size as usize],
            name_: String::new(),
            t,
        }
    }
}

impl<T: SubVariableArrayConfig> VariableArrayConfig for VariableArray<T> {
    fn iter(&self) -> std::slice::Iter<Variable> {
        self.contents.iter()
    }
    fn name(&self) -> &String {
        &self.name_
    }
    fn at(&self, i: usize) -> &Variable {
        &self.contents[i]
    }
    fn push(&mut self, val: Variable) {
        self.contents.push(val)
    }
    fn len(&self) -> usize {
        self.contents.len()
    }
    fn resize(&mut self, numBits: usize) {
        let sz = self.t.resize(numBits);
        self.contents.resize(sz, Variable::default());
    }
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/
impl SubVariableArrayConfig for UnpackedWord {}
impl UnpackedWord {
    pub fn new(numBits: usize, name: &str) -> VariableArray<Self> {
        VariableArray::<Self>::new(numBits, name.to_owned(), Self)
    }
    pub fn into_va(self) -> VariableArray<Self> {
        VariableArray::<Self>::new(0, String::new(), self)
    }
}

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 Custom Variable classes                    ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
impl From<FieldType> for VariableArray<MultiPackedWord> {
    fn from(rhs: FieldType) -> Self {
        Self {
            contents: vec![],
            name_: String::new(),
            t: MultiPackedWord {
                numBits_: 0,
                fieldType_: rhs,
            },
        }
    }
}
impl SubVariableArrayConfig for MultiPackedWord {
    fn resize(&mut self, numBits: usize) -> usize {
        self.numBits_ = numBits;
        Self::getMultipackedSize(&self.fieldType_)
        // self.resize(packedSize);
    }
}

impl MultiPackedWord {
    pub fn new(numBits: usize, fieldType: &FieldType, name: &str) -> VariableArray<Self> {
        // VariableArray(), numBits_(numBits), fieldType_(fieldType)
        let packedSize = Self::getMultipackedSize(fieldType);
        VariableArray::<Self>::new(
            packedSize,
            name.to_owned(),
            Self {
                numBits_: numBits,
                fieldType_: fieldType.clone(),
            },
        )
    }
    pub fn into_va(self) -> VariableArray<Self> {
        VariableArray::<Self>::new(0, String::new(), self)
    }

    pub fn getMultipackedSize(fieldType: &FieldType) -> usize {
        let mut packedSize = 0;
        assert!(
            fieldType == &FieldType::R1P,
            "Unknown field pub type for packed variable."
        );
        packedSize = 1; // TODO add assertion that numBits can fit in the field characteristic

        return packedSize;
    }
}

impl From<FieldType> for DualWord {
    fn from(rhs: FieldType) -> Self {
        Self {
            multipacked_: VariableArray::<MultiPackedWord>::from(rhs),
            unpacked_: VariableArray::<UnpackedWord>::default(),
        }
    }
}
impl DualWord {
    pub fn new(numBits: usize, fieldType: &FieldType, name: &String) -> Self {
        Self {
            multipacked_: MultiPackedWord::new(numBits, fieldType, &(name.to_owned() + "_p")),
            unpacked_: UnpackedWord::new(numBits, &(name.to_owned() + "_u")),
        }
    }

    pub fn new2(
        multipacked: VariableArray<MultiPackedWord>,
        unpacked: VariableArray<UnpackedWord>,
    ) -> Self {
        Self {
            multipacked_: multipacked,
            unpacked_: unpacked,
        }
    }
    pub fn multipacked(&self) -> VariableArray<MultiPackedWord> {
        self.multipacked_.clone()
    }
    pub fn unpacked(&self) -> VariableArray<UnpackedWord> {
        self.unpacked_.clone()
    }
    pub fn bit(&self, i: usize) -> FlagVariable {
        self.unpacked_[i].clone()
    } //syntactic sugar, same as unpacked()[i]
    pub fn numBits(&self) -> usize {
        self.unpacked_.len()
    }
    pub fn resize(&mut self, newSize: usize) {
        self.multipacked_.resize(newSize);
        self.unpacked_.resize(newSize);
    }
}
impl From<FieldType> for DualWordArray {
    fn from(rhs: FieldType) -> Self {
        Self {
            multipackedContents_: vec![VariableArray::<MultiPackedWord>::from(rhs)],
            unpackedContents_: vec![],
            numElements_: 0,
        }
    }
}
impl DualWordArray {
    pub fn new(
        multipackedContents: MultiPackedWordArray, // TODO delete, for dev
        unpackedContents: UnpackedWordArray,
    ) -> Self {
        let multipackedContents_ = multipackedContents;
        let unpackedContents_ = unpackedContents;
        let numElements_ = multipackedContents_.len();
        assert!(
            multipackedContents_.len() == numElements_,
            "Dual Variable multipacked contents size mismatch"
        );
        assert!(
            unpackedContents_.len() == numElements_,
            "Dual Variable packed contents size mismatch"
        );
        Self {
            multipackedContents_,
            unpackedContents_,
            numElements_,
        }
    }

    pub fn multipacked(&self) -> &MultiPackedWordArray {
        &self.multipackedContents_
    }
    pub fn unpacked(&self) -> &UnpackedWordArray {
        &self.unpackedContents_
    }
    pub fn packed(&self) -> PackedWordArray {
        assert!(
            self.numElements_ == self.multipackedContents_.len(),
            "multipacked contents size mismatch"
        );
        let mut retval = PackedWordArray::from(self.numElements_);
        for i in 0..self.numElements_ {
            let element = self.multipackedContents_[i].clone();
            assert!(
                element.len() == 1,
                "Cannot convert from multipacked to packed"
            );
            retval[i] = element[0].clone();
        }
        return retval;
    }

    pub fn push(&mut self, dualWord: &DualWord) {
        self.multipackedContents_.push(dualWord.multipacked());
        self.unpackedContents_.push(dualWord.unpacked());
        self.numElements_ += 1;
    }

    pub fn at(&self, i: usize) -> DualWord {
        //let multipackedRep= multipacked()[i];
        //let unpackedRep= unpacked()[i];
        //const DualWord retval(multipackedRep, unpackedRep);
        //return retval;
        DualWord::new2(self.multipacked()[i].clone(), self.unpacked()[i].clone())
    }

    pub fn len(&self) -> usize {
        assert!(
            self.multipackedContents_.len() == self.numElements_,
            "Dual Variable multipacked contents size mismatch"
        );
        assert!(
            self.unpackedContents_.len() == self.numElements_,
            "Dual Variable packed contents size mismatch"
        );
        return self.numElements_;
    }
}
/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    pub struct LinearTerm                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

//     LinearTerm operator-() const {return LinearTerm(variable_, -coeff_);}

//     // jSNARK-edit: These two operators are overloaded to support combining common factors for the same variables.
//     LinearTerm& operator-=(other:&FElem) {coeff_ -= other; return self;}
//     LinearTerm& operator+=(other:&FElem) {coeff_ += other; return self;}

//     LinearTerm& operator*=(other:&FElem) {coeff_ *= other; return self;}

impl Neg for LinearTerm {
    type Output = Self;

    fn neg(self) -> Self::Output {
        LinearTerm::new(self.variable_.clone(), -self.coeff_.clone())
    }
}
impl AddAssign<&FElem> for LinearTerm {
    #[inline]
    fn add_assign(&mut self, other: &FElem) {
        self.coeff_ += other;
    }
}

impl SubAssign<&FElem> for LinearTerm {
    #[inline]
    fn sub_assign(&mut self, other: &FElem) {
        self.coeff_ -= other;
    }
}

impl MulAssign<&FElem> for LinearTerm {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &FElem) {
        self.coeff_ *= other;
    }
}

impl From<Variable> for LinearTerm {
    fn from(rhs: Variable) -> Self {
        Self {
            variable_: rhs,
            coeff_: 1.into(),
        }
    }
}
impl LinearTerm {
    pub fn new(v: Variable, coeff: FElem) -> Self {
        Self {
            variable_: v,
            coeff_: coeff,
        }
    }
    pub fn new2(v: Variable, n: i64) -> Self {
        Self {
            variable_: v,
            coeff_: n.into(),
        }
    }
    // jSNARK-edit: A simple getter for the coefficient
    pub fn coeff(&self) -> FElem {
        self.coeff_.clone()
    }
    pub fn fieldtype(&self) -> FieldType {
        self.coeff_.fieldType()
    }
    pub fn variable(&self) -> Variable {
        self.variable_.clone()
    }
    pub fn asString(&self) -> String {
        match self.coeff_.asLong() {
            1 => self.variable_.name(),
            -1 => {
                format!("-1 * {}", self.variable_.name())
            }
            0 => {
                format!("0 * {}", self.variable_.name())
            }
            _ => {
                format!("{} * {}", self.coeff_.asString(), self.variable_.name())
            }
        }
    }

    pub fn eval(&self, assignment: &VariableAssignment) -> FElem {
        FElem::from(self.coeff_.clone()) * &self.variable_.eval(assignment)
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

impl AddAssign<&Self> for LinearCombination {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        // jSNARK-edit: This method is modified in order to reduce memory consumption when the same variable is
        // being added to a linear combination object multiple times.
        // This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.

        if self.indexMap_.len() == 0 {
            self.linearTerms_.extend(other.linearTerms_.clone());
            self.constant_ += &other.constant_;
        } else {
            for lt in other.linearTerms_.iter() {
                if let Some(v) = self.indexMap_.get(&(lt.variable().getIndex() as i32)) {
                    self.linearTerms_[(*v) as usize] += &lt.coeff();
                } else {
                    self.linearTerms_.push(lt.clone());
                    let k = self.indexMap_.len();
                    self.indexMap_
                        .insert(lt.variable().getIndex() as i32, k as i32);
                }
            }
            self.constant_ += &other.constant_;
        }

        // heuristic threshold
        if self.linearTerms_.len() > 10 && self.indexMap_.len() == 0 {
            let mut i = 0;
            let mut newVec = vec![];
            let mut it = self.linearTerms_.iter();
            while let Some(lt) = it.next() {
                if let Some(v) = self.indexMap_.get(&(lt.variable().getIndex() as i32)) {
                    newVec[(*v) as usize] += &lt.coeff();
                } else {
                    newVec.push(lt.clone());
                    self.indexMap_.insert(lt.variable().getIndex() as i32, i);
                    i += 1;
                }
            }
            self.linearTerms_ = newVec;
        }
    }
}

impl SubAssign<&Self> for LinearCombination {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        // jSNARK-edit: This method is rewritten in order to reduce memory consumption when the same variable is
        // being added to a linear combination object multiple times.
        // This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.
        if self.indexMap_.len() == 0 {
            for lt in other.linearTerms_.iter() {
                self.linearTerms_.push(-lt.clone());
            }
            self.constant_ -= &other.constant_;
        } else {
            for lt in &other.linearTerms_ {
                if let Some(v) = self.indexMap_.get(&(lt.variable().getIndex() as i32)) {
                    self.linearTerms_[(*v) as usize] -= &lt.coeff();
                } else {
                    self.linearTerms_.push(-lt.clone());
                    let k = self.indexMap_.len();
                    self.indexMap_
                        .insert(lt.variable().getIndex() as i32, k as i32);
                }
            }
            self.constant_ -= &other.constant_;
        }

        // heuristic threshold
        if self.linearTerms_.len() > 10 && self.indexMap_.len() == 0 {
            let mut i = 0;
            let mut newVec = vec![];
            let mut it = self.linearTerms_.iter();

            while let Some(lt) = it.next() {
                if let Some(v) = self.indexMap_.get(&(lt.variable().getIndex() as i32)) {
                    newVec[(*v) as usize] += &lt.coeff();
                } else {
                    newVec.push(lt.clone());
                    self.indexMap_.insert(lt.variable().getIndex() as i32, i);
                    i += 1;
                }
            }
            self.linearTerms_ = newVec;
        }
    }
}

impl MulAssign<&FElem> for LinearCombination {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &FElem) {
        self.constant_ *= other;
        for lt in self.linearTerms_.iter_mut() {
            *lt *= other;
        }
    }
}

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
// 				linearTerms_.push(lt);
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
// 				newVec.push(*lt);
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
// 			linearTerms_.push(-lt);
// 		}
// 		constant_ -= other.constant_;
// 	} else {
// 		for lt in &other.linearTerms_ {
// 			if indexMap_.find(lt.variable().getIndex()) != indexMap_.end() {
// 				linearTerms_[indexMap_[lt.variable().getIndex()]] -= lt.coeff();
// 			} else {
// 				linearTerms_.push(-lt);
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
// 				newVec.push(*lt);
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
impl Neg for LinearCombination {
    type Output = Self;

    fn neg(self) -> Self::Output {
        LinearCombination::from(0) - &self
    }
}

impl Default for LinearCombination {
    fn default() -> Self {
        Self {
            linearTerms_: vec![],
            indexMap_: HashMap::new(),
            constant_: FElem::from(0),
        }
    }
}
impl From<Variable> for LinearCombination {
    fn from(rhs: Variable) -> Self {
        Self {
            linearTerms_: vec![rhs.into()],
            indexMap_: HashMap::new(),
            constant_: FElem::from(0),
        }
    }
}
impl From<LinearTerm> for LinearCombination {
    fn from(rhs: LinearTerm) -> Self {
        Self {
            linearTerms_: vec![rhs],
            indexMap_: HashMap::new(),
            constant_: FElem::from(0),
        }
    }
}
impl From<u64> for LinearCombination {
    fn from(rhs: u64) -> Self {
        Self {
            linearTerms_: vec![],
            indexMap_: HashMap::new(),
            constant_: FElem::from(rhs),
        }
    }
}
impl From<FElem> for LinearCombination {
    fn from(rhs: FElem) -> Self {
        Self {
            linearTerms_: vec![],
            indexMap_: HashMap::new(),
            constant_: rhs,
        }
    }
}
impl LinearCombination {
    pub fn eval(&self, assignment: &VariableAssignment) -> FElem {
        let mut evaluation = self.constant_.clone();
        for lt in &self.linearTerms_ {
            evaluation += &lt.eval(assignment);
        }
        return evaluation;
    }

    pub fn asString(&self) -> String {
        let mut retval = String::new();
        let mut it = self.linearTerms_.iter();
        if let Some(v) = it.next() {
            retval += &v.asString();
        } else {
            return self.constant_.asString();
        }
        for v in it {
            retval += &(" + ".to_owned() + &v.asString());
        }
        if self.constant_.asLong() != 0 {
            retval += &(" + ".to_owned() + &self.constant_.asString());
        }
        retval
    }

    pub fn getUsedVariables(&self) -> BTreeSet<Variable> {
        let mut retSet = BTreeSet::new();
        for lt in &self.linearTerms_ {
            retSet.insert(lt.variable());
        }
        return retSet;
    }
}
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

pub fn sum(inputs: &VariableArrayType) -> LinearCombination {
    let mut retval = LinearCombination::default();
    for var in inputs.iter() {
        retval += &(var.clone().into());
    }
    return retval;
}

pub fn negate(lc: &LinearCombination) -> LinearCombination {
    LinearCombination::from(1) - lc
}
/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        pub struct Monomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

impl From<Variable> for Monomial {
    fn from(rhs: Variable) -> Self {
        Self {
            variables_: BTreeMap::from([(rhs, 1)]),
            coeff_: FElem::from(1),
        }
    }
}
impl From<LinearTerm> for Monomial {
    fn from(rhs: LinearTerm) -> Self {
        Self {
            coeff_: rhs.coeff_.clone(),
            variables_: BTreeMap::from([(rhs.variable_.clone(), 1)]),
        }
    }
}

impl From<FElem> for Monomial {
    fn from(rhs: FElem) -> Self {
        Self {
            variables_: BTreeMap::new(),
            coeff_: rhs,
        }
    }
}
impl Monomial {
    pub fn new(var: Variable, coeff: FElem) -> Self {
        Self {
            coeff_: coeff,
            variables_: BTreeMap::from([(var, 1)]),
        }
    }

    pub fn eval(&self, assignment: &VariableAssignment) -> FElem {
        let mut retval = self.coeff_.clone();
        for var in &self.variables_ {
            retval *= &var.0.eval(assignment);
        }
        return retval;
    }

    pub fn getUsedVariables(&self) -> BTreeMap<Variable, i32> {
        self.variables_.clone()
    }

    pub fn getCoefficient(&self) -> FElem {
        self.coeff_.clone()
    }

    pub fn asString(&self) -> String {
        if self.variables_.len() == 0 {
            return self.coeff_.asString();
        }
        let mut retval = String::new();
        if self.coeff_ != 1 {
            retval += &(self.coeff_.asString() + "*");
        }
        let mut iter = self.variables_.iter();
        retval += &iter.next().unwrap().0.name();
        while let Some(v) = iter.next() {
            retval += &("*".to_owned() + &v.0.name());
        }
        retval
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
impl Neg for Monomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut retval = self;
        retval.coeff_ = -retval.coeff_;
        retval
    }
}
impl AddAssign<&Self> for Monomial {
    #[inline]
    fn add_assign(&mut self, other: &Self) {}
}

impl SubAssign<&Self> for Monomial {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {}
}

impl MulAssign<&Self> for Monomial {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &Self) {
        self.coeff_ *= &other.coeff_;
        self.variables_.extend(other.variables_.clone());
    }
}

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
impl Default for Polynomial {
    fn default() -> Self {
        Self {
            monomials_: vec![],
            constant_: FElem::from(0),
        }
    }
}
impl From<Monomial> for Polynomial {
    fn from(rhs: Monomial) -> Self {
        Self {
            monomials_: vec![rhs],
            constant_: FElem::from(0),
        }
    }
}
impl From<Variable> for Polynomial {
    fn from(rhs: Variable) -> Self {
        Self {
            monomials_: vec![Monomial::from(rhs)],
            constant_: FElem::from(0),
        }
    }
}
impl From<LinearCombination> for Polynomial {
    fn from(rhs: LinearCombination) -> Self {
        let mut monomials_ = vec![];
        for linearTerm in &rhs.linearTerms_ {
            monomials_.push(Monomial::from(linearTerm.clone()));
        }
        Self {
            monomials_,
            constant_: rhs.constant_.clone(),
        }
    }
}
impl From<LinearTerm> for Polynomial {
    fn from(rhs: LinearTerm) -> Self {
        Self {
            monomials_: vec![Monomial::from(rhs)],
            constant_: FElem::from(0),
        }
    }
}
impl From<i32> for Polynomial {
    fn from(rhs: i32) -> Self {
        Self {
            monomials_: vec![],
            constant_: FElem::from(rhs),
        }
    }
}
impl From<FElem> for Polynomial {
    fn from(rhs: FElem) -> Self {
        Self {
            monomials_: vec![],
            constant_: rhs,
        }
    }
}
impl Polynomial {
    pub fn eval(&self, assignment: &VariableAssignment) -> FElem {
        let mut retval = self.constant_.clone();
        for monomial in &self.monomials_ {
            retval += &monomial.eval(assignment);
        }
        return retval;
    }

    pub fn getUsedVariables(&self) -> BTreeMap<Variable, i32> {
        let mut retset = BTreeMap::new();
        for monomial in &self.monomials_ {
            let mut curSet = monomial.getUsedVariables();
            retset.append(&mut curSet);
        }
        retset
    }

    pub fn getMonomials(&self) -> Vec<Monomial> {
        self.monomials_.clone()
    }

    pub fn getConstant(&self) -> FElem {
        self.constant_.clone()
    }

    pub fn asString(&self) -> String {
        if self.monomials_.len() == 0 {
            return self.constant_.asString();
        }
        let mut retval = String::new();
        let mut iter = self.monomials_.iter();
        retval += &iter.next().unwrap().asString();
        while let Some(v) = iter.next() {
            retval += &(" + ".to_owned() + &v.asString());
        }
        if self.constant_ != 0 {
            retval += &(" + ".to_owned() + &self.constant_.asString());
        }
        return retval;
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
// 			newMonomials.push(thisMonomial * otherMonomial);
// 		}
// 		newMonomials.push(thisMonomial * other.constant_);
// 	}
// 	for otherMonomial in &other.monomials_ {
// 		newMonomials.push(otherMonomial * self.constant_);
// 	}
// 	constant_ *= other.constant_;
// 	monomials_ = ::(newMonomials);
// 	return self;
// }

// Polynomial& pub fn operator-=(other:&Polynomial) {
// 	constant_ -= other.constant_;
// 	for otherMonomial in &other.monomials_ {
// 		monomials_.push(-otherMonomial);
// 	}
// 	return self;
// }
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/
impl Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Polynomial::from(FElem::from(0)) - &self
    }
}
impl AddAssign<&LinearTerm> for Polynomial {
    #[inline]
    fn add_assign(&mut self, other: &LinearTerm) {
        *self += &Polynomial::from(Monomial::from(other.clone()));
    }
}

impl AddAssign<&Self> for Polynomial {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.constant_ += &other.constant_;
        self.monomials_.extend(other.monomials_.clone());
    }
}

impl SubAssign<&Self> for Polynomial {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        self.constant_ -= &other.constant_.clone();
        for otherMonomial in &other.monomials_ {
            self.monomials_.push(-otherMonomial.clone());
        }
    }
}

impl MulAssign<&Self> for Polynomial {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    fn mul_assign(&mut self, other: &Self) {
        let mut newMonomials = vec![];
        for thisMonomial in &self.monomials_ {
            for otherMonomial in &other.monomials_ {
                newMonomials.push(thisMonomial.clone() * otherMonomial);
            }
            newMonomials.push(thisMonomial.clone() * &other.constant_);
        }
        for otherMonomial in &other.monomials_ {
            newMonomials.push(otherMonomial.clone() * &self.constant_);
        }
        self.constant_ *= &other.constant_;
        self.monomials_ = newMonomials;
    }
}
