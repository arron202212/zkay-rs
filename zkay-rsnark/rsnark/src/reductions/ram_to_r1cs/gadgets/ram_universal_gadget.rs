//  Declaration of interfaces for ram_universal_gadget.

//  Given bounds on a RAM computation size (program size bound, primary input
//  size bound, and time bound), the "RAM universal gadget" checks the correct
//  execution of any RAM computation that fits the bounds.

//  The implementation follows, extends, and optimizes the approach described
//  in \[BCTV14] (itself building on \[BCGTV13]). The code is parameterized by
//  the template parameter RamT, in order to support any RAM that fits certain
//  abstract interfaces.

//  Roughly, the gadget has three main components:
//  - For each time step, a copy of a *execution checker* (which is the RAM CPU checker).
//  - For each time step, a copy of a *memory checker* (which verifies memory consistency
//    between two 'memory lines' that are adjacent in a memory sort).
//  - A single *routing network* (specifically, an arbitrary-size Waksman network),
//    which is used check that memory accesses are permutated according to some permutation.

//  References:

//  \[BCGTV13]:
//  "SNARKs for C: verifying program executions succinctly and in zero knowledge",
//  Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
//  CRYPTO 2014,
//  <http://eprint.iacr.org/2013/507>

//  \[BCTV14]:
//  "Succinct Non-Interactive Zero Knowledge for a von Neumann Architecture",
//  Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, Madars Virza,
//  USENIX Security 2014,
//  <http://eprint.iacr.org/2013/879>

use crate::common::data_structures::integer_permutation::integer_permutation;
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_r1cs_equals_const_constraint, multipacking_gadget, multipacking_gadgets,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::gadgets::routing::as_waksman_routing_gadget::{
    as_waksman_routing_gadget, as_waksman_routing_gadgets,
};
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::prefix_format;
use crate::reductions::ram_to_r1cs::gadgets::{
    memory_checker_gadget::{memory_checker_gadget, memory_checker_gadgets},
    trace_lines::{
        execution_line_variable_gadget, execution_line_variable_gadgets,
        memory_line_variable_gadget, memory_line_variable_gadgets,
    },
};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::memory::memory_interface::memory_contents;
use crate::relations::ram_computations::memory::memory_interface::memory_interface;
use crate::relations::ram_computations::memory::ra_memory::ra_memory;
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, CpuCheckConfig, ram_architecture_params, ram_base_field,
    ram_boot_trace, ram_cpu_checker, ram_input_tape, ram_params_type, ram_protoboard,
};
use crate::relations::variable::linear_combination;
use ffec::common::profiling::{enter_block, leave_block, print_indent};
use ffec::scalar_multiplication::multiexp::inhibit_profiling_info;
use ffec::{FieldTConfig, One, Zero, div_ceil, log2};
use rccell::RcCell;

// use algebra::field_utils::field_utils;

// use crate::common::data_structures::integer_permutation;
// use crate::relations::ram_computations::memory::ra_memory;

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

type FieldT<RamT> = ram_base_field<RamT>;

#[derive(Clone, Default)]
pub struct ram_universal_gadget<RamT: ram_params_type> {
    // : public ram_gadget_base
    pub num_memory_lines: usize,
    pub boot_lines: Vec<execution_line_variable_gadgets<RamT>>,
    pub boot_line_bits: Vec<pb_variable_array<FieldT<RamT>, RamT::PB>>,
    pub unpack_boot_lines: Vec<multipacking_gadgets<FieldT<RamT>, RamT::PB>>,
    pub load_instruction_lines: Vec<execution_line_variable_gadgets<RamT>>,
    pub execution_lines: Vec<execution_line_variable_gadgets<RamT>>, /* including the initial execution line */
    pub unrouted_memory_lines: Vec<execution_line_variable_gadgets<RamT>>,
    pub routed_memory_lines: Vec<execution_line_variable_gadgets<RamT>>,
    pub execution_checkers: Vec<ram_cpu_checker<RamT>>,
    pub memory_checkers: Vec<memory_checker_gadgets<RamT>>,
    pub routing_inputs: Vec<pb_variable_array<FieldT<RamT>, RamT::PB>>,
    pub routing_outputs: Vec<pb_variable_array<FieldT<RamT>, RamT::PB>>,
    pub routing_network: RcCell<as_waksman_routing_gadgets<FieldT<RamT>, RamT::PB>>,
    pub boot_trace_size_bound: usize,
    pub time_bound: usize,
    pub packed_input: pb_variable_array<FieldT<RamT>, RamT::PB>,
}

