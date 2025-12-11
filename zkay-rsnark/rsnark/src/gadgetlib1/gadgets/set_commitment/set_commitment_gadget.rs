use crate::common::data_structures::merkle_tree::{HashTConfig, merkle_authentication_path};
use crate::common::data_structures::set_commitment::set_commitment_accumulator;
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
use crate::gadgetlib1::gadgets::merkle_tree::merkle_tree_check_read_gadget::{
    merkle_tree_check_read_gadget, merkle_tree_check_read_gadgets,
};
use crate::gadgetlib1::gadgets::set_commitment::set_membership_proof_variable::set_membership_proof_variable;
use crate::gadgetlib1::gadgets::set_commitment::set_membership_proof_variable::set_membership_proof_variables;
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
use ffec::common::utils::log2;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

pub type set_commitment_variable<FieldT, PB> = digest_variable<FieldT, PB>;
pub type set_commitment_variables<FieldT, PB> = digest_variables<FieldT, PB>;

#[derive(Clone, Default)]
pub struct set_commitment_gadget<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig> {
    //gadget<FieldT>
    pub element_block: RcCell<block_variables<FieldT, PB>>,
    pub element_digest: RcCell<digest_variables<FieldT, PB>>,
    pub hash_element: RcCell<HashT>,
    pub check_membership: RcCell<merkle_tree_check_read_gadgets<FieldT, PB, HashT>>,

    pub tree_depth: usize,
    pub element_bits: pb_variable_array<FieldT, PB>,
    pub root_digest: set_commitment_variables<FieldT, PB>,
    pub proof: set_membership_proof_variables<FieldT, PB, HashT>,
    pub check_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
}

