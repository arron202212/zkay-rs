use crate::common::data_structures::sparse_vector::sparse_vector;
/** @file
*****************************************************************************

Declaration of interfaces for:
- a knowledge commitment, and
- a knowledge commitment vector.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef KNOWLEDGE_COMMITMENT_HPP_
// // #define KNOWLEDGE_COMMITMENT_HPP_
use ffec::algebra::fields::prime_base::fp;
use ffec::common::serialization::{OUTPUT_NEWLINE, OUTPUT_SEPARATOR};
use ffec::field_utils::bigint::bigint;
use std::ops::{Add, Mul};

use ffec::One;
use ffec::field_utils::BigInteger;
use ffec::scalar_multiplication::multiexp::AsBigint;
use ffec::{Fp_model, Fp_modelConfig};
pub trait TConfig<const N: usize>:
    BigInteger
    + One
    + AsBigint
    + std::ops::Add<Output = Self>
    + for<'a> std::ops::Mul<&'a bigint<{ N }>, Output = Self>
{
    fn zero() -> Self;
    fn mixed_add(&self, other: &Self) -> Self;
    fn is_special(&self) -> bool;
    fn print(&self);
    fn size_in_bits() -> usize;
}
/********************** Knowledge commitment *********************************/

/**
 * A knowledge commitment is a pair (g,h) where g is in T1 and h in T2,
 * and T1 and T2 are groups (written additively).
 *
 * Such pairs form a group by defining:
 * - "zero" = (0,0)
 * - "one" = (1,1)
 * - a * (g,h) + b * (g',h')->Self= ( a * g + b * g', a * h + b * h').
 */

pub struct knowledge_commitment<const N: usize, T1: TConfig<N>, T2: TConfig<N>> {
    g: T1,
    h: T2,
}
// impl<const N:usize,T1:TConfig<N>,T2:TConfig<N>> knowledge_commitment<T1,T2>{
//     // knowledge_commitment<T1,T2>() = default;
//     // knowledge_commitment<T1,T2>(&other:knowledge_commitment<T1,T2>) = default;
//     // knowledge_commitment<T1,T2>(knowledge_commitment<T1,T2> &&other) = default;
//     pub fn new(g:T1, h:T2)->Self{
//         Self{g,h}
//     }

// knowledge_commitment<T1,T2>& operator=(&other:knowledge_commitment<T1,T2>) = default;
// knowledge_commitment<T1,T2>& operator=(knowledge_commitment<T1,T2> &&other) = default;
// knowledge_commitment<T1,T2> operator+(&other:knowledge_commitment<T1, T2>) const;
// knowledge_commitment<T1,T2> mixed_add(&other:knowledge_commitment<T1, T2>) const;
// knowledge_commitment<T1,T2> dbl() const;

//     pub fn to_special(&self){
//     }
//      pub fn is_special() ->bool{
// }

//      pub fn is_zero() ->bool{
//         }
// bool operator==(&other:knowledge_commitment<T1,T2>) const;
// bool operator!=(&other:knowledge_commitment<T1,T2>) const;

// static knowledge_commitment<T1,T2> zero();
// static knowledge_commitment<T1,T2> one();

// pub fn  print() const;

// static usize size_in_bits();

// static pub fn  batch_to_special_all_non_zeros(
//     Vec<knowledge_commitment<T1,T2> > &vec);
// }

//
// knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&ffec::bigint<m>T2>);

//
// knowledge_commitment<T1,T2> operator*(lhs, &rhs:knowledge_commitment<T1:&ffec::Fp_model<m, modulus_p>T2>);

//
// std::ostream& operator<<(std::ostream& out, &kc:knowledge_commitment<T1,T2>);

//
// std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc);

/******************** Knowledge commitment vector ****************************/

/**
 * A knowledge commitment vector is a sparse vector of knowledge commitments.
 */
//
type knowledge_commitment_vector<const N: usize, T1, T2> =
    sparse_vector<knowledge_commitment<N, T1, T2>>;

// use crate::knowledge_commitment::knowledge_commitment;

// //#endif // KNOWLEDGE_COMMITMENT_HPP_

