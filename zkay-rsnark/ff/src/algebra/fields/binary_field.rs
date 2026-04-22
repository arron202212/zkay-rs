
//  Declaration of common API for all finite fields in the binary/ directory.

//  Currently NOT used by the fields in this library. This pub struct is not actually
//  the parent pub struct of any field. All APIs are enforced through tests instead.

//  The reason for this is to ensure high performance of all fields. This class
//  exists as documentation for common API between fields.

//  Includes fields F_{2^n} for some selected values of n. All of the binary
//  entension fields must implement all functions declared in this class.

use crate::algebra::field_utils::bigint;




//  The type parameter T is intended to be set to the child class
// when this pub struct is extended. For example,
// pub struct gf32 : public BinaryField<gf32> ...


pub trait BinaryField<T>:Fields<T>{

    //Functions unique to binary fields

    // TODO: add documentation about how moduli are represented.
     const modulus_:u64,
     const num_bits:u64,

   //generator of gf2^n
     fn multiplicative_generator()->T;

   //If extension field, returns the base field's characteristic.
    
     fn field_char<const N:usize>() {  bigint::<N>::new(2) }

    //Functions common to all finite fields


