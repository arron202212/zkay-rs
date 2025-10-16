/** @file
 *****************************************************************************
 Declaration of the low level objects needed for field arithmetization.
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLE_HPP_
// #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_VARIABLE_HPP_

use  <cstddef>
use  <iostream>
use  <map>
use  <set>
use  <string>
use  <unordered_set>
use  <utility>
use  <vector>

use crate::gadgetlib2::infrastructure;
use crate::gadgetlib2::pp;

namespace gadgetlib2 {

class GadgetLibAdapter;

// Forward declarations
class Protoboard;
class FElemInterface;
class FElem;
class FConst;
class Variable;
class VariableArray;

type enum {R1P, AGNOSTIC} FieldType;

type ::std::shared_ptr<Variable> VariablePtr;
type ::std::shared_ptr<VariableArray> VariableArrayPtr;
type ::std::unique_ptr<FElemInterface> FElemInterfacePtr;
type ::std::shared_ptr<Protoboard> ProtoboardPtr;
type unsigned long VarIndex_t;

// Naming Conventions:
// R1P == Rank 1 Prime characteristic

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                   class FElemInterface                     ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/**
    An interface class for field elements.
    Currently 2 classes will derive from this interface:
    R1P_Elem - Elements of a field of prime characteristic
    FConst - Formally not a field, only placeholders for field agnostic constants, such as 0 and 1.
             Can be used for -1 or any other constant which makes semantic sense in all fields.
 */
class FElemInterface {
public:
    virtual FElemInterface& operator=(const long n) = 0;
    /// FConst will be field agnostic, allowing us to hold values such as 0 and 1 without knowing
    /// the underlying field. This assignment operator will convert to the correct field element.
    virtual FElemInterface& operator=(const FConst& src) = 0;
    virtual ::std::string asString() const = 0;
    virtual FieldType fieldType() const = 0;
    virtual FElemInterface& operator+=(const FElemInterface& other) = 0;
    virtual FElemInterface& operator-=(const FElemInterface& other) = 0;
    virtual FElemInterface& operator*=(const FElemInterface& other) = 0;
    virtual bool operator==(const FElemInterface& other) const = 0;
    virtual bool operator==(const FConst& other) const = 0;
    /// This operator is not always mathematically well defined. 'n' will be checked in runtime
    /// for fields in which integer values are not well defined.
    virtual bool operator==(const long n) const = 0;
    /// @returns a unique_ptr to a copy of the current element.
    virtual FElemInterfacePtr clone() const = 0;
    virtual FElemInterfacePtr inverse() const = 0;
    virtual long asLong() const = 0;
    virtual int getBit(unsigned int i) const = 0;
    virtual FElemInterface& power(long exponent) = 0;
    virtual ~FElemInterface(){};
}; // class FElemInterface

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

inline bool operator==(const long first, const FElemInterface& second) {return second == first;}
inline bool operator!=(const long first, const FElemInterface& second) {return !(first == second);}
inline bool operator!=(const FElemInterface& first, const long second) {return !(first == second);}
inline bool operator!=(const FElemInterface& first, const FElemInterface& second) {
    return !(first == second);
}

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class FElem                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/// A wrapper class for field elements. Can hold any derived type of FieldElementInterface
class FElem {
private:
    FElemInterfacePtr elem_;

public:
    explicit FElem(const FElemInterface& elem);
    /// Helper method. When doing arithmetic between a constant and a field specific element
    /// we want to "promote" the constant to the same field. This function changes the unique_ptr
    /// to point to a field specific element with the same value as the constant which it held.
    void promoteToFieldType(FieldType type);
    FElem();
    FElem(const long n);
    FElem(const int i);
    FElem(const size_t n);
    FElem(const Fp& elem);
    FElem(const FElem& src);

    FElem& operator=(const FElem& other);
    FElem& operator=(FElem&& other);
    FElem& operator=(const long i) { *elem_ = i; return *this;}
    ::std::string asString() const {return elem_->asString();}
    FieldType fieldType() const {return elem_->fieldType();}
    bool operator==(const FElem& other) const {return *elem_ == *other.elem_;}
    FElem& operator*=(const FElem& other);
    FElem& operator+=(const FElem& other);
    FElem& operator-=(const FElem& other);
    FElem operator-() const {FElem retval(0); retval -= FElem(*elem_); return retval;}
    FElem inverse(const FieldType& fieldType);
    long asLong() const {return elem_->asLong();}
    int getBit(unsigned int i, const FieldType& fieldType);
    friend FElem power(const FElem& base, long exponent);

    inline friend ::std::ostream& operator<<(::std::ostream& os, const FElem& elem) {
       return os << elem.elem_->asString();
    }

    friend class GadgetLibAdapter;
}; // class FElem

inline bool operator!=(const FElem& first, const FElem& second) {return !(first == second);}

