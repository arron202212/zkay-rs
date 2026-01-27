// Declaration of interfaces for memory_checker_gadget, a gadget that verifies the
// consistency of two accesses to memory that are adjacent in a "memory sort".

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    comparison_gadget, comparison_gadgets, generate_boolean_r1cs_constraint,
};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::prefix_format;
use crate::reductions::ram_to_r1cs::gadgets::trace_lines::{
    execution_line_variable_gadget, execution_line_variable_gadgets, memory_line_variable_gadget,
    memory_line_variable_gadgets,
};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, ram_architecture_params, ram_base_field, ram_params_type,
    ram_protoboard,
};
use crate::relations::variable::{linear_combination, variable};
use ffec::{One, Zero};
use rccell::RcCell;

type FieldT<RamT> = ram_base_field<RamT>;

#[derive(Clone, Default)]
pub struct memory_checker_gadget<RamT: ram_params_type> {
    // : public ram_gadget_base
    pub timestamps_leq: variable<FieldT<RamT>, pb_variable>,
    pub timestamps_less: variable<FieldT<RamT>, pb_variable>,
    pub compare_timestamps: RcCell<comparison_gadgets<FieldT<RamT>, RamT::PB>>,
    pub addresses_eq: variable<FieldT<RamT>, pb_variable>,
    pub addresses_leq: variable<FieldT<RamT>, pb_variable>,
    pub addresses_less: variable<FieldT<RamT>, pb_variable>,
    pub compare_addresses: RcCell<comparison_gadgets<FieldT<RamT>, RamT::PB>>,
    pub loose_contents_after1_equals_contents_before2: variable<FieldT<RamT>, pb_variable>,
    pub loose_contents_before2_equals_zero: variable<FieldT<RamT>, pb_variable>,
    pub loose_timestamp2_is_zero: variable<FieldT<RamT>, pb_variable>,
    pub line1: execution_line_variable_gadgets<RamT>,
    pub line2: execution_line_variable_gadgets<RamT>,
}

pub type memory_checker_gadgets<RamT> =
    gadget<<RamT as ppTConfig>::FieldT, <RamT as ppTConfig>::PB, memory_checker_gadget<RamT>>;

impl<RamT: ram_params_type> memory_checker_gadget<RamT> {
    pub fn new(
        pb: RcCell<ram_protoboard<RamT>>,
        timestamp_size: usize,
        line1: memory_line_variable_gadgets<RamT, execution_line_variable_gadget<RamT>>,
        line2: memory_line_variable_gadgets<RamT, execution_line_variable_gadget<RamT>>,
        annotation_prefix: String,
    ) -> memory_checker_gadgets<RamT> {
        /* compare the two timestamps */
        let mut timestamps_leq = variable::<FieldT<RamT>, pb_variable>::default();
        let mut timestamps_less = variable::<FieldT<RamT>, pb_variable>::default();
        let mut addresses_eq = variable::<FieldT<RamT>, pb_variable>::default();
        let mut addresses_leq = variable::<FieldT<RamT>, pb_variable>::default();
        let mut addresses_less = variable::<FieldT<RamT>, pb_variable>::default();
        let mut loose_contents_after1_equals_contents_before2 =
            variable::<FieldT<RamT>, pb_variable>::default();
        let mut loose_contents_before2_equals_zero =
            variable::<FieldT<RamT>, pb_variable>::default();
        let mut loose_timestamp2_is_zero = variable::<FieldT<RamT>, pb_variable>::default();
        timestamps_leq.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " timestamps_leq"),
        );
        timestamps_less.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " timestamps_less"),
        );
        let compare_timestamps = RcCell::new(comparison_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.borrow().clone().into_p(),
            timestamp_size,
            line1.t.timestamp.borrow().t.packed.clone().into(),
            line2.t.timestamp.borrow().t.packed.clone().into(),
            timestamps_less.clone(),
            timestamps_leq.clone(),
            prefix_format!(annotation_prefix, " compare_ts"),
        ));

        /* compare the two addresses */
        let address_size = pb
            .borrow()
            .ap::<ram_architecture_params<RamT>>()
            .address_size();
        addresses_eq.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " addresses_eq"),
        );
        addresses_leq.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " addresses_leq"),
        );
        addresses_less.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " addresses_less"),
        );
        let compare_addresses = RcCell::new(comparison_gadget::<FieldT<RamT>, RamT::PB>::new(
            pb.borrow().clone().into_p(),
            address_size,
            line1.t.address.borrow().t.packed.clone().into(),
            line2.t.address.borrow().t.packed.clone().into(),
            addresses_less.clone(),
            addresses_leq.clone(),
            prefix_format!(annotation_prefix, " compare_addresses"),
        ));

        /*
         Add variables that will contain flags representing the following relations:
         - "line1.contents_after = line2.contents_before" (to check that contents do not change between instructions);
         - "line2.contents_before = 0" (for the first access at an address); and
         - "line2.timestamp = 0" (for wrap-around checks to ensure only one 'cycle' in the memory sort).

         More precisely, each of the above flags is "loose" (i.e., it equals 0 if
         the relation holds, but can be either 0 or 1 if the relation does not hold).
        */
        loose_contents_after1_equals_contents_before2.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(
                annotation_prefix,
                " loose_contents_after1_equals_contents_before2",
            ),
        );
        loose_contents_before2_equals_zero.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " loose_contents_before2_equals_zero",),
        );
        loose_timestamp2_is_zero.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " loose_timestamp2_is_zero"),
        );
        gadget::<RamT::FieldT, RamT::PB, Self>::new(
            pb.borrow().clone().into_p(),
            annotation_prefix,
            Self {
                timestamps_leq,
                timestamps_less,
                compare_timestamps,
                addresses_eq,
                addresses_leq,
                addresses_less,
                compare_addresses,
                loose_contents_after1_equals_contents_before2,
                loose_contents_before2_equals_zero,
                loose_timestamp2_is_zero,
                line1,
                line2,
            },
        )
    }
}
impl<RamT: ram_params_type> memory_checker_gadgets<RamT> {
    pub fn generate_r1cs_constraints(&self) {
        /* compare the two timestamps */
        self.t
            .compare_timestamps
            .borrow()
            .generate_r1cs_constraints();

        /* compare the two addresses */
        self.t
            .compare_addresses
            .borrow()
            .generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.addresses_leq.clone().into(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.addresses_less.clone(),
                self.t.addresses_eq.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " addresses_eq"),
        );

