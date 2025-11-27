/** @file
 *****************************************************************************

 Declaration of interfaces for a compliance predicate for R1CS PCD.

 A compliance predicate specifies a local invariant to be enforced, by PCD,
 throughout a dynamic distributed computation. A compliance predicate
 receives input messages, local data, and an output message (and perhaps some
 other auxiliary information), and then either accepts or rejects.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef COMPLIANCE_PREDICATE_HPP_
// #define COMPLIANCE_PREDICATE_HPP_

// 

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



/********************************* Message ***********************************/

/**
 * A message for R1CS PCD.
 *
 * It is a pair, consisting of
 * - a type (a positive integer), and
 * - a payload (a vector of field elements).
 */
// 
pub struct   r1cs_pcd_message<FieldT> {

types:    usize,


}

/******************************* Local data **********************************/

/**
 * A local data for R1CS PCD.
 */
// 
pub struct   r1cs_pcd_local_data<FieldT> {

}
/******************************** Witness ************************************/

// 
type r1cs_pcd_witness<FieldT> = Vec<FieldT>;

/*************************** Compliance predicate ****************************/


/**
 * A compliance predicate for R1CS PCD.
 *
 * It is a wrapper around R1CS that also specifies how to parse a
 * variable assignment as:
 * - output message (the input)
 * - some number of input messages (part of the witness)
 * - local data (also part of the witness)
 * - auxiliary information (the remaining variables of the witness)
 *
 * A compliance predicate also has a type, allegedly the same
 * as the type of the output message.
 *
 * The input wires of R1CS appear in the following order:
 * - (1 + outgoing_message_payload_length) wires for outgoing message
 * - 1 wire for arity (allegedly, 0 <= arity <= max_arity)
 * - for i = 0, ..., max_arity-1:
 * - (1 + incoming_message_payload_lengths[i]) wires for i-th message of
 *   the input (in the array that's padded to max_arity messages)
 * - local_data_length wires for local data
 *
 * The rest witness_length wires of the R1CS constitute the witness.
 *
 * To allow for optimizations, the compliance predicate also
 * specififies a flag, called relies_on_same_type_inputs, denoting
 * whether the predicate works under the assumption that all input
 * messages have the same type. In such case a member
 * accepted_input_types lists all types accepted by the predicate
 * (accepted_input_types has no meaning if
 * relies_on_same_type_inputs=false).
 */

// 
pub struct   r1cs_pcd_compliance_predicate<FieldT>  {


name:    usize,
types:    usize,

constraint_system:    r1cs_constraint_system<FieldT,SLC>,

outgoing_message_payload_length:    usize,
max_arity:    usize,
incoming_message_payload_lengths:    Vec<usize>,
local_data_length:    usize,
witness_length:    usize,

relies_on_same_type_inputs:    bool,
accepted_input_types:    BTreeSet<usize>,

}




// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;

//#endif // COMPLIANCE_PREDICATE_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a compliance predicate for R1CS PCD.

 See compliance_predicate.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef COMPLIANCE_PREDICATE_TCC_
// #define COMPLIANCE_PREDICATE_TCC_

use ffec::common::utils;



impl r1cs_pcd_message<FieldT>{

 pub fn as_r1cs_variable_assignment() ->r1cs_variable_assignment<FieldT>
{
    let mut  result = self.payload_as_r1cs_variable_assignment();
    result.insert(result.begin(), FieldT(self.types));
    return result;
}


pub fn new(types:usize) ->Self
{
    Self{types}
}


pub fn print() 
{
    print!("PCD message (default print routines):\n");
    print!("  Type: {}\n", self.types);

    print!("  Payload\n");
    let  payload = self.payload_as_r1cs_variable_assignment();
    for elt in &payload
    {
        elt.print();
    }
}
}

