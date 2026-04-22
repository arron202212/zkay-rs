
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::reductions::r1cs_to_qap::r1cs_to_qap::{
    r1cs_to_qap_instance_map, r1cs_to_qap_instance_map_with_evaluation, r1cs_to_qap_witness_map,
};
use crate::relations::constraint_satisfaction_problems::r1cs::examples::r1cs_examples::{
    generate_r1cs_example_with_binary_input, generate_r1cs_example_with_field_input,
};
use ff_curves::Fr;
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent, start_profiling};
use tracing::{span, Level};


pub fn test_qap<FieldT: FieldTConfig>(qap_degree: usize, num_inputs: usize, binary_input: bool) {
    // /*
    //   We construct an instance where the QAP degree is qap_degree.
    //   So we generate an instance of R1CS where the number of constraints qap_degree - num_inputs - 1.
    //   See the transformation from R1CS to QAP for why this is the case.
    //   So we need that qap_degree >= num_inputs + 1.
    // */
    assert!(num_inputs + 1 <= qap_degree);
    let span = span!(Level::TRACE, "Call to test_qap").entered();

    let num_constraints = qap_degree - num_inputs - 1;

    print_indent();
    print!("* QAP degree: {}\n", qap_degree);
    print_indent();
    print!("* Number of inputs: {}\n", num_inputs);
    print_indent();
    print!("* Number of R1CS constraints: {}\n", num_constraints);
    print_indent();
    print!(
        "* Input type: {}\n",
        if binary_input { "binary" } else { "field" }
    );

    let span = span!(Level::TRACE, "Generate constraint system and assignment").entered();
    let example = if binary_input {
        generate_r1cs_example_with_binary_input::<FieldT, pb_variable, pb_linear_combination>(
            num_constraints,
            num_inputs,
        )
    } else {
        generate_r1cs_example_with_field_input::<FieldT, pb_variable, pb_linear_combination>(
            num_constraints,
            num_inputs,
        )
    };
    span.exit();

    let span = span!(Level::TRACE, "Check satisfiability of constraint system").entered();
    assert!(
        example
            .constraint_system
            .is_satisfied(&example.primary_input, &example.auxiliary_input)
    );
    span.exit();

    let (t, d1, d2, d3) = (
        FieldT::random_element(),
        FieldT::random_element(),
        FieldT::random_element(),
        FieldT::random_element(),
    );

    let span = span!(Level::TRACE, "Compute QAP instance 1").entered();
    let qap_inst_1 = r1cs_to_qap_instance_map(&example.constraint_system);
    span.exit();

    let span = span!(Level::TRACE, "Compute QAP instance 2").entered();
    let qap_inst_2 = r1cs_to_qap_instance_map_with_evaluation(&example.constraint_system, &t);
    span.exit();

    let span = span!(Level::TRACE, "Compute QAP witness").entered();
    let qap_wit = r1cs_to_qap_witness_map(
        &example.constraint_system,
        &example.primary_input,
        &example.auxiliary_input,
        &d1,
        &d2,
        &d3,
    );
    span.exit();

    let span = span!(Level::TRACE, "Check satisfiability of QAP instance 1").entered();
    assert!(qap_inst_1.is_satisfied(&qap_wit));
    span.exit();

    let span = span!(Level::TRACE, "Check satisfiability of QAP instance 2").entered();
    assert!(qap_inst_2.is_satisfied(&qap_wit));
    span.exit();

    span.exit();
}

fn main<mnt6_pp: ppTConfig, mnt6_Fr: ppTConfig>() -> i32 {
    start_profiling();

    mnt6_pp::init_public_params();

    let num_inputs = 10;

    let basic_domain_size = 1usize << mnt6_Fr::s;
    let step_domain_size = (1usize << 10) + (1usize << 8);
    let extended_domain_size = 1usize << (mnt6_Fr::s + 1);
    let extended_domain_size_special = extended_domain_size - 1;

    let span = span!(Level::TRACE, "Test QAP with binary input").entered();

    test_qap::<Fr<mnt6_pp>>(basic_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp>>(step_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp>>(extended_domain_size, num_inputs, true);
    test_qap::<Fr<mnt6_pp>>(extended_domain_size_special, num_inputs, true);

    span.exit();

    let span = span!(Level::TRACE, "Test QAP with field input").entered();

    test_qap::<Fr<mnt6_pp>>(basic_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp>>(step_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp>>(extended_domain_size, num_inputs, false);
    test_qap::<Fr<mnt6_pp>>(extended_domain_size_special, num_inputs, false);

    span.exit();
    0
}
