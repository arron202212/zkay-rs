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

use  <numeric>

use libsnark/gadgetlib1/gadgets/delegated_ra_memory/memory_load_gadget;
use libsnark/gadgetlib1/gadgets/delegated_ra_memory/memory_load_store_gadget;
use libsnark/relations/ram_computations/memory/delegated_ra_memory;
use libsnark/relations/ram_computations/rams/ram_params;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/compliance_predicate;
use libsnark/zk_proof_systems/pcd/r1cs_pcd/compliance_predicate/cp_handler;



/**
 * A RAM message specializes the generic PCD message, in order to
 * obtain a more user-friendly print method.
 */
template<typename ramT>
class ram_pcd_message : public r1cs_pcd_message<ram_base_field<ramT> > {
private:
    void print_bits(const ffec::bit_vector &bv) const;

public:
    type ram_base_field<ramT> FieldT;

    ram_architecture_params<ramT> ap;

    size_t timestamp;
    ffec::bit_vector root_initial;
    ffec::bit_vector root;
    size_t pc_addr;
    ffec::bit_vector cpu_state;
    size_t pc_addr_initial;
    ffec::bit_vector cpu_state_initial;
    bool has_accepted;

    ram_pcd_message(const size_t type,
                    const ram_architecture_params<ramT> &ap,
                    const size_t timestamp,
                    const ffec::bit_vector root_initial,
                    const ffec::bit_vector root,
                    const size_t pc_addr,
                    const ffec::bit_vector cpu_state,
                    const size_t pc_addr_initial,
                    const ffec::bit_vector cpu_state_initial,
                    const bool has_accepted);

    ffec::bit_vector unpacked_payload_as_bits() const;
    r1cs_variable_assignment<FieldT> payload_as_r1cs_variable_assignment() const;
    void print() const;

    static size_t unpacked_payload_size_in_bits(const ram_architecture_params<ramT> &ap);
};

template<typename ramT>
class ram_pcd_message_variable : public r1cs_pcd_message_variable<ram_base_field<ramT> > {
public:
    ram_architecture_params<ramT> ap;

    type ram_base_field<ramT> FieldT;

    pb_variable_array<FieldT> packed_payload;

    pb_variable_array<FieldT> timestamp;
    pb_variable_array<FieldT> root_initial;
    pb_variable_array<FieldT> root;
    pb_variable_array<FieldT> pc_addr;
    pb_variable_array<FieldT> cpu_state;
    pb_variable_array<FieldT> pc_addr_initial;
    pb_variable_array<FieldT> cpu_state_initial;
    pb_variable<FieldT> has_accepted;

    pb_variable_array<FieldT> all_unpacked_vars;

    std::shared_ptr<multipacking_gadget<FieldT> > unpack_payload;

    ram_pcd_message_variable(protoboard<FieldT> &pb,
                             const ram_architecture_params<ramT> &ap,
                             const std::string &annotation_prefix);

    void allocate_unpacked_part();
    void generate_r1cs_constraints();
    void generate_r1cs_witness_from_bits();
    void generate_r1cs_witness_from_packed();

    std::shared_ptr<r1cs_pcd_message<FieldT> > get_message() const;
};

template<typename ramT>
class ram_pcd_local_data : public r1cs_pcd_local_data<ram_base_field<ramT> > {
public:
    type ram_base_field<ramT> FieldT;

    bool is_halt_case;

    delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > &mem;
    typename ram_input_tape<ramT>::const_iterator &aux_it;
    const typename ram_input_tape<ramT>::const_iterator &aux_end;

    ram_pcd_local_data(const bool is_halt_case,
                       delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > &mem,
                       typename ram_input_tape<ramT>::const_iterator &aux_it,
                       const typename ram_input_tape<ramT>::const_iterator &aux_end);

    r1cs_variable_assignment<FieldT> as_r1cs_variable_assignment() const;
};

template<typename ramT>
class ram_pcd_local_data_variable : public r1cs_pcd_local_data_variable<ram_base_field<ramT> > {
public:
    type ram_base_field<ramT> FieldT;

    pb_variable<FieldT> is_halt_case;

    ram_pcd_local_data_variable(protoboard<FieldT> &pb,
                                const std::string &annotation_prefix);
};

/**
 * A RAM compliance predicate.
 */
template<typename ramT>
class ram_compliance_predicate_handler : public compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> > {
protected:

    ram_architecture_params<ramT> ap;

public:

    type ram_base_field<ramT> FieldT;
    type CRH_with_bit_out_gadget<FieldT> HashT;
    type compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> > base_handler;

    std::shared_ptr<ram_pcd_message_variable<ramT> > next;
    std::shared_ptr<ram_pcd_message_variable<ramT> > cur;
private:

    pb_variable<FieldT> zero; // TODO: promote linear combinations to first class objects
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_root_initial;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_pc_addr_initial;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_cpu_state_initial;

    pb_variable<FieldT> is_base_case;
    pb_variable<FieldT> is_not_halt_case;

    pb_variable<FieldT> packed_cur_timestamp;
    std::shared_ptr<packing_gadget<FieldT> > pack_cur_timestamp;
    pb_variable<FieldT> packed_next_timestamp;
    std::shared_ptr<packing_gadget<FieldT> > pack_next_timestamp;

