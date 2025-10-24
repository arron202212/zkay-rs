/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
use  <algorithm>
use  <cassert>
use  <cstdio>
use  <cstring>
use  <vector>

use ffec::algebra::curves::mnt::mnt6::mnt6_pp;
use ffec::algebra::field_utils::field_utils;
use ffec::common::profiling;
use ffec::common::utils;

use libsnark/reductions/uscs_to_ssp/uscs_to_ssp;
use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;



template<typename FieldT>
void test_ssp(const size_t num_constraints, const size_t num_inputs, const bool binary_input)
{
    ffec::enter_block("Call to test_ssp");

    ffec::print_indent(); print!("* Number of constraints: {}\n", num_constraints);
    ffec::print_indent(); print!("* Number of inputs: {}\n", num_inputs);
    ffec::print_indent(); print!("* Input type: %s\n",if  binary_input  {"binary" }else {"field"});

    ffec::enter_block("Generate constraint system and assignment");
    uscs_example<FieldT> example;
    if binary_input
    {
        example = generate_uscs_example_with_binary_input<FieldT>(num_constraints, num_inputs);
    }
    else
    {
        example = generate_uscs_example_with_field_input<FieldT>(num_constraints, num_inputs);
    }
    ffec::leave_block("Generate constraint system and assignment");

    ffec::enter_block("Check satisfiability of constraint system");
    assert!(example.constraint_system.is_satisfied(example.primary_input, example.auxiliary_input));
    ffec::leave_block("Check satisfiability of constraint system");

    const FieldT t = FieldT::random_element(),
                 d = FieldT::random_element();

    ffec::enter_block("Compute SSP instance 1");
    ssp_instance<FieldT> ssp_inst_1 = uscs_to_ssp_instance_map(example.constraint_system);
    ffec::leave_block("Compute SSP instance 1");

    ffec::enter_block("Compute SSP instance 2");
    ssp_instance_evaluation<FieldT> ssp_inst_2 = uscs_to_ssp_instance_map_with_evaluation(example.constraint_system, t);
    ffec::leave_block("Compute SSP instance 2");

    ffec::enter_block("Compute SSP witness");
    ssp_witness<FieldT> ssp_wit = uscs_to_ssp_witness_map(example.constraint_system, example.primary_input, example.auxiliary_input, d);
    ffec::leave_block("Compute SSP witness");

    ffec::enter_block("Check satisfiability of SSP instance 1");
    assert!(ssp_inst_1.is_satisfied(ssp_wit));
    ffec::leave_block("Check satisfiability of SSP instance 1");

    ffec::enter_block("Check satisfiability of SSP instance 2");
    assert!(ssp_inst_2.is_satisfied(ssp_wit));
    ffec::leave_block("Check satisfiability of SSP instance 2");

    ffec::leave_block("Call to test_ssp");
}

int main()
{
    ffec::start_profiling();

    ffec::mnt6_pp::init_public_params();

    const size_t num_inputs = 10;

    const size_t basic_domain_size = 1u64<<ffec::mnt6_Fr::s;
    const size_t step_domain_size = (1u64<<10) + (1u64<<8);
    const size_t extended_domain_size = 1u64<<(ffec::mnt6_Fr::s+1);
    const size_t extended_domain_size_special = extended_domain_size-1;

    ffec::enter_block("Test SSP for binary inputs");

    test_ssp<ffec::Fr<ffec::mnt6_pp> >(basic_domain_size, num_inputs, true);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(step_domain_size, num_inputs, true);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size, num_inputs, true);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size_special, num_inputs, true);

    ffec::leave_block("Test SSP for binary inputs");

    ffec::enter_block("Test SSP for field inputs");

    test_ssp<ffec::Fr<ffec::mnt6_pp> >(basic_domain_size, num_inputs, false);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(step_domain_size, num_inputs, false);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size, num_inputs, false);
    test_ssp<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size_special, num_inputs, false);

    ffec::leave_block("Test SSP for field inputs");
}
