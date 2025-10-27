/** @file
 *****************************************************************************

 Declaration of interfaces for the exponentiation gadget.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EXPONENTIATION_GADGET_HPP_
// #define EXPONENTIATION_GADGET_HPP_

// 
// 

use ffec::algebra::field_utils::bigint::bigint;
 use ffec::algebra::scalar_multiplication::wnaf;

use crate::gadgetlib1::gadget;



/**
 * The exponentiation gadget verifies field exponentiation in the field F_{p^k}.
 *
 * Note that the power is a constant (i.e., hardcoded into the gadget).
 */
// 
pub struct  exponentiation_gadget {
// : gadget<FpkT::my_Fp> 
    // type FieldT=FpkT::my_Fp;
NAF:    Vec<long>,

intermediate:    Vec<RcCell<Fpk_variableT<FpkT> > >,
addition_steps:    Vec<RcCell<Fpk_mul_gadgetT<FpkT> > >,
subtraction_steps:    Vec<RcCell<Fpk_mul_gadgetT<FpkT> > >,
doubling_steps:    Vec<RcCell<Fpk_sqr_gadgetT<FpkT> > >,

elt:    Fpk_variableT<FpkT>,
power:    ffec::bigint<m>,
result:    Fpk_variableT<FpkT>,

intermed_count:    usize,
add_count:    usize,
sub_count:    usize,
dbl_count:    usize,


}



// use crate::gadgetlib1::gadgets::fields::exponentiation_gadget;

//#endif // EXPONENTIATION_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the exponentiation gadget.

 See exponentiation_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef EXPONENTIATION_GADGET_TCC_
// #define EXPONENTIATION_GADGET_TCC_