/// These operators are not always mathematically well defined. The long will be checked in runtime
/// for fields in which values other than 0 and 1 are not well defined.
inline bool operator==(const FElem& first, const long second) {return first == FElem(second);}
inline bool operator==(const long first, const FElem& second) {return second == first;}
inline bool operator!=(const FElem& first, const long second) {return !(first == second);}
inline bool operator!=(const long first, const FElem& second) {return !(first == second);}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class FConst                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/**
    A field agnostic constant. All fields have constants 1 and 0 and this class allows us to hold
    an element agnostically while the context field is not known. For example, when given the
    very useful expression '1 - x' where x is a field agnostic formal variable, we must store the
    constant '1' without knowing over which field this expression will be evaluated.
    Constants can also hold integer values, which will be evaluated if possible, in runtime. For
    instance the expression '42 + x' will be evaluated in runtime in the trivial way when working
    over the prime characteristic Galois Field GF_43 but will cause a runtime error when evaluated
    over a GF2 extension field in which '42' has no obvious meaning, other than being the answer to
    life, the universe and everything.
*/
class FConst : public FElemInterface {
private:
    long contents_;
    explicit FConst(const long n) : contents_(n) {}
public:
    virtual FConst& operator=(const long n) {contents_ = n; return *this;}
    virtual FConst& operator=(const FConst& src) {contents_ = src.contents_; return *this;}
    virtual ::std::string asString() const {return GADGETLIB2_FMT("%ld",contents_);}
    virtual FieldType fieldType() const {return AGNOSTIC;}
    virtual FConst& operator+=(const FElemInterface& other);
    virtual FConst& operator-=(const FElemInterface& other);
    virtual FConst& operator*=(const FElemInterface& other);
    virtual bool operator==(const FElemInterface& other) const {return other == *this;}
    virtual bool operator==(const FConst& other) const {return contents_ == other.contents_;}
    virtual bool operator==(const long n) const {return contents_ == n;}
    /// @return a unique_ptr to a new copy of the element
    virtual FElemInterfacePtr clone() const {return FElemInterfacePtr(new FConst(*this));}
    /// @return a unique_ptr to a new copy of the element's multiplicative inverse
    virtual FElemInterfacePtr inverse() const;
    long asLong() const {return contents_;}
    int getBit(unsigned int i) const { ffec::UNUSED(i); GADGETLIB_FATAL("Cannot get bit from FConst."); }
    virtual FElemInterface& power(long exponent);

    friend class FElem; // allow constructor call
}; // class FConst

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     class R1P_Elem                         ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
/**
    Holds elements of a prime characteristic field. Currently implemented using the gmp (Linux) and
    mpir (Windows) libraries.
 */
class R1P_Elem : public FElemInterface {
private:
    Fp elem_;
public:

    explicit R1P_Elem(const Fp& elem) : elem_(elem) {}
    virtual R1P_Elem& operator=(const FConst& src) {elem_ = src.asLong(); return *this;}
    virtual R1P_Elem& operator=(const long n) {elem_ = Fp(n); return *this;}
    virtual ::std::string asString() const {return GADGETLIB2_FMT("%u", elem_.as_ulong());}
    virtual FieldType fieldType() const {return R1P;}
    virtual R1P_Elem& operator+=(const FElemInterface& other);
    virtual R1P_Elem& operator-=(const FElemInterface& other);
    virtual R1P_Elem& operator*=(const FElemInterface& other);
    virtual bool operator==(const FElemInterface& other) const;
    virtual bool operator==(const FConst& other) const {return elem_ == Fp(other.asLong());}
    virtual bool operator==(const long n) const {return elem_ == Fp(n);}
    /// @return a unique_ptr to a new copy of the element
    virtual FElemInterfacePtr clone() const {return FElemInterfacePtr(new R1P_Elem(*this));}
    /// @return a unique_ptr to a new copy of the element's multiplicative inverse
    virtual FElemInterfacePtr inverse() const;
    long asLong() const;
    int getBit(unsigned int i) const {return elem_.as_bigint().test_bit(i);}
    virtual FElemInterface& power(long exponent) {elem_^= exponent; return *this;}

    friend class FElem; // allow constructor call
    friend class GadgetLibAdapter;
};

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    class Variable                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

/**
    @brief A formal variable, field agnostic.

    Each variable is specified by an index. This can be imagined as the index in x_1, x_2,..., x_i
    These are formal variables and do not hold an assignment, later the class VariableAssignment
    will give each formal variable its own assignment.
    Variables have no comparison and assignment operators as evaluating (x_1 == x_2) has no sense
    without specific assignments.
    Variables are field agnostic, this means they can be used regardless of the context field,
    which will also be determined by the assignment.
 */
class Variable {
private:
    VarIndex_t index_;  ///< This index differentiates and identifies Variable instances.
    static VarIndex_t nextFreeIndex_; ///< Monotonically-increasing counter to allocate disinct indices.
// #ifdef DEBUG
    ::std::string name_;
//#endif

   /**
    * @brief allocates the variable
    */
public:
    explicit Variable(const ::std::string& name = "");
    virtual ~Variable();

    ::std::string name() const;

    /// A functor for strict ordering of Variables. Needed for STL containers.
    /// This is not an ordering of Variable assignments and has no semantic meaning.
    struct VariableStrictOrder {
        bool operator()(const Variable& first, const Variable& second)const {
            return first.index_ < second.index_;
        }
    };

    type ::std::map<Variable, FElem, Variable::VariableStrictOrder> VariableAssignment;
    FElem eval(const VariableAssignment& assignment) const;

