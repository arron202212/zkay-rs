/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PB_VARIABLE_HPP_
// #define PB_VARIABLE_HPP_

// use  <cstddef>
// use  <string>
// 

use ffec::common::utils;

use crate::relations::variable;



type lc_index_t=usize ;

// 
// pub struct protoboard;

// 
pub struct pb_variable<FieldT>   {
// public variable<FieldT> 
    // pb_variable(let index= 0)->Self variable<FieldT>(index) {};

    // pub fn  allocate(pb:&protoboard<FieldT>, annotation:&String="");
}
pub trait ContentsConfig{
     type contents;
}
impl ContentsConfig for pb_variable_array <FieldT>
{ type contents=Vec<pb_variable<FieldT> > ;}

// : private Vec<pb_variable<FieldT> >
pub struct pb_variable_array <FieldT>
{
   
// 
//     using contents::iterator;
//     using contents::const_iterator;
//     using contents::reverse_iterator;
//     using contents::const_reverse_iterator;

//     using contents::begin;
//     using contents::end;
//     using contents::rbegin;
//     using contents::rend;
//     using contents::push;
//     using contents::insert;
//     using contents::reserve;
//     using contents::size;
//     using contents::empty;
//     using contents::operator[];
//     using contents::resize;

    // pb_variable_array()->Self contents() {};
    // pb_variable_array(usize count, value:&pb_variable<FieldT>)->Self contents(count, value) {};
    // pb_variable_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_variable_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};
    // pub fn  allocate(pb:&protoboard<FieldT>, n:usize, annotation_prefix:&String="");

    // pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

/* index 0 corresponds to the constant term (used in legacy code) */
// #define ONE pb_variable<FieldT>(0)

// 
pub struct pb_linear_combination {
// : public linear_combination<FieldT> 
    is_variable:bool,
    index:lc_index_t,

    // pb_linear_combination();
    // pb_linear_combination(var:&pb_variable<FieldT>);

    // pub fn  assign(pb:&protoboard<FieldT>, lc:&linear_combination<FieldT>);
    // pub fn  evaluate(pb:&protoboard<FieldT>) ;

    // bool is_constant() ;
    // FieldT constant_term() ;
}
impl ContentsConfig for pb_linear_combination_array <FieldT>
{
type contents=Vec<pb_linear_combination<FieldT> > ;
}
// 
pub struct pb_linear_combination_array <FieldT>
{
//: private Vec<pb_linear_combination<FieldT> >
    // 
// 
//     using contents::iterator;
//     using contents::const_iterator;
//     using contents::reverse_iterator;
//     using contents::const_reverse_iterator;

//     using contents::begin;
//     using contents::end;
//     using contents::rbegin;
//     using contents::rend;
//     using contents::push;
//     using contents::insert;
//     using contents::reserve;
//     using contents::size;
//     using contents::empty;
//     using contents::operator[];
//     using contents::resize;

    // pb_linear_combination_array()->Self contents() {};
    // pb_linear_combination_array(arr:&arr:&pb_variable_array<FieldT>) { for v in self.push(pb_linear_combination<FieldT>(v)); };
    // pb_linear_combination_array(usize count)->Self contents(count) {};
    // pb_linear_combination_array(usize count, value:&pb_linear_combination<FieldT>)->Self contents(count, value) {};
    // pb_linear_combination_array(contents::const_iterator first, contents::const_iterator last)->Self contents(first, last) {};
    // pb_linear_combination_array(contents::const_reverse_iterator first, contents::const_reverse_iterator last)->Self contents(first, last) {};

    // pub fn  evaluate(pb:&protoboard<FieldT>) ;

    // pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) ;
    // pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) ;
    // pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>,  i:u64) ;
    // pub fn  fill_with_bits_of_field_element(pb:protoboard<FieldT>, r:&FieldT) ;

