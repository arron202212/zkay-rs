// Declaration of interfaces for trace-line variables.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{dual_variable_gadget, dual_variable_gadgets};
use crate::gadgetlib1::gadgets::pairing::pairing_params::ppTConfig;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::ProtoboardConfig;
use crate::prefix_format;
use crate::relations::ram_computations::rams::ram_params::{
    ArchitectureParamsTypeConfig, ram_architecture_params, ram_base_field, ram_params_type,
    ram_protoboard,
};
use crate::relations::variable::variable;
use rccell::RcCell;

/**
 * A memory line contains variables for the following:
 * - timestamp
 * - address
 * - contents_before
 * - contents_after
 *
 * Memory lines are used by memory_checker_gadget.
 */
//

type FieldT<RamT> = ram_base_field<RamT>;

#[derive(Clone, Default)]
pub struct memory_line_variable_gadget<RamT: ram_params_type, T: Default + Clone> {
    //: public ram_gadget_base
    pub timestamp: RcCell<dual_variable_gadgets<FieldT<RamT>, RamT::PB, RamT::DV>>,
    pub address: RcCell<dual_variable_gadgets<FieldT<RamT>, RamT::PB, RamT::DV>>,
    pub contents_before: RcCell<dual_variable_gadgets<FieldT<RamT>, RamT::PB, RamT::DV>>,
    pub contents_after: RcCell<dual_variable_gadgets<FieldT<RamT>, RamT::PB, RamT::DV>>,
    pub t: T,
}

/**
 * An execution line inherits from a memory line and, in addition, contains
 * variables for a CPU state and for a flag denoting if the machine has accepted.
 *
 * Execution lines are used by execution_checker_gadget.
 */
// type FieldT=ram_base_field<RamT> ;

#[derive(Clone, Default)]
pub struct execution_line_variable_gadget<RamT: ram_params_type> {
    // / : public memory_line_variable_gadget
    pub cpu_state: pb_variable_array<FieldT<RamT>, RamT::PB>,
    pub has_accepted: variable<FieldT<RamT>, pb_variable>,
}

pub type memory_line_variable_gadgets<RamT, T> = gadget<
    <RamT as ppTConfig>::FieldT,
    <RamT as ppTConfig>::PB,
    memory_line_variable_gadget<RamT, T>,
>;
impl<RamT: ram_params_type, T: Default + Clone> memory_line_variable_gadget<RamT, T> {
    pub fn new(
        pb: RcCell<ram_protoboard<RamT>>,
        timestamp_size: usize,
        ap: ram_architecture_params<RamT>,
        annotation_prefix: String,
        t: T,
    ) -> memory_line_variable_gadgets<RamT, T> {
        let address_size = ap.address_size();
        let value_size = ap.value_size();

        let timestamp = RcCell::new(
            dual_variable_gadget::<FieldT<RamT>, RamT::PB, RamT::DV>::new(
                pb.borrow().clone().into_p(),
                timestamp_size,
                prefix_format!(annotation_prefix, " timestamp"),
                RamT::DV::default(),
            ),
        );
        let address = RcCell::new(
            dual_variable_gadget::<FieldT<RamT>, RamT::PB, RamT::DV>::new(
                pb.borrow().clone().into_p(),
                address_size,
                prefix_format!(annotation_prefix, " address"),
                RamT::DV::default(),
            ),
        );
        let contents_before = RcCell::new(
            dual_variable_gadget::<FieldT<RamT>, RamT::PB, RamT::DV>::new(
                pb.borrow().clone().into_p(),
                value_size,
                prefix_format!(annotation_prefix, " contents_before"),
                RamT::DV::default(),
            ),
        );
        let contents_after = RcCell::new(
            dual_variable_gadget::<FieldT<RamT>, RamT::PB, RamT::DV>::new(
                pb.borrow().clone().into_p(),
                value_size,
                prefix_format!(annotation_prefix, " contents_after"),
                RamT::DV::default(),
            ),
        );
        gadget::<RamT::FieldT, RamT::PB, Self>::new(
            pb.borrow().clone().into_p(),
            annotation_prefix,
            Self {
                timestamp,
                address,
                contents_before,
                contents_after,
                t,
            },
        )
    }
}
impl<RamT: ram_params_type, T: Default + Clone> memory_line_variable_gadgets<RamT, T> {
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        self.t
            .timestamp
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
        self.t
            .address
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
        self.t
            .contents_before
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
        self.t
            .contents_after
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        self.t.timestamp.borrow().generate_r1cs_witness_from_bits();
        self.t.address.borrow().generate_r1cs_witness_from_bits();
        self.t
            .contents_before
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .contents_after
            .borrow()
            .generate_r1cs_witness_from_bits();
    }

    pub fn generate_r1cs_witness_from_packed(&self) {
        self.t
            .timestamp
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t.address.borrow().generate_r1cs_witness_from_packed();
        self.t
            .contents_before
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .contents_after
            .borrow()
            .generate_r1cs_witness_from_packed();
    }

    pub fn all_vars(&self) -> pb_variable_array<FieldT<RamT>, RamT::PB> {
        let mut r = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        r.extend(&self.t.timestamp.borrow().t.bits);
        r.extend(&self.t.address.borrow().t.bits);
        r.extend(&self.t.contents_before.borrow().t.bits);
        r.extend(&self.t.contents_after.borrow().t.bits);

        r
    }
}

pub type execution_line_variable_gadgets<RamT> =
    memory_line_variable_gadgets<RamT, execution_line_variable_gadget<RamT>>;

impl<RamT: ram_params_type> execution_line_variable_gadget<RamT> {
    pub fn new(
        pb: RcCell<ram_protoboard<RamT>>,
        timestamp_size: usize,
        ap: ram_architecture_params<RamT>,
        annotation_prefix: String,
    ) -> execution_line_variable_gadgets<RamT> {
        let cpu_state_size = ap.cpu_state_size();
        let mut cpu_state = pb_variable_array::<FieldT<RamT>, RamT::PB>::default();
        let mut has_accepted = variable::<FieldT<RamT>, pb_variable>::default();
        cpu_state.allocate(
            &pb.borrow().clone().into_p(),
            cpu_state_size,
            prefix_format!(annotation_prefix, " cpu_state"),
        );
        has_accepted.allocate(
            &pb.borrow().clone().into_p(),
            prefix_format!(annotation_prefix, " has_accepted"),
        );
        memory_line_variable_gadget::<RamT, Self>::new(
            pb,
            timestamp_size,
            ap,
            annotation_prefix,
            Self {
                cpu_state,
                has_accepted,
            },
        )
    }
}
