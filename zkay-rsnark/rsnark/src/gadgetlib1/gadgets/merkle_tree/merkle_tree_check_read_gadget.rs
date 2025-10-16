/** @file
 *****************************************************************************

 Declaration of interfaces for the Merkle tree check read gadget.

 The gadget checks the following: given a root R, address A, value V, and
 authentication path P, check that P is a valid authentication path for the
 value V as the A-th leaf in a Merkle tree with root R.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_READ_GADGET_HPP_
// #define MERKLE_TREE_CHECK_READ_GADGET_HPP_

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::hashes::crh_gadget;
use crate::gadgetlib1::gadgets::hashes::digest_selector_gadget;
use crate::gadgetlib1::gadgets::hashes::hash_io;
use crate::gadgetlib1::gadgets/merkle_tree/merkle_authentication_path_variable;



template<typename FieldT, typename HashT>
class merkle_tree_check_read_gadget : public gadget<FieldT> {
private:

    std::vector<HashT> hashers;
    std::vector<block_variable<FieldT> > hasher_inputs;
    std::vector<digest_selector_gadget<FieldT> > propagators;
    std::vector<digest_variable<FieldT> > internal_output;

    std::shared_ptr<digest_variable<FieldT> > computed_root;
    std::shared_ptr<bit_vector_copy_gadget<FieldT> > check_root;

public:

    const size_t digest_size;
    const size_t tree_depth;
    pb_linear_combination_array<FieldT> address_bits;
    digest_variable<FieldT> leaf;
    digest_variable<FieldT> root;
    merkle_authentication_path_variable<FieldT, HashT> path;
    pb_linear_combination<FieldT> read_successful;

    merkle_tree_check_read_gadget(protoboard<FieldT> &pb,
                                  const size_t tree_depth,
                                  const pb_linear_combination_array<FieldT> &address_bits,
                                  const digest_variable<FieldT> &leaf_digest,
                                  const digest_variable<FieldT> &root_digest,
                                  const merkle_authentication_path_variable<FieldT, HashT> &path,
                                  const pb_linear_combination<FieldT> &read_successful,
                                  const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness();

    static size_t root_size_in_bits();
    /* for debugging purposes */
    static size_t expected_constraints(const size_t tree_depth);
};

template<typename FieldT, typename HashT>
void test_merkle_tree_check_read_gadget();



use crate::gadgetlib1::gadgets/merkle_tree/merkle_tree_check_read_gadget;

//#endif // MERKLE_TREE_CHECK_READ_GADGET_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the Merkle tree check read.

 See merkle_tree_check_read_gadget.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_TREE_CHECK_READ_GADGET_TCC_
// #define MERKLE_TREE_CHECK_READ_GADGET_TCC_



