// Declaration of interfaces for a compliance predicate for RAM.

// The implementation follows, extends, and optimizes the approach described
// in \[BCTV14].

// Essentially, the RAM's CPU, which is expressed as an R1CS constraint system,
// is augmented to obtain another R1CS constraint system that implements a RAM
// compliance predicate. This predicate is responsible for checking:
// (1) transitions from a CPU state to the next;
// (2) correct load/stores; and
// (3) corner cases such as the first and last steps of the machine.
// The first can be done by suitably embedding the RAM's CPU in the constraint
// system. The second can be done by verifying authentication paths for the values
// of memory. The third mostly consists of bookkeeping (with some subtleties arising
// from the need to not break zero knowledge).

// The laying out of R1CS constraints is done via gadgetlib1 (a minimalistic
// library for writing R1CS constraint systems).

// References:

// \[BCTV14]:
// "Scalable Zero Knowledge via Cycles of Elliptic Curves",
// Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
// CRYPTO 2014,
// <http://eprint.iacr.org/2014/595>

use crate::common::data_structures::merkle_tree::HashTConfig;
use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTST};
use crate::gadgetlib1::gadgets::basic_gadgets::{
    bit_vector_copy_gadget, bit_vector_copy_gadgets, generate_boolean_r1cs_constraint,
    generate_r1cs_equals_const_constraint, multipacking_gadget, multipacking_gadgets,
    packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_gadget::{
    memory_load_gadget, memory_load_gadgets,
};
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_store_gadget::{
    memory_load_store_gadget, memory_load_store_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::crh_gadget::{
    CRH_with_bit_out_gadget, CRH_with_bit_out_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::hash_io::{digest_variable, digest_variables};
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable::{
    merkle_authentication_path_variable, merkle_authentication_path_variables,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_variable, pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::ram_computations::memory::memory_interface::memory_interface;
use crate::relations::variable::linear_combination;
use ffec::PpConfig;

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint, r1cs_variable_assignment,
};
use crate::relations::ram_computations::memory::delegated_ra_memory::{
    delegated_ra_memory, delegated_ra_memorys,
};
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, CpuCheckConfig, ram_architecture_params, ram_base_field,
    ram_boot_trace, ram_cpu_checker, ram_input_tape, ram_params_type, ram_protoboard,
};
use crate::relations::variable::variable;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::{
    LocalDataConfig, MessageConfig, r1cs_pcd_local_data, r1cs_pcd_message,
};

use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::{
    LocalDataVariableConfig, MessageVariableConfig, compliance_predicate_handler,
    r1cs_pcd_local_data_variable, r1cs_pcd_local_data_variables, r1cs_pcd_message_variable,
    r1cs_pcd_message_variables,
};
use crate::zk_proof_systems::pcd::r1cs_pcd::r1cs_sp_ppzkpcd::r1cs_sp_ppzkpcd::{
    r1cs_sp_ppzkpcd_proving_key, r1cs_sp_ppzkpcd_verification_key,
};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::{
    RamConfig, ram_zksnark_PCD_pp, ram_zksnark_architecture_params,
};
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::field_utils::field_utils::{
    convert_bit_vector_to_field_element, convert_field_element_to_bit_vector1,
    pack_bit_vector_into_field_element_vector,
};
use ffec::{One, Zero, bit_vector, div_ceil, int_list_to_bits, log2};
use rccell::RcCell;
use std::collections::BTreeSet;

/**
 * A RAM message specializes the generic PCD message, in order to
 * obtain a more user-friendly print method.
 */
//
type FieldT<RamT> = ram_base_field<RamT>;

#[derive(Default, Clone)]
pub struct ram_pcd_message<RamT: ram_params_type> {
    // : public r1cs_pcd_message<FieldT<RamT> >
    pub ap: ram_architecture_params<RamT>,
    pub timestamp: usize,
    pub root_initial: bit_vector,
    pub root: bit_vector,
    pub pc_addr: usize,
    pub cpu_state: bit_vector,
    pub pc_addr_initial: usize,
    pub cpu_state_initial: bit_vector,
    pub has_accepted: bool,
}

#[derive(Default, Clone)]
pub struct ram_pcd_message_variable<RamT: ram_params_type> {
    //  : public r1cs_pcd_message_variable<FieldT<RamT> >
    pub ap: ram_architecture_params<RamT>,
    pub packed_payload: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub timestamp: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub root_initial: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub root: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub pc_addr: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub cpu_state: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub pc_addr_initial: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub cpu_state_initial: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub has_accepted: variable<FieldT<RamT>, pb_variable>,
    pub all_unpacked_vars: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub unpack_payload: RcCell<multipacking_gadgets<FieldT<RamT>, RamT::PB>>,
}

#[derive(Default, Clone)]
pub struct ram_pcd_local_data<RamT: ram_params_type> {
    // : public r1cs_pcd_local_data<FieldT<RamT> >
    pub is_halt_case: bool,
    pub mem: delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>,
    pub aux: ram_input_tape,
}

#[derive(Default, Clone)]
pub struct ram_pcd_local_data_variable<RamT: ram_params_type> {
    // : public r1cs_pcd_local_data_variable<FieldT<RamT> >
    pub is_halt_case: variable<FieldT<RamT>, pb_variable>,
}

/**
 * A RAM compliance predicate.
 */

type HashT<RamT> = CRH_with_bit_out_gadgets<FieldT<RamT>, <RamT as ppTConfig>::PB>;
type base_handler<RamT> = compliance_predicate_handler<FieldT<RamT>, ram_protoboard<RamT>>;

#[derive(Default, Clone)]
pub struct ram_compliance_predicate_handler<RamT: ram_params_type> {
    // : public compliance_predicate_handler<FieldT<RamT>, ram_protoboard<RamT> >
    pub ap: ram_architecture_params<RamT>,
    pub next: RcCell<ram_pcd_message_variables<RamT>>,
    pub cur: RcCell<ram_pcd_message_variables<RamT>>,
    pub zero: variable<FieldT<RamT>, pb_variable>, // TODO: promote linear combinations to first pub struct objects
    pub copy_root_initial: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub copy_pc_addr_initial: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub copy_cpu_state_initial: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub is_base_case: variable<FieldT<RamT>, pb_variable>,
    pub is_not_halt_case: variable<FieldT<RamT>, pb_variable>,
    pub packed_cur_timestamp: variable<FieldT<RamT>, pb_variable>,
    pub pack_cur_timestamp: RcCell<packing_gadgets<FieldT<RamT>, RamT::PB>>,
    pub packed_next_timestamp: variable<FieldT<RamT>, pb_variable>,
    pub pack_next_timestamp: RcCell<packing_gadgets<FieldT<RamT>, RamT::PB>>,
    pub zero_cpu_state: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub zero_pc_addr: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub zero_root: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub initialize_cur_cpu_state: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub initialize_prev_pc_addr: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub initialize_root: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub prev_pc_val: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub prev_pc_val_digest: RcCell<digest_variables<FieldT<RamT>, RamT::PB>>,
    pub cur_root_digest: RcCell<digest_variables<FieldT<RamT>, RamT::PB>>,
    pub instruction_fetch_merkle_proof:
        RcCell<merkle_authentication_path_variables<FieldT<RamT>, RamT::PB, HashT<RamT>>>,
    pub instruction_fetch: RcCell<memory_load_gadgets<FieldT<RamT>, RamT::PB, HashT<RamT>>>,
    pub next_root_digest: RcCell<digest_variables<FieldT<RamT>, RamT::PB>>,
    pub ls_addr: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub ls_prev_val: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub ls_next_val: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub ls_prev_val_digest: RcCell<digest_variables<FieldT<RamT>, RamT::PB>>,
    pub ls_next_val_digest: RcCell<digest_variables<FieldT<RamT>, RamT::PB>>,
    pub load_merkle_proof:
        RcCell<merkle_authentication_path_variables<FieldT<RamT>, RamT::PB, HashT<RamT>>>,
    pub store_merkle_proof:
        RcCell<merkle_authentication_path_variables<FieldT<RamT>, RamT::PB, HashT<RamT>>>,
    pub load_store_checker: RcCell<memory_load_store_gadgets<FieldT<RamT>, RamT::PB, HashT<RamT>>>,
    pub temp_next_pc_addr: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub temp_next_cpu_state: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub cpu_checker: RcCell<ram_cpu_checker<RamT>>,
    pub do_halt: variable<FieldT<RamT>, pb_variable>,
    pub clear_next_root: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub clear_next_pc_addr: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub clear_next_cpu_state: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub copy_temp_next_root: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub copy_temp_next_pc_addr: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub copy_temp_next_cpu_state: RcCell<bit_vector_copy_gadgets<FieldT<RamT>, RamT::PB>>,
    pub addr_size: usize,
    pub value_size: usize,
    pub digest_size: usize,
    pub message_length: usize,
}
impl<RamT: ram_params_type> MessageConfig for ram_pcd_message<RamT> {
    type FieldT = FieldT<RamT>;
    fn payload_as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<FieldT<RamT>> {
        let mut payload_bits = self.unpacked_payload_as_bits();
        let result = pack_bit_vector_into_field_element_vector::<FieldT<RamT>>(&payload_bits);
        result
    }
}
// use crate::gadgetlib1::constraint_profiling;
pub type ram_pcd_messages<RamT> = r1cs_pcd_message<FieldT<RamT>, ram_pcd_message<RamT>>;
impl<RamT: ram_params_type> ram_pcd_message<RamT> {
    pub fn new(
        types: usize,
        ap: ram_architecture_params<RamT>,
        timestamp: usize,
        root_initial: bit_vector,
        root: bit_vector,
        pc_addr: usize,
        cpu_state: bit_vector,
        pc_addr_initial: usize,
        cpu_state_initial: bit_vector,
        has_accepted: bool,
    ) -> r1cs_pcd_message<FieldT<RamT>, Self> {
        let digest_size = CRH_with_bit_out_gadgets::<FieldT<RamT>, RamT::PB>::get_digest_len();
        assert!(log2(timestamp) < RamT::timestamp_length);
        assert!(root_initial.len() == digest_size);
        assert!(root.len() == digest_size);
        assert!(log2(pc_addr) < ap.address_size());
        assert!(cpu_state.len() == ap.cpu_state_size());
        assert!(log2(pc_addr_initial) < ap.address_size());
        assert!(cpu_state_initial.len() == ap.cpu_state_size());
        r1cs_pcd_message::<FieldT<RamT>, Self>::new(
            types,
            Self {
                ap,
                timestamp,
                root_initial,
                root,
                pc_addr,
                cpu_state,
                pc_addr_initial,
                cpu_state_initial,
                has_accepted,
            },
        )
    }

    pub fn unpacked_payload_as_bits(&self) -> bit_vector {
        let mut result = vec![];

        let timestamp_bits = convert_field_element_to_bit_vector1::<FieldT<RamT>>(
            &FieldT::<RamT>::from(self.timestamp),
            RamT::timestamp_length,
        );
        let pc_addr_bits = convert_field_element_to_bit_vector1::<FieldT<RamT>>(
            &FieldT::<RamT>::from(self.pc_addr),
            self.ap.address_size(),
        );
        let pc_addr_initial_bits = convert_field_element_to_bit_vector1::<FieldT<RamT>>(
            &FieldT::<RamT>::from(self.pc_addr_initial),
            self.ap.address_size(),
        );

        result.extend(&timestamp_bits);
        result.extend(&self.root_initial);
        result.extend(&self.root);
        result.extend(&pc_addr_bits);
        result.extend(&self.cpu_state);
        result.extend(&pc_addr_initial_bits);
        result.extend(&self.cpu_state_initial);
        result.push(self.has_accepted);

        assert!(result.len() == Self::unpacked_payload_size_in_bits(&self.ap));
        result
    }

    pub fn print_bits(bv: &bit_vector) {
        for &b in bv {
            print!("{}", b as u8);
        }
        print!("\n");
    }

    pub fn print(&self) {
        print!("ram_pcd_message:\n");
        // print!("  type: {}\n", self.types);
        print!("  timestamp: {}\n", self.timestamp);
        print!("  root_initial: ");
        Self::print_bits(&self.root_initial);
        print!("  root: ");
        Self::print_bits(&self.root);
        print!("  pc_addr: {}\n", self.pc_addr);
        print!("  cpu_state: ");
        Self::print_bits(&self.cpu_state);
        print!("  pc_addr_initial: {}\n", self.pc_addr_initial);
        print!("  cpu_state_initial: ");
        Self::print_bits(&self.cpu_state_initial);
        print!(
            "  has_accepted: {}\n",
            if self.has_accepted { "YES" } else { "no" }
        );
    }

    pub fn unpacked_payload_size_in_bits(ap: &ram_architecture_params<RamT>) -> usize {
        let digest_size = CRH_with_bit_out_gadgets::<FieldT<RamT>, RamT::PB>::get_digest_len();

        (RamT::timestamp_length + // timestamp
            2*digest_size + // root, root_initial
            2*ap.address_size() + // pc_addr, pc_addr_initial
            2*ap.cpu_state_size() + // cpu_state, cpu_state_initial
            1) // has_accepted
    }
}

pub type ram_pcd_message_variables<RamT> =
    r1cs_pcd_message_variables<ram_pcd_message_variable<RamT>>;
impl<RamT: ram_params_type> ram_pcd_message_variable<RamT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT<RamT>, RamT::PB>>,
        ap: ram_architecture_params<RamT>,
        annotation_prefix: String,
    ) -> ram_pcd_message_variables<RamT> {
        let unpacked_payload_size_in_bits =
            ram_pcd_message::<RamT>::unpacked_payload_size_in_bits(&ap);
        let packed_payload_size =
            div_ceil(unpacked_payload_size_in_bits, FieldT::<RamT>::capacity()).unwrap();
        let mut packed_payload = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        packed_payload.allocate(
            &pb,
            packed_payload_size,
            prefix_format!(annotation_prefix, " packed_payload"),
        );

        let mut _self = r1cs_pcd_message_variable::<Self>::new(
            pb,
            annotation_prefix,
            Self {
                ap,
                packed_payload,
                timestamp: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                root_initial: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                root: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                pc_addr: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                cpu_state: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                pc_addr_initial: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                cpu_state_initial: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                has_accepted: variable::<FieldT<RamT>, pb_variable>::default(),
                all_unpacked_vars: pb_variable_array::<FieldT<RamT>, RamT::PB>::default(),
                unpack_payload: RcCell::new(
                    multipacking_gadgets::<FieldT<RamT>, RamT::PB>::default(),
                ),
            },
        );
        _self.update_all_vars();
        _self
    }
}

