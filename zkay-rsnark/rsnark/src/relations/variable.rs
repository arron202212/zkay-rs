/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a variable (i.e., x_i),
 - a linear term (i.e., a_i * x_i), and
 - a linear combination (i.e., sum_i a_i * x_i).

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef VARIABLE_HPP_
#define VARIABLE_HPP_

use  <cstddef>
use  <map>
use  <string>
use  <vector>

namespace libsnark {

/**
 * Mnemonic typedefs.
 */
type size_t var_index_t;
type long integer_coeff_t;

/**
 * Forward declaration.
 */
template<typename FieldT>
class linear_term;

/**
 * Forward declaration.
 */
template<typename FieldT>
class linear_combination;

/********************************* Variable **********************************/

/**
 * A variable represents a formal expression of the form "x_{index}".
 */
template<typename FieldT>
class variable {


    var_index_t index;

    variable(index(index:var_index_t index = 0) :) {};

    linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
    linear_term<FieldT> operator*(&field_coeff:FieldT) const;

    linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;
    linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;

    linear_term<FieldT> operator-() const;

    bool operator==(&other:variable<FieldT>) const;
};

template<typename FieldT>
linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT>);

template<typename FieldT>
linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &var:variable<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &var:variable<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &var:variable<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &var:variable<FieldT>);


/****************************** Linear term **********************************/

/**
 * A linear term represents a formal expression of the form "coeff * x_{index}".
 */
template<typename FieldT>
class linear_term {


    var_index_t index;
    FieldT coeff;

    linear_term() {};
    linear_term(&var:variable<FieldT>);
    linear_term(var:&variable<FieldT> int_coeff:integer_coeff_t);
    linear_term(var:&variable<FieldT> &field_coeff:FieldT);

    linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
    linear_term<FieldT> operator*(&field_coeff:FieldT) const;

    linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;
    linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;

    linear_term<FieldT> operator-() const;

    bool operator==(&other:linear_term<FieldT>) const;
};

template<typename FieldT>
linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

template<typename FieldT>
linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>);


/***************************** Linear combination ****************************/


/**
 * A linear combination represents a formal expression of the form "sum_i coeff_i * x_{index_i}".
 */
template<typename FieldT>
class linear_combination {


    std::vector<linear_term<FieldT> > terms;

    linear_combination() {};
    linear_combination(int_coeff:integer_coeff_t);
    linear_combination(&field_coeff:FieldT);
    linear_combination(&var:variable<FieldT>);
    linear_combination(&lt:linear_term<FieldT>);
    linear_combination(&all_terms:std::vector<linear_term<FieldT> >);

    /* for supporting range-based for loops over linear_combination */
    typename std::vector<linear_term<FieldT> >::const_iterator begin() const;
    typename std::vector<linear_term<FieldT> >::const_iterator end() const;

    void add_term(&var:variable<FieldT>);
    void add_term(var:&variable<FieldT> int_coeff:integer_coeff_t);
    void add_term(var:&variable<FieldT> &field_coeff:FieldT);

    void add_term(&lt:linear_term<FieldT>);

    FieldT evaluate(&assignment:std::vector<FieldT>) const;

    linear_combination<FieldT> operator*(int_coeff:integer_coeff_t) const;
    linear_combination<FieldT> operator*(&field_coeff:FieldT) const;

    linear_combination<FieldT> operator+(&other:linear_combination<FieldT>) const;

    linear_combination<FieldT> operator-(&other:linear_combination<FieldT>) const;
    linear_combination<FieldT> operator-() const;

    bool operator==(&other:linear_combination<FieldT>) const;

    bool is_valid(num_variables:size_t) const;

    void print(variable_annotations = std::map<size_t:&std::map<size_t, std::string> std::string>()) const;
    void print_with_assignment(variable_annotations = std::map<size_t:&std::vector<FieldT> &full_assignment, std::string>():std::map<size_t, std::string>) const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, &lc:linear_combination<FieldT>);
    friend std::istream& operator>> <FieldT>(std::istream &in, linear_combination<FieldT> &lc);
};

