/** @file
 *****************************************************************************

 Declaration of interfaces for a TBCS example, as well as functions to sample
 TBCS examples with prescribed parameters (according to some distribution).

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TBCS_EXAMPLES_HPP_
// #define TBCS_EXAMPLES_HPP_

use crate::relations::circuit_satisfaction_problems/tbcs/tbcs;



/**
 * A TBCS example comprises a TBCS circuit, TBCS primary input, and TBCS auxiliary input.
 */
struct tbcs_example {

    tbcs_circuit circuit;
    tbcs_primary_input primary_input;
    tbcs_auxiliary_input auxiliary_input;

    tbcs_example() = default;
    tbcs_example(const tbcs_example &other) = default;
    tbcs_example(const tbcs_circuit &circuit,
                 const tbcs_primary_input &primary_input,
                 const tbcs_auxiliary_input &auxiliary_input) :
        circuit(circuit),
        primary_input(primary_input),
        auxiliary_input(auxiliary_input)
    {}

    tbcs_example(tbcs_circuit &&circuit,
                 tbcs_primary_input &&primary_input,
                 tbcs_auxiliary_input &&auxiliary_input) :
        circuit((circuit)),
        primary_input((primary_input)),
        auxiliary_input((auxiliary_input))
    {}
};

/**
 * Generate a TBCS example such that:
 * - the primary input has size primary_input_size;
 * - the auxiliary input has size auxiliary_input_size;
 * - the circuit has num_gates gates;
 * - the circuit has num_outputs (<= num_gates) output gates.
 *
 * This is done by first selecting primary and auxiliary inputs uniformly at random, and then for each gate:
 * - selecting random left and right wires from primary inputs, auxiliary inputs, and outputs of previous gates,
 * - selecting a gate type at random (subject to the constraint "output = 0" if this is an output gate).
 */
tbcs_example generate_tbcs_example(const size_t primary_input_size,
                                   const size_t auxiliary_input_size,
                                   const size_t num_gates,
                                   const size_t num_outputs);



//#endif // TBCS_EXAMPLES_HPP_
/** @file
 *****************************************************************************

 Implementation of functions to sample TBCS examples with prescribed parameters
 (according to some distribution).

 See tbcs_examples.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <cassert>

use ffec::common::utils;

use crate::relations::circuit_satisfaction_problems/tbcs/examples/tbcs_examples;



tbcs_example generate_tbcs_example(const size_t primary_input_size,
                                   const size_t auxiliary_input_size,
                                   const size_t num_gates,
                                   const size_t num_outputs)
{
    tbcs_example example;
    for i in 0..primary_input_size
    {
        example.primary_input.push_back(std::rand() % 2 == 0 ? false : true);
    }

    for i in 0..auxiliary_input_size
    {
        example.auxiliary_input.push_back(std::rand() % 2 == 0 ? false : true);
    }

    example.circuit.primary_input_size = primary_input_size;
    example.circuit.auxiliary_input_size = auxiliary_input_size;

    tbcs_variable_assignment all_vals;
    all_vals.insert(all_vals.end(), example.primary_input.begin(), example.primary_input.end());
    all_vals.insert(all_vals.end(), example.auxiliary_input.begin(), example.auxiliary_input.end());

    for i in 0..num_gates
    {
        const size_t num_variables = primary_input_size + auxiliary_input_size + i;
        tbcs_gate gate;
        gate.left_wire = std::rand() % (num_variables+1);
        gate.right_wire = std::rand() % (num_variables+1);
        gate.output = num_variables+1;

        if i >= num_gates - num_outputs
        {
            /* make gate a circuit output and fix */
            do
            {
                gate.type = (tbcs_gate_type)(std::rand() % num_tbcs_gate_types);
            }
            while (gate.evaluate(all_vals));

            gate.is_circuit_output = true;
        }
        else
        {
            gate.type = (tbcs_gate_type)(std::rand() % num_tbcs_gate_types);
            gate.is_circuit_output = false;
        }

        example.circuit.add_gate(gate);
        all_vals.push_back(gate.evaluate(all_vals));
    }

    assert!(example.circuit.is_satisfied(example.primary_input, example.auxiliary_input));

    return example;
}


