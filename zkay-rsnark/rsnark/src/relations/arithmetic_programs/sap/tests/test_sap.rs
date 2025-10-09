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

use ffec::algebra::curves::mnt/mnt6/mnt6_pp;
use ffec::algebra::fields::field_utils;
use ffec::common::profiling;
use ffec::common::utils;

use libsnark::reductions::r1cs_to_sap::r1cs_to_sap;
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;



template<typename FieldT>
void test_sap(const size_t sap_degree, const size_t num_inputs, const bool binary_input)
{
    /*
      We construct an instance where the SAP degree is <= sap_degree.
      The R1CS-to-SAP reduction produces SAPs with degree
        (2 * num_constraints + 2 * num_inputs + 1).
      So we generate an instance of R1CS where the number of constraints is
        (sap_degree - 1) / 2 - num_inputs.
    */
    ffec::enter_block("Call to test_sap");

    const size_t num_constraints = (sap_degree - 1) / 2 - num_inputs;
    assert!(num_constraints >= 1);

    ffec::print_indent(); print!("* Requested SAP degree: {}\n", sap_degree);
    ffec::print_indent(); print!("* Actual SAP degree: {}\n", 2 * num_constraints + 2 * num_inputs + 1);
    ffec::print_indent(); print!("* Number of inputs: {}\n", num_inputs);
    ffec::print_indent(); print!("* Number of R1CS constraints: {}\n", num_constraints);
    ffec::print_indent(); print!("* Input type: %s\n", binary_input ? "binary" : "field");

    ffec::enter_block("Generate constraint system and assignment");
    r1cs_example<FieldT> example;
    if (binary_input)
    {
        example = generate_r1cs_example_with_binary_input<FieldT>(num_constraints, num_inputs);
    }
    else
    {
        example = generate_r1cs_example_with_field_input<FieldT>(num_constraints, num_inputs);
    }
    ffec::leave_block("Generate constraint system and assignment");

    ffec::enter_block("Check satisfiability of constraint system");
    assert!(example.constraint_system.is_satisfied(example.primary_input, example.auxiliary_input));
    ffec::leave_block("Check satisfiability of constraint system");

    const FieldT t = FieldT::random_element(),
    d1 = FieldT::random_element(),
    d2 = FieldT::random_element();

    ffec::enter_block("Compute SAP instance 1");
    sap_instance<FieldT> sap_inst_1 = r1cs_to_sap_instance_map(example.constraint_system);
    ffec::leave_block("Compute SAP instance 1");

    ffec::enter_block("Compute SAP instance 2");
    sap_instance_evaluation<FieldT> sap_inst_2 = r1cs_to_sap_instance_map_with_evaluation(example.constraint_system, t);
    ffec::leave_block("Compute SAP instance 2");

    ffec::enter_block("Compute SAP witness");
    sap_witness<FieldT> sap_wit = r1cs_to_sap_witness_map(example.constraint_system, example.primary_input, example.auxiliary_input, d1, d2);
    ffec::leave_block("Compute SAP witness");

    ffec::enter_block("Check satisfiability of SAP instance 1");
    assert!(sap_inst_1.is_satisfied(sap_wit));
    ffec::leave_block("Check satisfiability of SAP instance 1");

    ffec::enter_block("Check satisfiability of SAP instance 2");
    assert!(sap_inst_2.is_satisfied(sap_wit));
    ffec::leave_block("Check satisfiability of SAP instance 2");

    ffec::leave_block("Call to test_sap");
}

int main()
{
    ffec::start_profiling();

    ffec::mnt6_pp::init_public_params();

    const size_t num_inputs = 10;

    /**
     * due to the specifics of our reduction, we can only get SAPs with odd
     * degrees, so we can only test "special" versions of the domains
     */

    const size_t basic_domain_size_special = (1ul<<ffec::mnt6_Fr::s) - 1ul;
    const size_t step_domain_size_special = (1ul<<10) + (1ul<<8) - 1ul;
    const size_t extended_domain_size_special = (1ul<<(ffec::mnt6_Fr::s+1)) - 1ul;

    ffec::enter_block("Test SAP with binary input");

    test_sap<ffec::Fr<ffec::mnt6_pp> >(basic_domain_size_special, num_inputs, true);
    test_sap<ffec::Fr<ffec::mnt6_pp> >(step_domain_size_special, num_inputs, true);
    test_sap<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size_special, num_inputs, true);

    ffec::leave_block("Test SAP with binary input");

    ffec::enter_block("Test SAP with field input");

    test_sap<ffec::Fr<ffec::mnt6_pp> >(basic_domain_size_special, num_inputs, false);
    test_sap<ffec::Fr<ffec::mnt6_pp> >(step_domain_size_special, num_inputs, false);
    test_sap<ffec::Fr<ffec::mnt6_pp> >(extended_domain_size_special, num_inputs, false);

    ffec::leave_block("Test SAP with field input");
}
