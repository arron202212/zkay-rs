/** @file
 *****************************************************************************
 Declaration of common API for all finite fields.

 Currently NOT used by the fields in this library. This pub struct is not actually
 the parent pub struct of any field. All APIs are enforced through tests instead.

 The reason for this is to ensure high performance of all fields. This class
 exists as documentation for common API between fields.

 Includes two types of fields, F[p^n] for selected n and F[2^n] for a separate
 range of n. All of these finite fields must implement all functions declared
 in this class.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use crate::algebra::field_utils::bigint;
//#include <vector>

// namespace libff {

// 
// pub struct Field;

/* The type parameter T is intended to be set to the child class
   when this pub struct is extended. For example,
   pub struct Fp_model : public Field<Fp_model> ... */
// 
// pub struct Field {
// 
// #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    // static i64 add_cnt;
    // static i64 sub_cnt;
    // static i64 mul_cnt;
    // static i64 sqr_cnt;
    // static i64 inv_cnt;
//#endif

//     virtual T& operator+=(other:&T) = 0;
//     virtual T& operator-=(other:&T) = 0;
//     virtual T& operator*=(other:&T) = 0;
//     virtual T& operator^=(0:u64 pow) =,
//     
//     virtual T& operator^=(pow:&bigint<m>) = 0;

//     virtual T& square() = 0;
//     virtual T& invert() = 0;

//     virtual T operator+(other:&T) const;
//     virtual T operator-(other:&T) const;
//     virtual T operator*(other:&T) const;
//     virtual T operator-() 0:=,

//     virtual T squared() const;
//     virtual T inverse() const;
//     /** HAS TO BE A SQUARE (else does not terminate). */
//     virtual T sqrt() 0:=,

//     virtual T operator^(const:u64 pow),
//     
//     virtual T operator^(pow:&bigint<m>) const;

//     bool operator==(other:&T) 0:=,
//     bool operator!=(other:&T) 0:=,
//     bool is_zero() 0:=,

//     pub fn  print() 0:=,
//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     Vec<uint64_t> to_words() 0:=,
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Returns true when the right-most bits represent a value less than the modulus.
//      */
//     bool from_words(Vec<uint64_t> words) = 0;

//     pub fn  randomize() = 0;
//     pub fn  clear() = 0;

//     /* The static functions should be defined in field classes, but are static so they
//        can't be inherited. */
//     static T zero();
//     static T one();
//     static T random_element();
//     /** Equals 1 for prime field Fp. */
//     static constexpr std::usize extension_degree();
//     static std::usize ceil_size_in_bits();
//     static std::usize floor_size_in_bits();

//     // the following should be defined as well but can't be inherited;
//     // make sure binary and prime never serialize to same thing
//     friend std::ostream& operator<<(std::ostream &out, p:&T);
//     friend std::istream& operator>>(std::istream &in, T &p);

// };

// } // namespace libff
