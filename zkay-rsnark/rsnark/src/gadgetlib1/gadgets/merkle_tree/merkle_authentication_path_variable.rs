/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP_
// #define MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP_

use crate::common::data_structures::merkle_tree;
use libsnark/gadgetlib1/gadget;
use libsnark/gadgetlib1/gadgets/hashes/hash_io;



template<typename FieldT, typename HashT>
class merkle_authentication_path_variable : public gadget<FieldT> {
public:

    const size_t tree_depth;
    std::vector<digest_variable<FieldT> > left_digests;
    std::vector<digest_variable<FieldT> > right_digests;

    merkle_authentication_path_variable(protoboard<FieldT> &pb,
                                        const size_t tree_depth,
                                        const std::string &annotation_prefix);

    void generate_r1cs_constraints();
    void generate_r1cs_witness(const size_t address, const merkle_authentication_path &path);
    merkle_authentication_path get_authentication_path(const size_t address) const;
};



use libsnark/gadgetlib1/gadgets/merkle_tree/merkle_authentication_path_variable;

//#endif // MERKLE_AUTHENTICATION_PATH_VARIABLE_HPP
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC_
// #define MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC_



template<typename FieldT, typename HashT>
merkle_authentication_path_variable<FieldT, HashT>::merkle_authentication_path_variable(protoboard<FieldT> &pb,
                                                                                        const size_t tree_depth,
                                                                                        const std::string &annotation_prefix) :
    gadget<FieldT>(pb, annotation_prefix),
    tree_depth(tree_depth)
{
    for (size_t i = 0; i < tree_depth; ++i)
    {
        left_digests.push(digest_variable<FieldT>(pb, HashT::get_digest_len(), FMT(annotation_prefix, " left_digests_{}", i)));
        right_digests.push(digest_variable<FieldT>(pb, HashT::get_digest_len(), FMT(annotation_prefix, " right_digests_{}", i)));
    }
}

template<typename FieldT, typename HashT>
void merkle_authentication_path_variable<FieldT, HashT>::generate_r1cs_constraints()
{
    for (size_t i = 0; i < tree_depth; ++i)
    {
        left_digests[i].generate_r1cs_constraints();
        right_digests[i].generate_r1cs_constraints();
    }
}

template<typename FieldT, typename HashT>
void merkle_authentication_path_variable<FieldT, HashT>::generate_r1cs_witness(const size_t address, const merkle_authentication_path &path)
{
    assert!(path.size() == tree_depth);

    for (size_t i = 0; i < tree_depth; ++i)
    {
        if (address & (1ul << (tree_depth-1-i)))
        {
            left_digests[i].generate_r1cs_witness(path[i]);
        }
        else
        {
            right_digests[i].generate_r1cs_witness(path[i]);
        }
    }
}

template<typename FieldT, typename HashT>
merkle_authentication_path merkle_authentication_path_variable<FieldT, HashT>::get_authentication_path(const size_t address) const
{
    merkle_authentication_path result;
    for (size_t i = 0; i < tree_depth; ++i)
    {
        if (address & (1ul << (tree_depth-1-i)))
        {
            result.push(left_digests[i].get_digest());
        }
        else
        {
            result.push(right_digests[i].get_digest());
        }
    }

    return result;
}



//#endif // MERKLE_AUTHENTICATION_PATH_VARIABLE_TCC
