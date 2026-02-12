// Declaration of interfaces for a compliance predicate handler.

// A compliance predicate handler is a base pub struct for creating compliance predicates.
// It relies on classes declared in gadgetlib1.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint_system, r1cs_variable_assignment,
};
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig, variable};
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::MessageConfig;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, r1cs_pcd_compliance_predicate, r1cs_pcd_local_data, r1cs_pcd_message,
    r1cs_pcd_witness,
};
use ffec::FieldTConfig;
use rccell::RcCell;
use std::collections::BTreeSet;

/**
 * A variable to represent an r1cs_pcd_message.
 */
#[derive(Clone, Default)]
pub struct r1cs_pcd_message_variable<T: MessageVariableConfig> {
    //  : public gadget
    pub num_vars_at_construction: usize,
    pub types: variable<T::FieldT, pb_variable>,
    pub all_vars: pb_variable_array<T::FieldT, T::PB>,
    pub t: T,
}

/**
 * A variable to represent an r1cs_pcd_local_data.
 */

#[derive(Clone, Default)]
pub struct r1cs_pcd_local_data_variable<T: LocalDataVariableConfig> {
    // : public gadget
    pub num_vars_at_construction: usize,
    pub all_vars: pb_variable_array<T::FieldT, T::PB>,
    pub t: T,
}

/**
 * A base pub struct for creating compliance predicates.
 */

pub trait CPHConfig: ppTConfig {
    // type ppT: ppTConfig;
    // type FieldT: FieldTConfig;
    // type PB: PBConfig;
    type protoboardT: ProtoboardConfig<FieldT = Self::FieldT, PB = Self::PB>;
    type MV: MessageVariableConfig<Output = Self::M, FieldT = Self::FieldT, PB = Self::PB>;
    type LDV: LocalDataVariableConfig<Output = Self::LD, FieldT = Self::FieldT, PB = Self::PB>;
    // type M: MessageConfig<FieldT = Self::FieldT>;
    // type LD: LocalDataConfig<FieldT = Self::FieldT>;
}
#[derive(Clone, Default)]
pub struct compliance_predicate_handler<CPH: CPHConfig, T: Default + Clone> {
    pub pb: RcCell<CPH::protoboardT>,
    pub outgoing_message: RcCell<r1cs_pcd_message_variables<CPH::MV>>,
    pub arity: variable<CPH::FieldT, pb_variable>,
    pub incoming_messages: Vec<RcCell<r1cs_pcd_message_variables<CPH::MV>>>,
    pub local_data: RcCell<r1cs_pcd_local_data_variables<CPH::LDV>>,
    pub name: usize,
    pub types: usize,
    pub max_arity: usize,
    pub relies_on_same_type_inputs: bool,
    pub accepted_input_types: BTreeSet<usize>,
    pub t: T,
}

pub type r1cs_pcd_message_variables<T> = gadget<
    <T as MessageVariableConfig>::FieldT,
    <T as MessageVariableConfig>::PB,
    r1cs_pcd_message_variable<T>,
>;
impl<T: MessageVariableConfig> r1cs_pcd_message_variable<T> {
    pub fn new(
        pb: RcCell<protoboard<T::FieldT, T::PB>>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<T::FieldT, T::PB, Self> {
        let mut types = variable::<T::FieldT, pb_variable>::default();
        types.allocate(&pb, prefix_format!(annotation_prefix, " type"));
        let mut all_vars = pb_variable_array::<T::FieldT, T::PB>::default();
        all_vars.contents.push(types.clone());

        let num_vars_at_construction = pb.borrow().num_variables();
        gadget::<T::FieldT, T::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                num_vars_at_construction,
                types,
                all_vars,
                t,
            },
        )
    }
}
pub trait MessageVariableConfig: Default + Clone {
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    type Output: MessageConfig<FieldT = Self::FieldT>;
    fn get_message(&self) -> RcCell<r1cs_pcd_message<Self::FieldT, Self::Output>>;
}
// impl<FieldT: FieldTConfig, PB: PBConfig,T: MessageVariableConfig<FieldT>> MessageVariableConfig<FieldT>
//     for r1cs_pcd_message_variables<FieldT, PB,T>
// {
//     type Output=T;
//     fn get_message(&self) -> RcCell<r1cs_pcd_message<FieldT,Self::Output>> {
//        self.t.t.get_message()
//     }
// }
impl<T: MessageVariableConfig> r1cs_pcd_message_variables<T> {
    pub fn update_all_vars(&mut self) {
        /* NOTE: this assumes that r1cs_pcd_message_variable has been the
         * only gadget allocating variables on the protoboard and needs to
         * be updated, e.g., in multicore variable allocation scenario. */

        for var_idx in self.t.num_vars_at_construction + 1..=self.pb.borrow().num_variables() {
            self.t
                .all_vars
                .contents
                .push(variable::<T::FieldT, pb_variable>::from(var_idx));
        }
    }

    pub fn generate_r1cs_witness(&self, message: &RcCell<r1cs_pcd_message<T::FieldT, T::Output>>) {
        self.t
            .all_vars
            .fill_with_field_elements(&self.pb, &message.borrow().as_r1cs_variable_assignment());
    }
}
pub type r1cs_pcd_local_data_variables<T> = gadget<
    <T as LocalDataVariableConfig>::FieldT,
    <T as LocalDataVariableConfig>::PB,
    r1cs_pcd_local_data_variable<T>,
