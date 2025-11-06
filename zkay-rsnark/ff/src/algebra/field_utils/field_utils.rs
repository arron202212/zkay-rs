#![allow(incomplete_features, dead_code, non_upper_case_globals)]
// #![feature(generic_const_exprs, generic_const_items)]
// /** @file
//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

//#ifndef FIELD_UTILS_HPP_
// #define FIELD_UTILS_HPP_
//#include <cstdint>

use crate::algebra::field_utils::bigint::bigint;
use crate::common::double;
use crate::algebra::field_utils::bigint::GMP_NUMB_BITS;
use num_traits::{One,Zero};
 use crate::common::utils::{log2,div_ceil,bit_vector};
use crate::algebra::fields::binary::gf64;
use crate::algebra::fields::binary::gf128;
use crate::algebra::fields::binary::gf192;
use crate::algebra::fields::binary::gf256;
use crate::algebra::fields::prime_base::fp;

// namespace libff {

pub trait is_additive{
    const  value:bool =false;
}

// struct is_additive {
//     static let mut value = false;
// };

// 
// struct is_additive<gf64> {
//     static let mut value = true;
// };

// 
// struct is_additive<gf128> {
//     static let mut value = true;
// };

// 
// struct is_additive<gf192> {
//     static let mut value = true;
// };

// 
// struct is_additive<gf256> {
//     static let mut value = true;
// };

pub trait is_multiplicative{
    const  value:bool =false;
}

// struct is_multiplicative {
//     static let mut value = false;
// };

// 
// struct is_multiplicative<Fp_model<n, modulus>> {
//     static let mut value = true;
// };

enum field_type {
    multiplicative_field_type = 1,
    additive_field_type = 2
}

// 
// field_type get_field_type(const enable_if<is_multiplicative<FieldT>::value, FieldT>::type elem);

// 
// field_type get_field_type(const enable_if<is_additive<FieldT>::value, FieldT>::type elem);

// 
// std::usize log_of_field_size_helper(
//     enable_if<is_multiplicative<FieldT>::value, FieldT>::type field_elem);

// 
// std::usize log_of_field_size_helper(
//     enable_if<is_additive<FieldT>::value, FieldT>::type field_elem);

// 
// std::usize soundness_log_of_field_size_helper(
//     enable_if<is_multiplicative<FieldT>::value, FieldT>::type field_elem);

// 
// std::usize soundness_log_of_field_size_helper(
//     enable_if<is_additive<FieldT>::value, FieldT>::type field_elem);

// 
// std::usize get_word_of_field_elem(
//     enable_if<is_additive<FieldT>::value, FieldT>::type field_elem, usize word);

// 
// std::usize get_word_of_field_elem(
//     enable_if<is_multiplicative<FieldT>::value, FieldT>::type field_elem, usize word);

// 
// FieldT coset_shift();

// // returns root of unity of order n (for n a power of 2), if one exists
// 
// std::enable_if<std::is_same<FieldT, Double>::value, FieldT>::type
// get_root_of_unity(const std::usize n);

// 
// std::enable_if<!std::is_same<FieldT, Double>::value, FieldT>::type
// get_root_of_unity(const std::usize n);

// 
// Vec<FieldT> pack_int_vector_into_field_element_vector(v:&Vec<std::usize>, const std::usize w);

// 
// Vec<FieldT> pack_bit_vector_into_field_element_vector(v:&bit_vector, const std::usize chunk_bits);

// 
// Vec<FieldT> pack_bit_vector_into_field_element_vector(v:&bit_vector);

// 
// Vec<FieldT> convert_bit_vector_to_field_element_vector(v:&bit_vector);

// 
// bit_vector convert_field_element_vector_to_bit_vector(v:&Vec<FieldT>);

// 
// bit_vector convert_field_element_to_bit_vector(el:FieldT);

// 
// bit_vector convert_field_element_to_bit_vector(el:FieldT, const std::usize bitcount);

// 
// FieldT convert_bit_vector_to_field_element(v:&bit_vector);

// 
// pub fn  batch_invert(Vec<FieldT> &vec);

// } // namespace libff
// use ffec::algebra::field_utils::/field_utils.tcc;

