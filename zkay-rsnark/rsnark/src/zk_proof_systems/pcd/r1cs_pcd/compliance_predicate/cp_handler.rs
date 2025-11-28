/** @file
 *****************************************************************************

 Declaration of interfaces for a compliance predicate handler.

 A compliance predicate handler is a base pub struct for creating compliance predicates.
 It relies on classes declared in gadgetlib1.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CP_HANDLER_HPP_
// #define CP_HANDLER_HPP_

// use  <numeric>

use crate::gadgetlib1::gadget;
use crate::gadgetlib1::protoboard;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;



/***************************** Message variable ******************************/

/**
 * A variable to represent an r1cs_pcd_message.
 */
// 
pub struct r1cs_pcd_message_variable<FieldT> {
//  : public gadget
num_vars_at_construction:    usize,


types:    pb_variable<FieldT>,

all_vars:    pb_variable_array<FieldT>,

   
}
/*************************** Local data variable *****************************/

/**
 * A variable to represent an r1cs_pcd_local_data.
 */
// 
pub struct r1cs_pcd_local_data_variable<FieldT> {
// : public gadget
num_vars_at_construction:    usize,


all_vars:    pb_variable_array<FieldT>,

}

/*********************** Compliance predicate handler ************************/

/**
 * A base pub struct for creating compliance predicates.
 */
// 
pub struct compliance_predicate_handler< FieldT,  protoboardT> {

pb:    protoboardT,

outgoing_message:    RcCell<r1cs_pcd_message_variable<FieldT> >,
arity:    pb_variable<FieldT>,
incoming_messages:    Vec<RcCell<r1cs_pcd_message_variable<FieldT> > >,
local_data:    RcCell<r1cs_pcd_local_data_variable<FieldT> >,

name:     usize,
types:     usize,
max_arity:     usize,
relies_on_same_type_inputs:     bool,
accepted_input_types:     BTreeSet<usize>,

}



// use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;

//#endif // CP_HANDLER_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a compliance predicate handler.

 See cp_handler.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CP_HANDLER_TCC_
// #define CP_HANDLER_TCC_

// use  <algorithm>


impl r1cs_pcd_message_variable<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                                             annotation_prefix:&String) ->Self
    
{
    types.allocate(&pb, FMT(annotation_prefix, " type"));
    all_vars.push(types);

    num_vars_at_construction = pb.num_variables();
    // gadget<FieldT>(&pb, annotation_prefix)
}


pub fn update_all_vars()
{
    /* NOTE: this assumes that r1cs_pcd_message_variable has been the
     * only gadget allocating variables on the protoboard and needs to
     * be updated, e.g., in multicore variable allocation scenario. */

    for var_idx in num_vars_at_construction + 1..=self.pb.num_variables()
    {
        all_vars.push(variable::<FieldT,pb_variable>(var_idx));
    }
}


pub fn generate_r1cs_witness(message:&RcCell<r1cs_pcd_message<FieldT> >)
{
    all_vars.fill_with_field_elements(self.pb, message.as_r1cs_variable_assignment());
}
}

impl r1cs_pcd_local_data_variable<FieldT>{

pub fn new(pb:RcCell<protoboard<FieldT>> ,
                                                                   annotation_prefix:&String)->Self
    
{
    num_vars_at_construction = pb.num_variables();
    // gadget<FieldT>(&pb, annotation_prefix)
}


pub fn update_all_vars()
{
    /* (the same NOTE as for r1cs_message_variable applies) */

    for var_idx in num_vars_at_construction + 1..=self.pb.num_variables()
    {
        all_vars.push(variable::<FieldT,pb_variable>(var_idx));
    }
}


pub fn generate_r1cs_witness(local_data:&RcCell<r1cs_pcd_local_data<FieldT> >)
{
    all_vars.fill_with_field_elements(self.pb, local_data.as_r1cs_variable_assignment());
}}