    pb_variable_array<FieldT> zero_cpu_state;
    pb_variable_array<FieldT> zero_pc_addr;
    pb_variable_array<FieldT> zero_root;

    std::shared_ptr<bit_vector_copy_gadget<FieldT> > initialize_cur_cpu_state;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > initialize_prev_pc_addr;

    std::shared_ptr<bit_vector_copy_gadget<FieldT> > initialize_root;

    pb_variable_array<FieldT> prev_pc_val;
    std::shared_ptr<digest_variable<FieldT> > prev_pc_val_digest;
    std::shared_ptr<digest_variable<FieldT> > cur_root_digest;
    std::shared_ptr<merkle_authentication_path_variable<FieldT, HashT> > instruction_fetch_merkle_proof;
    std::shared_ptr<memory_load_gadget<FieldT, HashT> > instruction_fetch;

    std::shared_ptr<digest_variable<FieldT> > next_root_digest;

    pb_variable_array<FieldT> ls_addr;
    pb_variable_array<FieldT> ls_prev_val;
    pb_variable_array<FieldT> ls_next_val;
    std::shared_ptr<digest_variable<FieldT> > ls_prev_val_digest;
    std::shared_ptr<digest_variable<FieldT> > ls_next_val_digest;
    std::shared_ptr<merkle_authentication_path_variable<FieldT, HashT> > load_merkle_proof;
    std::shared_ptr<merkle_authentication_path_variable<FieldT, HashT> > store_merkle_proof;
    std::shared_ptr<memory_load_store_gadget<FieldT, HashT> > load_store_checker;

    pb_variable_array<FieldT> temp_next_pc_addr;
    pb_variable_array<FieldT> temp_next_cpu_state;
    std::shared_ptr<ram_cpu_checker<ramT> > cpu_checker;

    pb_variable<FieldT> do_halt;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > clear_next_root;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > clear_next_pc_addr;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > clear_next_cpu_state;

    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_temp_next_root;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_temp_next_pc_addr;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > copy_temp_next_cpu_state;

public:
    const size_t addr_size;
    const size_t value_size;
    const size_t digest_size;

    size_t message_length;

    ram_compliance_predicate_handler(const ram_architecture_params<ramT> &ap);
    void generate_r1cs_constraints();
    void generate_r1cs_witness(const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_message_values,
                               const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data_value);

    static std::shared_ptr<r1cs_pcd_message<FieldT> > get_base_case_message(const ram_architecture_params<ramT> &ap,
                                                                            const ram_boot_trace<ramT> &primary_input);
    static std::shared_ptr<r1cs_pcd_message<FieldT> > get_final_case_msg(const ram_architecture_params<ramT> &ap,
                                                                         const ram_boot_trace<ramT> &primary_input,
                                                                         const size_t time_bound);
};



use libsnark/zk_proof_systems/zksnark/ram_zksnark/ram_compliance_predicate;

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

use libsnark/gadgetlib1/constraint_profiling;



template<typename ramT>
ram_pcd_message<ramT>::ram_pcd_message(const size_t type,
                                       const ram_architecture_params<ramT> &ap,
                                       const size_t timestamp,
                                       const ffec::bit_vector root_initial,
                                       const ffec::bit_vector root,
                                       const size_t pc_addr,
                                       const ffec::bit_vector cpu_state,
                                       const size_t pc_addr_initial,
                                       const ffec::bit_vector cpu_state_initial,
                                       const bool has_accepted) :
    r1cs_pcd_message<FieldT>(type),
    ap(ap),
    timestamp(timestamp),
    root_initial(root_initial),
    root(root),
    pc_addr(pc_addr),
    cpu_state(cpu_state),
    pc_addr_initial(pc_addr_initial),
    cpu_state_initial(cpu_state_initial),
    has_accepted(has_accepted)
{
    const size_t digest_size = CRH_with_bit_out_gadget<FieldT>::get_digest_len();
    assert!(ffec::log2(timestamp) < ramT::timestamp_length);
    assert!(root_initial.size() == digest_size);
    assert!(root.size() == digest_size);
    assert!(ffec::log2(pc_addr) < ap.address_size());
    assert!(cpu_state.size() == ap.cpu_state_size());
    assert!(ffec::log2(pc_addr_initial) < ap.address_size());
    assert!(cpu_state_initial.size() == ap.cpu_state_size());
}

template<typename ramT>
ffec::bit_vector ram_pcd_message<ramT>::unpacked_payload_as_bits() const
{
    ffec::bit_vector result;

    const ffec::bit_vector timestamp_bits = ffec::convert_field_element_to_bit_vector<FieldT>(FieldT(timestamp), ramT::timestamp_length);
    const ffec::bit_vector pc_addr_bits = ffec::convert_field_element_to_bit_vector<FieldT>(FieldT(pc_addr), ap.address_size());
    const ffec::bit_vector pc_addr_initial_bits = ffec::convert_field_element_to_bit_vector<FieldT>(FieldT(pc_addr_initial), ap.address_size());

    result.insert(result.end(), timestamp_bits.begin(), timestamp_bits.end());
    result.insert(result.end(), root_initial.begin(), root_initial.end());
    result.insert(result.end(), root.begin(), root.end());
    result.insert(result.end(), pc_addr_bits.begin(), pc_addr_bits.end());
    result.insert(result.end(), cpu_state.begin(), cpu_state.end());
    result.insert(result.end(), pc_addr_initial_bits.begin(), pc_addr_initial_bits.end());
    result.insert(result.end(), cpu_state_initial.begin(), cpu_state_initial.end());
    result.insert(result.end(), has_accepted);

    assert!(result.size() == unpacked_payload_size_in_bits(ap));
    return result;
}