//#endif // FIELD_UTILS_HPP_


/** @file
 *****************************************************************************
 Implementation of misc. math and serialization utility functions
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FIELD_UTILS_TCC_
// #define FIELD_UTILS_TCC_

//#include <complex>
//#include <stdexcept>

// namespace libff {

// using std::usize;

trait FTConfig{
    const NUM_LIMBS:usize;

}

 pub fn get_field_type_is_multiplicative<FieldT>( elem:FieldT)->field_type
{
    //UNUSED(elem); // only to identify field type
    return field_type::multiplicative_field_type;
}


 pub fn get_field_type_is_additive<FieldT>( elem:FieldT)->field_type
{
    //UNUSED(elem); // only to identify field type
    return field_type::additive_field_type;
}

pub fn log_of_field_size_helper_is_multiplicative<FieldT>( field_elem:FieldT)->usize
{
    //UNUSED(field_elem);
    // return FieldT::ceil_size_in_bits();
    0
}

pub fn  log_of_field_size_helper_is_additive<FieldT>(field_elem:FieldT)->usize
{
    //UNUSED(field_elem);
    // return FieldT::extension_degree();
0
}

pub fn soundness_log_of_field_size_helper<FieldT>(field_elem:FieldT)->usize
{
    //UNUSED(field_elem);
    /** size in bits is the number of bits needed to represent a field element.
     *  However there isn't perfect alignment between the number of bits and the number of field elements,
     *  there could be a factor of two difference.
     *  For calculating soundness, we use the log of field size as number of bits - 1,
     *  as (2 << returned) size lower bounds the actual size.
    */
    // return FieldT::ceil_size_in_bits() - 1;
    0
}

pub fn soundness_log_of_field_size_helper_is_additive<FieldT>( field_elem:FieldT)->usize
{
    //UNUSED(field_elem);
    // return FieldT::extension_degree();
    0
}

pub fn  get_word_of_field_elem_is_additive<FieldT>( field_elem:FieldT,  word:usize)->usize
{
    // return field_elem.to_words()[word];
    0
}

pub fn  get_word_of_field_elem_is_multiplicative<FieldT>(field_elem:FieldT,  word:usize)->usize
{
    // return field_elem.as_bigint().0.0[word];
    0
}

 pub fn coset_shift<FieldT:Default>()->FieldT
{
    // return FieldT::multiplicative_generator.squared();
    FieldT::default()
}


// std::enable_if<std::is_same<FieldT, Double>::value, FieldT>::type
pub fn get_root_of_unity_is_same_double<FieldT:Default>(n:usize)->FieldT
{
    const  PI:f64 = 3.141592653589793238460264338328;
    // return FieldT((2.0 * PI / n).cos(), (2.0 * PI / n).sin());
    FieldT::default()
}

// 
// std::enable_if<!std::is_same<FieldT, Double>::value, FieldT>::type
pub fn get_root_of_unity_is_not_same_double<FieldT:Default>(n:usize)->eyre::Result<FieldT>
{
    let logn = log2(n);
    if n != (1<< logn){ eyre::bail!("get_root_of_unity: expected n == (1<< logn)");}
    // if logn > FieldT::s{ eyre::bail!("get_root_of_unity: expected logn <= FieldT::s");}

    let mut  omega = FieldT::default();//root_of_unity;
    // for _ in (logn+1..=FieldT::s).rev()
    // {
    //     omega *= omega;
    // }

     Ok(omega)
}

pub fn pack_int_vector_into_field_element_vector<FieldT:FTConfig+Default>(v:&Vec<usize>, w:usize)->Vec<FieldT>
where [(); FieldT::NUM_LIMBS]: 
{
    let  chunk_bits = 0usize;//FieldT::floor_size_in_bits();
    let repacked_size = div_ceil((v.len() * w) as i64, chunk_bits as i64).unwrap() as usize;
    let  mut result=Vec::with_capacity(repacked_size);

    for i in 0..repacked_size
    {
        let mut b=bigint::<{FieldT::NUM_LIMBS}>::new(0);
        for j in 0..chunk_bits 
        {
            let  word_index = (i * chunk_bits + j) / w ;
            let  pos_in_word = (i * chunk_bits + j) % w;
            let  word_or_0 = if word_index < v.len()  {v[word_index]} else {0};
            let  bit = (word_or_0 >> pos_in_word) & 1;

            b.0.0[j / GMP_NUMB_BITS] |= (bit << (j % GMP_NUMB_BITS)) as u64;
        }
        result[i] = FieldT::default();//(b);
    }

    return result;
}

