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

use crate::gadgetlib1::gadgets::routing::as_waksman_routing_gadget;
use crate::reductions::ram_to_r1cs::gadgets::memory_checker_gadget;
use crate::reductions::ram_to_r1cs::gadgets::trace_lines;
use crate::relations::ram_computations::rams::ram_params;



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

type FieldT=ram_base_field<ramT> ;
pub struct  ram_universal_gadget <ramT> {
// : public ram_gadget_base
    

num_memory_lines:    usize,

boot_lines:    Vec<memory_line_variable_gadget<ramT> >,
boot_line_bits:    Vec<pb_variable_array<FieldT> >,
unpack_boot_lines:    Vec<multipacking_gadget<FieldT> >,

load_instruction_lines:    Vec<memory_line_variable_gadget<ramT> >,
execution_lines:    Vec<execution_line_variable_gadget<ramT> >, /* including the initial execution line */

unrouted_memory_lines:    Vec<memory_line_variable_gadget<ramT> >,
routed_memory_lines:    Vec<memory_line_variable_gadget<ramT> >,

execution_checkers:    Vec<ram_cpu_checker<ramT> >,
memory_checkers:    Vec<memory_checker_gadget<ramT> >,

routing_inputs:    Vec<pb_variable_array<FieldT> >,
routing_outputs:    Vec<pb_variable_array<FieldT> >,

routing_network:    RcCell<as_waksman_routing_gadget<FieldT> >,



boot_trace_size_bound:    usize,
time_bound:    usize,
packed_input:    pb_variable_array<FieldT>,

    
}



// use crate::reductions::ram_to_r1cs::gadgets::ram_universal_gadget;

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
use crate::relations::ram_computations::memory::ra_memory;