pub type ram_universal_gadgets<RamT> =
    gadget<<RamT as ppTConfig>::FieldT, <RamT as ppTConfig>::PB, ram_universal_gadget<RamT>>;

impl<RamT: ram_params_type> ram_universal_gadget<RamT> {
    pub fn new(
        pb: RcCell<ram_protoboard<RamT>>,
        boot_trace_size_bound: usize,
        time_bound: usize,
        packed_input: pb_variable_array<FieldT<RamT>, RamT::PB>,
        annotation_prefix: String,
    ) -> ram_universal_gadgets<RamT> {
        let num_memory_lines = boot_trace_size_bound + (time_bound + 1) + time_bound; /* boot lines, (time_bound + 1) execution lines (including initial) and time_bound load instruction lines */
        let timestamp_size = log2(num_memory_lines);

        /* allocate all lines on the execution side of the routing network */
        enter_block("Allocate initial state line", false);
        let mut unrouted_memory_lines = vec![];
        let mut execution_lines = Vec::with_capacity(1 + time_bound);
        execution_lines.push(execution_line_variable_gadget::<RamT>::new(
            pb.clone(),
            timestamp_size,
            pb.borrow().ap::<ram_architecture_params<RamT>>(),
            prefix_format!(annotation_prefix, " execution_lines_{}", 0),
        ));
        unrouted_memory_lines.push(execution_lines[0].clone());
        leave_block("Allocate initial state line", false);

        enter_block("Allocate boot lines", false);
        let mut boot_lines = Vec::with_capacity(boot_trace_size_bound);
        for i in 0..boot_trace_size_bound {
            boot_lines.push(memory_line_variable_gadget::<
                RamT,
                execution_line_variable_gadget<RamT>,
            >::new(
                pb.clone(),
                timestamp_size,
                pb.borrow().ap::<ram_architecture_params<RamT>>(),
                prefix_format!(annotation_prefix, " boot_lines_{}", i),
                execution_line_variable_gadget::<RamT>::default(),
            ));
            unrouted_memory_lines.push(boot_lines[i].clone());
        }
        leave_block("Allocate boot lines", false);

        enter_block("Allocate instruction fetch and execution lines", false);
        let mut load_instruction_lines = Vec::with_capacity(time_bound + 1); /* the last line is NOT a memory line, but here just for uniform coding (i.e. the (unused) result of next PC) */
        for i in 0..time_bound {
            load_instruction_lines.push(memory_line_variable_gadget::<
                RamT,
                execution_line_variable_gadget<RamT>,
            >::new(
                pb.clone(),
                timestamp_size,
                pb.borrow().ap::<ram_architecture_params<RamT>>(),
                prefix_format!(annotation_prefix, " load_instruction_lines_{}", i),
                execution_line_variable_gadget::<RamT>::default(),
            ));
            unrouted_memory_lines.push(load_instruction_lines[i].clone());

            execution_lines.push(execution_line_variable_gadget::<RamT>::new(
                pb.clone(),
                timestamp_size,
                pb.borrow().ap::<ram_architecture_params<RamT>>(),
                prefix_format!(annotation_prefix, " execution_lines_{}", i + 1),
            ));
            unrouted_memory_lines.push(execution_lines[i + 1].clone());
        }
        load_instruction_lines.push(memory_line_variable_gadget::<
            RamT,
            execution_line_variable_gadget<RamT>,
        >::new(
            pb.clone(),
            timestamp_size,
            pb.borrow().ap::<ram_architecture_params<RamT>>(),
            prefix_format!(annotation_prefix, " load_instruction_lines_{}", time_bound),
            execution_line_variable_gadget::<RamT>::default(),
        ));
        leave_block("Allocate instruction fetch and execution lines", false);

        /* deal with packing of the input */
        enter_block("Pack input", false);
        let line_size_bits = pb
            .borrow()
            .ap::<ram_architecture_params<RamT>>()
            .address_size()
            + pb.borrow()
                .ap::<ram_architecture_params<RamT>>()
                .value_size();
        let max_chunk_size = FieldT::<RamT>::capacity();
        let packed_line_size = div_ceil(line_size_bits, max_chunk_size).unwrap();
        assert!(packed_input.len() == packed_line_size * boot_trace_size_bound);
        let mut unpack_boot_lines = vec![];

        let mut input_it = 0;
        for i in 0..boot_trace_size_bound {
            /* note the reversed order */
            let mut boot_line_bits = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
            boot_line_bits.extend(
                &boot_lines[boot_trace_size_bound - 1 - i]
                    .t
                    .address
                    .borrow()
                    .t
                    .bits,
            );
            boot_line_bits.extend(
                &boot_lines[boot_trace_size_bound - 1 - i]
                    .t
                    .contents_after
                    .borrow()
                    .t
                    .bits,
            );

            let mut packed_boot_line = pb_variable_array::<FieldT<RamT>, RamT::PB>::new(
                packed_input.contents[input_it..input_it + packed_line_size].to_vec(),
            );
            // std::advance(input_it, packed_line_size);
            input_it += packed_line_size;

            unpack_boot_lines.push(multipacking_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.borrow().clone().into_p(),
                boot_line_bits.clone().into(),
                packed_boot_line.clone().into(),
                max_chunk_size,
                prefix_format!(annotation_prefix, " unpack_boot_lines_{}", i),
            ));
        }
        leave_block("Pack input", false);

