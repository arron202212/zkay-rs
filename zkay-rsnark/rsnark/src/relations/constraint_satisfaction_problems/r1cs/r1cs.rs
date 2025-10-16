/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a R1CS constraint,
 - a R1CS variable assignment, and
 - a R1CS constraint system.

 Above, R1CS stands for "Rank-1 Constraint System".

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_HPP_
// #define R1CS_HPP_

// use  <cstdlib>
// use  <iostream>
// use  <map>
// use  <string>
// use  <vector>

use crate::relations::variable;



/************************* R1CS constraint ***********************************/


/**
 * A R1CS constraint is a formal expression of the form
 *
 *                < A , X > * < B , X > = < C , X > ,
 *
 * where X = (x_0,x_1,...,x_m) is a vector of formal variables and A,B,C each
 * consist of 1+m elements in <FieldT>.
 *
 * A R1CS constraint is used to construct a R1CS constraint system (see below).
 */
// template<typename FieldT>
pub struct r1cs_constraint<FieldT> {


    linear_combination<FieldT> a, b, c;
}

//     r1cs_constraint() {};
//     r1cs_constraint(a:&linear_combination<FieldT>
//                     b:&linear_combination<FieldT>
//                     &c:linear_combination<FieldT>);

//     r1cs_constraint(A:&std::initializer_list<linear_combination<FieldT> >
//                     B:&std::initializer_list<linear_combination<FieldT> >
//                     &C:std::initializer_list<linear_combination<FieldT> >);

   
// };

/************************* R1CS variable assignment **************************/

/**
 * A R1CS variable assignment is a vector of <FieldT> elements that represents
 * a candidate solution to a R1CS constraint system (see below).
 */

/* TODO: specify that it does *NOT* include the constant 1 */
template<typename FieldT>
using r1cs_primary_input = std::vector<FieldT>;

template<typename FieldT>
using r1cs_auxiliary_input = std::vector<FieldT>;

template<typename FieldT>
using r1cs_variable_assignment = std::vector<FieldT>; /* note the changed name! (TODO: remove this comment after primary_input transition is complete) */

/************************* R1CS constraint system ****************************/


/**
 * A system of R1CS constraints looks like
 *
 *     { < A_k , X > * < B_k , X > = < C_k , X > }_{k=1}^{n}  .
 *
 * In other words, the system is satisfied if and only if there exist a
 * USCS variable assignment for which each R1CS constraint is satisfied.
 *
 * NOTE:
 * The 0-th variable (i.e., "x_{0}") always represents the constant 1.
 * Thus, the 0-th variable is not included in num_variables.
 */
template<typename FieldT>
class r1cs_constraint_system {

    size_t primary_input_size;
    size_t auxiliary_input_size;

    std::vector<r1cs_constraint<FieldT> > constraints;

    r1cs_constraint_system() : primary_input_size(0), auxiliary_input_size(0) {}

    size_t num_inputs() const;
    size_t num_variables() const;
    size_t num_constraints() const;

// #ifdef DEBUG
    std::map<size_t, std::string> constraint_annotations;
    std::map<size_t, std::string> variable_annotations;
//#endif

    bool is_valid() const;
    bool is_satisfied(primary_input:&r1cs_primary_input<FieldT>
                      &auxiliary_input:r1cs_auxiliary_input<FieldT>) const;

    void add_constraint(&c:r1cs_constraint<FieldT>);
    void add_constraint(c:&r1cs_constraint<FieldT> &annotation:std::string);

    void swap_AB_if_beneficial();


    void report_linear_constraint_statistics() const;
};




use crate::relations::constraint_satisfaction_problems/r1cs/r1cs;

//#endif // R1CS_HPP_