impl ram_universal_gadget<ramT>{

pub fn new( pb:ram_protoboard<ramT>,
                                                 boot_trace_size_bound:usize,
                                                 time_bound:usize,
                                                 packed_input:&pb_variable_array<FieldT>,
                                                 annotation_prefix:&String) ->Self
   
{
    num_memory_lines = boot_trace_size_bound + (time_bound + 1) + time_bound; /* boot lines, (time_bound + 1) execution lines (including initial) and time_bound load instruction lines */
    let  timestamp_size = ffec::log2(num_memory_lines);

    /* allocate all lines on the execution side of the routing network */
    ffec::enter_block("Allocate initial state line");
    execution_lines.reserve(1 + time_bound);
    execution_lines.push(execution_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " execution_lines_{}", 0)));
    unrouted_memory_lines.push(&execution_lines[0]);
    ffec::leave_block("Allocate initial state line");

    ffec::enter_block("Allocate boot lines");
    boot_lines.reserve(boot_trace_size_bound);
    for i in 0..boot_trace_size_bound
    {
        boot_lines.push(memory_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " boot_lines_{}", i)));
        unrouted_memory_lines.push(&boot_lines[i]);
    }
    ffec::leave_block("Allocate boot lines");

    ffec::enter_block("Allocate instruction fetch and execution lines");
    load_instruction_lines.reserve(time_bound+1); /* the last line is NOT a memory line, but here just for uniform coding (i.e. the (unused) result of next PC) */
    for i in 0..time_bound
    {
        load_instruction_lines.push(memory_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " load_instruction_lines_{}", i)));
        unrouted_memory_lines.push(&load_instruction_lines[i]);

        execution_lines.push(execution_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " execution_lines_{}", i+1)));
        unrouted_memory_lines.push(&execution_lines[i+1]);
    }
    load_instruction_lines.push(memory_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " load_instruction_lines_{}", time_bound)));
    ffec::leave_block("Allocate instruction fetch and execution lines");

    /* deal with packing of the input */
    ffec::enter_block("Pack input");
    let  line_size_bits = pb.ap.address_size() + pb.ap.value_size();
    let max_chunk_size = FieldT::capacity();
    let packed_line_size = ffec::div_ceil(line_size_bits, max_chunk_size);
    assert!(packed_input.len() == packed_line_size * boot_trace_size_bound);

    let mut  input_it = packed_input.begin();
    for i in 0..boot_trace_size_bound
    {
        /* note the reversed order */
        let mut  boot_line_bits=pb_variable_array::<FieldT>::new();
        boot_line_bits.insert(boot_line_bits.end(), boot_lines[boot_trace_size_bound-1-i].address.bits.begin(), boot_lines[boot_trace_size_bound-1-i].address.bits.end());
        boot_line_bits.insert(boot_line_bits.end(), boot_lines[boot_trace_size_bound-1-i].contents_after.bits.begin(), boot_lines[boot_trace_size_bound-1-i].contents_after.bits.end());

        let mut packed_boot_line = pb_variable_array::<FieldT>(input_it, input_it + packed_line_size);
        std::advance(input_it, packed_line_size);

        unpack_boot_lines.push(multipacking_gadget::<FieldT>(pb, boot_line_bits, packed_boot_line, max_chunk_size, FMT(annotation_prefix, " unpack_boot_lines_{}", i)));
    }
    ffec::leave_block("Pack input");

    /* deal with routing */
    ffec::enter_block("Allocate routed memory lines");
    for i in 0..num_memory_lines
    {
        routed_memory_lines.push(memory_line_variable_gadget::<ramT>(pb, timestamp_size, pb.ap, FMT(annotation_prefix, " routed_memory_lines_{}", i)));
    }
    ffec::leave_block("Allocate routed memory lines");

    ffec::enter_block("Collect inputs/outputs for the routing network");
    routing_inputs.reserve(num_memory_lines);
    routing_outputs.reserve(num_memory_lines);

    for i in 0..num_memory_lines
    {
        routing_inputs.push(unrouted_memory_lines[i].all_vars());
        routing_outputs.push(routed_memory_lines[i].all_vars());
    }
    ffec::leave_block("Collect inputs/outputs for the routing network");

    ffec::enter_block("Allocate routing network");
    routing_network.reset( as_waksman_routing_gadget::<FieldT>::new(pb, num_memory_lines, routing_inputs, routing_outputs, FMT(self.annotation_prefix, " routing_network")));
    ffec::leave_block("Allocate routing network");

    /* deal with all checkers */
    ffec::enter_block("Allocate execution checkers");
    execution_checkers.reserve(time_bound);
    for i in 0..time_bound
    {
        execution_checkers.push(ram_cpu_checker::<ramT>(pb,
                                                              load_instruction_lines[i].address.bits, // prev_pc_addr
                                                              load_instruction_lines[i].contents_after.bits, // prev_pc_val
                                                              execution_lines[i].cpu_state, // prev_state
                                                              execution_lines[i+1].address.bits, // ls_addr,
                                                              execution_lines[i+1].contents_before.bits, // ls_prev_val
                                                              execution_lines[i+1].contents_after.bits, // ls_next_val
                                                              execution_lines[i+1].cpu_state, // next_state
                                                              load_instruction_lines[i+1].address.bits, // next_pc_addr
                                                              execution_lines[i+1].has_accepted, // next_has_accepted
                                                            FMT(annotation_prefix, " execution_checkers_{}", i)));
    }
    ffec::leave_block("Allocate execution checkers");

    ffec::enter_block("Allocate all memory checkers");
    memory_checkers.reserve(num_memory_lines);
    for i in 0..num_memory_lines
    {
        memory_checkers.push(memory_checker_gadget::<ramT>(pb,
                                                                 timestamp_size,
                                                                 *unrouted_memory_lines[i],
                                                                 routed_memory_lines[i],
                                                               FMT(self.annotation_prefix, " memory_checkers_{}", i)));
    }
    ffec::leave_block("Allocate all memory checkers");

    /* done */
    //  ram_gadget_base<ramT>(pb, annotation_prefix),
    Self{boot_trace_size_bound,
    time_bound,
    packed_input}
}