impl<RamT: ram_params_type> MessageVariableConfig for ram_pcd_message_variable<RamT> {
    type FieldT = FieldT<RamT>;
    type PB = RamT::PB;
    type Output = ram_pcd_message<RamT>;
    fn get_message(&self) -> RcCell<ram_pcd_messages<RamT>> {
        panic!("");
    }
}
impl<RamT: ram_params_type> MessageVariableConfig for ram_pcd_message_variables<RamT> {
    type FieldT = FieldT<RamT>;
    type PB = RamT::PB;
    type Output = ram_pcd_message<RamT>;
    fn get_message(&self) -> RcCell<ram_pcd_messages<RamT>> {
        let type_val = self.pb.borrow().val(&self.t.types).as_ulong();
        let timestamp_val = self
            .t
            .t
            .timestamp
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        let root_initial_val = self.t.t.root_initial.get_bits(&self.pb);
        let root_val = self.t.t.root.get_bits(&self.pb);
        let pc_addr_val = self
            .t
            .t
            .pc_addr
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        let cpu_state_val = self.t.t.cpu_state.get_bits(&self.pb);
        let pc_addr_initial_val = self
            .t
            .t
            .pc_addr_initial
            .get_field_element_from_bits(&self.pb)
            .as_ulong();
        let cpu_state_initial_val = self.t.t.cpu_state_initial.get_bits(&self.pb);
        let has_accepted_val =
            (self.pb.borrow().val(&self.t.t.has_accepted) == FieldT::<RamT>::one());

        let mut result = RcCell::new(ram_pcd_message::<RamT>::new(
            type_val,
            self.t.t.ap.clone(),
            timestamp_val,
            root_initial_val,
            root_val,
            pc_addr_val,
            cpu_state_val,
            pc_addr_initial_val,
            cpu_state_initial_val,
            has_accepted_val,
        ));
        result
    }
}
impl<RamT: ram_params_type> ram_pcd_message_variables<RamT> {
    pub fn allocate_unpacked_part(&mut self) {
        let digest_size = CRH_with_bit_out_gadgets::<FieldT<RamT>, RamT::PB>::get_digest_len();

        self.t.t.timestamp.allocate(
            &self.pb,
            RamT::timestamp_length,
            prefix_format!(self.annotation_prefix, " timestamp"),
        );
        self.t.t.root_initial.allocate(
            &self.pb,
            digest_size,
            prefix_format!(self.annotation_prefix, " root_initial"),
        );
        self.t.t.root.allocate(
            &self.pb,
            digest_size,
            prefix_format!(self.annotation_prefix, " root"),
        );
        self.t.t.pc_addr.allocate(
            &self.pb,
            self.t.t.ap.address_size(),
            prefix_format!(self.annotation_prefix, " pc_addr"),
        );
        self.t.t.cpu_state.allocate(
            &self.pb,
            self.t.t.ap.cpu_state_size(),
            prefix_format!(self.annotation_prefix, " cpu_state"),
        );
        self.t.t.pc_addr_initial.allocate(
            &self.pb,
            self.t.t.ap.address_size(),
            prefix_format!(self.annotation_prefix, " pc_addr_initial"),
        );
        self.t.t.cpu_state_initial.allocate(
            &self.pb,
            self.t.t.ap.cpu_state_size(),
            prefix_format!(self.annotation_prefix, " cpu_state_initial"),
        );
        self.t.t.has_accepted.allocate(
            &self.pb,
            prefix_format!(self.annotation_prefix, " has_accepted"),
        );
        let mut all_unpacked_vars =
            pb_linear_combination_array::<FieldT<RamT>, RamT::PB>::default();
        all_unpacked_vars.extend(self.t.t.timestamp.clone().into());
        all_unpacked_vars.extend(self.t.t.root_initial.clone().into());
        all_unpacked_vars.extend(self.t.t.root.clone().into());
        all_unpacked_vars.extend(self.t.t.pc_addr.clone().into());
        all_unpacked_vars.extend(self.t.t.cpu_state.clone().into());
        all_unpacked_vars.extend(self.t.t.pc_addr_initial.clone().into());
        all_unpacked_vars.extend(self.t.t.cpu_state_initial.clone().into());
        all_unpacked_vars
            .contents
            .push(self.t.t.has_accepted.clone().into());

        self.t.t.unpack_payload = RcCell::new(multipacking_gadget::<FieldT<RamT>, RamT::PB>::new(
            self.pb.clone(),
            all_unpacked_vars,
            self.t.t.packed_payload.clone().into(),
            FieldT::<RamT>::capacity(),
            prefix_format!(self.annotation_prefix, " unpack_payload"),
        ));
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        self.t
            .t
            .unpack_payload
            .borrow()
            .generate_r1cs_witness_from_bits();
    }

