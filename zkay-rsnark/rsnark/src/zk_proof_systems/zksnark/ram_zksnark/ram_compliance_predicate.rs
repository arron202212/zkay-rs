/** @file
*****************************************************************************

Declaration of interfaces for a compliance predicate for RAM.

The implementation follows, extends, and optimizes the approach described
in \[BCTV14].

Essentially, the RAM's CPU, which is expressed as an R1CS constraint system,
is augmented to obtain another R1CS constraint system that implements a RAM
compliance predicate. This predicate is responsible for checking:
(1) transitions from a CPU state to the next;
(2) correct load/stores; and
(3) corner cases such as the first and last steps of the machine.
The first can be done by suitably embedding the RAM's CPU in the constraint
system. The second can be done by verifying authentication paths for the values
of memory. The third mostly consists of bookkeeping (with some subtleties arising
from the need to not break zero knowledge).

The laying out of R1CS constraints is done via gadgetlib1 (a minimalistic
library for writing R1CS constraint systems).

References:

\[BCTV14]:
"Scalable Zero Knowledge via Cycles of Elliptic Curves",
Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
CRYPTO 2014,
<http://eprint.iacr.org/2014/595>

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RAM_COMPLIANCE_PREDICATE_HPP_
// #define RAM_COMPLIANCE_PREDICATE_HPP_

// use  <numeric>
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_gadget;
use crate::gadgetlib1::gadgets::delegated_ra_memory::memory_load_store_gadget;
use crate::relations::ram_computations::memory::delegated_ra_memory;
use crate::relations::ram_computations::rams::ram_params;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::compliance_predicate;
use crate::zk_proof_systems::pcd::r1cs_pcd::compliance_predicate::cp_handler;

/**
 * A RAM message specializes the generic PCD message, in order to
 * obtain a more user-friendly print method.
 */
//
type FieldT = ram_base_field<ramT>;
pub struct ram_pcd_message<ramT> {
    // : public r1cs_pcd_message<ram_base_field<ramT> >
    // pub fn bit_vector) const;

    //
    ap: ram_architecture_params<ramT>,

    timestamp: usize,
    root_initial: bit_vector,
    root: bit_vector,
    pc_addr: usize,
    cpu_state: bit_vector,
    pc_addr_initial: usize,
    cpu_state_initial: bit_vector,
    has_accepted: bool,
}

//
// type FieldT=ram_base_field<ramT>;
pub struct ram_pcd_message_variable<ramT> {
    //  : public r1cs_pcd_message_variable<ram_base_field<ramT> >
    ap: ram_architecture_params<ramT>,

    packed_payload: pb_variable_array<FieldT>,

    timestamp: pb_variable_array<FieldT>,
    root_initial: pb_variable_array<FieldT>,
    root: pb_variable_array<FieldT>,
    pc_addr: pb_variable_array<FieldT>,
    cpu_state: pb_variable_array<FieldT>,
    pc_addr_initial: pb_variable_array<FieldT>,
    cpu_state_initial: pb_variable_array<FieldT>,
    has_accepted: pb_variable<FieldT>,

    all_unpacked_vars: pb_variable_array<FieldT>,

    unpack_payload: RcCell<multipacking_gadget<FieldT>>,
}

//
// type FieldT=ram_base_field<ramT>;
pub struct ram_pcd_local_data<ramT> {
    // : public r1cs_pcd_local_data<ram_base_field<ramT> >
    is_halt_case: bool,

    mem: delegated_ra_memory<CRH_with_bit_out_gadget<FieldT>>,
    aux_it: ram_input_tape<ramT>::const_iterator,
    aux_end: ram_input_tape<ramT>::const_iterator,
}

//
// type FieldT=ram_base_field<ramT>;
pub struct ram_pcd_local_data_variable<ramT> {
    // : public r1cs_pcd_local_data_variable<ram_base_field<ramT> >
    is_halt_case: pb_variable<FieldT>,
}

/**
 * A RAM compliance predicate.
 */
//
//    type FieldT=ram_base_field<ramT>;
type HashT = CRH_with_bit_out_gadget<FieldT>;
type base_handler = compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT>>;

pub struct ram_compliance_predicate_handler {
    // : public compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> >
    ap: ram_architecture_params<ramT>,

