/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef PB_VARIABLE_HPP_
#define PB_VARIABLE_HPP_

use  <cstddef>
use  <string>
use  <vector>

use  <libff/common/utils.hpp>

use  <libsnark/relations/variable.hpp>

namespace libsnark {

type size_t lc_index_t;

template<typename FieldT>
class protoboard;

template<typename FieldT>
class pb_variable : public variable<FieldT> {
public:
    pb_variable(const var_index_t index = 0) : variable<FieldT>(index) {};

    void allocate(protoboard<FieldT> &pb, const std::string &annotation="");
};

template<typename FieldT>
class pb_variable_array : private std::vector<pb_variable<FieldT> >
{
    type std::vector<pb_variable<FieldT> > contents;
public:
    using typename contents::iterator;
    using typename contents::const_iterator;
    using typename contents::reverse_iterator;
    using typename contents::const_reverse_iterator;

    using contents::begin;
    using contents::end;
    using contents::rbegin;
    using contents::rend;
    using contents::emplace_back;
    using contents::insert;
    using contents::reserve;
    using contents::size;
    using contents::empty;
    using contents::operator[];
    using contents::resize;

    pb_variable_array() : contents() {};
    pb_variable_array(size_t count, const pb_variable<FieldT> &value) : contents(count, value) {};
    pb_variable_array(typename contents::const_iterator first, typename contents::const_iterator last) : contents(first, last) {};
    pb_variable_array(typename contents::const_reverse_iterator first, typename contents::const_reverse_iterator last) : contents(first, last) {};
    void allocate(protoboard<FieldT> &pb, const size_t n, const std::string &annotation_prefix="");

    void fill_with_field_elements(protoboard<FieldT> &pb, const std::vector<FieldT>& vals) const;
    void fill_with_bits(protoboard<FieldT> &pb, const libff::bit_vector& bits) const;
    void fill_with_bits_of_ulong(protoboard<FieldT> &pb, const unsigned long i) const;
    void fill_with_bits_of_field_element(protoboard<FieldT> &pb, const FieldT &r) const;

    std::vector<FieldT> get_vals(const protoboard<FieldT> &pb) const;
    libff::bit_vector get_bits(const protoboard<FieldT> &pb) const;

    FieldT get_field_element_from_bits(const protoboard<FieldT> &pb) const;
};

/* index 0 corresponds to the constant term (used in legacy code) */
#define ONE pb_variable<FieldT>(0)

template<typename FieldT>
class pb_linear_combination : public linear_combination<FieldT> {
public:
    bool is_variable;
    lc_index_t index;

    pb_linear_combination();
    pb_linear_combination(const pb_variable<FieldT> &var);

    void assign(protoboard<FieldT> &pb, const linear_combination<FieldT> &lc);
    void evaluate(protoboard<FieldT> &pb) const;

    bool is_constant() const;
    FieldT constant_term() const;
};

template<typename FieldT>
class pb_linear_combination_array : private std::vector<pb_linear_combination<FieldT> >
{
    type std::vector<pb_linear_combination<FieldT> > contents;
public:
    using typename contents::iterator;
    using typename contents::const_iterator;
    using typename contents::reverse_iterator;
    using typename contents::const_reverse_iterator;

    using contents::begin;
    using contents::end;
    using contents::rbegin;
    using contents::rend;
    using contents::emplace_back;
    using contents::insert;
    using contents::reserve;
    using contents::size;
    using contents::empty;
    using contents::operator[];
    using contents::resize;

    pb_linear_combination_array() : contents() {};
    pb_linear_combination_array(const pb_variable_array<FieldT> &arr) { for (auto &v : arr) this->emplace_back(pb_linear_combination<FieldT>(v)); };
    pb_linear_combination_array(size_t count) : contents(count) {};
    pb_linear_combination_array(size_t count, const pb_linear_combination<FieldT> &value) : contents(count, value) {};
    pb_linear_combination_array(typename contents::const_iterator first, typename contents::const_iterator last) : contents(first, last) {};
    pb_linear_combination_array(typename contents::const_reverse_iterator first, typename contents::const_reverse_iterator last) : contents(first, last) {};

    void evaluate(protoboard<FieldT> &pb) const;

    void fill_with_field_elements(protoboard<FieldT> &pb, const std::vector<FieldT>& vals) const;
    void fill_with_bits(protoboard<FieldT> &pb, const libff::bit_vector& bits) const;
    void fill_with_bits_of_ulong(protoboard<FieldT> &pb, const unsigned long i) const;
    void fill_with_bits_of_field_element(protoboard<FieldT> &pb, const FieldT &r) const;

    std::vector<FieldT> get_vals(const protoboard<FieldT> &pb) const;
    libff::bit_vector get_bits(const protoboard<FieldT> &pb) const;

