// Declaration of interfaces for:
// - a variable (i.e., x_i),
// - a linear term (i.e., a_i * x_i), and
// - a linear combination (i.e., sum_i a_i * x_i).

use ffec::FieldTConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg, Sub};
// /**
//  * Mnemonic typedefs.
//  */
pub type var_index_t = usize;
pub type integer_coeff_t = i64;


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

// /**
//  *  A variable represents a formal expression of the form "x_{index}".
//  */
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

// /**
//  * A linear term represents a formal expression of the form "coeff * x_{index}".
//  */
#[derive(Clone)]
pub struct linear_term<FieldT: FieldTConfig, SV: SubVariableConfig> {
    pub index: variable<FieldT, SV>,
    pub coeff: FieldT,
}

// /**
//  * A linear combination represents a formal expression of the form "sum_i coeff_i * x_{index_i}".
//  */
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

use ffec::algebra::field_utils::bigint::bigint;

impl<FieldT: FieldTConfig, SLC: SubLinearCombinationConfig, SV: SubVariableConfig>
    Add<linear_combination<FieldT, SV, SLC>> for variable<FieldT, SV>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn add(self, rhs: linear_combination<FieldT, SV, SLC>) -> Self::Output {
        rhs
    }
}

impl<FieldT: FieldTConfig, SLC: SubLinearCombinationConfig, SV: SubVariableConfig>
    Sub<linear_combination<FieldT, SV, SLC>> for variable<FieldT, SV>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn sub(self, rhs: linear_combination<FieldT, SV, SLC>) -> Self::Output {
        rhs
    }
}



impl<FieldT: FieldTConfig, SV: SubVariableConfig> PartialEq for variable<FieldT, SV> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

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

// impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<integer_coeff_t>
//     for linear_term<FieldT, SV>
// {
//     fn from(rhs: integer_coeff_t) -> Self {
//         linear_term::<FieldT, SV> {
//             index: variable::<FieldT, SV>::default(),
//             coeff: FieldT::from(rhs),
//         }
//     }
// }

impl<FieldT: FieldTConfig, SV: SubVariableConfig> From<FieldT> for linear_term<FieldT, SV> {
    fn from(rhs: FieldT) -> Self {
        linear_term::<FieldT, SV> {
            index: variable::<FieldT, SV>::default(),
            coeff: rhs,
        }
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> Mul<FieldT> for linear_term<FieldT, SV> {
    type Output = linear_term<FieldT, SV>;

    fn mul(self, rhs: FieldT) -> Self::Output {
        linear_term::<FieldT, SV>::new_with_field(
            variable::<FieldT, SV>::from(self.index),
            rhs * self.coeff.clone(),
        )
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Sub<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn sub(self, rhs: integer_coeff_t) -> Self::Output {
        self - linear_combination::<FieldT, SV, SLC>::from(FieldT::from(rhs))
    }
}
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Sub<variable<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn sub(self, rhs: variable<FieldT, SV>) -> Self::Output {
        self - linear_combination::<FieldT, SV, SLC>::from(rhs)
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Sub<linear_term<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn sub(self, rhs: linear_term<FieldT, SV>) -> Self::Output {
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
// impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
//     Mul<integer_coeff_t> for linear_combination<FieldT, SV, SLC>
// {
//     type Output = Self;

//     fn mul(self, rhs: integer_coeff_t) -> Self::Output {
//         self
//     }
// }
impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Mul<variable<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn mul(self, rhs: variable<FieldT, SV>) -> Self::Output {
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


impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig>
    Add<linear_term<FieldT, SV>> for linear_combination<FieldT, SV, SLC>
{
    type Output = linear_combination<FieldT, SV, SLC>;

    fn add(self, rhs: linear_term<FieldT, SV>) -> Self::Output {
        linear_combination::<FieldT, SV, SLC>::from(rhs) + self
    }
}

impl<FieldT: FieldTConfig, SV: SubVariableConfig> PartialEq for linear_term<FieldT, SV> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.coeff == other.coeff
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
        //check that all terms in linear combination are sorted
        for i in 1..self.terms.len() {
            if self.terms[i - 1].index.index >= self.terms[i].index.index {
                return false;
            }
        }

        // /* check that the variables are in proper range. as the variables
        // are sorted, it suffices to check the last term */
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


impl<FieldT: FieldTConfig, SV: SubVariableConfig, SLC: SubLinearCombinationConfig> Add
    for linear_combination<FieldT, SV, SLC>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        rhs
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