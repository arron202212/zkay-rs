/** @file
*****************************************************************************

Declaration of interfaces for a BACS-to-R1CS reduction, that is, constructing
a R1CS ("Rank-1 Constraint System") from a BACS ("Bilinear Arithmetic Circuit Satisfiability").

The reduction is straightforward: each bilinear gate gives rises to a
corresponding R1CS constraint that enforces correct computation of the gate;
also, each output gives rise to a corresponding R1CS constraint that enforces
that the output is zero.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef BACS_TO_R1CS_HPP_
// #define BACS_TO_R1CS_HPP_
use crate::relations::FieldTConfig;
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_primary_input,
};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint, r1cs_constraint_system, r1cs_variable_assignment,
};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination,
};
use ffec::common::profiling::{enter_block, leave_block};
use ffec::common::utils::FMT;
// /**
//  * Instance map for the BACS-to-R1CS reduction.
//  */
// r1cs_constraint_system<FieldT> bacs_to_r1cs_instance_map(circuit:&bacs_circuit<FieldT,SV,SLC>);

// /**
//  * Witness map for the BACS-to-R1CS reduction.
//  */
// r1cs_variable_assignment<FieldT> bacs_to_r1cs_witness_map(circuit:&bacs_circuit<FieldT,SV,SLC>,
//                                                                primary_input:&bacs_primary_input<FieldT>,
//                                                                auxiliary_input:&bacs_auxiliary_input<FieldT>);

// use crate::reductions::bacs_to_r1cs::bacs_to_r1cs;

//#endif // BACS_TO_R1CS_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a BACS-to-R1CS reduction.

See bacs_to_r1cs.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef BACS_TO_R1CS_TCC_
// #define BACS_TO_R1CS_TCC_

// use crate::relations::circuit_satisfaction_problems::bacs::bacs;
// use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;

pub fn bacs_to_r1cs_instance_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    circuit: &bacs_circuit<FieldT, SV, SLC>,
) -> r1cs_constraint_system<FieldT, SV, SLC> {
    enter_block("Call to bacs_to_r1cs_instance_map", false);
    assert!(circuit.is_valid());
    let mut result = r1cs_constraint_system::<FieldT, SV, SLC>::default();

    // #ifdef DEBUG
    result.variable_annotations = circuit.variable_annotations.clone();
    //#endif

    result.primary_input_size = circuit.primary_input_size;
    result.auxiliary_input_size = circuit.auxiliary_input_size + circuit.gates.len();

    for g in &circuit.gates {
        result
            .constraints
            .push(r1cs_constraint::<FieldT, SV, SLC>::new(
                g.lhs.clone(),
                g.rhs.clone(),
                linear_combination::<FieldT, SV, SLC>::from(g.output.clone()),
            ));
        // #ifdef DEBUG
        if let Some(v) = circuit.gate_annotations.get(&g.output.index) {
            result
                .constraint_annotations
                .insert(result.constraints.len() - 1, v.clone());
        }
        //#endif
    }

    for g in &circuit.gates {
        if g.is_circuit_output {
            result
                .constraints
                .push(r1cs_constraint::<FieldT, SV, SLC>::new(
                    linear_combination::<FieldT, SV, SLC>::from(1),
                    linear_combination::<FieldT, SV, SLC>::from(g.output.clone()),
                    linear_combination::<FieldT, SV, SLC>::from(0),
                ));

            // #ifdef DEBUG
            result.constraint_annotations.insert(
                result.constraints.len() - 1,
                format!("output_{}_is_circuit_output", g.output.index),
            );
            //#endif
        }
    }

    leave_block("Call to bacs_to_r1cs_instance_map", false);

    return result;
}

pub fn bacs_to_r1cs_witness_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    circuit: &bacs_circuit<FieldT, SV, SLC>,
    primary_input: &bacs_primary_input<FieldT>,
    auxiliary_input: &bacs_auxiliary_input<FieldT>,
) -> r1cs_variable_assignment<FieldT> {
    enter_block("Call to bacs_to_r1cs_witness_map", false);
    let result = circuit.get_all_wires(primary_input, auxiliary_input);
    leave_block("Call to bacs_to_r1cs_witness_map", false);

    return result;
}

//#endif // BACS_TO_R1CS_TCC_
