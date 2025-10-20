/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a USCS constraint,
 - a USCS variable assignment, and
 - a USCS constraint system.

 Above, USCS stands for "Unitary-Square Constraint System".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_HPP_
// #define USCS_HPP_

use  <cstdlib>
use  <iostream>
use  <map>
use  <string>
use  <vector>

use crate::relations::variable;



/************************* USCS constraint ***********************************/

/**
 * A USCS constraint is a formal expression of the form
 *
 *                \sum_{i=1}^{m} a_i * x_{i} ,
 *
 * where each a_i is in <FieldT> and each x_{i} is a formal variable.
 *
 * A USCS constraint is used to construct a USCS constraint system (see below).
 */
template<typename FieldT>
using uscs_constraint = linear_combination<FieldT>;


/************************* USCS variable assignment **************************/

/**
 * A USCS variable assignment is a vector of <FieldT> elements that represents
 * a candidate solution to a USCS constraint system (see below).
 */
template<typename FieldT>
using uscs_primary_input = std::vector<FieldT>;

template<typename FieldT>
using uscs_auxiliary_input = std::vector<FieldT>;

template<typename FieldT>
using uscs_variable_assignment = std::vector<FieldT>;



/************************* USCS constraint system ****************************/

template<typename FieldT>
class uscs_constraint_system;

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const uscs_constraint_system<FieldT> &cs);

template<typename FieldT>
std::istream& operator>>(std::istream &in, uscs_constraint_system<FieldT> &cs);

/**
 * A system of USCS constraints looks like
 *
 *     { ( \sum_{i=1}^{m_k} a_{k,i} * x_{k,i} )^2 = 1 }_{k=1}^{n}  .
 *
 * In other words, the system is satisfied if and only if there exist a
 * USCS variable assignment for which each USCS constraint evaluates to -1 or 1.
 *
 * NOTE:
 * The 0-th variable (i.e., "x_{0}") always represents the constant 1.
 * Thus, the 0-th variable is not included in num_variables.
 */
template<typename FieldT>
class uscs_constraint_system {
public:
    size_t primary_input_size;
    size_t auxiliary_input_size;

    std::vector<uscs_constraint<FieldT> > constraints;

    uscs_constraint_system() : primary_input_size(0), auxiliary_input_size(0) {};

    size_t num_inputs() const;
    size_t num_variables() const;
    size_t num_constraints() const;

// #ifdef DEBUG
    std::map<size_t, std::string> constraint_annotations;
    std::map<size_t, std::string> variable_annotations;
//#endif

    bool is_valid() const;
    bool is_satisfied(const uscs_primary_input<FieldT> &primary_input,
                      const uscs_auxiliary_input<FieldT> &auxiliary_input) const;

    void add_constraint(const uscs_constraint<FieldT> &constraint);
    void add_constraint(const uscs_constraint<FieldT> &constraint, const std::string &annotation);

    bool operator==(const uscs_constraint_system<FieldT> &other) const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, const uscs_constraint_system<FieldT> &cs);
    friend std::istream& operator>> <FieldT>(std::istream &in, uscs_constraint_system<FieldT> &cs);

    void report_linear_constraint_statistics() const;
};




use crate::relations::constraint_satisfaction_problems/uscs/uscs;

//#endif // USCS_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for:
 - a USCS constraint,
 - a USCS variable assignment, and
 - a USCS constraint system.

 See uscs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_TCC_
// #define USCS_TCC_

use  <algorithm>
use  <cassert>
use  <set>

use ffec::algebra::fields::bigint;
use ffec::common::profiling;
use ffec::common::utils;



template<typename FieldT>
size_t uscs_constraint_system<FieldT>::num_inputs() const
{
    return primary_input_size;
}

template<typename FieldT>
size_t uscs_constraint_system<FieldT>::num_variables() const
{
    return primary_input_size + auxiliary_input_size;
}


