// Declaration of interfaces for a compliance predicate handler.

// A compliance predicate handler is a base pub struct for creating compliance predicates.
// It relies on classes declared in gadgetlib1.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint_system, r1cs_variable_assignment,
};
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig, variable};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    R1csPcdLocalDataConfig, r1cs_pcd_compliance_predicate, r1cs_pcd_local_data, r1cs_pcd_message,
    r1cs_pcd_witness,
};
use rccell::RcCell;
use std::collections::BTreeSet;

/***************************** Message variable ******************************/

/**
 * A variable to represent an r1cs_pcd_message.
 */
#[derive(Clone, Default)]
pub struct r1cs_pcd_message_variable<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget
    num_vars_at_construction: usize,

    types: variable<FieldT, pb_variable>,

    all_vars: pb_variable_array<FieldT, PB>,
}
/*************************** Local data variable *****************************/

/**
 * A variable to represent an r1cs_pcd_local_data.
 */

#[derive(Clone, Default)]
pub struct r1cs_pcd_local_data_variable<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget
    num_vars_at_construction: usize,

    all_vars: pb_variable_array<FieldT, PB>,
}

/*********************** Compliance predicate handler ************************/

/**
 * A base pub struct for creating compliance predicates.
 */
pub trait ProtoboardConfig<FieldT: FieldTConfig, PB: PBConfig> {
    fn clear_values(&self);
    fn val_ref(&mut self, var: &variable<FieldT, pb_variable>) -> &mut FieldT;
    fn val(&self, var: &variable<FieldT, pb_variable>) -> FieldT;
    fn num_variables(&self) -> usize;
    fn full_variable_assignment(&self) -> Vec<FieldT>;
    fn get_constraint_system(
        &self,
    ) -> r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>;
}

pub struct compliance_predicate_handler<
    FieldT: FieldTConfig,
    PB: PBConfig,
    protoboardT: ProtoboardConfig<FieldT, PB>,
> {
    pb: protoboardT,
    outgoing_message: RcCell<r1cs_pcd_message_variables<FieldT, PB>>,
    arity: variable<FieldT, pb_variable>,
    incoming_messages: Vec<RcCell<r1cs_pcd_message_variables<FieldT, PB>>>,
    local_data: RcCell<r1cs_pcd_local_data_variables<FieldT, PB>>,

    name: usize,
    types: usize,
    max_arity: usize,
    relies_on_same_type_inputs: bool,
    accepted_input_types: BTreeSet<usize>,
}

pub type r1cs_pcd_message_variables<FieldT, PB> =
    gadget<FieldT, PB, r1cs_pcd_message_variable<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> r1cs_pcd_message_variable<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut types = variable::<FieldT, pb_variable>::default();
        types.allocate(&pb, prefix_format!(annotation_prefix, " type"));
        let mut all_vars = pb_variable_array::<FieldT, PB>::default();
        all_vars.contents.push(types.clone());

        let num_vars_at_construction = pb.borrow().num_variables();
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                num_vars_at_construction,
                types,
                all_vars,
            },
        )
    }
}
pub trait R1csPcdMessageVariableConfig<FieldT: FieldTConfig> {
    fn get_message(&self) -> RcCell<r1cs_pcd_message<FieldT>>;
}
impl<FieldT: FieldTConfig, PB: PBConfig> R1csPcdMessageVariableConfig<FieldT>
    for r1cs_pcd_message_variables<FieldT, PB>
{
    fn get_message(&self) -> RcCell<r1cs_pcd_message<FieldT>> {
        panic!("");
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> r1cs_pcd_message_variables<FieldT, PB> {
    pub fn update_all_vars(&mut self) {
        /* NOTE: this assumes that r1cs_pcd_message_variable has been the
         * only gadget allocating variables on the protoboard and needs to
         * be updated, e.g., in multicore variable allocation scenario. */

        for var_idx in self.t.num_vars_at_construction + 1..=self.pb.borrow().num_variables() {
            self.t
                .all_vars
                .contents
                .push(variable::<FieldT, pb_variable>::from(var_idx));
        }
    }

    pub fn generate_r1cs_witness(&self, message: &RcCell<r1cs_pcd_message<FieldT>>) {
        self.t
            .all_vars
            .fill_with_field_elements(&self.pb, &message.borrow().as_r1cs_variable_assignment());
    }
}
pub type r1cs_pcd_local_data_variables<FieldT, PB> =
    gadget<FieldT, PB, r1cs_pcd_local_data_variable<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> r1cs_pcd_local_data_variable<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let num_vars_at_construction = pb.borrow().num_variables();
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                num_vars_at_construction,
                all_vars: pb_variable_array::<FieldT, PB>::default(),
            },
        )
    }
}

pub trait R1csPcdLocalDataVariableConfig<FieldT: FieldTConfig> {
    fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<FieldT>>;
}