    // Vec<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

// 
// linear_combination<FieldT> pb_sum(v:&pb_linear_combination_array<FieldT>);

// 
// linear_combination<FieldT> pb_packing_sum(v:&pb_linear_combination_array<FieldT>);

// 
// linear_combination<FieldT> pb_coeff_sum(v:&pb_linear_combination_array<FieldT>, coeffs:&Vec<FieldT>);


// use crate::gadgetlib1::pb_variable;

//#endif // PB_VARIABLE_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PB_VARIABLE_TCC_
// #define PB_VARIABLE_TCC_
// use  <cassert>

// use ffec::common::utils;

use crate::gadgetlib1::protoboard;

impl pb_variable<FieldT>{


pub fn allocate(pb:&protoboard<FieldT>, annotation:&String)
{
    self.index = pb.allocate_var_index(annotation);
}
}

impl pb_variable_array<FieldT>{
/* allocates pb_variable<FieldT> array in MSB->LSB order */

pub fn allocate(pb:&protoboard<FieldT>,  n:usize, annotation_prefix:&String)
{
// #ifdef DEBUG
    assert!(annotation_prefix != "");
//#endif
    self.resize(n);

    for i in 0..n
    {
        self[i].allocate(pb, format!("{annotation_prefix}_{}", i));
    }
}


pub fn fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) 
{
    assert!(self.len() == vals.len());
    for i in 0..vals.len()
    {
        pb.val(self[i]) = vals[i];
    }
}


pub fn fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) 
{
    assert!(self.len() == bits.len());
    for i in 0..bits.len()
    {
        pb.val(self[i]) = if bits[i] {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) 
{
    let  rint = r.as_bigint::<FieldT::num_limbs>();
    for i in 0..self.len()
    {
        pb.val(self[i])=  if rint.test_bit(i) {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) 
{
    self.fill_with_bits_of_field_element(pb, FieldT::from(i, true));
}


pub fn get_vals(pb:&protoboard<FieldT>)->Vec<FieldT> 
{
    let mut  result=Vec::with_capacity(self.len());
    for i in 0..self.len()
    {
        result[i] = pb.val(self[i]);
    }
    return result;
}


 pub fn get_bits(pb:&protoboard<FieldT>) ->bit_vector
{
    let mut  result=bit_vector::new();
    for i in 0..self.len()
    {
        let  v = pb.val(self[i]);
        assert!(v == FieldT::zero() || v == FieldT::one());
        result.push_back(v == FieldT::one());
    }
    return result;
}


 pub fn get_field_element_from_bits(pb:&protoboard<FieldT>) ->FieldT
{
    let  result = FieldT::zero();

    for i in 0..self.len()
    {
        /* push in the new bit */
        let  v = pb.val(self[self.len()-1-i]);
        assert!(v == FieldT::zero() || v == FieldT::one());
        result += result + v;
    }

    return result;
}
}

impl pb_linear_combination<FieldT>{

pub fn new()
{
    self.is_variable = false;
}


pub fn new2(var:&pb_variable<FieldT>)
{
    self.is_variable = true;
    self.index = var.index;
    self.terms.push(linear_term::<FieldT>(var));
}


pub fn assign(pb:&protoboard<FieldT>, lc:&linear_combination<FieldT>)
{
    assert!(self.is_variable == false);
    self.index = pb.allocate_lc_index();
    self.terms = lc.terms;
}


pub fn evaluate(pb:&protoboard<FieldT>) 
{
    if self.is_variable
    {
        return; // do nothing
    }

    let mut  sum = 0;
    for term in &self.terms
    {
        sum += term.coeff * pb.val(pb_variable::<FieldT>(term.index));
    }

    pb.lc_valself = sum;
}


 pub fn is_constant() ->bool
{
    if is_variable
    {
        return (index == 0);
    }
    else
    {
        for term in &self.terms
        {
            if term.index != 0
            {
                return false;
            }
        }

        return true;
    }
}


 pub fn constant_term() ->FieldT
{
    if is_variable
    {
        return if index == 0 {FieldT::one()} else{FieldT::zero()};
    }
    else
    {
        let mut  result = FieldT::zero();
        for term in &self.terms
        {
            if term.index == 0
            {
                result += term.coeff;
            }
        }
        return result;
    }
}


pub fn  evaluate(pb:&protoboard<FieldT>) 
{
    for i in 0..self.len()
    {
        self[i].evaluate(pb);
    }
}


pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&Vec<FieldT>) 
{
    assert!(self.len() == vals.len());
    for i in 0..vals.len()
    {
        pb.lc_val(self[i]) = vals[i];
    }
}


pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&bit_vector) 
{
    assert!(self.len() == bits.len());
    for i in 0..bits.len()
    {
        pb.lc_val(self[i]) = if bits[i] {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn  fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) 
{
    let rint = r.as_bigint::<FieldT::num_limbs>();
    for i in 0..self.len()
    {
        pb.lc_val(self[i])=  if rint.test_bit(i) {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) 
{
    self.fill_with_bits_of_field_element(pb, FieldT(i));
}


 pub fn get_vals(pb:&protoboard<FieldT>) ->Vec<FieldT>
{
    let mut result=Vec::with_capacity(self.len());
    for i in 0..self.len()
    {
        result[i] = pb.lc_val(self[i]);
    }
    return result;
}


 pub fn get_bits(pb:&protoboard<FieldT>) ->bit_vector
{
    let mut  result=bit_vector::new();
    for i in 0..self.len()
    {
        let  v = pb.lc_val(self[i]);
        assert!(v == FieldT::zero() || v == FieldT::one());
        result.push_back(v == FieldT::one());
    }
    return result;
}


 pub fn get_field_element_from_bits(pb:&protoboard<FieldT>) ->FieldT
{
    let mut  result = FieldT::zero();

    for i in 0..self.len()
    {
        /* push in the new bit */
        let v = pb.lc_val(self[self.len()-1-i]);
        assert!(v == FieldT::zero() || v == FieldT::one());
        result += result + v;
    }

    return result;
}
}


 pub fn pb_sum<FieldT>(v:&pb_linear_combination_array<FieldT>)->linear_combination<FieldT>
{
    let mut result=linear_combination::<FieldT> ::new();
    for term in &v
    {
        result = result + term;
    }

    return result;
}


pub fn pb_packing_sum<FieldT>(v:&pb_linear_combination_array<FieldT>)->linear_combination<FieldT> 
{
    let  twoi = FieldT::one(); // will hold 2^i entering each iteration
    let mut all_terms=vec![];//Vec<linear_term<FieldT> > 
    for lc in &v
    {
        for term in &lc.terms
        {
            all_terms.push(twoi * term);
        }
        twoi += twoi;
    }

    return linear_combination::<FieldT>::new(all_terms);
}


 pub fn pb_coeff_sum<FieldT>(v:&pb_linear_combination_array<FieldT>, coeffs:&Vec<FieldT>)->linear_combination<FieldT>
{
    assert!(v.len() == coeffs.len());
    let mut  all_terms=vec![];//Vec<linear_term<FieldT> >

    let mut  coeff_it = coeffs.iter();
    for lc in &v
    {
        for term in &lc.terms
        {
            all_terms.push((*coeff_it.next().unwrap()) * term);
        }
    }

    return linear_combination::<FieldT>::new(all_terms);
}



//#endif // PB_VARIABLE_TCC