        /*
         Add constraints for the following three flags:
          - loose_contents_after1_equals_contents_before2;
          - loose_contents_before2_equals_zero;
          - loose_timestamp2_is_zero.
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t
                    .loose_contents_after1_equals_contents_before2
                    .clone()
                    .into(),
                self.t.line1.t.contents_after.borrow().t.packed.clone()
                    - linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                        self.t.line2.t.contents_before.borrow().t.packed.clone(),
                    ),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(
                self.annotation_prefix,
                " loose_contents_after1_equals_contents_before2",
            ),
        );
        generate_boolean_r1cs_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb,
            &(self
                .t
                .loose_contents_after1_equals_contents_before2
                .clone()
                .into()),
            prefix_format!(
                self.annotation_prefix,
                " loose_contents_after1_equals_contents_before2",
            ),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.loose_contents_before2_equals_zero.clone().into(),
                self.t
                    .line2
                    .t
                    .contents_before
                    .borrow()
                    .t
                    .packed
                    .clone()
                    .into(),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(
                self.annotation_prefix,
                " loose_contents_before2_equals_zero",
            ),
        );
        generate_boolean_r1cs_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb,
            &(self.t.loose_contents_before2_equals_zero.clone().into()),
            prefix_format!(
                self.annotation_prefix,
                " loose_contents_before2_equals_zero",
            ),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.loose_timestamp2_is_zero.clone().into(),
                self.t.line2.t.timestamp.borrow().t.packed.clone().into(),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(self.annotation_prefix, " loose_timestamp2_is_zero"),
        );
        generate_boolean_r1cs_constraint::<FieldT<RamT>, RamT::PB>(
            &self.pb,
            &(self.t.loose_timestamp2_is_zero.clone().into()),
            prefix_format!(self.annotation_prefix, " loose_timestamp2_is_zero"),
        );

        /*
          The three cases that need to be checked are:

          line1.address = line2.address => line1.contents_after = line2.contents_before
          (i.e. contents do not change between accesses to the same address)

          line1.address < line2.address => line2.contents_before = 0
          (i.e. access to new address has the "before" value set to 0)

          line1.address > line2.address => line2.timestamp = 0
          (i.e. there is only one cycle with non-decreasing addresses, except
          for the case where we go back to a unique pre-set timestamp; we choose
          timestamp 0 to be the one that touches address 0)

          As usual, we implement "A => B" as "NOT (A AND (NOT B))".
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.addresses_eq.clone().into(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.loose_contents_after1_equals_contents_before2.clone(),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(
                self.annotation_prefix,
                " memory_retains_contents_between_accesses",
            ),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                self.t.addresses_less.clone().into(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.loose_contents_before2_equals_zero.clone(),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(self.annotation_prefix, " new_address_starts_at_zero"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT<RamT>, pb_variable, pb_linear_combination>::new(
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.addresses_leq.clone(),
                linear_combination::<FieldT<RamT>, pb_variable, pb_linear_combination>::from(
                    FieldT::<RamT>::from(1),
                ) - self.t.loose_timestamp2_is_zero.clone(),
                FieldT::<RamT>::from(0).into(),
            ),
            prefix_format!(self.annotation_prefix, " only_one_cycle"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        /* compare the two addresses */
        self.t.compare_addresses.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.addresses_eq) =
            self.pb.borrow().val(&self.t.addresses_leq)
                * (FieldT::<RamT>::one() - self.pb.borrow().val(&self.t.addresses_less));

        /* compare the two timestamps */
        self.t.compare_timestamps.borrow().generate_r1cs_witness();

        /*
         compare the values of:
         - loose_contents_after1_equals_contents_before2;
         - loose_contents_before2_equals_zero;
         - loose_timestamp2_is_zero.
        */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.loose_contents_after1_equals_contents_before2) = if (self
            .pb
            .borrow()
            .val(&self.t.line1.t.contents_after.borrow().t.packed)
            == self
                .pb
                .borrow()
                .val(&self.t.line2.t.contents_before.borrow().t.packed))
        {
            FieldT::<RamT>::one()
        } else {
            FieldT::<RamT>::zero()
        };
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.loose_contents_before2_equals_zero) = if self
            .pb
            .borrow()
            .val(&self.t.line2.t.contents_before.borrow().t.packed)
            .is_zero()
        {
            FieldT::<RamT>::one()
        } else {
            FieldT::<RamT>::zero()
        };
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.loose_timestamp2_is_zero) = (if self
            .pb
            .borrow()
            .val(&self.t.line2.t.timestamp.borrow().t.packed)
            == FieldT::<RamT>::zero()
        {
            FieldT::<RamT>::one()
        } else {
            FieldT::<RamT>::zero()
        });
    }
}
