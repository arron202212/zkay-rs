//  Declaration of interfaces for a RAM-to-R1CS reduction, that is, constructing
//  a R1CS ("Rank-1 Constraint System") from a RAM ("Random-Access Machine").

//  The implementation is a thin layer around a "RAM universal gadget", which is
//  where most of the work is done. See gadgets::ram_universal_gadget.hpp for details.

use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::reductions::ram_to_r1cs::gadgets::ram_universal_gadget::{
    ram_universal_gadget, ram_universal_gadgets,
};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_constraint_system, r1cs_primary_input,
};
use crate::relations::ram_computations::memory::memory_store_trace::address_and_value;
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, ram_input_tape,
};
use crate::relations::ram_computations::rams::ram_params::{
    ram_architecture_params, ram_base_field, ram_boot_trace, ram_params_type, ram_protoboard,
};
use ffec::common::profiling::{enter_block, leave_block};
use ffec::field_utils::field_utils::{
    convert_field_element_to_bit_vector1, pack_bit_vector_into_field_element_vector,
};
use rccell::RcCell;
use std::collections::BTreeSet;

type FieldT<RamT> = ram_base_field<RamT>;

#[derive(Clone, Default)]
pub struct ram_to_r1cs<RamT: ram_params_type> {
    pub boot_trace_size_bound: usize,
    pub main_protoboard: RcCell<ram_protoboard<RamT>>,
    pub r1cs_input: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub universal_gadget: RcCell<ram_universal_gadgets<RamT>>,
}

impl<RamT: ram_params_type> ram_to_r1cs<RamT> {
    pub fn new(
        ap: ram_architecture_params<RamT>,
        boot_trace_size_bound: usize,
        time_bound: usize,
    ) -> Self {
        let mut main_protoboard = RcCell::new(ram_protoboard::<RamT>::default());
        let r1cs_input_size =
            ram_universal_gadgets::<RamT>::packed_input_size(&ap, boot_trace_size_bound);
        let mut r1cs_input = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        r1cs_input.allocate(
            &main_protoboard.borrow().clone().into_p(),
            r1cs_input_size,
            "r1cs_input",
        );
        let universal_gadget = RcCell::new(ram_universal_gadget::<RamT>::new(
            main_protoboard.clone(),
            boot_trace_size_bound,
            time_bound,
            r1cs_input.clone(),
            "universal_gadget".to_owned(),
        ));
        main_protoboard
            .borrow_mut()
            .set_input_sizes(r1cs_input_size);
        Self {
            boot_trace_size_bound,
            main_protoboard,
            r1cs_input,
            universal_gadget,
        }
    }

    pub fn instance_map(&self) {
        enter_block("Call to instance_map of ram_to_r1cs", false);
        self.universal_gadget.borrow().generate_r1cs_constraints();
        leave_block("Call to instance_map of ram_to_r1cs", false);
    }

    pub fn get_constraint_system(
        &self,
    ) -> r1cs_constraint_system<ram_base_field<RamT>, pb_variable, pb_linear_combination> {
        self.main_protoboard.borrow().get_constraint_system()
    }

    pub fn auxiliary_input_map(
        &self,
        boot_trace: &ram_boot_trace,
        auxiliary_input: &ram_input_tape,
    ) -> r1cs_primary_input<ram_base_field<RamT>> {
        enter_block("Call to witness_map of ram_to_r1cs", false);
        self.universal_gadget
            .borrow()
            .generate_r1cs_witness(boot_trace, auxiliary_input);
        // #ifdef DEBUG
        let primary_input_from_input_map = Self::primary_input_map(
            &self
                .main_protoboard
                .borrow()
                .ap::<ram_architecture_params<RamT>>(),
            self.boot_trace_size_bound,
            boot_trace,
        );
        let primary_input_from_witness_map = self.main_protoboard.borrow().primary_input();
        assert!(primary_input_from_input_map == primary_input_from_witness_map);
        //#endif
        leave_block("Call to witness_map of ram_to_r1cs", false);
        self.main_protoboard.borrow().auxiliary_input()
    }

    pub fn print_execution_trace(&self) {
        self.universal_gadget.borrow().print_execution_trace();
    }

    pub fn print_memory_trace(&self) {
        self.universal_gadget.borrow().print_memory_trace();
    }

    pub fn pack_primary_input_address_and_value(
        ap: &ram_architecture_params<RamT>,
        av: &address_and_value,
    ) -> Vec<ram_base_field<RamT>> {
        // type FieldT = ram_base_field<RamT>;

        let &(address, contents) = av;

        let address_bits = convert_field_element_to_bit_vector1::<FieldT<RamT>>(
            &FieldT::<RamT>::from(address), //, true
            ap.address_size(),
        );
        let contents_bits = convert_field_element_to_bit_vector1::<FieldT<RamT>>(
            &FieldT::<RamT>::from(contents), //, true
            ap.value_size(),
        );

        let mut trace_element_bits: Vec<_> =
            address_bits.iter().chain(&contents_bits).cloned().collect();

        let trace_element =
            pack_bit_vector_into_field_element_vector::<FieldT<RamT>>(&trace_element_bits);

        trace_element
    }

    pub fn primary_input_map(
        ap: &ram_architecture_params<RamT>,
        boot_trace_size_bound: usize,
        boot_trace: &ram_boot_trace,
    ) -> r1cs_primary_input<ram_base_field<RamT>> {
        // type FieldT = ram_base_field<RamT>;

        let packed_input_element_size =
            ram_universal_gadgets::<RamT>::packed_input_element_size(ap);
        let mut result = r1cs_primary_input::<FieldT<RamT>>::with_capacity(
            ram_universal_gadgets::<RamT>::packed_input_size(ap, boot_trace_size_bound),
        );

        let mut bound_input_locations = BTreeSet::new();

        for (&input_pos, av) in boot_trace.get_all_trace_entries() {
            assert!(input_pos < boot_trace_size_bound);
            assert!(!bound_input_locations.contains(&input_pos));

            let packed_input_element = Self::pack_primary_input_address_and_value(ap, av);
            // std::copy(packed_input_element.begin(), packed_input_element.end(), result.begin() + ));
            let i = packed_input_element_size * (boot_trace_size_bound - 1 - input_pos);
            result.splice(i..i, packed_input_element);
            bound_input_locations.insert(input_pos);
        }

        result
    }
}
