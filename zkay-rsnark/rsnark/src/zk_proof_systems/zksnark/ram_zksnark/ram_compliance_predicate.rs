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

use crate::gadgetlib1::gadgets::basic_gadgets::{
    bit_vector_copy_gadget, generate_boolean_r1cs_constraint,
    generate_r1cs_equals_const_constraint, multipacking_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_gadget::memory_load_gadget;
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_store_gadget::memory_load_store_gadget;
use crate::gadgetlib1::gadgets::hashes::crh_gadget::CRH_with_bit_out_gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io::digest_variable;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable::merkle_authentication_path_variable;
use crate::gadgetlib1::pb_variable::{ONE, pb_variable, pb_variable_array};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::relations::ram_computations::memory::delegated_ra_memory::delegated_ra_memory;
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_base_field, ram_boot_trace, ram_cpu_checker, ram_input_tape,
    ram_protoboard,
};
use crate::relations::variable::variable;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_message;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler::{
    compliance_predicate_handler, r1cs_pcd_local_data_variable, r1cs_pcd_message_variable,
    r1cs_pcd_message_variables,
};
use crate::zk_proof_systems::zksnark::ram_zksnark::ram_zksnark_params::{
    ram_zksnark_PCD_pp, ram_zksnark_architecture_params,
};
use ffec::field_utils::field_utils::{
    convert_bit_vector_to_field_element, convert_field_element_to_bit_vector,
    pack_bit_vector_into_field_element_vector,
};
use ffec::{bit_vector, div_ceil, int_list_to_bits, log2};
use rccell::RcCell;

use crate::gadgetlib1::constraint_profiling::{PRINT_CONSTRAINT_PROFILING, PROFILE_CONSTRAINTS};
use crate::gadgetlib1::protoboard::protoboard;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate::r1cs_pcd_local_data;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use std::collections::BTreeSet;

/**
 * A RAM message specializes the generic PCD message, in order to
 * obtain a more user-friendly print method.
 */
//
type FieldT<ramT> = ram_base_field<ramT>;

#[derive(Default, Clone)]
pub struct ram_pcd_message<ramT> {
    // : public r1cs_pcd_message<ram_base_field<ramT> >
    // pub fn bit_vector) const;
    pub ap: ram_architecture_params<ramT>,

    pub timestamp: usize,
    pub root_initial: bit_vector,
    pub root: bit_vector,
    pub pc_addr: usize,
    pub cpu_state: bit_vector,
    pub pc_addr_initial: usize,
    pub cpu_state_initial: bit_vector,
    pub has_accepted: bool,
}

//
// type FieldT=ram_base_field<ramT>;
#[derive(Default, Clone)]
pub struct ram_pcd_message_variable<ramT> {
    //  : public r1cs_pcd_message_variable<ram_base_field<ramT> >
    pub ap: ram_architecture_params<ramT>,

    pub packed_payload: pb_variable_array<FieldT>,

    pub timestamp: pb_variable_array<FieldT>,
    pub root_initial: pb_variable_array<FieldT>,
    pub root: pb_variable_array<FieldT>,
    pub pc_addr: pb_variable_array<FieldT>,
    pub cpu_state: pb_variable_array<FieldT>,
    pub pc_addr_initial: pb_variable_array<FieldT>,
    pub cpu_state_initial: pb_variable_array<FieldT>,
    pub has_accepted: variable<FieldT, pb_variable>,

    pub all_unpacked_vars: pb_variable_array<FieldT>,

    pub unpack_payload: RcCell<multipacking_gadget<FieldT>>,
}

//
// type FieldT=ram_base_field<ramT>;
#[derive(Default, Clone)]
pub struct ram_pcd_local_data<ramT> {
    // : public r1cs_pcd_local_data<ram_base_field<ramT> >
    pub is_halt_case: bool,

    pub mem: delegated_ra_memory<CRH_with_bit_out_gadget<FieldT>>,
    pub aux_it: ram_input_tape<ramT>::const_iterator,
    pub aux_end: ram_input_tape<ramT>::const_iterator,
}

//
// type FieldT=ram_base_field<ramT>;
#[derive(Default, Clone)]
pub struct ram_pcd_local_data_variable<ramT> {
    // : public r1cs_pcd_local_data_variable<ram_base_field<ramT> >
    pub is_halt_case: variable<FieldT, pb_variable>,
}

/**
 * A RAM compliance predicate.
 */
//
//    type FieldT=ram_base_field<ramT>;
type HashT<FieldT> = CRH_with_bit_out_gadget<FieldT>;
type base_handler<ramT> = compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT>>;

#[derive(Default, Clone)]
pub struct ram_compliance_predicate_handler<ramT> {
    // : public compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> >
    pub ap: ram_architecture_params<ramT>,

