/** @file
 *****************************************************************************

 Declaration of interfaces for (single and double) word gadgets.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef WORD_VARIABLE_GADGET_HPP_
// #define WORD_VARIABLE_GADGET_HPP_

use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;



/**
 * Holds both binary and field representaton of a word.
 */
// template<typename FieldT>
pub struct word_variable_gadget;
impl word_variable_gadget {
// : public dual_variable_gadget<FieldT> 
    pub fn new(pb:tinyram_protoboard<FieldT>, annotation_prefix:std::string) ->Self
        {
//  dual_variable_gadget<FieldT>(pb, pb.ap.w, annotation_prefix)
    Self
}
    pub fn new1(pb:tinyram_protoboard<FieldT>, bits:pb_variable_array<FieldT>, annotation_prefix:std::string) ->Self
        {
// dual_variable_gadget<FieldT>(pb, bits, annotation_prefix) 
    Self

}
    pub fn new2(pb:tinyram_protoboard<FieldT>, packed:pb_variable<FieldT>, annotation_prefix:std::string) ->Self
        {
//  dual_variable_gadget<FieldT>(pb, packed, pb.ap.w, annotation_prefix)
        Self
}
}

/**
 * Holds both binary and field representaton of a double word.
 */
// template<typename FieldT>
pub struct doubleword_variable_gadget;
impl doubleword_variable_gadget  {
// : public dual_variable_gadget<FieldT>
    pub fn new(pb:tinyram_protoboard<FieldT>, annotation_prefix:std::string) ->Self
        {
// dual_variable_gadget<FieldT>(pb, 2*pb.ap.w, annotation_prefix) 
}
    pub fn new1(pb:tinyram_protoboard<FieldT>, bits:pb_variable_array<FieldT>, annotation_prefix:std::string) ->Self
       {
//  dual_variable_gadget<FieldT>(pb, bits, annotation_prefix) 
}
    pub fn new2(pb:tinyram_protoboard<FieldT>, packed:pb_variable<FieldT>, annotation_prefix:std::string) ->Self
         {
// dual_variable_gadget<FieldT>(pb, packed, 2*pb.ap.w, annotation_prefix)
}
}



//#endif // WORD_VARIABLE_GADGET_HPP_
