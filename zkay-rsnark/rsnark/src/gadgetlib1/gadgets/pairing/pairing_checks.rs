/** @file
 *****************************************************************************

 Declaration of interfaces for pairing-check gadgets.

 Given that e(.,.) denotes a pairing,
 - the gadget "check_e_equals_e_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)"; and
 - the gadget "check_e_equals_ee_gadget" checks the equation "e(P1,Q1)=e(P2,Q2)*e(P3,Q3)".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PAIRING_CHECKS_HPP_
// #define PAIRING_CHECKS_HPP_



use crate::gadgetlib1::gadgets::pairing::pairing_params;
use crate::gadgetlib1::gadgets::pairing::weierstrass_final_exponentiation;
use crate::gadgetlib1::gadgets::pairing::weierstrass_miller_loop;



 type FieldT=ffec::Fr<ppT>;
pub struct check_e_equals_e_gadget<ppT> {//gadget<ffec::Fr<ppT> >


   

ratio:    RcCell<Fqk_variable<ppT> >,
compute_ratio:    RcCell<e_over_e_miller_loop_gadget<ppT> >,
check_finexp:    RcCell<final_exp_gadget<ppT> >,

lhs_G1:    G1_precomputation<ppT>,
lhs_G2:    G2_precomputation<ppT>,
rhs_G1:    G1_precomputation<ppT>,
rhs_G2:    G2_precomputation<ppT>,

result:    pb_variable<FieldT>,

 
}


pub struct check_e_equals_ee_gadget<ppT> {//gadget<ffec::Fr<ppT> >


    // type FieldT=ffec::Fr<ppT>;

ratio:    RcCell<Fqk_variable<ppT> >,
compute_ratio:    RcCell<e_times_e_over_e_miller_loop_gadget<ppT> >,
check_finexp:    RcCell<final_exp_gadget<ppT> >,

lhs_G1:    G1_precomputation<ppT>,
lhs_G2:    G2_precomputation<ppT>,
rhs1_G1:    G1_precomputation<ppT>,
rhs1_G2:    G2_precomputation<ppT>,
rhs2_G1:    G1_precomputation<ppT>,
rhs2_G2:    G2_precomputation<ppT>,

result:    pb_variable<FieldT>,

    
}



// use crate::gadgetlib1::gadgets::pairing::pairing_checks;

//#endif // PAIRING_CHECKS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for pairing-check gadgets.

 See pairing_checks.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PAIRING_CHECKS_TCC_
// #define PAIRING_CHECKS_TCC_


impl check_e_equals_e_gadget<ppT> {

pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                      lhs_G1:&G1_precomputation<ppT>,
                                                      lhs_G2:&G2_precomputation<ppT>,
                                                      rhs_G1:&G1_precomputation<ppT>,
                                                      rhs_G2:&G2_precomputation<ppT>,
                                                      result:&pb_variable<FieldT>,
                                                      annotation_prefix:&String)->Self
  
{
    ratio=RcCell::new(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " ratio")));
    compute_ratio=RcCell::new(e_over_e_miller_loop_gadget::<ppT>::new(pb, lhs_G1, lhs_G2, rhs_G1, rhs_G2, *ratio, FMT(annotation_prefix, " compute_ratio")));
    check_finexp=RcCell::new(final_exp_gadget::<ppT>::new(pb, *ratio, result, FMT(annotation_prefix, " check_finexp")));
    //   gadget<FieldT>(&pb, annotation_prefix),
   Self{lhs_G1,
   lhs_G2,
   rhs_G1,
   rhs_G2,
    result}
}


pub fn generate_r1cs_constraints()
{
    compute_ratio.generate_r1cs_constraints();
    check_finexp.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_ratio.generate_r1cs_witness();
    check_finexp.generate_r1cs_witness();
}

}

impl check_e_equals_ee_gadget<ppT> {
pub fn new(pb:RcCell<protoboard<FieldT>>,
                                                        lhs_G1:&G1_precomputation<ppT>,
                                                        lhs_G2:&G2_precomputation<ppT>,
                                                        rhs1_G1:&G1_precomputation<ppT>,
                                                        rhs1_G2:&G2_precomputation<ppT>,
                                                        rhs2_G1:&G1_precomputation<ppT>,
                                                        rhs2_G2:&G2_precomputation<ppT>,
                                                        result:&pb_variable<FieldT>,
                                                        annotation_prefix:&String)->Self
    
{
    ratio=RcCell::new(Fqk_variable::<ppT>::new(pb, FMT(annotation_prefix, " ratio")));
    compute_ratio=RcCell::new(e_times_e_over_e_miller_loop_gadget::<ppT>::new(pb, rhs1_G1, rhs1_G2, rhs2_G1, rhs2_G2, lhs_G1, lhs_G2, *ratio, FMT(annotation_prefix, " compute_ratio")));
    check_finexp=RcCell::new(final_exp_gadget::<ppT>::new(pb, *ratio, result, FMT(annotation_prefix, " check_finexp")));
    // gadget<FieldT>(&pb, annotation_prefix),
   Self{lhs_G1,
   lhs_G2,
   rhs1_G1,
   rhs1_G2,
   rhs2_G1,
   rhs2_G2,
    result}
}


pub fn generate_r1cs_constraints()
{
    compute_ratio.generate_r1cs_constraints();
    check_finexp.generate_r1cs_constraints();
}


pub fn generate_r1cs_witness()
{
    compute_ratio.generate_r1cs_witness();
    check_finexp.generate_r1cs_witness();
}

}

//#endif // PAIRING_CHECKS_TCC_