    pub fn generate_r1cs_witness_from_packed(&self) {
        self.t
            .t
            .unpack_payload
            .borrow()
            .generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_constraints(&self) {
        self.t
            .t
            .unpack_payload
            .borrow()
            .generate_r1cs_constraints(true);
    }
}
pub type ram_pcd_local_datas<RamT> = r1cs_pcd_local_data<FieldT<RamT>, ram_pcd_local_data<RamT>>;
impl<RamT: ram_params_type> ram_pcd_local_data<RamT> {
    pub fn new(
        is_halt_case: bool,
        mem: delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>,
        aux: ram_input_tape,
    ) -> ram_pcd_local_datas<RamT> {
        r1cs_pcd_local_data::<FieldT<RamT>, Self>::new(Self {
            is_halt_case,
            mem,
            aux,
        })
    }
}
impl<RamT: ram_params_type> LocalDataConfig for ram_pcd_local_data<RamT> {
    type FieldT = FieldT<RamT>;
    fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<FieldT<RamT>> {
        let mut result = r1cs_variable_assignment::<FieldT<RamT>>::default();
        result.push(if self.is_halt_case {
            FieldT::<RamT>::one()
        } else {
            FieldT::<RamT>::zero()
        });
        result
    }
}
pub type ram_pcd_local_data_variables<RamT> =
    r1cs_pcd_local_data_variables<ram_pcd_local_data_variable<RamT>>;
impl<RamT: ram_params_type> ram_pcd_local_data_variable<RamT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT<RamT>, RamT::PB>>,
        annotation_prefix: String,
    ) -> ram_pcd_local_data_variables<RamT> {
        let mut is_halt_case = variable::<FieldT<RamT>, pb_variable>::default();
        is_halt_case.allocate(&pb, prefix_format!(annotation_prefix, " is_halt_case"));

        let mut _self =
            r1cs_pcd_local_data_variable::<Self>::new(pb, annotation_prefix, Self { is_halt_case });
        _self.update_all_vars();
        _self
    }
}