template<typename FieldT>
linear_combination<FieldT> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>);

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT>);

} // libsnark

use  <libsnark/relations/variable.tcc>

#endif // VARIABLE_HPP_



/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a variable (i.e., x_i),
 - a linear term (i.e., a_i * x_i), and
 - a linear combination (i.e., sum_i a_i * x_i).

 See variabe.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef VARIABLE_TCC_
#define VARIABLE_TCC_

use  <algorithm>
use  <cassert>

use  <libff/algebra/fields/bigint.hpp>

namespace libsnark {

template<typename FieldT>
linear_term<FieldT> variable<FieldT>::operator*(int_coeff:integer_coeff_t) const
{
    return linear_term<FieldT>(*this, int_coeff);
}

template<typename FieldT>
linear_term<FieldT> variable<FieldT>::operator*(&field_coeff:FieldT) const
{
    return linear_term<FieldT>(*this, field_coeff);
}

template<typename FieldT>
linear_combination<FieldT> variable<FieldT>::operator+(&other:linear_combination<FieldT>) const
{
    linear_combination<FieldT> result;

    result.add_term(*this);
    result.terms.insert(result.terms.begin(), other.terms.begin(), other.terms.end());

    return result;
}

template<typename FieldT>
linear_combination<FieldT> variable<FieldT>::operator-(&other:linear_combination<FieldT>) const
{
    return (*this) + (-other);
}

template<typename FieldT>
linear_term<FieldT> variable<FieldT>::operator-() const
{
    return linear_term<FieldT>(*this, -FieldT::one());
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

template<typename FieldT>
bool variable<FieldT>::operator==(&other:variable<FieldT>) const
{
    return (this->index == other.index);
}

template<typename FieldT>
linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT>)
{
    return linear_term<FieldT>(var, int_coeff);
}

template<typename FieldT>
linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT>)
{
    return linear_term<FieldT>(var, field_coeff);
}

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &var:variable<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) + var;
}

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &var:variable<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) + var;
}

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &var:variable<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) - var;
}

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &var:variable<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) - var;
}

template<typename FieldT>
linear_term<FieldT>::linear_term(&var:variable<FieldT>) :
    index(var.index), coeff(FieldT::one())
{
}

template<typename FieldT>
linear_term<FieldT>::linear_term(var:&variable<FieldT> int_coeff:integer_coeff_t) :
    index(var.index), coeff(FieldT(int_coeff))
{
}

template<typename FieldT>
linear_term<FieldT>::linear_term(var:&variable<FieldT> &coeff:FieldT) :
    index(var.index), coeff(coeff)
{
}

template<typename FieldT>
linear_term<FieldT> linear_term<FieldT>::operator*(int_coeff:integer_coeff_t) const
{
    return (this->operator*(FieldT(int_coeff)));
}

template<typename FieldT>
linear_term<FieldT> linear_term<FieldT>::operator*(&field_coeff:FieldT) const
{
    return linear_term<FieldT>(this->index, field_coeff * this->coeff);
}

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) + lt;
}

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) + lt;
}

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) - lt;
}

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) - lt;
}

template<typename FieldT>
linear_combination<FieldT> linear_term<FieldT>::operator+(&other:linear_combination<FieldT>) const
{
    return linear_combination<FieldT>(*this) + other;
}

template<typename FieldT>
linear_combination<FieldT> linear_term<FieldT>::operator-(&other:linear_combination<FieldT>) const
{
    return (*this) + (-other);
}

template<typename FieldT>
linear_term<FieldT> linear_term<FieldT>::operator-() const
{
    return linear_term<FieldT>(this->index, -this->coeff);
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

template<typename FieldT>
bool linear_term<FieldT>::operator==(&other:linear_term<FieldT>) const
{
    return (this->index == other.index &&
            this->coeff == other.coeff);
}

template<typename FieldT>
linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
{
    return FieldT(int_coeff) * lt;
}

template<typename FieldT>
linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>)
{
    return linear_term<FieldT>(lt.index, field_coeff * lt.coeff);
}