        /* deal with routing */
        enter_block("Allocate routed memory lines", false);
        let mut routed_memory_lines = vec![];
        for i in 0..num_memory_lines {
            routed_memory_lines.push(memory_line_variable_gadget::<
                RamT,
                execution_line_variable_gadget<RamT>,
            >::new(
                pb.clone(),
                timestamp_size,
                pb.borrow().ap::<ram_architecture_params<RamT>>().clone(),
                prefix_format!(annotation_prefix, " routed_memory_lines_{}", i),
                execution_line_variable_gadget::<RamT>::default(),
            ));
        }
        leave_block("Allocate routed memory lines", false);

        enter_block("Collect inputs/outputs for the routing network", false);
        let mut routing_inputs = Vec::with_capacity(num_memory_lines);
        let mut routing_outputs = Vec::with_capacity(num_memory_lines);

        for i in 0..num_memory_lines {
            routing_inputs.push(unrouted_memory_lines[i].all_vars());
            routing_outputs.push(routed_memory_lines[i].all_vars());
        }
        leave_block("Collect inputs/outputs for the routing network", false);

        enter_block("Allocate routing network", false);
        let routing_network =
            RcCell::new(as_waksman_routing_gadget::<FieldT<RamT>, RamT::PB>::new(
                pb.borrow().clone().into_p(),
                num_memory_lines,
                routing_inputs.clone(),
                routing_outputs.clone(),
                prefix_format!(annotation_prefix, " routing_network"),
            ));
        leave_block("Allocate routing network", false);

        /* deal with all checkers */
        enter_block("Allocate execution checkers", false);
        let mut execution_checkers = Vec::with_capacity(time_bound);
        for i in 0..time_bound {
            execution_checkers.push(ram_cpu_checker::<RamT>::new(
                pb.borrow().clone().into_p(),
                load_instruction_lines[i].t.address.borrow().t.bits.clone(), // prev_pc_addr
                load_instruction_lines[i]
                    .t
                    .contents_after
                    .borrow()
                    .t
                    .bits
                    .clone(), // prev_pc_val
                execution_lines[i].t.t.cpu_state.clone(),                    // prev_state
                execution_lines[i + 1].t.address.borrow().t.bits.clone(),    // ls_addr,
                execution_lines[i + 1]
                    .t
                    .contents_before
                    .borrow()
                    .t
                    .bits
                    .clone(), // ls_prev_val
                execution_lines[i + 1]
                    .t
                    .contents_after
                    .borrow()
                    .t
                    .bits
                    .clone(), // ls_next_val
                execution_lines[i + 1].t.t.cpu_state.clone(),                // next_state
                load_instruction_lines[i + 1]
                    .t
                    .address
                    .borrow()
                    .t
                    .bits
                    .clone(), // next_pc_addr
                execution_lines[i + 1].t.t.has_accepted.clone(),             // next_has_accepted
                prefix_format!(annotation_prefix, " execution_checkers_{}", i),
            ));
        }
        leave_block("Allocate execution checkers", false);

