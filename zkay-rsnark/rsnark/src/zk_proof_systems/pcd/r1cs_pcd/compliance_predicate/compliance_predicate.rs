// Declaration of interfaces for a compliance predicate for R1CS PCD.

// A compliance predicate specifies a local invariant to be enforced, by PCD,
// throughout a dynamic distributed computation. A compliance predicate
// receives input messages, local data, and an output message (and perhaps some
// other auxiliary information), and then either accepts or rejects.
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint_system, r1cs_variable_assignment,
};
use crate::relations::ram_computations::memory::memory_interface::memory_interface;
use crate::relations::ram_computations::rams::ram_params::ram_input_tape;
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_pcd_params::{
    r1cs_pcd_compliance_predicate_auxiliary_input, r1cs_pcd_compliance_predicate_primary_input,
};
use ffec::FieldTConfig;
use ffec::PpConfig;
use rccell::RcCell;
use std::collections::BTreeSet;
use std::marker::PhantomData;
/********************************* Message ***********************************/

/**
 * A message for R1CS PCD.
 *
 * It is a pair, consisting of
 * - a type (a positive integer), and
 * - a payload (a vector of field elements).
 */
#[derive(Default, Clone)]
pub struct r1cs_pcd_message<FieldT: FieldTConfig, T: MessageConfig> {
    pub types: usize,
    pub t: T,
    _t: PhantomData<FieldT>,
}
pub trait MessageConfig: Default + Clone {
    type FieldT: FieldTConfig;
    fn payload_as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<Self::FieldT>;
}

/******************************* Local data **********************************/

/**
 * A local data for R1CS PCD.
 */
#[derive(Default, Clone)]
pub struct r1cs_pcd_local_data<FieldT: FieldTConfig, T: LocalDataConfig> {
    pub t: T,
    _t: PhantomData<FieldT>,
}
impl<FieldT: FieldTConfig, T: LocalDataConfig> r1cs_pcd_local_data<FieldT, T> {
    pub fn new(t: T) -> Self {
        Self { t, _t: PhantomData }
    }
}
pub trait LocalDataConfig {
    type FieldT: FieldTConfig;
    fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<Self::FieldT>;
    fn mem<MI: memory_interface>(&self) -> MI {
        MI::default()
    }
    fn is_halt_case(&self) -> bool {
        false
    }
    fn aux(&self) -> ram_input_tape {
        vec![]
    }
}

impl<FieldT: FieldTConfig, T: LocalDataConfig> LocalDataConfig for r1cs_pcd_local_data<FieldT, T> {
    type FieldT = T::FieldT;
    fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<T::FieldT> {
        self.t.as_r1cs_variable_assignment()
    }
}

/******************************** Witness ************************************/

pub type r1cs_pcd_witness<FieldT> = Vec<FieldT>;

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
#[derive(Default, Clone)]
pub struct r1cs_pcd_compliance_predicate<FieldT: FieldTConfig, ppT: ppTConfig> {
    pub name: usize,
    pub types: usize,
    pub constraint_system: r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
    pub outgoing_message_payload_length: usize,
    pub max_arity: usize,
    pub incoming_message_payload_lengths: Vec<usize>,
    pub local_data_length: usize,
    pub witness_length: usize,
    pub relies_on_same_type_inputs: bool,
    pub accepted_input_types: BTreeSet<usize>,
    _t: PhantomData<ppT>,
}

impl<FieldT: FieldTConfig, T: MessageConfig> MessageConfig for r1cs_pcd_message<FieldT, T> {
    type FieldT = T::FieldT;
    fn payload_as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<T::FieldT> {
        self.t.payload_as_r1cs_variable_assignment()
    }
}

impl<FieldT: FieldTConfig, T: MessageConfig> r1cs_pcd_message<FieldT, T> {
    pub fn new(types: usize, t: T) -> Self {
        Self {
            types,
            t,
            _t: PhantomData,
        }
    }
    pub fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<T::FieldT> {
        let mut result = self.t.payload_as_r1cs_variable_assignment();
        result.insert(0, T::FieldT::from(self.types));
        result
    }

    pub fn print(&self) {
        print!("PCD message (default print routines):\n");
        print!("  Type: {}\n", self.types);

        print!("  Payload\n");
        let payload = self.t.payload_as_r1cs_variable_assignment();
        for elt in &payload {
            elt.print();
        }
    }
}