template<typename ramT>
r1cs_variable_assignment<ram_base_field<ramT> > ram_pcd_message<ramT>::payload_as_r1cs_variable_assignment() const
{
    const ffec::bit_vector payload_bits = unpacked_payload_as_bits();
    const r1cs_variable_assignment<FieldT> result = ffec::pack_bit_vector_into_field_element_vector<FieldT>(payload_bits);
    return result;
}

template<typename ramT>
void ram_pcd_message<ramT>::print_bits(const ffec::bit_vector &bv) const
{
    for (bool b : bv)
    {
        print!("%d", b ? 1 : 0);
    }
    print!("\n");
}

template<typename ramT>
void ram_pcd_message<ramT>::print() const
{
    print!("ram_pcd_message:\n");
    print!("  type: {}\n", self.type);
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
    print!("  has_accepted: %s\n", has_accepted ? "YES" : "no");
}

template<typename ramT>
size_t ram_pcd_message<ramT>::unpacked_payload_size_in_bits(const ram_architecture_params<ramT> &ap)
{
    const size_t digest_size = CRH_with_bit_out_gadget<FieldT>::get_digest_len();

    return (ramT::timestamp_length + // timestamp
            2*digest_size + // root, root_initial
            2*ap.address_size() + // pc_addr, pc_addr_initial
            2*ap.cpu_state_size() + // cpu_state, cpu_state_initial
            1); // has_accepted
}

template<typename ramT>
ram_pcd_message_variable<ramT>::ram_pcd_message_variable(protoboard<FieldT> &pb,
                                                         const ram_architecture_params<ramT> &ap,
                                                         const std::string &annotation_prefix) :
    r1cs_pcd_message_variable<ram_base_field<ramT> >(pb, annotation_prefix), ap(ap)
{
    const size_t unpacked_payload_size_in_bits = ram_pcd_message<ramT>::unpacked_payload_size_in_bits(ap);
    const size_t packed_payload_size = ffec::div_ceil(unpacked_payload_size_in_bits, FieldT::capacity());
    packed_payload.allocate(pb, packed_payload_size, FMT(annotation_prefix, " packed_payload"));

    self.update_all_vars();
}

template<typename ramT>
void ram_pcd_message_variable<ramT>::allocate_unpacked_part()
{
    const size_t digest_size = CRH_with_bit_out_gadget<FieldT>::get_digest_len();

    timestamp.allocate(self.pb, ramT::timestamp_length, FMT(self.annotation_prefix, " timestamp"));
    root_initial.allocate(self.pb, digest_size, FMT(self.annotation_prefix, " root_initial"));
    root.allocate(self.pb, digest_size, FMT(self.annotation_prefix, " root"));
    pc_addr.allocate(self.pb, ap.address_size(), FMT(self.annotation_prefix, " pc_addr"));
    cpu_state.allocate(self.pb, ap.cpu_state_size(), FMT(self.annotation_prefix, " cpu_state"));
    pc_addr_initial.allocate(self.pb, ap.address_size(), FMT(self.annotation_prefix, " pc_addr_initial"));
    cpu_state_initial.allocate(self.pb, ap.cpu_state_size(), FMT(self.annotation_prefix, " cpu_state_initial"));
    has_accepted.allocate(self.pb, FMT(self.annotation_prefix, " has_accepted"));

    all_unpacked_vars.insert(all_unpacked_vars.end(), timestamp.begin(), timestamp.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), root_initial.begin(), root_initial.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), root.begin(), root.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), pc_addr.begin(), pc_addr.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), cpu_state.begin(), cpu_state.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), pc_addr_initial.begin(), pc_addr_initial.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), cpu_state_initial.begin(), cpu_state_initial.end());
    all_unpacked_vars.insert(all_unpacked_vars.end(), has_accepted);

    unpack_payload.reset(new multipacking_gadget<FieldT>(self.pb, all_unpacked_vars, packed_payload, FieldT::capacity(), FMT(self.annotation_prefix, " unpack_payload")));
}

template<typename ramT>
void ram_pcd_message_variable<ramT>::generate_r1cs_witness_from_bits()
{
    unpack_payload->generate_r1cs_witness_from_bits();
}

template<typename ramT>
void ram_pcd_message_variable<ramT>::generate_r1cs_witness_from_packed()
{
    unpack_payload->generate_r1cs_witness_from_packed();
}

template<typename ramT>
void ram_pcd_message_variable<ramT>::generate_r1cs_constraints()
{
    unpack_payload->generate_r1cs_constraints(true);
}

