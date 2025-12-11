// Declaration of interfaces for the Merkle tree check read gadget.

// The gadget checks the following: given two roots R1 and R2, address A, two
// values V1 and V2, and authentication path P, check that
// - P is a valid authentication path for the value V1 as the A-th leaf in a Merkle tree with root R1, and
// - P is a valid authentication path for the value V2 as the A-th leaf in a Merkle tree with root R2.

use crate::common::data_structures::merkle_tree::{
    HashTConfig, merkle_authentication_node, merkle_authentication_path,
};
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    bit_vector_copy_gadget, bit_vector_copy_gadgets, generate_boolean_r1cs_constraint,
    packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::digest_selector_gadget::{
    digest_selector_gadget, digest_selector_gadgets,
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
use crate::gadgetlib1::gadgets::merkle_tree::merkle_authentication_path_variable::{
    merkle_authentication_path_variable, merkle_authentication_path_variables,
};
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::common::utils::bit_vector;
use ffec::common::utils::div_ceil;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct merkle_tree_check_update_gadget<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig> {
    //gadget<FieldT>
    prev_hashers: Vec<HashT>,
    prev_hasher_inputs: Vec<block_variables<FieldT, PB>>,
    prev_propagators: Vec<digest_selector_gadgets<FieldT, PB>>,
    prev_internal_output: Vec<digest_variables<FieldT, PB>>,

    next_hashers: Vec<HashT>,
    next_hasher_inputs: Vec<block_variables<FieldT, PB>>,
    next_propagators: Vec<digest_selector_gadgets<FieldT, PB>>,
    next_internal_output: Vec<digest_variables<FieldT, PB>>,

    computed_next_root: RcCell<digest_variables<FieldT, PB>>,
    check_next_root: RcCell<bit_vector_copy_gadgets<FieldT, PB>>,

    digest_size: usize,
    tree_depth: usize,

    address_bits: pb_variable_array<FieldT, PB>,
    prev_leaf_digest: digest_variables<FieldT, PB>,
    prev_root_digest: digest_variables<FieldT, PB>,
    prev_path: merkle_authentication_path_variables<FieldT, PB, HashT>,
    next_leaf_digest: digest_variables<FieldT, PB>,
    next_root_digest: digest_variables<FieldT, PB>,
    next_path: merkle_authentication_path_variables<FieldT, PB, HashT>,
    update_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    /* Note that while it is necessary to generate R1CS constraints
    for prev_path, it is not necessary to do so for next_path. See
    comment in the implementation of generate_r1cs_constraints() */
}

pub type merkle_tree_check_update_gadgets<FieldT, PB, HashT> =
    gadget<FieldT, PB, merkle_tree_check_update_gadget<FieldT, PB, HashT>>;
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_tree_check_update_gadget<FieldT, PB, HashT>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        tree_depth: usize,
        address_bits: pb_variable_array<FieldT, PB>,
        prev_leaf_digest: digest_variables<FieldT, PB>,
        prev_root_digest: digest_variables<FieldT, PB>,
        prev_path: merkle_authentication_path_variables<FieldT, PB, HashT>,
        next_leaf_digest: digest_variables<FieldT, PB>,
        next_root_digest: digest_variables<FieldT, PB>,
        next_path: merkle_authentication_path_variables<FieldT, PB, HashT>,
        update_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> merkle_tree_check_update_gadgets<FieldT, PB, HashT> {
        let mut prev_hashers = vec![];
        let mut prev_hasher_inputs = vec![];
        let mut prev_propagators = vec![];
        let mut prev_internal_output = vec![];
        let mut next_hashers = vec![];
        let mut next_hasher_inputs = vec![];
        let mut next_propagators = vec![];
        let mut next_internal_output = vec![];

        let digest_size = HashT::get_digest_len();
        assert!(tree_depth > 0);
        assert!(tree_depth == address_bits.len());

        for i in 0..tree_depth - 1 {
            prev_internal_output.push(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                prefix_format!(annotation_prefix, " prev_internal_output_{}", i),
            ));
            next_internal_output.push(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                prefix_format!(annotation_prefix, " next_internal_output_{}", i),
            ));
        }

        let computed_next_root = RcCell::new(digest_variable::<FieldT, PB>::new(
            pb.clone(),
            digest_size,
            prefix_format!(annotation_prefix, " computed_root"),
        ));

        for i in 0..tree_depth {
            let mut prev_inp = block_variable::<FieldT, PB>::new3(
                pb.clone(),
                prev_path.t.left_digests[i].clone(),
                prev_path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " prev_inp_{}", i),
            );
            prev_hasher_inputs.push(prev_inp.clone());
            prev_hashers.push(HashT::new5(
                pb.clone(),
                2 * digest_size,
                prev_inp.clone(),
                if i == 0 {
                    prev_root_digest.clone()
                } else {
                    prev_internal_output[i - 1].clone()
                },
                prefix_format!(annotation_prefix, " prev_hashers_{}", i),
            ));

            let mut next_inp = block_variable::<FieldT, PB>::new3(
                pb.clone(),
                next_path.t.left_digests[i].clone(),
                next_path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " next_inp_{}", i),
            );
            next_hasher_inputs.push(next_inp.clone());
            next_hashers.push(HashT::new5(
                pb.clone(),
                2 * digest_size,
                next_inp.clone(),
                if i == 0 {
                    computed_next_root.borrow().clone()
                } else {
                    next_internal_output[i - 1].clone()
                },
                prefix_format!(annotation_prefix, " next_hashers_{}", i),
            ));
        }

        for i in 0..tree_depth {
            prev_propagators.push(digest_selector_gadget::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                if i < tree_depth - 1 {
                    prev_internal_output[i].clone()
                } else {
                    prev_leaf_digest.clone()
                },
                address_bits[tree_depth - 1 - i].clone().into(),
                prev_path.t.left_digests[i].clone(),
                prev_path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " prev_propagators_{}", i),
            ));
            next_propagators.push(digest_selector_gadget::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                if i < tree_depth - 1 {
                    next_internal_output[i].clone()
                } else {
                    next_leaf_digest.clone()
                },
                address_bits[tree_depth - 1 - i].clone().into(),
                next_path.t.left_digests[i].clone(),
                next_path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " next_propagators_{}", i),
            ));
        }

        let check_next_root = RcCell::new(bit_vector_copy_gadget::<FieldT, PB>::new(
            pb.clone(),
            computed_next_root.borrow().t.bits.clone(),
            next_root_digest.t.bits.clone(),
            update_successful.clone(),
            FieldT::capacity(),
            prefix_format!(annotation_prefix, " check_next_root"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                prev_hashers,
                prev_hasher_inputs,
                prev_propagators,
                prev_internal_output,
                next_hashers,
                next_hasher_inputs,
                next_propagators,
                next_internal_output,
                computed_next_root,
                check_next_root,
                digest_size,
                tree_depth,
                address_bits,
                prev_leaf_digest,
                prev_root_digest,
                prev_path,
                next_leaf_digest,
                next_root_digest,
                next_path,
                update_successful,
            },
        )
    }

    pub fn root_size_in_bits() -> usize {
        return HashT::get_digest_len();
    }

    pub fn expected_constraints(tree_depth: usize) -> usize {
        /* NB: this includes path constraints */
        let prev_hasher_constraints = tree_depth * HashT::expected_constraints(false);
        let next_hasher_constraints = tree_depth * HashT::expected_constraints(true);
        let prev_authentication_path_constraints = 2 * tree_depth * HashT::get_digest_len();
        let prev_propagator_constraints = tree_depth * HashT::get_digest_len();
        let next_propagator_constraints = tree_depth * HashT::get_digest_len();
        let check_next_root_constraints =
            3 * div_ceil(HashT::get_digest_len(), FieldT::capacity()).unwrap();
        let aux_equality_constraints = tree_depth * HashT::get_digest_len();

        return (prev_hasher_constraints
            + next_hasher_constraints
            + prev_authentication_path_constraints
            + prev_propagator_constraints
            + next_propagator_constraints
            + check_next_root_constraints
            + aux_equality_constraints);
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_tree_check_update_gadgets<FieldT, PB, HashT>
{
    pub fn generate_r1cs_constraints(&self) {
        /* ensure correct hash computations */
        for i in 0..self.t.tree_depth {
            self.t.prev_hashers[i].generate_r1cs_constraints(false); // we check root outside and prev_left/prev_right above
            self.t.next_hashers[i].generate_r1cs_constraints(true); // however we must check right side hashes
        }

        /* ensure consistency of internal_left/internal_right with internal_output */
        for i in 0..self.t.tree_depth {
            self.t.prev_propagators[i].generate_r1cs_constraints();
            self.t.next_propagators[i].generate_r1cs_constraints();
        }

        /* ensure that prev auxiliary input and next auxiliary input match */
        for i in 0..self.t.tree_depth {
            for j in 0..self.t.digest_size {
                /*
                  addr * (prev_left - next_left) + (1 - addr) * (prev_right - next_right) = 0
                  addr * (prev_left - next_left - prev_right + next_right) = next_right - prev_right
                */
                self.pb.borrow_mut().add_r1cs_constraint(
                    r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                        self.t.address_bits[self.t.tree_depth - 1 - i].clone().into(),
                        self.t.prev_path.t.left_digests[i].t.bits[j].clone()
                            - linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(self.t.next_path.t.left_digests[i].t.bits[j].clone())
                            - self.t.prev_path.t.right_digests[i].t.bits[j].clone()
                            + self.t.next_path.t.right_digests[i].t.bits[j].clone(),
                        self.t.next_path.t.right_digests[i].t.bits[j].clone() - linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(self.t.prev_path.t.right_digests[i].t.bits[j].clone()),
                    ),
                    prefix_format!(self.annotation_prefix, " aux_check_{}_{}", i, j),
                );
            }
        }

        /* Note that while it is necessary to generate R1CS constraints
        for prev_path, it is not necessary to do so for next_path.

        This holds, because { next_path.left_inputs[i],
        next_path.right_inputs[i] } is a pair { hash_output,
        auxiliary_input }. The bitness for hash_output is enforced
        above by next_hashers[i].generate_r1cs_constraints.

        Because auxiliary input is the same for prev_path and next_path
        (enforced above), we have that auxiliary_input part is also
        constrained to be boolean, because prev_path is *all*
        constrained to be all boolean. */

        self.t
            .check_next_root
            .borrow()
            .generate_r1cs_constraints(false, false);
    }

    pub fn generate_r1cs_witness(&self) {
        /* do the hash computations bottom-up */
        for i in (0..=self.t.tree_depth - 1).rev() {
            /* ensure consistency of prev_path and next_path */
            if self
                .pb
                .borrow()
                .val(&self.t.address_bits[self.t.tree_depth - 1 - i])
                == FieldT::one()
            {
                self.t.next_path.t.left_digests[i]
                    .generate_r1cs_witness(&self.t.prev_path.t.left_digests[i].get_digest());
            } else {
                self.t.next_path.t.right_digests[i]
                    .generate_r1cs_witness(&self.t.prev_path.t.right_digests[i].get_digest());
            }

            /* propagate previous input */
            self.t.prev_propagators[i].generate_r1cs_witness();
            self.t.next_propagators[i].generate_r1cs_witness();

            /* compute hash */
            self.t.prev_hashers[i].generate_r1cs_witness();
            self.t.next_hashers[i].generate_r1cs_witness();
        }

        self.t.check_next_root.borrow().generate_r1cs_witness();
    }
}