    pub next: RcCell<ram_pcd_message_variable<ramT>>,
    pub cur: RcCell<ram_pcd_message_variable<ramT>>,
    pub zero: variable<FieldT, pb_variable>, // TODO: promote linear combinations to first pub struct objects
    pub copy_root_initial: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub copy_pc_addr_initial: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub copy_cpu_state_initial: RcCell<bit_vector_copy_gadget<FieldT>>,

    pub is_base_case: variable<FieldT, pb_variable>,
    pub is_not_halt_case: variable<FieldT, pb_variable>,

    pub packed_cur_timestamp: variable<FieldT, pb_variable>,
    pub pack_cur_timestamp: RcCell<packing_gadget<FieldT>>,
    pub packed_next_timestamp: variable<FieldT, pb_variable>,
    pub pack_next_timestamp: RcCell<packing_gadget<FieldT>>,

    pub zero_cpu_state: pb_variable_array<FieldT>,
    pub zero_pc_addr: pb_variable_array<FieldT>,
    pub zero_root: pb_variable_array<FieldT>,

    pub initialize_cur_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub initialize_prev_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,

    pub initialize_root: RcCell<bit_vector_copy_gadget<FieldT>>,

    pub prev_pc_val: pb_variable_array<FieldT>,
    pub prev_pc_val_digest: RcCell<digest_variable<FieldT>>,
    pub cur_root_digest: RcCell<digest_variable<FieldT>>,
    pub instruction_fetch_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    pub instruction_fetch: RcCell<memory_load_gadget<FieldT, HashT>>,

    pub next_root_digest: RcCell<digest_variable<FieldT>>,

    pub ls_addr: pb_variable_array<FieldT>,
    pub ls_prev_val: pb_variable_array<FieldT>,
    pub ls_next_val: pb_variable_array<FieldT>,
    pub ls_prev_val_digest: RcCell<digest_variable<FieldT>>,
    pub ls_next_val_digest: RcCell<digest_variable<FieldT>>,
    pub load_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    pub store_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    pub load_store_checker: RcCell<memory_load_store_gadget<FieldT, HashT>>,

    pub temp_next_pc_addr: pb_variable_array<FieldT>,
    pub temp_next_cpu_state: pb_variable_array<FieldT>,
    pub cpu_checker: RcCell<ram_cpu_checker<ramT>>,

    pub do_halt: variable<FieldT, pb_variable>,
    pub clear_next_root: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub clear_next_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub clear_next_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,

    pub copy_temp_next_root: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub copy_temp_next_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,
    pub copy_temp_next_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,

    pub addr_size: usize,
    pub value_size: usize,
    pub digest_size: usize,

    pub message_length: usize,
}

// use crate::gadgetlib1::constraint_profiling;

impl<ramT> ram_pcd_message<ramT> {
    pub fn new(
        types: usize,
        ap: ram_architecture_params<ramT>,
        timestamp: usize,
        root_initial: bit_vector,
        root: bit_vector,
        pc_addr: usize,
        cpu_state: bit_vector,
        pc_addr_initial: usize,
        cpu_state_initial: bit_vector,
        has_accepted: bool,
    ) -> r1cs_pcd_message<FieldT, Self> {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();
        assert!(log2(timestamp) < ramT::timestamp_length);
        assert!(root_initial.len() == digest_size);
        assert!(root.len() == digest_size);
        assert!(log2(pc_addr) < ap.address_size());
        assert!(cpu_state.len() == ap.cpu_state_size());
        assert!(log2(pc_addr_initial) < ap.address_size());
        assert!(cpu_state_initial.len() == ap.cpu_state_size());
        r1cs_pcd_message::<FieldT, Self>::new(
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

        let timestamp_bits = convert_field_element_to_bit_vector::<FieldT>(
            FieldT::from(self.timestamp),
            ramT::timestamp_length,
        );
        let pc_addr_bits = convert_field_element_to_bit_vector::<FieldT>(
            FieldT::from(self.pc_addr),
            self.ap.address_size(),
        );
        let pc_addr_initial_bits = convert_field_element_to_bit_vector::<FieldT>(
            FieldT::from(self.pc_addr_initial),
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

        assert!(result.len() == unpacked_payload_size_in_bits(self.ap));
        result
    }

    pub fn payload_as_r1cs_variable_assignment() -> r1cs_variable_assignment<ram_base_field<ramT>> {
        let payload_bits = unpacked_payload_as_bits();
        let result = pack_bit_vector_into_field_element_vector::<FieldT>(payload_bits);
        result
    }

    pub fn print_bits(bv: &bit_vector) {
        for b in &bv {
            print!("{}", b as u8);
        }
        print!("\n");
    }

    pub fn print(&self) {
        print!("ram_pcd_message:\n");
        print!("  type: {}\n", self.types);
        print!("  timestamp: {}\n", self.timestamp);
        print!("  root_initial: ");
        print_bits(self.root_initial);
        print!("  root: ");
        print_bits(self.root);
        print!("  pc_addr: {}\n", self.pc_addr);
        print!("  cpu_state: ");
        print_bits(self.cpu_state);
        print!("  pc_addr_initial: {}\n", self.pc_addr_initial);
        print!("  cpu_state_initial: ");
        print_bits(self.cpu_state_initial);
        print!(
            "  has_accepted: {}\n",
            if self.has_accepted { "YES" } else { "no" }
        );
    }

    pub fn unpacked_payload_size_in_bits(ap: &ram_architecture_params<ramT>) -> usize {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();

        (ramT::timestamp_length + // timestamp
            2*digest_size + // root, root_initial
            2*ap.address_size() + // pc_addr, pc_addr_initial
            2*ap.cpu_state_size() + // cpu_state, cpu_state_initial
            1) // has_accepted
    }
}

impl<ramT> ram_pcd_message_variable<ramT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT>>,
        ap: ram_architecture_params<ramT>,
        annotation_prefix: &String,
    ) -> r1cs_pcd_message_variables<ram_base_field<ramT>, Self> {
        let unpacked_payload_size_in_bits =
            ram_pcd_message::<ramT>::unpacked_payload_size_in_bits(ap);
        let packed_payload_size = div_ceil(unpacked_payload_size_in_bits, FieldT::capacity());
        let mut packed_payload = pb_variable_array::<FieldT>::default();
        packed_payload.allocate(
            &pb,
            packed_payload_size,
            prefix_format!(annotation_prefix, " packed_payload"),
        );

        let _self = r1cs_pcd_message_variable::<ram_base_field<ramT>, Self>::new(
            pb,
            annotation_prefix,
            Self { ap },
        );
        _self.update_all_vars();
        _self
    }

