/** @file
 *****************************************************************************

 Declaration of interfaces for ram_universal_gadget.

 Given bounds on a RAM computation size (program size bound, primary input
 size bound, and time bound), the "RAM universal gadget" checks the correct
 execution of any RAM computation that fits the bounds.

 The implementation follows, extends, and optimizes the approach described
 in \[BCTV14] (itself building on \[BCGTV13]). The code is parameterized by
 the template parameter ramT, in order to support any RAM that fits certain
 abstract interfaces.

 Roughly, the gadget has three main components:
 - For each time step, a copy of a *execution checker* (which is the RAM CPU checker).
 - For each time step, a copy of a *memory checker* (which verifies memory consistency
   between two 'memory lines' that are adjacent in a memory sort).
 - A single *routing network* (specifically, an arbitrary-size Waksman network),
   which is used check that memory accesses are permutated according to some permutation.

 References:

 \[BCGTV13]:
 "SNARKs for C: verifying program executions succinctly and in zero knowledge",
 Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
 CRYPTO 2014,
 <http://eprint.iacr.org/2013/507>

 \[BCTV14]:
 "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
 Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
 USENIX Security 2014,
 <http://eprint.iacr.org/2013/879>

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_UNIVERSAL_GADGET_HPP_
// #define RAM_UNIVERSAL_GADGET_HPP_

use crate::gadgetlib1::gadgets/routing/as_waksman_routing_gadget;
use libsnark/reductions/ram_to_r1cs/gadgets/memory_checker_gadget;
use libsnark/reductions/ram_to_r1cs/gadgets/trace_lines;
use crate::relations::ram_computations/rams/ram_params;



/*
  Memory layout for our reduction is as follows:

  (1) An initial execution line carrying the initial state (set
      to all zeros)
  (2) program_size_bound + primary_input_size_bound memory lines for
      storing input and program (boot)
  (3) time_bound pairs for (fetch instruction memory line, execute
      instruction execution line)

  Memory line stores address, previous value and the next value of the
  memory cell specified by the address. An execution line additionally
  carries the CPU state.

  Our memory handling technique has a technical requirement that
  address 0 must be accessed. We fulfill this by requiring the initial
  execution line to act as "store 0 to address 0".

  ---

  As an implementation detail if less than program_size_bound +
  primary_input_size_bound are used in the initial memory map, then we
  pre-pend (!) them with "store 0 to address 0" lines. This
  pre-pending means that memory maps that have non-zero value at
  address 0 will still be handled correctly.

  The R1CS input packs the memory map starting from the last entry to
  the first. This way, the prepended zeros arrive at the end of R1CS
  input and thus can be ignored by the "weak" input consistency R1CS
  verifier.
*/

template<typename ramT>
class ram_universal_gadget : public ram_gadget_base<ramT> {
public:
    type ram_base_field<ramT> FieldT;

    size_t num_memory_lines;

    std::vector<memory_line_variable_gadget<ramT> > boot_lines;
    std::vector<pb_variable_array<FieldT> > boot_line_bits;
    std::vector<multipacking_gadget<FieldT> > unpack_boot_lines;

    std::vector<memory_line_variable_gadget<ramT> > load_instruction_lines;
    std::vector<execution_line_variable_gadget<ramT> > execution_lines; /* including the initial execution line */

    std::vector<memory_line_variable_gadget<ramT>* > unrouted_memory_lines;
    std::vector<memory_line_variable_gadget<ramT> > routed_memory_lines;

    std::vector<ram_cpu_checker<ramT> > execution_checkers;
    std::vector<memory_checker_gadget<ramT> > memory_checkers;

    std::vector<pb_variable_array<FieldT> > routing_inputs;
    std::vector<pb_variable_array<FieldT> > routing_outputs;

    std::shared_ptr<as_waksman_routing_gadget<FieldT> > routing_network;

public:

    size_t boot_trace_size_bound;
    size_t time_bound;
    pb_variable_array<FieldT> packed_input;

    ram_universal_gadget(ram_protoboard<ramT> &pb,
                         const size_t boot_trace_size_bound,
                         const size_t time_bound,
                         const pb_variable_array<FieldT> &packed_input,
                         const std::string &annotation_prefix="");

    void generate_r1cs_constraints();
    void generate_r1cs_witness(const ram_boot_trace<ramT> &boot_trace,
                               const ram_input_tape<ramT> &auxiliary_input);

    /* both methods assume that generate_r1cs_witness has been called */
    void print_execution_trace() const;
    void print_memory_trace() const;

