


// use ff_curves::algebra::curves::mnt::mnt6::mnt6_pp;
// use algebra::field_utils::field_utils;
// use common::profiling;
// use common::utils;

// use crate::reductions::r1cs_to_qap::r1cs_to_qap;
// use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples;




pub fn  test_qap(qap_degree:usize, num_inputs:usize, binary_input:bool)
{
    /*
      We construct an instance where the QAP degree is qap_degree.
      So we generate an instance of R1CS where the number of constraints qap_degree - num_inputs - 1.
      See the transformation from R1CS to QAP for why this is the case.
      So we need that qap_degree >= num_inputs + 1.
    */
    assert!(num_inputs + 1 <= qap_degree);
    enter_block("Call to test_qap",false);

    let num_constraints = qap_degree - num_inputs - 1;

    print_indent(); print!("* QAP degree: {}\n", qap_degree);
    print_indent(); print!("* Number of inputs: {}\n", num_inputs);
    print_indent(); print!("* Number of R1CS constraints: {}\n", num_constraints);
    print_indent(); print!("* Input type: {}\n", if binary_input  {"binary" }else {"field"});

    enter_block("Generate constraint system and assignment",false);
    let  example=
    if binary_input
    {
         generate_r1cs_example_with_binary_input::<FieldT>(num_constraints, num_inputs)
    }
    else
    {
         generate_r1cs_example_with_field_input::<FieldT>(num_constraints, num_inputs)
    };
    leave_block("Generate constraint system and assignment",false);

    enter_block("Check satisfiability of constraint system",false);
    assert!(example.constraint_system.is_satisfied(example.primary_input, example.auxiliary_input));
    leave_block("Check satisfiability of constraint system",false);

    let t= FieldT::random_element(),
    d1 = FieldT::random_element(),
    d2 = FieldT::random_element(),
    d3 = FieldT::random_element();

    enter_block("Compute QAP instance 1",false);
    let qap_inst_1 = r1cs_to_qap_instance_map(example.constraint_system);
    leave_block("Compute QAP instance 1",false);

    enter_block("Compute QAP instance 2",false);
    let qap_inst_2 = r1cs_to_qap_instance_map_with_evaluation(example.constraint_system, t);
    leave_block("Compute QAP instance 2",false);

    enter_block("Compute QAP witness",false);
    let  qap_wit = r1cs_to_qap_witness_map(example.constraint_system, example.primary_input, example.auxiliary_input, d1, d2, d3);
    leave_block("Compute QAP witness",false);

    enter_block("Check satisfiability of QAP instance 1",false);
    assert!(qap_inst_1.is_satisfied(qap_wit));
    leave_block("Check satisfiability of QAP instance 1",false);

    enter_block("Check satisfiability of QAP instance 2",false);
    assert!(qap_inst_2.is_satisfied(qap_wit));
    leave_block("Check satisfiability of QAP instance 2",false);

    leave_block("Call to test_qap",false);
}

fn  main()->i32
{
    start_profiling();

    mnt6_pp::init_public_params();

    let num_inputs = 10;

    let basic_domain_size = 1u64<<mnt6_Fr::s;
    let step_domain_size = (1u64<<10) + (1u64<<8);
    let extended_domain_size = 1u64<<(mnt6_Fr::s+1);
    let extended_domain_size_special = extended_domain_size-1;

    enter_block("Test QAP with binary input",false);

    test_qap::<Fr<mnt6_pp> >(basic_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp> >(step_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp> >(extended_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp> >(extended_domain_size_special, num_inputs, true);

    leave_block("Test QAP with binary input",false);

    enter_block("Test QAP with field input",false);

    test_qap::<Fr<mnt6_pp> >(basic_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp> >(step_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp> >(extended_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp> >(extended_domain_size_special, num_inputs, false);

    leave_block("Test QAP with field input",false);
0
}