    /// A set of Variables should be declared as follows:    Variable::set s1;
    type ::std::set<Variable, VariableStrictOrder> set;
    type ::std::multiset<Variable, VariableStrictOrder> multiset;

    // jSNARK-edit: A simple getter for the Variable index
    int getIndex() const { return index_;}

    friend class GadgetLibAdapter;
}; // class Variable
/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

type ::std::map<Variable, FElem, Variable::VariableStrictOrder> VariableAssignment;

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 class VariableArray                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

type ::std::vector<Variable> VariableArrayContents;

class VariableArray : public VariableArrayContents {
private:
#   ifdef DEBUG
    ::std::string name_;
#   endif
public:
    explicit VariableArray(const ::std::string& name = "");
    explicit VariableArray(const int size, const ::std::string& name = "");
    explicit VariableArray(const size_t size, const ::std::string& name = "");
    explicit VariableArray(const size_t size, const Variable& contents)
            : VariableArrayContents(size, contents) {}

    using VariableArrayContents::operator[];
    using VariableArrayContents::at;
    using VariableArrayContents::push_back;
    using VariableArrayContents::size;

    ::std::string name() const;
}; // class VariableArray

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

type Variable FlagVariable; ///< Holds variable whose purpose is to be populated with a boolean
                               ///< value, Field(0) or Field(1)
type VariableArray FlagVariableArray;
type Variable PackedWord;   ///< Represents a packed word that can fit in a field element.
                               ///< For a word representing an unsigned integer for instance this
                               ///< means we require (int < fieldSize)
type VariableArray PackedWordArray;

/// Holds variables whose purpose is to be populated with the unpacked form of some word, bit by bit
class UnpackedWord : public VariableArray {
public:
    UnpackedWord() : VariableArray() {}
    UnpackedWord(const size_t numBits, const ::std::string& name) : VariableArray(numBits, name) {}
}; // class UnpackedWord

type ::std::vector<UnpackedWord> UnpackedWordArray;

/// Holds variables whose purpose is to be populated with the packed form of some word.
/// word representation can be larger than a single field element in small enough fields
class MultiPackedWord : public VariableArray {
private:
    size_t numBits_;
    FieldType fieldType_;
    size_t getMultipackedSize() const;
public:
    MultiPackedWord(const FieldType& fieldType = AGNOSTIC);
    MultiPackedWord(const size_t numBits, const FieldType& fieldType, const ::std::string& name);
    void resize(const size_t numBits);
    ::std::string name() const {return VariableArray::name();}
}; // class MultiPackedWord

type ::std::vector<MultiPackedWord> MultiPackedWordArray;

/// Holds both representations of a word, both multipacked and unpacked
class DualWord {
private:
    MultiPackedWord multipacked_;
    UnpackedWord unpacked_;
public:
    DualWord(const FieldType& fieldType) : multipacked_(fieldType), unpacked_() {}
    DualWord(const size_t numBits, const FieldType& fieldType, const ::std::string& name);
    DualWord(const MultiPackedWord& multipacked, const UnpackedWord& unpacked);
    MultiPackedWord multipacked() const {return multipacked_;}
    UnpackedWord unpacked() const {return unpacked_;}
    FlagVariable bit(size_t i) const {return unpacked_[i];} //syntactic sugar, same as unpacked()[i]
    size_t numBits() const { return unpacked_.size(); }
    void resize(size_t newSize);
}; // class DualWord

class DualWordArray {
private:
    // kept as 2 separate arrays because the more common usecase will be to request one of these,
    // and not dereference a specific DualWord
    MultiPackedWordArray multipackedContents_;
    UnpackedWordArray unpackedContents_;
    size_t numElements_;
public:
    DualWordArray(const FieldType& fieldType);
    DualWordArray(const MultiPackedWordArray& multipackedContents, // TODO delete, for dev
                  const UnpackedWordArray& unpackedContents);
    MultiPackedWordArray multipacked() const;
    UnpackedWordArray unpacked() const;
    PackedWordArray packed() const; //< For cases in which we can assume each unpacked value fits
                                    //< in 1 packed Variable
    void push_back(const DualWord& dualWord);
    DualWord at(size_t i) const;
    size_t size() const;
}; // class DualWordArray


/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     class LinearTerm                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

class LinearTerm {
private:
    Variable variable_;
    FElem coeff_;
public:
    LinearTerm(const Variable& v) : variable_(v), coeff_(1) {}
    LinearTerm(const Variable& v, const FElem& coeff) : variable_(v), coeff_(coeff) {}
    LinearTerm(const Variable& v, long n) : variable_(v), coeff_(n) {}
    LinearTerm operator-() const {return LinearTerm(variable_, -coeff_);}

    // jSNARK-edit: These two operators are overloaded to support combining common factors for the same variables.
    LinearTerm& operator-=(const FElem& other) {coeff_ -= other; return *this;}
    LinearTerm& operator+=(const FElem& other) {coeff_ += other; return *this;}

    LinearTerm& operator*=(const FElem& other) {coeff_ *= other; return *this;}
    FieldType fieldtype() const {return coeff_.fieldType();}
    ::std::string asString() const;
    FElem eval(const VariableAssignment& assignment) const;
    Variable variable() const {return variable_;}

    // jSNARK-edit: A simple getter for the coefficient
    FElem coeff() const {return coeff_;}

