/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef R1CS_PCD_PARAMS_HPP_
// #define R1CS_PCD_PARAMS_HPP_

use  <memory>
use  <vector>

use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/cp_handler;



template<typename FieldT>
class r1cs_pcd_compliance_predicate_primary_input {
public:
    std::shared_ptr<r1cs_pcd_message<FieldT> > outgoing_message;

    r1cs_pcd_compliance_predicate_primary_input(const std::shared_ptr<r1cs_pcd_message<FieldT> > &outgoing_message) : outgoing_message(outgoing_message) {}
    r1cs_primary_input<FieldT> as_r1cs_primary_input() const;
};

template<typename FieldT>
class r1cs_pcd_compliance_predicate_auxiliary_input {
public:
    std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > incoming_messages;
    std::shared_ptr<r1cs_pcd_local_data<FieldT> > local_data;
    r1cs_pcd_witness<FieldT> witness;

    r1cs_pcd_compliance_predicate_auxiliary_input(const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_messages,
                                                  const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data,
                                                  const r1cs_pcd_witness<FieldT> &witness) :
        incoming_messages(incoming_messages), local_data(local_data), witness(witness) {}

    r1cs_auxiliary_input<FieldT> as_r1cs_auxiliary_input(const std::vector<size_t> &incoming_message_payload_lengths) const;
};



use libsnark/zk_proof_systems/pcd/r1cs_pcd/r1cs_pcd_params;

//#endif // R1CS_PCD_PARAMS_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#ifndef R1CS_PCD_PARAMS_TCC_
// #define R1CS_PCD_PARAMS_TCC_



template<typename FieldT>
r1cs_primary_input<FieldT> r1cs_pcd_compliance_predicate_primary_input<FieldT>::as_r1cs_primary_input() const
{
    return outgoing_message->as_r1cs_variable_assignment();
}

template<typename FieldT>
r1cs_auxiliary_input<FieldT> r1cs_pcd_compliance_predicate_auxiliary_input<FieldT>::as_r1cs_auxiliary_input(const std::vector<size_t> &incoming_message_payload_lengths) const
{
    const size_t arity = incoming_messages.size();

    r1cs_auxiliary_input<FieldT> result;
    result.push(FieldT(arity));

    const size_t max_arity = incoming_message_payload_lengths.size();
    assert!(arity <= max_arity);

    for i in 0..arity
    {
        const r1cs_variable_assignment<FieldT> msg_as_r1cs_va = incoming_messages[i]->as_r1cs_variable_assignment();
        assert!(msg_as_r1cs_va.size() == (1 + incoming_message_payload_lengths[i]));
        result.insert(result.end(), msg_as_r1cs_va.begin(), msg_as_r1cs_va.end());
    }

    /* pad with dummy messages of appropriate size */
    for i in arity..max_arity
    {
        result.resize(result.size() + (1 + incoming_message_payload_lengths[i]), FieldT::zero());
    }

    const r1cs_variable_assignment<FieldT> local_data_as_r1cs_va = local_data->as_r1cs_variable_assignment();
    result.insert(result.end(), local_data_as_r1cs_va.begin(), local_data_as_r1cs_va.end());
    result.insert(result.end(), witness.begin(), witness.end());

    return result;
}



//#endif // R1CS_PCD_PARAMS_TCC_