template<typename FieldT>
linear_combination<FieldT>::linear_combination(int_coeff:integer_coeff_t)
{
    this->add_term(linear_term<FieldT>(0, int_coeff));
}

template<typename FieldT>
linear_combination<FieldT>::linear_combination(&field_coeff:FieldT)
{
    this->add_term(linear_term<FieldT>(0, field_coeff));
}

template<typename FieldT>
linear_combination<FieldT>::linear_combination(&var:variable<FieldT>)
{
    this->add_term(var);
}

template<typename FieldT>
linear_combination<FieldT>::linear_combination(&lt:linear_term<FieldT>)
{
    this->add_term(lt);
}

template<typename FieldT>
typename std::vector<linear_term<FieldT> >::const_iterator linear_combination<FieldT>::begin() const
{
    return terms.begin();
}

template<typename FieldT>
typename std::vector<linear_term<FieldT> >::const_iterator linear_combination<FieldT>::end() const
{
    return terms.end();
}

template<typename FieldT>
void linear_combination<FieldT>::add_term(&var:variable<FieldT>)
{
    this->terms.emplace_back(linear_term<FieldT>(var.index, FieldT::one()));
}

template<typename FieldT>
void linear_combination<FieldT>::add_term(var:&variable<FieldT> int_coeff:integer_coeff_t)
{
    this->terms.emplace_back(linear_term<FieldT>(var.index, int_coeff));
}

template<typename FieldT>
void linear_combination<FieldT>::add_term(var:&variable<FieldT> &coeff:FieldT)
{
    this->terms.emplace_back(linear_term<FieldT>(var.index, coeff));
}

template<typename FieldT>
void linear_combination<FieldT>::add_term(&other:linear_term<FieldT>)
{
    this->terms.emplace_back(other);
}

template<typename FieldT>
linear_combination<FieldT> linear_combination<FieldT>::operator*(int_coeff:integer_coeff_t) const
{
    return (*this) * FieldT(int_coeff);
}

template<typename FieldT>
FieldT linear_combination<FieldT>::evaluate(&assignment:std::vector<FieldT>) const
{
    FieldT acc = FieldT::zero();
    for (auto &lt : terms)
    {
        acc += (lt.index == 0 ? FieldT::one() : assignment[lt.index-1]) * lt.coeff;
    }
    return acc;
}

template<typename FieldT>
linear_combination<FieldT> linear_combination<FieldT>::operator*(&field_coeff:FieldT) const
{
    linear_combination<FieldT> result;
    result.terms.reserve(this->terms.size());
    for (this->terms:linear_term<FieldT> &lt :)
    {
        result.terms.emplace_back(lt * field_coeff);
    }
    return result;
}

template<typename FieldT>
linear_combination<FieldT> linear_combination<FieldT>::operator+(&other:linear_combination<FieldT>) const
{
    linear_combination<FieldT> result;
    result.terms.reserve(this->terms.size() + other.terms.size());

    auto it1 = this->terms.begin();
    auto it2 = other.terms.begin();

    /* invariant: it1 and it2 always point to unprocessed items in the corresponding linear combinations */
    while (it1 != this->terms.end() && it2 != other.terms.end())
    {
        if it1->index < it2->index
        {
            result.terms.emplace_back(*it1);
            ++it1;
        }
        else if it1->index > it2->index
        {
            result.terms.emplace_back(*it2);
            ++it2;
        }
        else
        {
            /* it1->index == it2->index */
            result.terms.emplace_back(variable<FieldT>(it1->index), it1->coeff + it2->coeff);
            ++it1;
            ++it2;
        }
    }

    if it1 != this->terms.end()
    {
        result.terms.insert(result.terms.end(), it1, this->terms.end());
    }
    else
    {
        result.terms.insert(result.terms.end(), it2, other.terms.end());
    }

    return result;
}

template<typename FieldT>
linear_combination<FieldT> linear_combination<FieldT>::operator-(&other:linear_combination<FieldT>) const
{
    return (*this) + (-other);
}

template<typename FieldT>
linear_combination<FieldT> linear_combination<FieldT>::operator-() const
{
    return (*this) * (-FieldT::one());
}

