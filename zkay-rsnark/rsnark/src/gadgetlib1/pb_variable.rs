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
// use  <vector>

use ffec::common::utils;

use crate::relations::variable;



type lc_index_t=usize ;

// 
// class protoboard;

// 
pub struct pb_variable<FieldT>   {
// public:public variable<FieldT> 
    // pb_variable(const var_index_t index = 0) : variable<FieldT>(index) {};

    // void allocate(pb:&protoboard<FieldT>, annotation:&std::string="");
}
pub trait ContentsConfig{
     type contents;
}
impl ContentsConfig for pb_variable_array <FieldT>
{ type contents=std::vector<pb_variable<FieldT> > ;}

// : private std::vector<pb_variable<FieldT> >
pub struct pb_variable_array <FieldT>
{
   
// public:
//     using typename contents::iterator;
//     using typename contents::const_iterator;
//     using typename contents::reverse_iterator;
//     using typename contents::const_reverse_iterator;

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

    // pb_variable_array() : contents() {};
    // pb_variable_array(size_t count, value:&pb_variable<FieldT>) : contents(count, value) {};
    // pb_variable_array(typename contents::const_iterator first, typename contents::const_iterator last) : contents(first, last) {};
    // pb_variable_array(typename contents::const_reverse_iterator first, typename contents::const_reverse_iterator last) : contents(first, last) {};
    // void allocate(pb:&protoboard<FieldT>, n:usize, annotation_prefix:&std::string="");

    // void fill_with_field_elements(pb:&protoboard<FieldT>, vals:&std::vector<FieldT>) ;
    // void fill_with_bits(pb:&protoboard<FieldT>, bits:&ffec::bit_vector) ;
    // void fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) ;
    // void fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) ;

    // std::vector<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // ffec::bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

/* index 0 corresponds to the constant term (used in legacy code) */
// #define ONE pb_variable<FieldT>(0)

// 
pub struct pb_linear_combination {
// public:: public linear_combination<FieldT> 
    is_variable:bool,
    index:lc_index_t,

    // pb_linear_combination();
    // pb_linear_combination(var:&pb_variable<FieldT>);

    // void assign(pb:&protoboard<FieldT>, lc:&linear_combination<FieldT>);
    // void evaluate(pb:&protoboard<FieldT>) ;

    // bool is_constant() ;
    // FieldT constant_term() ;
}
impl ContentsConfig for pb_linear_combination_array <FieldT>
{
type contents=std::vector<pb_linear_combination<FieldT> > ;
}
// 
pub struct pb_linear_combination_array <FieldT>
{
//: private std::vector<pb_linear_combination<FieldT> >
    // 
// public:
//     using typename contents::iterator;
//     using typename contents::const_iterator;
//     using typename contents::reverse_iterator;
//     using typename contents::const_reverse_iterator;

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

    // pb_linear_combination_array() : contents() {};
    // pb_linear_combination_array(arr:&arr:&pb_variable_array<FieldT>) { for v in self.push(pb_linear_combination<FieldT>(v)); };
    // pb_linear_combination_array(size_t count) : contents(count) {};
    // pb_linear_combination_array(size_t count, value:&pb_linear_combination<FieldT>) : contents(count, value) {};
    // pb_linear_combination_array(typename contents::const_iterator first, typename contents::const_iterator last) : contents(first, last) {};
    // pb_linear_combination_array(typename contents::const_reverse_iterator first, typename contents::const_reverse_iterator last) : contents(first, last) {};

    // void evaluate(pb:&protoboard<FieldT>) ;

    // void fill_with_field_elements(pb:&protoboard<FieldT>, vals:&std::vector<FieldT>) ;
    // void fill_with_bits(pb:&protoboard<FieldT>, bits:&ffec::bit_vector) ;
    // void fill_with_bits_of_ulong(pb:&protoboard<FieldT>,  i:u64) ;
    // void fill_with_bits_of_field_element(protoboard<FieldT> &pb, r:&FieldT) ;

    // std::vector<FieldT> get_vals(pb:&protoboard<FieldT>) ;
    // ffec::bit_vector get_bits(pb:&protoboard<FieldT>) ;

