use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::reductions::uscs_to_ssp::uscs_to_ssp::{
    uscs_to_ssp_instance_map, uscs_to_ssp_instance_map_with_evaluation, uscs_to_ssp_witness_map,
};
use crate::relations::constraint_satisfaction_problems::uscs::examples::uscs_examples::{
    generate_uscs_example_with_binary_input, generate_uscs_example_with_field_input,
};
use ff_curves::Fr;
use ffec::FieldTConfig;
use ffec::PpConfig;
use ffec::common::profiling::{
    enter_block, leave_block, print_header, print_indent, start_profiling,
};
use std::marker::PhantomData;
use tracing::{Level, span};

pub fn test_ssp<FieldT: FieldTConfig>(
    num_constraints: usize,
    num_inputs: usize,
    binary_input: bool,
) {
    let span0 = span!(Level::TRACE, "Call to test_ssp");
    let _=span0.enter();

    print_indent();
    print!("* Number of constraints: {}\n", num_constraints);
    print_indent();
    print!("* Number of inputs: {}\n", num_inputs);
    print_indent();
    print!(
        "* Input type: {}\n",
        if binary_input { "binary" } else { "field" }
    );

    let spang = span!(Level::TRACE, "Generate constraint system and assignment").entered();
    let example = if binary_input {
        generate_uscs_example_with_binary_input::<FieldT, pb_variable, pb_linear_combination>(
            num_constraints,
            num_inputs,
        )
    } else {
        generate_uscs_example_with_field_input::<FieldT, pb_variable, pb_linear_combination>(
            num_constraints,
            num_inputs,
        )
    };
    spang.exit();

    let spanc = span!(Level::TRACE, "Check satisfiability of constraint system").entered();
    assert!(
        example
            .constraint_system
            .is_satisfied(&example.primary_input, &example.auxiliary_input)
    );
    spanc.exit();

    let t = FieldT::random_element();
    let d = FieldT::random_element();

    let spans1 = span!(Level::TRACE, "Compute SSP instance 1").entered();
    let ssp_inst_1 = uscs_to_ssp_instance_map(&example.constraint_system);
    spans1.exit();

    let spans2= span!(Level::TRACE, "Compute SSP instance 2").entered();
    let ssp_inst_2 = uscs_to_ssp_instance_map_with_evaluation(&example.constraint_system, &t);
    spans2.exit();

    let spans = span!(Level::TRACE, "Compute SSP witness").entered();
    let ssp_wit = uscs_to_ssp_witness_map(
        &example.constraint_system,
        &example.primary_input,
        &example.auxiliary_input,
        &d,
    );
    spans.exit();

    let spans1 = span!(Level::TRACE, "Check satisfiability of SSP instance 1").entered();
    assert!(ssp_inst_1.is_satisfied(&ssp_wit));
    spans1.exit();

    let span = span!(Level::TRACE, "Check satisfiability of SSP instance 2").entered();
    assert!(ssp_inst_2.is_satisfied(&ssp_wit));
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

    let spanb = span!(Level::TRACE, "Test SSP for binary inputs").entered();

    test_ssp::<Fr<mnt6_pp>>(basic_domain_size, num_inputs, true);
    test_ssp::<Fr<mnt6_pp>>(step_domain_size, num_inputs, true);
    test_ssp::<Fr<mnt6_pp>>(extended_domain_size, num_inputs, true);
    test_ssp::<Fr<mnt6_pp>>(extended_domain_size_special, num_inputs, true);

    spanb.exit();

    let span = span!(Level::TRACE, "Test SSP for field inputs").entered();

    test_ssp::<Fr<mnt6_pp>>(basic_domain_size, num_inputs, false);
    test_ssp::<Fr<mnt6_pp>>(step_domain_size, num_inputs, false);
    test_ssp::<Fr<mnt6_pp>>(extended_domain_size, num_inputs, false);
    test_ssp::<Fr<mnt6_pp>>(extended_domain_size_special, num_inputs, false);

    span.exit();
    0
}