    pub fn allocate_unpacked_part(&mut self) {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();

        self.timestamp.allocate(
            self.pb,
            ramT::timestamp_length,
            prefix_format!(self.annotation_prefix, " timestamp"),
        );
        self.root_initial.allocate(
            self.pb,
            digest_size,
            prefix_format!(self.annotation_prefix, " root_initial"),
        );
        self.root.allocate(
            self.pb,
            digest_size,
            prefix_format!(self.annotation_prefix, " root"),
        );
        self.pc_addr.allocate(
            self.pb,
            self.ap.address_size(),
            prefix_format!(self.annotation_prefix, " pc_addr"),
        );
        self.cpu_state.allocate(
            self.pb,
            self.ap.cpu_state_size(),
            prefix_format!(self.annotation_prefix, " cpu_state"),
        );
        self.pc_addr_initial.allocate(
            self.pb,
            self.ap.address_size(),
            prefix_format!(self.annotation_prefix, " pc_addr_initial"),
        );
        self.cpu_state_initial.allocate(
            self.pb,
            self.ap.cpu_state_size(),
            prefix_format!(self.annotation_prefix, " cpu_state_initial"),
        );
        self.has_accepted.allocate(
            self.pb,
            prefix_format!(self.annotation_prefix, " has_accepted"),
        );
        let mut all_unpacked_vars = vec![];
        all_unpacked_vars.extend(&self.timestamp);
        all_unpacked_vars.extend(&self.root_initial);
        all_unpacked_vars.extend(&self.root);
        all_unpacked_vars.extend(&self.pc_addr);
        all_unpacked_vars.extend(&self.cpu_state);
        all_unpacked_vars.extend(&self.pc_addr_initial);
        all_unpacked_vars.extend(&self.cpu_state_initial);
        all_unpacked_vars.push(self.has_accepted);

        self.unpack_payload = RcCell::new(multipacking_gadget::<FieldT>::new(
            self.pb,
            all_unpacked_vars,
            self.packed_payload,
            FieldT::capacity(),
            prefix_format!(self.annotation_prefix, " unpack_payload"),
        ));
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        self.unpack_payload.generate_r1cs_witness_from_bits();
    }