    static size_t packed_input_element_size(const ram_architecture_params<ramT> &ap);
    static size_t packed_input_size(const ram_architecture_params<ramT> &ap,
                                    const size_t boot_trace_size_bound);
};



use libsnark/reductions/ram_to_r1cs/gadgets/ram_universal_gadget;

//#endif // RAM_UNIVERSAL_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for ram_universal_gadget.

 See ram_universal_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef RAM_UNIVERSAL_GADGET_TCC_
// #define RAM_UNIVERSAL_GADGET_TCC_

use ffec::algebra::field_utils::field_utils;
use ffec::common::profiling;
use ffec::common::utils;

use crate::common::data_structures::integer_permutation;
use crate::relations::ram_computations/memory/ra_memory;



template<typename ramT>
ram_universal_gadget<ramT>::ram_universal_gadget(ram_protoboard<ramT> &pb,
                                                 const size_t boot_trace_size_bound,
                                                 const size_t time_bound,
                                                 const pb_variable_array<FieldT> &packed_input,
                                                 const std::string &annotation_prefix) :
    ram_gadget_base<ramT>(pb, annotation_prefix),
    boot_trace_size_bound(boot_trace_size_bound),
    time_bound(time_bound),
    packed_input(packed_input)
{
    num_memory_lines = boot_trace_size_bound + (time_bound + 1) + time_bound; /* boot lines, (time_bound + 1) execution lines (including initial) and time_bound load instruction lines */
    const size_t timestamp_size = ffec::log2(num_memory_lines);

    /* allocate all lines on the execution side of the routing network */
    ffec::enter_block("Allocate initial state line");
    execution_lines.reserve(1 + time_bound);
    execution_lines.push(execution_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " execution_lines_{}", 0)));
    unrouted_memory_lines.push(&execution_lines[0]);
    ffec::leave_block("Allocate initial state line");

    ffec::enter_block("Allocate boot lines");
    boot_lines.reserve(boot_trace_size_bound);
    for i in 0..boot_trace_size_bound
    {
        boot_lines.push(memory_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " boot_lines_{}", i)));
        unrouted_memory_lines.push(&boot_lines[i]);
    }
    ffec::leave_block("Allocate boot lines");

    ffec::enter_block("Allocate instruction fetch and execution lines");
    load_instruction_lines.reserve(time_bound+1); /* the last line is NOT a memory line, but here just for uniform coding (i.e. the (unused) result of next PC) */
    for i in 0..time_bound
    {
        load_instruction_lines.push(memory_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " load_instruction_lines_{}", i)));
        unrouted_memory_lines.push(&load_instruction_lines[i]);

        execution_lines.push(execution_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " execution_lines_{}", i+1)));
        unrouted_memory_lines.push(&execution_lines[i+1]);
    }
    load_instruction_lines.push(memory_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " load_instruction_lines_{}", time_bound)));
    ffec::leave_block("Allocate instruction fetch and execution lines");

    /* deal with packing of the input */
    ffec::enter_block("Pack input");
    const size_t line_size_bits = pb.ap.address_size() + pb.ap.value_size();
    const size_t max_chunk_size = FieldT::capacity();
    const size_t packed_line_size = ffec::div_ceil(line_size_bits, max_chunk_size);
    assert!(packed_input.size() == packed_line_size * boot_trace_size_bound);

    auto input_it = packed_input.begin();
    for i in 0..boot_trace_size_bound
    {
        /* note the reversed order */
        pb_variable_array<FieldT> boot_line_bits;
        boot_line_bits.insert(boot_line_bits.end(), boot_lines[boot_trace_size_bound-1-i].address->bits.begin(), boot_lines[boot_trace_size_bound-1-i].address->bits.end());
        boot_line_bits.insert(boot_line_bits.end(), boot_lines[boot_trace_size_bound-1-i].contents_after->bits.begin(), boot_lines[boot_trace_size_bound-1-i].contents_after->bits.end());

        pb_variable_array<FieldT> packed_boot_line = pb_variable_array<FieldT>(input_it, input_it + packed_line_size);
        std::advance(input_it, packed_line_size);

        unpack_boot_lines.push(multipacking_gadget<FieldT>(pb, boot_line_bits, packed_boot_line, max_chunk_size, FMT(annotation_prefix, " unpack_boot_lines_{}", i)));
    }
    ffec::leave_block("Pack input");

    /* deal with routing */
    ffec::enter_block("Allocate routed memory lines");
    for i in 0..num_memory_lines
    {
        routed_memory_lines.push(memory_line_variable_gadget<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " routed_memory_lines_{}", i)));
    }
    ffec::leave_block("Allocate routed memory lines");

    ffec::enter_block("Collect inputs/outputs for the routing network");
    routing_inputs.reserve(num_memory_lines);
    routing_outputs.reserve(num_memory_lines);

    for i in 0..num_memory_lines
    {
        routing_inputs.push(unrouted_memory_lines[i]->all_vars());
        routing_outputs.push(routed_memory_lines[i].all_vars());
    }
    ffec::leave_block("Collect inputs/outputs for the routing network");

    ffec::enter_block("Allocate routing network");
    routing_network.reset(new as_waksman_routing_gadget<FieldT>(pb, num_memory_lines, routing_inputs, routing_outputs, FMT(self.annotation_prefix, " routing_network")));
    ffec::leave_block("Allocate routing network");

    /* deal with all checkers */
    ffec::enter_block("Allocate execution checkers");
    execution_checkers.reserve(time_bound);
    for i in 0..time_bound
    {
        execution_checkers.push(ram_cpu_checker<ramT>(pb,
                                                              load_instruction_lines[i].address->bits, // prev_pc_addr
                                                              load_instruction_lines[i].contents_after->bits, // prev_pc_val
                                                              execution_lines[i].cpu_state, // prev_state
                                                              execution_lines[i+1].address->bits, // ls_addr,
                                                              execution_lines[i+1].contents_before->bits, // ls_prev_val
                                                              execution_lines[i+1].contents_after->bits, // ls_next_val
                                                              execution_lines[i+1].cpu_state, // next_state
                                                              load_instruction_lines[i+1].address->bits, // next_pc_addr
                                                              execution_lines[i+1].has_accepted, // next_has_accepted
                                                              FMT(annotation_prefix, " execution_checkers_{}", i)));
    }
    ffec::leave_block("Allocate execution checkers");

    ffec::enter_block("Allocate all memory checkers");
    memory_checkers.reserve(num_memory_lines);
    for i in 0..num_memory_lines
    {
        memory_checkers.push(memory_checker_gadget<ramT>(pb,
                                                                 timestamp_size,
                                                                 *unrouted_memory_lines[i],
                                                                 routed_memory_lines[i],
                                                                 FMT(self.annotation_prefix, " memory_checkers_{}", i)));
    }
    ffec::leave_block("Allocate all memory checkers");

    /* done */
}