        enter_block("Allocate all memory checkers", false);
        let mut memory_checkers = Vec::with_capacity(num_memory_lines);
        for i in 0..num_memory_lines {
            memory_checkers.push(memory_checker_gadget::<RamT>::new(
                pb.clone(),
                timestamp_size,
                unrouted_memory_lines[i].clone(),
                routed_memory_lines[i].clone(),
                prefix_format!(annotation_prefix, " memory_checkers_{}", i),
            ));
        }
        leave_block("Allocate all memory checkers", false);

        /* done */
        gadget::<RamT::FieldT, RamT::PB, Self>::new(
            pb.borrow().clone().into_p(),
            annotation_prefix,
            Self {
                num_memory_lines,
                boot_lines,
                boot_line_bits: vec![],
                unpack_boot_lines,
                load_instruction_lines,
                execution_lines,
                unrouted_memory_lines,
                routed_memory_lines,
                execution_checkers,
                memory_checkers,
                routing_inputs,
                routing_outputs,
                routing_network,
                boot_trace_size_bound,
                time_bound,
                packed_input,
            },
        )
    }
}
impl<RamT: ram_params_type> ram_universal_gadgets<RamT> {
    pub fn generate_r1cs_constraints(&self) {
        enter_block(
            "Call to generate_r1cs_constraints of ram_universal_gadget",
            false,
        );
        for i in 0..self.t.boot_trace_size_bound {
            self.t.unpack_boot_lines[i].generate_r1cs_constraints(false);
        }

        /* ensure that we start with all zeros state */
        for i in 0..self
            .pb
            .borrow()
            .ap::<ram_architecture_params<RamT>>()
            .cpu_state_size()
        {
            generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb,
                &(self.t.execution_lines[0].t.t.cpu_state[i].clone().into()),
                &FieldT::<RamT>::zero(),
                String::new(),
            );
        }

