/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef DIGEST_SELECTOR_GADGET_HPP_
// #define DIGEST_SELECTOR_GADGET_HPP_

// 

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::hashes::hash_io;




pub struct digest_selector_gadget {//gadget<FieldT>

digest_size:    usize,
input:    digest_variable<FieldT>,
is_right:    pb_linear_combination<FieldT>,
left:    digest_variable<FieldT>,
right:    digest_variable<FieldT>,

}



// use crate::gadgetlib1::gadgets::hashes::digest_selector_gadget;

//#endif // DIGEST_SELECTOR_GADGET_HPP_
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef DIGEST_SELECTOR_GADGET_TCC_
// #define DIGEST_SELECTOR_GADGET_TCC_


impl digest_selector_gadget<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                       digest_size:usize,
                                                       input:&digest_variable<FieldT>,
                                                       is_right:&pb_linear_combination<FieldT>,
                                                       left:&digest_variable<FieldT>,
                                                       right:&digest_variable<FieldT>,
                                                       annotation_prefix:&String)->Self

{
    // gadget<FieldT>(&pb, annotation_prefix),
    Self{digest_size,input,is_right,left,right}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..digest_size
    {
        /*
          input = is_right * right + (1-is_right) * left
          input - left = is_right(right - left)
        */
        self.pb.borrow_mut().add_r1cs_constraint(r1cs_constraint::<FieldT>(is_right, right.bits[i] - left.bits[i], input.bits[i] - left.bits[i]),
                                   FMT(self.annotation_prefix, " propagate_{}", i));
    }
}


pub fn generate_r1cs_witness()
{
    is_right.evaluate(self.pb);

    assert!(self.pb.lc_val(is_right) == FieldT::one() || self.pb.lc_val(is_right) == FieldT::zero());
    if self.pb.lc_val(is_right) == FieldT::one()
    {
        for i in 0..digest_size
        {
            self.pb.borrow().val(&right.bits[i]) = self.pb.borrow().val(&input.bits[i]);
        }
    }
    else
    {
        for i in 0..digest_size
        {
            self.pb.borrow().val(&left.bits[i]) = self.pb.borrow().val(&input.bits[i]);
        }
    }
}

}

//#endif // DIGEST_SELECTOR_GADGET_TCC_