template<typename FieldT>
size_t uscs_constraint_system<FieldT>::num_constraints() const
{
    return constraints.size();
}

template<typename FieldT>
bool uscs_constraint_system<FieldT>::is_valid() const
{
    if self.num_inputs() > self.num_variables() return false;

    for c in 0..constraints.size()
    {
        if !valid_vector(constraints[c], self.num_variables())
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
void dump_uscs_constraint(const uscs_constraint<FieldT> &constraint,
                          const uscs_variable_assignment<FieldT> &full_variable_assignment,
                          const std::map<size_t, std::string> &variable_annotations)
{
    print!("terms:\n");
    constraint.print_with_assignment(full_variable_assignment, variable_annotations);
}

template<typename FieldT>
bool uscs_constraint_system<FieldT>::is_satisfied(const uscs_primary_input<FieldT> &primary_input,
                                                  const uscs_auxiliary_input<FieldT> &auxiliary_input) const
{
    assert!(primary_input.size() == num_inputs());
    assert!(primary_input.size() + auxiliary_input.size() == num_variables());

    uscs_variable_assignment<FieldT> full_variable_assignment = primary_input;
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());

    for c in 0..constraints.size()
    {
        FieldT res = constraints[c].evaluate(full_variable_assignment);
        if !(res.squared() == FieldT::one())
        {
// #ifdef DEBUG
            auto it = constraint_annotations.find(c);
            print!("constraint {} (%s) unsatisfied\n", c, ( if let Some(it) =constraint_annotations.find(c) {it.1.to_string()} else {"no annotation" }));
            print!("<a,(1,x)> = "); res.print();
            print!("constraint was:\n");
            dump_uscs_constraint(constraints[c], full_variable_assignment, variable_annotations);
//#endif // DEBUG
            return false;
        }
    }

    return true;
}

template<typename FieldT>
void uscs_constraint_system<FieldT>::add_constraint(const uscs_constraint<FieldT> &c)
{
    constraints.push(c);
}

template<typename FieldT>
void uscs_constraint_system<FieldT>::add_constraint(const uscs_constraint<FieldT> &c, const std::string &annotation)
{
// #ifdef DEBUG
    constraint_annotations[constraints.size()] = annotation;
#else
    ffec::UNUSED(annotation);
//#endif
    constraints.push(c);
}

template<typename FieldT>
bool uscs_constraint_system<FieldT>::operator==(const uscs_constraint_system<FieldT> &other) const
{
    return (self.constraints == other.constraints &&
            self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size);
}

template<typename FieldT>
std::ostream& operator<<(std::ostream &out, const uscs_constraint_system<FieldT> &cs)
{
    out << cs.primary_input_size << "\n";
    out << cs.auxiliary_input_size << "\n";

    out << cs.num_constraints() << "\n";
    for c in &cs.constraints
    {
        out << c;
    }

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, uscs_constraint_system<FieldT> &cs)
{
    in >> cs.primary_input_size;
    in >> cs.auxiliary_input_size;

    cs.constraints.clear();

    size_t s;
    in >> s;

    char b;
    in.read(&b, 1);

    cs.constraints.reserve(s);

    for i in 0..s
    {
        uscs_constraint<FieldT> c;
        in >> c;
        cs.constraints.push(c);
    }

    return in;
}

template<typename FieldT>
void uscs_constraint_system<FieldT>::report_linear_constraint_statistics() const
{
// #ifdef DEBUG
    for i in 0..constraints.size()
    {
        auto &constr = constraints[i];
        bool a_is_const = true;
        for t in &constr.terms
        {
            a_is_const = a_is_const && (t.index == 0);
        }

        if a_is_const
        {
            print!("%s\n", (if let Some(it) = constraint_annotations.find(i){it.1.to_string()} else{FMT("", "constraint_{}", i)} );
        }
    }
//#endif
}



//#endif // USCS_TCC_