/** @file
 *****************************************************************************

 Declaration of interfaces for:
 - a R1CS constraint,
 - a R1CS variable assignment, and
 - a R1CS constraint system.

 See r1cs.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_TCC_
// #define R1CS_TCC_

use  <algorithm>
use  <cassert>
use  <set>

use ffec::algebra::fields::bigint;
use ffec::common::profiling;
use ffec::common::utils;



template<typename FieldT>
r1cs_constraint<FieldT>::r1cs_constraint(a:&linear_combination<FieldT>
                                         b:&linear_combination<FieldT>
                                         &c:linear_combination<FieldT>) :
    a(a), b(b), c(c)
{
}

template<typename FieldT>
r1cs_constraint<FieldT>::r1cs_constraint(A:&std::initializer_list<linear_combination<FieldT> >
                                         B:&std::initializer_list<linear_combination<FieldT> >
                                         &C:std::initializer_list<linear_combination<FieldT> >)
{
    for lc_A in &A
    {
        a.terms.insert(a.terms.end(), lc_A.terms.begin(), lc_A.terms.end());
    }
    for lc_B in &B
    {
        b.terms.insert(b.terms.end(), lc_B.terms.begin(), lc_B.terms.end());
    }
    for lc_C in &C
    {
        c.terms.insert(c.terms.end(), lc_C.terms.begin(), lc_C.terms.end());
    }
}

template<typename FieldT>
bool r1cs_constraint<FieldT>::operator==(&other:r1cs_constraint<FieldT>) const
{
    return (self.a == other.a &&
            self.b == other.b &&
            self.c == other.c);
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
std::ostream& operator<<(std::ostream &out, &c:r1cs_constraint<FieldT>)
{
    out << c.a;
    out << c.b;
    out << c.c;

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, r1cs_constraint<FieldT> &c)
{
    in >> c.a;
    in >> c.b;
    in >> c.c;

    return in;
}

template<typename FieldT>
size_t r1cs_constraint_system<FieldT>::num_inputs() const
{
    return primary_input_size;
}

template<typename FieldT>
size_t r1cs_constraint_system<FieldT>::num_variables() const
{
    return primary_input_size + auxiliary_input_size;
}


template<typename FieldT>
size_t r1cs_constraint_system<FieldT>::num_constraints() const
{
    return constraints.size();
}

template<typename FieldT>
bool r1cs_constraint_system<FieldT>::is_valid() const
{
    if self.num_inputs() > self.num_variables() return false;

    for c in 0..constraints.size()
    {
        if !(constraints[c].a.is_valid(self.num_variables() &&
              constraints[c].b.is_valid(self.num_variables()) &&
              constraints[c].c.is_valid(self.num_variables())))
        {
            return false;
        }
    }

    return true;
}

template<typename FieldT>
void dump_r1cs_constraint(constraint:&r1cs_constraint<FieldT>
                          full_variable_assignment:&r1cs_variable_assignment<FieldT>
                          &variable_annotations:std::map<size_t, std::string>)
{
    print!("terms for a:\n"); constraint.a.print_with_assignment(full_variable_assignment, variable_annotations);
    print!("terms for b:\n"); constraint.b.print_with_assignment(full_variable_assignment, variable_annotations);
    print!("terms for c:\n"); constraint.c.print_with_assignment(full_variable_assignment, variable_annotations);
}

template<typename FieldT>
bool r1cs_constraint_system<FieldT>::is_satisfied(primary_input:&r1cs_primary_input<FieldT>
                                                  &auxiliary_input:r1cs_auxiliary_input<FieldT>) const
{
    assert!(primary_input.size() == num_inputs());
    assert!(primary_input.size() + auxiliary_input.size() == num_variables());

    r1cs_variable_assignment<FieldT> full_variable_assignment = primary_input;
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());

    for c in 0..constraints.size()
    {
        constraints[c].a.evaluate(full_variable_assignment:FieldT ares =);
        constraints[c].b.evaluate(full_variable_assignment:FieldT bres =);
        constraints[c].c.evaluate(full_variable_assignment:FieldT cres =);

        if !(ares*bres == cres)
        {
// #ifdef DEBUG
            auto it = constraint_annotations.find(c);
            print!("constraint {} (%s) unsatisfied\n", c, (it == constraint_annotations.end() ? "no annotation" : it->second.c_str()));
            print!("<a,(1,x)> = "); ares.print();
            print!("<b,(1,x)> = "); bres.print();
            print!("<c,(1,x)> = "); cres.print();
            print!("constraint was:\n");
            dump_r1cs_constraint(constraints[c], full_variable_assignment, variable_annotations);
//#endif // DEBUG
            return false;
        }
    }

    return true;
}

template<typename FieldT>
void r1cs_constraint_system<FieldT>::add_constraint(&c:r1cs_constraint<FieldT>)
{
    constraints.push(c);
}

template<typename FieldT>
void r1cs_constraint_system<FieldT>::add_constraint(c:&r1cs_constraint<FieldT> &annotation:std::string)
{
// #ifdef DEBUG
    constraint_annotations[constraints.size()] = annotation;
//#endif
    constraints.push(c);
}

template<typename FieldT>
void r1cs_constraint_system<FieldT>::swap_AB_if_beneficial()
{
    ffec::enter_block("Call to r1cs_constraint_system::swap_AB_if_beneficial");

    ffec::enter_block("Estimate densities");
    ffec::bit_vector touched_by_A(self.num_variables() + 1, false), touched_by_B(self.num_variables() + 1, false);

    for i in 0..self.constraints.size()
    {
        for j in 0..self.constraints[i].a.terms.size()
        {
            touched_by_A[self.constraints[i].a.terms[j].index] = true;
        }

        for j in 0..self.constraints[i].b.terms.size()
        {
            touched_by_B[self.constraints[i].b.terms[j].index] = true;
        }
    }

    size_t non_zero_A_count = 0, non_zero_B_count = 0;
    for i in 0..self.num_variables() + 1
    {
        non_zero_A_count += touched_by_A[i] ? 1 : 0;
        non_zero_B_count += touched_by_B[i] ? 1 : 0;
    }

    if !ffec::inhibit_profiling_info
    {
        ffec::print_indent(); print!("* Non-zero A-count (estimate): {}\n", non_zero_A_count);
        ffec::print_indent(); print!("* Non-zero B-count (estimate): {}\n", non_zero_B_count);
    }
    ffec::leave_block("Estimate densities");

    if non_zero_B_count > non_zero_A_count
    {
        ffec::enter_block("Perform the swap");
        for i in 0..self.constraints.size()
        {
            std::swap(self.constraints[i].a, self.constraints[i].b);
        }
        ffec::leave_block("Perform the swap");
    }
    else
    {
        ffec::print_indent(); print!("Swap is not beneficial, not performing\n");
    }

    ffec::leave_block("Call to r1cs_constraint_system::swap_AB_if_beneficial");
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
bool r1cs_constraint_system<FieldT>::operator==(&other:r1cs_constraint_system<FieldT>) const
{
    return (self.constraints == other.constraints &&
            self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size);
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
std::ostream& operator<<(std::ostream &out, &cs:r1cs_constraint_system<FieldT>)
{
    out << cs.primary_input_size << "\n";
    out << cs.auxiliary_input_size << "\n";

    out << cs.num_constraints() << "\n";
    for (cs.constraints:r1cs_constraint<FieldT>& c :)
    {
        out << c;
    }

    return out;
}

template<typename FieldT>
std::istream& operator>>(std::istream &in, r1cs_constraint_system<FieldT> &cs)
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
        r1cs_constraint<FieldT> c;
        in >> c;
        cs.constraints.push(c);
    }

    return in;
}

template<typename FieldT>
void r1cs_constraint_system<FieldT>::report_linear_constraint_statistics() const
{
// #ifdef DEBUG
    for i in 0..constraints.size()
    {
        auto &constr = constraints[i];
        bool a_is_const = true;
        for t in &constr.a.terms
        {
            a_is_const = a_is_const && (t.index == 0);
        }

        bool b_is_const = true;
        for t in &constr.b.terms
        {
            b_is_const = b_is_const && (t.index == 0);
        }

        if a_is_const || b_is_const
        {
            auto it = constraint_annotations.find(i);
            print!("%s\n", (it == constraint_annotations.end() ? FMT("", "constraint_{}", i) : it->second).c_str());
        }
    }
//#endif
}


//#endif // R1CS_TCC_
