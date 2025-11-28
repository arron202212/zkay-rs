use crate::relations::FieldTConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;
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
//#ifndef VARIABLE_HPP_
// #define VARIABLE_HPP_

// use  <cstddef>
// use  <map>
// use  <string>
//
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg, Sub};
/**
 * Mnemonic typedefs.
 */
pub type var_index_t = usize;
pub type integer_coeff_t = i64;

// /**
//  * Forward declaration.
//  */
//
// pub structlinear_term;

// /**
//  * Forward declaration.
//  */
//
// pub structlinear_combination;

/********************************* Variable **********************************/

pub trait SubVariableConfig: Default + Clone + std::cmp::PartialEq {}
#[derive(Clone, Default, PartialEq)]
pub struct DefaultVariable;
impl SubVariableConfig for DefaultVariable {}
pub type DV = DefaultVariable;

pub trait SubLinearCombinationConfig: Default + Clone + std::cmp::PartialEq {}

#[derive(Clone, Default, PartialEq)]
pub struct DefaultLinearCombination;
impl SubLinearCombinationConfig for DefaultLinearCombination {}
pub type DLC = DefaultLinearCombination;

/**
 *  A variable represents a formal expression of the form "x_{index}".
 */
#[derive(Clone, Default)]
pub struct variable<FieldT: FieldTConfig, SV: SubVariableConfig> {
    pub index: var_index_t,
    _t: PhantomData<FieldT>,
    t: SV,
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig> variable<FieldT, SV> {
    pub fn new(index: var_index_t, t: SV) -> Self {
        Self {
            index,
            _t: PhantomData,
            t,
        }
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<var_index_t> for variable<FieldT, SV> {
    fn from(rhs: var_index_t) -> Self {
        Self {
            index: rhs,
            _t: PhantomData,
            t: SV::default(),
        }
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig> Mul<FieldT> for variable<FieldT, SV> {
    type Output = linear_term<FieldT, SV>;

    fn mul(self, rhs: FieldT) -> Self::Output {
        linear_term::<FieldT, SV>::new_with_field(self, rhs)
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> Mul<integer_coeff_t> for variable<FieldT, SV> {
    type Output = linear_term<FieldT, SV>;

    fn mul(self, rhs: integer_coeff_t) -> Self::Output {
        linear_term::<FieldT, SV>::new_with_int_coeff(self, rhs)
    }
}

//     variable(index(index:var_index_t index = 0)->Self) {};

//     linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
//     linear_term<FieldT> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT,T> operator+(&other:linear_combination<FieldT,T>) const;
//     linear_combination<FieldT,T> operator-(&other:linear_combination<FieldT,T>) const;

//     linear_term<FieldT> operator-() const;

//     bool operator==(&other:variable<FieldT,T>) const;
// };

//
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT,T>);

//
// linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT,T>);

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &var:variable<FieldT,T>);

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &var:variable<FieldT,T>);

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &var:variable<FieldT,T>);

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &var:variable<FieldT,T>);

/****************************** Linear term **********************************/

/**
 * A linear term represents a formal expression of the form "coeff * x_{index}".
 */
#[derive(Clone)]
pub struct linear_term<FieldT: FieldTConfig, SV: SubVariableConfig> {
    pub index: variable<FieldT, SV>,
    pub coeff: FieldT,
}
//     linear_term() {};
//     linear_term(&var:variable<FieldT,T>);
//     linear_term(var:&variable<FieldT,T> int_coeff:integer_coeff_t);
//     linear_term(var:&variable<FieldT,T> &field_coeff:FieldT);

//     linear_term<FieldT> operator*(int_coeff:integer_coeff_t) const;
//     linear_term<FieldT> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT,T> operator+(&other:linear_combination<FieldT,T>) const;
//     linear_combination<FieldT,T> operator-(&other:linear_combination<FieldT,T>) const;

//     linear_term<FieldT> operator-() const;

//     bool operator==(&other:linear_term<FieldT>) const;
// };

//
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

//
// linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>);

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>);

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>);

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>);

/***************************** Linear combination ****************************/

/**
 * A linear combination represents a formal expression of the form "sum_i coeff_i * x_{index_i}".
 */
