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

use crate::relations::circuit_satisfaction_problems::bacs::bacs;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



// /**
//  * Instance map for the BACS-to-R1CS reduction.
//  */

// r1cs_constraint_system<FieldT> bacs_to_r1cs_instance_map(circuit:&bacs_circuit<FieldT>);

// /**
//  * Witness map for the BACS-to-R1CS reduction.
//  */

// r1cs_variable_assignment<FieldT> bacs_to_r1cs_witness_map(circuit:&bacs_circuit<FieldT>,
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




 pub fn bacs_to_r1cs_instance_map<FieldT>(circuit:&bacs_circuit<FieldT>)->r1cs_constraint_system<FieldT>
{
    enter_block("Call to bacs_to_r1cs_instance_map");
    assert!(circuit.is_valid());
    let mut result= r1cs_constraint_system::<FieldT>::new();

// #ifdef DEBUG
    result.variable_annotations = circuit.variable_annotations;
//#endif

    result.primary_input_size = circuit.primary_input_size;
    result.auxiliary_input_size = circuit.auxiliary_input_size + circuit.gates.len();

    for g in &circuit.gates
    {
        result.constraints.push(r1cs_constraint::<FieldT>(g.lhs, g.rhs, g.output));
// #ifdef DEBUG
        if let Some(v) = circuit.gate_annotations.get(g.output.index)
        {
            result.constraint_annotations[result.constraints.len()-1] = v;
        }
//#endif
    }

    for g in &circuit.gates
    {
        if g.is_circuit_output
        {
            result.constraints.push(r1cs_constraint::<FieldT>(1, g.output, 0));

// #ifdef DEBUG
            result.constraint_annotations[result.constraints.len()-1] = FMT("", "output_{}_is_circuit_output", g.output.index);
//#endif
        }
    }

    leave_block("Call to bacs_to_r1cs_instance_map");

    return result;
}


pub fn bacs_to_r1cs_witness_map<FieldT>(circuit:&bacs_circuit<FieldT>,
                                                               primary_input:&bacs_primary_input<FieldT>,
                                                               auxiliary_input:&bacs_auxiliary_input<FieldT>)->r1cs_variable_assignment<FieldT> 
{
    enter_block("Call to bacs_to_r1cs_witness_map");
    const result = circuit.get_all_wires(primary_input, auxiliary_input);
    leave_block("Call to bacs_to_r1cs_witness_map");

    return result;
}



//#endif // BACS_TO_R1CS_TCC_