    FieldT get_field_element_from_bits(const protoboard<FieldT> &pb) const;
};

template<typename FieldT>
linear_combination<FieldT> pb_sum(const pb_linear_combination_array<FieldT> &v);

template<typename FieldT>
linear_combination<FieldT> pb_packing_sum(const pb_linear_combination_array<FieldT> &v);

template<typename FieldT>
linear_combination<FieldT> pb_coeff_sum(const pb_linear_combination_array<FieldT> &v, const std::vector<FieldT> &coeffs);

} // libsnark
use  <libsnark/gadgetlib1/pb_variable.tcc>

#endif // PB_VARIABLE_HPP_
/** @file
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef PB_VARIABLE_TCC_
#define PB_VARIABLE_TCC_
use  <cassert>

use  <libff/common/utils.hpp>

use  <libsnark/gadgetlib1/protoboard.hpp>

namespace libsnark {

template<typename FieldT>
void pb_variable<FieldT>::allocate(protoboard<FieldT> &pb, const std::string &annotation)
{
    this->index = pb.allocate_var_index(annotation);
}

/* allocates pb_variable<FieldT> array in MSB->LSB order */
template<typename FieldT>
void pb_variable_array<FieldT>::allocate(protoboard<FieldT> &pb, const size_t n, const std::string &annotation_prefix)
{
#ifdef DEBUG
    assert(annotation_prefix != "");
#endif
    (*this).resize(n);

    for (size_t i = 0; i < n; ++i)
    {
        (*this)[i].allocate(pb, FMT(annotation_prefix, "_%zu", i));
    }
}

template<typename FieldT>
void pb_variable_array<FieldT>::fill_with_field_elements(protoboard<FieldT> &pb, const std::vector<FieldT>& vals) const
{
    assert(this->size() == vals.size());
    for (size_t i = 0; i < vals.size(); ++i)
    {
        pb.val((*this)[i]) = vals[i];
    }
}

template<typename FieldT>
void pb_variable_array<FieldT>::fill_with_bits(protoboard<FieldT> &pb, const libff::bit_vector& bits) const
{
    assert(this->size() == bits.size());
    for (size_t i = 0; i < bits.size(); ++i)
    {
        pb.val((*this)[i]) = (bits[i] ? FieldT::one() : FieldT::zero());
    }
}

template<typename FieldT>
void pb_variable_array<FieldT>::fill_with_bits_of_field_element(protoboard<FieldT> &pb, const FieldT &r) const
{
    const libff::bigint<FieldT::num_limbs> rint = r.as_bigint();
    for (size_t i = 0; i < this->size(); ++i)
    {
        pb.val((*this)[i]) = rint.test_bit(i) ? FieldT::one() : FieldT::zero();
    }
}

template<typename FieldT>
void pb_variable_array<FieldT>::fill_with_bits_of_ulong(protoboard<FieldT> &pb, const unsigned long i) const
{
    this->fill_with_bits_of_field_element(pb, FieldT(i, true));
}

template<typename FieldT>
std::vector<FieldT> pb_variable_array<FieldT>::get_vals(const protoboard<FieldT> &pb) const
{
    std::vector<FieldT> result(this->size());
    for (size_t i = 0; i < this->size(); ++i)
    {
        result[i] = pb.val((*this)[i]);
    }
    return result;
}

template<typename FieldT>
libff::bit_vector pb_variable_array<FieldT>::get_bits(const protoboard<FieldT> &pb) const
{
    libff::bit_vector result;
    for (size_t i = 0; i < this->size(); ++i)
    {
        const FieldT v = pb.val((*this)[i]);
        assert(v == FieldT::zero() || v == FieldT::one());
        result.push_back(v == FieldT::one());
    }
    return result;
}

template<typename FieldT>
FieldT pb_variable_array<FieldT>::get_field_element_from_bits(const protoboard<FieldT> &pb) const
{
    FieldT result = FieldT::zero();

    for (size_t i = 0; i < this->size(); ++i)
    {
        /* push in the new bit */
        const FieldT v = pb.val((*this)[this->size()-1-i]);
        assert(v == FieldT::zero() || v == FieldT::one());
        result += result + v;
    }

    return result;
}

template<typename FieldT>
pb_linear_combination<FieldT>::pb_linear_combination()
{
    this->is_variable = false;
}

template<typename FieldT>
pb_linear_combination<FieldT>::pb_linear_combination(const pb_variable<FieldT> &var)
{
    this->is_variable = true;
    this->index = var.index;
    this->terms.emplace_back(linear_term<FieldT>(var));
}

template<typename FieldT>
void pb_linear_combination<FieldT>::assign(protoboard<FieldT> &pb, const linear_combination<FieldT> &lc)
{
    assert(this->is_variable == false);
    this->index = pb.allocate_lc_index();
    this->terms = lc.terms;
}

template<typename FieldT>
void pb_linear_combination<FieldT>::evaluate(protoboard<FieldT> &pb) const
{
    if (this->is_variable)
    {
        return; // do nothing
    }

    FieldT sum = 0;
    for (auto term : this->terms)
    {
        sum += term.coeff * pb.val(pb_variable<FieldT>(term.index));
    }

    pb.lc_val(*this) = sum;
}