    // FieldT get_field_element_from_bits(pb:&protoboard<FieldT>) ;
}

// 
// linear_combination<FieldT> pb_sum(v:&pb_linear_combination_array<FieldT>);

// 
// linear_combination<FieldT> pb_packing_sum(v:&pb_linear_combination_array<FieldT>);

// 
// linear_combination<FieldT> pb_coeff_sum(v:&pb_linear_combination_array<FieldT>, coeffs:&std::vector<FieldT>);


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


pub fn allocate(pb:&protoboard<FieldT>, annotation:&std::string)
{
    self.index = pb.allocate_var_index(annotation);
}
}

impl pb_variable_array<FieldT>{
/* allocates pb_variable<FieldT> array in MSB->LSB order */

pub fn allocate(pb:&protoboard<FieldT>,  n:usize, annotation_prefix:&std::string)
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


pub fn fill_with_field_elements(pb:&protoboard<FieldT>, vals:&std::vector<FieldT>) 
{
    assert!(self.size() == vals.size());
    for i in 0..vals.size()
    {
        pb.val(self[i]) = vals[i];
    }
}


pub fn fill_with_bits(pb:&protoboard<FieldT>, bits:&ffec::bit_vector) 
{
    assert!(self.size() == bits.size());
    for i in 0..bits.size()
    {
        pb.val(self[i]) = if bits[i] {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) 
{
    let  rint = r.as_bigint::<FieldT::num_limbs>();
    for i in 0..self.size()
    {
        pb.val(self[i])=  if rint.test_bit(i) {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) 
{
    self.fill_with_bits_of_field_element(pb, FieldT::from(i, true));
}


pub fn get_vals(pb:&protoboard<FieldT>)->std::vector<FieldT> 
{
    let mut  result=Vec::with_capacity(self.size());
    for i in 0..self.size()
    {
        result[i] = pb.val(self[i]);
    }
    return result;
}


 pub fn get_bits(pb:&protoboard<FieldT>) ->ffec::bit_vector
{
    let mut  result=bit_vector::new();
    for i in 0..self.size()
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

    for i in 0..self.size()
    {
        /* push in the new bit */
        let  v = pb.val(self[self.size()-1-i]);
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
    for i in 0..self.size()
    {
        self[i].evaluate(pb);
    }
}


pub fn  fill_with_field_elements(pb:&protoboard<FieldT>, vals:&std::vector<FieldT>) 
{
    assert!(self.size() == vals.size());
    for i in 0..vals.size()
    {
        pb.lc_val(self[i]) = vals[i];
    }
}


pub fn  fill_with_bits(pb:&protoboard<FieldT>, bits:&ffec::bit_vector) 
{
    assert!(self.size() == bits.size());
    for i in 0..bits.size()
    {
        pb.lc_val(self[i]) = if bits[i] {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn  fill_with_bits_of_field_element(pb:&protoboard<FieldT>, r:&FieldT) 
{
    let rint = r.as_bigint::<FieldT::num_limbs>();
    for i in 0..self.size()
    {
        pb.lc_val(self[i])=  if rint.test_bit(i) {FieldT::one()} else{FieldT::zero()};
    }
}


pub fn  fill_with_bits_of_ulong(pb:&protoboard<FieldT>, i:u64) 
{
    self.fill_with_bits_of_field_element(pb, FieldT(i));
}


 pub fn get_vals(pb:&protoboard<FieldT>) ->std::vector<FieldT>
{
    let mut result=Vec::with_capacity(self.size());
    for i in 0..self.size()
    {
        result[i] = pb.lc_val(self[i]);
    }
    return result;
}


 pub fn get_bits(pb:&protoboard<FieldT>) ->ffec::bit_vector
{
    let mut  result=bit_vector::new();
    for i in 0..self.size()
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

    for i in 0..self.size()
    {
        /* push in the new bit */
        let v = pb.lc_val(self[self.size()-1-i]);
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
    let mut all_terms=vec![];//std::vector<linear_term<FieldT> > 
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


 pub fn pb_coeff_sum<FieldT>(v:&pb_linear_combination_array<FieldT>, coeffs:&std::vector<FieldT>)->linear_combination<FieldT>
{
    assert!(v.size() == coeffs.size());
    let mut  all_terms=vec![];//std::vector<linear_term<FieldT> >

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