template<typename ramT>
void ram_universal_gadget<ramT>::generate_r1cs_constraints()
{
    ffec::enter_block("Call to generate_r1cs_constraints of ram_universal_gadget");
    for i in 0..boot_trace_size_bound
    {
        unpack_boot_lines[i].generate_r1cs_constraints(false);
    }

    /* ensure that we start with all zeros state */
    for i in 0..self.pb.ap.cpu_state_size()
    {
        generate_r1cs_equals_const_constraint<FieldT>(self.pb, execution_lines[0].cpu_state[i], FieldT::zero());
    }

    /* ensure increasing timestamps */
    for i in 0..num_memory_lines
    {
        generate_r1cs_equals_const_constraint<FieldT>(self.pb, unrouted_memory_lines[i]->timestamp->packed, FieldT(i));
    }

    /* ensure bitness of trace lines on the time side */
    for i in 0..boot_trace_size_bound
    {
        boot_lines[i].generate_r1cs_constraints(true);
    }

    execution_lines[0].generate_r1cs_constraints(true);
    for i in 0..time_bound
    {
        load_instruction_lines[i].generate_r1cs_constraints(true);
        execution_lines[i+1].generate_r1cs_constraints(true);
    }

    /* ensure bitness of trace lines on the memory side */
    for i in 0..num_memory_lines
    {
        routed_memory_lines[i].generate_r1cs_constraints();
    }

    /* ensure that load instruction lines really do loads */
    for i in 0..time_bound
    {
        self.pb.add_r1cs_constraint(r1cs_constraint<FieldT>(1, load_instruction_lines[i].contents_before->packed,
                                                             load_instruction_lines[i].contents_after->packed),
                                     FMT(self.annotation_prefix, " load_instruction_%zu_is_a_load", i));
    }

    /* ensure correct execution */
    for i in 0..time_bound
    {
        execution_checkers[i].generate_r1cs_constraints();
    }

    /* check memory */
    routing_network->generate_r1cs_constraints();

    for i in 0..num_memory_lines
    {
        memory_checkers[i].generate_r1cs_constraints();
    }

    /* ensure that PC started at the prescribed value */
    generate_r1cs_equals_const_constraint<FieldT>(self.pb, load_instruction_lines[0].address->packed, FieldT(self.pb.ap.initial_pc_addr()));

    /* ensure that the last state was an accepting one */
    generate_r1cs_equals_const_constraint<FieldT>(self.pb, execution_lines[time_bound].has_accepted, FieldT::one(), "last_state_must_be_accepting");

    /* print constraint profiling */
    const size_t num_constraints = self.pb.num_constraints();
    const size_t num_variables = self.pb.num_variables();

    if !ffec::inhibit_profiling_info
    {
        ffec::print_indent(); print!("* Number of constraints: {}\n", num_constraints);
        ffec::print_indent(); print!("* Number of constraints / cycle: %0.1f\n", 1.*num_constraints/self.time_bound);

        ffec::print_indent(); print!("* Number of variables: {}\n", num_variables);
        ffec::print_indent(); print!("* Number of variables / cycle: %0.1f\n", 1.*num_variables/self.time_bound);
    }
    ffec::leave_block("Call to generate_r1cs_constraints of ram_universal_gadget");
}