pub fn generate_r1cs_constraints()
{
    ffec::enter_block("Call to generate_r1cs_constraints of ram_universal_gadget");
    for i in 0..boot_trace_size_bound
    {
        unpack_boot_lines[i].generate_r1cs_constraints(false);
    }

    /* ensure that we start with all zeros state */
    for i in 0..self.pb.ap.cpu_state_size()
    {
        generate_r1cs_equals_const_constraint::<FieldT>(self.pb, execution_lines[0].cpu_state[i], FieldT::zero());
    }

    /* ensure increasing timestamps */
    for i in 0..num_memory_lines
    {
        generate_r1cs_equals_const_constraint::<FieldT>(self.pb, unrouted_memory_lines[i].timestamp.packed, FieldT(i));
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
        self.pb.add_r1cs_constraint(r1cs_constraint::<FieldT>(1, load_instruction_lines[i].contents_before.packed,
                                                             load_instruction_lines[i].contents_after.packed),
                                   FMT(self.annotation_prefix, " load_instruction_{}_is_a_load", i));
    }

    /* ensure correct execution */
    for i in 0..time_bound
    {
        execution_checkers[i].generate_r1cs_constraints();
    }

    /* check memory */
    routing_network.generate_r1cs_constraints();

    for i in 0..num_memory_lines
    {
        memory_checkers[i].generate_r1cs_constraints();
    }

    /* ensure that PC started at the prescribed value */
    generate_r1cs_equals_const_constraint::<FieldT>(self.pb, load_instruction_lines[0].address.packed, FieldT(self.pb.ap.initial_pc_addr()));

    /* ensure that the last state was an accepting one */
    generate_r1cs_equals_const_constraint::<FieldT>(self.pb, execution_lines[time_bound].has_accepted, FieldT::one(), "last_state_must_be_accepting");

    /* print constraint profiling */
    let num_constraints = self.pb.num_constraints();
    let num_variables = self.pb.num_variables();

    if !ffec::inhibit_profiling_info
    {
        ffec::print_indent(); print!("* Number of constraints: {}\n", num_constraints);
        ffec::print_indent(); print!("* Number of constraints / cycle: {:.1}\n", 1.*num_constraints/self.time_bound);

        ffec::print_indent(); print!("* Number of variables: {}\n", num_variables);
        ffec::print_indent(); print!("* Number of variables / cycle: {:.1}\n", 1.*num_variables/self.time_bound);
    }
    ffec::leave_block("Call to generate_r1cs_constraints of ram_universal_gadget");
}


pub fn generate_r1cs_witness(boot_trace:&ram_boot_trace<ramT>,
                                                       auxiliary_input:&ram_input_tape<ramT>)
{
    /* assign correct timestamps to all lines */
    for i in 0..num_memory_lines
    {
        self.pb.val(unrouted_memory_lines[i].timestamp.packed) = FieldT(i);
        unrouted_memory_lines[i].timestamp.generate_r1cs_witness_from_packed();
    }

    /* fill in the initial state */
    let initial_state = self.pb.ap.initial_cpu_state();
    execution_lines[0].cpu_state.fill_with_bits(self.pb, initial_state);

    /* fill in the boot section */
    let mut  memory_after_boot=memory_contents::new();

    for it in &boot_trace.get_all_trace_entries()
    {
        let boot_pos = it.first;
        assert!(boot_pos < boot_trace_size_bound);
        let address = it.second.first;
        let contents = it.second.second;

        self.pb.val(boot_lines[boot_pos].address.packed) = FieldT(address, true);
        self.pb.val(boot_lines[boot_pos].contents_after.packed) = FieldT(contents, true);
        boot_lines[boot_pos].generate_r1cs_witness_from_packed();

        memory_after_boot[address] = contents;
    }

    /* do the actual execution */
    let  mem_backend=ra_memory(1u64<<(self.pb.ap.address_size()), self.pb.ap.value_size(), memory_after_boot);
    let  auxiliary_input_it = auxiliary_input.begin();

    self.pb.val(load_instruction_lines[0].address.packed) = FieldT(self.pb.ap.initial_pc_addr(), true);
    load_instruction_lines[0].address.generate_r1cs_witness_from_packed();

    for i in 0..time_bound
    {
        /* load instruction */
        let pc_addr = self.pb.val(load_instruction_lines[i].address.packed).as_ulong();
        let pc_val = mem_backend.get_value(pc_addr);

        self.pb.val(load_instruction_lines[i].contents_before.packed) = FieldT(pc_val, true);
        self.pb.val(load_instruction_lines[i].contents_after.packed) = FieldT(pc_val, true);
        load_instruction_lines[i].generate_r1cs_witness_from_packed();

        /* first fetch the address part of the memory */
        execution_checkers[i].generate_r1cs_witness_address();
        execution_lines[i+1].address.generate_r1cs_witness_from_bits();

        /* fill it in */
        let load_store_addr = self.pb.val(execution_lines[i+1].address.packed).as_ulong();
        let load_store_prev_val = mem_backend.get_value(load_store_addr);

        self.pb.val(execution_lines[i+1].contents_before.packed) = FieldT(load_store_prev_val, true);
        execution_lines[i+1].contents_before.generate_r1cs_witness_from_packed();

        /* then execute the rest of the instruction */
        execution_checkers[i].generate_r1cs_witness_other(auxiliary_input_it, auxiliary_input.end());

        /* update the memory possibly changed by the CPU checker */
        execution_lines[i+1].contents_after.generate_r1cs_witness_from_bits();
        let load_store_next_val = self.pb.val(execution_lines[i+1].contents_after.packed).as_ulong();
        mem_backend.set_value(load_store_addr, load_store_next_val);

        /* the next PC address was passed in a bit form, so maintain packed form as well */
        load_instruction_lines[i+1].address.generate_r1cs_witness_from_bits();
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

    type mem_pair=std::pair<usize, usize> ; /* a pair of address, timestamp */
    let mut  mem_pairs=vec![];

    for i in 0..self.num_memory_lines
    {
        mem_pairs.push(std::make_pair(self.pb.val(unrouted_memory_lines[i].address.packed).as_ulong(),
                                              self.pb.val(unrouted_memory_lines[i].timestamp.packed).as_ulong()));
    }

    std::sort(mem_pairs.begin(), mem_pairs.end());

     let mut pi=integer_permutation::new(self.num_memory_lines);
    for i in 0..self.num_memory_lines
    {
        let timestamp = self.pb.val(unrouted_memory_lines[i].timestamp.packed).as_ulong();
        let address = self.pb.val(unrouted_memory_lines[i].address.packed).as_ulong();

        let it = std::upper_bound(mem_pairs.begin(), mem_pairs.end(), std::make_pair(address, timestamp));
        let prev = if it == mem_pairs.end() {0} else{it.1};
        pi.set(prev, i);
    }

    /* route according to the memory permutation */
    routing_network.generate_r1cs_witness(pi);

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
        print!("* Protoboard satisfied: {}\n",  (if self.pb.is_satisfied() {"YES"} else{"no"}));
    }
}


