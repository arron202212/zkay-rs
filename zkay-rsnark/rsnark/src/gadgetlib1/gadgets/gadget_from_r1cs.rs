/** @file
 *****************************************************************************

 Declaration of interfaces for a gadget that can be created from an R1CS constraint system.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef GADGET_FROM_R1CS_HPP_
// #define GADGET_FROM_R1CS_HPP_

// use  <map>

use crate::gadgetlib1::gadget;




pub struct gadget_from_r1cs<FieldT> {//gadget<FieldT>


    vars:Vec<pb_variable_array<FieldT> >,
    cs:r1cs_constraint_system<FieldT>,
cs_to_vars:    BTreeMap<usize, usize>,



}



// use crate::gadgetlib1::gadgets::gadget_from_r1cs;

//#endif // GADGET_FROM_R1CS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a gadget that can be created from an R1CS constraint system.

 See gadget_from_r1cs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef GADGET_FROM_R1CS_TCC_
// #define GADGET_FROM_R1CS_TCC_


impl gadget_from_r1cs<FieldT>{

pub fn new(pb:protoboard<FieldT>,
                                           vars:&Vec<pb_variable_array<FieldT> >,
                                           cs:&r1cs_constraint_system<FieldT>,
                                           annotation_prefix:&String)->Self
    
{
    cs_to_vars[0] = 0; /* constant term maps to constant term */

    let mut  cs_var_idx = 1;
    for va in &vars
    {
// #ifdef DEBUG
        print!("gadget_from_r1cs: translating a block of variables with length {}\n", va.len());
//#endif
        for v in &va
        {
            cs_to_vars[cs_var_idx] = v.index;

// #ifdef DEBUG
            if v.index != 0
            {
                // handle annotations, except for re-annotating constant term
                let  it = cs.variable_annotations.find(cs_var_idx);

                let  annotation = FMT(annotation_prefix, " variable_{}", cs_var_idx);
                if it != cs.variable_annotations.end()
                {
                    annotation = annotation_prefix + " " + it.1;
                }

                pb.augment_variable_annotation(v, annotation);
            }
//#endif
            cs_var_idx+=1;
        }
    }

// #ifdef DEBUG
    print!("gadget_from_r1cs: sum of all block lengths: {}\n", cs_var_idx-1);
    print!("gadget_from_r1cs: cs.num_variables(): {}\n", cs.num_variables());
//#endif

    assert!(cs_var_idx - 1 == cs.num_variables());
    // gadget<FieldT>(pb, annotation_prefix),
   Self{vars,
    cs}
}


pub fn generate_r1cs_constraints()
{
    for i in 0..cs.num_constraints()
    {
       let  constr = &cs.constraints[i];
        let mut translated_constr=r1cs_constraint::<FieldT> ::new();

        for t in &constr.a.terms
        {
            translated_constr.a.terms.push(linear_term::<FieldT>(pb_variable::<FieldT>(cs_to_vars[t.index]), t.coeff));
        }

        for t in &constr.b.terms
        {
            translated_constr.b.terms.push(linear_term::<FieldT>(pb_variable::<FieldT>(cs_to_vars[t.index]), t.coeff));
        }

        for t in &constr.c.terms
        {
            translated_constr.c.terms.push(linear_term::<FieldT>(pb_variable::<FieldT>(cs_to_vars[t.index]), t.coeff));
        }

        let  annotation = FMT(self.annotation_prefix, " constraint_{}", i);

// #ifdef DEBUG
        let  it = cs.constraint_annotations.find(i);
        if it != cs.constraint_annotations.end()
        {
            annotation = self.annotation_prefix + " " + it.1;
        }
//#endif
        self.pb.add_r1cs_constraint(translated_constr, annotation);
    }
}


pub fn generate_r1cs_witness(primary_input:&r1cs_primary_input<FieldT>,
                                                     auxiliary_input:&r1cs_auxiliary_input<FieldT>)
{
    assert!(cs.num_inputs() == primary_input.len());
    assert!(cs.num_variables() == primary_input.len() + auxiliary_input.len());

    for i in 0..primary_input.len()
    {
        self.pb.val(pb_variable::<FieldT>(cs_to_vars[i+1])) = primary_input[i];
    }

    for i in 0..auxiliary_input.len()
    {
        self.pb.val(pb_variable::<FieldT>(cs_to_vars[primary_input.len()+i+1])) = auxiliary_input[i];
    }
}

}

//#endif // GADGET_FROM_R1CS_TCC_