pub fn  pack_bit_vector_into_field_element_vector<FieldT:FTConfig+Default>(v:&bit_vector,  chunk_bits:usize)->Vec<FieldT>
 where [(); {FieldT::NUM_LIMBS}]:
{
    // assert!(chunk_bits <= FieldT::floor_size_in_bits());

    let repacked_size = div_ceil(v.len() as i64, chunk_bits as i64).unwrap() as usize;
   let mut result=Vec::with_capacity(repacked_size);

    for i in 0..repacked_size
    {
       let mut b= bigint::<{FieldT::NUM_LIMBS}> ::new(0);
        for j in 0..chunk_bits
        {
            b.0.0[j / GMP_NUMB_BITS] |= ( if (i * chunk_bits + j) < v.len() && v[i * chunk_bits + j] { 1} else {0}) << (j % GMP_NUMB_BITS);
        }
        result[i] = FieldT::default();//(b);
    }

    return result;
}


pub fn  pack_bit_vector_into_field_element_vector1<FieldT>(v:&bit_vector)->Vec<FieldT>
{
    // return pack_bit_vector_into_field_element_vector::<FieldT>(v, FieldT::floor_size_in_bits());
    vec![]
}


pub fn  convert_bit_vector_to_field_element_vector<FieldT:One+Zero>(v:&bit_vector)->Vec<FieldT>
{
    let mut result=Vec::with_capacity(v.len());
    

    for  &b in  v
    {
        result.push( if b  {FieldT::one()} else {FieldT::zero()});
    }

    return result;
}


pub fn  convert_field_element_vector_to_bit_vector<FieldT>(v:&Vec<FieldT>)->bit_vector
{
     let mut result=bit_vector::new();

    for el in  v
    {
        let mut  el_bits = convert_field_element_to_bit_vector::<FieldT>(el);
        result.append(&mut el_bits);
    }

    return result;
}


pub fn  convert_field_element_to_bit_vector<FieldT>(el:&FieldT)->bit_vector
{
     let mut result=bit_vector::new();

    // let b = el.as_bigint();//bigint<FieldT::num_limbs>
    // for i in 0..0//FieldT::ceil_size_in_bits()
    // {
    //     result.push(b.test_bit(i));
    // }

    return result;
}


pub fn  convert_field_element_to_bit_vector1<FieldT>(el:FieldT,  bitcount:usize)->bit_vector
{
    let mut  result = convert_field_element_to_bit_vector(&el);
    result.resize(bitcount,false);

    return result;
}

pub fn  convert_bit_vector_to_field_element<FieldT:One+Zero+ Clone+std::ops::AddAssign>(v:&bit_vector)->FieldT
{
    // assert!(v.len() <= FieldT::ceil_size_in_bits());

    let mut  res = FieldT::zero();
    let mut  c = FieldT::one();
    for  &b in  v
    {
        res += if b {c.clone()} else {FieldT::zero()};
        c += c.clone();
    }
    return res;
}


pub fn  batch_invert<FieldT:One+Clone>(vec:&mut Vec<FieldT>)
{
    let mut  prod=Vec::with_capacity(vec.len());
   
    let mut  acc = FieldT::one();

    for el in  &*vec
    {
        // assert!(!el.is_zero());
        prod.push(acc.clone());
        // acc = acc * el;
    }

    let mut  acc_inverse = acc.clone();//.inverse();

    for  i in (0..vec.len()).rev()
    {
        let  old_el = vec[i].clone();
        vec[i] = acc_inverse.clone() * prod[i].clone();
        acc_inverse = acc_inverse * old_el;
    }
}

// } // namespace libff
//#endif // FIELD_UTILS_TCC_