pub fn test_merkle_tree_check_update_gadget<
    FieldT: FieldTConfig,
    PB: PBConfig,
    HashT: HashTConfig,
>() {
    /* prepare test */
    let digest_len = HashT::get_digest_len();

    let tree_depth = 16;
    let mut prev_path = vec![merkle_authentication_node::new(); tree_depth];

    let mut prev_load_hash: Vec<_> = (0..digest_len)
        .map(|_| rand::random::<usize>() % 2 != 0)
        .collect();
    let mut prev_store_hash: Vec<_> = (0..digest_len)
        .map(|_| rand::random::<usize>() % 2 != 0)
        .collect();

    let loaded_leaf = prev_load_hash.clone();
    let stored_leaf = prev_store_hash.clone();

    let mut address_bits = vec![];

    let mut address = 0;
    for level in (0..=tree_depth - 1).rev() {
        let mut computed_is_right = (rand::random::<usize>() % 2 != 0);
        address |= if computed_is_right {
            1usize << (tree_depth - 1 - level)
        } else {
            0
        };
        address_bits.push(computed_is_right);
        let mut other: Vec<_> = (0..digest_len)
            .map(|_| rand::random::<usize>() % 2 != 0)
            .collect();

        let mut load_block = prev_load_hash.clone();
        if computed_is_right {
            load_block.splice(0..0, other.clone());
        } else {
            load_block.extend(other.clone());
        }

        let mut store_block = prev_store_hash.clone();
        if computed_is_right {
            store_block.splice(0..0, other.clone());
        } else {
            store_block.extend(other.clone());
        }

        let mut load_h = HashT::get_hash(load_block);
        let mut store_h = HashT::get_hash(store_block);

        prev_path[level] = other;

        prev_load_hash = load_h;
        prev_store_hash = store_h;
    }

    let mut load_root = prev_load_hash.clone();
    let mut store_root = prev_store_hash.clone();

    /* execute the test */
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut address_bits_va = pb_variable_array::<FieldT, PB>::default();
    address_bits_va.allocate(&pb, tree_depth, "address_bits");
    let mut prev_leaf_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "prev_leaf_digest".to_owned());
    let mut prev_root_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "prev_root_digest".to_owned());
    let mut prev_path_var = merkle_authentication_path_variable::<FieldT, PB, HashT>::new(
        pb.clone(),
        tree_depth,
        "prev_path_var".to_owned(),
    );
    let mut next_leaf_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "next_leaf_digest".to_owned());
    let mut next_root_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "next_root_digest".to_owned());
    let mut next_path_var = merkle_authentication_path_variable::<FieldT, PB, HashT>::new(
        pb.clone(),
        tree_depth,
        "next_path_var".to_owned(),
    );
    let mut mls = merkle_tree_check_update_gadget::<FieldT, PB, HashT>::new(
        pb.clone(),
        tree_depth.clone(),
        address_bits_va.clone(),
        prev_leaf_digest.clone(),
        prev_root_digest.clone(),
        prev_path_var.clone(),
        next_leaf_digest.clone(),
        next_root_digest.clone(),
        next_path_var.clone(),
        variable::<FieldT, pb_variable>::from(ONE).into(),
        "mls".to_owned(),
    );

    prev_path_var.generate_r1cs_constraints();
    mls.generate_r1cs_constraints();

    address_bits_va.fill_with_bits(&pb, &address_bits);
    assert!(address_bits_va.get_field_element_from_bits(&pb).as_ulong() == address);
    prev_leaf_digest.generate_r1cs_witness(&loaded_leaf);
    prev_path_var.generate_r1cs_witness(address, prev_path);
    next_leaf_digest.generate_r1cs_witness(&stored_leaf);
    address_bits_va.fill_with_bits(&pb, &address_bits);
    mls.generate_r1cs_witness();

    /* make sure that update check will check for the right things */
    prev_leaf_digest.generate_r1cs_witness(&loaded_leaf);
    next_leaf_digest.generate_r1cs_witness(&stored_leaf);
    prev_root_digest.generate_r1cs_witness(&load_root);
    next_root_digest.generate_r1cs_witness(&store_root);
    address_bits_va.fill_with_bits(&pb, &address_bits);
    assert!(pb.borrow().is_satisfied());

    let num_constraints = pb.borrow().num_constraints();
    let expected_constraints =
        merkle_tree_check_update_gadget::<FieldT, PB, HashT>::expected_constraints(tree_depth);
    assert!(num_constraints == expected_constraints);
}
