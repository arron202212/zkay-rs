/** @file
 *****************************************************************************

 Declaration of interfaces for a protoboard for TinyRAM.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_PROTOBOARD_HPP_
// #define TINYRAM_PROTOBOARD_HPP_

use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::protoboard;
use crate::relations::ram_computations::rams::ram_params;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux;



// 
pub struct tinyram_protoboard  {
// : public RcCell<protoboard<FieldT>>
      ap:tinyram_architecture_params,

    // tinyram_protoboard(ap:tinyram_architecture_params);
}

// 
pub struct tinyram_gadget {
// : public gadget<FieldT> 
    pb:tinyram_protoboard<FieldT> ,
// 
//     tinyram_gadget(pb:tinyram_protoboard<FieldT>, annotation_prefix:String="");
}

// standard gadgets provide two methods: generate_r1cs_constraints and generate_r1cs_witness
// 
pub struct tinyram_standard_gadget {
// : public tinyram_gadget<FieldT> 
//     tinyram_standard_gadget(pb:tinyram_protoboard<FieldT>, annotation_prefix:String="");

//     virtual pub fn  generate_r1cs_constraints() = 0;
//     virtual pub fn  generate_r1cs_witness() = 0;
}



// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard;

//#endif // TINYRAM_PROTOBOARD_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a protoboard for TinyRAM.

 See tinyram_protoboard.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TINYRAM_PROTOBOARD_TCC_
// #define TINYRAM_PROTOBOARD_TCC_


impl tinyram_protoboard<FieldT>{

pub fn new(ap:tinyram_architecture_params) ->Self
    
{
   Self{ap}
}
}
impl tinyram_gadget<FieldT>{

pub fn new(pb:tinyram_protoboard<FieldT>, annotation_prefix:String) ->Self
    
{
// gadget<FieldT>(&pb, annotation_prefix),pb
  Self{}
}
}
impl tinyram_standard_gadget<FieldT>{

pub fn new(pb:tinyram_protoboard<FieldT>, annotation_prefix:String) ->Self
    
{
// tinyram_gadget<FieldT>(&pb, annotation_prefix)
    Self{}
}
}


//#endif // TINYRAM_PROTOBOARD_TCC_