impl<RamT: ram_params_type> LocalDataVariableConfig for ram_pcd_local_data_variable<RamT> {
    type FieldT = FieldT<RamT>;
    type PB = RamT::PB;
    type Output = ram_pcd_local_datas<RamT>;
    fn get_local_data(
        &self,
    ) -> RcCell<r1cs_pcd_local_data<FieldT<RamT>, ram_pcd_local_datas<RamT>>> {
        panic!("");
    }
}
/*
  We need to perform the following checks:

  Always:
  next.root_initial = cur.root_initial
  next.pc_addr_init = cur.pc_addr_initial
  next.cpu_state_initial = cur.cpu_state_initial

  If is_is_base_case = 1: (base case)
  that cur.timestamp = 0, cur.cpu_state = cpu_state_init, cur.pc_addr = pc_addr_initial, cur.has_accepted = 0
  that cur.root = cur.root_initial

  If do_halt = 0: (regular case)
  that instruction fetch was correctly executed
  next.timestamp = cur.timestamp + 1
  that CPU accepted on (cur, temp)
  that load-then-store was correctly handled
  that next.root = temp.root, next.cpu_state = temp.cpu_state, next.pc_addr = temp.pc_addr

  If do_halt = 1: (final case)
  that cur.has_accepted = 1
  that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
  that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
*/

pub type ram_compliance_predicate_handlers<RamT> = compliance_predicate_handler<
    <RamT as ram_params_type>::CPH,
    ram_compliance_predicate_handler<RamT>,
