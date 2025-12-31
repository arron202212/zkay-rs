use crate::relations::circuit_satisfaction_problems::tbcs::tbcs::tbcs_gate_type;
/** @file
*****************************************************************************

Declaration of interfaces for a TBCS-to-USCS reduction, that is, constructing
a USCS ("Unitary-Square Constraint System") from a TBCS ("Two-input Boolean Circuit Satisfiability").

The reduction is straightforward: each non-output wire is mapped to a
corresponding USCS constraint that enforces the wire to carry a boolean value;
each 2-input boolean gate is mapped to a corresponding USCS constraint that
enforces correct computation of the gate; each output wire is mapped to a
corresponding USCS constraint that enforces that the output is zero.

The mapping of a gate to a USCS constraint is due to \[GOS12].

References:

\[GOS12]:
"New techniques for noninteractive zero-knowledge",
Jens Groth, Rafail Ostrovsky, Amit Sahai
JACM 2012,
<http://www0.cs.ucl.ac.uk/staff/J.Groth/NIZKJournal.pdf>

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef TBCS_TO_USCS_HPP_
// #define TBCS_TO_USCS_HPP_
use crate::relations::circuit_satisfaction_problems::tbcs::tbcs::{
    tbcs_auxiliary_input, tbcs_circuit, tbcs_primary_input,
};
use crate::relations::constraint_satisfaction_problems::uscs::uscs::{
    uscs_constraint_system, uscs_variable_assignment,
};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination, variable,
};
use ffec::FieldTConfig;
use ffec::common::utils::FMT;
use ffec::field_utils::field_utils::convert_bit_vector_to_field_element_vector;
// /**
//  * Instance map for the TBCS-to-USCS reduction.
//  */
// uscs_constraint_system<FieldT> tbcs_to_uscs_instance_map(circuit:&tbcs_circuit);

// /**
//  * Witness map for the TBCS-to-USCS reduction.
//  */
// uscs_variable_assignment<FieldT> tbcs_to_uscs_witness_map(circuit:&tbcs_circuit,
//                                                                primary_input:&tbcs_primary_input,
//                                                                auxiliary_input:&tbcs_auxiliary_input);

