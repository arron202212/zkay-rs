use crate::common::data_structures::merkle_tree::{HashTConfig, merkle_authentication_path};
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
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;
#[derive(Clone, Default)]
pub struct merkle_authentication_path_variable<
    FieldT: FieldTConfig,
    PB: PBConfig,
    HashT: HashTConfig,
> {
    //gadget<FieldT>
    pub tree_depth: usize,
    pub left_digests: Vec<digest_variables<FieldT, PB>>,
    pub right_digests: Vec<digest_variables<FieldT, PB>>,
    _t: PhantomData<HashT>,
}

pub type merkle_authentication_path_variables<FieldT, PB, HashT> =
    gadget<FieldT, PB, merkle_authentication_path_variable<FieldT, PB, HashT>>;
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_authentication_path_variable<FieldT, PB, HashT>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        tree_depth: usize,
        annotation_prefix: String,
    ) -> merkle_authentication_path_variables<FieldT, PB, HashT> {
        let (mut left_digests, mut right_digests) = (vec![], vec![]);
        for i in 0..tree_depth {
            left_digests.push(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                HashT::get_digest_len(),
                prefix_format!(annotation_prefix, " left_digests_{}", i),
            ));
            right_digests.push(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                HashT::get_digest_len(),
                prefix_format!(annotation_prefix, " right_digests_{}", i),
            ));
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                left_digests,
                right_digests,
                tree_depth,
                _t: PhantomData,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_authentication_path_variables<FieldT, PB, HashT>
{
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.tree_depth {
            self.t.left_digests[i].generate_r1cs_constraints();
            self.t.right_digests[i].generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self, address: usize, path: merkle_authentication_path) {
        assert!(path.len() == self.t.tree_depth);

        for i in 0..self.t.tree_depth {
            if address & (1usize << (self.t.tree_depth - 1 - i)) != 0 {
                self.t.left_digests[i].generate_r1cs_witness(&path[i]);
            } else {
                self.t.right_digests[i].generate_r1cs_witness(&path[i]);
            }
        }
    }

    pub fn get_authentication_path(&self, address: usize) -> merkle_authentication_path {
        let mut result = merkle_authentication_path::new();
        for i in 0..self.t.tree_depth {
            if address & (1usize << (self.t.tree_depth - 1 - i)) != 0 {
                result.push(self.t.left_digests[i].get_digest());
            } else {
                result.push(self.t.right_digests[i].get_digest());
            }
        }

        return result;
    }
}