#[derive(Clone, Default, PartialEq)]
pub struct linear_combination<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
> {
    pub terms: Vec<linear_term<FieldT, SV>>,
    pub t: SLC,
}
use std::borrow::Borrow;
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> IntoIterator
    for linear_combination<FieldT, SV, SLC>
{
    type Item = linear_term<FieldT, SV>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.terms.into_iter()
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> IntoIterator
    for &linear_combination<FieldT, SV, SLC>
{
    type Item = linear_term<FieldT, SV>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.terms.clone().into_iter()
    }
}
//     linear_combination() {};
//     linear_combination(int_coeff:integer_coeff_t);
//     linear_combination(&field_coeff:FieldT);
//     linear_combination(&var:variable<FieldT,T>);
//     linear_combination(&lt:linear_term<FieldT>);
//     linear_combination(&all_terms:Vec<linear_term<FieldT> >);

//     /* for supporting range-based for loops over linear_combination */
//     Vec<linear_term<FieldT> >::const_iterator begin() const;
//     Vec<linear_term<FieldT> >::const_iterator end() const;

//     pub fn  add_term(&var:variable<FieldT,T>);
//     pub fn  add_term(var:&variable<FieldT,T> int_coeff:integer_coeff_t);
//     pub fn  add_term(var:&variable<FieldT,T> &field_coeff:FieldT);

//     pub fn  add_term(&lt:linear_term<FieldT>);

//     FieldT evaluate(&assignment:Vec<FieldT>) const;

//     linear_combination<FieldT,T> operator*(int_coeff:integer_coeff_t) const;
//     linear_combination<FieldT,T> operator*(&field_coeff:FieldT) const;

//     linear_combination<FieldT,T> operator+(&other:linear_combination<FieldT,T>) const;

//     linear_combination<FieldT,T> operator-(&other:linear_combination<FieldT,T>) const;
//     linear_combination<FieldT,T> operator-() const;

//     bool operator==(&other:linear_combination<FieldT,T>) const;

//     bool is_valid(num_variables:usize) const;

//     pub fn  print(variable_annotations = BTreeMap<usize:&BTreeMap<usize, String> String>()) const;
//     pub fn  print_with_assignment(variable_annotations = BTreeMap<usize:&Vec<FieldT> &full_assignment, String>():BTreeMap<usize, String>) const;

//     friend std::ostream& operator<< <FieldT>(std::ostream &out, &lc:linear_combination<FieldT,T>);
//     friend std::istream& operator>> <FieldT>(std::istream &in, linear_combination<FieldT,T> &lc);
// };

//
// linear_combination<FieldT,T> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>);

//
// linear_combination<FieldT,T> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT,T>);

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>);

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT,T>);

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>);

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT,T>);

// use crate::relations::variable;

//#endif // VARIABLE_HPP_

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
//#ifndef VARIABLE_TCC_
// #define VARIABLE_TCC_

// use  <algorithm>
// use  <cassert>
use ffec::algebra::field_utils::bigint::bigint;

//
// linear_term<FieldT> variable<FieldT,T>::operator*(int_coeff:integer_coeff_t) const
// {
//     return linear_term::<FieldT>(*this, int_coeff);
// }

//
// linear_term<FieldT> variable<FieldT,T>::operator*(&field_coeff:FieldT) const
// {
//     return linear_term::<FieldT>(*this, field_coeff);
// }

//
// linear_combination<FieldT,T> variable<FieldT,T>::operator+(&other:linear_combination<FieldT,T>) const
// {
//     linear_combination<FieldT,T> result;

//     result.add_term(*this);
//     result.terms.insert(result.terms.begin(), other.terms.begin(), other.terms.end());

//     return result;
// }
impl<FieldT: FieldTConfig, SLC: SubLinearCombinationConfig, SV: SubVariableConfig>
    Add<linear_combination<FieldT, SV, SLC>> for variable<FieldT, SV>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn add(self, rhs: linear_combination<FieldT, SV, SLC>) -> Self::Output {
        rhs
    }
}
//
// linear_combination<FieldT,T> variable<FieldT,T>::operator-(&other:linear_combination<FieldT,T>) const
// {
//     return (*this) + (-other);
// }