pub fn tbcs_to_uscs_instance_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    circuit: &tbcs_circuit,
) -> uscs_constraint_system<FieldT, SV, SLC> {
    assert!(circuit.is_valid());
    let mut result = uscs_constraint_system::<FieldT, SV, SLC>::default();

    // #ifdef DEBUG
    result.variable_annotations = circuit.variable_annotations.clone();
    //#endif

    result.primary_input_size = circuit.primary_input_size;
    result.auxiliary_input_size = circuit.auxiliary_input_size + circuit.gates.len();

    for g in &circuit.gates {
        let x = linear_combination::<FieldT, SV, SLC>::from(variable::<FieldT, SV>::new(
            g.left_wire,
            SV::default(),
        ));
        let y = linear_combination::<FieldT, SV, SLC>::from(variable::<FieldT, SV>::new(
            g.right_wire,
            SV::default(),
        ));
        let z = linear_combination::<FieldT, SV, SLC>::from(variable::<FieldT, SV>::new(
            g.output,
            SV::default(),
        ));

        // #ifdef DEBUG
        // auto it = circuit.gate_annotations.find(g.output);
        // let annotation= if it != circuit.gate_annotations.end() {it.1} else{FMT("", "compute_wire_{}", g.output)};
        // #else
        let annotation = "";
        //#endif

        match g.types {
            tbcs_gate_type::TBCS_GATE_CONSTANT_0 => {
                /* Truth table (00, 01, 10, 11): (0, 0, 0, 0)
                0 * x + FieldT::from(0) * y + FieldT::from(1) * z + FieldT::from(1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(0)
                        + y * FieldT::from(0)
                        + z * FieldT::from(1)
                        + FieldT::from(1),
                    annotation,
                );
            }
            tbcs_gate_type::TBCS_GATE_AND => {
                /* Truth table (00, 01, 10, 11): (0, 0, 0, 1)
                -2 * x + FieldT::from(-2) * y + FieldT::from(4) * z + FieldT::from(1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-2)
                        + y * FieldT::from(-2)
                        + z * FieldT::from(4)
                        + FieldT::from(1),
                    annotation,
                );
            }
            tbcs_gate_type::TBCS_GATE_X_AND_NOT_Y => {
                /* Truth table (00, 01, 10, 11): (0, 0, 1, 0)
                -2 * x + FieldT::from(2) * y + FieldT::from(4) * z + FieldT::from(-1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-2)
                        + y * FieldT::from(2)
                        + z * FieldT::from(4)
                        + FieldT::from(-1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_X => {
                /* Truth table (00, 01, 10, 11): (0, 0, 1, 1)
                -1 * x + FieldT::from(0) * y + FieldT::from(1) * z + FieldT::from(1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-1)
                        + y * FieldT::from(0)
                        + z * FieldT::from(1)
                        + FieldT::from(1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_NOT_X_AND_Y => {
                /* Truth table (00, 01, 10, 11): (0, 1, 0, 0)
                2 * x + FieldT::from(-2) * y + FieldT::from(4) * z + FieldT::from(-1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(2)
                        + y * FieldT::from(-2)
                        + z * FieldT::from(4)
                        + FieldT::from(-1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_Y => {
                /* Truth table (00, 01, 10, 11): (0, 1, 0, 1)
                0 * x + FieldT::from(1) * y + FieldT::from(1) * z + FieldT::from(-1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(0)
                        + y * FieldT::from(1)
                        + z * FieldT::from(1)
                        + FieldT::from(-1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_XOR => {
                /* Truth table (00, 01, 10, 11): (0, 1, 1, 0)
                1 * x + FieldT::from(1) * y + FieldT::from(1) * z + FieldT::from(-1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(1)
                        + y * FieldT::from(1)
                        + z * FieldT::from(1)
                        + FieldT::from(-1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_OR => {
                /* Truth table (00, 01, 10, 11): (0, 1, 1, 1)
                -2 * x + FieldT::from(-2) * y + FieldT::from(4) * z + FieldT::from(-1) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-2)
                        + y * FieldT::from(-2)
                        + z * FieldT::from(4)
                        + FieldT::from(-1),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_NOR => {
                /* Truth table (00, 01, 10, 11): (1, 0, 0, 0)
                2 * x + FieldT::from(2) * y + FieldT::from(4) * z + FieldT::from(-3) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(2)
                        + y * FieldT::from(2)
                        + z * FieldT::from(4)
                        + FieldT::from(-3),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_EQUIVALENCE => {
                /* Truth table (00, 01, 10, 11): (1, 0, 0, 1)
                1 * x + FieldT::from(1) * y + FieldT::from(1) * z + FieldT::from(-2) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(1)
                        + y * FieldT::from(1)
                        + z * FieldT::from(1)
                        + FieldT::from(-2),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_NOT_Y => {
                /* Truth table (00, 01, 10, 11): (1, 0, 1, 0)
                0 * x + FieldT::from(-1) * y + FieldT::from(1) * z + FieldT::from(0) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(0)
                        + y * FieldT::from(-1)
                        + z * FieldT::from(1)
                        + FieldT::from(0),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_IF_Y_THEN_X => {
                /* Truth table (00, 01, 10, 11): (1, 0, 1, 1)
                -2 * x + FieldT::from(2) * y + FieldT::from(4) * z + FieldT::from(-3) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-2)
                        + y * FieldT::from(2)
                        + z * FieldT::from(4)
                        + FieldT::from(-3),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_NOT_X => {
                /* Truth table (00, 01, 10, 11): (1, 1, 0, 0)
                -1 * x + FieldT::from(0) * y + FieldT::from(1) * z + FieldT::from(0) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(-1)
                        + y * FieldT::from(0)
                        + z * FieldT::from(1)
                        + FieldT::from(0),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_IF_X_THEN_Y => {
                /* Truth table (00, 01, 10, 11): (1, 1, 0, 1)
                2 * x + FieldT::from(-2) * y + FieldT::from(4) * z + FieldT::from(-3) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(2)
                        + y * FieldT::from(-2)
                        + z * FieldT::from(4)
                        + FieldT::from(-3),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_NAND => {
                /* Truth table (00, 01, 10, 11): (1, 1, 1, 0)
                2 * x + FieldT::from(2) * y + FieldT::from(4) * z + FieldT::from(-5) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(2)
                        + y * FieldT::from(2)
                        + z * FieldT::from(4)
                        + FieldT::from(-5),
                    annotation,
                );
            }

            tbcs_gate_type::TBCS_GATE_CONSTANT_1 => {
                /* Truth table (00, 01, 10, 11): (1, 1, 1, 1)
                0 * x + FieldT::from(0) * y + FieldT::from(1) * z + FieldT::from(0) \in {-1, 1} */
                result.add_constraint(
                    x * FieldT::from(0)
                        + y * FieldT::from(0)
                        + z * FieldT::from(1)
                        + FieldT::from(0),
                    annotation,
                );
            }

            _ => {
                panic!("0");
            }
        }
    }

    for i in 0..circuit.primary_input_size + circuit.auxiliary_input_size + circuit.gates.len() {
        /* require that 2 * wire - 1 \in {-1,1}, that is wire \in {0,1} */
        result.add_constraint(
            linear_combination::<FieldT, SV, SLC>::from(
                variable::<FieldT, SV>::new(i, SV::default()) * FieldT::from(2),
            ) - 1i64,
            &format!("wire_{}", i),
        );
    }

    for g in &circuit.gates {
        if g.is_circuit_output {
            /* require that output + FieldT::from(1) \in {-1,1}, this together with output binary (above) enforces output = 0 */
            result.add_constraint(
                variable::<FieldT, SV>::new(g.output, SV::default()) + FieldT::from(1i64).into(),
                &format!("output_{}", g.output),
            );
        }
    }

    return result;
}

pub fn tbcs_to_uscs_witness_map<FieldT: FieldTConfig>(
    circuit: &tbcs_circuit,
    primary_input: &tbcs_primary_input,
    auxiliary_input: &tbcs_auxiliary_input,
) -> uscs_variable_assignment<FieldT> {
    let all_wires = circuit.get_all_wires(primary_input, auxiliary_input);
    let result = convert_bit_vector_to_field_element_vector::<FieldT>(&all_wires);
    return result;
}

//#endif // TBCS_TO_USCS_TCC_