    friend class Monomial;
    friend class GadgetLibAdapter;
}; // class LinearTerm

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/



/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  class LinearCombination                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

class LinearCombination {
protected:
    ::std::vector<LinearTerm> linearTerms_;
    std::map<int, int> indexMap_; // jSNARK-edit: This map is used to reduce memory consumption. Can be helpful for some circuits produced by Pinocchio compiler.
    FElem constant_;
    type ::std::vector<LinearTerm>::size_type size_type;
public:
    LinearCombination() : linearTerms_(), constant_(0) {}
    LinearCombination(const Variable& var) : linearTerms_(1,var), constant_(0) {}
    LinearCombination(const LinearTerm& linTerm) : linearTerms_(1,linTerm), constant_(0) {}
    LinearCombination(long i) : linearTerms_(), constant_(i) {}
    LinearCombination(const FElem& elem) : linearTerms_(), constant_(elem) {}

    LinearCombination& operator+=(const LinearCombination& other);
    LinearCombination& operator-=(const LinearCombination& other);
    LinearCombination& operator*=(const FElem& other);
    FElem eval(const VariableAssignment& assignment) const;
    ::std::string asString() const;
    const Variable::set getUsedVariables() const;

    friend class Polynomial;
    friend class GadgetLibAdapter;
}; // class LinearCombination

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

inline LinearCombination operator-(const LinearCombination& lc){return LinearCombination(0) -= lc;}

LinearCombination sum(const VariableArray& inputs);
//TODO : change this to member function
LinearCombination negate(const LinearCombination& lc);

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                       class Monomial                       ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

class Monomial {
private:
    FElem coeff_;
    Variable::multiset variables_; // currently just a vector of variables. This can
                                   // surely be optimized e.g. hold a variable-degree pair
                                   // but is not needed for concrete efficiency as we will
                                   // only be invoking degree 2 constraints in the near
                                   // future.
public:
    Monomial(const Variable& var) : coeff_(1), variables_() {variables_.insert(var);}
    Monomial(const Variable& var, const FElem& coeff) : coeff_(coeff), variables_() {variables_.insert(var);}
    Monomial(const FElem& val) : coeff_(val), variables_() {}
    Monomial(const LinearTerm& linearTerm);

    FElem eval(const VariableAssignment& assignment) const;
    const Variable::set getUsedVariables() const;
    const FElem getCoefficient() const;
    ::std::string asString() const;
    Monomial operator-() const;
    Monomial& operator*=(const Monomial& other);
}; // class Monomial

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/


/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class Polynomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

class Polynomial {
private:
    ::std::vector<Monomial> monomials_;
    FElem constant_;
public:
    Polynomial() : monomials_(), constant_(0) {}
    Polynomial(const Monomial& monomial) : monomials_(1, monomial), constant_(0) {}
    Polynomial(const Variable& var) : monomials_(1, Monomial(var)), constant_(0) {}
    Polynomial(const FElem& val) : monomials_(), constant_(val) {}
    Polynomial(const LinearCombination& linearCombination);
    Polynomial(const LinearTerm& linearTerm) : monomials_(1, Monomial(linearTerm)), constant_(0) {}
    Polynomial(int i) : monomials_(), constant_(i) {}

    FElem eval(const VariableAssignment& assignment) const;
    const Variable::set getUsedVariables() const;
    const std::vector<Monomial>& getMonomials()const;
    const FElem getConstant()const;
    ::std::string asString() const;
    Polynomial& operator+=(const Polynomial& other);
    Polynomial& operator*=(const Polynomial& other);
    Polynomial& operator-=(const Polynomial& other);
    Polynomial& operator+=(const LinearTerm& other) {return *this += Polynomial(Monomial(other));}
}; // class Polynomial

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

inline Polynomial operator-(const Polynomial& src) {return Polynomial(FElem(0)) -= src;}

} // namespace gadgetlib2

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

use  <climits>
use  <iostream>
use  <set>
use  <stdexcept>
use  <vector>

use crate::gadgetlib2::infrastructure;
use crate::gadgetlib2::pp;
use crate::gadgetlib2::variable;

using ::std::string;
using ::std::stringstream;
using ::std::set;
using ::std::vector;
using ::std::shared_ptr;
using ::std::cout;
using ::std::endl;
using ::std::dynamic_pointer_cast;