    //
    next: RcCell<ram_pcd_message_variable<ramT>>,
    cur: RcCell<ram_pcd_message_variable<ramT>>,
    //
    zero: pb_variable<FieldT>, // TODO: promote linear combinations to first pub struct objects
    copy_root_initial: RcCell<bit_vector_copy_gadget<FieldT>>,
    copy_pc_addr_initial: RcCell<bit_vector_copy_gadget<FieldT>>,
    copy_cpu_state_initial: RcCell<bit_vector_copy_gadget<FieldT>>,

    is_base_case: pb_variable<FieldT>,
    is_not_halt_case: pb_variable<FieldT>,

    packed_cur_timestamp: pb_variable<FieldT>,
    pack_cur_timestamp: RcCell<packing_gadget<FieldT>>,
    packed_next_timestamp: pb_variable<FieldT>,
    pack_next_timestamp: RcCell<packing_gadget<FieldT>>,

    zero_cpu_state: pb_variable_array<FieldT>,
    zero_pc_addr: pb_variable_array<FieldT>,
    zero_root: pb_variable_array<FieldT>,

    initialize_cur_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,
    initialize_prev_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,

    initialize_root: RcCell<bit_vector_copy_gadget<FieldT>>,

    prev_pc_val: pb_variable_array<FieldT>,
    prev_pc_val_digest: RcCell<digest_variable<FieldT>>,
    cur_root_digest: RcCell<digest_variable<FieldT>>,
    instruction_fetch_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    instruction_fetch: RcCell<memory_load_gadget<FieldT, HashT>>,

    next_root_digest: RcCell<digest_variable<FieldT>>,

    ls_addr: pb_variable_array<FieldT>,
    ls_prev_val: pb_variable_array<FieldT>,
    ls_next_val: pb_variable_array<FieldT>,
    ls_prev_val_digest: RcCell<digest_variable<FieldT>>,
    ls_next_val_digest: RcCell<digest_variable<FieldT>>,
    load_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    store_merkle_proof: RcCell<merkle_authentication_path_variable<FieldT, HashT>>,
    load_store_checker: RcCell<memory_load_store_gadget<FieldT, HashT>>,

    temp_next_pc_addr: pb_variable_array<FieldT>,
    temp_next_cpu_state: pb_variable_array<FieldT>,
    cpu_checker: RcCell<ram_cpu_checker<ramT>>,

    do_halt: pb_variable<FieldT>,
    clear_next_root: RcCell<bit_vector_copy_gadget<FieldT>>,
    clear_next_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,
    clear_next_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,

    copy_temp_next_root: RcCell<bit_vector_copy_gadget<FieldT>>,
    copy_temp_next_pc_addr: RcCell<bit_vector_copy_gadget<FieldT>>,
    copy_temp_next_cpu_state: RcCell<bit_vector_copy_gadget<FieldT>>,

    //
    addr_size: usize,
    value_size: usize,
    digest_size: usize,

    message_length: usize,
}

// use crate::zk_proof_systems::zksnark::ram_zksnark::ram_compliance_predicate;

//#endif // RAM_COMPLIANCE_PREDICATE_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a compliance predicate for RAM.

See ram_compliance_predicate.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef RAM_COMPLIANCE_PREDICATE_TCC_
// #define RAM_COMPLIANCE_PREDICATE_TCC_
use crate::gadgetlib1::constraint_profiling;