pub fn print_execution_trace() 
{
    for i in 0..boot_trace_size_bound
    {
        print!("Boot process at t=#{}: store {} at {}\n",
               i,
               self.pb.val(boot_lines[i].contents_after.packed).as_ulong(),
               self.pb.val(boot_lines[i].address.packed).as_ulong());
    }

    for i in 0..time_bound
    {
        print!("Execution step {}:\n", i);
        print!("  Loaded instruction {} from address {} (ts = {})\n",
               self.pb.val(load_instruction_lines[i].contents_after.packed).as_ulong(),
               self.pb.val(load_instruction_lines[i].address.packed).as_ulong(),
               self.pb.val(load_instruction_lines[i].timestamp.packed).as_ulong());

        print!("  Debugging information from the transition function:\n");
        execution_checkers[i].dump();

        print!("  Memory operation executed: addr = {}, contents_before = {}, contents_after = {} (ts_before = {}, ts_after = {})\n",
               self.pb.val(execution_lines[i+1].address.packed).as_ulong(),
               self.pb.val(execution_lines[i+1].contents_before.packed).as_ulong(),
               self.pb.val(execution_lines[i+1].contents_after.packed).as_ulong(),
               self.pb.val(execution_lines[i].timestamp.packed).as_ulong(),
               self.pb.val(execution_lines[i+1].timestamp.packed).as_ulong());
    }
}


pub fn print_memory_trace() 
{
    for i in 0..num_memory_lines
    {
        print!("Memory access #{}:\n", i);
        print!("  Time side  : ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
               self.pb.val(unrouted_memory_lines[i].timestamp.packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i].address.packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i].contents_before.packed).as_ulong(),
               self.pb.val(unrouted_memory_lines[i].contents_after.packed).as_ulong());
        print!("  Memory side: ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
               self.pb.val(routed_memory_lines[i].timestamp.packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].address.packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].contents_before.packed).as_ulong(),
               self.pb.val(routed_memory_lines[i].contents_after.packed).as_ulong());
    }
}


pub fn packed_input_element_size(ap:&ram_architecture_params<ramT>)->usize
{
    let line_size_bits = ap.address_size() + ap.value_size();
    let max_chunk_size = FieldT::capacity();
    let packed_line_size = ffec::div_ceil(line_size_bits, max_chunk_size);

    return packed_line_size;
}


pub fn packed_input_size(ap:&ram_architecture_params<ramT>,
                                                     boot_trace_size_bound:usize)->usize
{
    return packed_input_element_size(ap) * boot_trace_size_bound;
}


}

//#endif // RAM_UNIVERSAL_GADGET_TCC_
