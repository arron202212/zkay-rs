/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef INTEGRATION_HPP_
// #define INTEGRATION_HPP_

use ffec::common::default_types::ec_pp;

use crate::gadgetlib2::protoboard;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



// r1cs_constraint_system<Fr<default_ec_pp> > get_constraint_system_from_gadgetlib2(const gadgetlib2::Protoboard &pb);
// r1cs_variable_assignment<Fr<default_ec_pp> > get_variable_assignment_from_gadgetlib2(const gadgetlib2::Protoboard &pb);



//#endif // INTEGRATION_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use crate::gadgetlib2::adapters;
use crate::gadgetlib2::integration;
use crate::gadgetlib2::adapters::GadgetLibAdapter::linear_combination_t;
use crate::relations::linear_combination;
use crate::gadgetlib2::Protoboard;
use crate::gadgetlib2::adapters::GadgetLibAdapter;
 type FieldT=Fr<default_ec_pp> ;
    type GLA=GadgetLibAdapter ;
pub fn convert_gadgetlib2_linear_combination(lc:&linear_combination_t)->linear_combination<Fr<default_ec_pp> > 
{


    let mut  result = lc.2 * variable::<FieldT>(0);
    for lt in &lc.0
    {
        result = result + lt.2 * variable::<FieldT>(lt.0+1);
    }

    return result;
}

pub fn get_constraint_system_from_gadgetlib2(pb:&Protoboard)->r1cs_constraint_system<Fr<default_ec_pp> > 
{


    let mut  result=r1cs_constraint_system::<FieldT>();
    let   adapter=GLA();

    let  converted_pb = adapter.convert(pb);
    let num_constraints = converted_pb.0.len();
    result.constraints.resize(num_constraints,FieldT::zero());
    print!("Num constraints: {}\n", num_constraints);

// #ifdef MULTICORE
// #pragma omp parallel default(none) shared(converted_pb, result), firstprivate(num_constraints)
//   {
// #pragma omp single nowait
//     {
//#endif
      for i in 0..num_constraints {
        let  constr = converted_pb.0[i];
// #ifdef MULTICORE
// #pragma omp task default(none) shared(result, constr, i)
//         {
//#endif
          result.constraints[i].a = convert_gadgetlib2_linear_combination(constr[0]);
          result.constraints[i].b = convert_gadgetlib2_linear_combination(constr[1]);
          result.constraints[i].c = convert_gadgetlib2_linear_combination(constr[2]);
        }
// #ifdef MULTICORE
//       }
//     }
// #pragma omp taskwait
//   }
//#endif

    //The number of variables is the highest index created.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_variable_assignment_from_gadgetlib2.
    let  num_variables = GLA::getNextFreeIndex();
    result.primary_input_size = pb.numInputs();
    result.auxiliary_input_size = num_variables - pb.numInputs();
    return result;
}

pub fn  get_variable_assignment_from_gadgetlib2(pb:&Protoboard)->r1cs_variable_assignment<Fr<default_ec_pp> >
{
    // type Fr<default_ec_pp> FieldT;
    // type gadgetlib2::GadgetLibAdapter GLA;

    //The number of variables is the highest index created. This is also the required size for the assignment vector.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_constraint_system_from_gadgetlib2.
    let  num_vars = GLA::getNextFreeIndex();
    let   adapter=GLA;
    let result=r1cs_variable_assignment::<FieldT> ::new(num_vars, FieldT::zero());
    let  assignment = pb.assignment();

    //Go over all assigned values of the protoboard, from every variable-value pair, put the value in the variable.index place of the new assignment.
    for  iter in  &assignment{
    	result[GLA::getVariableIndex(iter.0)] = adapter.convert(iter.1);
    }

    return result;
}