template<typename ramT>
void ram_universal_gadget<ramT>::generate_r1cs_witness(const ram_boot_trace<ramT> &boot_trace,
                                                       const ram_input_tape<ramT> &auxiliary_input)
{
    /* assign correct timestamps to all lines */
    for i in 0..num_memory_lines
    {
        self.pb.val(unrouted_memory_lines[i]->timestamp->packed) = FieldT(i);
        unrouted_memory_lines[i]->timestamp->generate_r1cs_witness_from_packed();
    }

    /* fill in the initial state */
    const ram_cpu_state<ramT> initial_state = self.pb.ap.initial_cpu_state();
    execution_lines[0].cpu_state.fill_with_bits(self.pb, initial_state);

    /* fill in the boot section */
    memory_contents memory_after_boot;

    for it in &boot_trace.get_all_trace_entries()
    {
        const size_t boot_pos = it.first;
        assert!(boot_pos < boot_trace_size_bound);
        const size_t address = it.second.first;
        const size_t contents = it.second.second;

        self.pb.val(boot_lines[boot_pos].address->packed) = FieldT(address, true);
        self.pb.val(boot_lines[boot_pos].contents_after->packed) = FieldT(contents, true);
        boot_lines[boot_pos].generate_r1cs_witness_from_packed();

        memory_after_boot[address] = contents;
    }

    /* do the actual execution */
    ra_memory mem_backend(1ul<<(self.pb.ap.address_size()), self.pb.ap.value_size(), memory_after_boot);
    typename ram_input_tape<ramT>::const_iterator auxiliary_input_it = auxiliary_input.begin();

    self.pb.val(load_instruction_lines[0].address->packed) = FieldT(self.pb.ap.initial_pc_addr(), true);
    load_instruction_lines[0].address->generate_r1cs_witness_from_packed();

    for i in 0..time_bound
    {
        /* load instruction */
        const size_t pc_addr = self.pb.val(load_instruction_lines[i].address->packed).as_ulong();
        const size_t pc_val = mem_backend.get_value(pc_addr);

        self.pb.val(load_instruction_lines[i].contents_before->packed) = FieldT(pc_val, true);
        self.pb.val(load_instruction_lines[i].contents_after->packed) = FieldT(pc_val, true);
        load_instruction_lines[i].generate_r1cs_witness_from_packed();

        /* first fetch the address part of the memory */
        execution_checkers[i].generate_r1cs_witness_address();
        execution_lines[i+1].address->generate_r1cs_witness_from_bits();

        /* fill it in */
        const size_t load_store_addr = self.pb.val(execution_lines[i+1].address->packed).as_ulong();
        const size_t load_store_prev_val = mem_backend.get_value(load_store_addr);

        self.pb.val(execution_lines[i+1].contents_before->packed) = FieldT(load_store_prev_val, true);
        execution_lines[i+1].contents_before->generate_r1cs_witness_from_packed();

        /* then execute the rest of the instruction */
        execution_checkers[i].generate_r1cs_witness_other(auxiliary_input_it, auxiliary_input.end());

        /* update the memory possibly changed by the CPU checker */
        execution_lines[i+1].contents_after->generate_r1cs_witness_from_bits();
        const size_t load_store_next_val = self.pb.val(execution_lines[i+1].contents_after->packed).as_ulong();
        mem_backend.set_value(load_store_addr, load_store_next_val);

        /* the next PC address was passed in a bit form, so maintain packed form as well */
        load_instruction_lines[i+1].address->generate_r1cs_witness_from_bits();
    }

    /*
      Get the correct memory permutation.

      We sort all memory accesses by address breaking ties by
      timestamp. In our routing configuration we pair each memory
      access with subsequent access in this ordering.

      That way num_memory_pairs of memory checkers will do a full
      cycle over all memory accesses, enforced by the proper ordering
      property.
    */

    type std::pair<size_t, size_t> mem_pair; /* a pair of address, timestamp */
    std::vector<mem_pair> mem_pairs;

    for i in 0..self.num_memory_lines
    {
        mem_pairs.push(std::make_pair(self.pb.val(unrouted_memory_lines[i]->address->packed).as_ulong(),
                                              self.pb.val(unrouted_memory_lines[i]->timestamp->packed).as_ulong()));
    }

    std::sort(mem_pairs.begin(), mem_pairs.end());

    integer_permutation pi(self.num_memory_lines);
    for i in 0..self.num_memory_lines
    {
        const size_t timestamp = self.pb.val(unrouted_memory_lines[i]->timestamp->packed).as_ulong();
        const size_t address = self.pb.val(unrouted_memory_lines[i]->address->packed).as_ulong();

        const auto it = std::upper_bound(mem_pairs.begin(), mem_pairs.end(), std::make_pair(address, timestamp));
        const size_t prev = (it == mem_pairs.end() ? 0 : it->second);
        pi.set(prev, i);
    }

    /* route according to the memory permutation */
    routing_network->generate_r1cs_witness(pi);

    for i in 0..self.num_memory_lines
    {
        routed_memory_lines[i].generate_r1cs_witness_from_bits();
    }

    /* generate witness for memory checkers */
    for i in 0..self.num_memory_lines
    {
        memory_checkers[i].generate_r1cs_witness();
    }

    /* repack back the input */
    for i in 0..boot_trace_size_bound
    {
        unpack_boot_lines[i].generate_r1cs_witness_from_bits();
    }

    /* print debugging information */
    if !ffec::inhibit_profiling_info
    {
        ffec::print_indent();
        print!("* Protoboard satisfied: %s\n", (self.pb.is_satisfied() ? "YES" : "no"));
    }
}

