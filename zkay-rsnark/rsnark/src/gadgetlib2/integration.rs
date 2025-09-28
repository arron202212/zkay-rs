/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef INTEGRATION_HPP_
#define INTEGRATION_HPP_

use  <libff/common/default_types/ec_pp.hpp>

use  <libsnark/gadgetlib2/protoboard.hpp>
use  <libsnark/relations/constraint_satisfaction_problems/r1cs/r1cs.hpp>

namespace libsnark {

r1cs_constraint_system<libff::Fr<libff::default_ec_pp> > get_constraint_system_from_gadgetlib2(const gadgetlib2::Protoboard &pb);
r1cs_variable_assignment<libff::Fr<libff::default_ec_pp> > get_variable_assignment_from_gadgetlib2(const gadgetlib2::Protoboard &pb);

} // libsnark

#endif // INTEGRATION_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <libsnark/gadgetlib2/adapters.hpp>
use  <libsnark/gadgetlib2/integration.hpp>

namespace libsnark {

linear_combination<libff::Fr<libff::default_ec_pp> > convert_gadgetlib2_linear_combination(const gadgetlib2::GadgetLibAdapter::linear_combination_t &lc)
{
    type libff::Fr<libff::default_ec_pp> FieldT;
    type gadgetlib2::GadgetLibAdapter GLA;

    linear_combination<FieldT> result = lc.second * variable<FieldT>(0);
    for (const GLA::linear_term_t &lt : lc.first)
    {
        result = result + lt.second * variable<FieldT>(lt.first+1);
    }

    return result;
}

r1cs_constraint_system<libff::Fr<libff::default_ec_pp> > get_constraint_system_from_gadgetlib2(const gadgetlib2::Protoboard &pb)
{
    type libff::Fr<libff::default_ec_pp> FieldT;
    type gadgetlib2::GadgetLibAdapter GLA;

    r1cs_constraint_system<FieldT> result;
    const GLA adapter;

    GLA::protoboard_t converted_pb = adapter.convert(pb);
    const int num_constraints = converted_pb.first.size();
    result.constraints.resize(num_constraints);
    printf("Num constraints: %d\n", num_constraints);

#ifdef MULTICORE
#pragma omp parallel default(none) shared(converted_pb, result), firstprivate(num_constraints)
  {
#pragma omp single nowait
    {
#endif
      for (int i = 0; i < num_constraints; ++i) {
        const auto& constr = converted_pb.first[i];
#ifdef MULTICORE
#pragma omp task default(none) shared(result, constr, i)
        {
#endif
          result.constraints[i].a = convert_gadgetlib2_linear_combination(std::get<0>(constr));
          result.constraints[i].b = convert_gadgetlib2_linear_combination(std::get<1>(constr));
          result.constraints[i].c = convert_gadgetlib2_linear_combination(std::get<2>(constr));
        }
#ifdef MULTICORE
      }
    }
#pragma omp taskwait
  }
#endif

    //The number of variables is the highest index created.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_variable_assignment_from_gadgetlib2.
    const size_t num_variables = GLA::getNextFreeIndex();
    result.primary_input_size = pb.numInputs();
    result.auxiliary_input_size = num_variables - pb.numInputs();
    return result;
}

r1cs_variable_assignment<libff::Fr<libff::default_ec_pp> > get_variable_assignment_from_gadgetlib2(const gadgetlib2::Protoboard &pb)
{
    type libff::Fr<libff::default_ec_pp> FieldT;
    type gadgetlib2::GadgetLibAdapter GLA;

    //The number of variables is the highest index created. This is also the required size for the assignment vector.
    //TODO: If there are multiple protoboards, or variables not assigned to a protoboard, then getNextFreeIndex() is *not* the number of variables! See also in get_constraint_system_from_gadgetlib2.
    const size_t num_vars = GLA::getNextFreeIndex();
    const GLA adapter;
    r1cs_variable_assignment<FieldT> result(num_vars, FieldT::zero());
    VariableAssignment assignment = pb.assignment();

    //Go over all assigned values of the protoboard, from every variable-value pair, put the value in the variable.index place of the new assignment.
    for(VariableAssignment::iterator iter = assignment.begin(); iter != assignment.end(); ++iter){
    	result[GLA::getVariableIndex(iter->first)] = adapter.convert(iter->second);
    }

    return result;
}

}