impl ram_pcd_message<ramT> {
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
    ) -> Self {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();
        assert!(log2(timestamp) < ramT::timestamp_length);
        assert!(root_initial.len() == digest_size);
        assert!(root.len() == digest_size);
        assert!(log2(pc_addr) < ap.address_size());
        assert!(cpu_state.len() == ap.cpu_state_size());
        assert!(log2(pc_addr_initial) < ap.address_size());
        assert!(cpu_state_initial.len() == ap.cpu_state_size());
        Self {
            // r1cs_pcd_message<FieldT>(types),
            ap,
            timestamp,
            root_initial,
            root,
            pc_addr,
            cpu_state,
            pc_addr_initial,
            cpu_state_initial,
            has_accepted,
        }
    }

    pub fn unpacked_payload_as_bits() -> bit_vector {
        let mut result = vec![];

        let timestamp_bits = convert_field_element_to_bit_vector::<FieldT>(
            FieldT(timestamp),
            ramT::timestamp_length,
        );
        let pc_addr_bits =
            convert_field_element_to_bit_vector::<FieldT>(FieldT(pc_addr), ap.address_size());
        let pc_addr_initial_bits = convert_field_element_to_bit_vector::<FieldT>(
            FieldT(pc_addr_initial),
            ap.address_size(),
        );

        result.extend(&timestamp_bits);
        result.extend(&root_initial);
        result.extend(&root);
        result.extend(&pc_addr_bits);
        result.extend(&cpu_state);
        result.extend(&pc_addr_initial_bits);
        result.extend(&cpu_state_initial);
        result.push(has_accepted);

        assert!(result.len() == unpacked_payload_size_in_bits(ap));
        return result;
    }

    pub fn payload_as_r1cs_variable_assignment() -> r1cs_variable_assignment<ram_base_field<ramT>> {
        let payload_bits = unpacked_payload_as_bits();
        let result = pack_bit_vector_into_field_element_vector::<FieldT>(payload_bits);
        return result;
    }

    pub fn print_bits(bv: &bit_vector) {
        for b in &bv {
            print!("{}", b as u8);
        }
        print!("\n");
    }

    pub fn print() {
        print!("ram_pcd_message:\n");
        print!("  type: {}\n", self.types);
        print!("  timestamp: {}\n", timestamp);
        print!("  root_initial: ");
        print_bits(root_initial);
        print!("  root: ");
        print_bits(root);
        print!("  pc_addr: {}\n", pc_addr);
        print!("  cpu_state: ");
        print_bits(cpu_state);
        print!("  pc_addr_initial: {}\n", pc_addr_initial);
        print!("  cpu_state_initial: ");
        print_bits(cpu_state_initial);
        print!(
            "  has_accepted: {}\n",
            if has_accepted { "YES" } else { "no" }
        );
    }

    pub fn unpacked_payload_size_in_bits(ap: &ram_architecture_params<ramT>) -> usize {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();

        return (ramT::timestamp_length + // timestamp
            2*digest_size + // root, root_initial
            2*ap.address_size() + // pc_addr, pc_addr_initial
            2*ap.cpu_state_size() + // cpu_state, cpu_state_initial
            1); // has_accepted
    }
}

impl ram_pcd_message_variable<ramT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT>>,
        ap: ram_architecture_params<ramT>,
        annotation_prefix: &String,
    ) -> Self {
        let unpacked_payload_size_in_bits =
            ram_pcd_message::<ramT>::unpacked_payload_size_in_bits(ap);
        let packed_payload_size = div_ceil(unpacked_payload_size_in_bits, FieldT::capacity());
        packed_payload.allocate(
            &pb,
            packed_payload_size,
            FMT(annotation_prefix, " packed_payload"),
        );

        self.update_all_vars();
        // r1cs_pcd_message_variable<ram_base_field<ramT> >(&pb, annotation_prefix),ap
        Self { ap }
    }

    pub fn allocate_unpacked_part() {
        let digest_size = CRH_with_bit_out_gadget::<FieldT>::get_digest_len();

        timestamp.allocate(
            self.pb,
            ramT::timestamp_length,
            FMT(self.annotation_prefix, " timestamp"),
        );
        root_initial.allocate(
            self.pb,
            digest_size,
            FMT(self.annotation_prefix, " root_initial"),
        );
        root.allocate(self.pb, digest_size, FMT(self.annotation_prefix, " root"));
        pc_addr.allocate(
            self.pb,
            ap.address_size(),
            FMT(self.annotation_prefix, " pc_addr"),
        );
        cpu_state.allocate(
            self.pb,
            ap.cpu_state_size(),
            FMT(self.annotation_prefix, " cpu_state"),
        );
        pc_addr_initial.allocate(
            self.pb,
            ap.address_size(),
            FMT(self.annotation_prefix, " pc_addr_initial"),
        );
        cpu_state_initial.allocate(
            self.pb,
            ap.cpu_state_size(),
            FMT(self.annotation_prefix, " cpu_state_initial"),
        );
        has_accepted.allocate(self.pb, FMT(self.annotation_prefix, " has_accepted"));

        all_unpacked_vars.extend(&timestamp);
        all_unpacked_vars.extend(&root_initial);
        all_unpacked_vars.extend(&root);
        all_unpacked_vars.extend(&pc_addr);
        all_unpacked_vars.extend(&cpu_state);
        all_unpacked_vars.extend(&pc_addr_initial);
        all_unpacked_vars.extend(&cpu_state_initial);
        all_unpacked_vars.push(has_accepted);

        unpack_payload = RcCell::new(multipacking_gadget::<FieldT>::new(
            self.pb,
            all_unpacked_vars,
            packed_payload,
            FieldT::capacity(),
            FMT(self.annotation_prefix, " unpack_payload"),
        ));
    }

    pub fn generate_r1cs_witness_from_bits() {
        unpack_payload.generate_r1cs_witness_from_bits();
    }

    pub fn generate_r1cs_witness_from_packed() {
        unpack_payload.generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_constraints() {
        unpack_payload.generate_r1cs_constraints(true);
    }

    pub fn get_message() -> RcCell<r1cs_pcd_message<ram_base_field<ramT>>> {
        let type_val = self.pb.borrow().val(&self.types).as_ulong();
        let timestamp_val = timestamp.get_field_element_from_bits(self.pb).as_ulong();
        let root_initial_val = root_initial.get_bits(self.pb);
        let root_val = root.get_bits(self.pb);
        let pc_addr_val = pc_addr.get_field_element_from_bits(self.pb).as_ulong();
        let cpu_state_val = cpu_state.get_bits(self.pb);
        let pc_addr_initial_val = pc_addr_initial
            .get_field_element_from_bits(self.pb)
            .as_ulong();
        let cpu_state_initial_val = cpu_state_initial.get_bits(self.pb);
        let has_accepted_val = (self.pb.borrow().val(&has_accepted) == FieldT::one());

        let mut result = r1cs_pcd_message::<FieldT>::new();
        result = RcCell::new(ram_pcd_message::<ramT>::new(
            type_val,
            ap,
            timestamp_val,
            root_initial_val,
            root_val,
            pc_addr_val,
            cpu_state_val,
            pc_addr_initial_val,
            cpu_state_initial_val,
            has_accepted_val,
        ));
        return result;
    }
}

