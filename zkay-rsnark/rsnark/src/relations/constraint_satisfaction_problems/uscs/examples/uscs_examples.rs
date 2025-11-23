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

use crate::relations::constraint_satisfaction_problems/uscs/uscs;



/**
 * A USCS example comprises a USCS constraint system, USCS input, and USCS witness.
 */

struct uscs_example {
constraint_system:    uscs_constraint_system<FieldT>,
primary_input:    uscs_primary_input<FieldT>,
auxiliary_input:    uscs_auxiliary_input<FieldT>,
}

    // uscs_example<FieldT>() = default;
    // uscs_example<FieldT>(other:&uscs_example<FieldT>) = default;
    uscs_example<FieldT>(constraint_system:&uscs_constraint_system<FieldT>,
                         primary_input:&uscs_primary_input<FieldT>,
                         auxiliary_input:&uscs_auxiliary_input<FieldT>)->Self
       constraint_system,
       primary_input,
        auxiliary_input(auxiliary_input)
    {};
    uscs_example<FieldT>(uscs_constraint_system<FieldT> &&constraint_system,
                         uscs_primary_input<FieldT> &&primary_input,
                         uscs_auxiliary_input<FieldT> &&auxiliary_input)->Self
        constraint_system((constraint_system)),
        primary_input((primary_input)),
        auxiliary_input((auxiliary_input))
    {};
};

/**
 * Generate a USCS example such that:
 * - the number of constraints of the USCS constraint system is num_constraints;
 * - the number of variables of the USCS constraint system is (approximately) num_constraints;
 * - the number of inputs of the USCS constraint system is num_inputs;
 * - the USCS input consists of ``full'' field elements (typically require the whole log|Field| bits to represent).
 */

uscs_example<FieldT> generate_uscs_example_with_field_input(num_constraints:usize,
                                                            num_inputs:usize);

/**
 * Generate a USCS example such that:
 * - the number of constraints of the USCS constraint system is num_constraints;
 * - the number of variables of the USCS constraint system is (approximately) num_constraints;
 * - the number of inputs of the USCS constraint system is num_inputs;
 * - the USCS input consists of binary values (as opposed to ``full'' field elements).
 */

uscs_example<FieldT> generate_uscs_example_with_binary_input(num_constraints:usize,
                                                             num_inputs:usize);



use crate::relations::constraint_satisfaction_problems/uscs/examples/uscs_examples;

//#endif // USCS_EXAMPLES_HPP_
/** @file
 *****************************************************************************

 Implementation of functions to sample USCS examples with prescribed parameters
 (according to some distribution).

 See uscs_examples.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_EXAMPLES_TCC_
// #define USCS_EXAMPLES_TCC_

use  <cassert>

use ffec::common::utils;




uscs_example<FieldT> generate_uscs_example_with_field_input(num_constraints:usize,
                                                            num_inputs:usize)
{
    ffec::enter_block("Call to generate_uscs_example_with_field_input");

    assert!(num_inputs >= 1);
    assert!(num_constraints >= num_inputs);

    uscs_constraint_system<FieldT> cs;
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = num_constraints - num_inputs;

    uscs_variable_assignment<FieldT> full_variable_assignment;
    for i in 0..num_constraints
    {
        full_variable_assignment.push(FieldT(rand::random()));
    }

    for i in 0..num_constraints
    {
        usize x, y, z;

        do
        {
            x = rand::random() % num_constraints;
            y = rand::random() % num_constraints;
            z = rand::random() % num_constraints;
        } while (x == z || y == z);

        let x_coeff= FieldT(rand::random());
        let y_coeff= FieldT(rand::random());
        let val= if rand::random() % 2 == 0 {FieldT::one()} else{-FieldT::one()};
        let z_coeff= (val - x_coeff * full_variable_assignment[x] - y_coeff * full_variable_assignment[y]) * full_variable_assignment[z].inverse();

        uscs_constraint<FieldT> constr;
        constr.add_term(x+1, x_coeff);
        constr.add_term(y+1, y_coeff);
        constr.add_term(z+1, z_coeff);

        cs.add_constraint(constr);
    }

    /* split variable assignment */
    uscs_primary_input<FieldT> primary_input(full_variable_assignment.begin(), full_variable_assignment.begin() + num_inputs);
    uscs_primary_input<FieldT> auxiliary_input(full_variable_assignment.begin() + num_inputs, full_variable_assignment.end());

    /* sanity checks */
    assert!(cs.num_variables() == full_variable_assignment.len());
    assert!(cs.num_variables() >= num_inputs);
    assert!(cs.num_inputs() == num_inputs);
    assert!(cs.num_constraints() == num_constraints);
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    ffec::leave_block("Call to generate_uscs_example_with_field_input");

    return uscs_example<FieldT>((cs), (primary_input), (auxiliary_input));
}


uscs_example<FieldT> generate_uscs_example_with_binary_input(num_constraints:usize,
                                                             num_inputs:usize)
{
    ffec::enter_block("Call to generate_uscs_example_with_binary_input");

    assert!(num_inputs >= 1);

    uscs_constraint_system<FieldT> cs;
    cs.primary_input_size = num_inputs;
    cs.auxiliary_input_size = num_constraints;

    uscs_variable_assignment<FieldT> full_variable_assignment;
    for i in 0..num_inputs
    {
        full_variable_assignment.push_back(FieldT(rand::random() % 2));
    }

    usize lastvar = num_inputs-1;
    for i in 0..num_constraints
    {
        lastvar+=1;

        /* chose two random bits and XOR them together */
        let u = if i == 0 {rand::random() % num_inputs} else{rand::random() % i};
        let v = if i == 0 {rand::random() % num_inputs} else{rand::random() % i};

        uscs_constraint<FieldT> constr;
        constr.add_term(u+1, 1);
        constr.add_term(v+1, 1);
        constr.add_term(lastvar+1, 1);
        constr.add_term(0,-FieldT::one()); // shift constant term (which is 0) by 1

        cs.add_constraint(constr);
        full_variable_assignment.push_back(full_variable_assignment[u] + full_variable_assignment[v] - full_variable_assignment[u] * full_variable_assignment[v] - full_variable_assignment[u] * full_variable_assignment[v]);
    }

    /* split variable assignment */
    uscs_primary_input<FieldT> primary_input(full_variable_assignment.begin(), full_variable_assignment.begin() + num_inputs);
    uscs_primary_input<FieldT> auxiliary_input(full_variable_assignment.begin() + num_inputs, full_variable_assignment.end());

    /* sanity checks */
    assert!(cs.num_variables() == full_variable_assignment.len());
    assert!(cs.num_variables() >= num_inputs);
    assert!(cs.num_inputs() == num_inputs);
    assert!(cs.num_constraints() == num_constraints);
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    ffec::leave_block("Call to generate_uscs_example_with_binary_input");

    return uscs_example<FieldT>((cs), (primary_input), (auxiliary_input));
}


//#endif // USCS_EXAMPLES_TCC