impl compliance_predicate_handler<FieldT, protoboardT>{

pub fn new(pb:&protoboardT,
                                                                                name:usize,
                                                                                types:usize,
                                                                                max_arity:usize,
                                                                                relies_on_same_type_inputs:bool,
                                                                                accepted_input_types:BTreeSet<usize>) ->Self
    
{
    incoming_messages.resize(max_arity);
    Self{
    pb, name, types, max_arity, relies_on_same_type_inputs,
    accepted_input_types
    }
}


pub fn generate_r1cs_witness(incoming_message_values:&Vec<RcCell<r1cs_pcd_message<FieldT> > >,
                                                                              local_data_value:&RcCell<r1cs_pcd_local_data<FieldT> >)
{
    pb.clear_values();
    pb.val(outgoing_message.types) = FieldT(types);
    pb.val(arity) = FieldT(incoming_message_values.len());

    for i in 0..incoming_message_values.len()
    {
        incoming_messages[i].generate_r1cs_witness(incoming_message_values[i]);
    }

    local_data.generate_r1cs_witness(local_data_value);
}



pub fn get_compliance_predicate()->r1cs_pcd_compliance_predicate<FieldT>
{
    assert!(incoming_messages.len() == max_arity);

    let  outgoing_message_payload_length = outgoing_message.all_vars.len() - 1;

    let mut incoming_message_payload_lengths=vec![0;max_arity];
    std::transform(incoming_messages.begin(), incoming_messages.end(),
                   incoming_message_payload_lengths.begin(),
                   |msg:&RcCell<r1cs_pcd_message_variable<FieldT> >| { return msg.all_vars.len() - 1; });

    let local_data_length = local_data.all_vars.len();

    let all_but_witness_length = ((1 + outgoing_message_payload_length) + 1 +
                                           (max_arity + std::accumulate(incoming_message_payload_lengths.begin(),
                                                                        incoming_message_payload_lengths.end(), 0)) +
                                           local_data_length);
    let witness_length = pb.num_variables() - all_but_witness_length;

    let mut  constraint_system = pb.get_constraint_system();
    constraint_system.primary_input_size = 1 + outgoing_message_payload_length;
    constraint_system.auxiliary_input_size = pb.num_variables() - constraint_system.primary_input_size;

    return r1cs_pcd_compliance_predicate::<FieldT>(name,
                                                 types,
                                                 constraint_system,
                                                 outgoing_message_payload_length,
                                                 max_arity,
                                                 incoming_message_payload_lengths,
                                                 local_data_length,
                                                 witness_length,
                                                 relies_on_same_type_inputs,
                                                 accepted_input_types);
}


pub fn get_full_variable_assignment()->r1cs_variable_assignment<FieldT>
{
    return pb.full_variable_assignment();
}


pub fn get_outgoing_message()->RcCell<r1cs_pcd_message<FieldT>>
{
    return outgoing_message.get_message();
}


pub fn get_arity()->usize
{
    return pb.val(arity).as_ulong();
}


pub fn get_incoming_message(message_idx:usize)->RcCell<r1cs_pcd_message<FieldT>>
{
    assert!(message_idx < max_arity);
    return incoming_messages[message_idx].get_message();
}


pub fn get_local_data()->RcCell<r1cs_pcd_local_data<FieldT>>
{
    return local_data.get_local_data();
}


pub fn get_witness()->r1cs_pcd_witness<FieldT>
{
    let va = pb.full_variable_assignment();
    // outgoing_message + arity + incoming_messages + local_data
    let witness_pos = (outgoing_message.all_vars.len() + 1 +
                                std::accumulate(incoming_messages.begin(), incoming_messages.end(),
                                                0, | acc:usize, msg:&RcCell<r1cs_pcd_message_variable<FieldT> >| {
                                                    return acc + msg.all_vars.len(); }) +
                                local_data.all_vars.len());

    return r1cs_variable_assignment::<FieldT>(va.begin() + witness_pos, va.end());
}

}

//#endif // CP_HANDLER_TCC_