impl ram_pcd_local_data<ramT> {
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

    pub fn as_r1cs_variable_assignment() -> r1cs_variable_assignment<ram_base_field<ramT>> {
        let mut result = r1cs_variable_assignment::<FieldT>::new();
        result.push(if is_halt_case {
            FieldT::one()
        } else {
            FieldT::zero()
        });
        return result;
    }
}
impl ram_pcd_local_data_variable<ramT> {
    pub fn new(pb: RcCell<protoboard<FieldT>>, annotation_prefix: &String) -> Self {
        is_halt_case.allocate(&pb, FMT(annotation_prefix, " is_halt_case"));

        self.update_all_vars();
        r1cs_pcd_local_data_variable::<ram_base_field<ramT>>(&pb, annotation_prefix)
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
impl ram_compliance_predicate_handler<ramT> {
    pub fn new(ap: &ram_architecture_params<ramT>) -> Self {
        // TODO: assert that message has fields of lengths consistent with num_addresses/value_size (as a method for ram_message)
        // choose a constant for timestamp_len
        // check that value_size <= digest_size; digest_size is not assumed to fit in chunk size (more precisely, it is handled correctly in the other gadgets).
        // check if others fit (timestamp_length, value_size, addr_size)

        // the variables allocated are: next, cur, local data (nil for us), is_base_case, witness

        self.outgoing_message = RcCell::new(ram_pcd_message_variable::<ramT>::new(
            self.pb,
            ap,
            "outgoing_message",
        ));
        self.arity.allocate(self.pb, "arity");
        self.incoming_messages[0] = RcCell::new(ram_pcd_message_variable::<ramT>::new(
            self.pb,
            ap,
            "incoming_message",
        ));
        self.local_data = RcCell::new(ram_pcd_local_data_variable::<ramT>::new(
            self.pb,
            "local_data",
        ));

        is_base_case.allocate(self.pb, "is_base_case");

        let mut next = ram_pcd_message_variable::<ramT>::new(self.outgoing_message);
        let mut cur = ram_pcd_message_variable::<ramT>::new(self.incoming_messages[0]);

        next.allocate_unpacked_part();
        cur.allocate_unpacked_part();

        // work-around for bad linear combination handling
        zero.allocate(self.pb, "zero"); // will go away when we properly support linear terms

        temp_next_pc_addr.allocate(self.pb, addr_size, "temp_next_pc_addr");
        temp_next_cpu_state.allocate(self.pb, ap.cpu_state_size(), "temp_next_cpu_state");

        let chunk_size = FieldT::capacity();

        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        copy_root_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            cur.root_initial,
            next.root_initial,
            ONE,
            chunk_size,
            "copy_root_initial",
        ));
        copy_pc_addr_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            cur.pc_addr_initial,
            next.pc_addr_initial,
            ONE,
            chunk_size,
            "copy_pc_addr_initial",
        ));
        copy_cpu_state_initial = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            cur.cpu_state_initial,
            next.cpu_state_initial,
            ONE,
            chunk_size,
            "copy_cpu_state_initial",
        ));

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        packed_cur_timestamp.allocate(self.pb, "packed_cur_timestamp");
        pack_cur_timestamp = RcCell::new(packing_gadget::<FieldT>::new(
            self.pb,
            cur.timestamp,
            packed_cur_timestamp,
            "pack_cur_timestamp",
        ));

        zero_cpu_state = pb_variable_array::<FieldT>(cur.cpu_state.len(), zero);
        zero_pc_addr = pb_variable_array::<FieldT>(cur.pc_addr.len(), zero);

        initialize_cur_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            cur.cpu_state_initial,
            cur.cpu_state,
            is_base_case,
            chunk_size,
            "initialize_cur_cpu_state",
        ));
        initialize_prev_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            cur.pc_addr_initial,
            cur.pc_addr,
            is_base_case,
            chunk_size,
            "initialize_prev_pc_addr",
        ));

        initialize_root = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
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
        is_not_halt_case.allocate(self.pb, "is_not_halt_case");
        // for performing instruction fetch
        prev_pc_val.allocate(self.pb, value_size, "prev_pc_val");
        prev_pc_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            self.pb,
            digest_size,
            prev_pc_val,
            zero,
            "prev_pc_val_digest",
        ));
        cur_root_digest = RcCell::new(digest_variable::<FieldT>::new(
            self.pb,
            digest_size,
            cur.root,
            zero,
            "cur_root_digest",
        ));
        instruction_fetch_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
                self.pb,
                addr_size,
                "instruction_fetch_merkle_proof",
            ));
        instruction_fetch = RcCell::new(memory_load_gadget::<FieldT, HashT>::new(
            self.pb,
            addr_size,
            cur.pc_addr,
            *prev_pc_val_digest,
            *cur_root_digest,
            *instruction_fetch_merkle_proof,
            ONE,
            "instruction_fetch",
        ));

        // for next.timestamp = cur.timestamp + 1
        packed_next_timestamp.allocate(self.pb, "packed_next_timestamp");
        pack_next_timestamp = RcCell::new(packing_gadget::<FieldT>::new(
            self.pb,
            next.timestamp,
            packed_next_timestamp,
            "pack_next_timestamp",
        ));

        // that CPU accepted on (cur, temp)
        ls_addr.allocate(self.pb, addr_size, "ls_addr");
        ls_prev_val.allocate(self.pb, value_size, "ls_prev_val");
        ls_next_val.allocate(self.pb, value_size, "ls_next_val");
        cpu_checker = RcCell::new(ram_cpu_checker::<ramT>::new(
            self.pb,
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
        ls_prev_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            self.pb,
            digest_size,
            ls_prev_val,
            zero,
            "ls_prev_val_digest",
        ));
        ls_next_val_digest = RcCell::new(digest_variable::<FieldT>::new(
            self.pb,
            digest_size,
            ls_next_val,
            zero,
            "ls_next_val_digest",
        ));
        next_root_digest = RcCell::new(digest_variable::<FieldT>::new(
            self.pb,
            digest_size,
            next.root,
            zero,
            "next_root_digest",
        ));
        load_merkle_proof = RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
            self.pb,
            addr_size,
            "load_merkle_proof",
        ));
        store_merkle_proof =
            RcCell::new(merkle_authentication_path_variable::<FieldT, HashT>::new(
                self.pb,
                addr_size,
                "store_merkle_proof",
            ));
        load_store_checker = RcCell::new(memory_load_store_gadget::<FieldT, HashT>::new(
            self.pb,
            addr_size,
            ls_addr,
            *ls_prev_val_digest,
            *cur_root_digest,
            *load_merkle_proof,
            *ls_next_val_digest,
            *next_root_digest,
            *store_merkle_proof,
            is_not_halt_case,
            "load_store_checker",
        ));
        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        do_halt.allocate(self.pb, "do_halt");
        zero_root = pb_variable_array::<FieldT>(next.root.len(), zero);
        clear_next_root = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            zero_root,
            next.root,
            do_halt,
            chunk_size,
            "clear_next_root",
        ));
        clear_next_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            zero_pc_addr,
            next.pc_addr,
            do_halt,
            chunk_size,
            "clear_next_pc_addr",
        ));
        clear_next_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            zero_cpu_state,
            next.cpu_state,
            do_halt,
            chunk_size,
            "clear_cpu_state",
        ));

        copy_temp_next_pc_addr = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            temp_next_pc_addr,
            next.pc_addr,
            is_not_halt_case,
            chunk_size,
            "copy_temp_next_pc_addr",
        ));
        copy_temp_next_cpu_state = RcCell::new(bit_vector_copy_gadget::<FieldT>::new(
            self.pb,
            temp_next_cpu_state,
            next.cpu_state,
            is_not_halt_case,
            chunk_size,
            "copy_temp_next_cpu_state",
        ));

        Self {
            //  compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> >(ram_protoboard<ramT>(ap),
            //                                                                               100,
            //                                                                               1,
            //                                                                               1,
            //                                                                               true,
            //                                                                               BTreeSet::from([1])),
            ap,
            addr_size: ap.address_size(),
            value_size: ap.value_size(),
            digest_size: CRH_with_bit_out_gadget::<FieldT>::get_digest_len(),
        }
    }

    pub fn generate_r1cs_constraints() {
        print_indent();
        print!("* Message size: {}\n", next.all_vars.len());
        print_indent();
        print!("* Address size: {}\n", addr_size);
        print_indent();
        print!("* CPU state size: {}\n", ap.cpu_state_size());
        print_indent();
        print!("* Digest size: {}\n", digest_size);

        PROFILE_CONSTRAINTS(self.pb, "handle next_type, arity and cur_type");
        {
            generate_r1cs_equals_const_constraint::<FieldT>(
                self.pb,
                next.types,
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
                r1cs_constraint::<FieldT>(is_base_case, cur.types, 0),
                "nonzero_cur_type_implies_base_case_0",
            );
            generate_boolean_r1cs_constraint::<FieldT>(self.pb, cur.types, "cur_type_boolean");
            generate_boolean_r1cs_constraint::<FieldT>(
                self.pb,
                is_base_case,
                "is_base_case_boolean",
            );
        }

        PROFILE_CONSTRAINTS(self.pb, "unpack messages");
        {
            next.generate_r1cs_constraints();
            cur.generate_r1cs_constraints();
        }

        // work-around for bad linear combination handling
        generate_r1cs_equals_const_constraint::<FieldT>(self.pb, zero, FieldT::zero(), " zero");

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
            copy_root_initial.generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTS(self.pb, "copy pc_addr_initial and cpu_state_initial");
        {
            copy_pc_addr_initial.generate_r1cs_constraints(false, false);
            copy_cpu_state_initial.generate_r1cs_constraints(false, false);
        }

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        pack_cur_timestamp.generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(is_base_case, packed_cur_timestamp, 0),
            "clear_ts_on_is_base_case",
        );
        PROFILE_CONSTRAINTS(self.pb, "copy cur_cpu_state and prev_pc_addr");
        {
            initialize_cur_cpu_state.generate_r1cs_constraints(false, false);
            initialize_prev_pc_addr.generate_r1cs_constraints(false, false);
        }
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(is_base_case, cur.has_accepted, 0),
            "is_base_case_is_not_accepting",
        );
        PROFILE_CONSTRAINTS(self.pb, "initialize root");
        {
            initialize_root.generate_r1cs_constraints(false, false);
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
            r1cs_constraint::<FieldT>(1, 1 - do_halt, is_not_halt_case),
            "is_not_halt_case",
        );
        PROFILE_CONSTRAINTS(self.pb, "instruction fetch");
        {
            instruction_fetch_merkle_proof.generate_r1cs_constraints();
            instruction_fetch.generate_r1cs_constraints();
        }
        pack_next_timestamp.generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(
                is_not_halt_case,
                (packed_cur_timestamp + 1) - packed_next_timestamp,
                0,
            ),
            "increment_timestamp",
        );
        PROFILE_CONSTRAINTS(self.pb, "CPU checker");
        {
            cpu_checker.generate_r1cs_constraints();
        }
        PROFILE_CONSTRAINTS(self.pb, "load/store checker");
        {
            // See comment in merkle_tree_check_update_gadget::generate_r1cs_witness() for why we don't need to call store_merkle_proof.generate_r1cs_constraints()
            load_merkle_proof.generate_r1cs_constraints();
            load_store_checker.generate_r1cs_constraints();
        }

        PROFILE_CONSTRAINTS(self.pb, "copy temp_next_pc_addr and temp_next_cpu_state");
        {
            copy_temp_next_pc_addr.generate_r1cs_constraints(true, false);
            copy_temp_next_cpu_state.generate_r1cs_constraints(true, false);
        }

        /*
          If do_halt = 1: (final case)
          that cur.has_accepted = 1
          that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
          that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(do_halt, 1 - cur.has_accepted, 0),
            "final_case_must_be_accepting",
        );

        PROFILE_CONSTRAINTS(self.pb, "clear next root");
        {
            clear_next_root.generate_r1cs_constraints(false, false);
        }

        PROFILE_CONSTRAINTS(self.pb, "clear next_pc_addr and next_cpu_state");
        {
            clear_next_pc_addr.generate_r1cs_constraints(false, false);
            clear_next_cpu_state.generate_r1cs_constraints(false, false);
        }

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(do_halt, packed_cur_timestamp - packed_next_timestamp, 0),
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
        incoming_message_values: &Vec<RcCell<r1cs_pcd_message<FieldT>>>,
        local_data_value: &RcCell<r1cs_pcd_local_data<FieldT>>,
    ) {
        let ram_local_data_value = ram_pcd_local_data::<ramT>(local_data_value);
        assert!(ram_local_data_value.mem.num_addresses == 1u64 << addr_size); // check value_size and num_addresses too

        base_handler::generate_r1cs_witness(incoming_message_values, local_data_value);
        cur.generate_r1cs_witness_from_packed();

        self.pb.borrow().val(&next.types) = FieldT::one();
        self.pb.borrow().val(&self.arity) = FieldT::one();
        self.pb.borrow().val(&is_base_case) = (if self.pb.borrow().val(&cur.types) == FieldT::zero()
        {
            FieldT::one()
        } else {
            FieldT::zero()
        });

        self.pb.borrow().val(&zero) = FieldT::zero();
        /*
          Always:
          next.root_initial = cur.root_initial
          next.pc_addr_init = cur.pc_addr_initial
          next.cpu_state_initial = cur.cpu_state_initial
        */
        copy_root_initial.generate_r1cs_witness();
        for i in 0..next.root_initial.len() {
            self.pb.borrow().val(&cur.root_initial[i]).print();
            self.pb.borrow().val(&next.root_initial[i]).print();
            assert!(
                self.pb.borrow().val(&cur.root_initial[i])
                    == self.pb.borrow().val(&next.root_initial[i])
            );
        }

        copy_pc_addr_initial.generate_r1cs_witness();
        copy_cpu_state_initial.generate_r1cs_witness();

        /*
          If is_base_case = 1: (base case)
          that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
          that cur.root = cur.root_initial
        */
        let base_case = (incoming_message_values[0].types == 0);
        self.pb.borrow().val(&is_base_case) = if base_case {
            FieldT::one()
        } else {
            FieldT::zero()
        };

        initialize_cur_cpu_state.generate_r1cs_witness();
        initialize_prev_pc_addr.generate_r1cs_witness();

        if base_case {
            self.pb.borrow().val(&packed_cur_timestamp) = FieldT::zero();
            self.pb.borrow().val(&cur.has_accepted) = FieldT::zero();
            pack_cur_timestamp.generate_r1cs_witness_from_packed();
        } else {
            pack_cur_timestamp.generate_r1cs_witness_from_bits();
        }

        initialize_root.generate_r1cs_witness();

        /*
          If do_halt = 0: (regular case)
          that instruction fetch was correctly executed
          next.timestamp = cur.timestamp + 1
          that CPU accepted on (cur, temp)
          that load-then-store was correctly handled
        */
        self.pb.borrow().val(&do_halt) = if ram_local_data_value.is_halt_case {
            FieldT::one()
        } else {
            FieldT::zero()
        };
        self.pb.borrow().val(&is_not_halt_case) = FieldT::one() - self.pb.borrow().val(&do_halt);

        // that instruction fetch was correctly executed
        let int_pc_addr =
            convert_bit_vector_to_field_element::<FieldT>(cur.pc_addr.get_bits(self.pb)).as_ulong();
        let int_pc_val = ram_local_data_value.mem.get_value(int_pc_addr);
        // #ifdef DEBUG
        print!(
            "pc_addr (in units) = {}, pc_val = {} (0x{:08x})\n",
            int_pc_addr, int_pc_val, int_pc_val
        );
        //#endif
        let mut pc_val_bv = int_list_to_bits({ int_pc_val }, value_size);
        pc_val_bv.reverse();

        prev_pc_val.fill_with_bits(self.pb, pc_val_bv);
        let pc_path = ram_local_data_value.mem.get_path(int_pc_addr);
        instruction_fetch_merkle_proof.generate_r1cs_witness(int_pc_addr, pc_path);
        instruction_fetch.generate_r1cs_witness();

        // next.timestamp = cur.timestamp + 1 (or cur.timestamp if do_halt)
        self.pb.borrow().val(&packed_next_timestamp) =
            self.pb.borrow().val(&packed_cur_timestamp) + self.pb.borrow().val(&is_not_halt_case);
        pack_next_timestamp.generate_r1cs_witness_from_packed();

        // that CPU accepted on (cur, temp)
        // Step 1: Get address and old witnesses for delegated memory.
        cpu_checker.generate_r1cs_witness_address();
        let int_ls_addr = ls_addr.get_field_element_from_bits(self.pb).as_ulong();
        let int_ls_prev_val = ram_local_data_value.mem.get_value(int_ls_addr);
        let prev_path = ram_local_data_value.mem.get_path(int_ls_addr);
        ls_prev_val.fill_with_bits_of_ulong(self.pb, int_ls_prev_val);
        assert!(ls_prev_val.get_field_element_from_bits(self.pb) == FieldT(int_ls_prev_val, true));
        // Step 2: Execute CPU checker and delegated memory
        cpu_checker
            .generate_r1cs_witness_other(ram_local_data_value.aux_it, ram_local_data_value.aux_end);
        // #ifdef DEBUG
        print!("Debugging information from transition function:\n");
        cpu_checker.dump();
        //#endif
        let int_ls_next_val = ls_next_val.get_field_element_from_bits(self.pb).as_ulong();
        ram_local_data_value
            .mem
            .set_value(int_ls_addr, int_ls_next_val);
        // #ifdef DEBUG
        print!(
            "Memory location {} changed from {} (0x{:08x}) to {} (0x{:08x})\n",
            int_ls_addr, int_ls_prev_val, int_ls_prev_val, int_ls_next_val, int_ls_next_val
        );
        //#endif
        // Step 4: Use both to satisfy load_store_checker
        load_merkle_proof.generate_r1cs_witness(int_ls_addr, prev_path);
        load_store_checker.generate_r1cs_witness();

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
        if self.pb.borrow().val(&do_halt).is_zero() {
            copy_temp_next_pc_addr.generate_r1cs_witness();
            copy_temp_next_cpu_state.generate_r1cs_witness();

            clear_next_root.generate_r1cs_witness();
            clear_next_pc_addr.generate_r1cs_witness();
            clear_next_cpu_state.generate_r1cs_witness();
        } else {
            clear_next_root.generate_r1cs_witness();
            clear_next_pc_addr.generate_r1cs_witness();
            clear_next_cpu_state.generate_r1cs_witness();

            copy_temp_next_pc_addr.generate_r1cs_witness();
            copy_temp_next_cpu_state.generate_r1cs_witness();
        }

        // #ifdef DEBUG
        print!("next.has_accepted: ");
        self.pb.borrow().val(&next.has_accepted).print();
        //#endif

        next.generate_r1cs_witness_from_bits();
    }

    pub fn get_base_case_message(
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

        let result = r1cs_pcd_message::<FieldT>::new();
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
        leave_block("Call to ram_compliance_predicate_handler::get_base_case_message");
        return result;
    }

    pub fn get_final_case_msg(
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

        return result;
    }
}

//#endif // RAM_COMPLIANCE_PREDICATE_TCC_
