// /** @file
//  *****************************************************************************

//  Declaration of interfaces for a R1CS example, as well as functions to sample
//  R1CS examples with prescribed parameters (according to some distribution).

//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
//#ifndef R1CS_EXAMPLES_HPP_
// #define R1CS_EXAMPLES_HPP_
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
    r1cs_variable_assignment,
};
use crate::relations::variable::linear_combination;
use ffec::common::profiling::{enter_block, leave_block};
use std::collections::BTreeMap;
/**
 * A R1CS example comprises a R1CS constraint system, R1CS input, and R1CS witness.
 */

pub struct r1cs_example<FieldT: FieldTConfig> {
    constraint_system: r1cs_constraint_system<FieldT>,
    primary_input: r1cs_primary_input<FieldT>,
    auxiliary_input: r1cs_auxiliary_input<FieldT>,
}
impl<FieldT: FieldTConfig> r1cs_example<FieldT> {
    // r1cs_example<FieldT>() = default;
    // r1cs_example<FieldT>(other:&r1cs_example<FieldT>) = default;
    pub fn new(
        constraint_system: r1cs_constraint_system<FieldT>,
        primary_input: r1cs_primary_input<FieldT>,
        auxiliary_input: r1cs_auxiliary_input<FieldT>,
    ) -> Self {
        Self {
            constraint_system,
            primary_input,
            auxiliary_input,
        }
    }
    pub fn new2(
        constraint_system: r1cs_constraint_system<FieldT>,
        primary_input: r1cs_primary_input<FieldT>,
        auxiliary_input: r1cs_auxiliary_input<FieldT>,
    ) -> Self {
        Self {
            constraint_system,
            primary_input,
            auxiliary_input,
        }
    }
}

/**
 * Generate a R1CS example such that:
 * - the number of constraints of the R1CS constraint system is num_constraints;
 * - the number of variables of the R1CS constraint system is (approximately) num_constraints;
 * - the number of inputs of the R1CS constraint system is num_inputs;
 * - the R1CS input consists of ``full'' field elements (typically require the whole log|Field| bits to represent).
//  */
// < FieldT>
// r1cs_example<FieldT> generate_r1cs_example_with_field_input(num_constraints:usize,
//                                                             num_inputs:usize);

// /**
//  * Generate a R1CS example such that:
//  * - the number of constraints of the R1CS constraint system is num_constraints;
//  * - the number of variables of the R1CS constraint system is (approximately) num_constraints;
//  * - the number of inputs of the R1CS constraint system is num_inputs;
//  * - the R1CS input consists of binary values (as opposed to ``full'' field elements).
//  */
// < FieldT>
// r1cs_example<FieldT> generate_r1cs_example_with_binary_input(num_constraints:usize,
//                                                              num_inputs:usize);

// use crate::relations::constraint_satisfaction_problems/r1cs/examples/r1cs_examples;

