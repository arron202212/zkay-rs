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
// < FieldT>
pub struct r1cs_constraint<FieldT> {

     a:linear_combination<FieldT>, b:linear_combination<FieldT>, c:linear_combination<FieldT>
}

//     r1cs_constraint() {};
//     r1cs_constraint(a:&linear_combination<FieldT>
//                     b:&linear_combination<FieldT>
//                     c:&linear_combination<FieldT>);

//     r1cs_constraint(A:Vec<linear_combination<FieldT> >
//                     B:Vec<linear_combination<FieldT> >
//                     C:Vec<linear_combination<FieldT> >);

   
// };

/************************* R1CS variable assignment **************************/

/**
 * A R1CS variable assignment is a vector of <FieldT> elements that represents
 * a candidate solution to a R1CS constraint system (see below).
 */

/* TODO: specify that it does *NOT* include the constant 1 */
// < FieldT>
// using r1cs_primary_input = std::vector<FieldT>;

// < FieldT>
// using r1cs_auxiliary_input = std::vector<FieldT>;

// < FieldT>
// using r1cs_variable_assignment = std::vector<FieldT>; /* note the changed name! (TODO: remove this comment after primary_input transition is complete) */

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

pub struct  r1cs_constraint_system< FieldT> {

primary_input_size:    size_t,
auxiliary_input_size:    size_t,

constraints:    std::vector<r1cs_constraint<FieldT> >,


    // size_t num_inputs() const;
    // size_t num_variables() const;
    // size_t num_constraints() const;

// #ifdef DEBUG
constraint_annotations:    std::map<size_t, std::string>,
variable_annotations:    std::map<size_t, std::string>,
//#endif
}

//     bool is_valid() const;
//     bool is_satisfied(primary_input:&r1cs_primary_input<FieldT>
//                       &auxiliary_input:r1cs_auxiliary_input<FieldT>) const;

//     void add_constraint(&c:r1cs_constraint<FieldT>);
//     void add_constraint(c:&r1cs_constraint<FieldT> &annotation:std::string);

//     void swap_AB_if_beneficial();


//     void report_linear_constraint_statistics() const;
// };

impl r1cs_constraint_system< FieldT> {
    pub fn new()->Self {Self{primary_input_size:0, auxiliary_input_size:0 }}
}


// use crate::relations::constraint_satisfaction_problems/r1cs/r1cs;

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

// use  <algorithm>
// use  <cassert>
// use  <set>

use ffec::algebra::field_utils::bigint::bigint;
use ffec::common::profiling;
use ffec::common::utils;


impl r1cs_constraint<FieldT>{

pub fn  new(a:&linear_combination<FieldT>,
                                         b:&linear_combination<FieldT>,
                                         &c:linear_combination<FieldT>) ->Self
   
{
 Self{a, b, c}
}


pub fn new2(A:Vec<linear_combination<FieldT> >,
                                         B:Vec<linear_combination<FieldT> >,
                                         C:Vec<linear_combination<FieldT> >)->Self
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
    Self{a,b,c}
}
}
impl<FieldT> PartialEq for r1cs_constraint<FieldT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a &&
            self.b == other.b &&
            self.c == other.c
    }
}
// < FieldT>
// bool r1cs_constraint<FieldT>::operator==(&other:r1cs_constraint<FieldT>) const
// {
//     return (self.a == other.a &&
//             self.b == other.b &&
//             self.c == other.c);
// }
impl<FieldT> fmt::Display for  r1cs_constraint<FieldT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}",  
     self.a,
     self.b,
     self.c,
)
    }
}
// < FieldT>
// std::ostream& operator<<(std::ostream &out, &c:r1cs_constraint<FieldT>)
// {
//     out << c.a;
//     out << c.b;
//     out << c.c;

//     return out;
// }

// < FieldT>
// std::istream& operator>>(std::istream &in, r1cs_constraint<FieldT> &c)
// {
//     in >> c.a;
//     in >> c.b;
//     in >> c.c;

//     return in;
// }

impl r1cs_constraint_system<FieldT>{
pub fn  num_inputs()  ->usize
{
    return primary_input_size;
}

pub fn num_variables()  ->usize
{
    return primary_input_size + auxiliary_input_size;
}



 pub fn num_constraints() ->usize
{
    return constraints.size();
}


