/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef HASH_IO_HPP_
// #define HASH_IO_HPP_
// use  <cstddef>


use crate::gadgetlib1::gadgets::basic_gadgets;




pub struct digest_variable<FieldT> {//gadget<FieldT>

digest_size:    usize,
bits:    pb_variable_array<FieldT>,

}


pub struct block_variable<FieldT> {//gadget<FieldT>

block_size:    usize,
bits:    pb_variable_array<FieldT>,

}



// use crate::gadgetlib1::gadgets::hashes::hash_io;

//#endif // HASH_IO_HPP_
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef HASH_IO_TCC_
// #define HASH_IO_TCC_



impl digest_variable<FieldT>{
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                         digest_size:usize,
                                         annotation_prefix:&String)->Self
    
{
    bits.allocate(&pb, digest_size, FMT(self.annotation_prefix, " bits"));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{digest_size}
}



pub fn new2(pb:RcCell<protoboard<FieldT>>,
                                         digest_size:usize,
                                         partial_bits:&pb_variable_array<FieldT>,
                                         padding:&pb_variable<FieldT>,
                                         annotation_prefix:&String)->Self
   
{
    assert!(bits.len() <= digest_size);
    bits = partial_bits;
    while (bits.len() != digest_size)
    {
        bits.push(padding);
    }
    //   gadget<FieldT>(&pb, annotation_prefix),
    Self{digest_size}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..digest_size
    {
        generate_boolean_r1cs_constraint::<FieldT>(self.pb, bits[i], FMT(self.annotation_prefix, " bits_{}", i));
    }
}


pub fn generate_r1cs_witness(contents:&bit_vector)
{
    bits.fill_with_bits(self.pb, contents);
}


pub fn get_digest()->bit_vector
{
    return bits.get_bits(self.pb);
}
}

impl block_variable<FieldT> {
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                       block_size:usize,
                                       annotation_prefix:&String)->Self
    
{
    bits.allocate(&pb, block_size, FMT(self.annotation_prefix, " bits"));
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{block_size}
    
}


pub fn new2(pb:RcCell<protoboard<FieldT>>,
                                       parts:&Vec<pb_variable_array<FieldT> >,
                                       annotation_prefix:&String)->Self
    
{
    for part in &parts
    {
        bits.insert(bits.end(), part.begin(), part.end());
    }
    // gadget<FieldT>(&pb, annotation_prefix)
    Self{}
}


pub fn new3(pb:RcCell<protoboard<FieldT>>,
                                       left:&digest_variable<FieldT>,
                                       right:&digest_variable<FieldT>,
                                       annotation_prefix:&String)->Self
    
{
    assert!(left.bits.len() == right.bits.len());
    block_size = 2 * left.bits.len();
    bits.insert(bits.end(), left.bits.begin(), left.bits.end());
    bits.insert(bits.end(), right.bits.begin(), right.bits.end());
// gadget<FieldT>(&pb, annotation_prefix)
    Self{}
}


pub fn generate_r1cs_witness(contents:&bit_vector)
{
    bits.fill_with_bits(self.pb, contents);
}


pub fn get_block()->bit_vector
{
    return bits.get_bits(self.pb);
}

}

//#endif // HASH_IO_TCC_
