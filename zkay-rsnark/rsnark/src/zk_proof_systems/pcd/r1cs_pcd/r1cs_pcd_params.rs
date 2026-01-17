use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_primary_input,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::MessageConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, r1cs_pcd_local_data, r1cs_pcd_message, r1cs_pcd_witness,
};
use ffec::FieldTConfig;
use rccell::RcCell;

#[derive(Default, Clone)]
pub struct r1cs_pcd_compliance_predicate_primary_input<
    FieldT: FieldTConfig,
    T: MessageConfig<FieldT = FieldT>,
> {
    pub outgoing_message: RcCell<r1cs_pcd_message<FieldT, T>>,
}
impl<FieldT: FieldTConfig, T: MessageConfig<FieldT = FieldT>>
    r1cs_pcd_compliance_predicate_primary_input<FieldT, T>
{
    pub fn new(outgoing_message: RcCell<r1cs_pcd_message<FieldT, T>>) -> Self {
        Self { outgoing_message }
    }
}

pub struct r1cs_pcd_compliance_predicate_auxiliary_input<
    FieldT: FieldTConfig,
    M: MessageConfig<FieldT = FieldT>,
    LD: LocalDataConfig<FieldT = FieldT>,
> {
    incoming_messages: Vec<RcCell<r1cs_pcd_message<FieldT, M>>>,
    local_data: RcCell<r1cs_pcd_local_data<FieldT, LD>>,
    witness: r1cs_pcd_witness<FieldT>,
}
impl<FieldT: FieldTConfig, M: MessageConfig<FieldT = FieldT>, LD: LocalDataConfig<FieldT = FieldT>>
    r1cs_pcd_compliance_predicate_auxiliary_input<FieldT, M, LD>
{
    pub fn new(
        incoming_messages: Vec<RcCell<r1cs_pcd_message<FieldT, M>>>,
        local_data: RcCell<r1cs_pcd_local_data<FieldT, LD>>,
        witness: r1cs_pcd_witness<FieldT>,
    ) -> Self {
        Self {
            incoming_messages,
            local_data,
            witness,
        }
    }
}

impl<FieldT: FieldTConfig, T: MessageConfig<FieldT = FieldT>>
    r1cs_pcd_compliance_predicate_primary_input<FieldT, T>
{
    pub fn as_r1cs_primary_input(&self) -> r1cs_primary_input<FieldT> {
        self.outgoing_message.borrow().as_r1cs_variable_assignment()
    }
}
impl<FieldT: FieldTConfig, M: MessageConfig<FieldT = FieldT>, LD: LocalDataConfig<FieldT = FieldT>>
    r1cs_pcd_compliance_predicate_auxiliary_input<FieldT, M, LD>
{
    pub fn as_r1cs_auxiliary_input(
        &self,
        incoming_message_payload_lengths: &Vec<usize>,
    ) -> r1cs_auxiliary_input<FieldT> {
        let arity = self.incoming_messages.len();

        let mut result = r1cs_auxiliary_input::<FieldT>::default();
        result.push(FieldT::from(arity));

        let max_arity = incoming_message_payload_lengths.len();
        assert!(arity <= max_arity);

        for i in 0..arity {
            let msg_as_r1cs_va = self.incoming_messages[i]
                .borrow()
                .as_r1cs_variable_assignment();
            assert!(msg_as_r1cs_va.len() == (1 + incoming_message_payload_lengths[i]));
            result.extend(msg_as_r1cs_va);
        }

        /* pad with dummy messages of appropriate size */
        for i in arity..max_arity {
            result.resize(
                result.len() + (1 + incoming_message_payload_lengths[i]),
                FieldT::zero(),
            );
        }

        let local_data_as_r1cs_va = self.local_data.borrow().as_r1cs_variable_assignment();
        result.extend(local_data_as_r1cs_va);
        result.extend(self.witness.clone());

        result
    }
}
