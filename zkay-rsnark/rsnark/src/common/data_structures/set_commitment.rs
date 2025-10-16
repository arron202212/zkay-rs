/** @file
 *****************************************************************************

 Declaration of interfaces for a Merkle tree based set commitment scheme.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_COMMITMENT_HPP_
// #define SET_COMMITMENT_HPP_

use ffec::common::utils;

use crate::common::data_structures::merkle_tree;
use crate::gadgetlib1::gadgets::hashes::hash_io; // TODO: the current structure is suboptimal



type ffec::bit_vector set_commitment;

struct set_membership_proof {
    size_t address;
    merkle_authentication_path merkle_path;

    bool operator==(const set_membership_proof &other) const;
    size_t size_in_bits() const;
    friend std::ostream& operator<<(std::ostream &out, const set_membership_proof &other);
    friend std::istream& operator>>(std::istream &in, set_membership_proof &other);
};

template<typename HashT>
class set_commitment_accumulator {
private:
    std::shared_ptr<merkle_tree<HashT> > tree;
    std::map<ffec::bit_vector, size_t> hash_to_pos;
public:

    size_t depth;
    size_t digest_size;
    size_t value_size;

    set_commitment_accumulator(const size_t max_entries, const size_t value_size=0);

    void add(const ffec::bit_vector &value);
    bool is_in_set(const ffec::bit_vector &value) const;
    set_commitment get_commitment() const;

    set_membership_proof get_membership_proof(const ffec::bit_vector &value) const;
};



/* note that set_commitment has both .cpp, for implementation of
   non-templatized code (methods of set_membership_proof) and .tcc
   (implementation of set_commitment_accumulator<HashT> */
use crate::common::data_structures::set_commitment;

//#endif // SET_COMMITMENT_HPP_
/**
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use ffec::common::serialization;

use crate::common::data_structures::set_commitment;



bool set_membership_proof::operator==(const set_membership_proof &other) const
{
    return (self.address == other.address &&
            self.merkle_path == other.merkle_path);
}

size_t set_membership_proof::size_in_bits() const
{
    if merkle_path.empty()
    {
        return (8 * sizeof(address));
    }
    else
    {
        return (8 * sizeof(address) + merkle_path[0].size() * merkle_path.size());
    }
}

std::ostream& operator<<(std::ostream &out, const set_membership_proof &proof)
{
    out << proof.address << "\n";
    out << proof.merkle_path.size() << "\n";
    for i in 0..proof.merkle_path.size()
    {
        ffec::output_bool_vector(out, proof.merkle_path[i]);
    }

    return out;
}

std::istream& operator>>(std::istream &in, set_membership_proof &proof)
{
    in >> proof.address;
    ffec::consume_newline(in);
    size_t tree_depth;
    in >> tree_depth;
    ffec::consume_newline(in);
    proof.merkle_path.resize(tree_depth);

    for i in 0..tree_depth
    {
        ffec::input_bool_vector(in, proof.merkle_path[i]);
    }

    return in;
}


/** @file
 *****************************************************************************

 Implementation of a Merkle tree based set commitment scheme.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SET_COMMITMENT_TCC_
// #define SET_COMMITMENT_TCC_



template<typename HashT>
set_commitment_accumulator<HashT>::set_commitment_accumulator(const size_t max_entries, const size_t value_size) :
    value_size(value_size)
{
    depth = ffec::log2(max_entries);
    digest_size = HashT::get_digest_len();

    tree.reset(new merkle_tree<HashT>(depth, digest_size));
}

template<typename HashT>
void set_commitment_accumulator<HashT>::add(const ffec::bit_vector &value)
{
    assert!(value_size == 0 || value.size() == value_size);
    const ffec::bit_vector hash = HashT::get_hash(value);
    if hash_to_pos.find(hash) == hash_to_pos.end()
    {
        const size_t pos = hash_to_pos.size();
        tree->set_value(pos, hash);
        hash_to_pos[hash] = pos;
    }
}

template<typename HashT>
bool set_commitment_accumulator<HashT>::is_in_set(const ffec::bit_vector &value) const
{
    assert!(value_size == 0 || value.size() == value_size);
    const ffec::bit_vector hash = HashT::get_hash(value);
    return (hash_to_pos.find(hash) != hash_to_pos.end());
}

template<typename HashT>
set_commitment set_commitment_accumulator<HashT>::get_commitment() const
{
    return tree->get_root();
}

template<typename HashT>
set_membership_proof set_commitment_accumulator<HashT>::get_membership_proof(const ffec::bit_vector &value) const
{
    const ffec::bit_vector hash = HashT::get_hash(value);
    auto it = hash_to_pos.find(hash);
    assert!(it != hash_to_pos.end());

    set_membership_proof proof;
    proof.address = it->second;
    proof.merkle_path = tree->get_path(it->second);

    return proof;
}



//#endif // SET_COMMITMENT_TCC_
