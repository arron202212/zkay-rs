/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef GADGET_HPP_
// #define GADGET_HPP_

use crate::gadgetlib1::protoboard;



// template<typename FieldT>
pub struct gadget<FieldT> {
// protected:
     pb:protoboard<FieldT>,
    annotation_prefix:String,
// public:
//     gadget(protoboard<FieldT> &pb, const std::string &annotation_prefix="");
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

// template<typename FieldT>
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
