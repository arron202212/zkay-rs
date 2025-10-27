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

using uscs_constraint = linear_combination<FieldT>;


/************************* USCS variable assignment **************************/

/**
 * A USCS variable assignment is a vector of <FieldT> elements that represents
 * a candidate solution to a USCS constraint system (see below).
 */

using uscs_primary_input = Vec<FieldT>;


using uscs_auxiliary_input = Vec<FieldT>;


using uscs_variable_assignment = Vec<FieldT>;



/************************* USCS constraint system ****************************/


pub struct uscs_constraint_system;


std::ostream& operator<<(std::ostream &out, cs:&uscs_constraint_system<FieldT>);


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

pub struct uscs_constraint_system {

    usize primary_input_size;
    usize auxiliary_input_size;

    Vec<uscs_constraint<FieldT> > constraints;

    uscs_constraint_system()->Self primary_input_size(0), auxiliary_input_size(0) {};

    usize num_inputs() const;
    usize num_variables() const;
    usize num_constraints() const;

// #ifdef DEBUG
    BTreeMap<usize, String> constraint_annotations;
    BTreeMap<usize, String> variable_annotations;
//#endif

    bool is_valid() const;
    bool is_satisfied(primary_input:&uscs_primary_input<FieldT>,
                      auxiliary_input:&uscs_auxiliary_input<FieldT>) const;

    pub fn  add_constraint(constraint:&uscs_constraint<FieldT>);
    pub fn  add_constraint(constraint:&uscs_constraint<FieldT>, annotation:&String);

    bool operator==(other:&uscs_constraint_system<FieldT>) const;

    friend std::ostream& operator<< <FieldT>(std::ostream &out, cs:&uscs_constraint_system<FieldT>);
    friend std::istream& operator>> <FieldT>(std::istream &in, uscs_constraint_system<FieldT> &cs);

    pub fn  report_linear_constraint_statistics() const;
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

use ffec::algebra::field_utils::bigint::bigint;
use ffec::common::profiling;
use ffec::common::utils;




pub fn num_inputs()->usize
{
    return primary_input_size;
}


pub fn num_variables()->usize
{
    return primary_input_size + auxiliary_input_size;
}



pub fn num_constraints()->usize
{
    return constraints.len();
}


pub fn is_valid()->bool
{
    if self.num_inputs() > self.num_variables() return false;

    for c in 0..constraints.len()
    {
        if !valid_vector(constraints[c], self.num_variables())
        {
            return false;
        }
    }

    return true;
}


pub fn  dump_uscs_constraint(constraint:&uscs_constraint<FieldT>,
                          full_variable_assignment:&uscs_variable_assignment<FieldT>,
                          variable_annotations:&BTreeMap<usize, String>)
{
    print!("terms:\n");
    constraint.print_with_assignment(full_variable_assignment, variable_annotations);
}


bool uscs_constraint_system<FieldT>::is_satisfied(primary_input:&uscs_primary_input<FieldT>,
                                                  auxiliary_input:&uscs_auxiliary_input<FieldT>) const
{
    assert!(primary_input.len() == num_inputs());
    assert!(primary_input.len() + auxiliary_input.len() == num_variables());

    uscs_variable_assignment<FieldT> full_variable_assignment = primary_input;
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());

    for c in 0..constraints.len()
    {
        FieldT res = constraints[c].evaluate(full_variable_assignment);
        if !(res.squared() == FieldT::one())
        {
// #ifdef DEBUG
            auto it = constraint_annotations.find(c);
            print!("constraint {} ({}) unsatisfied\n", c, ( if let Some(it) =constraint_annotations.find(c) {it.1.to_string()} else {"no annotation" }));
            print!("<a,(1,x)> = "); res.print();
            print!("constraint was:\n");
            dump_uscs_constraint(constraints[c], full_variable_assignment, variable_annotations);
//#endif // DEBUG
            return false;
        }
    }

    return true;
}


pub fn add_constraint(c:&uscs_constraint<FieldT>)
{
    constraints.push(c);
}


pub fn add_constraint(c:&uscs_constraint<FieldT>, annotation:&String)
{
// #ifdef DEBUG
    constraint_annotations[constraints.len()] = annotation;
#else
    //ffec::UNUSED(annotation);
//#endif
    constraints.push(c);
}


bool uscs_constraint_system<FieldT>::operator==(other:&uscs_constraint_system<FieldT>) const
{
    return (self.constraints == other.constraints &&
            self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size);
}


std::ostream& operator<<(std::ostream &out, cs:&uscs_constraint_system<FieldT>)
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


std::istream& operator>>(std::istream &in, uscs_constraint_system<FieldT> &cs)
{
    in >> cs.primary_input_size;
    in >> cs.auxiliary_input_size;

    cs.constraints.clear();

    usize s;
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


pub fn report_linear_constraint_statistics() const
{
// #ifdef DEBUG
    for i in 0..constraints.len()
    {
        auto &constr = constraints[i];
        bool a_is_const = true;
        for t in &constr.terms
        {
            a_is_const = a_is_const && (t.index == 0);
        }

        if a_is_const
        {
            print!("{}\n", (if let Some(it) = constraint_annotations.find(i){it.1.to_string()} else{FMT("", "constraint_{}", i)} );
        }
    }
//#endif
}



//#endif // USCS_TCC_