impl<FieldT: FieldTConfig, ppT: ppTConfig<FieldT = FieldT>>
    r1cs_pcd_compliance_predicate<FieldT, ppT>
{
    pub fn new(
        name: usize,
        types: usize,
        constraint_system: r1cs_constraint_system<FieldT, pb_variable, pb_linear_combination>,
        outgoing_message_payload_length: usize,
        max_arity: usize,
        incoming_message_payload_lengths: Vec<usize>,
        local_data_length: usize,
        witness_length: usize,
        relies_on_same_type_inputs: bool,
        accepted_input_types: BTreeSet<usize>,
    ) -> Self {
        assert!(max_arity == incoming_message_payload_lengths.len());
        Self {
            name,
            types,
            constraint_system,
            outgoing_message_payload_length,
            max_arity,
            incoming_message_payload_lengths,
            local_data_length,
            witness_length,
            relies_on_same_type_inputs,
            accepted_input_types,
            _t: PhantomData,
        }
    }
    pub fn size_in_bits(&self) -> usize {
        0
    }
    pub fn is_well_formed(&self) -> bool {
        let type_not_zero = (self.types != 0);
        let incoming_message_payload_lengths_well_specified =
            (self.incoming_message_payload_lengths.len() == self.max_arity);

        let mut all_message_payload_lengths = self.outgoing_message_payload_length;
        for i in 0..self.incoming_message_payload_lengths.len() {
            all_message_payload_lengths += self.incoming_message_payload_lengths[i];
        }
        let type_vec_length = self.max_arity + 1;
        let arity_length = 1;

        let correct_num_inputs =
            ((self.outgoing_message_payload_length + 1) == self.constraint_system.num_inputs());
        let correct_num_variables = ((all_message_payload_lengths
            + self.local_data_length
            + type_vec_length
            + arity_length
            + self.witness_length)
            == self.constraint_system.num_variables());

        // #ifdef DEBUG
        print!(
            "outgoing_message_payload_length: {}\n",
            self.outgoing_message_payload_length
        );
        print!("incoming_message_payload_lengths:");
        for l in &self.incoming_message_payload_lengths {
            print!(" {}", l);
        }
        print!("\n");
        print!("type_not_zero: {}\n", type_not_zero);
        print!(
            "incoming_message_payload_lengths_well_specified: {}\n",
            incoming_message_payload_lengths_well_specified
        );
        print!(
            "correct_num_inputs: {} (outgoing_message_payload_length = {}, constraint_system.num_inputs() = {})\n",
            correct_num_inputs,
            self.outgoing_message_payload_length,
            self.constraint_system.num_inputs()
        );
        print!(
            "correct_num_variables: {} (all_message_payload_lengths = {}, local_data_length = {}, type_vec_length = {}, arity_length = {}, witness_length = {}, constraint_system.num_variables() = {})\n",
            correct_num_variables,
            all_message_payload_lengths,
            self.local_data_length,
            type_vec_length,
            arity_length,
            self.witness_length,
            self.constraint_system.num_variables()
        );
        //#endif

        (type_not_zero
            && incoming_message_payload_lengths_well_specified
            && correct_num_inputs
            && correct_num_variables)
    }

    pub fn has_equal_input_and_output_lengths(&self) -> bool {
        for i in 0..self.incoming_message_payload_lengths.len() {
            if self.incoming_message_payload_lengths[i] != self.outgoing_message_payload_length {
                return false;
            }
        }

        true
    }

    pub fn has_equal_input_lengths(&self) -> bool {
        for i in 1..self.incoming_message_payload_lengths.len() {
            if self.incoming_message_payload_lengths[i] != self.incoming_message_payload_lengths[0]
            {
                return false;
            }
        }

        true
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

    pub fn is_satisfied(
        &self,
        outgoing_message: &RcCell<r1cs_pcd_message<FieldT, ppT::M>>,
        incoming_messages: &Vec<RcCell<r1cs_pcd_message<FieldT, ppT::M>>>,
        local_data: &RcCell<r1cs_pcd_local_data<FieldT, ppT::LD>>,
        witness: &r1cs_pcd_witness<FieldT>,
    ) -> bool {
        assert!(
            outgoing_message
                .borrow()
                .payload_as_r1cs_variable_assignment()
                .len()
                == self.outgoing_message_payload_length
        );
        assert!(incoming_messages.len() <= self.max_arity);
        for i in 0..incoming_messages.len() {
            assert!(
                incoming_messages[i]
                    .borrow()
                    .payload_as_r1cs_variable_assignment()
                    .len()
                    == self.incoming_message_payload_lengths[i]
            );
        }
        assert!(local_data.borrow().as_r1cs_variable_assignment().len() == self.local_data_length);

        let cp_primary_input = r1cs_pcd_compliance_predicate_primary_input::<FieldT, ppT::M>::from(
            outgoing_message.clone(),
        );
        let cp_auxiliary_input =
            r1cs_pcd_compliance_predicate_auxiliary_input::<FieldT, ppT::M, ppT::LD>::new(
                incoming_messages.clone(),
                local_data.clone(),
                witness.clone(),
            );

        self.constraint_system.is_satisfied(
            &cp_primary_input.as_r1cs_primary_input(),
            &cp_auxiliary_input.as_r1cs_auxiliary_input(&self.incoming_message_payload_lengths),
        )
    }
}
