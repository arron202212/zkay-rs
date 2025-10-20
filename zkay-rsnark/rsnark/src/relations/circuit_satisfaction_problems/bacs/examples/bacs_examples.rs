/** @file
 *****************************************************************************

 Declaration of interfaces for a BACS example, as well as functions to sample
 BACS examples with prescribed parameters (according to some distribution).

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_EXAMPLES_HPP_
// #define BACS_EXAMPLES_HPP_

use crate::relations::circuit_satisfaction_problems/bacs/bacs;



/**
 * A BACS example comprises a BACS circuit, BACS primary input, and BACS auxiliary input.
 */
template<typename FieldT>
struct bacs_example {

    bacs_circuit<FieldT> circuit;
    bacs_primary_input<FieldT> primary_input;
    bacs_auxiliary_input<FieldT> auxiliary_input;

    bacs_example<FieldT>() = default;
    bacs_example<FieldT>(const bacs_example<FieldT> &other) = default;
    bacs_example<FieldT>(const bacs_circuit<FieldT> &circuit,
                         const bacs_primary_input<FieldT> &primary_input,
                         const bacs_auxiliary_input<FieldT> &auxiliary_input) :
        circuit(circuit),
        primary_input(primary_input),
        auxiliary_input(auxiliary_input)
    {}

    bacs_example<FieldT>(bacs_circuit<FieldT> &&circuit,
                         bacs_primary_input<FieldT> &&primary_input,
                         bacs_auxiliary_input<FieldT> &&auxiliary_input) :
        circuit((circuit)),
        primary_input((primary_input)),
        auxiliary_input((auxiliary_input))
    {}
};

/**
 * Generate a BACS example such that:
 * - the primary input has size primary_input_size;
 * - the auxiliary input has size auxiliary_input_size;
 * - the circuit has num_gates gates;
 * - the circuit has num_outputs (<= num_gates) output gates.
 *
 * This is done by first selecting primary and auxiliary inputs uniformly at random, and then for each gate:
 * - selecting random left and right wires from primary inputs, auxiliary inputs, and outputs of previous gates,
 * - selecting random linear combinations for left and right wires, consisting of 1, 2, 3 or 4 terms each, with random coefficients,
 * - if the gate is an output gate, then adding a random non-output wire to either left or right linear combination, with appropriate coefficient, so that the linear combination evaluates to 0.
 */
template<typename FieldT>
bacs_example<FieldT> generate_bacs_example(const size_t primary_input_size,
                                           const size_t auxiliary_input_size,
                                           const size_t num_gates,
                                           const size_t num_outputs);



use crate::relations::circuit_satisfaction_problems/bacs/examples/bacs_examples;

//#endif // BACS_EXAMPLES_HPP_
/** @file
 *****************************************************************************

 Implementation of functions to sample BACS examples with prescribed parameters
 (according to some distribution).

 See bacs_examples.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BACS_EXAMPLES_TCC_
// #define BACS_EXAMPLES_TCC_

use  <cassert>

use ffec::common::utils;



template<typename FieldT>
linear_combination<FieldT> random_linear_combination(const size_t num_variables)
{
    const size_t terms = 1 + (std::rand() % 3);
    linear_combination<FieldT> result;

    for i in 0..terms
    {
        const FieldT coeff = FieldT(std::rand()); // TODO: replace with FieldT::random_element(), when it becomes faster...
        result = result + coeff * variable<FieldT>(std::rand() % (num_variables + 1));
    }

    return result;
}

template<typename FieldT>
bacs_example<FieldT> generate_bacs_example(const size_t primary_input_size,
                                           const size_t auxiliary_input_size,
                                           const size_t num_gates,
                                           const size_t num_outputs)
{
    bacs_example<FieldT> example;
    for i in 0..primary_input_size
    {
        example.primary_input.push(FieldT::random_element());
    }

    for i in 0..auxiliary_input_size
    {
        example.auxiliary_input.push(FieldT::random_element());
    }

    example.circuit.primary_input_size = primary_input_size;
    example.circuit.auxiliary_input_size = auxiliary_input_size;

    bacs_variable_assignment<FieldT> all_vals;
    all_vals.insert(all_vals.end(), example.primary_input.begin(), example.primary_input.end());
    all_vals.insert(all_vals.end(), example.auxiliary_input.begin(), example.auxiliary_input.end());

    for i in 0..num_gates
    {
        const size_t num_variables = primary_input_size + auxiliary_input_size + i;
        bacs_gate<FieldT> gate;
        gate.lhs = random_linear_combination<FieldT>(num_variables);
        gate.rhs = random_linear_combination<FieldT>(num_variables);
        gate.output = variable<FieldT>(num_variables+1);

        if i >= num_gates - num_outputs
        {
            /* make gate a circuit output and fix */
            gate.is_circuit_output = true;
            const var_index_t var_idx = std::rand() % (1 + primary_input_size + std::min(num_gates-num_outputs, i));
            const FieldT var_val = if var_idx == 0 {FieldT::one()} else{all_vals[var_idx-1]};

            if std::rand() % 2 == 0
            {
                const FieldT lhs_val = gate.lhs.evaluate(all_vals);
                const FieldT coeff = -(lhs_val * var_val.inverse());
                gate.lhs = gate.lhs + coeff * variable<FieldT>(var_idx);
            }
            else
            {
                const FieldT rhs_val = gate.rhs.evaluate(all_vals);
                const FieldT coeff = -(rhs_val * var_val.inverse());
                gate.rhs = gate.rhs + coeff * variable<FieldT>(var_idx);
            }

            assert!(gate.evaluate(all_vals).is_zero());
        }
        else
        {
            gate.is_circuit_output = false;
        }

        example.circuit.add_gate(gate);
        all_vals.push(gate.evaluate(all_vals));
    }

    assert!(example.circuit.is_satisfied(example.primary_input, example.auxiliary_input));

    return example;
}



//#endif // BACS_EXAMPLES_TCC
