/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef R1CS_PCD_PARAMS_HPP_
// #define R1CS_PCD_PARAMS_HPP_

// use  <memory>
// use  <vector>

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;




pub struct  r1cs_pcd_compliance_predicate_primary_input<FieldT> {

outgoing_message:    std::shared_ptr<r1cs_pcd_message<FieldT> >,
}
impl r1cs_pcd_compliance_predicate_primary_input<FieldT> {
    pub fn new(outgoing_message:&std::shared_ptr<r1cs_pcd_message<FieldT> >) ->Self {
    Self{outgoing_message}}
}


pub struct  r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> {

incoming_messages:    std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > >,
local_data:    std::shared_ptr<r1cs_pcd_local_data<FieldT> >,
witness:    r1cs_pcd_witness<FieldT>,
}
impl r1cs_pcd_compliance_predicate_auxiliary_input<FieldT> {
    pub fn new(incoming_messages:&std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > >,
                                                  local_data:&std::shared_ptr<r1cs_pcd_local_data<FieldT> >,
                                                  witness:&r1cs_pcd_witness<FieldT>) ->Self
        {
        Self{incoming_messages ,local_data, witness
    }}

}



// use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params;

//#endif // R1CS_PCD_PARAMS_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef R1CS_PCD_PARAMS_TCC_
// #define R1CS_PCD_PARAMS_TCC_


impl r1cs_pcd_compliance_predicate_primary_input<FieldT>{

pub fn as_r1cs_primary_input() ->r1cs_primary_input<FieldT>
{
    return outgoing_message.as_r1cs_variable_assignment();
}
}
impl r1cs_pcd_compliance_predicate_auxiliary_input<FieldT>{

pub fn as_r1cs_auxiliary_input(incoming_message_payload_lengths:&std::vector<size_t>) ->r1cs_auxiliary_input<FieldT> 
{
   let  arity = incoming_messages.len();

    let mut  result=r1cs_auxiliary_input::<FieldT>::new();
    result.push(FieldT(arity));

   let max_arity = incoming_message_payload_lengths.len();
    assert!(arity <= max_arity);

    for i in 0..arity
    {
        letmsg_as_r1cs_va = incoming_messages[i].as_r1cs_variable_assignment();
        assert!(msg_as_r1cs_va.len() == (1 + incoming_message_payload_lengths[i]));
        result.insert(result.end(), msg_as_r1cs_va.begin(), msg_as_r1cs_va.end());
    }

    /* pad with dummy messages of appropriate size */
    for i in arity..max_arity
    {
        result.resize(result.len() + (1 + incoming_message_payload_lengths[i]), FieldT::zero());
    }

    let  local_data_as_r1cs_va = local_data.as_r1cs_variable_assignment();
    result.insert(result.end(), local_data_as_r1cs_va.begin(), local_data_as_r1cs_va.end());
    result.insert(result.end(), witness.begin(), witness.end());

    return result;
}
}


//#endif // R1CS_PCD_PARAMS_TCC_
