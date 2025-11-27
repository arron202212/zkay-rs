use crate::relations::FieldTConfig;
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

// use  <cstdlib>
// use  <iostream>
// use  <map>
// use  <string>
use crate::relations::variable;
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination,
};
use std::collections::BTreeMap;
use std::fmt;
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

pub type uscs_constraint<FieldT, SV, SLC> = linear_combination<FieldT, SV, SLC>;

/************************* USCS variable assignment **************************/

/**
 * A USCS variable assignment is a vector of <FieldT> elements that represents
 * a candidate solution to a USCS constraint system (see below).
 */

pub type uscs_primary_input<FieldT> = Vec<FieldT>;

pub type uscs_auxiliary_input<FieldT> = Vec<FieldT>;

pub type uscs_variable_assignment<FieldT> = Vec<FieldT>;

/************************* USCS constraint system ****************************/

// pub struct uscs_constraint_system;

// std::ostream& operator<<(std::ostream &out, cs:&uscs_constraint_system<FieldT>);

// std::istream& operator>>(std::istream &in, uscs_constraint_system<FieldT> &cs);

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
#[derive(Default, Clone)]
pub struct uscs_constraint_system<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    pub primary_input_size: usize,
    pub auxiliary_input_size: usize,
    pub constraints: Vec<uscs_constraint<FieldT, SV, SLC>>,

    //     uscs_constraint_system()->Self primary_input_size(0), auxiliary_input_size(0) {};

    //     usize num_inputs() const;
    //     usize num_variables() const;
    //     usize num_constraints() const;

    // // #ifdef DEBUG
    pub constraint_annotations: BTreeMap<usize, String>,
    pub variable_annotations: BTreeMap<usize, String>,
    // //#endif

    //     bool is_valid() const;
    //     bool is_satisfied(primary_input:&uscs_primary_input<FieldT>,
    //                       auxiliary_input:&uscs_auxiliary_input<FieldT>) const;

    //     pub fn  add_constraint(constraint:&uscs_constraint<FieldT,SLC>);
    //     pub fn  add_constraint(constraint:&uscs_constraint<FieldT,SLC>, annotation:&String);

    //     bool operator==(other:&uscs_constraint_system<FieldT>) const;

    //     friend std::ostream& operator<< <FieldT>(std::ostream &out, cs:&uscs_constraint_system<FieldT>);
    //     friend std::istream& operator>> <FieldT>(std::istream &in, uscs_constraint_system<FieldT> &cs);

    //     pub fn  report_linear_constraint_statistics() const;
}

// use crate::relations::constraint_satisfaction_problems/uscs/uscs;

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

// use  <algorithm>
// use  <cassert>
// use  <set>
use ffec::algebra::field_utils::bigint::bigint;
use ffec::common::profiling;
use ffec::common::utils;

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    uscs_constraint_system<FieldT, SV, SLC>
{
    pub fn num_inputs(&self) -> usize {
        return self.primary_input_size;
    }

    pub fn num_variables(&self) -> usize {
        return self.primary_input_size + self.auxiliary_input_size;
    }

    pub fn num_constraints(&self) -> usize {
        return self.constraints.len();
    }

    pub fn is_valid(&self) -> bool {
        if self.num_inputs() > self.num_variables() {
            return false;
        }

        for c in 0..self.constraints.len() {
            if self.constraints[c].terms.len() != self.num_variables() {
                return false;
            }
        }

        return true;
    }

    pub fn is_satisfied(
        &self,
        primary_input: &uscs_primary_input<FieldT>,
        auxiliary_input: &uscs_auxiliary_input<FieldT>,
    ) -> bool {
        assert!(primary_input.len() == self.num_inputs());
        assert!(primary_input.len() + auxiliary_input.len() == self.num_variables());

        let mut full_variable_assignment: Vec<_> = primary_input
            .iter()
            .chain(auxiliary_input)
            .cloned()
            .collect();

        for c in 0..self.constraints.len() {
            let mut res = self.constraints[c].evaluate(&full_variable_assignment);
            if !(res.squared() == FieldT::one()) {
                // #ifdef DEBUG
                print!(
                    "constraint {} ({}) unsatisfied\n",
                    c,
                    (if let Some(it) = self.constraint_annotations.get(&c) {
                        it
                    } else {
                        "no annotation"
                    })
                );
                print!("<a,(1,x)> = ");
                res.print();
                print!("constraint was:\n");
                dump_uscs_constraint(
                    &self.constraints[c],
                    &full_variable_assignment,
                    &self.variable_annotations,
                );
                //#endif // DEBUG
                return false;
            }
        }

        return true;
    }

    pub fn add_constraint0(&mut self, c: uscs_constraint<FieldT, SV, SLC>) {
        self.constraints.push(c);
    }

    pub fn add_constraint(&mut self, c: uscs_constraint<FieldT, SV, SLC>, annotation: &str) {
        // #ifdef DEBUG
        self.constraint_annotations
            .insert(self.constraints.len(), annotation.to_owned());
        // #else
        //ffec::UNUSED(annotation);
        //#endif
        self.constraints.push(c);
    }

    pub fn report_linear_constraint_statistics(&self) {
        // #ifdef DEBUG
        for i in 0..self.constraints.len() {
            let constr = &self.constraints[i];
            let mut a_is_const = true;
            for t in &constr.terms {
                a_is_const = a_is_const && (t.index.index == 0);
            }

            if a_is_const {
                print!(
                    "{}\n",
                    if let Some(it) = self.constraint_annotations.get(&i) {
                        it.to_string()
                    } else {
                        format!("constraint_{}", i)
                    }
                );
            }
        }
        //#endif
    }
}

//#endif // USCS_TCC_

pub fn dump_uscs_constraint<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    constraint: &uscs_constraint<FieldT, SV, SLC>,
    full_variable_assignment: &uscs_variable_assignment<FieldT>,
    variable_annotations: &BTreeMap<usize, String>,
) {
    print!("terms:\n");
    constraint.print_with_assignment(full_variable_assignment, variable_annotations);
}

// bool uscs_constraint_system<FieldT>::operator==(other:&uscs_constraint_system<FieldT>) const
// {
//     return (self.constraints == other.constraints &&
//             self.primary_input_size == other.primary_input_size &&
//             self.auxiliary_input_size == other.auxiliary_input_size);
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> PartialEq
    for uscs_constraint_system<FieldT, SV, SLC>
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.constraints == other.constraints
            && self.primary_input_size == other.primary_input_size
            && self.auxiliary_input_size == other.auxiliary_input_size
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> fmt::Display
    for uscs_constraint_system<FieldT, SV, SLC>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.primary_input_size,
            self.auxiliary_input_size,
            self.num_constraints(),
            self.constraints
                .iter()
                .map(|c| format!("{c}"))
                .collect::<String>(),
        )
    }
}
// std::ostream& operator<<(std::ostream &out, cs:&uscs_constraint_system<FieldT>)
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

// std::istream& operator>>(std::istream &in, uscs_constraint_system<FieldT> &cs)
// {
//     in >> cs.primary_input_size;
//     in >> cs.auxiliary_input_size;

//     cs.constraints.clear();

//     usize s;
//     in >> s;

//     char b;
//     in.read(&b, 1);

//     cs.constraints.reserve(s);

//     for i in 0..s
//     {
//         uscs_constraint<FieldT,SLC> c;
//         in >> c;
//         cs.constraints.push(c);
//     }

//     return in;
// }