//#endif // R1CS_EXAMPLES_HPP_
/** @file
*****************************************************************************

Implementation of functions to sample R1CS examples with prescribed parameters
(according to some distribution).

See r1cs_examples.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef R1CS_EXAMPLES_TCC_
// #define R1CS_EXAMPLES_TCC_

// use  <cassert>
use ffec::common::utils;

use rand::Rng;

pub fn generate_r1cs_example_with_field_input<FieldT: FieldTConfig>(
    num_constraints: usize,
    num_inputs: usize,
) -> r1cs_example<FieldT> {
    enter_block("Call to generate_r1cs_example_with_field_input", false);

    assert!(num_inputs <= num_constraints + 2);

    let mut cs = r1cs_constraint_system::<FieldT>::default();
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = 2 + num_constraints - num_inputs; // TODO: explain this

    let mut full_variable_assignment = r1cs_variable_assignment::<FieldT>::new();
    let mut a = FieldT::random_element();
    let mut b = FieldT::random_element();
    full_variable_assignment.push(a.clone());
    full_variable_assignment.push(b.clone());

    for i in 0..num_constraints - 1 {
        let (mut A, mut B, mut C) = (
            linear_combination::<FieldT>::default(),
            linear_combination::<FieldT>::default(),
            linear_combination::<FieldT>::default(),
        );

        if i % 2 != 0 {
            // a * b = c
            A.add_term(i + 1, 1);
            B.add_term(i + 2, 1);
            C.add_term(i + 3, 1);
            let tmp = a * b.clone();
            full_variable_assignment.push(tmp.clone());
            a = b;
            b = tmp;
        } else {
            // a + b = c
            B.add_term(0, 1);
            A.add_term(i + 1, 1);
            A.add_term(i + 2, 1);
            C.add_term(i + 3, 1);
            let tmp = a + b.clone();
            full_variable_assignment.push(tmp.clone());
            a = b;
            b = tmp;
        }

        cs.add_constraint(r1cs_constraint::<FieldT>::new(A, B, C));
    }

    let (mut A, mut B, mut C) = (
        linear_combination::<FieldT>::default(),
        linear_combination::<FieldT>::default(),
        linear_combination::<FieldT>::default(),
    );
    let mut fin = FieldT::zero();
    for i in 1..cs.num_variables() {
        A.add_term(i, 1);
        B.add_term(i, 1);
        fin = fin + full_variable_assignment[i - 1].clone();
    }
    C.add_term(cs.num_variables(), 1);
    cs.add_constraint(r1cs_constraint::<FieldT>::new(A, B, C));
    full_variable_assignment.push(fin.squared());

    /* split variable assignment */
    let primary_input = full_variable_assignment[..num_inputs].to_vec();
    let auxiliary_input = full_variable_assignment[num_inputs..].to_vec();

    /* sanity checks */
    assert!(cs.num_variables() == full_variable_assignment.len());
    assert!(cs.num_variables() >= num_inputs);
    assert!(cs.num_inputs() == num_inputs);
    assert!(cs.num_constraints() == num_constraints);
    assert!(cs.is_satisfied(&primary_input, &auxiliary_input));

    leave_block("Call to generate_r1cs_example_with_field_input", false);

    return r1cs_example::<FieldT>::new(cs, primary_input, auxiliary_input);
}

pub fn generate_r1cs_example_with_binary_input<FieldT: FieldTConfig>(
    num_constraints: usize,
    num_inputs: usize,
) -> r1cs_example<FieldT> {
    enter_block("Call to generate_r1cs_example_with_binary_input", false);

    assert!(num_inputs >= 1);

    let mut cs = r1cs_constraint_system::<FieldT>::default();
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = num_constraints; /* we will add one auxiliary variable per constraint */
    let mut full_variable_assignment = r1cs_variable_assignment::<FieldT>::default();
    for i in 0..num_inputs {
        full_variable_assignment.push(FieldT::from(rand::random::<i64>() % 2));
    }

    let mut lastvar = num_inputs - 1;
    for i in 0..num_constraints {
        lastvar += 1;
        let u = (if i == 0 {
            rand::random::<usize>() % num_inputs
        } else {
            rand::random::<usize>() % i
        });
        let v = (if i == 0 {
            rand::random::<usize>() % num_inputs
        } else {
            rand::random::<usize>() % i
        });

        /* chose two random bits and XOR them together:
           res = u + v - 2 * u * v
           2 * u * v = u + v - res
        */
        let (mut A, mut B, mut C) = (
            linear_combination::<FieldT>::default(),
            linear_combination::<FieldT>::default(),
            linear_combination::<FieldT>::default(),
        );
        A.add_term(u + 1, 2);
        B.add_term(v + 1, 1);
        if u == v {
            C.add_term(u + 1, 2);
        } else {
            C.add_term(u + 1, 1);
            C.add_term(v + 1, 1);
        }
        C.add_term_with_field(lastvar + 1, -FieldT::one());

        cs.add_constraint(r1cs_constraint::<FieldT>::new(A, B, C));
        full_variable_assignment.push(
            full_variable_assignment[u].clone() + full_variable_assignment[v].clone()
                - full_variable_assignment[u].clone() * full_variable_assignment[v].clone()
                - full_variable_assignment[u].clone() * full_variable_assignment[v].clone(),
        );
    }

    /* split variable assignment */
    let primary_input = full_variable_assignment[..num_inputs].to_vec();
    let auxiliary_input = full_variable_assignment[num_inputs..].to_vec();

    /* sanity checks */
    assert!(cs.num_variables() == full_variable_assignment.len());
    assert!(cs.num_variables() >= num_inputs);
    assert!(cs.num_inputs() == num_inputs);
    assert!(cs.num_constraints() == num_constraints);
    assert!(cs.is_satisfied(&primary_input, &auxiliary_input));

    leave_block("Call to generate_r1cs_example_with_binary_input", false);

    return r1cs_example::<FieldT>::new(cs, primary_input, auxiliary_input);
}

//#endif // R1CS_EXAMPLES_TCC