template<typename ramT>
void ram_universal_gadget<ramT>::print_execution_trace() const
{
    for i in 0..boot_trace_size_bound
    {
        print!("Boot process at t=#{}: store {} at {}\n",
               i,
               self.pb.val(boot_lines[i].contents_after->packed).as_ulong(),
               self.pb.val(boot_lines[i].address->packed).as_ulong());
    }

    for i in 0..time_bound
    {
        print!("Execution step {}:\n", i);
        print!("  Loaded instruction {} from address {} (ts = {})\n",
               self.pb.val(load_instruction_lines[i].contents_after->packed).as_ulong(),
               self.pb.val(load_instruction_lines[i].address->packed).as_ulong(),
               self.pb.val(load_instruction_lines[i].timestamp->packed).as_ulong());

        print!("  Debugging information from the transition function:\n");
        execution_checkers[i].dump();

        print!("  Memory operation executed: addr = {}, contents_before = {}, contents_after = {} (ts_before = {}, ts_after = {})\n",
               self.pb.val(execution_lines[i+1].address->packed).as_ulong(),
               self.pb.val(execution_lines[i+1].contents_before->packed).as_ulong(),
               self.pb.val(execution_lines[i+1].contents_after->packed).as_ulong(),
               self.pb.val(execution_lines[i].timestamp->packed).as_ulong(),
               self.pb.val(execution_lines[i+1].timestamp->packed).as_ulong());
    }
}

template<typename ramT>
void ram_universal_gadget<ramT>::print_memory_trace() const
{
    for i in 0..num_memory_lines
    {
        print!("Memory access #{}:\n", i);
        print!("  Time side  : ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
               self.pb.val(unrouted_memory_lines[i]->timestamp->packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i]->address->packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i]->contents_before->packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i]->contents_after->packed).as_ulong());
        print!("  Memory side: ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
               self.pb.val(routed_memory_lines[i].timestamp->packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].address->packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].contents_before->packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].contents_after->packed).as_ulong());
    }
}

template<typename ramT>
size_t ram_universal_gadget<ramT>::packed_input_element_size(const ram_architecture_params<ramT> &ap)
{
    const size_t line_size_bits = ap.address_size() + ap.value_size();
    const size_t max_chunk_size = FieldT::capacity();
    const size_t packed_line_size = ffec::div_ceil(line_size_bits, max_chunk_size);

    return packed_line_size;
}

template<typename ramT>
size_t ram_universal_gadget<ramT>::packed_input_size(const ram_architecture_params<ramT> &ap,
                                                     const size_t boot_trace_size_bound)
{
    return packed_input_element_size(ap) * boot_trace_size_bound;
}



//#endif // RAM_UNIVERSAL_GADGET_TCC_