//
// linear_term<FieldT> variable<FieldT,T>::operator-() const
// {
//     return linear_term::<FieldT>(*this, -FieldT::one());
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig> PartialEq for variable<FieldT, SV> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

//
// bool variable<FieldT,T>::operator==(&other:variable<FieldT,T>) const
// {
//     return (self.index == other.index);
// }

//
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &var:variable<FieldT,T>)
// {
//     return linear_term::<FieldT>(var, int_coeff);
// }

//
// linear_term<FieldT> operator*(field_coeff:&FieldT &var:variable<FieldT,T>)
// {
//     return linear_term::<FieldT>(var, field_coeff);
// }

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &var:variable<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(int_coeff) + var;
// }
// impl<FieldT: FieldTConfig,SV: SubVariableConfig, SLC:SubLinearCombinationConfig> Add<linear_combination<FieldT, SLC>> for variable<FieldT, SV> {
//     type Output = linear_combination<FieldT, SLC>;

//     fn add(self, rhs: linear_combination<FieldT, SLC>) -> Self::Output {
//         self + rhs
//     }
// }

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &var:variable<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(field_coeff) + var;
// }

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &var:variable<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(int_coeff) - var;
// }

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &var:variable<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(field_coeff) - var;
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig> linear_term<FieldT, SV> {
    pub fn new_with_int_coeff(var: variable<FieldT, SV>, int_coeff: integer_coeff_t) -> Self {
        Self {
            index: var,
            coeff: FieldT::from(int_coeff),
        }
    }

    pub fn new_with_field(var: variable<FieldT, SV>, coeff: FieldT) -> Self {
        Self { index: var, coeff }
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<variable<FieldT, SV>>
    for linear_term<FieldT, SV>
{
    fn from(rhs: variable<FieldT, SV>) -> Self {
        linear_term::<FieldT, SV> {
            index: rhs,
            coeff: FieldT::one(),
        }
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<integer_coeff_t>
    for linear_term<FieldT, SV>
{
    fn from(rhs: integer_coeff_t) -> Self {
        linear_term::<FieldT, SV> {
            index: variable::<FieldT, SV>::default(),
            coeff: FieldT::from(rhs),
        }
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<FieldT> for linear_term<FieldT, SV> {
    fn from(rhs: FieldT) -> Self {
        linear_term::<FieldT, SV> {
            index: variable::<FieldT, SV>::default(),
            coeff: rhs,
        }
    }
}

//
// linear_term<FieldT> linear_term<FieldT>::operator*(int_coeff:integer_coeff_t) const
// {
//     return (self.operator*(FieldT(int_coeff)));
// }

//
// linear_term<FieldT> linear_term<FieldT>::operator*(&field_coeff:FieldT) const
// {
//     return linear_term::<FieldT>(self.index, field_coeff * self.coeff);
// }
impl<FieldT: FieldTConfig, SV: SubVariableConfig> Mul<FieldT> for linear_term<FieldT, SV> {
    type Output = linear_term<FieldT, SV>;

    fn mul(self, rhs: FieldT) -> Self::Output {
        linear_term::<FieldT, SV>::new_with_field(
            variable::<FieldT, SV>::from(self.index),
            rhs * self.coeff.clone(),
        )
    }
}

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT,T>(int_coeff) + lt;
// }
// impl<FieldT: FieldTConfig,SV:SubVariableConfig,SLC:SubLinearCombinationConfig> Add<integer_coeff_t> for linear_term<FieldT,SV> {
//     type Output = linear_combination<FieldT,SV, SLC>;

//     fn add(self, rhs: integer_coeff_t) -> Self::Output {
//         linear_combination::<FieldT,SV,SLC>::from(rhs) + self
//     }
// }

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT,T>(field_coeff) + lt;
// }

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT,T>(int_coeff) - lt;
// }
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Sub<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn sub(self, rhs: integer_coeff_t) -> Self::Output {
        self - linear_combination::<FieldT, SV, SLC>::from(rhs)
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Sub
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Neg
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * (-FieldT::one())
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Mul<FieldT>
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn mul(self, rhs: FieldT) -> Self::Output {
        self
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Mul<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn mul(self, rhs: integer_coeff_t) -> Self::Output {
        self
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Mul
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self
    }
}

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_combination<FieldT,T>(field_coeff) - lt;
// }

//
// linear_combination<FieldT,T> linear_term<FieldT>::operator+(&other:linear_combination<FieldT,T>) const
// {
//     return linear_combination<FieldT,T>(*this) + other;
// }
// impl<FieldT: FieldTConfig,SV:SubVariableConfig,SLC:SubLinearCombinationConfig> Add<linear_combination<FieldT,SV, SLC>> for linear_term<FieldT,SV> {
//     type Output = linear_combination<FieldT,SV, SLC>;

//     fn add(self, rhs: linear_combination<FieldT,SV, SLC>) -> Self::Output {
//         linear_combination::<FieldT,SV, SLC>::from(self) + rhs
//     }
// }
// impl<FieldT: FieldTConfig,SV:SubVariableConfig,SLC:SubLinearCombinationConfig> Add for linear_term<FieldT,SV> {
//     type Output = linear_combination<FieldT,SV, SLC>;

//     fn add(self, rhs: Self) -> Self::Output {
//         linear_combination::<FieldT,SV, SLC>::from(self) + linear_combination::<FieldT, SLC>::new4(rhs)
//     }
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Add<linear_term<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn add(self, rhs: linear_term<FieldT, SV>) -> Self::Output {
        linear_combination::<FieldT, SV, SLC>::from(rhs) + self
    }
}

//
// linear_combination<FieldT,T> linear_term<FieldT>::operator-(&other:linear_combination<FieldT,T>) const
// {
//     return (*this) + (-other);
// }

//
// linear_term<FieldT> linear_term<FieldT>::operator-() const
// {
//     return linear_term::<FieldT>(self.index, -self.coeff);
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig> PartialEq for linear_term<FieldT, SV> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.coeff == other.coeff
    }
}

//
// bool linear_term<FieldT>::operator==(&other:linear_term<FieldT>) const
// {
//     return (self.index == other.index &&
//             self.coeff == other.coeff);
// }

//
// linear_term<FieldT> operator*(int_coeff:integer_coeff_t &lt:linear_term<FieldT>)
// {
//     return FieldT(int_coeff) * lt;
// }

//
// linear_term<FieldT> operator*(field_coeff:&FieldT &lt:linear_term<FieldT>)
// {
//     return linear_term::<FieldT>(lt.index, field_coeff * lt.coeff);
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    From<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
{
    fn from(rhs: integer_coeff_t) -> Self {
        linear_combination::<FieldT, SV, SLC>::from(linear_term::<FieldT, SV>::from(rhs))
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> From<FieldT>
    for linear_combination<FieldT, SV, SLC>
{
    fn from(rhs: FieldT) -> Self {
        linear_combination::<FieldT, SV, SLC>::from(linear_term::<FieldT, SV>::from(rhs))
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    From<variable<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    fn from(rhs: variable<FieldT, SV>) -> Self {
        linear_combination::<FieldT, SV, SLC>::from(linear_term::<FieldT, SV>::from(rhs))
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    From<linear_term<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    fn from(rhs: linear_term<FieldT, SV>) -> Self {
        linear_combination::<FieldT, SV, SLC> {
            terms: vec![rhs],
            t: SLC::default(),
        }
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    linear_combination<FieldT, SV, SLC>
{
    // Vec<linear_term<FieldT> >::const_iterator linear_combination<FieldT,T>::begin() const
    // {
    //     return terms.begin();
    // }

    // Vec<linear_term<FieldT> >::const_iterator linear_combination<FieldT,T>::end() const
    // {
    //     return terms.end();
    // }
    pub fn add_term_with_index(&mut self, var: usize) {
        self.terms.push(linear_term::<FieldT, SV>::new_with_field(
            variable::<FieldT, SV>::from(var),
            FieldT::one(),
        ));
    }
    pub fn add_term_with_variable(&mut self, var: variable<FieldT, SV>) {
        self.terms.push(linear_term::<FieldT, SV>::new_with_field(
            var,
            FieldT::one(),
        ));
    }
    pub fn add_term(&mut self, var: usize, int_coeff: integer_coeff_t) {
        self.terms
            .push(linear_term::<FieldT, SV>::new_with_int_coeff(
                variable::<FieldT, SV>::from(var),
                int_coeff,
            ));
    }

    pub fn add_term_with_field(&mut self, var: usize, coeff: FieldT) {
        self.terms.push(linear_term::<FieldT, SV>::new_with_field(
            variable::<FieldT, SV>::from(var),
            coeff,
        ));
    }

    // linear_combination<FieldT,T> linear_combination<FieldT,T>::operator*(int_coeff:integer_coeff_t) const
    // {
    //     return (*this) * FieldT(int_coeff);
    // }

    pub fn evaluate(&self, assignment: &Vec<FieldT>) -> FieldT {
        let mut acc = FieldT::zero();
        for lt in &self.terms {
            acc += if lt.index.index == 0 {
                FieldT::one()
            } else {
                assignment[lt.index.index - 1].clone()
            } * lt.coeff.clone();
        }
        return acc;
    }

    pub fn is_valid(&self, num_variables: usize) -> bool {
        /* check that all terms in linear combination are sorted */
        for i in 1..self.terms.len() {
            if self.terms[i - 1].index.index >= self.terms[i].index.index {
                return false;
            }
        }

        /* check that the variables are in proper range. as the variables
        are sorted, it suffices to check the last term */
        self.terms.last().unwrap().index.index < num_variables
    }

    pub fn print(&self, variable_annotations: &BTreeMap<usize, String>) {
        for lt in &self.terms {
            if lt.index.index == 0 {
                print!("    1 * ");
                lt.coeff.print();
            } else {
                print!(
                    "    x_{} ({}) * ",
                    lt.index.index,
                    (if let Some(it) = variable_annotations.get(&lt.index.index) {
                        it
                    } else {
                        "no annotation"
                    })
                );
                lt.coeff.print();
            }
        }
    }

    pub fn print_with_assignment(
        &self,
        full_assignment: &Vec<FieldT>,
        variable_annotations: &BTreeMap<usize, String>,
    ) {
        for lt in &self.terms {
            if lt.index.index == 0 {
                print!("    1 * ");
                lt.coeff.print();
            } else {
                print!("    x_{} * ", lt.index.index);
                lt.coeff.print();

                print!(
                    "    where x_{} ({}) was assigned value ",
                    lt.index.index,
                    (if let Some(it) = variable_annotations.get(&lt.index.index) {
                        it
                    } else {
                        "no annotation"
                    })
                );
                full_assignment[lt.index.index - 1].print();
                print!("      i.e. negative of ");
                (-full_assignment[lt.index.index - 1].clone())
                    .clone()
                    .print();
            }
        }
    }
}
use std::fmt;
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> fmt::Display
    for linear_combination<FieldT, SV, SLC>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.terms.len(),
            self.terms
                .iter()
                .map(|lt| format!("{}\n{}{OUTPUT_NEWLINE}", lt.index.index, lt.coeff))
                .collect::<String>(),
        )
    }
}

//
// std::ostream& operator<<(std::ostream &out, &lc:linear_combination<FieldT,T>)
// {
//     out << lc.terms.len() << "\n";
//     for lt in &lc.terms
//     {
//         out << lt.index << "\n";
//         out << lt.coeff << OUTPUT_NEWLINE;
//     }

//     return out;
// }

//
// std::istream& operator>>(std::istream &in, linear_combination<FieldT,T> &lc)
// {
//     lc.terms.clear();

//     usize s;
//     in >> s;

//     ffec::consume_newline(in);

//     lc.terms.reserve(s);

//     for i in 0..s
//     {
//         linear_term<FieldT> lt;
//         in >> lt.index;
//         ffec::consume_newline(in);
//         in >> lt.coeff;
//         ffec::consume_OUTPUT_NEWLINE(in);
//         lc.terms.push(lt);
//     }

//     return in;
// }

//
// linear_combination<FieldT,T> operator*(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>)
// {
//     return lc * int_coeff;
// }

//
// linear_combination<FieldT,T> operator*(field_coeff:&FieldT &lc:linear_combination<FieldT,T>)
// {
//     return lc * field_coeff;
// }

//
// linear_combination<FieldT,T> operator+(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(int_coeff) + lc;
// }

//
// linear_combination<FieldT,T> operator+(field_coeff:&FieldT &lc:linear_combination<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(field_coeff) + lc;
// }

//
// linear_combination<FieldT,T> operator-(int_coeff:integer_coeff_t &lc:linear_combination<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(int_coeff) - lc;
// }

//
// linear_combination<FieldT,T> operator-(field_coeff:&FieldT &lc:linear_combination<FieldT,T>)
// {
//     return linear_combination<FieldT,T>(field_coeff) - lc;
// }
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    linear_combination<FieldT, SV, SLC>
{
    pub fn new(all_terms: Vec<linear_term<FieldT, SV>>) -> Self {
        if all_terms.is_empty() {
            return Self {
                terms: vec![],
                t: SLC::default(),
            };
        }

        let mut terms = all_terms;
        terms.sort_unstable_by_key(|v| v.index.index);

        // auto result_it = terms.begin();
        let mut j = 0;
        for i in 1..terms.len() {
            if terms[i].index == terms[j].index {
                let c = terms[i].coeff.clone();
                terms[j].coeff += c;
            } else {
                j += 1;
                terms[j] = terms[i].clone();
            }
        }
        terms.resize(
            j + 1,
            linear_term::<FieldT, SV>::new_with_field(
                variable::<FieldT, SV>::default(),
                FieldT::zero(),
            ),
        );
        Self {
            terms,
            t: SLC::default(),
        }
    }
}

//#endif // VARIABLE_TCC

//
// linear_combination<FieldT,T> linear_combination<FieldT,T>::operator*(&field_coeff:FieldT) const
// {
//     linear_combination<FieldT,T> result;
//     result.terms.reserve(self.terms.len());
//     for lt in &self.terms
//     {
//         result.terms.push(lt * field_coeff);
//     }
//     return result;
// }

//
// linear_combination<FieldT,T> linear_combination<FieldT,T>::operator+(&other:linear_combination<FieldT,T>) const
// {
//     linear_combination<FieldT,T> result;
//     result.terms.reserve(self.terms.len() + other.terms.len());

//     auto it1 = self.terms.begin();
//     auto it2 = other.terms.begin();

//     /* invariant: it1 and it2 always point to unprocessed items in the corresponding linear combinations */
//     while (it1 != self.terms.end() && it2 != other.terms.end())
//     {
//         if it1->index < it2->index
//         {
//             result.terms.push(*it1);
//             it1+=1;
//         }
//         else if it1->index > it2->index
//         {
//             result.terms.push(*it2);
//             it2+=1;
//         }
//         else
//         {
//             /* it1->index == it2->index */
//             result.terms.push(variable<FieldT,T>(it1->index), it1->coeff + it2->coeff);
//             it1+=1;
//             it2+=1;
//         }
//     }

//     if it1 != self.terms.end()
//     {
//         result.terms.insert(result.terms.end(), it1, self.terms.end());
//     }
//     else
//     {
//         result.terms.insert(result.terms.end(), it2, other.terms.end());
//     }

//     return result;
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Add
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        rhs
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Add<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn add(self, rhs: integer_coeff_t) -> Self::Output {
        self + linear_combination::<FieldT, SV, SLC>::from(rhs)
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Add<variable<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn add(self, rhs: variable<FieldT, SV>) -> Self::Output {
        self + linear_combination::<FieldT, SV, SLC>::from(rhs)
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Add<FieldT>
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn add(self, rhs: FieldT) -> Self::Output {
        self + linear_combination::<FieldT, SV, SLC>::from(rhs)
    }
}
//
// linear_combination<FieldT,T> linear_combination<FieldT,T>::operator-(&other:linear_combination<FieldT,T>) const
// {
//     return (*this) + (-other);
// }

//
// linear_combination<FieldT,T> linear_combination<FieldT,T>::operator-() const
// {
//     return (*this) * (-FieldT::one());
// }

//
// bool linear_combination<FieldT,T>::operator==(&other:linear_combination<FieldT,T>) const
// {
//     return (self.terms == other.terms);
// }