    pub fn generate_r1cs_witness_from_packed(&self) {
        self.unpack_payload.generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_constraints(&self) {
        self.unpack_payload.generate_r1cs_constraints(true);
    }

    pub fn get_message(&self) -> RcCell<r1cs_pcd_message<ram_base_field<ramT>>> {
        let type_val = self.pb.borrow().val(&self.types).as_ulong();
        let timestamp_val = self
            .timestamp
            .get_field_element_from_bits(self.pb)
            .as_ulong();
        let root_initial_val = self.root_initial.get_bits(self.pb);
        let root_val = self.root.get_bits(self.pb);
        let pc_addr_val = self.pc_addr.get_field_element_from_bits(self.pb).as_ulong();
        let cpu_state_val = self.cpu_state.get_bits(self.pb);
        let pc_addr_initial_val = self
            .pc_addr_initial
            .get_field_element_from_bits(self.pb)
            .as_ulong();
        let cpu_state_initial_val = self.cpu_state_initial.get_bits(self.pb);
        let has_accepted_val = (self.pb.borrow().val(&self.has_accepted) == FieldT::one());

        let mut result = RcCell::new(ram_pcd_message::<ramT>::new(
            type_val,
            self.ap,
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

impl<ramT> ram_pcd_local_data<ramT> {
    pub fn new(
        is_halt_case: bool,
        mem: delegated_ra_memory<CRH_with_bit_out_gadget<FieldT>>,
        aux_it: ram_input_tape<ramT>::const_iterator,
        aux_end: &ram_input_tape<ramT>::const_iterator,
    ) -> Self {
        Self {
            is_halt_case,
            mem,
            aux_it,
            aux_end,
        }
    }

    pub fn as_r1cs_variable_assignment(&self) -> r1cs_variable_assignment<ram_base_field<ramT>> {
        let mut result = r1cs_variable_assignment::<FieldT>::default();
        result.push(if self.is_halt_case {
            FieldT::one()
        } else {
            FieldT::zero()
        });
        result
    }
}
impl<ramT> ram_pcd_local_data_variable<ramT> {
    pub fn new(pb: RcCell<protoboard<FieldT>>, annotation_prefix: &String) -> Self {
        let mut is_halt_case = variable::<FieldT, pb_variable>::default();
        is_halt_case.allocate(&pb, prefix_format!(annotation_prefix, " is_halt_case"));

        let _self = r1cs_pcd_local_data_variable::<ram_base_field<ramT>, Self>::new(
            pb,
            annotation_prefix,
            Self {},
        );
        _self.update_all_vars();
        _self
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
impl<ramT> ram_compliance_predicate_handler<ramT> {
    pub fn new(ap: ram_architecture_params<ramT>) -> Self {
        // TODO: assert that message has fields of lengths consistent with num_addresses/value_size (as a method for ram_message)
        // choose a constant for timestamp_len
        // check that value_size <= digest_size; digest_size is not assumed to fit in chunk size (more precisely, it is handled correctly in the other gadgets).
        // check if others fit (timestamp_length, value_size, addr_size)

        // the variables allocated are: next, cur, local data (nil for us), is_base_case, witness
        let addr_size = ap.address_size();
        let value_size = ap.value_size();
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();
        let pb = ram_protoboard::<ramT>(ap);
        let outgoing_message = RcCell::new(ram_pcd_message_variable::<ramT>::new(
            pb.clone(),
            ap,
            "outgoing_message",
        ));
        let mut arity = variable::<FieldT, pb_variable>::default();
        arity.allocate(pb.clone(), "arity");
        let incoming_messages = vec![RcCell::new(ram_pcd_message_variable::<ramT>::new(
            pb.clone(),
            ap,
            "incoming_message",
        ))];
        let local_data = RcCell::new(ram_pcd_local_data_variable::<ramT>::new(
            pb.clone(),
            "local_data",
        ));
        let mut is_base_case = variable::<FieldT, pb_variable>::default();
        is_base_case.allocate(pb.clone(), "is_base_case");

        let mut next = ram_pcd_message_variable::<ramT>::new(outgoing_message);
        let mut cur = ram_pcd_message_variable::<ramT>::new(incoming_messages[0].clone());

        next.allocate_unpacked_part();
        cur.allocate_unpacked_part();

        // work-around for bad linear combination handling
        let mut zero = variable::<FieldT, pb_variable>::default();
        zero.allocate(pb.clone(), "zero"); // will go away when we properly support linear terms

        let mut temp_next_pc_addr = pb_variable_array::<FieldT>::default();
        let mut temp_next_cpu_state = pb_variable_array::<FieldT>::default();
        temp_next_pc_addr.allocate(pb.clone(), addr_size, "temp_next_pc_addr");
        temp_next_cpu_state.allocate(pb.clone(), ap.cpu_state_size(), "temp_next_cpu_state");

        let chunk_size = FieldT::capacity();

        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        let copy_root_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.root_initial,
            next.root_initial,
            variable::<FieldT, pb_variable>::from(ONE),
            chunk_size,
            "copy_root_initial",
        ));
        let copy_pc_addr_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.pc_addr_initial,
            next.pc_addr_initial,
            variable::<FieldT, pb_variable>::from(ONE),
            chunk_size,
            "copy_pc_addr_initial",
        ));
        let copy_cpu_state_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.cpu_state_initial,
            next.cpu_state_initial,
            variable::<FieldT, pb_variable>::from(ONE),
            chunk_size,
            "copy_cpu_state_initial",
        ));

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        let mut packed_cur_timestamp = variable::<FieldT, pb_variable>::default();
        packed_cur_timestamp.allocate(pb.clone(), "packed_cur_timestamp");
        let pack_cur_timestamp = RcCell::new(packing_gadget::<FieldT>::new(
            pb.clone(),
            cur.timestamp,
            packed_cur_timestamp,
            "pack_cur_timestamp",
        ));

        let zero_cpu_state = pb_variable_array::<FieldT>(cur.cpu_state.len(), zero);
        let zero_pc_addr = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);

        let initialize_cur_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.cpu_state_initial,
            cur.cpu_state,
            is_base_case,
            chunk_size,
            "initialize_cur_cpu_state",
        ));
        let initialize_prev_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.pc_addr_initial,
            cur.pc_addr,
            is_base_case,
            chunk_size,
            "initialize_prev_pc_addr",
        ));

        let initialize_root = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            cur.root_initial,
            cur.root,
            is_base_case,
            chunk_size,
            "initialize_root",
        ));
        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, next)
          that load-then-store was correctly handled
        */
        let mut is_not_halt_case = variable::<FieldT, pb_variable>::default();
        is_not_halt_case.allocate(pb.clone(), "is_not_halt_case");
        // for performing instruction fetch
        let prev_pc_val = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);
        prev_pc_val.allocate(pb.clone(), value_size, "prev_pc_val");
        let prev_pc_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            pb.clone(),
            digest_size,
            prev_pc_val,
            zero,
            "prev_pc_val_digest",
        ));
        let cur_root_digest = RcCell::new(digest_variable::<FieldT>::new(
            pb.clone(),
            digest_size,
            cur.root,
            zero,
            "cur_root_digest",
        ));
        let instruction_fetch_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
                pb.clone(),
                addr_size,
                "instruction_fetch_merkle_proof",
            ));
        let instruction_fetch = RcCell::new(memory_load_gadget::<FieldT, HashT>::new(
            pb.clone(),
            addr_size,
            cur.pc_addr,
            prev_pc_val_digest,
            cur_root_digest,
            instruction_fetch_merkle_proof,
            variable::<FieldT, pb_variable>::from(ONE),
            "instruction_fetch",
        ));

        // for next.timestamp = cur.timestamp + 1
        let mut packed_next_timestamp = variable::<FieldT, pb_variable>::default();
        packed_next_timestamp.allocate(pb.clone(), "packed_next_timestamp");
        let pack_next_timestamp = RcCell::new(packing_gadget::<FieldT>::new(
            pb.clone(),
            next.timestamp,
            packed_next_timestamp,
            "pack_next_timestamp",
        ));

        // that CPU accepted on (cur, temp)
        let ls_addr = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);
        let ls_prev_val = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);
        let ls_next_val = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);
        ls_addr.allocate(pb.clone(), addr_size, "ls_addr");
        ls_prev_val.allocate(pb.clone(), value_size, "ls_prev_val");
        ls_next_val.allocate(pb.clone(), value_size, "ls_next_val");
        let cpu_checker = RcCell::new(ram_cpu_checker::<ramT>::new(
            pb.clone(),
            cur.pc_addr,
            prev_pc_val,
            cur.cpu_state,
            ls_addr,
            ls_prev_val,
            ls_next_val,
            temp_next_cpu_state,
            temp_next_pc_addr,
            next.has_accepted,
            "cpu_checker",
        ));

        // that load-then-store was correctly handled
        let ls_prev_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            pb.clone(),
            digest_size,
            ls_prev_val,
            zero,
            "ls_prev_val_digest",
        ));
        let ls_next_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            pb.clone(),
            digest_size,
            ls_next_val,
            zero,
            "ls_next_val_digest",
        ));
        let next_root_digest = RcCell::new(digest_variable::<FieldT>::new(
            pb.clone(),
            digest_size,
            next.root,
            zero,
            "next_root_digest",
        ));
        let load_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
                pb.clone(),
                addr_size,
                "load_merkle_proof",
            ));
        let store_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
                pb.clone(),
                addr_size,
                "store_merkle_proof",
            ));
        let load_store_checker = RcCell::new(memory_load_store_gadget::<FieldT, HashT>::new(
            pb.clone(),
            addr_size,
            ls_addr,
            ls_prev_val_digest,
            cur_root_digest,
            load_merkle_proof,
            ls_next_val_digest,
            next_root_digest,
            store_merkle_proof,
            is_not_halt_case,
            "load_store_checker",
        ));
        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        let mut do_halt = variable::<FieldT, pb_variable>::default();
        do_halt.allocate(pb.clone(), "do_halt");
        let zero_root = pb_variable_array::<FieldT>(next.root.len(), zero);
        let clear_next_root = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            zero_root,
            next.root,
            do_halt,
            chunk_size,
            "clear_next_root",
        ));
        let clear_next_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            zero_pc_addr,
            next.pc_addr,
            do_halt,
            chunk_size,
            "clear_next_pc_addr",
        ));
        let clear_next_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            zero_cpu_state,
            next.cpu_state,
            do_halt,
            chunk_size,
            "clear_cpu_state",
        ));

        let copy_temp_next_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            temp_next_pc_addr,
            next.pc_addr,
            is_not_halt_case,
            chunk_size,
            "copy_temp_next_pc_addr",
        ));
        let copy_temp_next_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            pb.clone(),
            temp_next_cpu_state,
            next.cpu_state,
            is_not_halt_case,
            chunk_size,
            "copy_temp_next_cpu_state",
        ));

        compliance_predicate_handler::<ram_base_field<ramT>, ram_protoboard<ramT>, Self>::new(
            pb,
            100,
            1,
            1,
            true,
            BTreeSet::from([1]),
            Self {
                ap,
                addr_size: ap.address_size(),
                value_size: ap.value_size(),
                digest_size: CRH_with_bit_out_gadget::<FieldT>::get_digest_len(),
            },
        )
    }

    pub fn generate_r1cs_constraints(&self) {
        print_indent();
        print!("* Message size: {}\n", self.next.all_vars.len());
        print_indent();
        print!("* Address size: {}\n", self.addr_size);
        print_indent();
        print!("* CPU state size: {}\n", self.ap.cpu_state_size());
        print_indent();
        print!("* Digest size: {}\n", self.digest_size);

        PROFILE_CONSTRAINTS(self.pb, "handle next_type, arity and cur_type");
        {
            generate_r1cs_equals_const_constraint::<FieldT>(
                self.pb,
                self.next.types,
                FieldT::one(),
                "next_type",
            );
            generate_r1cs_equals_const_constraint::<FieldT>(
                self.pb,
                self.arity,
                FieldT::one(),
                "arity",
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT>(self.is_base_case, self.cur.types, 0),
                "nonzero_cur_type_implies_base_case_0",
            );
            generate_boolean_r1cs_constraint::<FieldT>(self.pb, self.cur.types, "cur_type_boolean");
            generate_boolean_r1cs_constraint::<FieldT>(
                self.pb,
                self.is_base_case,
                "is_base_case_boolean",
            );
        }

        PROFILE_CONSTRAINTS(self.pb, "unpack messages");
        {
            self.next.generate_r1cs_constraints();
            self.cur.generate_r1cs_constraints();
        }

        // work-around for bad linear combination handling
        generate_r1cs_equals_const_constraint::<FieldT>(
            self.pb,
            self.zero,
            FieldT::zero(),
            " zero",
        );

        /* recall that Booleanity of PCD messages has already been enforced by the PCD machine, which is explains the absence of Booleanity checks */
        /*
          We need to perform the following checks:

          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        PROFILE_CONSTRAINTS(self.pb, "copy root_initial");
        {
            self.copy_root_initial
                .generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTS(self.pb, "copy pc_addr_initial and cpu_state_initial");
        {
            self.copy_pc_addr_initial
                .generate_r1cs_constraints(false, false);
            self.copy_cpu_state_initial
                .generate_r1cs_constraints(false, false);
        }

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        self.pack_cur_timestamp.generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(self.is_base_case, self.packed_cur_timestamp, 0),
            "clear_ts_on_is_base_case",
        );
        PROFILE_CONSTRAINTS(self.pb, "copy cur_cpu_state and prev_pc_addr");
        {
            self.initialize_cur_cpu_state
                .generate_r1cs_constraints(false, false);
            self.initialize_prev_pc_addr
                .generate_r1cs_constraints(false, false);
        }
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(self.is_base_case, self.cur.has_accepted, 0),
            "is_base_case_is_not_accepting",
        );
        PROFILE_CONSTRAINTS(self.pb, "initialize root");
        {
            self.initialize_root.generate_r1cs_constraints(false, false);
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
            r1cs_constraint::<FieldT>(1, 1 - self.do_halt, self.is_not_halt_case),
            "is_not_halt_case",
        );
        PROFILE_CONSTRAINTS(self.pb, "instruction fetch");
        {
            self.instruction_fetch_merkle_proof
                .generate_r1cs_constraints();
            self.instruction_fetch.generate_r1cs_constraints();
        }
        self.pack_next_timestamp.generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(
                self.is_not_halt_case,
                (self.packed_cur_timestamp + 1) - self.packed_next_timestamp,
                0,
            ),
            "increment_timestamp",
        );
        PROFILE_CONSTRAINTS(self.pb, "CPU checker");
        {
            self.cpu_checker.generate_r1cs_constraints();
        }
        PROFILE_CONSTRAINTS(self.pb, "load/store checker");
        {
            // See comment in merkle_tree_check_update_gadget::generate_r1cs_witness() for why we don't need to call store_merkle_proof.generate_r1cs_constraints()
            self.load_merkle_proof.generate_r1cs_constraints();
            self.load_store_checker.generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTS(self.pb, "copy temp_next_pc_addr and temp_next_cpu_state");
        {
            self.copy_temp_next_pc_addr
                .generate_r1cs_constraints(true, false);
            self.copy_temp_next_cpu_state
                .generate_r1cs_constraints(true, false);
        }

        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(self.do_halt, 1 - self.cur.has_accepted, 0),
            "final_case_must_be_accepting",
        );

        PROFILE_CONSTRAINTS(self.pb, "clear next root");
        {
            self.clear_next_root.generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTS(self.pb, "clear next_pc_addr and next_cpu_state");
        {
            self.clear_next_pc_addr
                .generate_r1cs_constraints(false, false);
            self.clear_next_cpu_state
                .generate_r1cs_constraints(false, false);
        }

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(
                self.do_halt,
                self.packed_cur_timestamp - self.packed_next_timestamp,
                0,
            ),
            "equal_ts_on_halt",
        );

        let accounted = PRINT_CONSTRAINT_PROFILING();
        let total = self.pb.num_constraints();
        print_indent();
        print!("* Unaccounted constraints: {}\n", total - accounted);
        print_indent();
        print!(
            "* Number of constraints in ram_compliance_predicate: {}\n",
            total
        );
    }

    pub fn generate_r1cs_witness(
        &self,
        incoming_message_values: &Vec<RcCell<r1cs_pcd_message<FieldT>>>,
        local_data_value: &RcCell<r1cs_pcd_local_data<FieldT>>,
    ) {
        let ram_local_data_value = ram_pcd_local_data::<ramT>(local_data_value);
        assert!(ram_local_data_value.mem.num_addresses == 1u64 << self.addr_size); // check value_size and num_addresses too

        base_handler::generate_r1cs_witness(incoming_message_values, local_data_value);
        self.cur.generate_r1cs_witness_from_packed();

        self.pb.borrow().val(&self.next.types) = FieldT::one();
        self.pb.borrow().val(&self.arity) = FieldT::one();
        self.pb.borrow().val(&self.is_base_case) =
            (if self.pb.borrow().val(&self.cur.types) == FieldT::zero() {
                FieldT::one()
            } else {
                FieldT::zero()
            });

        self.pb.borrow().val(&self.zero) = FieldT::zero();
        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        self.copy_root_initial.generate_r1cs_witness();
        for i in 0..self.next.root_initial.len() {
            self.pb.borrow().val(&self.cur.root_initial[i]).print();
            self.pb.borrow().val(&self.next.root_initial[i]).print();
            assert!(
                self.pb.borrow().val(&self.cur.root_initial[i])
                    == self.pb.borrow().val(&self.next.root_initial[i])
            );
        }

        self.copy_pc_addr_initial.generate_r1cs_witness();
        self.copy_cpu_state_initial.generate_r1cs_witness();

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        let base_case = (self.incoming_message_values[0].types == 0);
        self.pb.borrow().val(&self.is_base_case) = if base_case {
            FieldT::one()
        } else {
            FieldT::zero()
        };

        self.initialize_cur_cpu_state.generate_r1cs_witness();
        self.initialize_prev_pc_addr.generate_r1cs_witness();

        if self.base_case {
            self.pb.borrow().val(&self.packed_cur_timestamp) = FieldT::zero();
            self.pb.borrow().val(&self.cur.has_accepted) = FieldT::zero();
            self.pack_cur_timestamp.generate_r1cs_witness_from_packed();
        } else {
            self.pack_cur_timestamp.generate_r1cs_witness_from_bits();
        }

        self.initialize_root.generate_r1cs_witness();

        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, temp)
          that load-then-store was correctly handled
        */
        self.pb.borrow().val(&self.do_halt) = if self.ram_local_data_value.is_halt_case {
            FieldT::one()
        } else {
            FieldT::zero()
        };
        self.pb.borrow().val(&self.is_not_halt_case) =
            FieldT::one() - self.pb.borrow().val(&self.do_halt);

        // that instruction fetch was correctly executed
        let int_pc_addr =
            convert_bit_vector_to_field_element::<FieldT>(self.cur.pc_addr.get_bits(self.pb))
                .as_ulong();
        let int_pc_val = self.ram_local_data_value.mem.get_value(int_pc_addr);
        // #ifdef DEBUG
        print!(
            "pc_addr (in units) = {}, pc_val = {} (0x{:08x})\n",
            int_pc_addr, int_pc_val, int_pc_val
        );
        //#endif
        let mut pc_val_bv = int_list_to_bits({ int_pc_val }, self.value_size);
        pc_val_bv.reverse();

        self.prev_pc_val.fill_with_bits(self.pb, pc_val_bv);
        let pc_path = self.ram_local_data_value.mem.get_path(int_pc_addr);
        self.instruction_fetch_merkle_proof
            .generate_r1cs_witness(int_pc_addr, pc_path);
        self.instruction_fetch.generate_r1cs_witness();

        // next.timestamp = cur.timestamp + 1 (or cur.timestamp if do_halt)
        self.pb.borrow().val(&self.packed_next_timestamp) =
            self.pb.borrow().val(&self.packed_cur_timestamp)
                + self.pb.borrow().val(&self.is_not_halt_case);
        self.pack_next_timestamp.generate_r1cs_witness_from_packed();

        // that CPU accepted on (cur, temp)
        // Step 1: Get address and old witnesses for delegated memory.
        self.cpu_checker.generate_r1cs_witness_address();
        let int_ls_addr = self.ls_addr.get_field_element_from_bits(self.pb).as_ulong();
        let int_ls_prev_val = self.ram_local_data_value.mem.get_value(int_ls_addr);
        let prev_path = self.ram_local_data_value.mem.get_path(int_ls_addr);
        self.ls_prev_val
            .fill_with_bits_of_ulong(self.pb, int_ls_prev_val);
        assert!(
            self.ls_prev_val.get_field_element_from_bits(self.pb)
                == FieldT::from(int_ls_prev_val, true)
        );
        // Step 2: Execute CPU checker and delegated memory
        self.cpu_checker.generate_r1cs_witness_other(
            self.ram_local_data_value.aux_it,
            self.ram_local_data_value.aux_end,
        );
        // #ifdef DEBUG
        print!("Debugging information from transition function:\n");
        self.cpu_checker.dump();
        //#endif
        let int_ls_next_val = self
            .ls_next_val
            .get_field_element_from_bits(self.pb)
            .as_ulong();
        self.ram_local_data_value
            .mem
            .set_value(int_ls_addr, int_ls_next_val);
        // #ifdef DEBUG
        print!(
            "Memory location {} changed from {} (0x{:08x}) to {} (0x{:08x})\n",
            int_ls_addr, int_ls_prev_val, int_ls_prev_val, int_ls_next_val, int_ls_next_val
        );
        //#endif
        // Step 4: Use both to satisfy load_store_checker
        self.load_merkle_proof
            .generate_r1cs_witness(int_ls_addr, prev_path);
        self.load_store_checker.generate_r1cs_witness();

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
        if self.pb.borrow().val(&self.do_halt).is_zero() {
            self.copy_temp_next_pc_addr.generate_r1cs_witness();
            self.copy_temp_next_cpu_state.generate_r1cs_witness();

            self.clear_next_root.generate_r1cs_witness();
            self.clear_next_pc_addr.generate_r1cs_witness();
            self.clear_next_cpu_state.generate_r1cs_witness();
        } else {
            self.clear_next_root.generate_r1cs_witness();
            self.clear_next_pc_addr.generate_r1cs_witness();
            self.clear_next_cpu_state.generate_r1cs_witness();

            self.copy_temp_next_pc_addr.generate_r1cs_witness();
            self.copy_temp_next_cpu_state.generate_r1cs_witness();
        }

        // #ifdef DEBUG
        print!("next.has_accepted: ");
        self.pb.borrow().val(&self.next.has_accepted).print();
        //#endif

        self.next.generate_r1cs_witness_from_bits();
    }

    pub fn get_base_case_message(
        &self,
        ap: &ram_architecture_params<ramT>,
        primary_input: &ram_boot_trace<ramT>,
    ) -> RcCell<r1cs_pcd_message<ram_base_field<ramT>>> {
        enter_block("Call to ram_compliance_predicate_handler::get_base_case_message");
        let num_addresses = 1u64 << ap.address_size();
        let value_size = ap.value_size();
        let mem = delegated_ra_memory::<CRH_with_bit_out_gadget<FieldT>>::new(
            num_addresses,
            value_size,
            primary_input.as_memory_contents(),
        );

        let types = 0;

        let timestamp = 0;

        let root_initial = mem.get_root();
        let pc_addr_initial = ap.initial_pc_addr();
        let cpu_state_initial = vec![false; ap.cpu_state_size()];

        let root = root_initial;
        let pc_addr = pc_addr_initial;
        let cpu_state = cpu_state_initial;

        let has_accepted = false;

        let result = RcCell::new(ram_pcd_message::<ramT>::new(
            types,
            ap,
            timestamp,
            root_initial,
            root,
            pc_addr,
            cpu_state,
            pc_addr_initial,
            cpu_state_initial,
            has_accepted,
        ));
        leave_block("Call to ram_compliance_predicate_handler::get_base_case_message");
        result
    }

    pub fn get_final_case_msg(
        &self,
        ap: &ram_architecture_params<ramT>,
        primary_input: &ram_boot_trace<ramT>,
        time_bound: usize,
    ) -> RcCell<r1cs_pcd_message<ram_base_field<ramT>>> {
        enter_block("Call to ram_compliance_predicate_handler::get_final_case_msg");
        let num_addresses = 1u64 << ap.address_size();
        let value_size = ap.value_size();
        let mem = delegated_ra_memory::<CRH_with_bit_out_gadget<FieldT>>::new()(
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
        let cpu_state = cpu_state_initial;

        let has_accepted = true;

        let mut result = r1cs_pcd_message::<FieldT>::new();
        result = RcCell::new(ram_pcd_message::<ramT>::new(
            types,
            ap,
            timestamp,
            root_initial,
            root,
            pc_addr,
            cpu_state,
            pc_addr_initial,
            cpu_state_initial,
            has_accepted,
        ));
        leave_block("Call to ram_compliance_predicate_handler::get_final_case_msg");

        result
    }
}