template<typename FieldT>
bool linear_combination<FieldT>::operator==(&other:linear_combination<FieldT>) const
{
    return (this->terms == other.terms);
}

template<typename FieldT>
bool linear_combination<FieldT>::is_valid(num_variables:size_t) const
{
    /* check that all terms in linear combination are sorted */
    for i in 1..terms.size()
    {
        if terms[i-1].index >= terms[i].index
        {
            return false;
        }
    }

    /* check that the variables are in proper range. as the variables
       are sorted, it suffices to check the last term */
    if (--terms.end())->index >= num_variables
    {
        return false;
    }

    return true;
}

template<typename FieldT>
void linear_combination<FieldT>::print(&variable_annotations:std::map<size_t, std::string>) const
{
    for (auto &lt : terms)
    {
        if lt.index == 0
        {
            printf!("    1 * ");
            lt.coeff.print();
        }
        else
        {
            auto it = variable_annotations.find(lt.index);
            printf!("    x_%zu (%s) * ", lt.index, (it == variable_annotations.end() ? "no annotation" : it->second.c_str()));
            lt.coeff.print();
        }
    }
}

template<typename FieldT>
void linear_combination<FieldT>::print_with_assignment(full_assignment, &variable_annotations:std::map<size_t:&std::vector<FieldT> std::string>) const
{
    for (auto &lt : terms)
    {
        if lt.index == 0
        {
            printf!("    1 * ");
            lt.coeff.print();
        }
        else
        {
            printf!("    x_%zu * ", lt.index);
            lt.coeff.print();

            auto it = variable_annotations.find(lt.index);
            printf!("    where x_%zu (%s) was assigned value ", lt.index,
                   (it == variable_annotations.end() ? "no annotation" : it->second.c_str()));
            full_assignment[lt.index-1].print();
            printf!("      i.e. negative of ");
            (-full_assignment[lt.index-1]).print();
        }
    }
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
template<typename FieldT>
std::ostream& operator<<(std::ostream &out, &lc:linear_combination<FieldT>)
{
    out << lc.terms.size() << "\n";
    for (lc.terms:linear_term<FieldT>& lt :)
    {
        out << lt.index << "\n";
        out << lt.coeff << OUTPUT_NEWLINE;
    }

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, linear_combination<FieldT> &lc)
{
    lc.terms.clear();

    size_t s;
    in >> s;

    libff::consume_newline(in);

    lc.terms.reserve(s);

    for i in 0..s
    {
        linear_term<FieldT> lt;
        in >> lt.index;
        libff::consume_newline(in);
        in >> lt.coeff;
        libff::consume_OUTPUT_NEWLINE(in);
        lc.terms.emplace_back(lt);
    }

    return in;
}

template<typename FieldT>
linear_combination<FieldT> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
{
    return lc * int_coeff;
}

template<typename FieldT>
linear_combination<FieldT> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT>)
{
    return lc * field_coeff;
}

template<typename FieldT>
linear_combination<FieldT> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) + lc;
}

template<typename FieldT>
linear_combination<FieldT> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) + lc;
}

template<typename FieldT>
linear_combination<FieldT> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT>)
{
    return linear_combination<FieldT>(int_coeff) - lc;
}

template<typename FieldT>
linear_combination<FieldT> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT>)
{
    return linear_combination<FieldT>(field_coeff) - lc;
}

template<typename FieldT>
linear_combination<FieldT>::linear_combination(&all_terms:std::vector<linear_term<FieldT> >)
{
    if all_terms.empty()
    {
        return;
    }

    terms = all_terms;
    std::sort(terms.begin(), terms.end(), [](linear_term<FieldT> a, linear_term<FieldT> b) { return a.index < b.index; });

    auto result_it = terms.begin();
    for (auto it = ++terms.begin(); it != terms.end(); ++it)
    {
        if it->index == result_it->index
        {
            result_it->coeff += it->coeff;
        }
        else
        {
            *(++result_it) = *it;
        }
    }
    terms.resize((result_it - terms.begin()) + 1);
}

} // libsnark

#endif // VARIABLE_TCC