        /* ensure increasing timestamps */
        for i in 0..self.t.num_memory_lines {
            generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
                &self.pb,
                &(self.t.unrouted_memory_lines[i]
                    .t
                    .timestamp
                    .borrow()
                    .t
                    .packed
                    .clone()
                    .into()),
                &FieldT::<RamT>::from(i),
                String::new(),
            );
        }

        /* ensure bitness of trace lines on the time side */
        for i in 0..self.t.boot_trace_size_bound {
            self.t.boot_lines[i].generate_r1cs_constraints(true);
        }

        self.t.execution_lines[0].generate_r1cs_constraints(true);
        for i in 0..self.t.time_bound {
            self.t.load_instruction_lines[i].generate_r1cs_constraints(true);
            self.t.execution_lines[i + 1].generate_r1cs_constraints(true);
        }

        /* ensure bitness of trace lines on the memory side */
        for i in 0..self.t.num_memory_lines {
            self.t.routed_memory_lines[i].generate_r1cs_constraints(true);
        }

        /* ensure that load instruction lines really do loads */
        for i in 0..self.t.time_bound {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                    linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                        FieldT::<RamT>::from(1),
                    ),
                    self.t.load_instruction_lines[i]
                        .t
                        .contents_before
                        .borrow()
                        .t
                        .packed
                        .clone()
                        .into(),
                    self.t.load_instruction_lines[i]
                        .t
                        .contents_after
                        .borrow()
                        .t
                        .packed
                        .clone()
                        .into(),
                ),
                prefix_format!(self.annotation_prefix, " load_instruction_{}_is_a_load", i),
            );
        }

        /* ensure correct execution */
        for i in 0..self.t.time_bound {
            self.t.execution_checkers[i].generate_r1cs_constraints();
        }

        /* check memory */
        self.t.routing_network.borrow().generate_r1cs_constraints();

        for i in 0..self.t.num_memory_lines {
            self.t.memory_checkers[i].generate_r1cs_constraints();
        }

        /* ensure that PC started at the prescribed value */
        generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb,
            &(self.t.load_instruction_lines[0]
                .t
                .address
                .borrow()
                .t
                .packed
                .clone()
                .into()),
            &FieldT::<RamT>::from(
                self.pb
                    .borrow()
                    .ap::<ram_architecture_params<RamT>>()
                    .initial_pc_addr(),
            ),
            String::new(),
        );

        /* ensure that the last state was an accepting one */
        generate_r1cs_equals_const_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb,
            &(self.t.execution_lines[self.t.time_bound]
                .t
                .t
                .has_accepted
                .clone()
                .into()),
            &FieldT::<RamT>::one(),
            "last_state_must_be_accepting".to_owned(),
        );

        /* print constraint profiling */
        let num_constraints = self.pb.borrow().num_constraints();
        let num_variables = self.pb.borrow().num_variables();

        if !inhibit_profiling_info {
            print_indent();
            print!("* Number of constraints: {}\n", num_constraints);
            print_indent();
            print!(
                "* Number of constraints / cycle: {:.1}\n",
                1.0 * num_constraints as f64 / self.t.time_bound as f64
            );

            print_indent();
            print!("* Number of variables: {}\n", num_variables);
            print_indent();
            print!(
                "* Number of variables / cycle: {:.1}\n",
                1. * num_variables as f64 / self.t.time_bound as f64
            );
        }
        leave_block(
            "Call to generate_r1cs_constraints of ram_universal_gadget",
            false,
        );
    }

    pub fn generate_r1cs_witness(
        &self,
        boot_trace: &ram_boot_trace,
        auxiliary_input: &ram_input_tape,
    ) {
        /* assign correct timestamps to all lines */
        for i in 0..self.t.num_memory_lines {
            *self.pb.borrow_mut().val_ref(
                &self.t.unrouted_memory_lines[i]
                    .t
                    .timestamp
                    .borrow()
                    .t
                    .packed,
            ) = FieldT::<RamT>::from(i);
            self.t.unrouted_memory_lines[i]
                .t
                .timestamp
                .borrow()
                .generate_r1cs_witness_from_packed();
        }

        /* fill in the initial state */
        let initial_state = self
            .pb
            .borrow()
            .ap::<ram_architecture_params<RamT>>()
            .initial_cpu_state();
        self.t.execution_lines[0]
            .t
            .t
            .cpu_state
            .fill_with_bits(&self.pb, &initial_state);

        /* fill in the boot section */
        let mut memory_after_boot = memory_contents::new();

        for (&boot_pos, &(address, contents)) in boot_trace.get_all_trace_entries() {
            assert!(boot_pos < self.t.boot_trace_size_bound);
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.boot_lines[boot_pos].t.address.borrow().t.packed) =
                FieldT::<RamT>::from(address); //, true
            *self.pb.borrow_mut().val_ref(
                &self.t.boot_lines[boot_pos]
                    .t
                    .contents_after
                    .borrow()
                    .t
                    .packed,
            ) = FieldT::<RamT>::from(contents); //, true
            self.t.boot_lines[boot_pos].generate_r1cs_witness_from_packed();

            memory_after_boot.insert(address, contents);
        }

        /* do the actual execution */
        let mut mem_backend = ra_memory::new3(
            1usize
                << (self
                    .pb
                    .borrow()
                    .ap::<ram_architecture_params<RamT>>()
                    .address_size()),
            self.pb
                .borrow()
                .ap::<ram_architecture_params<RamT>>()
                .value_size(),
            memory_after_boot,
        );
        let mut auxiliary_input_it = 0;

        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.load_instruction_lines[0].t.address.borrow().t.packed) =
            FieldT::<RamT>::from(
                self.pb
                    .borrow()
                    .ap::<ram_architecture_params<RamT>>()
                    .initial_pc_addr(),
            ); //, true
        self.t.load_instruction_lines[0]
            .t
            .address
            .borrow()
            .generate_r1cs_witness_from_packed();

        for i in 0..self.t.time_bound {
            /* load instruction */
            let pc_addr = self
                .pb
                .borrow()
                .val(&self.t.load_instruction_lines[i].t.address.borrow().t.packed)
                .as_ulong();
            let pc_val = mem_backend.get_value(pc_addr);

            *self.pb.borrow_mut().val_ref(
                &self.t.load_instruction_lines[i]
                    .t
                    .contents_before
                    .borrow()
                    .t
                    .packed,
            ) = FieldT::<RamT>::from(pc_val); //, true
            *self.pb.borrow_mut().val_ref(
                &self.t.load_instruction_lines[i]
                    .t
                    .contents_after
                    .borrow()
                    .t
                    .packed,
            ) = FieldT::<RamT>::from(pc_val); //, true
            self.t.load_instruction_lines[i].generate_r1cs_witness_from_packed();

            /* first fetch the address part of the memory */
            self.t.execution_checkers[i].generate_r1cs_witness_address();
            self.t.execution_lines[i + 1]
                .t
                .address
                .borrow()
                .generate_r1cs_witness_from_bits();

            /* fill it in */
            let load_store_addr = self
                .pb
                .borrow()
                .val(&self.t.execution_lines[i + 1].t.address.borrow().t.packed)
                .as_ulong();
            let load_store_prev_val = mem_backend.get_value(load_store_addr);

            *self.pb.borrow_mut().val_ref(
                &self.t.execution_lines[i + 1]
                    .t
                    .contents_before
                    .borrow()
                    .t
                    .packed,
            ) = FieldT::<RamT>::from(load_store_prev_val); //, true
            self.t.execution_lines[i + 1]
                .t
                .contents_before
                .borrow()
                .generate_r1cs_witness_from_packed();

            /* then execute the rest of the instruction */
            self.t.execution_checkers[i].generate_r1cs_witness_other(auxiliary_input);

            /* update the memory possibly changed by the CPU checker */
            self.t.execution_lines[i + 1]
                .t
                .contents_after
                .borrow()
                .generate_r1cs_witness_from_bits();
            let load_store_next_val = self
                .pb
                .borrow()
                .val(
                    &self.t.execution_lines[i + 1]
                        .t
                        .contents_after
                        .borrow()
                        .t
                        .packed,
                )
                .as_ulong();
            mem_backend.set_value(load_store_addr, load_store_next_val);

            /* the next PC address was passed in a bit form, so maintain packed form as well */
            self.t.load_instruction_lines[i + 1]
                .t
                .address
                .borrow()
                .generate_r1cs_witness_from_bits();
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

        // type mem_pair = std::pair<usize, usize>; /* a pair of address, timestamp */
        let mut mem_pairs = vec![];

        for i in 0..self.t.num_memory_lines {
            mem_pairs.push((
                self.pb
                    .borrow()
                    .val(&self.t.unrouted_memory_lines[i].t.address.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.unrouted_memory_lines[i]
                            .t
                            .timestamp
                            .borrow()
                            .t
                            .packed,
                    )
                    .as_ulong(),
            ));
        }

        mem_pairs.sort_unstable();

        let mut pi = integer_permutation::new(self.t.num_memory_lines);
        for i in 0..self.t.num_memory_lines {
            let timestamp = self
                .pb
                .borrow()
                .val(
                    &self.t.unrouted_memory_lines[i]
                        .t
                        .timestamp
                        .borrow()
                        .t
                        .packed,
                )
                .as_ulong();
            let address = self
                .pb
                .borrow()
                .val(&self.t.unrouted_memory_lines[i].t.address.borrow().t.packed)
                .as_ulong();

            let it = mem_pairs.partition_point(|&v| v <= (address, timestamp));
            let prev = if it == mem_pairs.len() { 0 } else { it };
            pi.set(prev, i);
        }

        /* route according to the memory permutation */
        self.t.routing_network.borrow().generate_r1cs_witness(&pi);

        for i in 0..self.t.num_memory_lines {
            self.t.routed_memory_lines[i].generate_r1cs_witness_from_bits();
        }

        /* generate witness for memory checkers */
        for i in 0..self.t.num_memory_lines {
            self.t.memory_checkers[i].generate_r1cs_witness();
        }

        /* repack back the input */
        for i in 0..self.t.boot_trace_size_bound {
            self.t.unpack_boot_lines[i].generate_r1cs_witness_from_bits();
        }

        /* print debugging information */
        if !inhibit_profiling_info {
            print_indent();
            print!(
                "* Protoboard satisfied: {}\n",
                (if self.pb.borrow().is_satisfied() {
                    "YES"
                } else {
                    "no"
                })
            );
        }
    }

    pub fn print_execution_trace(&self) {
        for i in 0..self.t.boot_trace_size_bound {
            print!(
                "Boot process at t=#{}: store {} at {}\n",
                i,
                self.pb
                    .borrow()
                    .val(&self.t.boot_lines[i].t.contents_after.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.boot_lines[i].t.address.borrow().t.packed)
                    .as_ulong()
            );
        }

        for i in 0..self.t.time_bound {
            print!("Execution step {}:\n", i);
            print!(
                "  Loaded instruction {} from address {} (ts = {})\n",
                self.pb
                    .borrow()
                    .val(
                        &self.t.load_instruction_lines[i]
                            .t
                            .contents_after
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.load_instruction_lines[i].t.address.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.load_instruction_lines[i]
                            .t
                            .timestamp
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong()
            );

            print!("  Debugging information from the transition function:\n");
            self.t.execution_checkers[i].dump();

            print!(
                "  Memory operation executed: addr = {}, contents_before = {}, contents_after = {} (ts_before = {}, ts_after = {})\n",
                self.pb
                    .borrow()
                    .val(&self.t.execution_lines[i + 1].t.address.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.execution_lines[i + 1]
                            .t
                            .contents_before
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.execution_lines[i + 1]
                            .t
                            .contents_after
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.execution_lines[i].t.timestamp.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.execution_lines[i + 1].t.timestamp.borrow().t.packed)
                    .as_ulong()
            );
        }
    }

    pub fn print_memory_trace(&self) {
        for i in 0..self.t.num_memory_lines {
            print!("Memory access #{}:\n", i);
            print!(
                "  Time side  : ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
                self.pb
                    .borrow()
                    .val(
                        &self.t.unrouted_memory_lines[i]
                            .t
                            .timestamp
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.unrouted_memory_lines[i].t.address.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.unrouted_memory_lines[i]
                            .t
                            .contents_before
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.unrouted_memory_lines[i]
                            .t
                            .contents_after
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong()
            );
            print!(
                "  Memory side: ts = {}, address = {}, contents_before = {}, contents_after = {}\n",
                self.pb
                    .borrow()
                    .val(&self.t.routed_memory_lines[i].t.timestamp.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(&self.t.routed_memory_lines[i].t.address.borrow().t.packed)
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.routed_memory_lines[i]
                            .t
                            .contents_before
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong(),
                self.pb
                    .borrow()
                    .val(
                        &self.t.routed_memory_lines[i]
                            .t
                            .contents_after
                            .borrow()
                            .t
                            .packed
                    )
                    .as_ulong()
            );
        }
    }

    pub fn packed_input_element_size(ap: &ram_architecture_params<RamT>) -> usize {
        let line_size_bits = ap.address_size() + ap.value_size();
        let max_chunk_size = FieldT::<RamT>::capacity();
        let packed_line_size = div_ceil(line_size_bits, max_chunk_size).unwrap();

        packed_line_size
    }

    pub fn packed_input_size(
        ap: &ram_architecture_params<RamT>,
        boot_trace_size_bound: usize,
    ) -> usize {
        Self::packed_input_element_size(ap) * boot_trace_size_bound
    }
}
