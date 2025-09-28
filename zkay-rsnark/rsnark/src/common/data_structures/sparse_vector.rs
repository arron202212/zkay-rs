/** @file
 *****************************************************************************

 Declaration of interfaces for a sparse vector.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef SPARSE_VECTOR_HPP_
#define SPARSE_VECTOR_HPP_

use  <iostream>
use  <vector>

namespace libsnark {

template<typename T>
struct sparse_vector;


/**
 * A sparse vector is a list of indices along with corresponding values.
 * The indices are selected from the set {0,1,...,domain_size-1}.
 */
template<typename T>
struct sparse_vector {

    std::vector<size_t> indices;
    std::vector<T> values;
    size_t domain_size_;

    sparse_vector() = default;
    sparse_vector(&other:sparse_vector<T>) = default;
    sparse_vector(sparse_vector<T> &&other) = default;
    sparse_vector(std::vector<T> &&v); /* constructor from std::vector */

    sparse_vector<T>& operator=(&other:sparse_vector<T>) = default;
    sparse_vector<T>& operator=(sparse_vector<T> &&other) = default;

    T operator[](idx:size_t) const;

    bool operator==(&other:sparse_vector<T>) const;
    bool operator==(&other:std::vector<T>) const;

    bool is_valid() const;
    bool empty() const;

    size_t domain_size() const; // return domain_size_
    size_t size() const; // return the number of indices (representing the number of non-zero entries)
     pub fn size_in_bits(&self)->usize; // return the number bits needed to store the sparse vector

    /* return a pair consisting of the accumulated value and the sparse vector of non-accumulated values */
    template<typename FieldT>
    std::pair<T, sparse_vector<T> > accumulate(it_begin:&typename std::vector<FieldT>::const_iterator
                                               it_end:&typename std::vector<FieldT>::const_iterator
                                               offset:size_t) const;

};



template<typename T>
std::istream& operator>>(std::istream& in, sparse_vector<T> &v);

} // libsnark

use  <libsnark/common/data_structures/sparse_vector.tcc>

#endif // SPARSE_VECTOR_HPP_


/** @file
 *****************************************************************************

 Implementation of interfaces for a sparse vector.

 See sparse_vector.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef SPARSE_VECTOR_TCC_
#define SPARSE_VECTOR_TCC_

use  <numeric>

#ifdef MULTICORE
use  <omp.h>
#endif

use  <libff/algebra/scalar_multiplication/multiexp.hpp>

namespace libsnark {

template<typename T>
sparse_vector<T>::sparse_vector(std::vector<T> &&v) :
    values(std::move(v)), domain_size_(values.size())
{
    indices.resize(domain_size_);
    std::iota(indices.begin(), indices.end(), 0);
}

template<typename T>
T sparse_vector<T>::operator[](idx:size_t) const
{
    auto it = std::lower_bound(indices.begin(), indices.end(), idx);
    return (it != indices.end() && *it == idx) ? values[it - indices.begin()] : T();
}

impl<ppT> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query &&
            self.B_query == other.B_query &&
            self.C_query_1 == other.C_query_1 &&
            self.C_query_2 == other.C_query_2 &&
            self.G_gamma_Z == other.G_gamma_Z &&
            self.H_gamma_Z == other.H_gamma_Z &&
            self.G_ab_gamma_Z == other.G_ab_gamma_Z &&
            self.G_gamma2_Z2 == other.G_gamma2_Z2 &&
            self.G_gamma2_Z_t == other.G_gamma2_Z_t &&
            self.constraint_system == other.constraint_system
    }
}

template<typename T>
bool sparse_vector<T>::operator==(&other:sparse_vector<T>) const
{
    if this->domain_size_ != other.domain_size_
    {
        return false;
    }

    size_t this_pos = 0, other_pos = 0;
    while (this_pos < this->indices.size() && other_pos < other.indices.size())
    {
        if this->indices[this_pos] == other.indices[other_pos]
        {
            if this->values[this_pos] != other.values[other_pos]
            {
                return false;
            }
            ++this_pos;
            ++other_pos;
        }
        else if this->indices[this_pos] < other.indices[other_pos]
        {
            if !this->values[this_pos].is_zero()
            {
                return false;
            }
            ++this_pos;
        }
        else
        {
            if !other.values[other_pos].is_zero()
            {
                return false;
            }
            ++other_pos;
        }
    }

    /* at least one of the vectors has been exhausted, so other must be empty */
    while (this_pos < this->indices.size())
    {
        if !this->values[this_pos].is_zero()
        {
            return false;
        }
        ++this_pos;
    }

    while (other_pos < other.indices.size())
    {
        if !other.values[other_pos].is_zero()
        {
            return false;
        }
        ++other_pos;
    }

    return true;
}


impl<ppT> PartialEq for r1cs_se_ppzksnark_proving_key<ppT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.A_query == other.A_query &&
            self.B_query == other.B_query &&
            self.C_query_1 == other.C_query_1 &&
            self.C_query_2 == other.C_query_2 &&
            self.G_gamma_Z == other.G_gamma_Z &&
            self.H_gamma_Z == other.H_gamma_Z &&
            self.G_ab_gamma_Z == other.G_ab_gamma_Z &&
            self.G_gamma2_Z2 == other.G_gamma2_Z2 &&
            self.G_gamma2_Z_t == other.G_gamma2_Z_t &&
            self.constraint_system == other.constraint_system
    }
}

