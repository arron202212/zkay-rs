use crate::relations::constraint_satisfaction_problems::uscs::uscs::{
    uscs_auxiliary_input, uscs_constraint, uscs_constraint_system, uscs_primary_input,
    uscs_variable_assignment,
};
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig};
/** @file

Declaration of interfaces for a USCS example, as well as functions to sample
USCS examples with prescribed parameters (according to some distribution).

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef USCS_EXAMPLES_HPP_
// #define USCS_EXAMPLES_HPP_
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block};
/**
 * A USCS example comprises a USCS constraint system, USCS input, and USCS witness.
 */

pub struct uscs_example<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    pub constraint_system: uscs_constraint_system<FieldT, SV, SLC>,
    pub primary_input: uscs_primary_input<FieldT>,
    pub auxiliary_input: uscs_auxiliary_input<FieldT>,
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    uscs_example<FieldT, SV, SLC>
{
    // uscs_example<FieldT>() = default;
    // uscs_example<FieldT>(other:&uscs_example<FieldT>) = default;
    pub fn new(
        constraint_system: uscs_constraint_system<FieldT, SV, SLC>,
        primary_input: uscs_primary_input<FieldT>,
        auxiliary_input: uscs_auxiliary_input<FieldT>,
    ) -> Self {
        Self {
            constraint_system,
            primary_input,
            auxiliary_input,
        }
    }
}

/**
 * Generate a USCS example such that:
 * - the number of constraints of the USCS constraint system is num_constraints;
 * - the number of variables of the USCS constraint system is (approximately) num_constraints;
 * - the number of inputs of the USCS constraint system is num_inputs;
 * - the USCS input consists of ``full'' field elements (typically require the whole log|Field| bits to represent).
 */

// uscs_example<FieldT> generate_uscs_example_with_field_input(num_constraints:usize,
//                                                             num_inputs:usize);

/**
 * Generate a USCS example such that:
 * - the number of constraints of the USCS constraint system is num_constraints;
 * - the number of variables of the USCS constraint system is (approximately) num_constraints;
 * - the number of inputs of the USCS constraint system is num_inputs;
 * - the USCS input consists of binary values (as opposed to ``full'' field elements).
 */
// uscs_example<FieldT> generate_uscs_example_with_binary_input(num_constraints:usize,
//                                                              num_inputs:usize);

// // use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;

// //#endif // USCS_EXAMPLES_HPP_
// /** @file
//  *****************************************************************************

//  Implementation of functions to sample USCS examples with prescribed parameters
//  (according to some distribution).

//  See uscs_examples.hpp .

//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// //#ifndef USCS_EXAMPLES_TCC_
// // #define USCS_EXAMPLES_TCC_

// use  <cassert>
use ffec::common::utils;

pub fn generate_uscs_example_with_field_input<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    num_constraints: usize,
    num_inputs: usize,
) -> uscs_example<FieldT, SV, SLC> {
    enter_block("Call to generate_uscs_example_with_field_input", false);

    assert!(num_inputs >= 1);
    assert!(num_constraints >= num_inputs);

    let mut cs = uscs_constraint_system::<FieldT, SV, SLC>::default();
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = num_constraints - num_inputs;

    let mut full_variable_assignment = uscs_variable_assignment::<FieldT>::default();
    for i in 0..num_constraints {
        full_variable_assignment.push(FieldT::from(rand::random::<usize>()));
    }

    for i in 0..num_constraints {
        let (mut x, mut y, mut z);

        loop {
            x = rand::random::<usize>() % num_constraints;
            y = rand::random::<usize>() % num_constraints;
            z = rand::random::<usize>() % num_constraints;
            if !(x == z || y == z) {
                break;
            }
        }

        let x_coeff = FieldT::from(rand::random::<usize>());
        let y_coeff = FieldT::from(rand::random::<usize>());
        let val = if rand::random::<usize>() % 2 == 0 {
            FieldT::one()
        } else {
            -FieldT::one()
        };
        let z_coeff = (val.clone()
            - x_coeff.clone() * full_variable_assignment[x].clone()
            - y_coeff.clone() * full_variable_assignment[y].clone())
            * full_variable_assignment[z].inverse();

        let mut constr = uscs_constraint::<FieldT, SV, SLC>::default();
        constr.add_term_with_field(x + 1, x_coeff);
        constr.add_term_with_field(y + 1, y_coeff);
        constr.add_term_with_field(z + 1, z_coeff);

        cs.add_constraint0(constr);
    }

    /* split variable assignment */
    //uscs_primary_input<FieldT>
    let primary_input = full_variable_assignment[..num_inputs].to_vec();
    let auxiliary_input = full_variable_assignment[num_inputs..].to_vec();

    /* sanity checks */
    assert!(cs.num_variables() == full_variable_assignment.len());
    assert!(cs.num_variables() >= num_inputs);
    assert!(cs.num_inputs() == num_inputs);
    assert!(cs.num_constraints() == num_constraints);
    assert!(cs.is_satisfied(&primary_input, &auxiliary_input));

    leave_block("Call to generate_uscs_example_with_field_input", false);

    return uscs_example::<FieldT, SV, SLC>::new(cs, primary_input, auxiliary_input);
}

pub fn generate_uscs_example_with_binary_input<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    num_constraints: usize,
    num_inputs: usize,
) -> uscs_example<FieldT, SV, SLC> {
    enter_block("Call to generate_uscs_example_with_binary_input", false);

    assert!(num_inputs >= 1);

    let mut cs = uscs_constraint_system::<FieldT, SV, SLC>::default();
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = num_constraints;

    let mut full_variable_assignment = uscs_variable_assignment::<FieldT>::default();
    for i in 0..num_inputs {
        full_variable_assignment.push(FieldT::from(rand::random::<usize>() % 2));
    }

    let mut lastvar = num_inputs - 1;
    for i in 0..num_constraints {
        lastvar += 1;

        /* chose two random bits and XOR them together */
        let u = if i == 0 {
            rand::random::<usize>() % num_inputs
        } else {
            rand::random::<usize>() % i
        };
        let v = if i == 0 {
            rand::random::<usize>() % num_inputs
        } else {
            rand::random::<usize>() % i
        };

        let mut constr = uscs_constraint::<FieldT, SV, SLC>::default();
        constr.add_term(u + 1, 1);
        constr.add_term(v + 1, 1);
        constr.add_term(lastvar + 1, 1);
        constr.add_term_with_field(0, -FieldT::one()); // shift constant term (which is 0) by 1

        cs.add_constraint0(constr);
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

    leave_block("Call to generate_uscs_example_with_binary_input", false);

    return uscs_example::<FieldT, SV, SLC>::new(cs, primary_input, auxiliary_input);
}

//#endif // USCS_EXAMPLES_TCC