namespace gadgetlib2 {

// Optimization: In the future we may want to port most of the member functions  from this file to
// the .hpp files in order to allow for compiler inlining. As inlining has tradeoffs this should be
// profiled before doing so.

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class FElem                           ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

FElem::FElem(const FElemInterface& elem) :
		elem_(elem.clone()) {
}
FElem::FElem() :
		elem_(new FConst(0)) {
}
FElem::FElem(const long n) :
		elem_(new FConst(n)) {
}
FElem::FElem(const int i) :
		elem_(new FConst(i)) {
}
FElem::FElem(const size_t n) :
		elem_(new FConst(n)) {
}
FElem::FElem(const Fp& elem) :
		elem_(new R1P_Elem(elem)) {
}
FElem::FElem(const FElem& src) :
		elem_(src.elem_->clone()) {
}

FElem& FElem::operator=(const FElem& other) {
	if fieldType() == other.fieldType() || fieldType() == AGNOSTIC {
		elem_ = other.elem_->clone();
	} else if other.fieldType() != AGNOSTIC {
		GADGETLIB_FATAL("Attempted to assign field element of incorrect type");
	} else {
		*elem_ = dynamic_cast<FConst*>(other.elem_.get())->asLong();
	}
	return *this;
}

FElem& FElem::operator=(FElem&& other) {
	if fieldType() == other.fieldType() || fieldType() == AGNOSTIC {
		elem_ = ::(other.elem_);
	} else if other.elem_->fieldType() != AGNOSTIC {
		GADGETLIB_FATAL(
				"Attempted to move assign field element of incorrect type");
	} else {
		*elem_ = dynamic_cast<FConst*>(other.elem_.get())->asLong();
	}
	return *this;
}

bool fieldMustBePromotedForArithmetic(const FieldType& lhsField,
		const FieldType& rhsField) {
	if lhsField == rhsField
		return false;
	if rhsField == AGNOSTIC
		return false;
	return true;
}

void FElem::promoteToFieldType(FieldType type) {
	if !fieldMustBePromotedForArithmetic(self.fieldType(), type) {
		return;
	}
	if type == R1P {
		const FConst* fConst = dynamic_cast<FConst*>(elem_.get());
		GADGETLIB_ASSERT(fConst != NULL,
				"Cannot convert between specialized field types.");
		elem_.reset(new R1P_Elem(fConst->asLong()));
	} else {
		GADGETLIB_FATAL("Attempted to promote to unknown field type");
	}
}

FElem& FElem::operator*=(const FElem& other) {
	promoteToFieldType(other.fieldType());
	*elem_ *= *other.elem_;
	return *this;
}

FElem& FElem::operator+=(const FElem& other) {
	promoteToFieldType(other.fieldType());
	*elem_ += *other.elem_;
	return *this;
}

FElem& FElem::operator-=(const FElem& other) {
	promoteToFieldType(other.fieldType());
	*elem_ -= *other.elem_;
	return *this;
}

FElem FElem::inverse(const FieldType& fieldType) {
	promoteToFieldType(fieldType);
	return FElem(*(elem_->inverse()));
}

int FElem::getBit(unsigned int i, const FieldType& fieldType) {
    promoteToFieldType(fieldType);
    if self.fieldType() == fieldType {
        return elem_->getBit(i);
    } else {
        GADGETLIB_FATAL("Attempted to extract bits from incompatible field type.");
    }
}

FElem power(const FElem& base, long exponent) { // TODO .cpp
	FElem retval(base);
	retval.elem_->power(exponent);
	return retval;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class FConst                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

FConst& FConst::operator+=(const FElemInterface& other) {
	contents_ += dynamic_cast<const FConst&>(other).contents_;
	return *this;
}

FConst& FConst::operator-=(const FElemInterface& other) {
	contents_ -= dynamic_cast<const FConst&>(other).contents_;
	return *this;
}

FConst& FConst::operator*=(const FElemInterface& other) {
	contents_ *= dynamic_cast<const FConst&>(other).contents_;
	return *this;
}

FElemInterfacePtr FConst::inverse() const {
	GADGETLIB_FATAL("Attempted to invert an FConst element.");
}

FElemInterface& FConst::power(long exponent) {
	contents_ = 0.5 + ::std::pow(double(contents_), double(exponent));
	return *this;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                     class R1P_Elem                         ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

R1P_Elem& R1P_Elem::operator+=(const FElemInterface& other) {
	if other.fieldType() == R1P {
		elem_ += dynamic_cast<const R1P_Elem&>(other).elem_;
	} else if other.fieldType() == AGNOSTIC {
		elem_ += dynamic_cast<const FConst&>(other).asLong();
	} else {
		GADGETLIB_FATAL("Attempted to add incompatible type to R1P_Elem.");
	}
	return *this;
}

R1P_Elem& R1P_Elem::operator-=(const FElemInterface& other) {
	if other.fieldType() == R1P {
		elem_ -= dynamic_cast<const R1P_Elem&>(other).elem_;
	} else if other.fieldType() == AGNOSTIC {
		elem_ -= dynamic_cast<const FConst&>(other).asLong();
	} else {
		GADGETLIB_FATAL("Attempted to add incompatible type to R1P_Elem.");
	}
	return *this;
}

R1P_Elem& R1P_Elem::operator*=(const FElemInterface& other) {
	if other.fieldType() == R1P {
		elem_ *= dynamic_cast<const R1P_Elem&>(other).elem_;
	} else if other.fieldType() == AGNOSTIC {
		elem_ *= dynamic_cast<const FConst&>(other).asLong();
	} else {
		GADGETLIB_FATAL("Attempted to add incompatible type to R1P_Elem.");
	}
	return *this;
}

bool R1P_Elem::operator==(const FElemInterface& other) const {
	const R1P_Elem* pOther = dynamic_cast<const R1P_Elem*>(&other);
	if pOther {
		return elem_ == pOther->elem_;
	}
	const FConst* pConst = dynamic_cast<const FConst*>(&other);
	if pConst {
		return *this == *pConst;
	}
	GADGETLIB_FATAL("Attempted to Compare R1P_Elem with incompatible type.");
}

FElemInterfacePtr R1P_Elem::inverse() const {
	return FElemInterfacePtr(new R1P_Elem(elem_.inverse()));
}

long R1P_Elem::asLong() const {
	//GADGETLIB_ASSERT(elem_.as_ulong() <= LONG_MAX, "long overflow occured.");
	return long(elem_.as_ulong());
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    class Variable                          ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/
VarIndex_t Variable::nextFreeIndex_ = 0;

// #ifdef DEBUG
Variable::Variable(const string& name) : index_(nextFreeIndex_++), name_(name) {
	GADGETLIB_ASSERT(nextFreeIndex_ > 0, GADGETLIB2_FMT("Variable index overflow has occured, maximum number of "
					"Variables is %lu", ULONG_MAX));
}
#else
Variable::Variable(const string& name) : index_(nextFreeIndex_++) {
    ffec::UNUSED(name);
    GADGETLIB_ASSERT(nextFreeIndex_ > 0, GADGETLIB2_FMT("Variable index overflow has occured, maximum number of "
                                         "Variables is %lu", ULONG_MAX));
}
//#endif

Variable::~Variable() {
}
;

string Variable::name() const {
#    ifdef DEBUG
	return name_;
#    else
	return "";
#    endif
}

FElem Variable::eval(const VariableAssignment& assignment) const {
	try {
		return assignment.at(*this);
	} catch (::std::out_of_range) {
		GADGETLIB_FATAL(
				GADGETLIB2_FMT(
						"Attempted to evaluate unassigned Variable \"%s\", idx:%lu",
						name().c_str(), index_));
	}
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                 class VariableArray                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

// #ifdef DEBUG
VariableArray::VariableArray(const string& name) : VariableArrayContents(), name_(name) {}
VariableArray::VariableArray(const int size, const ::std::string& name) : VariableArrayContents() {
    for i in 0..size {
        push_back(Variable(GADGETLIB2_FMT("%s[%d]", name.c_str(), i)));
    }
}

VariableArray::VariableArray(const size_t size, const ::std::string& name) : VariableArrayContents() {
    for i in 0..size {
        push_back(Variable(GADGETLIB2_FMT("%s[%d]", name.c_str(), i)));
    }
}
::std::string VariableArray::name() const {
	return name_;
}

#else
::std::string VariableArray::name() const {
	return "";
}


VariableArray::VariableArray(const string& name) : VariableArrayContents() { ffec::UNUSED(name); }
VariableArray::VariableArray(const int size, const ::std::string& name)
    : VariableArrayContents(size) { ffec::UNUSED(name); }
VariableArray::VariableArray(const size_t size, const ::std::string& name)
    : VariableArrayContents(size) { ffec::UNUSED(name); }
//#endif

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

MultiPackedWord::MultiPackedWord(const FieldType& fieldType) :
		VariableArray(), numBits_(0), fieldType_(fieldType) {
}

MultiPackedWord::MultiPackedWord(const size_t numBits,
		const FieldType& fieldType, const ::std::string& name) :
		VariableArray(), numBits_(numBits), fieldType_(fieldType) {
	size_t packedSize = getMultipackedSize();
	VariableArray varArray(packedSize, name);
	VariableArray::swap(varArray);
}

void MultiPackedWord::resize(const size_t numBits) {
	numBits_ = numBits;
	size_t packedSize = getMultipackedSize();
	VariableArray::resize(packedSize);
}

size_t MultiPackedWord::getMultipackedSize() const {
	size_t packedSize = 0;
	if fieldType_ == R1P {
		packedSize = 1; // TODO add assertion that numBits can fit in the field characteristic
	} else {
		GADGETLIB_FATAL("Unknown field type for packed variable.");
	}
	return packedSize;
}

DualWord::DualWord(const size_t numBits, const FieldType& fieldType,
		const ::std::string& name) :
		multipacked_(numBits, fieldType, name + "_p"), unpacked_(numBits,
				name + "_u") {
}

DualWord::DualWord(const MultiPackedWord& multipacked,
		const UnpackedWord& unpacked) :
		multipacked_(multipacked), unpacked_(unpacked) {
}

void DualWord::resize(size_t newSize) {
	multipacked_.resize(newSize);
	unpacked_.resize(newSize);
}

DualWordArray::DualWordArray(const FieldType& fieldType) :
		multipackedContents_(0, MultiPackedWord(fieldType)), unpackedContents_(
				0), numElements_(0) {
}

DualWordArray::DualWordArray(const MultiPackedWordArray& multipackedContents, // TODO delete, for dev
		const UnpackedWordArray& unpackedContents) :
		multipackedContents_(multipackedContents), unpackedContents_(
				unpackedContents), numElements_(multipackedContents_.size()) {
	GADGETLIB_ASSERT(multipackedContents_.size() == numElements_,
			"Dual Variable multipacked contents size mismatch");
	GADGETLIB_ASSERT(unpackedContents_.size() == numElements_,
			"Dual Variable packed contents size mismatch");
}

MultiPackedWordArray DualWordArray::multipacked() const {
	return multipackedContents_;
}
UnpackedWordArray DualWordArray::unpacked() const {
	return unpackedContents_;
}
PackedWordArray DualWordArray::packed() const {
	GADGETLIB_ASSERT(numElements_ == multipackedContents_.size(),
			"multipacked contents size mismatch")
	PackedWordArray retval(numElements_);
	for i in 0..numElements_ {
		const auto element = multipackedContents_[i];
		GADGETLIB_ASSERT(element.size() == 1,
				"Cannot convert from multipacked to packed");
		retval[i] = element[0];
	}
	return retval;
}

void DualWordArray::push_back(const DualWord& dualWord) {
	multipackedContents_.push_back(dualWord.multipacked());
	unpackedContents_.push_back(dualWord.unpacked());
	numElements_+=1;
}

DualWord DualWordArray::at(size_t i) const {
	//const MultiPackedWord multipackedRep = multipacked()[i];
	//const UnpackedWord unpackedRep = unpacked()[i];
	//const DualWord retval(multipackedRep, unpackedRep);
	//return retval;
	return DualWord(multipacked()[i], unpacked()[i]);
}

size_t DualWordArray::size() const {
	GADGETLIB_ASSERT(multipackedContents_.size() == numElements_,
			"Dual Variable multipacked contents size mismatch");
	GADGETLIB_ASSERT(unpackedContents_.size() == numElements_,
			"Dual Variable packed contents size mismatch");
	return numElements_;
}

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                    class LinearTerm                        ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

::std::string LinearTerm::asString() const {
	if coeff_ == 1 {
		return variable_.name();
	} else if coeff_ == -1 {
		return GADGETLIB2_FMT("-1 * %s", variable_.name().c_str());
	} else if coeff_ == 0 {
		return GADGETLIB2_FMT("0 * %s", variable_.name().c_str());
	} else {
		return GADGETLIB2_FMT("%s * %s", coeff_.asString().c_str(),
				variable_.name().c_str());
	}
}

FElem LinearTerm::eval(const VariableAssignment& assignment) const {
	return FElem(coeff_) *= variable_.eval(assignment);
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                  class LinearCombination                   ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

LinearCombination& LinearCombination::operator+=(
		const LinearCombination& other) {

	// jSNARK-edit: This method is modified in order to reduce memory consumption when the same variable is
	// being added to a linear combination object multiple times.
	// This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.

	if indexMap_.size() == 0 {
		linearTerms_.insert(linearTerms_.end(), other.linearTerms_.cbegin(),
				other.linearTerms_.cend());
		constant_ += other.constant_;
	} else {
		for lt in &other.linearTerms_ {
			if indexMap_.find(lt.variable().getIndex()) != indexMap_.end() {
				linearTerms_[indexMap_[lt.variable().getIndex()]] += lt.coeff();
			} else {
				linearTerms_.push_back(lt);
				int k = indexMap_.size();
				indexMap_[lt.variable().getIndex()] = k;
			}
		}
		constant_ += other.constant_;
	}

	// heuristic threshold
	if linearTerms_.size() > 10 && indexMap_.size() == 0 {
		int i = 0;
		::std::vector<LinearTerm> newVec;
		::std::vector<LinearTerm>::iterator lt = (linearTerms_.begin());
		while (lt != linearTerms_.end()) {

			if indexMap_.find(lt->variable().getIndex()) != indexMap_.end() {
				newVec[indexMap_[lt->variable().getIndex()]] += lt->coeff();
			} else {
				newVec.push_back(*lt);
				indexMap_[lt->variable().getIndex()] = i++;

			}
			lt+=1;
		}
		linearTerms_ = newVec;
	}

	return *this;
}

LinearCombination& LinearCombination::operator-=(
		const LinearCombination& other) {

	// jSNARK-edit: This method is rewritten in order to reduce memory consumption when the same variable is
	// being added to a linear combination object multiple times.
	// This can be helpful for some of the circuits produced by the Pinocchio compiler in some cases.
	if indexMap_.size() == 0 {
		for lt in &other.linearTerms_ {
			linearTerms_.push_back(-lt);
		}
		constant_ -= other.constant_;
	} else {
		for lt in &other.linearTerms_ {
			if indexMap_.find(lt.variable().getIndex()) != indexMap_.end() {
				linearTerms_[indexMap_[lt.variable().getIndex()]] -= lt.coeff();
			} else {
				linearTerms_.push_back(-lt);
				int k = indexMap_.size();
				indexMap_[lt.variable().getIndex()] = k;
			}
		}
		constant_ -= other.constant_;
	}

	// heuristic threshold
	if linearTerms_.size() > 10 && indexMap_.size() == 0 {
		int i = 0;
		::std::vector<LinearTerm> newVec;
		::std::vector<LinearTerm>::iterator lt = (linearTerms_.begin());

		while (lt != linearTerms_.end()) {

			if indexMap_.find(lt->variable().getIndex()) != indexMap_.end() {
				newVec[indexMap_[lt->variable().getIndex()]] += lt->coeff();
			} else {
				newVec.push_back(*lt);
				indexMap_[lt->variable().getIndex()] = i++;
			}
			lt+=1;
		}
		linearTerms_ = newVec;
	}

	return *this;

}

LinearCombination& LinearCombination::operator*=(const FElem& other) {
	constant_ *= other;
	for lt in &linearTerms_ {
		lt *= other;
	}
	return *this;
}

FElem LinearCombination::eval(const VariableAssignment& assignment) const {
	FElem evaluation = constant_;
	for lt in &linearTerms_ {
		evaluation += lt.eval(assignment);
	}
	return evaluation;
}

::std::string LinearCombination::asString() const {
// #ifdef DEBUG
	::std::string retval;
	auto it = linearTerms_.begin();
	if it == linearTerms_.end() {
		return constant_.asString();
	} else {
		retval += it->asString();
	}
	for(it+=1; it != linearTerms_.end(); ++it) {
		retval += " + " + it->asString();
	}
	if constant_ != 0 {
		retval += " + " + constant_.asString();
	}
	return retval;
#else // ifdef DEBUG
	return "";
//#endif // ifdef DEBUG
}

const Variable::set LinearCombination::getUsedVariables() const {
	Variable::set retSet;
	for lt in &linearTerms_ {
		retSet.insert(lt.variable());
	}
	return retSet;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

LinearCombination sum(const VariableArray& inputs) {
	LinearCombination retval(0);
	for var in &inputs {
		retval += var;
	}
	return retval;
}

LinearCombination negate(const LinearCombination& lc) {
	return (1 - lc);
}

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                        class Monomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

Monomial::Monomial(const LinearTerm& linearTerm) :
		coeff_(linearTerm.coeff_), variables_() {
	variables_.insert(linearTerm.variable_);
}

FElem Monomial::eval(const VariableAssignment& assignment) const {
	FElem retval = coeff_;
	for var in &variables_ {
		retval *= var.eval(assignment);
	}
	return retval;
}

const Variable::set Monomial::getUsedVariables() const {
	return Variable::set(variables_.begin(), variables_.end());
}

const FElem Monomial::getCoefficient() const {
	return coeff_;
}

::std::string Monomial::asString() const {
// #ifdef DEBUG
	if variables_.size() == 0 {
		return coeff_.asString();
	}
	string retval;
	if coeff_ != 1 {
		retval += coeff_.asString() + "*";
	}
	auto iter = variables_.begin();
	retval += iter->name();
	for(iter+=1; iter != variables_.end(); ++iter) {
		retval += "*" + iter->name();
	}
	return retval;
#else // ifdef DEBUG
	return "";
//#endif // ifdef DEBUG
}

Monomial Monomial::operator-() const {
	Monomial retval = *this;
	retval.coeff_ = -retval.coeff_;
	return retval;
}

Monomial& Monomial::operator*=(const Monomial& other) {
	coeff_ *= other.coeff_;
	variables_.insert(other.variables_.begin(), other.variables_.end());
	return *this;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

/*************************************************************************************************/
/*************************************************************************************************/
/*******************                                                            ******************/
/*******************                      class Polynomial                      ******************/
/*******************                                                            ******************/
/*************************************************************************************************/
/*************************************************************************************************/

Polynomial::Polynomial(const LinearCombination& linearCombination) :
		monomials_(), constant_(linearCombination.constant_) {
	for linearTerm in &linearCombination.linearTerms_ {
		monomials_.push_back(Monomial(linearTerm));
	}
}

FElem Polynomial::eval(const VariableAssignment& assignment) const {
	FElem retval = constant_;
	for monomial in &monomials_ {
		retval += monomial.eval(assignment);
	}
	return retval;
}

const Variable::set Polynomial::getUsedVariables() const {
	Variable::set retset;
	for monomial in &monomials_ {
		const Variable::set curSet = monomial.getUsedVariables();
		retset.insert(curSet.begin(), curSet.end());
	}
	return retset;
}

const vector<Monomial>& Polynomial::getMonomials() const {
	return monomials_;
}

const FElem Polynomial::getConstant() const {
	return constant_;
}

::std::string Polynomial::asString() const {
#   ifndef DEBUG
	return "";
#   endif
	if monomials_.size() == 0 {
		return constant_.asString();
	}
	string retval;
	auto iter = monomials_.begin();
	retval += iter->asString();
	for (iter+=1; iter != monomials_.end(); ++iter) {
		retval += " + " + iter->asString();
	}
	if constant_ != 0 {
		retval += " + " + constant_.asString();
	}
	return retval;
}

Polynomial& Polynomial::operator+=(const Polynomial& other) {
	constant_ += other.constant_;
	monomials_.insert(monomials_.end(), other.monomials_.begin(),
			other.monomials_.end());
	return *this;
}

Polynomial& Polynomial::operator*=(const Polynomial& other) {
	vector<Monomial> newMonomials;
	for thisMonomial in &monomials_ {
		for otherMonomial in &other.monomials_ {
			newMonomials.push_back(thisMonomial * otherMonomial);
		}
		newMonomials.push_back(thisMonomial * other.constant_);
	}
	for otherMonomial in &other.monomials_ {
		newMonomials.push_back(otherMonomial * self.constant_);
	}
	constant_ *= other.constant_;
	monomials_ = ::(newMonomials);
	return *this;
}

Polynomial& Polynomial::operator-=(const Polynomial& other) {
	constant_ -= other.constant_;
	for otherMonomial in &other.monomials_ {
		monomials_.push_back(-otherMonomial);
	}
	return *this;
}

/***********************************/
/***   END OF CLASS DEFINITION   ***/
/***********************************/

} // namespace gadgetlib2