template<typename ramT>
std::shared_ptr<r1cs_pcd_message<ram_base_field<ramT> > > ram_pcd_message_variable<ramT>::get_message() const
{
    const size_t type_val = self.pb.val(self.type).as_ulong();
    const size_t timestamp_val = timestamp.get_field_element_from_bits(self.pb).as_ulong();
    const ffec::bit_vector root_initial_val = root_initial.get_bits(self.pb);
    const ffec::bit_vector root_val = root.get_bits(self.pb);
    const size_t pc_addr_val = pc_addr.get_field_element_from_bits(self.pb).as_ulong();
    const ffec::bit_vector cpu_state_val = cpu_state.get_bits(self.pb);
    const size_t pc_addr_initial_val = pc_addr_initial.get_field_element_from_bits(self.pb).as_ulong();
    const ffec::bit_vector cpu_state_initial_val = cpu_state_initial.get_bits(self.pb);
    const bool has_accepted_val = (self.pb.val(has_accepted) == FieldT::one());

    std::shared_ptr<r1cs_pcd_message<FieldT> > result;
    result.reset(new ram_pcd_message<ramT>(type_val,
                                           ap,
                                           timestamp_val,
                                           root_initial_val,
                                           root_val,
                                           pc_addr_val,
                                           cpu_state_val,
                                           pc_addr_initial_val,
                                           cpu_state_initial_val,
                                           has_accepted_val));
    return result;
}

template<typename ramT>
ram_pcd_local_data<ramT>::ram_pcd_local_data(const bool is_halt_case,
                                             delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > &mem,
                                             typename ram_input_tape<ramT>::const_iterator &aux_it,
                                             const typename ram_input_tape<ramT>::const_iterator &aux_end) :
    is_halt_case(is_halt_case), mem(mem), aux_it(aux_it), aux_end(aux_end)
{
}

template<typename ramT>
r1cs_variable_assignment<ram_base_field<ramT> > ram_pcd_local_data<ramT>::as_r1cs_variable_assignment() const
{
    r1cs_variable_assignment<FieldT> result;
    result.push(is_halt_case ? FieldT::one() : FieldT::zero());
    return result;
}