>;
impl<T: LocalDataVariableConfig> r1cs_pcd_local_data_variable<T> {
    pub fn new(
        pb: RcCell<protoboard<T::FieldT, T::PB>>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<T::FieldT, T::PB, Self> {
        let num_vars_at_construction = pb.borrow().num_variables();
        gadget::<T::FieldT, T::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                num_vars_at_construction,
                all_vars: pb_variable_array::<T::FieldT, T::PB>::default(),
                t,
            },
        )
    }
}

pub trait LocalDataVariableConfig: Default + Clone {
    type FieldT: FieldTConfig;
    type PB: PBConfig;
    type Output: LocalDataConfig<FieldT = Self::FieldT>;
    fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<Self::FieldT, Self::Output>>;
}

// impl<FieldT: FieldTConfig, PB: PBConfig> LocalDataVariableConfig<FieldT>
//     for r1cs_pcd_local_data_variables<FieldT, PB>
// {
//     fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<FieldT>> {
//         panic!("");
//     }
// }
impl<T: LocalDataVariableConfig> r1cs_pcd_local_data_variables<T> {
    pub fn update_all_vars(&mut self) {
        /* (the same NOTE as for r1cs_message_variable applies) */
        for var_idx in self.t.num_vars_at_construction + 1..=self.pb.borrow().num_variables() {
            self.t
                .all_vars
                .contents
                .push(variable::<T::FieldT, pb_variable>::from(var_idx));
        }
    }

    pub fn generate_r1cs_witness(
        &self,
        local_data: &RcCell<r1cs_pcd_local_data<T::FieldT, T::Output>>,
    ) {
        self.t
            .all_vars
            .fill_with_field_elements(&self.pb, &local_data.borrow().as_r1cs_variable_assignment());
    }
}

impl<CPH: CPHConfig, T: Default + Clone> compliance_predicate_handler<CPH, T> {
    pub fn new(
        pb: RcCell<CPH::protoboardT>,
        name: usize,
        types: usize,
        max_arity: usize,
        relies_on_same_type_inputs: bool,
        accepted_input_types: BTreeSet<usize>,
        t: T,
    ) -> Self {
        let mut incoming_messages =
            vec![RcCell::new(r1cs_pcd_message_variables::<CPH::MV>::default()); max_arity];
        Self {
            pb,
            outgoing_message: RcCell::new(r1cs_pcd_message_variables::<CPH::MV>::default()),
            arity: variable::<CPH::FieldT, pb_variable>::default(),
            incoming_messages,
            local_data: RcCell::new(r1cs_pcd_local_data_variables::<CPH::LDV>::default()),
            name,
            types,
            max_arity,
            relies_on_same_type_inputs,
            accepted_input_types,
            t,
        }
    }

    pub fn generate_r1cs_witness_base(
        &mut self,
        incoming_message_values: &Vec<RcCell<r1cs_pcd_message<CPH::FieldT, CPH::M>>>,
        local_data_value: &RcCell<r1cs_pcd_local_data<CPH::FieldT, CPH::LD>>,
    ) {
        self.pb.borrow_mut().clear_values();
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.outgoing_message.borrow().t.types) = CPH::FieldT::from(self.types);
        *self.pb.borrow_mut().val_ref(&self.arity) =
            CPH::FieldT::from(incoming_message_values.len());

        for i in 0..incoming_message_values.len() {
            self.incoming_messages[i]
                .borrow()
                .generate_r1cs_witness(&incoming_message_values[i]);
        }

        self.local_data
            .borrow()
            .generate_r1cs_witness(local_data_value);
    }

    pub fn get_compliance_predicate(&self) -> r1cs_pcd_compliance_predicate<CPH::FieldT, CPH> {
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
        let witness_length = self.pb.borrow().num_variables() - all_but_witness_length;

        let mut constraint_system = self.pb.borrow().get_constraint_system();
        constraint_system.primary_input_size = 1 + outgoing_message_payload_length;
        constraint_system.auxiliary_input_size =
            self.pb.borrow().num_variables() - constraint_system.primary_input_size;

        r1cs_pcd_compliance_predicate::<CPH::FieldT, CPH>::new(
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

    pub fn get_full_variable_assignment(&self) -> r1cs_variable_assignment<CPH::FieldT> {
        self.pb.borrow().full_variable_assignment()
    }

    pub fn get_outgoing_message(&self) -> RcCell<r1cs_pcd_message<CPH::FieldT, CPH::M>> {
        self.outgoing_message.borrow().t.t.get_message()
    }

    pub fn get_arity(&self) -> usize {
        self.pb.borrow().val(&self.arity).as_ulong()
    }

    pub fn get_incoming_message(
        &self,
        message_idx: usize,
    ) -> RcCell<r1cs_pcd_message<CPH::FieldT, CPH::M>> {
        assert!(message_idx < self.max_arity);
        self.incoming_messages[message_idx]
            .borrow()
            .t
            .t
            .get_message()
    }

    pub fn get_local_data(&self) -> RcCell<r1cs_pcd_local_data<CPH::FieldT, CPH::LD>> {
        self.local_data.borrow().t.t.get_local_data()
    }

    pub fn get_witness(&self) -> r1cs_pcd_witness<CPH::FieldT> {
        let va = self.pb.borrow().full_variable_assignment();
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