pub type set_commitment_gadgets<FieldT, PB, HashT> =
    gadget<FieldT, PB, set_commitment_gadget<FieldT, PB, HashT>>;
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    set_commitment_gadget<FieldT, PB, HashT>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        max_entries: usize,
        element_bits: pb_variable_array<FieldT, PB>,
        root_digest: set_commitment_variables<FieldT, PB>,
        proof: set_membership_proof_variables<FieldT, PB, HashT>,
        check_successful: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> set_commitment_gadgets<FieldT, PB, HashT> {
        let tree_depth = log2(max_entries);
        let element_block = RcCell::new(block_variable::<FieldT, PB>::new2(
            pb.clone(),
            vec![element_bits.clone()],
            prefix_format!(annotation_prefix, " element_block"),
        ));

        let (element_digest, hash_element, check_membership) = if tree_depth == 0 {
            (
                RcCell::new(digest_variables::<FieldT, PB>::default()),
                RcCell::new(HashT::new5(
                    pb.clone(),
                    element_bits.len(),
                    element_block.borrow().clone(),
                    root_digest.clone(),
                    prefix_format!(annotation_prefix, " hash_element"),
                )),
                RcCell::new(merkle_tree_check_read_gadgets::<FieldT, PB, HashT>::default()),
            )
        } else {
            let element_digest = RcCell::new(digest_variable::<FieldT, PB>::new(
                pb.clone(),
                HashT::get_digest_len(),
                prefix_format!(annotation_prefix, " element_digest"),
            ));
            let hash_element = RcCell::new(HashT::new5(
                pb.clone(),
                element_bits.len(),
                element_block.borrow().clone(),
                element_digest.borrow().clone(),
                prefix_format!(annotation_prefix, " hash_element"),
            ));
            let check_membership =
                RcCell::new(merkle_tree_check_read_gadget::<FieldT, PB, HashT>::new(
                    pb.clone(),
                    tree_depth,
                    proof.t.address_bits.clone().into(),
                    element_digest.borrow().clone(),
                    root_digest.clone(),
                    proof.t.merkle_path.borrow().clone(),
                    check_successful.clone(),
                    prefix_format!(annotation_prefix, " check_membership"),
                ));
            (element_digest, hash_element, check_membership)
        };
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                element_block,
                element_digest,
                hash_element,
                check_membership,
                tree_depth,
                element_bits,
                root_digest,
                proof,
                check_successful,
            },
        )
    }
    pub fn root_size_in_bits() -> usize {
        return merkle_tree_check_read_gadget::<FieldT, PB, HashT>::root_size_in_bits();
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>
    set_commitment_gadgets<FieldT, PB, HashT>
{
    pub fn generate_r1cs_constraints(&self) {
        self.t
            .hash_element
            .borrow()
            .generate_r1cs_constraints(false);

        if self.t.tree_depth > 0 {
            self.t.check_membership.borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.hash_element.borrow().generate_r1cs_witness();

        if self.t.tree_depth > 0 {
            self.t.check_membership.borrow().generate_r1cs_witness();
        }
    }
}

pub fn test_set_commitment_gadget<FieldT: FieldTConfig, PB: PBConfig, HashT: HashTConfig>()
where
    [(); { FieldT::num_limbs as usize }]:,
{
    let digest_len = HashT::get_digest_len();
    let max_set_size = 16;
    let value_size = (if HashT::get_block_len() > 0 {
        HashT::get_block_len()
    } else {
        10
    });

    let mut accumulator = set_commitment_accumulator::<HashT>::new(max_set_size, value_size);

    let mut set_elems = vec![];
    for i in 0..max_set_size {
        let mut elem: Vec<_> = (0..value_size)
            .map(|_| rand::random::<usize>() % 2 != 0)
            .collect();
        set_elems.push(elem.clone());
        accumulator.add(&elem);
        assert!(accumulator.is_in_set(&elem));
    }

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut element_bits = pb_variable_array::<FieldT, PB>::default();
    element_bits.allocate(&pb, value_size, "element_bits");
    let mut root_digest = set_commitment_variable::<FieldT, PB>::new(
        pb.clone(),
        digest_len,
        "root_digest".to_owned(),
    );

    let mut check_succesful = variable::<FieldT, pb_variable>::default();
    check_succesful.allocate(&pb, "check_succesful".to_owned());

    let mut proof = set_membership_proof_variable::<FieldT, PB, HashT>::new(
        pb.clone(),
        max_set_size,
        "proof".to_owned(),
    );

    let mut sc = set_commitment_gadget::<FieldT, PB, HashT>::new(
        pb.clone(),
        max_set_size,
        element_bits.clone(),
        root_digest.clone(),
        proof.clone(),
        check_succesful.clone().into(),
        "sc".to_owned(),
    );
    sc.generate_r1cs_constraints();

    /* test all elements from set */
    for i in 0..max_set_size {
        element_bits.fill_with_bits(&pb, &set_elems[i]);
        *pb.borrow_mut().val_ref(&check_succesful) = FieldT::one();
        proof.generate_r1cs_witness(&accumulator.get_membership_proof(&set_elems[i]));
        sc.generate_r1cs_witness();
        root_digest.generate_r1cs_witness(&accumulator.get_commitment());
        assert!(pb.borrow().is_satisfied());
    }
    print!("membership tests OK\n");

    /* test an element not in set */
    for i in 0..value_size {
        *pb.borrow_mut().val_ref(&element_bits[i]) = FieldT::from(rand::random::<usize>() % 2);
    }

    *pb.borrow_mut().val_ref(&check_succesful) = FieldT::zero(); /* do not require the check result to be successful */
    proof.generate_r1cs_witness(&accumulator.get_membership_proof(&set_elems[0])); /* try it with invalid proof */
    sc.generate_r1cs_witness();
    root_digest.generate_r1cs_witness(&accumulator.get_commitment());
    assert!(pb.borrow().is_satisfied());

    *pb.borrow_mut().val_ref(&check_succesful) = FieldT::one(); /* now require the check result to be succesful */
    proof.generate_r1cs_witness(&accumulator.get_membership_proof(&set_elems[0])); /* try it with invalid proof */
    sc.generate_r1cs_witness();
    root_digest.generate_r1cs_witness(&accumulator.get_commitment());
    assert!(!pb.borrow().is_satisfied()); /* the protoboard should be unsatisfied */
    print!("non-membership test OK\n");
}
