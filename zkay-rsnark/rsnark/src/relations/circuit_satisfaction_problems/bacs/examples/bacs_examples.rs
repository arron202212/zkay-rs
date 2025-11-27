use crate::relations::FieldTConfig;
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_gate, bacs_primary_input, bacs_variable_assignment,
};
use crate::relations::variable::SubLinearCombinationConfig;
use crate::relations::variable::SubVariableConfig;
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
use crate::relations::variable::{linear_combination, variable};
/**
 * A BACS example comprises a BACS circuit, BACS primary input, and BACS auxiliary input.
 */
#[derive(Default)]
pub struct bacs_example<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    circuit: bacs_circuit<FieldT, SV, SLC>,
    primary_input: bacs_primary_input<FieldT>,
    auxiliary_input: bacs_auxiliary_input<FieldT>,
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    bacs_example<FieldT, SV, SLC>
{
    // bacs_example<FieldT>() = default;
    // bacs_example<FieldT>(other:&bacs_example<FieldT>) = default;
    pub fn new(
        circuit: bacs_circuit<FieldT, SV, SLC>,
        primary_input: bacs_primary_input<FieldT>,
        auxiliary_input: bacs_auxiliary_input<FieldT>,
    ) -> Self {
        Self {
            circuit,
            primary_input,
            auxiliary_input,
        }
    }
}

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

// bacs_example<FieldT> generate_bacs_example(primary_input_size:usize,
//                                            auxiliary_input_size:usize,
//                                            num_gates:usize,
//                                            num_outputs:usize);

// use crate::relations::circuit_satisfaction_problems/bacs/examples/bacs_examples;

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

// use  <cassert>
use ffec::common::utils;

pub fn random_linear_combination<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    num_variables: usize,
) -> linear_combination<FieldT, SV, SLC> {
    let terms = 1i32 + (rand::random::<i32>() % 3);
    let mut result = linear_combination::<FieldT, SV, SLC>::from(0);

    for i in 0..terms {
        let coeff = FieldT::random_element(); //FieldT(rand::random()); // TODO: replace with FieldT::random_element(), when it becomes faster...
        result = result
            + variable::<FieldT, SV>::from(rand::random::<usize>() % (num_variables + 1))
                * coeff.clone();
    }

    return result;
}

pub fn generate_bacs_example<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    primary_input_size: usize,
    auxiliary_input_size: usize,
    num_gates: usize,
    num_outputs: usize,
) -> bacs_example<FieldT, SV, SLC> {
    let mut example = bacs_example::<FieldT, SV, SLC>::default();
    for i in 0..primary_input_size {
        example.primary_input.push(FieldT::random_element());
    }

    for i in 0..auxiliary_input_size {
        example.auxiliary_input.push(FieldT::random_element());
    }

    example.circuit.primary_input_size = primary_input_size;
    example.circuit.auxiliary_input_size = auxiliary_input_size;

    let mut all_vals = bacs_variable_assignment::<FieldT>::new();
    all_vals.extend(example.primary_input.clone());
    all_vals.extend(example.auxiliary_input.clone());

    for i in 0..num_gates {
        let num_variables = primary_input_size + auxiliary_input_size + i;
        let mut gate = bacs_gate::default();
        gate.lhs = random_linear_combination::<FieldT, SV, SLC>(num_variables);
        gate.rhs = random_linear_combination::<FieldT, SV, SLC>(num_variables);
        gate.output = variable::<FieldT, SV>::new(num_variables + 1, SV::default());

        if i >= num_gates - num_outputs {
            /* make gate a circuit output and fix */
            gate.is_circuit_output = true;
            let var_idx = rand::random::<usize>()
                % (1 + primary_input_size + std::cmp::min(num_gates - num_outputs, i));
            let var_val = if var_idx == 0 {
                FieldT::one()
            } else {
                all_vals[var_idx - 1].clone()
            };

            if rand::random::<i32>() % 2 == 0 {
                let lhs_val = gate.lhs.evaluate(&all_vals);
                let coeff: FieldT = -(lhs_val * var_val.inverse());
                gate.lhs =
                    gate.lhs.clone() + variable::<FieldT, SV>::new(var_idx, SV::default()) * coeff;
            } else {
                let rhs_val = gate.rhs.evaluate(&all_vals);
                let coeff = -(rhs_val * var_val.inverse());
                gate.rhs =
                    gate.rhs.clone() + variable::<FieldT, SV>::new(var_idx, SV::default()) * coeff;
            }

            assert!(gate.evaluate(&all_vals).is_zero());
        } else {
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

//#endif // BACS_EXAMPLES_TCC