impl r1cs_pcd_compliance_predicate<FieldT>{

pub fn new(name:&usize,
                                                                     types:&usize,
                                                                     constraint_system:r1cs_constraint_system<FieldT,SLC>,
                                                                     outgoing_message_payload_length:&usize,
                                                                     max_arity:&usize,
                                                                     incoming_message_payload_lengths:Vec<usize>,
                                                                     local_data_length:&usize,
                                                                     witness_length:&usize,
                                                                     relies_on_same_type_inputs:&bool,
                                                                     accepted_input_types:&BTreeSet<usize>) ->Self
    
{
    assert!(max_arity == incoming_message_payload_lengths.len());
    Self{name,
    types,
    constraint_system,
    outgoing_message_payload_length,
    max_arity,
    incoming_message_payload_lengths,
    local_data_length,
    witness_length,
    relies_on_same_type_inputs,
    accepted_input_types
    }
}


pub fn is_well_formed() ->bool
{
    let  type_not_zero = (types != 0);
    let incoming_message_payload_lengths_well_specified = (incoming_message_payload_lengths.len() == max_arity);

    let  all_message_payload_lengths = outgoing_message_payload_length;
    for i in 0..incoming_message_payload_lengths.len()
    {
        all_message_payload_lengths += incoming_message_payload_lengths[i];
    }
    let type_vec_length = max_arity+1;
    let arity_length = 1;

    let correct_num_inputs = ((outgoing_message_payload_length + 1) == constraint_system.num_inputs());
    let correct_num_variables = ((all_message_payload_lengths + local_data_length + type_vec_length + arity_length + witness_length) == constraint_system.num_variables());

// #ifdef DEBUG
    print!("outgoing_message_payload_length: {}\n", outgoing_message_payload_length);
    print!("incoming_message_payload_lengths:");
    for l in &incoming_message_payload_lengths
    {
        print!(" {}", l);
    }
    print!("\n");
    print!("type_not_zero: {}\n", type_not_zero);
    print!("incoming_message_payload_lengths_well_specified: {}\n", incoming_message_payload_lengths_well_specified);
    print!("correct_num_inputs: {} (outgoing_message_payload_length = {}, constraint_system.num_inputs() = {})\n",
           correct_num_inputs, outgoing_message_payload_length, constraint_system.num_inputs());
    print!("correct_num_variables: {} (all_message_payload_lengths = {}, local_data_length = {}, type_vec_length = {}, arity_length = {}, witness_length = {}, constraint_system.num_variables() = {})\n",
           correct_num_variables,
           all_message_payload_lengths, local_data_length, type_vec_length, arity_length, witness_length,
           constraint_system.num_variables());
//#endif

    return (type_not_zero && incoming_message_payload_lengths_well_specified && correct_num_inputs && correct_num_variables);
}


pub fn has_equal_input_and_output_lengths() ->bool
{
    for i in 0..incoming_message_payload_lengths.len()
    {
        if incoming_message_payload_lengths[i] != outgoing_message_payload_length
        {
            return false;
        }
    }

    return true;
}


pub fn has_equal_input_lengths() ->bool
{
    for i in 1..incoming_message_payload_lengths.len()
    {
        if incoming_message_payload_lengths[i] != incoming_message_payload_lengths[0]
        {
            return false;
        }
    }

    return true;
}


// 
// bool r1cs_pcd_compliance_predicate<FieldT>::operator==(other:r1cs_pcd_compliance_predicate<FieldT>) const
// {
//     return (self.name == other.name &&
//             self.types == other.types &&
//             self.constraint_system == other.constraint_system &&
//             self.outgoing_message_payload_length == other.outgoing_message_payload_length &&
//             self.max_arity == other.max_arity &&
//             self.incoming_message_payload_lengths == other.incoming_message_payload_lengths &&
//             self.local_data_length == other.local_data_length &&
//             self.witness_length == other.witness_length &&
//             self.relies_on_same_type_inputs == other.relies_on_same_type_inputs &&
//             self.accepted_input_types == other.accepted_input_types);
// }

// 
// std::ostream& operator<<(std::ostream &out, cp:r1cs_pcd_compliance_predicate<FieldT>)
// {
//     out << cp.name << "\n";
//     out << cp.types << "\n";
//     out << cp.max_arity << "\n";
//     assert!(cp.max_arity == cp.incoming_message_payload_lengths.len());
//     for i in 0..cp.max_arity
//     {
//         out << cp.incoming_message_payload_lengths[i] << "\n";
//     }
//     out << cp.outgoing_message_payload_length << "\n";
//     out << cp.local_data_length << "\n";
//     out << cp.witness_length << "\n";
//     ffec::output_bool(out, cp.relies_on_same_type_inputs);
//     ffec::operator<<(out, cp.accepted_input_types);
//     out << "\n" << cp.constraint_system << "\n";

//     return out;
// }

// 
// std::istream& operator>>(std::istream &in, r1cs_pcd_compliance_predicate<FieldT> &cp)
// {
//     in >> cp.name;
//     ffec::consume_newline(in);
//     in >> cp.types;
//     ffec::consume_newline(in);
//     in >> cp.max_arity;
//     ffec::consume_newline(in);
//     cp.incoming_message_payload_lengths.resize(cp.max_arity);
//     for i in 0..cp.max_arity
//     {
//         in >> cp.incoming_message_payload_lengths[i];
//         ffec::consume_newline(in);
//     }
//     in >> cp.outgoing_message_payload_length;
//     ffec::consume_newline(in);
//     in >> cp.local_data_length;
//     ffec::consume_newline(in);
//     in >> cp.witness_length;
//     ffec::consume_newline(in);
//     ffec::input_bool(in, cp.relies_on_same_type_inputs);
//     ffec::operator>>(in, cp.accepted_input_types);
//     ffec::consume_newline(in);
//     in >> cp.constraint_system;
//     ffec::consume_newline(in);

//     return in;
// }



pub fn is_satisfied(outgoing_message:&RcCell<r1cs_pcd_message<FieldT> >,
                                                         incoming_messages:&Vec<RcCell<r1cs_pcd_message<FieldT> > >,
                                                         local_data:&RcCell<r1cs_pcd_local_data<FieldT> >,
                                                         witness:&r1cs_pcd_witness<FieldT>) ->bool
{
    assert!(outgoing_message.payload_as_r1cs_variable_assignment().len() == outgoing_message_payload_length);
    assert!(incoming_messages.len() <= max_arity);
    for i in 0..incoming_messages.len()
    {
        assert!(incoming_messages[i].payload_as_r1cs_variable_assignment().len() == incoming_message_payload_lengths[i]);
    }
    assert!(local_data.as_r1cs_variable_assignment().len() == local_data_length);

     let cp_primary_input=r1cs_pcd_compliance_predicate_primary_input::<FieldT>::new(outgoing_message);
     let cp_auxiliary_input=r1cs_pcd_compliance_predicate_auxiliary_input::<FieldT>::new(incoming_messages, local_data, witness);

    return constraint_system.is_satisfied(cp_primary_input.as_r1cs_primary_input(),
                                          cp_auxiliary_input.as_r1cs_auxiliary_input(incoming_message_payload_lengths));
}

}

//#endif //  COMPLIANCE_PREDICATE_TCC_