template<typename ramT>
ram_pcd_local_data_variable<ramT>::ram_pcd_local_data_variable(protoboard<FieldT> &pb,
                                                               const std::string &annotation_prefix) :
    r1cs_pcd_local_data_variable<ram_base_field<ramT> >(pb, annotation_prefix)
{
    is_halt_case.allocate(pb, FMT(annotation_prefix, " is_halt_case"));

    self.update_all_vars();
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

template<typename ramT>
ram_compliance_predicate_handler<ramT>::ram_compliance_predicate_handler(const ram_architecture_params<ramT> &ap) :
    compliance_predicate_handler<ram_base_field<ramT>, ram_protoboard<ramT> >(ram_protoboard<ramT>(ap),
                                                                              100,
                                                                              1,
                                                                              1,
                                                                              true,
                                                                              std::set<size_t>{1}),
    ap(ap),
    addr_size(ap.address_size()),
    value_size(ap.value_size()),
    digest_size(CRH_with_bit_out_gadget<FieldT>::get_digest_len())
{
    // TODO: assert that message has fields of lengths consistent with num_addresses/value_size (as a method for ram_message)
    // choose a constant for timestamp_len
    // check that value_size <= digest_size; digest_size is not assumed to fit in chunk size (more precisely, it is handled correctly in the other gadgets).
    // check if others fit (timestamp_length, value_size, addr_size)

    // the variables allocated are: next, cur, local data (nil for us), is_base_case, witness

    self.outgoing_message.reset(new ram_pcd_message_variable<ramT>(self.pb, ap, "outgoing_message"));
    self.arity.allocate(self.pb, "arity");
    self.incoming_messages[0].reset(new ram_pcd_message_variable<ramT>(self.pb, ap, "incoming_message"));
    self.local_data.reset(new ram_pcd_local_data_variable<ramT>(self.pb, "local_data"));

    is_base_case.allocate(self.pb, "is_base_case");

    next = std::dynamic_pointer_cast<ram_pcd_message_variable<ramT> >(self.outgoing_message);
    cur = std::dynamic_pointer_cast<ram_pcd_message_variable<ramT> >(self.incoming_messages[0]);

    next->allocate_unpacked_part();
    cur->allocate_unpacked_part();

    // work-around for bad linear combination handling
    zero.allocate(self.pb, "zero"); // will go away when we properly support linear terms

    temp_next_pc_addr.allocate(self.pb, addr_size, "temp_next_pc_addr");
    temp_next_cpu_state.allocate(self.pb, ap.cpu_state_size(), "temp_next_cpu_state");

    const size_t chunk_size = FieldT::capacity();

    /*
      Always:
      next.root_initial = cur.root_initial
      next.pc_addr_init = cur.pc_addr_initial
      next.cpu_state_initial = cur.cpu_state_initial
    */
    copy_root_initial.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->root_initial, next->root_initial, ONE, chunk_size, "copy_root_initial"));
    copy_pc_addr_initial.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->pc_addr_initial, next->pc_addr_initial, ONE, chunk_size, "copy_pc_addr_initial"));
    copy_cpu_state_initial.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->cpu_state_initial, next->cpu_state_initial, ONE, chunk_size, "copy_cpu_state_initial"));

    /*
      If is_base_case = 1: (base case)
      that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
      that cur.root = cur.root_initial
    */
    packed_cur_timestamp.allocate(self.pb, "packed_cur_timestamp");
    pack_cur_timestamp.reset(new packing_gadget<FieldT>(self.pb, cur->timestamp, packed_cur_timestamp, "pack_cur_timestamp"));

    zero_cpu_state = pb_variable_array<FieldT>(cur->cpu_state.size(), zero);
    zero_pc_addr = pb_variable_array<FieldT>(cur->pc_addr.size(), zero);

    initialize_cur_cpu_state.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->cpu_state_initial, cur->cpu_state, is_base_case, chunk_size, "initialize_cur_cpu_state"));
    initialize_prev_pc_addr.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->pc_addr_initial, cur->pc_addr, is_base_case, chunk_size, "initialize_prev_pc_addr"));

    initialize_root.reset(new bit_vector_copy_gadget<FieldT>(self.pb, cur->root_initial, cur->root, is_base_case, chunk_size, "initialize_root"));
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
    prev_pc_val_digest.reset(new digest_variable<FieldT>(self.pb, digest_size, prev_pc_val, zero, "prev_pc_val_digest"));
    cur_root_digest.reset(new digest_variable<FieldT>(self.pb, digest_size, cur->root, zero, "cur_root_digest"));
    instruction_fetch_merkle_proof.reset(new merkle_authentication_path_variable<FieldT, HashT>(self.pb, addr_size, "instruction_fetch_merkle_proof"));
    instruction_fetch.reset(new memory_load_gadget<FieldT, HashT>(self.pb, addr_size,
                                                                  cur->pc_addr,
                                                                  *prev_pc_val_digest,
                                                                  *cur_root_digest,
                                                                  *instruction_fetch_merkle_proof,
                                                                  ONE,
                                                                  "instruction_fetch"));

    // for next.timestamp = cur.timestamp + 1
    packed_next_timestamp.allocate(self.pb, "packed_next_timestamp");
    pack_next_timestamp.reset(new packing_gadget<FieldT>(self.pb, next->timestamp, packed_next_timestamp, "pack_next_timestamp"));

    // that CPU accepted on (cur, temp)
    ls_addr.allocate(self.pb, addr_size, "ls_addr");
    ls_prev_val.allocate(self.pb, value_size, "ls_prev_val");
    ls_next_val.allocate(self.pb, value_size, "ls_next_val");
    cpu_checker.reset(new ram_cpu_checker<ramT>(self.pb, cur->pc_addr, prev_pc_val, cur->cpu_state,
                                                ls_addr, ls_prev_val, ls_next_val,
                                                temp_next_cpu_state, temp_next_pc_addr, next->has_accepted,
                                                "cpu_checker"));

    // that load-then-store was correctly handled
    ls_prev_val_digest.reset(new digest_variable<FieldT>(self.pb, digest_size, ls_prev_val, zero, "ls_prev_val_digest"));
    ls_next_val_digest.reset(new digest_variable<FieldT>(self.pb, digest_size, ls_next_val, zero, "ls_next_val_digest"));
    next_root_digest.reset(new digest_variable<FieldT>(self.pb, digest_size, next->root, zero, "next_root_digest"));
    load_merkle_proof.reset(new merkle_authentication_path_variable<FieldT, HashT>(self.pb, addr_size, "load_merkle_proof"));
    store_merkle_proof.reset(new merkle_authentication_path_variable<FieldT, HashT>(self.pb, addr_size, "store_merkle_proof"));
    load_store_checker.reset(new memory_load_store_gadget<FieldT, HashT>(self.pb, addr_size, ls_addr,
                                                                         *ls_prev_val_digest, *cur_root_digest, *load_merkle_proof,
                                                                         *ls_next_val_digest, *next_root_digest, *store_merkle_proof, is_not_halt_case,
                                                                         "load_store_checker"));
    /*
      If do_halt = 1: (final case)
      that cur.has_accepted = 1
      that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
      that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
    */
    do_halt.allocate(self.pb, "do_halt");
    zero_root = pb_variable_array<FieldT>(next->root.size(), zero);
    clear_next_root.reset(new bit_vector_copy_gadget<FieldT>(self.pb, zero_root, next->root, do_halt, chunk_size, "clear_next_root"));
    clear_next_pc_addr.reset(new bit_vector_copy_gadget<FieldT>(self.pb, zero_pc_addr, next->pc_addr, do_halt, chunk_size, "clear_next_pc_addr"));
    clear_next_cpu_state.reset(new bit_vector_copy_gadget<FieldT>(self.pb, zero_cpu_state, next->cpu_state, do_halt, chunk_size, "clear_cpu_state"));

    copy_temp_next_pc_addr.reset(new bit_vector_copy_gadget<FieldT>(self.pb, temp_next_pc_addr, next->pc_addr, is_not_halt_case, chunk_size, "copy_temp_next_pc_addr"));
    copy_temp_next_cpu_state.reset(new bit_vector_copy_gadget<FieldT>(self.pb, temp_next_cpu_state, next->cpu_state, is_not_halt_case, chunk_size, "copy_temp_next_cpu_state"));
}

