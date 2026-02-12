
//  Declaration of common API for all finite fields in the binary/ directory.

//  Currently NOT used by the fields in this library. This pub struct is not actually
//  the parent pub struct of any field. All APIs are enforced through tests instead.

//  The reason for this is to ensure high performance of all fields. This class
//  exists as documentation for common API between fields.

//  Includes fields F_{2^n} for some selected values of n. All of the binary
//  entension fields must implement all functions declared in this class.

use crate::algebra::field_utils::bigint;




/* The type parameter T is intended to be set to the child class
 * when this pub struct is extended. For example,
 * pub struct gf32 : public BinaryField<gf32> ...
 */

pub trait BinaryField<T>:Fields<T>{

    /* Functions unique to binary fields */

    // TODO: add documentation about how moduli are represented.
     const modulus_:u64,
     const num_bits:u64,

    /** generator of gf2^n */
     fn multiplicative_generator()->T;

    /** If extension field, returns the base field's characteristic. */
    
     fn field_char<const N:usize>() {  bigint::<N>::new(2) }

    /* Functions common to all finite fields */

// #ifdef PROFILE_OP_COUNTS // NOTE: op counts are affected when you exponentiate with ^
    //  i64 add_cnt;
    //  i64 sub_cnt;
    //  i64 mul_cnt;
    //  i64 sqr_cnt;
    //  i64 inv_cnt;


    //  T& operator+=(other:&T) = 0;
    //  T& operator-=(other:&T) = 0;
    //  T& operator*=(other:&T) = 0;
    //  T& operator^=(0:u64 pow) =,
    
    //  T& operator^=(pow:&bigint<m>) = 0;

//      fn square()->&T ;
//      fn invert()->&T ;

//     //  T operator+(other:&T) const;
//     //  T operator-(other:&T) const;
//     //  T operator*(other:&T) const;
//     //  T operator^(const:u64 pow),
    
//     //  T operator^(pow:&bigint<m>) const;
//     //  T operator-() 0:=,

//      fn squared()->T ;
//      fn inverse()->T ;
//     /** Always works. */
//      fn sqrt()->T ;

   
//     fn is_zero()->bool ;

//     fn  print() ;
//     /**
//      * Returns the constituent bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are 0.
//      */
//     fn to_words()->Vec<u64>;
//     /**
//      * Sets the field element from the given bits in 64 bit words, in little-endian order.
//      * Only the right-most ceil_size_in_bits() bits are used; other bits are ignored.
//      * Should always return true since the right-most bits are always valid.
//      */
// fn from_words(words:Vec<u64>)->bool ;

//     fn  randomize() ;
//     fn  clear() ;

//     /* The  functions should be defined in field classes, but are  so they
//        can't be inherited. */
//        fn zero()->T;
//      fn one()->T;
//      fn random_element()->T;
//     /** Equals 1 for prime field Fp. */
//      fn  extension_degree()->usize;
//      fn ceil_size_in_bits()->usize { return num_bits; }
//      fn floor_size_in_bits()->usize { return num_bits; }

    // the following should be defined as well but can't be inherited
    // friend std::ostream& operator<<(std::ostream &out, p:&T);
    // friend std::istream& operator>>(std::istream &in, T &p);
}