 pub fn is_valid() ->bool
{
    if self.num_inputs() > self.num_variables() {return false;}

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



pub fn  dump_r1cs_constraint(constraint:&r1cs_constraint<FieldT>,
                          full_variable_assignment:&r1cs_variable_assignment<FieldT>,
                          variable_annotations:&std::map<size_t, std::string>)
{
    print!("terms for a:\n"); constraint.a.print_with_assignment(full_variable_assignment, variable_annotations);
    print!("terms for b:\n"); constraint.b.print_with_assignment(full_variable_assignment, variable_annotations);
    print!("terms for c:\n"); constraint.c.print_with_assignment(full_variable_assignment, variable_annotations);
}


pub fn is_satisfied(primary_input:&r1cs_primary_input<FieldT>,
                                                  auxiliary_input:&r1cs_auxiliary_input<FieldT>) ->bool
{
    assert!(primary_input.size() == num_inputs());
    assert!(primary_input.size() + auxiliary_input.size() == num_variables());

    let mut  full_variable_assignment = primary_input.clone();
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());

    for c in 0..constraints.size()
    {
       let  ares =constraints[c].a.evaluate(full_variable_assignment);
       let bres = constraints[c].b.evaluate(full_variable_assignment);
       let cres = constraints[c].c.evaluate(full_variable_assignment);

        if ares*bres != cres
        {
// #ifdef DEBUG
            // let  it = constraint_annotations.find(c);
            print!("constraint {} ({}) unsatisfied\n", c, (if let Some(it)=constraint_annotations.find(c) {it.1}else{ "no annotation"}));
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


pub fn add_constraint(&c:r1cs_constraint<FieldT>)
{
    constraints.push(c);
}


pub fn add_constraint(c:&r1cs_constraint<FieldT> ,annotation:&std::string)
{
// #ifdef DEBUG
    // constraint_annotations[constraints.size()] = annotation;
//#endif
    constraints.push(c);
}


pub fn swap_AB_if_beneficial()
{
    ffec::enter_block("Call to r1cs_constraint_system::swap_AB_if_beneficial");

    ffec::enter_block("Estimate densities");
    let mut  touched_by_A=vec![false;self.num_variables() + 1];
    let mut  touched_by_B=vec![false;self.num_variables() + 1];

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

    let mut  non_zero_A_count = 0;
    let mut  non_zero_B_count = 0;
    for i in 0..self.num_variables() + 1
    {
        non_zero_A_count += if touched_by_A[i]  { 1 } else{ 0};
        non_zero_B_count += if touched_by_B[i]  { 1 } else{ 0};
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
           (self.constraints[i].b, self.constraints[i].a) =(self.constraints[i].a, self.constraints[i].b);
        }
        ffec::leave_block("Perform the swap");
    }
    else
    {
        ffec::print_indent(); print!("Swap is not beneficial, not performing\n");
    }

    ffec::leave_block("Call to r1cs_constraint_system::swap_AB_if_beneficial");
}





pub fn report_linear_constraint_statistics() 
{
// #ifdef DEBUG
    for i in 0..constraints.size()
    {
        let constr = constraints[i];
        let mut  a_is_const = true;
        for t in &constr.a.terms
        {
            a_is_const = a_is_const && (t.index == 0);
        }

        let mut  b_is_const = true;
        for t in &constr.b.terms
        {
            b_is_const = b_is_const && (t.index == 0);
        }

        if a_is_const || b_is_const
        {
            print!("{}\n", if let Some(it)=constraint_annotations.find(i){it.1} else{ FMT("", "constraint_{}", i)});
        }
    }
//#endif
}
}

//#endif // R1CS_TCC_



impl<FieldT> PartialEq for r1cs_constraint_system<FieldT> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.constraints == other.constraints &&
            self.primary_input_size == other.primary_input_size &&
            self.auxiliary_input_size == other.auxiliary_input_size
    }
}

// < FieldT>
// bool r1cs_constraint_system<FieldT>::operator==(&other:r1cs_constraint_system<FieldT>) const
// {
//     return (self.constraints == other.constraints &&
//             self.primary_input_size == other.primary_input_size &&
//             self.auxiliary_input_size == other.auxiliary_input_size);
// }
impl<FieldT> fmt::Display for r1cs_constraint_system<FieldT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{}",  
cs.primary_input_size,
cs.auxiliary_input_size,cs.num_constraints(),
cs.constraints.iter().map(|c|format!("{c}")).collect::<String>(),
)
    }
}

// std::ostream& operator<<(std::ostream &out, &cs:r1cs_constraint_system<FieldT>)
// {
//     out << cs.primary_input_size << "\n";
//     out << cs.auxiliary_input_size << "\n";

//     out << cs.num_constraints() << "\n";
//     for c in &cs.constraints
//     {
//         out << c;
//     }

//     return out;
// }

// < FieldT>
// std::istream& operator>>(std::istream &in, r1cs_constraint_system<FieldT> &cs)
// {
//     in >> cs.primary_input_size;
//     in >> cs.auxiliary_input_size;

//     cs.constraints.clear();

//     size_t s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     cs.constraints.reserve(s);

//     for i in 0..s
//     {
//         r1cs_constraint<FieldT> c;
//         in >> c;
//         cs.constraints.push(c);
//     }

//     return in;
// }