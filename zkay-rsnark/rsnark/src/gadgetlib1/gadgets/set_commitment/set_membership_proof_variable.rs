use crate::common::data_structures::merkle_tree::{HashTConfig, merkle_authentication_path};
use crate::common::data_structures::set_commitment::set_membership_proof;
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::hash_io::{
    block_variable, block_variables, digest_variable, digest_variables,
};
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_aux::{
    big_sigma_gadget, big_sigma_gadgets, choice_gadget, choice_gadgets, lastbits_gadget,
    lastbits_gadgets, majority_gadget, majority_gadgets, small_sigma_gadget, small_sigma_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components;
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components::{
    SHA256_K, SHA256_block_size, SHA256_default_IV, SHA256_digest_size,
    sha256_message_schedule_gadget, sha256_message_schedule_gadgets, sha256_round_function_gadget,
    sha256_round_function_gadgets,
};
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable::merkle_authentication_path_variable;
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable::merkle_authentication_path_variables;
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_variable_assignment;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use ffec::common::utils::log2;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct set_membership_proof_variable<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig> {
    //gadget<FieldT>
    pub address_bits: pb_variable_array<FieldT, PB>,
    pub merkle_path: RcCell<merkle_authentication_path_variables<FieldT, PB, HashT>>,

    pub max_entries: usize,
    pub tree_depth: usize,
}

pub type set_membership_proof_variables<FieldT, PB, HashT> =
    gadget<FieldT, PB, set_membership_proof_variable<FieldT, PB, HashT>>;
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    set_membership_proof_variable<FieldT, PB, HashT>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        max_entries: usize,
        annotation_prefix: String,
    ) -> set_membership_proof_variables<FieldT, PB, HashT> {
        let tree_depth = log2(max_entries);
        let mut address_bits = pb_variable_array::<FieldT, PB>::default();
        let mut merkle_path =
            RcCell::new(merkle_authentication_path_variables::<FieldT, PB, HashT>::default());
        if tree_depth > 0 {
            address_bits.allocate(
                &pb,
                tree_depth,
                &prefix_format!(annotation_prefix, " address_bits"),
            );
            merkle_path = RcCell::new(
                merkle_authentication_path_variable::<FieldT, PB, HashT>::new(
                    pb.clone(),
                    tree_depth,
                    prefix_format!(annotation_prefix, " merkle_path"),
                ),
            );
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                address_bits,
                merkle_path,
                max_entries,
                tree_depth: log2(max_entries),
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    set_membership_proof_variables<FieldT, PB, HashT>
{
    pub fn generate_r1cs_constraints(&self) {
        if self.t.tree_depth > 0 {
            for i in 0..self.t.tree_depth {
                generate_boolean_r1cs_constraint::<FieldT, PB>(
                    &self.pb,
                    &(self.t.address_bits[i].clone().into()),
                    prefix_format!(self.annotation_prefix, " address_bits"),
                );
            }
            self.t.merkle_path.borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self, proof: &set_membership_proof) {
        if self.t.tree_depth > 0 {
            self.t
                .address_bits
                .fill_with_bits_of_field_element(&self.pb, &FieldT::from(proof.address));
            self.t
                .merkle_path
                .borrow()
                .generate_r1cs_witness(proof.address, proof.merkle_path.clone());
        }
    }

    pub fn get_membership_proof(&self) -> set_membership_proof {
        let mut result = set_membership_proof::default();

        if self.t.tree_depth == 0 {
            result.address = 0;
        } else {
            result.address = self
                .t
                .address_bits
                .get_field_element_from_bits(&self.pb)
                .as_ulong();
            result.merkle_path = self
                .t
                .merkle_path
                .borrow()
                .get_authentication_path(result.address);
        }

        result
    }

    pub fn as_r1cs_variable_assignment(
        &self,
        proof: &set_membership_proof,
    ) -> r1cs_variable_assignment<FieldT> {
        let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
        let max_entries = (1usize << (proof.merkle_path.len()));
        let mut proof_variable = set_membership_proof_variable::<FieldT, PB, HashT>::new(
            pb.clone(),
            max_entries,
            "proof_variable".to_owned(),
        );
        proof_variable.generate_r1cs_witness(proof);

        return pb.borrow().full_variable_assignment();
    }
}