template<typename FieldT, typename HashT>
merkle_tree_check_read_gadget<FieldT, HashT>::merkle_tree_check_read_gadget(protoboard<FieldT> &pb,
                                                                            const size_t tree_depth,
                                                                            const pb_linear_combination_array<FieldT> &address_bits,
                                                                            const digest_variable<FieldT> &leaf,
                                                                            const digest_variable<FieldT> &root,
                                                                            const merkle_authentication_path_variable<FieldT, HashT> &path,
                                                                            const pb_linear_combination<FieldT> &read_successful,
                                                                            const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    digest_size(HashT::get_digest_len()),
    tree_depth(tree_depth),
    address_bits(address_bits),
    leaf(leaf),
    root(root),
    path(path),
    read_successful(read_successful)
{
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
    assert!(tree_depth == address_bits.size());

    for i in 0..tree_depth-1
    {
        internal_output.push(digest_variable<FieldT>(pb, digest_size, FMT(self.annotation_prefix, " internal_output_{}", i)));
    }

    computed_root.reset(new digest_variable<FieldT>(pb, digest_size, FMT(self.annotation_prefix, " computed_root")));

    for i in 0..tree_depth
    {
        block_variable<FieldT> inp(pb, path.left_digests[i], path.right_digests[i], FMT(self.annotation_prefix, " inp_{}", i));
        hasher_inputs.push(inp);
        hashers.push(HashT(pb, 2*digest_size, inp, (i == 0 ? *computed_root : internal_output[i-1]),
                                   FMT(self.annotation_prefix, " load_hashers_{}", i)));
    }

    for i in 0..tree_depth
    {
        /*
          The propagators take a computed hash value (or leaf in the
          base case) and propagate it one layer up, either in the left
          or the right slot of authentication_path_variable.
        */
        propagators.push(digest_selector_gadget<FieldT>(pb, digest_size, i < tree_depth - 1 ? internal_output[i] : leaf,
                                                                address_bits[tree_depth-1-i], path.left_digests[i], path.right_digests[i],
                                                                FMT(self.annotation_prefix, " digest_selector_{}", i)));
    }

    check_root.reset(new bit_vector_copy_gadget<FieldT>(pb, computed_root->bits, root.bits, read_successful, FieldT::capacity(), FMT(annotation_prefix, " check_root")));
}

template<typename FieldT, typename HashT>
void merkle_tree_check_read_gadget<FieldT, HashT>::generate_r1cs_constraints()
{
    /* ensure correct hash computations */
    for i in 0..tree_depth
    {
        // Note that we check root outside and have enforced booleanity of path.left_digests/path.right_digests outside in path.generate_r1cs_constraints
        hashers[i].generate_r1cs_constraints(false);
    }

    /* ensure consistency of path.left_digests/path.right_digests with internal_output */
    for i in 0..tree_depth
    {
        propagators[i].generate_r1cs_constraints();
    }

    check_root->generate_r1cs_constraints(false, false);
}

template<typename FieldT, typename HashT>
void merkle_tree_check_read_gadget<FieldT, HashT>::generate_r1cs_witness()
{
    /* do the hash computations bottom-up */
    for i in ( 0..=tree_depth-1).rev()
    {
        /* propagate previous input */
        propagators[i].generate_r1cs_witness();

        /* compute hash */
        hashers[i].generate_r1cs_witness();
    }

    check_root->generate_r1cs_witness();
}

template<typename FieldT, typename HashT>
size_t merkle_tree_check_read_gadget<FieldT, HashT>::root_size_in_bits()
{
    return HashT::get_digest_len();
}

template<typename FieldT, typename HashT>
size_t merkle_tree_check_read_gadget<FieldT, HashT>::expected_constraints(const size_t tree_depth)
{
    /* NB: this includes path constraints */
    const size_t hasher_constraints = tree_depth * HashT::expected_constraints(false);
    const size_t propagator_constraints = tree_depth * HashT::get_digest_len();
    const size_t authentication_path_constraints = 2 * tree_depth * HashT::get_digest_len();
    const size_t check_root_constraints = 3 * ffec::div_ceil(HashT::get_digest_len(), FieldT::capacity());

    return hasher_constraints + propagator_constraints + authentication_path_constraints + check_root_constraints;
}

template<typename FieldT, typename HashT>
void test_merkle_tree_check_read_gadget()
{
    /* prepare test */
    const size_t digest_len = HashT::get_digest_len();
    const size_t tree_depth = 16;
    std::vector<merkle_authentication_node> path(tree_depth);

    ffec::bit_vector prev_hash(digest_len);
    std::generate(prev_hash.begin(), prev_hash.end(), [&]() { return std::rand() % 2; });
    ffec::bit_vector leaf = prev_hash;

    ffec::bit_vector address_bits;

    size_t address = 0;
    for level in ( 0..=tree_depth-1).rev()
    {
        const bool computed_is_right = (std::rand() % 2);
        address |= (computed_is_right ? 1ul << (tree_depth-1-level) : 0);
        address_bits.push_back(computed_is_right);
        ffec::bit_vector other(digest_len);
        std::generate(other.begin(), other.end(), [&]() { return std::rand() % 2; });

        ffec::bit_vector block = prev_hash;
        block.insert(computed_is_right ? block.begin() : block.end(), other.begin(), other.end());
        ffec::bit_vector h = HashT::get_hash(block);

        path[level] = other;

        prev_hash = h;
    }
    ffec::bit_vector root = prev_hash;

    /* execute test */
    protoboard<FieldT> pb;
    pb_variable_array<FieldT> address_bits_va;
    address_bits_va.allocate(pb, tree_depth, "address_bits");
    digest_variable<FieldT> leaf_digest(pb, digest_len, "input_block");
    digest_variable<FieldT> root_digest(pb, digest_len, "output_digest");
    merkle_authentication_path_variable<FieldT, HashT> path_var(pb, tree_depth, "path_var");
    merkle_tree_check_read_gadget<FieldT, HashT> ml(pb, tree_depth, address_bits_va, leaf_digest, root_digest, path_var, ONE, "ml");

    path_var.generate_r1cs_constraints();
    ml.generate_r1cs_constraints();

    address_bits_va.fill_with_bits(pb, address_bits);
    assert!(address_bits_va.get_field_element_from_bits(pb).as_ulong() == address);
    leaf_digest.generate_r1cs_witness(leaf);
    path_var.generate_r1cs_witness(address, path);
    ml.generate_r1cs_witness();

    /* make sure that read checker didn't accidentally overwrite anything */
    address_bits_va.fill_with_bits(pb, address_bits);
    leaf_digest.generate_r1cs_witness(leaf);
    root_digest.generate_r1cs_witness(root);
    assert!(pb.is_satisfied());

    const size_t num_constraints = pb.num_constraints();
    const size_t expected_constraints = merkle_tree_check_read_gadget<FieldT, HashT>::expected_constraints(tree_depth);
    assert!(num_constraints == expected_constraints);
}



//#endif // MERKLE_TREE_CHECK_READ_GADGET_TCC_