template<typename ramT>
void ram_compliance_predicate_handler<ramT>::generate_r1cs_constraints()
{
    ffec::print_indent(); print!("* Message size: {}\n", next->all_vars.size());
    ffec::print_indent(); print!("* Address size: {}\n", addr_size);
    ffec::print_indent(); print!("* CPU state size: {}\n", ap.cpu_state_size());
    ffec::print_indent(); print!("* Digest size: {}\n", digest_size);

    PROFILE_CONSTRAINTS(self.pb, "handle next_type, arity and cur_type")
    {
        generate_r1cs_equals_const_constraint<FieldT>(self.pb, next->type, FieldT::one(), "next_type");
        generate_r1cs_equals_const_constraint<FieldT>(self.pb, self.arity, FieldT::one(), "arity");
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(is_base_case, cur->type, 0), "nonzero_cur_type_implies_base_case_0");
        generate_boolean_r1cs_constraint<FieldT>(self.pb, cur->type, "cur_type_boolean");
        generate_boolean_r1cs_constraint<FieldT>(self.pb, is_base_case, "is_base_case_boolean");
    }

    PROFILE_CONSTRAINTS(self.pb, "unpack messages")
    {
        next->generate_r1cs_constraints();
        cur->generate_r1cs_constraints();
    }

    // work-around for bad linear combination handling
    generate_r1cs_equals_const_constraint<FieldT>(self.pb, zero, FieldT::zero(), " zero");

    /* recall that Booleanity of PCD messages has already been enforced by the PCD machine, which is explains the absence of Booleanity checks */
    /*
      We need to perform the following checks:

      Always:
      next.root_initial = cur.root_initial
      next.pc_addr_init = cur.pc_addr_initial
      next.cpu_state_initial = cur.cpu_state_initial
    */
    PROFILE_CONSTRAINTS(self.pb, "copy root_initial")
    {
        copy_root_initial->generate_r1cs_constraints(false, false);
    }

    PROFILE_CONSTRAINTS(self.pb, "copy pc_addr_initial and cpu_state_initial")
    {
        copy_pc_addr_initial->generate_r1cs_constraints(false, false);
        copy_cpu_state_initial->generate_r1cs_constraints(false, false);
    }

    /*
      If is_base_case = 1: (base case)
      that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
      that cur.root = cur.root_initial
    */
    pack_cur_timestamp->generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(is_base_case, packed_cur_timestamp, 0), "clear_ts_on_is_base_case");
    PROFILE_CONSTRAINTS(self.pb, "copy cur_cpu_state and prev_pc_addr")
    {
        initialize_cur_cpu_state->generate_r1cs_constraints(false, false);
        initialize_prev_pc_addr->generate_r1cs_constraints(false, false);
    }
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(is_base_case, cur->has_accepted, 0), "is_base_case_is_not_accepting");
    PROFILE_CONSTRAINTS(self.pb, "initialize root")
    {
        initialize_root->generate_r1cs_constraints(false, false);
    }

    /*
      If do_halt = 0: (regular case)
      that instruction fetch was correctly executed
      next.timestamp = cur.timestamp + 1
      that CPU accepted on (cur, next)
      that load-then-store was correctly handled
      that next.root = temp.root, next.cpu_state = temp.cpu_state, next.pc_addr = temp.pc_addr
    */
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, 1 - do_halt, is_not_halt_case), "is_not_halt_case");
    PROFILE_CONSTRAINTS(self.pb, "instruction fetch")
    {
        instruction_fetch_merkle_proof->generate_r1cs_constraints();
        instruction_fetch->generate_r1cs_constraints();
    }
    pack_next_timestamp->generate_r1cs_constraints(false);
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(is_not_halt_case, (packed_cur_timestamp + 1) - packed_next_timestamp, 0), "increment_timestamp");
    PROFILE_CONSTRAINTS(self.pb, "CPU checker")
    {
        cpu_checker->generate_r1cs_constraints();
    }
    PROFILE_CONSTRAINTS(self.pb, "load/store checker")
    {
        // See comment in merkle_tree_check_update_gadget::generate_r1cs_witness() for why we don't need to call store_merkle_proof->generate_r1cs_constraints()
        load_merkle_proof->generate_r1cs_constraints();
        load_store_checker->generate_r1cs_constraints();
    }

    PROFILE_CONSTRAINTS(self.pb, "copy temp_next_pc_addr and temp_next_cpu_state")
    {
        copy_temp_next_pc_addr->generate_r1cs_constraints(true, false);
        copy_temp_next_cpu_state->generate_r1cs_constraints(true, false);
    }

    /*
      If do_halt = 1: (final case)
      that cur.has_accepted = 1
      that next.root = 0, next.cpu_state = 0, next.pc_addr = 0
      that next.timestamp = cur.timestamp and next.has_accepted = cur.has_accepted
    */
    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(do_halt, 1 - cur->has_accepted, 0), "final_case_must_be_accepting");

    PROFILE_CONSTRAINTS(self.pb, "clear next root")
    {
        clear_next_root->generate_r1cs_constraints(false, false);
    }

    PROFILE_CONSTRAINTS(self.pb, "clear next_pc_addr and next_cpu_state")
    {
        clear_next_pc_addr->generate_r1cs_constraints(false, false);
        clear_next_cpu_state->generate_r1cs_constraints(false, false);
    }

    self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(do_halt,  packed_cur_timestamp - packed_next_timestamp, 0), "equal_ts_on_halt");

    const size_t accounted = PRINT_CONSTRAINT_PROFILING();
    const size_t total = self.pb.num_constraints();
    ffec::print_indent(); print!("* Unaccounted constraints: {}\n", total - accounted);
    ffec::print_indent(); print!("* Number of constraints in ram_compliance_predicate: {}\n", total);
}