template<typename FieldT>
bool pb_linear_combination<FieldT>::is_constant() const
{
    if (is_variable)
    {
        return (index == 0);
    }
    else
    {
        for (auto term : this->terms)
        {
            if (term.index != 0)
            {
                return false;
            }
        }

        return true;
    }
}

template<typename FieldT>
FieldT pb_linear_combination<FieldT>::constant_term() const
{
    if (is_variable)
    {
        return (index == 0 ? FieldT::one() : FieldT::zero());
    }
    else
    {
        FieldT result = FieldT::zero();
        for (auto term : this->terms)
        {
            if (term.index == 0)
            {
                result += term.coeff;
            }
        }
        return result;
    }
}

template<typename FieldT>
void pb_linear_combination_array<FieldT>::evaluate(protoboard<FieldT> &pb) const
{
    for (size_t i = 0; i < this->size(); ++i)
    {
        (*this)[i].evaluate(pb);
    }
}

template<typename FieldT>
void pb_linear_combination_array<FieldT>::fill_with_field_elements(protoboard<FieldT> &pb, const std::vector<FieldT>& vals) const
{
    assert(this->size() == vals.size());
    for (size_t i = 0; i < vals.size(); ++i)
    {
        pb.lc_val((*this)[i]) = vals[i];
    }
}

template<typename FieldT>
void pb_linear_combination_array<FieldT>::fill_with_bits(protoboard<FieldT> &pb, const libff::bit_vector& bits) const
{
    assert(this->size() == bits.size());
    for (size_t i = 0; i < bits.size(); ++i)
    {
        pb.lc_val((*this)[i]) = (bits[i] ? FieldT::one() : FieldT::zero());
    }
}

template<typename FieldT>
void pb_linear_combination_array<FieldT>::fill_with_bits_of_field_element(protoboard<FieldT> &pb, const FieldT &r) const
{
    const libff::bigint<FieldT::num_limbs> rint = r.as_bigint();
    for (size_t i = 0; i < this->size(); ++i)
    {
        pb.lc_val((*this)[i]) = rint.test_bit(i) ? FieldT::one() : FieldT::zero();
    }
}

template<typename FieldT>
void pb_linear_combination_array<FieldT>::fill_with_bits_of_ulong(protoboard<FieldT> &pb, const unsigned long i) const
{
    this->fill_with_bits_of_field_element(pb, FieldT(i));
}

template<typename FieldT>
std::vector<FieldT> pb_linear_combination_array<FieldT>::get_vals(const protoboard<FieldT> &pb) const
{
    std::vector<FieldT> result(this->size());
    for (size_t i = 0; i < this->size(); ++i)
    {
        result[i] = pb.lc_val((*this)[i]);
    }
    return result;
}

template<typename FieldT>
libff::bit_vector pb_linear_combination_array<FieldT>::get_bits(const protoboard<FieldT> &pb) const
{
    libff::bit_vector result;
    for (size_t i = 0; i < this->size(); ++i)
    {
        const FieldT v = pb.lc_val((*this)[i]);
        assert(v == FieldT::zero() || v == FieldT::one());
        result.push_back(v == FieldT::one());
    }
    return result;
}

template<typename FieldT>
FieldT pb_linear_combination_array<FieldT>::get_field_element_from_bits(const protoboard<FieldT> &pb) const
{
    FieldT result = FieldT::zero();

    for (size_t i = 0; i < this->size(); ++i)
    {
        /* push in the new bit */
        const FieldT v = pb.lc_val((*this)[this->size()-1-i]);
        assert(v == FieldT::zero() || v == FieldT::one());
        result += result + v;
    }

    return result;
}

template<typename FieldT>
linear_combination<FieldT> pb_sum(const pb_linear_combination_array<FieldT> &v)
{
    linear_combination<FieldT> result;
    for (auto &term  : v)
    {
        result = result + term;
    }

    return result;
}

template<typename FieldT>
linear_combination<FieldT> pb_packing_sum(const pb_linear_combination_array<FieldT> &v)
{
    FieldT twoi = FieldT::one(); // will hold 2^i entering each iteration
    std::vector<linear_term<FieldT> > all_terms;
    for (auto &lc : v)
    {
        for (auto &term : lc.terms)
        {
            all_terms.emplace_back(twoi * term);
        }
        twoi += twoi;
    }

    return linear_combination<FieldT>(all_terms);
}

template<typename FieldT>
linear_combination<FieldT> pb_coeff_sum(const pb_linear_combination_array<FieldT> &v, const std::vector<FieldT> &coeffs)
{
    assert(v.size() == coeffs.size());
    std::vector<linear_term<FieldT> > all_terms;

    auto coeff_it = coeffs.begin();
    for (auto &lc : v)
    {
        for (auto &term : lc.terms)
        {
            all_terms.emplace_back((*coeff_it) * term);
        }
        ++coeff_it;
    }

    return linear_combination<FieldT>(all_terms);
}


} // libsnark
#endif // PB_VARIABLE_TCC
