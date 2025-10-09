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

use libsnark/relations/circuit_satisfaction_problems/tbcs/tbcs;
use libsnark/relations/constraint_satisfaction_problems/uscs/uscs;



/**
 * Instance map for the TBCS-to-USCS reduction.
 */
template<typename FieldT>
uscs_constraint_system<FieldT> tbcs_to_uscs_instance_map(const tbcs_circuit &circuit);

/**
 * Witness map for the TBCS-to-USCS reduction.
 */
template<typename FieldT>
uscs_variable_assignment<FieldT> tbcs_to_uscs_witness_map(const tbcs_circuit &circuit,
                                                               const tbcs_primary_input &primary_input,
                                                               const tbcs_auxiliary_input &auxiliary_input);



use libsnark/reductions/tbcs_to_uscs/tbcs_to_uscs;

//#endif // TBCS_TO_USCS_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a TBCS-to-USCS reduction.

See tbcs_to_uscs.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef TBCS_TO_USCS_TCC_
// #define TBCS_TO_USCS_TCC_

use ffec::algebra::fields::field_utils;



template<typename FieldT>
uscs_constraint_system<FieldT> tbcs_to_uscs_instance_map(const tbcs_circuit &circuit)
{
    assert!(circuit.is_valid());
    uscs_constraint_system<FieldT> result;

// #ifdef DEBUG
    result.variable_annotations = circuit.variable_annotations;
//#endif

    result.primary_input_size = circuit.primary_input_size;
    result.auxiliary_input_size = circuit.auxiliary_input_size + circuit.gates.size();

    for (auto &g : circuit.gates)
    {
        const variable<FieldT> x(g.left_wire);
        const variable<FieldT> y(g.right_wire);
        const variable<FieldT> z(g.output);

// #ifdef DEBUG
        auto it = circuit.gate_annotations.find(g.output);
        const std::string annotation = (it != circuit.gate_annotations.end() ? it->second : FMT("", "compute_wire_{}", g.output));
#else
        const std::string annotation = "";
//#endif

        switch (g.type)
        {
        case TBCS_GATE_CONSTANT_0:
            /* Truth table (00, 01, 10, 11): (0, 0, 0, 0)
               0 * x + 0 * y + 1 * z + 1 \in {-1, 1} */
            result.add_constraint(0 * x + 0 * y + 1 * z + 1, annotation);
            break;
        case TBCS_GATE_AND:
            /* Truth table (00, 01, 10, 11): (0, 0, 0, 1)
               -2 * x + -2 * y + 4 * z + 1 \in {-1, 1} */
            result.add_constraint(-2 * x + -2 * y + 4 * z + 1, annotation);
            break;
        case TBCS_GATE_X_AND_NOT_Y:
            /* Truth table (00, 01, 10, 11): (0, 0, 1, 0)
               -2 * x + 2 * y + 4 * z + -1 \in {-1, 1} */
            result.add_constraint(-2 * x + 2 * y + 4 * z + -1, annotation);
            break;
        case TBCS_GATE_X:
            /* Truth table (00, 01, 10, 11): (0, 0, 1, 1)
               -1 * x + 0 * y + 1 * z + 1 \in {-1, 1} */
            result.add_constraint(-1 * x + 0 * y + 1 * z + 1, annotation);
            break;
        case TBCS_GATE_NOT_X_AND_Y:
            /* Truth table (00, 01, 10, 11): (0, 1, 0, 0)
               2 * x + -2 * y + 4 * z + -1 \in {-1, 1} */
            result.add_constraint(2 * x + -2 * y + 4 * z + -1, annotation);
            break;
        case TBCS_GATE_Y:
            /* Truth table (00, 01, 10, 11): (0, 1, 0, 1)
               0 * x + 1 * y + 1 * z + -1 \in {-1, 1} */
            result.add_constraint(0 * x + 1 * y + 1 * z + -1, annotation);
            break;
        case TBCS_GATE_XOR:
            /* Truth table (00, 01, 10, 11): (0, 1, 1, 0)
               1 * x + 1 * y + 1 * z + -1 \in {-1, 1} */
            result.add_constraint(1 * x + 1 * y + 1 * z + -1, annotation);
            break;
        case TBCS_GATE_OR:
            /* Truth table (00, 01, 10, 11): (0, 1, 1, 1)
               -2 * x + -2 * y + 4 * z + -1 \in {-1, 1} */
            result.add_constraint(-2 * x + -2 * y + 4 * z + -1, annotation);
            break;
        case TBCS_GATE_NOR:
            /* Truth table (00, 01, 10, 11): (1, 0, 0, 0)
               2 * x + 2 * y + 4 * z + -3 \in {-1, 1} */
            result.add_constraint(2 * x + 2 * y + 4 * z + -3, annotation);
            break;
        case TBCS_GATE_EQUIVALENCE:
            /* Truth table (00, 01, 10, 11): (1, 0, 0, 1)
               1 * x + 1 * y + 1 * z + -2 \in {-1, 1} */
            result.add_constraint(1 * x + 1 * y + 1 * z + -2, annotation);
            break;
        case TBCS_GATE_NOT_Y:
            /* Truth table (00, 01, 10, 11): (1, 0, 1, 0)
               0 * x + -1 * y + 1 * z + 0 \in {-1, 1} */
            result.add_constraint(0 * x + -1 * y + 1 * z + 0, annotation);
            break;
        case TBCS_GATE_IF_Y_THEN_X:
            /* Truth table (00, 01, 10, 11): (1, 0, 1, 1)
               -2 * x + 2 * y + 4 * z + -3 \in {-1, 1} */
            result.add_constraint(-2 * x + 2 * y + 4 * z + -3, annotation);
            break;
        case TBCS_GATE_NOT_X:
            /* Truth table (00, 01, 10, 11): (1, 1, 0, 0)
               -1 * x + 0 * y + 1 * z + 0 \in {-1, 1} */
            result.add_constraint(-1 * x + 0 * y + 1 * z + 0, annotation);
            break;
        case TBCS_GATE_IF_X_THEN_Y:
            /* Truth table (00, 01, 10, 11): (1, 1, 0, 1)
               2 * x + -2 * y + 4 * z + -3 \in {-1, 1} */
            result.add_constraint(2 * x + -2 * y + 4 * z + -3, annotation);
            break;
        case TBCS_GATE_NAND:
            /* Truth table (00, 01, 10, 11): (1, 1, 1, 0)
               2 * x + 2 * y + 4 * z + -5 \in {-1, 1} */
            result.add_constraint(2 * x + 2 * y + 4 * z + -5, annotation);
            break;
        case TBCS_GATE_CONSTANT_1:
            /* Truth table (00, 01, 10, 11): (1, 1, 1, 1)
               0 * x + 0 * y + 1 * z + 0 \in {-1, 1} */
            result.add_constraint(0 * x + 0 * y + 1 * z + 0, annotation);
            break;
        default:
            assert!(0);
        }
    }

    for (size_t i = 0; i < circuit.primary_input_size + circuit.auxiliary_input_size + circuit.gates.size(); ++i)
    {
        /* require that 2 * wire - 1 \in {-1,1}, that is wire \in {0,1} */
        result.add_constraint(2 * variable<FieldT>(i) - 1, FMT("", "wire_{}", i));
    }

    for (auto &g : circuit.gates)
    {
        if (g.is_circuit_output)
        {
            /* require that output + 1 \in {-1,1}, this together with output binary (above) enforces output = 0 */
            result.add_constraint(variable<FieldT>(g.output) + 1, FMT("", "output_{}", g.output));
        }
    }

    return result;
}

template<typename FieldT>
uscs_variable_assignment<FieldT> tbcs_to_uscs_witness_map(const tbcs_circuit &circuit,
                                                               const tbcs_primary_input &primary_input,
                                                               const tbcs_auxiliary_input &auxiliary_input)
{
    const tbcs_variable_assignment all_wires = circuit.get_all_wires(primary_input, auxiliary_input);
    const uscs_variable_assignment<FieldT> result = ffec::convert_bit_vector_to_field_element_vector<FieldT>(all_wires);
    return result;
}




//#endif // TBCS_TO_USCS_TCC_