/** @file
*****************************************************************************

Implementation of interfaces for:
- a knowledge commitment, and
- a knowledge commitment vector.

See knowledge_commitment.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// //#ifndef KNOWLEDGE_COMMITMENT_TCC_
// // #define KNOWLEDGE_COMMITMENT_TCC_

impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>> knowledge_commitment<N, T1, T2> {
    pub fn new(g: T1, h: T2) -> Self {
        Self { g, h }
    }

    pub fn zero() -> Self {
        return Self::new(T1::zero(), T2::zero());
    }

    pub fn one() -> Self {
        return Self::new(T1::one(), T2::one());
    }

    pub fn mixed_add(&self, other: &knowledge_commitment<N, T1, T2>) -> Self {
        return Self::new(self.g.mixed_add(&other.g), self.h.mixed_add(&other.h));
    }

    pub fn dbl(&self) -> Self {
        return Self::new(self.g.dbl(), self.h.dbl());
    }

    pub fn to_special(&self) {
        self.g.to_special();
        self.h.to_special();
    }

    pub fn is_special(&self) -> bool {
        return self.g.is_special() && self.h.is_special();
    }

    pub fn is_zero(&self) -> bool {
        return (self.g.is_zero() && self.h.is_zero());
    }

    pub fn print(&self) {
        print!("knowledge_commitment.g:\n");
        self.g.print();
        print!("knowledge_commitment.h:\n");
        self.h.print();
    }

    pub fn size_in_bits(&self) -> usize {
        return T1::size_in_bits() + T2::size_in_bits();
    }

    pub fn batch_to_special_all_non_zeros(vec: &mut Vec<Self>) {
        // it is guaranteed that every vec[i] is non-zero,
        // but, for any i, *one* of vec[i].g and vec[i].h might still be zero,
        // so we still have to handle zeros separately

        // we separately process g's first, then h's
        // to lower memory consumption
        let mut g_vec = Vec::with_capacity(vec.len());

        for i in 0..vec.len() {
            if !vec[i].g.is_zero() {
                g_vec.push(vec[i].g);
            }
        }

        T1::batch_to_special_all_non_zeros(g_vec.clone());
        let mut g_it = g_vec.iter();
        let mut T1_zero_special = T1::zero();
        T1_zero_special.to_special();

        for i in 0..vec.len() {
            if !vec[i].g.is_zero() {
                vec[i].g = *g_it.next().unwrap();
            } else {
                vec[i].g = T1_zero_special;
            }
        }

        g_vec.clear();

        // exactly the same thing, but for h:
        let mut h_vec = Vec::with_capacity(vec.len());

        for i in 0..vec.len() {
            if !vec[i].h.is_zero() {
                h_vec.push(vec[i].h);
            }
        }

        T2::batch_to_special_all_non_zeros(h_vec.clone());
        let mut h_it = h_vec.iter();
        let mut T2_zero_special = T2::zero();
        T2_zero_special.to_special();

        for i in 0..vec.len() {
            if !vec[i].h.is_zero() {
                vec[i].h = *h_it.next().unwrap();
            } else {
                vec[i].h = T2_zero_special;
            }
        }

        h_vec.clear();
    }
}

impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>> PartialEq for knowledge_commitment<N, T1, T2> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.g == other.g && self.h == other.h
    }
}

use std::fmt;
impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>> fmt::Display
    for knowledge_commitment<N, T1, T2>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{OUTPUT_SEPARATOR}{}", self.g, self.h)
    }
}

//
// std::istream& operator>>(std::istream& in, knowledge_commitment<T1,T2> &kc)
// {
//     in >> kc.g;
//     ffec::consume_OUTPUT_SEPARATOR(in);
//     in >> kc.h;
//     return in;
// }

// bool pub fn operator!=(&other:knowledge_commitment<T1,T2>) const
// {
//     return !((*this) == other);
// }

impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>> Add for knowledge_commitment<N, T1, T2> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.g + other.g, self.h + other.h)
    }
}

impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>> Mul<&bigint<N>>
    for knowledge_commitment<N, T1, T2>
{
    type Output = Self;

    fn mul(self, rhs: &bigint<N>) -> Self {
        Self::new(self.g * rhs, self.h * rhs)
    }
}

impl<const N: usize, T1: TConfig<N>, T2: TConfig<N>, T: Fp_modelConfig<N>> Mul<&Fp_model<N, T>>
    for knowledge_commitment<N, T1, T2>
{
    type Output = Self;

    fn mul(self, rhs: &Fp_model<N, T>) -> Self {
        self * &rhs.as_bigint()
    }
}