template<typename ramT>
void ram_compliance_predicate_handler<ramT>::generate_r1cs_witness(const std::vector<std::shared_ptr<r1cs_pcd_message<FieldT> > > &incoming_message_values,
                                                                   const std::shared_ptr<r1cs_pcd_local_data<FieldT> > &local_data_value)
{
    const std::shared_ptr<ram_pcd_local_data<ramT> > ram_local_data_value = std::dynamic_pointer_cast<ram_pcd_local_data<ramT> >(local_data_value);
    assert!(ram_local_data_value->mem.num_addresses == 1ul << addr_size); // check value_size and num_addresses too

    base_handler::generate_r1cs_witness(incoming_message_values, local_data_value);
    cur->generate_r1cs_witness_from_packed();

    self.pb.val(next->type) = FieldT::one();
    self.pb.val(self.arity) = FieldT::one();
    self.pb.val(is_base_case) = (self.pb.val(cur->type) == FieldT::zero() ? FieldT::one() : FieldT::zero());

    self.pb.val(zero) = FieldT::zero();
    /*
      Always:
      next.root_initial = cur.root_initial
      next.pc_addr_init = cur.pc_addr_initial
      next.cpu_state_initial = cur.cpu_state_initial
    */
    copy_root_initial->generate_r1cs_witness();
    for (size_t i = 0 ; i < next->root_initial.size(); ++i)
    {
        self.pb.val(cur->root_initial[i]).print();
        self.pb.val(next->root_initial[i]).print();
        assert!(self.pb.val(cur->root_initial[i]) == self.pb.val(next->root_initial[i]));
    }

    copy_pc_addr_initial->generate_r1cs_witness();
    copy_cpu_state_initial->generate_r1cs_witness();

    /*
      If is_base_case = 1: (base case)
      that cur.timestamp = 0, cur.cpu_state = 0, cur.pc_addr = 0, cur.has_accepted = 0
      that cur.root = cur.root_initial
    */
    const bool base_case = (incoming_message_values[0]->type == 0);
    self.pb.val(is_base_case) = base_case ? FieldT::one() : FieldT::zero();

    initialize_cur_cpu_state->generate_r1cs_witness();
    initialize_prev_pc_addr->generate_r1cs_witness();

    if (base_case)
    {
        self.pb.val(packed_cur_timestamp) = FieldT::zero();
        self.pb.val(cur->has_accepted) = FieldT::zero();
        pack_cur_timestamp->generate_r1cs_witness_from_packed();
    }
    else
    {
        pack_cur_timestamp->generate_r1cs_witness_from_bits();
    }

    initialize_root->generate_r1cs_witness();

    /*
      If do_halt = 0: (regular case)
      that instruction fetch was correctly executed
      next.timestamp = cur.timestamp + 1
      that CPU accepted on (cur, temp)
      that load-then-store was correctly handled
    */
    self.pb.val(do_halt) = ram_local_data_value->is_halt_case ? FieldT::one() : FieldT::zero();
    self.pb.val(is_not_halt_case) = FieldT::one() - self.pb.val(do_halt);

    // that instruction fetch was correctly executed
    const size_t int_pc_addr = ffec::convert_bit_vector_to_field_element<FieldT>(cur->pc_addr.get_bits(self.pb)).as_ulong();
    const size_t int_pc_val = ram_local_data_value->mem.get_value(int_pc_addr);
// #ifdef DEBUG
    print!("pc_addr (in units) = {}, pc_val = {} (0x%08zx)\n", int_pc_addr, int_pc_val, int_pc_val);
//#endif
    ffec::bit_vector pc_val_bv = ffec::int_list_to_bits({ int_pc_val }, value_size);
    std::reverse(pc_val_bv.begin(), pc_val_bv.end());

    prev_pc_val.fill_with_bits(self.pb, pc_val_bv);
    const merkle_authentication_path pc_path = ram_local_data_value->mem.get_path(int_pc_addr);
    instruction_fetch_merkle_proof->generate_r1cs_witness(int_pc_addr, pc_path);
    instruction_fetch->generate_r1cs_witness();

    // next.timestamp = cur.timestamp + 1 (or cur.timestamp if do_halt)
    self.pb.val(packed_next_timestamp) = self.pb.val(packed_cur_timestamp) + self.pb.val(is_not_halt_case);
    pack_next_timestamp->generate_r1cs_witness_from_packed();

    // that CPU accepted on (cur, temp)
    // Step 1: Get address and old witnesses for delegated memory.
    cpu_checker->generate_r1cs_witness_address();
    const size_t int_ls_addr = ls_addr.get_field_element_from_bits(self.pb).as_ulong();
    const size_t int_ls_prev_val = ram_local_data_value->mem.get_value(int_ls_addr);
    const merkle_authentication_path prev_path = ram_local_data_value->mem.get_path(int_ls_addr);
    ls_prev_val.fill_with_bits_of_ulong(self.pb, int_ls_prev_val);
    assert!(ls_prev_val.get_field_element_from_bits(self.pb) == FieldT(int_ls_prev_val, true));
    // Step 2: Execute CPU checker and delegated memory
    cpu_checker->generate_r1cs_witness_other(ram_local_data_value->aux_it, ram_local_data_value->aux_end);
// #ifdef DEBUG
    print!("Debugging information from transition function:\n");
    cpu_checker->dump();
//#endif
    const size_t int_ls_next_val = ls_next_val.get_field_element_from_bits(self.pb).as_ulong();
    ram_local_data_value->mem.set_value(int_ls_addr, int_ls_next_val);
// #ifdef DEBUG
    print!("Memory location {} changed from {} (0x%08zx) to {} (0x%08zx)\n", int_ls_addr, int_ls_prev_val, int_ls_prev_val, int_ls_next_val, int_ls_next_val);
//#endif
    // Step 4: Use both to satisfy load_store_checker
    load_merkle_proof->generate_r1cs_witness(int_ls_addr, prev_path);
    load_store_checker->generate_r1cs_witness();

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
    if (self.pb.val(do_halt).is_zero())
    {
        copy_temp_next_pc_addr->generate_r1cs_witness();
        copy_temp_next_cpu_state->generate_r1cs_witness();

        clear_next_root->generate_r1cs_witness();
        clear_next_pc_addr->generate_r1cs_witness();
        clear_next_cpu_state->generate_r1cs_witness();
    }
    else
    {
        clear_next_root->generate_r1cs_witness();
        clear_next_pc_addr->generate_r1cs_witness();
        clear_next_cpu_state->generate_r1cs_witness();

        copy_temp_next_pc_addr->generate_r1cs_witness();
        copy_temp_next_cpu_state->generate_r1cs_witness();
    }

// #ifdef DEBUG
    print!("next.has_accepted: ");
    self.pb.val(next->has_accepted).print();
//#endif

    next->generate_r1cs_witness_from_bits();
}