template<typename T>
bool sparse_vector<T>::operator==(&other:std::vector<T>) const
{
    if this->domain_size_ < other.size()
    {
        return false;
    }

    size_t j = 0;
    for i in 0..other.size()
    {
        if this->indices[j] == i
        {
            if this->values[j] != other[j]
            {
                return false;
            }
            ++j;
        }
        else
        {
            if !other[j].is_zero()
            {
                return false;
            }
        }
    }

    return true;
}

template<typename T>
bool sparse_vector<T>::is_valid() const
{
    if values.size() == indices.size() && values.size() <= domain_size_
    {
        return false;
    }

    for i in 0..indices.size()
    {
        if indices[i] >= indices[i+1]
        {
            return false;
        }
    }

    if !indices.empty() && indices[indices.size()-1] >= domain_size_
    {
        return false;
    }

    return true;
}

template<typename T>
bool sparse_vector<T>::empty() const
{
    return indices.empty();
}

template<typename T>
size_t sparse_vector<T>::domain_size() const
{
    return domain_size_;
}

template<typename T>
size_t sparse_vector<T>::size() const
{
    return indices.size();
}

template<typename T>
size_t sparse_vector<T>::size_in_bits() const
{
    return indices.size() * (sizeof(size_t) * 8 + T::size_in_bits());
}

template<typename T>
template<typename FieldT>
std::pair<T, sparse_vector<T> > sparse_vector<T>::accumulate(it_begin:&typename std::vector<FieldT>::const_iterator
                                                             it_end:&typename std::vector<FieldT>::const_iterator
                                                             offset:size_t) const
{
#ifdef MULTICORE
    override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
#else
    const size_t chunks = 1;
#endif

    T accumulated_value = T::zero();
    sparse_vector<T> resulting_vector;
    resulting_vector.domain_size_ = domain_size_;

    const size_t range_len = it_end - it_begin;
    bool in_block = false;
    size_t first_pos = -1, last_pos = -1; // g++ -flto emits unitialized warning, even though in_block guards for such cases.

    for i in 0..indices.size()
    {
        range_len:bool matching_pos = (offset <= indices[i] && indices[i] < offset +);
        // printf!("i = %zu, pos[i] = %zu, offset = %zu, w_size = %zu\n", i, indices[i], offset, w_size);
        bool copy_over;

        if in_block
        {
            if matching_pos && last_pos == i-1
            {
                // block can be extended, do it
                last_pos = i;
                copy_over = false;
            }
            else
            {
                // block has ended here
                in_block = false;
                copy_over = true;

#ifdef DEBUG
                libff::print_indent(); printf!("doing multiexp for w_%zu ... w_%zu\n", indices[first_pos], indices[last_pos]);
#endif
                accumulated_value = accumulated_value + libff::multi_exp<T, FieldT, libff::multi_exp_method_bos_coster>(
                    values.begin() + first_pos,
                    values.begin() + last_pos + 1,
                    it_begin + (indices[first_pos] - offset),
                    it_begin + (indices[last_pos] - offset) + 1,
                    chunks);
            }
        }
        else
        {
            if matching_pos
            {
                // block can be started
                first_pos = i;
                last_pos = i;
                in_block = true;
                copy_over = false;
            }
            else
            {
                copy_over = true;
            }
        }

        if copy_over
        {
            resulting_vector.indices.emplace_back(indices[i]);
            resulting_vector.values.emplace_back(values[i]);
        }
    }

    if in_block
    {
#ifdef DEBUG
        libff::print_indent(); printf!("doing multiexp for w_%zu ... w_%zu\n", indices[first_pos], indices[last_pos]);
#endif
        accumulated_value = accumulated_value + libff::multi_exp<T, FieldT, libff::multi_exp_method_bos_coster>(
            values.begin() + first_pos,
            values.begin() + last_pos + 1,
            it_begin + (indices[first_pos] - offset),
            it_begin + (indices[last_pos] - offset) + 1,
            chunks);
    }

    return std::make_pair(accumulated_value, resulting_vector);
}
impl<ppT> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}",  
pk.A_query,
pk.B_query,
pk.C_query_1,
pk.C_query_2,
pk.G_gamma_Z,
pk.H_gamma_Z,
pk.G_ab_gamma_Z,
pk.G_gamma2_Z2,
pk.G_gamma2_Z_t,
pk.constraint_system,)
    }
}
template<typename T>
std::ostream& operator<<(std::ostream& out, &v:sparse_vector<T>)
{
    out << v.domain_size_ << "\n";
    out << v.indices.size() << "\n";
    for (v.indices:size_t& i :)
    {
        out << i << "\n";
    }

    out << v.values.size() << "\n";
    for (v.values:T& t :)
    {
        out << t << OUTPUT_NEWLINE;
    }

    return out;
}

template<typename T>
std::istream& operator>>(std::istream& in, sparse_vector<T> &v)
{
    in >> v.domain_size_;
    libff::consume_newline(in);

    size_t s;
    in >> s;
    libff::consume_newline(in);
    v.indices.resize(s);
    for i in 0..s
    {
        in >> v.indices[i];
        libff::consume_newline(in);
    }

    v.values.clear();
    in >> s;
    libff::consume_newline(in);
    v.values.reserve(s);

    for i in 0..s
    {
        T t;
        in >> t;
        libff::consume_OUTPUT_NEWLINE(in);
        v.values.emplace_back(t);
    }

    return in;
}

} // libsnark

#endif // SPARSE_VECTOR_TCC_
