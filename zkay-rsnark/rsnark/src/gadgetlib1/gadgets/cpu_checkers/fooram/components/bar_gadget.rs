/** @file
 *****************************************************************************

 Declaration of interfaces for an auxiliarry gadget for the FOORAM CPU.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BAR_GADGET_HPP_
// #define BAR_GADGET_HPP_

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;



/**
 * The bar gadget checks linear combination
 *                   Z = aX + bY (mod 2^w)
 * for a, b - const, X, Y - vectors of w bits,
 * where w is implicitly inferred, Z - a packed variable.
 *
 * This gadget is used four times in fooram:
 * - PC' = PC + 1
 * - load_addr = 2 * x + PC'
 * - store_addr = x + PC
 */
// 
pub struct bar_gadget {
// : public gadget<FieldT> 
X:    pb_linear_combination_array<FieldT>,
a:    FieldT,
Y:    pb_linear_combination_array<FieldT>,
b:    FieldT,
Z_packed:    pb_linear_combination<FieldT>,
Z_bits:    pb_variable_array<FieldT>,

result:    pb_variable<FieldT>,
overflow:    pb_variable_array<FieldT>,
unpacked_result:    pb_variable_array<FieldT>,

unpack_result:    RcCell<packing_gadget<FieldT> >,
pack_Z:    RcCell<packing_gadget<FieldT> >,

width:    usize,

    // bar_gadget(pb:protoboard<FieldT>,
    //            X:&pb_linear_combination_array<FieldT>,
    //            a:&FieldT,
    //            Y:&pb_linear_combination_array<FieldT>,
    //            b:&FieldT,
    //            Z_packed:&pb_linear_combination<FieldT>,
    //            annotation_prefix:&String);
    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness();
}



// use crate::gadgetlib1::gadgets::cpu_checkers/fooram/components/bar_gadget;

//#endif // BAR_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for an auxiliary gadget for the FOORAM CPU.

 See bar_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BAR_GADGET_TCC_
// #define BAR_GADGET_TCC_

impl bar_gadget<FieldT>{

// 
pub fn new(
pb:&protoboard<FieldT>,
X:&                                pb_linear_combination_array<FieldT>,
a:&                                FieldT,
Y:&                                pb_linear_combination_array<FieldT>,
b:&                                FieldT,
Z_packed:&                                pb_linear_combination<FieldT>,
annotation_prefix:&                                String
        ) ->Self
    
{
    assert!(X.len() == Y.len());
    let width = X.len();

    result.allocate(pb, FMT(annotation_prefix, " result"));
    Z_bits.allocate(pb, width, FMT(annotation_prefix, " Z_bits"));
    overflow.allocate(pb, 2*width, FMT(annotation_prefix, " overflow"));

    unpacked_result.insert(unpacked_result.end(), Z_bits.begin(), Z_bits.end());
    unpacked_result.insert(unpacked_result.end(), overflow.begin(), overflow.end());

    unpack_result.reset( packing_gadget::<FieldT>(pb, unpacked_result, result, FMT(annotation_prefix, " unpack_result")));
    pack_Z.reset( packing_gadget::<FieldT>::new(pb, Z_bits, Z_packed, FMT(annotation_prefix, " pack_Z")));
    // gadget<FieldT>(pb, annotation_prefix),
    Self{X,
    a,
    Y,
    b,
    Z_packed}
}


pub fn generate_r1cs_constraints()
{
    unpack_result.generate_r1cs_constraints(true);
    pack_Z.generate_r1cs_constraints(false);

    self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>::new(1, a * pb_packing_sum::<FieldT>(X) + b * pb_packing_sum::<FieldT>(Y), result), FMT(self.annotation_prefix, " compute_result"));
}


pub fn generate_r1cs_witness()
{
    self.pb.val(result) = X.get_field_element_from_bits(self.pb) * a + Y.get_field_element_from_bits(self.pb) * b;
    unpack_result.generate_r1cs_witness_from_packed();

    pack_Z.generate_r1cs_witness_from_bits();
}

}

//#endif // BAR_GADGET_TCC_
