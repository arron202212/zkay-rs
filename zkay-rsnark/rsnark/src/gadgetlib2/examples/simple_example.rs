/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SIMPLE_EXAMPLE_HPP_
// #define SIMPLE_EXAMPLE_HPP_

use ffec::common::default_types::ec_pp;

use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;



// r1cs_example<ffec::Fr<ffec::default_ec_pp> > gen_r1cs_example_from_gadgetlib2_protoboard(size:usize);



//#endif // SIMPLE_EXAMPLE_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::gadgetlib2::adapters;
// use crate::gadgetlib2::examples::simple_example;
use crate::gadgetlib2::gadget;
use crate::gadgetlib2::integration;



/* NOTE: all examples here actually generate one constraint less to account for soundness constraint in QAP */
pub fn  gen_r1cs_example_from_gadgetlib2_protoboard(size:usize)->r1cs_example<ffec::Fr<ffec::default_ec_pp> >
{
    // type FieldT=ffec::Fr<ffec::default_ec_pp> ;

    initPublicParamsFromDefaultPp();
    // necessary in case a protoboard was built before,  libsnark assumes variable indices always
    // begin with 0 so we must reset the index before creating constraints which will be used by
    // libsnark
    GadgetLibAdapter::resetVariableIndex();

    // create a gadgetlib2 gadget. This part is done by both generator and prover.
    let pb = Protoboard::create(R1P);
    let  A=VariableArray::new(size, "A");
    let  B=VariableArray::new(size, "B");
    let result= Variable::new("result");
    let g = InnerProduct_Gadget::create(pb, A, B, result);
    // create constraints. This part is done by generator.
    g.generateConstraints();
    // create assignment (witness). This part is done by prover.
     use rand::Rng;
    let mut rng = rand::thread_rng();
    for  k in  0.. size
    {
        pb.val(A[k]) = rng::r#gen::<i32>() % 2;
        pb.val(B[k]) = rng::r#gen::<i32>() % 2;
    }
    g.generateWitness();
    // translate constraint system to libsnark format.
    let   cs = get_constraint_system_from_gadgetlib2(pb);
    // translate full variable assignment to libsnark format
    let  full_assignment = get_variable_assignment_from_gadgetlib2(pb);
    // extract primary and auxiliary input
    let   primary_input=r1cs_primary_input::<FieldT>::new(&full_assignment[..cs.num_inputs()]);
    let   auxiliary_input=r1cs_auxiliary_input::<FieldT>::new(&full_assignment[cs.num_inputs()..]);

    assert!(cs.is_valid());
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    return r1cs_example::<FieldT>::new(cs, primary_input, auxiliary_input);
}