impl exponentiation_gadget<FpkT, Fpk_variableT, Fpk_mul_gadgetT, Fpk_sqr_gadgetT, m>{
// 
pub fn new(pb:&protoboard<FieldT>,
                                                                                                       elt:&Fpk_variableT<FpkT>,
                                                                                                       power:&ffec::bigint<m>,
                                                                                                       result:&Fpk_variableT<FpkT>,
                                                                                                       annotation_prefix:&String) ->Self
    
{
    NAF = find_wnaf(1, power);

    intermed_count = 0;
    add_count = 0;
    sub_count = 0;
    dbl_count = 0;

    let mut  found_nonzero = false;
    for i in ( 0..=NAF.len() - 1).rev()
    {
        if found_nonzero
        {
            dbl_count+=1;
            intermed_count+=1;
        }

        if NAF[i] != 0
        {
            found_nonzero = true;

            if NAF[i] > 0
            {
                add_count+=1;
                intermed_count+=1;
            }
            else
            {
                sub_count+=1;
                intermed_count+=1;
            }
        }
    }

    intermediate.resize(intermed_count);
    intermediate[0].reset(Fpk_variableT::<FpkT>::new(pb, FpkT::one(), FMT(annotation_prefix, " intermediate_0")));
    for i in 1..intermed_count
    {
        intermediate[i].reset(Fpk_variableT::<FpkT>::new(pb, FMT(annotation_prefix, " intermediate_{}", i)));
    }
    addition_steps.resize(add_count);
    subtraction_steps.resize(sub_count);
    doubling_steps.resize(dbl_count);

    found_nonzero = false;

    let  (dbl_id , add_id , sub_id , intermed_id )=( 0,  0,  0,  0);

    for i in ( 0..=NAF.len() - 1).rev()
    {
        if found_nonzero
        {
            doubling_steps[dbl_id].reset(Fpk_sqr_gadgetT::<FpkT>::new(pb,
                                                                   *intermediate[intermed_id],
                                                                   if intermed_id + 1 == intermed_count {result} else{*intermediate[intermed_id+1]},
                                                                   FMT(annotation_prefix, " doubling_steps_{}", dbl_count)));
            intermed_id+=1;
            dbl_id+=1;
        }

        if NAF[i] != 0
        {
            found_nonzero = true;

            if NAF[i] > 0
            {
                /* next = cur * elt */
                addition_steps[add_id].reset(Fpk_mul_gadgetT::<FpkT>::new(pb,
                                                                       *intermediate[intermed_id],
                                                                       elt,
                                                                       if intermed_id + 1 == intermed_count {result} else{*intermediate[intermed_id+1]},
                                                                       FMT(annotation_prefix, " addition_steps_{}", dbl_count)));
                add_id+=1;
                intermed_id+=1;
            }
            else
            {
                /* next = cur / elt, i.e. next * elt = cur */
                subtraction_steps[sub_id].reset(Fpk_mul_gadgetT::<FpkT>::new(pb,
                                                                          if intermed_id + 1 == intermed_count {result} else{*intermediate[intermed_id+1]},
                                                                          elt,
                                                                          *intermediate[intermed_id],
                                                                          FMT(annotation_prefix, " subtraction_steps_{}", dbl_count)));
                sub_id+=1;
                intermed_id+=1;
            }
        }
    }
    // gadget<FieldT>(pb, annotation_prefix), 
    Self{elt, power, result}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..add_count
    {
        addition_steps[i].generate_r1cs_constraints();
    }

    for i in 0..sub_count
    {
        subtraction_steps[i].generate_r1cs_constraints();
    }

    for i in 0..dbl_count
    {
        doubling_steps[i].generate_r1cs_constraints();
    }
}

// 
pub fn generate_r1cs_witness()
{
    intermediate[0].generate_r1cs_witness(FpkT::one());

    let  found_nonzero = false;
    let mut  dbl_id = 0;let mut  add_id = 0;let mut  sub_id = 0;let mut  intermed_id = 0;

    for i in ( 0..=NAF.len() - 1).rev()
    {
        if found_nonzero
        {
            doubling_steps[dbl_id].generate_r1cs_witness();
            intermed_id+=1;
            dbl_id+=1;
        }

        if NAF[i] != 0
        {
            found_nonzero = true;

            if NAF[i] > 0
            {
                addition_steps[add_id].generate_r1cs_witness();
                intermed_id+=1;
                add_id+=1;
            }
            else
            {
                let cur_val= intermediate[intermed_id].get_element();
                let elt_val= elt.get_element();
                let next_val= cur_val * elt_val.inverse();

                 (if intermed_id + 1 == intermed_count {result} else{*intermediate[intermed_id+1]}).generate_r1cs_witness(next_val);

                subtraction_steps[sub_id].generate_r1cs_witness();

                intermed_id+=1;
                sub_id+=1;
            }
        }
    }
}}

// 
pub fn  test_exponentiation_gadget(power:&ffec::bigint<m>, annotation:&String)
{
    type FieldT= FpkT::my_Fp ;

    let mut  pb=protoboard::<FieldT>::new();
    let mut  x=Fpk_variableT::<FpkT>::new(pb, "x");
    let mut  x_to_power=Fpk_variableT::<FpkT>::new(pb, "x_to_power");
    let mut exp_gadget=exponentiation_gadget::<FpkT, Fpk_variableT, Fpk_mul_gadgetT, Fpk_sqr_gadgetT, m> ::new(pb, x, power, x_to_power, "exp_gadget");
    exp_gadget.generate_r1cs_constraints();

    for i in 0..10
    {
        let x_val= FpkT::random_element();
        x.generate_r1cs_witness(x_val);
        exp_gadget.generate_r1cs_witness();
        let res= x_to_power.get_element();
        assert!(pb.is_satisfied());
        assert!(res == (x_val ^ power));
    }
    print!("number of constraints for {}_exp = {}\n", annotation, pb.num_constraints());
    print!("exponent was: ");
    power.print();
}



//#endif // EXPONENTIATION_GADGET_TCC_