template<typename ramT>
std::shared_ptr<r1cs_pcd_message<ram_base_field<ramT> > > ram_compliance_predicate_handler<ramT>::get_base_case_message(const ram_architecture_params<ramT> &ap,
                                                                                                                        const ram_boot_trace<ramT> &primary_input)
{
    ffec::enter_block("Call to ram_compliance_predicate_handler::get_base_case_message");
    const size_t num_addresses = 1ul << ap.address_size();
    const size_t value_size = ap.value_size();
    delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > mem(num_addresses, value_size, primary_input.as_memory_contents());

    const size_t type = 0;

    const size_t timestamp = 0;

    const ffec::bit_vector root_initial = mem.get_root();
    const size_t pc_addr_initial = ap.initial_pc_addr();
    const ffec::bit_vector cpu_state_initial(ap.cpu_state_size(), false);

    const ffec::bit_vector root = root_initial;
    const size_t pc_addr = pc_addr_initial;
    const ffec::bit_vector cpu_state = cpu_state_initial;

    const bool has_accepted = false;

    std::shared_ptr<r1cs_pcd_message<FieldT> > result;
    result.reset(new ram_pcd_message<ramT>(type, ap, timestamp, root_initial, root, pc_addr, cpu_state, pc_addr_initial, cpu_state_initial, has_accepted));
    ffec::leave_block("Call to ram_compliance_predicate_handler::get_base_case_message");
    return result;
}

template<typename ramT>
std::shared_ptr<r1cs_pcd_message<ram_base_field<ramT> > > ram_compliance_predicate_handler<ramT>::get_final_case_msg(const ram_architecture_params<ramT> &ap,
                                                                                                                     const ram_boot_trace<ramT> &primary_input,
                                                                                                                     const size_t time_bound)
{
    ffec::enter_block("Call to ram_compliance_predicate_handler::get_final_case_msg");
    const size_t num_addresses = 1ul << ap.address_size();
    const size_t value_size = ap.value_size();
    delegated_ra_memory<CRH_with_bit_out_gadget<FieldT> > mem(num_addresses, value_size, primary_input.as_memory_contents());

    const size_t type = 1;

    const size_t timestamp = time_bound;

    const ffec::bit_vector root_initial = mem.get_root();
    const size_t pc_addr_initial = ap.initial_pc_addr();
    const ffec::bit_vector cpu_state_initial(ap.cpu_state_size(), false);

    const ffec::bit_vector root(root_initial.size(), false);
    const size_t pc_addr = 0;
    const ffec::bit_vector cpu_state = cpu_state_initial;

    const bool has_accepted = true;

    std::shared_ptr<r1cs_pcd_message<FieldT> > result;
    result.reset(new ram_pcd_message<ramT>(type, ap, timestamp, root_initial, root, pc_addr, cpu_state, pc_addr_initial, cpu_state_initial, has_accepted));
    ffec::leave_block("Call to ram_compliance_predicate_handler::get_final_case_msg");

    return result;
}



//#endif // RAM_COMPLIANCE_PREDICATE_TCC_
