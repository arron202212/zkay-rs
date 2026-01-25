use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::generate_boolean_r1cs_constraint;
use crate::gadgetlib1::gadgets::hashes::hash_io::{digest_variable, digest_variables};
use crate::gadgetlib1::pb_variable::{pb_linear_combination, pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use rccell::RcCell;

#[derive(Clone, Default)]
pub struct digest_selector_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>
    pub digest_size: usize,
    pub input: digest_variables<FieldT, PB>,
    pub is_right: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub left: digest_variables<FieldT, PB>,
    pub right: digest_variables<FieldT, PB>,
}

pub type digest_selector_gadgets<FieldT, PB> =
    gadget<FieldT, PB, digest_selector_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> digest_selector_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        digest_size: usize,
        input: digest_variables<FieldT, PB>,
        is_right: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        left: digest_variables<FieldT, PB>,
        right: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> digest_selector_gadgets<FieldT, PB> {
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                digest_size,
                input,
                is_right,
                left,
                right,
            },
        )
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> digest_selector_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.digest_size {
            /*
              input = is_right * right + (1-is_right) * left
              input - left = is_right(right - left)
            */
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.is_right.clone().into(),
                    (linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.right.t.bits[i].clone(),
                    ) - linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.left.t.bits[i].clone(),
                    ))
                    .into(),
                    (linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.input.t.bits[i].clone(),
                    ) - linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.left.t.bits[i].clone(),
                    ))
                    .into(),
                ),
                prefix_format!(self.annotation_prefix, " propagate_{}", i),
            );
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.is_right.evaluate_pb(&self.pb);

        assert!(
            self.pb.borrow().lc_val(&self.t.is_right) == FieldT::one()
                || self.pb.borrow().lc_val(&self.t.is_right) == FieldT::zero()
        );
        if self.pb.borrow().lc_val(&self.t.is_right) == FieldT::one() {
            for i in 0..self.t.digest_size {
                *self.pb.borrow_mut().val_ref(&self.t.right.t.bits[i]) =
                    self.pb.borrow().val(&self.t.input.t.bits[i]);
            }
        } else {
            for i in 0..self.t.digest_size {
                *self.pb.borrow_mut().val_ref(&self.t.left.t.bits[i]) =
                    self.pb.borrow().val(&self.t.input.t.bits[i]);
            }
        }
    }
}
