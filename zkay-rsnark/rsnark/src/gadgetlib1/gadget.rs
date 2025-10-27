/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef GADGET_HPP_
// #define GADGET_HPP_

use crate::gadgetlib1::protoboard;



// 
pub struct gadget<FieldT> {
// 
     pb:protoboard<FieldT>,
    annotation_prefix:String,
// 
//     gadget(pb:protoboard<FieldT>, annotation_prefix:&String="");
}


// use crate::gadgetlib1::gadget;

//#endif // GADGET_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef GADGET_TCC_
// #define GADGET_TCC_

// 
impl<FieldT> gadget<FieldT>{
pub fn new(pb:protoboard<FieldT> , annotation_prefix:String) ->Self
{
// #ifdef DEBUG
    // assert!(annotation_prefix != "");
//#endif
    Self {pb, annotation_prefix}
}
}

//#endif // GADGET_TCC_
