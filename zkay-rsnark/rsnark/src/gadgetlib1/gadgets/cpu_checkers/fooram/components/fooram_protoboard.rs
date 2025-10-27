/** @file
 *****************************************************************************

 Declaration of interfaces for a protoboard for the FOORAM CPU.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_PROTOBOARD_HPP_
// #define FOORAM_PROTOBOARD_HPP_

use crate::gadgetlib1::gadget;
use crate::relations::ram_computations::rams::fooram::fooram_aux;



// 
pub struct fooram_protoboard<FieldT>  {
// : public protoboard<FieldT>
      ap:fooram_architecture_params,

    // fooram_protoboard(ap:&fooram_architecture_params);
}

// 
pub struct fooram_gadget  {
// : public gadget<FieldT>
     pb:fooram_protoboard<FieldT>,
// 
//     fooram_gadget(fooram_protoboard<FieldT> &pb, annotation_prefix:&String="");
}



// use crate::gadgetlib1::gadgets::cpu_checkers/fooram/components/fooram_protoboard;

//#endif // FOORAM_PROTOBOARD_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a protoboard for the FOORAM CPU.

 See fooram_protoboard.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef FOORAM_PROTOBOARD_TCC_
// #define FOORAM_PROTOBOARD_TCC_

impl fooram_protoboard<FieldT>{

// 
pub fn new(ap:fooram_architecture_params) ->Self
   
{
    // protoboard<FieldT>(), 
    Self{ap}
}
}

impl fooram_gadget<FieldT>{
// 
pub fn new( pb:fooram_protoboard<FieldT>, annotation_prefix: String ) ->Self
{
// gadget<FieldT>(pb, annotation_prefix)
Self{pb}
}

}

//#endif // FOORAM_PROTOBOARD_HPP_
