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
use crate::relations::circuit_satisfaction_problems::tbcs::tbcs::{
    num_tbcs_gate_types, tbcs_auxiliary_input, tbcs_circuit, tbcs_gate, tbcs_gate_type,
    tbcs_primary_input, tbcs_variable_assignment,
};
use num_enum::FromPrimitive;
/**
 * A TBCS example comprises a TBCS circuit, TBCS primary input, and TBCS auxiliary input.
 */
#[derive(Default)]
pub struct tbcs_example {
    pub circuit: tbcs_circuit,
    pub primary_input: tbcs_primary_input,
    pub auxiliary_input: tbcs_auxiliary_input,
}
impl tbcs_example {
    // tbcs_example() = default;
    // tbcs_example(other:&tbcs_example) = default;
    pub fn new(
        circuit: tbcs_circuit,
        primary_input: tbcs_primary_input,
        auxiliary_input: tbcs_auxiliary_input,
    ) -> Self {
        Self {
            circuit,
            primary_input,
            auxiliary_input,
        }
    }
}

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
// tbcs_example generate_tbcs_example(primary_input_size:usize,
//                                    auxiliary_input_size:usize,
//                                    num_gates:usize,
//                                    num_outputs:usize);

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
// use  <cassert>
use ffec::common::utils;

// use crate::relations::circuit_satisfaction_problems/tbcs/examples/tbcs_examples;

pub fn generate_tbcs_example(
    primary_input_size: usize,
    auxiliary_input_size: usize,
    num_gates: usize,
    num_outputs: usize,
) -> tbcs_example {
    let mut example = tbcs_example::default();
    for i in 0..primary_input_size {
        example.primary_input.push(rand::random::<usize>() % 2 != 0);
    }

    for i in 0..auxiliary_input_size {
        example
            .auxiliary_input
            .push(rand::random::<usize>() % 2 != 0);
    }

    example.circuit.primary_input_size = primary_input_size;
    example.circuit.auxiliary_input_size = auxiliary_input_size;

    let mut all_vals = tbcs_variable_assignment::new();
    all_vals.extend(&example.primary_input);
    all_vals.extend(&example.auxiliary_input);

    for i in 0..num_gates {
        let num_variables = primary_input_size + auxiliary_input_size + i;
        let mut gate = tbcs_gate::default();
        gate.left_wire = rand::random::<usize>() % (num_variables + 1);
        gate.right_wire = rand::random::<usize>() % (num_variables + 1);
        gate.output = num_variables + 1;

        if i >= num_gates - num_outputs {
            /* make gate a circuit output and fix */
            loop {
                gate.types = tbcs_gate_type::from(rand::random::<u8>() % num_tbcs_gate_types);
                if !gate.evaluate(&all_vals) {
                    break;
                }
            }

            gate.is_circuit_output = true;
        } else {
            gate.types = tbcs_gate_type::from(rand::random::<u8>() % num_tbcs_gate_types);
            gate.is_circuit_output = false;
        }

        example.circuit.add_gate(gate.clone());
        all_vals.push(gate.evaluate(&all_vals));
    }

    assert!(
        example
            .circuit
            .is_satisfied(&example.primary_input, &example.auxiliary_input)
    );

    return example;
}
