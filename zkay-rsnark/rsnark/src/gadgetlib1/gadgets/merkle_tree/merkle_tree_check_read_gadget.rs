// Declaration of interfaces for the Merkle tree check read gadget.

// The gadget checks the following: given a root R, address A, value V, and
// authentication path P, check that P is a valid authentication path for the
// value V as the A-th leaf in a Merkle tree with root R.

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
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use ffec::common::utils::div_ceil;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct merkle_tree_check_read_gadget<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig> {
    //gadget<FieldT>
    hashers: Vec<HashT>,
    hasher_inputs: Vec<block_variables<FieldT, PB>>,
    propagators: Vec<digest_selector_gadgets<FieldT, PB>>,
    internal_output: Vec<digest_variables<FieldT, PB>>,

    computed_root: RcCell<digest_variables<FieldT, PB>>,
    check_root: RcCell<bit_vector_copy_gadgets<FieldT, PB>>,

    digest_size: usize,
    tree_depth: usize,
    address_bits: pb_linear_combination_array<FieldT, PB>,
    leaf: digest_variables<FieldT, PB>,
    root: digest_variables<FieldT, PB>,
    path: merkle_authentication_path_variables<FieldT, PB, HashT>,
    read_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
}

pub type merkle_tree_check_read_gadgets<FieldT, PB, HashT> =
    gadget<FieldT, PB, merkle_tree_check_read_gadget<FieldT, PB, HashT>>;

impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_tree_check_read_gadget<FieldT, PB, HashT>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        tree_depth: usize,
        address_bits: pb_linear_combination_array<FieldT, PB>,
        leaf: digest_variables<FieldT, PB>,
        root: digest_variables<FieldT, PB>,
        path: merkle_authentication_path_variables<FieldT, PB, HashT>,
        read_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> merkle_tree_check_read_gadgets<FieldT, PB, HashT> {
        let digest_size = HashT::get_digest_len();
        let mut internal_output = vec![];
        let mut hasher_inputs = vec![];
        let mut hashers = vec![];
        let mut propagators = vec![];
        /*
           The tricky part here is ordering. For Merkle tree
           authentication paths, path[0] corresponds to one layer below
           the root (and path[tree_depth-1] corresponds to the layer
           containing the leaf), while address_bits has the reverse order:
           address_bits[0] is LSB, and corresponds to layer containing the
           leaf, and address_bits[tree_depth-1] is MSB, and corresponds to
           the subtree directly under the root.
        */
        assert!(tree_depth > 0);
        assert!(tree_depth == address_bits.len());

        for i in 0..tree_depth - 1 {
            internal_output.push(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                prefix_format!(annotation_prefix, " internal_output_{}", i),
            ));
        }

        let computed_root = RcCell::new(digest_variable::<FieldT, PB>::new(
            pb.clone(),
            digest_size,
            prefix_format!(annotation_prefix, " computed_root"),
        ));

        for i in 0..tree_depth {
            let mut inp = block_variable::<FieldT, PB>::new3(
                pb.clone(),
                path.t.left_digests[i].clone(),
                path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " inp_{}", i),
            );
            hasher_inputs.push(inp.clone());
            hashers.push(HashT::new5(
                pb.clone(),
                2 * digest_size,
                inp.clone(),
                if i == 0 {
                    computed_root.borrow().clone()
                } else {
                    internal_output[i - 1].clone()
                },
                prefix_format!(annotation_prefix, " load_hashers_{}", i),
            ));
        }

        for i in 0..tree_depth {
            /*
              The propagators take a computed hash value (or leaf in the
              base case) and propagate it one layer up, either in the left
              or the right slot of authentication_path_variable.
            */
            propagators.push(digest_selector_gadget::<FieldT, PB>::new(
                pb.clone(),
                digest_size,
                if i < tree_depth - 1 {
                    internal_output[i].clone()
                } else {
                    leaf.clone()
                },
                address_bits[tree_depth - 1 - i].clone(),
                path.t.left_digests[i].clone(),
                path.t.right_digests[i].clone(),
                prefix_format!(annotation_prefix, " digest_selector_{}", i),
            ));
        }

        let check_root = RcCell::new(bit_vector_copy_gadget::<FieldT, PB>::new(
            pb.clone(),
            computed_root.borrow().t.bits.clone(),
            root.t.bits.clone(),
            read_successful.clone(),
            FieldT::capacity(),
            prefix_format!(annotation_prefix, " check_root"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                hashers,
                hasher_inputs,
                propagators,
                internal_output,
                computed_root,
                check_root,
                digest_size,
                tree_depth,
                address_bits,
                leaf,
                root,
                path,
                read_successful,
            },
        )
    }

    pub fn root_size_in_bits() -> usize {
        return HashT::get_digest_len();
    }

    pub fn expected_constraints(tree_depth: usize) -> usize {
        /* NB: this includes path constraints */
        let hasher_constraints = tree_depth * HashT::expected_constraints(false);
        let propagator_constraints = tree_depth * HashT::get_digest_len();
        let authentication_path_constraints = 2 * tree_depth * HashT::get_digest_len();
        let check_root_constraints =
            div_ceil(HashT::get_digest_len(), FieldT::capacity()).unwrap() * 3;

        return hasher_constraints
            + propagator_constraints
            + authentication_path_constraints
            + check_root_constraints;
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    merkle_tree_check_read_gadgets<FieldT, PB, HashT>
{
    pub fn generate_r1cs_constraints(&self) {
        /* ensure correct hash computations */
        for i in 0..self.t.tree_depth {
            // Note that we check root outside and have enforced booleanity of path.left_digests/path.right_digests outside in path.generate_r1cs_constraints
            self.t.hashers[i].generate_r1cs_constraints(false);
        }

        /* ensure consistency of path.left_digests/path.right_digests with internal_output */
        for i in 0..self.t.tree_depth {
            self.t.propagators[i].generate_r1cs_constraints();
        }

        self.t
            .check_root
            .borrow()
            .generate_r1cs_constraints(false, false);
    }

    pub fn generate_r1cs_witness(&self) {
        /* do the hash computations bottom-up */
        for i in (0..=self.t.tree_depth - 1).rev() {
            /* propagate previous input */
            self.t.propagators[i].generate_r1cs_witness();

            /* compute hash */
            self.t.hashers[i].generate_r1cs_witness();
        }

        self.t.check_root.borrow().generate_r1cs_witness();
    }
}

pub fn test_merkle_tree_check_read_gadget<
    FieldT: FieldTConfig,
    PB: PBConfig,
    HashT: HashTConfig,
>() {
    /* prepare test */
    let digest_len = HashT::get_digest_len();
    let tree_depth = 16;
    let mut path = vec![merkle_authentication_node::new(); tree_depth];

    let mut prev_hash: Vec<_> = (0..digest_len)
        .map(|_| rand::random::<usize>() % 2 != 0)
        .collect();
    let mut leaf = prev_hash.clone();

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
            .map(|_| (rand::random::<usize>() % 2) != 0)
            .collect();

        let mut block = prev_hash.clone();
        if computed_is_right {
            block.splice(0..0, other.clone());
        } else {
            block.extend(other.clone());
        }

        let mut h = HashT::get_hash(&block);

        path[level] = other;

        prev_hash = h.clone();
    }
    let mut root = prev_hash.clone();

    /* execute test */
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut address_bits_va = pb_variable_array::<FieldT, PB>::default();
    address_bits_va.allocate(&pb, tree_depth, "address_bits");
    let mut leaf_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "input_block".to_owned());
    let mut root_digest =
        digest_variable::<FieldT, PB>::new(pb.clone(), digest_len, "output_digest".to_owned());
    let mut path_var = merkle_authentication_path_variable::<FieldT, PB, HashT>::new(
        pb.clone(),
        tree_depth,
        "path_var".to_owned(),
    );
    let mut ml = merkle_tree_check_read_gadget::<FieldT, PB, HashT>::new(
        pb.clone(),
        tree_depth.clone(),
        address_bits_va.clone().into(),
        leaf_digest.clone(),
        root_digest.clone(),
        path_var.clone(),
        variable::<FieldT, pb_variable>::from(ONE).into(),
        "ml".to_owned(),
    );

    path_var.generate_r1cs_constraints();
    ml.generate_r1cs_constraints();

    address_bits_va.fill_with_bits(&pb, &address_bits);
    assert!(address_bits_va.get_field_element_from_bits(&pb).as_ulong() == address as usize);
    leaf_digest.generate_r1cs_witness(&leaf);
    path_var.generate_r1cs_witness(address as usize, path);
    ml.generate_r1cs_witness();

    /* make sure that read checker didn't accidentally overwrite anything */
    address_bits_va.fill_with_bits(&pb, &address_bits);
    leaf_digest.generate_r1cs_witness(&leaf);
    root_digest.generate_r1cs_witness(&root);
    assert!(pb.borrow().is_satisfied());

    let num_constraints = pb.borrow().num_constraints();
    let expected_constraints =
        merkle_tree_check_read_gadget::<FieldT, PB, HashT>::expected_constraints(tree_depth);
    assert!(num_constraints == expected_constraints);
}