>;
//<protoboard_type = protoboard<<RamT as ram_params_type>::base_field_type, <RamT as ppTConfig>::PB>>
impl<RamT: ram_params_type> ram_compliance_predicate_handler<RamT> {
    pub fn new(ap: ram_architecture_params<RamT>) -> ram_compliance_predicate_handlers<RamT> {
        // TODO: assert that message has fields of lengths consistent with num_addresses/value_size (as a method for ram_message)
        // choose a constant for timestamp_len
        // check that value_size <= digest_size; digest_size is not assumed to fit in chunk size (more precisely, it is handled correctly in the other gadgets).
        // check if others fit (timestamp_length, value_size, addr_size)

        // the variables allocated are: next, cur, local data (nil for us), is_base_case, witness
        let addr_size = ap.address_size();
        let value_size = ap.value_size();
        let digest_size = CRH_with_bit_out_gadgets::<FieldT<RamT>, RamT::PB>::get_digest_len();
        let pbt = ram_protoboard::<RamT>::new_with_ap::<ram_architecture_params<RamT>>(ap.clone());
        let pb = pbt.clone().into_p();
        let outgoing_message = RcCell::new(ram_pcd_message_variable::<RamT>::new(
            pb.clone(),
            ap.clone(),
            "outgoing_message".to_owned(),
        ));
        let mut arity = variable::<FieldT<RamT>, pb_variable>::default();
        arity.allocate(&pb, "arity");
        let incoming_messages = vec![RcCell::new(ram_pcd_message_variable::<RamT>::new(
            pb.clone(),
            ap.clone(),
            "incoming_message".to_owned(),
        ))];
        let local_data = RcCell::new(ram_pcd_local_data_variable::<RamT>::new(
            pb.clone(),
            "local_data".to_owned(),
        ));
        let mut is_base_case = variable::<FieldT<RamT>, pb_variable>::default();
        is_base_case.allocate(&pb, "is_base_case");

        let mut next = outgoing_message.clone();
        let mut cur = incoming_messages[0].clone();

        next.borrow_mut().allocate_unpacked_part();
        cur.borrow_mut().allocate_unpacked_part();

        // work-around for bad linear combination handling
        let mut zero = variable::<FieldT<RamT>, pb_variable>::default();
        zero.allocate(&pb, "zero"); // will go away when we properly support linear terms

        let mut temp_next_pc_addr = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        let mut temp_next_cpu_state = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        temp_next_pc_addr.allocate(&pb, addr_size, "temp_next_pc_addr");
        temp_next_cpu_state.allocate(&pb, ap.cpu_state_size(), "temp_next_cpu_state");

        let chunk_size = FieldT::<RamT>::capacity();

        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        let copy_root_initial = RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.clone(),
            cur.borrow().t.t.root_initial.clone(),
            next.borrow().t.t.root_initial.clone(),
            variable::<FieldT<RamT>, pb_variable>::from(ONE).into(),
            chunk_size,
            "copy_root_initial".to_owned(),
        ));
        let copy_pc_addr_initial =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                cur.borrow().t.t.pc_addr_initial.clone(),
                next.borrow().t.t.pc_addr_initial.clone(),
                variable::<FieldT<RamT>, pb_variable>::from(ONE).into(),
                chunk_size,
                "copy_pc_addr_initial".to_owned(),
            ));
        let copy_cpu_state_initial =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                cur.borrow().t.t.cpu_state_initial.clone(),
                next.borrow().t.t.cpu_state_initial.clone(),
                variable::<FieldT<RamT>, pb_variable>::from(ONE).into(),
                chunk_size,
                "copy_cpu_state_initial".to_owned(),
            ));

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        let mut packed_cur_timestamp = variable::<FieldT<RamT>, pb_variable>::default();
        packed_cur_timestamp.allocate(&pb, "packed_cur_timestamp");
        let pack_cur_timestamp = RcCell::new(packing_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.clone(),
            cur.borrow().t.t.timestamp.clone().into(),
            packed_cur_timestamp.clone().into(),
            "pack_cur_timestamp".to_owned(),
        ));

        let zero_cpu_state =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);
        let zero_pc_addr =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);

        let initialize_cur_cpu_state =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                cur.borrow().t.t.cpu_state_initial.clone(),
                cur.borrow().t.t.cpu_state.clone(),
                is_base_case.clone().into(),
                chunk_size,
                "initialize_cur_cpu_state".to_owned(),
            ));
        let initialize_prev_pc_addr =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                cur.borrow().t.t.pc_addr_initial.clone(),
                cur.borrow().t.t.pc_addr.clone(),
                is_base_case.clone().into(),
                chunk_size,
                "initialize_prev_pc_addr".to_owned(),
            ));

        let initialize_root = RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.clone(),
            cur.borrow().t.t.root_initial.clone(),
            cur.borrow().t.t.root.clone(),
            is_base_case.clone().into(),
            chunk_size,
            "initialize_root".to_owned(),
        ));
        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, next)
          that load-then-store was correctly handled
        */
        let mut is_not_halt_case = variable::<FieldT<RamT>, pb_variable>::default();
        is_not_halt_case.allocate(&pb, "is_not_halt_case");
        // for performing instruction fetch
        let mut prev_pc_val =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);
        prev_pc_val.allocate(&pb, value_size, "prev_pc_val");
        let prev_pc_val_digest = RcCell::new(digest_variable::<FieldT<RamT>, RamT::PB>::new2(
            pb.clone(),
            digest_size,
            prev_pc_val.clone(),
            zero.clone(),
            "prev_pc_val_digest".to_owned(),
        ));
        let cur_root_digest = RcCell::new(digest_variable::<FieldT<RamT>, RamT::PB>::new2(
            pb.clone(),
            digest_size,
            cur.borrow().t.t.root.clone(),
            zero.clone(),
            "cur_root_digest".to_owned(),
        ));
        let instruction_fetch_merkle_proof = RcCell::new(merkle_authentication_path_variable::<
            FieldT<RamT>,
            RamT::PB,
            HashT<RamT>,
        >::new(
            pb.clone(),
            addr_size,
            "instruction_fetch_merkle_proof".to_owned(),
        ));
        let instruction_fetch =
            RcCell::new(
                memory_load_gadget::<FieldT<RamT>, RamT::PB, HashT<RamT>>::new(
                    pb.clone(),
                    addr_size,
                    cur.borrow().t.t.pc_addr.clone().into(),
                    prev_pc_val_digest.borrow().clone(),
                    cur_root_digest.borrow().clone(),
                    instruction_fetch_merkle_proof.borrow().clone(),
                    variable::<FieldT<RamT>, pb_variable>::from(ONE).into(),
                    "instruction_fetch".to_owned(),
                ),
            );

        // for next.timestamp = cur.timestamp + 1
        let mut packed_next_timestamp = variable::<FieldT<RamT>, pb_variable>::default();
        packed_next_timestamp.allocate(&pb, "packed_next_timestamp");
        let pack_next_timestamp = RcCell::new(packing_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.clone(),
            next.borrow().t.t.timestamp.clone().into(),
            packed_next_timestamp.clone().into(),
            "pack_next_timestamp".to_owned(),
        ));

        // that CPU accepted on (cur, temp)
        let mut ls_addr =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);
        let mut ls_prev_val =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);
        let mut ls_next_val =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                cur.borrow().t.t.pc_addr.len()
            ]);
        ls_addr.allocate(&pb, addr_size, "ls_addr");
        ls_prev_val.allocate(&pb, value_size, "ls_prev_val");
        ls_next_val.allocate(&pb, value_size, "ls_next_val");
        let cpu_checker = RcCell::new(ram_cpu_checker::<RamT>::new(
            pb.clone(),
            cur.borrow().t.t.pc_addr.clone(),
            prev_pc_val.clone(),
            cur.borrow().t.t.cpu_state.clone(),
            ls_addr.clone(),
            ls_prev_val.clone(),
            ls_next_val.clone(),
            temp_next_cpu_state.clone(),
            temp_next_pc_addr.clone(),
            next.borrow().t.t.has_accepted.clone(),
            "cpu_checker".to_owned(),
        ));

        // that load-then-store was correctly handled
        let ls_prev_val_digest = RcCell::new(digest_variable::<FieldT<RamT>, RamT::PB>::new2(
            pb.clone(),
            digest_size,
            ls_prev_val.clone(),
            zero.clone(),
            "ls_prev_val_digest".to_owned(),
        ));
        let ls_next_val_digest = RcCell::new(digest_variable::<FieldT<RamT>, RamT::PB>::new2(
            pb.clone(),
            digest_size,
            ls_next_val.clone(),
            zero.clone(),
            "ls_next_val_digest".to_owned(),
        ));
        let next_root_digest = RcCell::new(digest_variable::<FieldT<RamT>, RamT::PB>::new2(
            pb.clone(),
            digest_size,
            next.borrow().t.t.root.clone(),
            zero.clone(),
            "next_root_digest".to_owned(),
        ));
        let load_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<
                FieldT<RamT>,
                RamT::PB,
                HashT<RamT>,
            >::new(
                pb.clone(), addr_size, "load_merkle_proof".to_owned()
            ));
        let store_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<
                FieldT<RamT>,
                RamT::PB,
                HashT<RamT>,
            >::new(
                pb.clone(), addr_size, "store_merkle_proof".to_owned()
            ));
        let load_store_checker = RcCell::new(memory_load_store_gadget::<
            FieldT<RamT>,
            RamT::PB,
            HashT<RamT>,
        >::new(
            pb.clone(),
            addr_size,
            ls_addr.clone(),
            ls_prev_val_digest.borrow().clone(),
            cur_root_digest.borrow().clone(),
            load_merkle_proof.borrow().clone(),
            ls_next_val_digest.borrow().clone(),
            next_root_digest.borrow().clone(),
            store_merkle_proof.borrow().clone(),
            is_not_halt_case.clone().into(),
            "load_store_checker".to_owned(),
        ));
        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        let mut do_halt = variable::<FieldT<RamT>, pb_variable>::default();
        do_halt.allocate(&pb, "do_halt");
        let zero_root =
            pb_variable_array::<FieldT<RamT>, RamT::PB>::new(vec![
                zero.clone();
                next.borrow().t.t.root.len()
            ]);
        let clear_next_root = RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.clone(),
            zero_root.clone(),
            next.borrow().t.t.root.clone(),
            do_halt.clone().into(),
            chunk_size,
            "clear_next_root".to_owned(),
        ));
        let clear_next_pc_addr =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                zero_pc_addr.clone(),
                next.borrow().t.t.pc_addr.clone(),
                do_halt.clone().into(),
                chunk_size,
                "clear_next_pc_addr".to_owned(),
            ));
        let clear_next_cpu_state =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                zero_cpu_state.clone(),
                next.borrow().t.t.cpu_state.clone(),
                do_halt.clone().into(),
                chunk_size,
                "clear_cpu_state".to_owned(),
            ));

        let copy_temp_next_pc_addr =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                temp_next_pc_addr.clone(),
                next.borrow().t.t.pc_addr.clone(),
                is_not_halt_case.clone().into(),
                chunk_size,
                "copy_temp_next_pc_addr".to_owned(),
            ));
        let copy_temp_next_cpu_state =
            RcCell::new(bit_vector_copy_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.clone(),
                temp_next_cpu_state.clone(),
                next.borrow().t.t.cpu_state.clone(),
                is_not_halt_case.clone().into(),
                chunk_size,
                "copy_temp_next_cpu_state".to_owned(),
            ));

        compliance_predicate_handler::<RamT::CPH, Self>::new(
            RcCell::new(pbt),
            100,
            1,
            1,
            true,
            BTreeSet::from([1]),
            Self {
                ap: ap.clone(),
                next,
                cur,
                zero,
                copy_root_initial,
                copy_pc_addr_initial,
                copy_cpu_state_initial,
                is_base_case,
                is_not_halt_case,
                packed_cur_timestamp,
                pack_cur_timestamp,
                packed_next_timestamp,
                pack_next_timestamp,
                zero_cpu_state,
                zero_pc_addr,
                zero_root,
                initialize_cur_cpu_state,
                initialize_prev_pc_addr,
                initialize_root,
                prev_pc_val,
                prev_pc_val_digest,
                cur_root_digest,
                instruction_fetch_merkle_proof,
                instruction_fetch,
                next_root_digest,
                ls_addr,
                ls_prev_val,
                ls_next_val,
                ls_prev_val_digest,
                ls_next_val_digest,
                load_merkle_proof,
                store_merkle_proof,
                load_store_checker,
                temp_next_pc_addr,
                temp_next_cpu_state,
                cpu_checker,
                do_halt,
                clear_next_root,
                clear_next_pc_addr,
                clear_next_cpu_state,
                copy_temp_next_root: RcCell::new(
                    bit_vector_copy_gadgets::<FieldT<RamT>, RamT::PB>::default(),
                ),
                copy_temp_next_pc_addr,
                copy_temp_next_cpu_state,
                addr_size: ap.address_size(),
                value_size: ap.value_size(),
                digest_size: CRH_with_bit_out_gadgets::<FieldT<RamT>, RamT::PB>::get_digest_len(),
                message_length: 0,
            },
        )
    }
    pub fn get_final_case_msg(
        ap: &ram_architecture_params<RamT>,
        primary_input: &ram_boot_trace,
        time_bound: usize,
    ) -> RcCell<r1cs_pcd_message<FieldT<RamT>, ram_pcd_message<RamT>>> {
        enter_block(
            "Call to ram_compliance_predicate_handler::get_final_case_msg",
            false,
        );
        let num_addresses = 1usize << ap.address_size();
        let value_size = ap.value_size();
        let mem = delegated_ra_memory::<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>::new3(
            num_addresses,
            value_size,
            primary_input.as_memory_contents(),
        );

        let types = 1;

        let timestamp = time_bound;

        let root_initial = mem.get_root();
        let pc_addr_initial = ap.initial_pc_addr();
        let cpu_state_initial = vec![false; ap.cpu_state_size()];

        let root = vec![false; root_initial.len()];
        let pc_addr = 0;
        let cpu_state = cpu_state_initial.clone();

        let has_accepted = true;

        let mut result = RcCell::new(ram_pcd_message::<RamT>::new(
            types,
            ap.clone(),
            timestamp,
            root_initial,
            root,
            pc_addr,
            cpu_state,
            pc_addr_initial,
            cpu_state_initial,
            has_accepted,
        ));
        leave_block(
            "Call to ram_compliance_predicate_handler::get_final_case_msg",
            false,
        );

        result
    }
}
impl<RamT: ram_params_type> ram_compliance_predicate_handlers<RamT> {
    pub fn generate_r1cs_constraints(&self) {
        print_indent();
        print!(
            "* Message size: {}\n",
            self.t.next.borrow().t.all_vars.len()
        );
        print_indent();
        print!("* Address size: {}\n", self.t.addr_size);
        print_indent();
        print!("* CPU state size: {}\n", self.t.ap.cpu_state_size());
        print_indent();
        print!("* Digest size: {}\n", self.t.digest_size);

        PROFILE_CONSTRAINTST(&self.pb, "handle next_type, arity and cur_type");
        {
            generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb.borrow().clone().into_p(),
                &(self.t.next.borrow().t.types.clone().into()),
                &FieldT::<RamT>::one(),
                "next_type".to_owned(),
            );
            generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb.borrow().clone().into_p(),
                &(self.arity.clone().into()),
                &FieldT::<RamT>::one(),
                "arity".to_owned(),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                    self.t.is_base_case.clone().into(),
                    self.t.cur.borrow().t.types.clone().into(),
                    FieldT::<RamT>::from(0).into(),
                ),
                "nonzero_cur_type_implies_base_case_0".to_owned(),
            );
            generate_boolean_r1cs_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb.borrow().clone().into_p(),
                &(self.t.cur.borrow().t.types.clone().into()),
                "cur_type_boolean".to_owned(),
            );
            generate_boolean_r1cs_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb.borrow().clone().into_p(),
                &(self.t.is_base_case.clone().into()),
                "is_base_case_boolean".to_owned(),
            );
        }

        PROFILE_CONSTRAINTST(&self.pb, "unpack messages");
        {
            self.t.next.borrow().generate_r1cs_constraints();
            self.t.cur.borrow().generate_r1cs_constraints();
        }

        // work-around for bad linear combination handling
        generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb.borrow().clone().into_p(),
            &(self.t.zero.clone().into()),
            &FieldT::<RamT>::zero(),
            " zero".to_owned(),
        );

        /* recall that Booleanity of PCD messages has already been enforced by the PCD machine, which is explains the absence of Booleanity checks */
        /*
          We need to perform the following checks:

          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        PROFILE_CONSTRAINTST(&self.pb, "copy root_initial");
        {
            self.t
                .copy_root_initial
                .borrow()
                .generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTST(&self.pb, "copy pc_addr_initial and cpu_state_initial");
        {
            self.t
                .copy_pc_addr_initial
                .borrow()
                .generate_r1cs_constraints(false, false);
            self.t
                .copy_cpu_state_initial
                .borrow()
                .generate_r1cs_constraints(false, false);
        }

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        self.t
            .pack_cur_timestamp
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.is_base_case.clone().into(),
                self.t.packed_cur_timestamp.clone().into(),
                FieldT::<RamT>::from(0).into(),
            ),
            "clear_ts_on_is_base_case".to_owned(),
        );
        PROFILE_CONSTRAINTST(&self.pb, "copy cur_cpu_state and prev_pc_addr");
        {
            self.t
                .initialize_cur_cpu_state
                .borrow()
                .generate_r1cs_constraints(false, false);
            self.t
                .initialize_prev_pc_addr
                .borrow()
                .generate_r1cs_constraints(false, false);
        }
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.is_base_case.clone().into(),
                self.t.cur.borrow().t.t.has_accepted.clone().into(),
                FieldT::<RamT>::from(0).into(),
            ),
            "is_base_case_is_not_accepting".to_owned(),
        );
        PROFILE_CONSTRAINTST(&self.pb, "initialize root");
        {
            self.t
                .initialize_root
                .borrow()
                .generate_r1cs_constraints(false, false);
        }

        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, next)
          that load-then-store was correctly handled
          that next.root = temp.root, next.cpu_state = temp.cpu_state, next.pc_addr = temp.pc_addr
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                FieldT::<RamT>::from(1).into(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.do_halt.clone(),
                self.t.is_not_halt_case.clone().into(),
            ),
            "is_not_halt_case".to_owned(),
        );
        PROFILE_CONSTRAINTST(&self.pb, "instruction fetch");
        {
            self.t
                .instruction_fetch_merkle_proof
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .instruction_fetch
                .borrow()
                .generate_r1cs_constraints();
        }
        self.t
            .pack_next_timestamp
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.is_not_halt_case.clone().into(),
                (self.t.packed_cur_timestamp.clone()
                    + linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                        FieldT::<RamT>::from(1),
                    ))
                    - self.t.packed_next_timestamp.clone(),
                FieldT::<RamT>::from(0).into(),
            ),
            "increment_timestamp".to_owned(),
        );
        PROFILE_CONSTRAINTST(&self.pb, "CPU checker");
        {
            self.t.cpu_checker.borrow().generate_r1cs_constraints();
        }
        PROFILE_CONSTRAINTST(&self.pb, "load/store checker");
        {
            // See comment in merkle_tree_check_update_gadget::generate_r1cs_witness() for why we don't need to call store_merkle_proof.generate_r1cs_constraints()
            self.t
                .load_merkle_proof
                .borrow()
                .generate_r1cs_constraints();
            self.t
                .load_store_checker
                .borrow()
                .generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTST(&self.pb, "copy temp_next_pc_addr and temp_next_cpu_state");
        {
            self.t
                .copy_temp_next_pc_addr
                .borrow()
                .generate_r1cs_constraints(true, false);
            self.t
                .copy_temp_next_cpu_state
                .borrow()
                .generate_r1cs_constraints(true, false);
        }

        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.do_halt.clone().into(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.cur.borrow().t.t.has_accepted.clone(),
                FieldT::<RamT>::from(0).into(),
            ),
            "final_case_must_be_accepting".to_owned(),
        );

        PROFILE_CONSTRAINTST(&self.pb, "clear next root");
        {
            self.t
                .clear_next_root
                .borrow()
                .generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTST(&self.pb, "clear next_pc_addr and next_cpu_state");
        {
            self.t
                .clear_next_pc_addr
                .borrow()
                .generate_r1cs_constraints(false, false);
            self.t
                .clear_next_cpu_state
                .borrow()
                .generate_r1cs_constraints(false, false);
        }

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.do_halt.clone().into(),
                self.t.packed_cur_timestamp.clone()
                    - linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                        self.t.packed_next_timestamp.clone(),
                    ),
                FieldT::<RamT>::from(0).into(),
            ),
            "equal_ts_on_halt".to_owned(),
        );

        let accounted = PRINT_CONSTRAINT_PROFILING();
        let total = self.pb.borrow().num_constraints();
        print_indent();
        print!("* Unaccounted constraints: {}\n", total - accounted);
        print_indent();
        print!(
            "* Number of constraints in ram_compliance_predicate: {}\n",
            total
        );
    }

    pub fn generate_r1cs_witness(
        &mut self,
        incoming_message_values: &Vec<RcCell<r1cs_pcd_message<FieldT<RamT>, RamT::M>>>,
        local_data_value: &RcCell<r1cs_pcd_local_data<FieldT<RamT>, RamT::LD>>,
    ) {
        let ram_local_data_value = local_data_value.clone();
        assert!(
            ram_local_data_value
                .borrow()
                .t
                .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
                .num_addresses
                == 1usize << self.t.addr_size
        ); // check value_size and num_addresses too

        self.generate_r1cs_witness_base(incoming_message_values, local_data_value);
        self.t.cur.borrow().generate_r1cs_witness_from_packed();

        *self.pb.borrow_mut().val_ref(&self.t.next.borrow().t.types) = FieldT::<RamT>::one();
        *self.pb.borrow_mut().val_ref(&self.arity) = FieldT::<RamT>::one();
        *self.pb.borrow_mut().val_ref(&self.t.is_base_case) =
            (if self.pb.borrow().val(&self.t.cur.borrow().t.types) == FieldT::<RamT>::zero() {
                FieldT::<RamT>::one()
            } else {
                FieldT::<RamT>::zero()
            });

        *self.pb.borrow_mut().val_ref(&self.t.zero) = FieldT::<RamT>::zero();
        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        self.t.copy_root_initial.borrow().generate_r1cs_witness();
        for i in 0..self.t.next.borrow().t.t.root_initial.len() {
            self.pb
                .borrow()
                .val(&self.t.cur.borrow().t.t.root_initial[i])
                .print();
            self.pb
                .borrow()
                .val(&self.t.next.borrow().t.t.root_initial[i])
                .print();
            assert!(
                self.pb
                    .borrow()
                    .val(&self.t.cur.borrow().t.t.root_initial[i])
                    == self
                        .pb
                        .borrow()
                        .val(&self.t.next.borrow().t.t.root_initial[i])
            );
        }

        self.t.copy_pc_addr_initial.borrow().generate_r1cs_witness();
        self.t
            .copy_cpu_state_initial
            .borrow()
            .generate_r1cs_witness();

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        let base_case = (0 == incoming_message_values[0].borrow().types);
        *self.pb.borrow_mut().val_ref(&self.t.is_base_case) = if base_case {
            FieldT::<RamT>::one()
        } else {
            FieldT::<RamT>::zero()
        };

        self.t
            .initialize_cur_cpu_state
            .borrow()
            .generate_r1cs_witness();
        self.t
            .initialize_prev_pc_addr
            .borrow()
            .generate_r1cs_witness();

        if base_case {
            *self.pb.borrow_mut().val_ref(&self.t.packed_cur_timestamp) = FieldT::<RamT>::zero();
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.cur.borrow().t.t.has_accepted) = FieldT::<RamT>::zero();
            self.t
                .pack_cur_timestamp
                .borrow()
                .generate_r1cs_witness_from_packed();
        } else {
            self.t
                .pack_cur_timestamp
                .borrow()
                .generate_r1cs_witness_from_bits();
        }

        self.t.initialize_root.borrow().generate_r1cs_witness();

        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, temp)
          that load-then-store was correctly handled
        */
        *self.pb.borrow_mut().val_ref(&self.t.do_halt) =
            if ram_local_data_value.borrow().t.is_halt_case() {
                FieldT::<RamT>::one()
            } else {
                FieldT::<RamT>::zero()
            };
        *self.pb.borrow_mut().val_ref(&self.t.is_not_halt_case) =
            FieldT::<RamT>::one() - self.pb.borrow().val(&self.t.do_halt);

        // that instruction fetch was correctly executed
        let int_pc_addr = convert_bit_vector_to_field_element::<FieldT<RamT>>(
            &self
                .t
                .cur
                .borrow()
                .t
                .t
                .pc_addr
                .get_bits(&self.pb.borrow().clone().into_p()),
        )
        .as_ulong();
        let int_pc_val = ram_local_data_value
            .borrow()
            .t
            .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
            .get_value(int_pc_addr);
        // #ifdef DEBUG
        print!(
            "pc_addr (in units) = {}, pc_val = {} (0x{:08x})\n",
            int_pc_addr, int_pc_val, int_pc_val
        );
        //#endif
        let mut pc_val_bv = int_list_to_bits(&[int_pc_val], self.t.value_size);
        pc_val_bv.reverse();

        self.t
            .prev_pc_val
            .fill_with_bits(&self.pb.borrow().clone().into_p(), &pc_val_bv);
        let pc_path = ram_local_data_value
            .borrow()
            .t
            .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
            .get_path(int_pc_addr);
        self.t
            .instruction_fetch_merkle_proof
            .borrow()
            .generate_r1cs_witness(int_pc_addr, pc_path);
        self.t.instruction_fetch.borrow().generate_r1cs_witness();

        // next.timestamp = cur.timestamp + 1 (or cur.timestamp if do_halt)
        *self.pb.borrow_mut().val_ref(&self.t.packed_next_timestamp) =
            self.pb.borrow().val(&self.t.packed_cur_timestamp)
                + self.pb.borrow().val(&self.t.is_not_halt_case);
        self.t
            .pack_next_timestamp
            .borrow()
            .generate_r1cs_witness_from_packed();

        // that CPU accepted on (cur, temp)
        // Step 1: Get address and old witnesses for delegated memory.
        self.t.cpu_checker.borrow().generate_r1cs_witness_address();
        let int_ls_addr = self
            .t
            .ls_addr
            .get_field_element_from_bits(&self.pb.borrow().clone().into_p())
            .as_ulong();
        let int_ls_prev_val = ram_local_data_value
            .borrow()
            .t
            .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
            .get_value(int_ls_addr) as usize;
        let prev_path = ram_local_data_value
            .borrow()
            .t
            .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
            .get_path(int_ls_addr);
        self.t
            .ls_prev_val
            .fill_with_bits_of_ulong(&self.pb.borrow().clone().into_p(), int_ls_prev_val as u64);
        assert!(
            self.t
                .ls_prev_val
                .get_field_element_from_bits(&self.pb.borrow().clone().into_p())
                == FieldT::<RamT>::from(int_ls_prev_val) //, true
        );
        // Step 2: Execute CPU checker and delegated memory
        self.t
            .cpu_checker
            .borrow()
            .generate_r1cs_witness_other(&ram_local_data_value.borrow().t.aux());
        // #ifdef DEBUG
        print!("Debugging information from transition function:\n");
        self.t.cpu_checker.borrow().dump();
        //#endif
        let int_ls_next_val = self
            .t
            .ls_next_val
            .get_field_element_from_bits(&self.pb.borrow().clone().into_p())
            .as_ulong();
        ram_local_data_value
            .borrow()
            .t
            .mem::<delegated_ra_memorys<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>>()
            .set_value(int_ls_addr, int_ls_next_val);
        // #ifdef DEBUG
        print!(
            "Memory location {} changed from {} (0x{:08x}) to {} (0x{:08x})\n",
            int_ls_addr, int_ls_prev_val, int_ls_prev_val, int_ls_next_val, int_ls_next_val
        );
        //#endif
        // Step 4: Use both to satisfy load_store_checker
        self.t
            .load_merkle_proof
            .borrow()
            .generate_r1cs_witness(int_ls_addr, prev_path);
        self.t.load_store_checker.borrow().generate_r1cs_witness();

        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */

        // Order matters here: both witness maps touch next_root, but the
        // one that does not set values must be executed the last, so its
        // auxiliary variables are filled in correctly according to values
        // actually set by the other witness map.
        if self.pb.borrow().val(&self.t.do_halt).is_zero() {
            self.t
                .copy_temp_next_pc_addr
                .borrow()
                .generate_r1cs_witness();
            self.t
                .copy_temp_next_cpu_state
                .borrow()
                .generate_r1cs_witness();

            self.t.clear_next_root.borrow().generate_r1cs_witness();
            self.t.clear_next_pc_addr.borrow().generate_r1cs_witness();
            self.t.clear_next_cpu_state.borrow().generate_r1cs_witness();
        } else {
            self.t.clear_next_root.borrow().generate_r1cs_witness();
            self.t.clear_next_pc_addr.borrow().generate_r1cs_witness();
            self.t.clear_next_cpu_state.borrow().generate_r1cs_witness();

            self.t
                .copy_temp_next_pc_addr
                .borrow()
                .generate_r1cs_witness();
            self.t
                .copy_temp_next_cpu_state
                .borrow()
                .generate_r1cs_witness();
        }

        // #ifdef DEBUG
        print!("next.has_accepted: ");
        self.pb
            .borrow()
            .val(&self.t.next.borrow().t.t.has_accepted)
            .print();
        //#endif

        self.t.next.borrow().generate_r1cs_witness_from_bits();
    }

    pub fn get_base_case_message(
        ap: &ram_architecture_params<RamT>,
        primary_input: &ram_boot_trace,
    ) -> RcCell<r1cs_pcd_message<FieldT<RamT>, ram_pcd_message<RamT>>> {
        enter_block(
            "Call to ram_compliance_predicate_handler::get_base_case_message",
            false,
        );
        let num_addresses = 1usize << ap.address_size();
        let value_size = ap.value_size();
        let mem = delegated_ra_memory::<CRH_with_bit_out_gadgets<FieldT<RamT>, RamT::PB>>::new3(
            num_addresses,
            value_size,
            primary_input.as_memory_contents(),
        );

        let types = 0;

        let timestamp = 0;

        let root_initial = mem.get_root();
        let pc_addr_initial = ap.initial_pc_addr();
        let cpu_state_initial = vec![false; ap.cpu_state_size()];

        let root = root_initial.clone();
        let pc_addr = pc_addr_initial;
        let cpu_state = cpu_state_initial.clone();

        let has_accepted = false;

        let result = RcCell::new(ram_pcd_message::<RamT>::new(
            types,
            ap.clone(),
            timestamp,
            root_initial,
            root,
            pc_addr,
            cpu_state,
            pc_addr_initial,
            cpu_state_initial,
            has_accepted,
        ));
        leave_block(
            "Call to ram_compliance_predicate_handler::get_base_case_message",
            false,
        );
        result
    }
}