impl<FieldT: FieldTConfig, PB: PBConfig> R1csPcdLocalDataVariableConfig<FieldT>
    for r1cs_pcd_local_data_variables<FieldT, PB>
{
    fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<FieldT>> {
        panic!("");
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> r1cs_pcd_local_data_variables<FieldT, PB> {
    pub fn update_all_vars(&mut self) {
        /* (the same NOTE as for r1cs_message_variable applies) */

        for var_idx in self.t.num_vars_at_construction + 1..=self.pb.borrow().num_variables() {
            self.t
                .all_vars
                .contents
                .push(variable::<FieldT, pb_variable>::from(var_idx));
        }
    }

    pub fn generate_r1cs_witness(&self, local_data: &RcCell<r1cs_pcd_local_data<FieldT>>) {
        self.t
            .all_vars
            .fill_with_field_elements(&self.pb, &local_data.borrow().as_r1cs_variable_assignment());
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig, protoboardT: ProtoboardConfig<FieldT, PB>>
    compliance_predicate_handler<FieldT, PB, protoboardT>
{
    pub fn new(
        pb: protoboardT,
        name: usize,
        types: usize,
        max_arity: usize,
        relies_on_same_type_inputs: bool,
        accepted_input_types: BTreeSet<usize>,
    ) -> Self {
        let mut incoming_messages =
            vec![RcCell::new(r1cs_pcd_message_variables::<FieldT, PB>::default()); max_arity];
        Self {
            pb,
            outgoing_message: RcCell::new(r1cs_pcd_message_variables::<FieldT, PB>::default()),
            arity: variable::<FieldT, pb_variable>::default(),
            incoming_messages,
            local_data: RcCell::new(r1cs_pcd_local_data_variables::<FieldT, PB>::default()),
            name,
            types,
            max_arity,
            relies_on_same_type_inputs,
            accepted_input_types,
        }
    }

    pub fn generate_r1cs_witness(
        &mut self,
        incoming_message_values: &Vec<RcCell<r1cs_pcd_message<FieldT>>>,
        local_data_value: &RcCell<r1cs_pcd_local_data<FieldT>>,
    ) {
        self.pb.clear_values();
        *self.pb.val_ref(&self.outgoing_message.borrow().t.types) = FieldT::from(self.types);
        *self.pb.val_ref(&self.arity) = FieldT::from(incoming_message_values.len());

        for i in 0..incoming_message_values.len() {
            self.incoming_messages[i]
                .borrow()
                .generate_r1cs_witness(&incoming_message_values[i]);
        }

        self.local_data
            .borrow()
            .generate_r1cs_witness(local_data_value);
    }

    pub fn get_compliance_predicate(
        &self,
    ) -> r1cs_pcd_compliance_predicate<FieldT, pb_variable, pb_linear_combination> {
        assert!(self.incoming_messages.len() == self.max_arity);

        let outgoing_message_payload_length = self.outgoing_message.borrow().t.all_vars.len() - 1;

        let mut incoming_message_payload_lengths: Vec<_> = self
            .incoming_messages
            .iter()
            .map(|msg| msg.borrow().t.all_vars.len() - 1)
            .collect();

        let local_data_length = self.local_data.borrow().t.all_vars.len();

        let all_but_witness_length = ((1 + outgoing_message_payload_length)
            + 1
            + (self.max_arity + incoming_message_payload_lengths.iter().sum::<usize>())
            + local_data_length);
        let witness_length = self.pb.num_variables() - all_but_witness_length;

        let mut constraint_system = self.pb.get_constraint_system();
        constraint_system.primary_input_size = 1 + outgoing_message_payload_length;
        constraint_system.auxiliary_input_size =
            self.pb.num_variables() - constraint_system.primary_input_size;

        r1cs_pcd_compliance_predicate::<FieldT, pb_variable, pb_linear_combination>::new(
            self.name.clone(),
            self.types.clone(),
            constraint_system,
            outgoing_message_payload_length,
            self.max_arity,
            incoming_message_payload_lengths,
            local_data_length,
            witness_length,
            self.relies_on_same_type_inputs.clone(),
            self.accepted_input_types.clone(),
        )
    }

    pub fn get_full_variable_assignment(&self) -> r1cs_variable_assignment<FieldT> {
        return self.pb.full_variable_assignment();
    }

    pub fn get_outgoing_message(&self) -> RcCell<r1cs_pcd_message<FieldT>> {
        self.outgoing_message.borrow().get_message()
    }

    pub fn get_arity(&self) -> usize {
        return self.pb.val(&self.arity).as_ulong();
    }

    pub fn get_incoming_message(&self, message_idx: usize) -> RcCell<r1cs_pcd_message<FieldT>> {
        assert!(message_idx < self.max_arity);
        return self.incoming_messages[message_idx].borrow().get_message();
    }

    pub fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<FieldT>> {
        return self.local_data.borrow().get_local_data();
    }

    pub fn get_witness(&self) -> r1cs_pcd_witness<FieldT> {
        let va = self.pb.full_variable_assignment();
        // outgoing_message + arity + incoming_messages + local_data
        let witness_pos = (self.outgoing_message.borrow().t.all_vars.len()
            + 1
            + self
                .incoming_messages
                .iter()
                .fold(0, |acc, msg| acc + msg.borrow().t.all_vars.len())
            + self.local_data.borrow().t.all_vars.len());

        va[witness_pos..].to_vec()
    }
}
