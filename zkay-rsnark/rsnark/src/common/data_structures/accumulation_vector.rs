/** @file
*****************************************************************************

Declaration of interfaces for an accumulation vector.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef ACCUMULATION_VECTOR_HPP_
// // #define ACCUMULATION_VECTOR_HPP_

// use  <iostream>
use crate::common::data_structures::sparse_vector::{SparseVectorConfig, sparse_vector};
use ffec::common::serialization::OUTPUT_NEWLINE;

//
// pub struct accumulation_vector;

/**
 * An accumulation vector comprises an accumulation value and a sparse vector.
 * The method "accumulate_chunk" allows one to accumlate portions of the sparse
 * vector into the accumualation value.
 */
pub trait AccumulationVectorConfig:
    Clone
    + Default
    + std::cmp::PartialEq
    + std::fmt::Display
    + std::ops::Add
    + Sized
    + SparseVectorConfig
{
    // fn zero()->Self;
    // fn size_in_bits()->usize;
}
#[derive(Clone, Default)]
pub struct accumulation_vector<T: AccumulationVectorConfig> {
    pub first: T,
    pub rest: sparse_vector<T>,
}
impl<T: AccumulationVectorConfig> accumulation_vector<T> {
    // accumulation_vector() = default;
    // accumulation_vector(&other:accumulation_vector<T>) = default;
    // accumulation_vector(accumulation_vector<T> &&other) = default;
    pub fn new(first: T, rest: sparse_vector<T>) -> Self {
        Self { first, rest }
    }
    pub fn new2(first: T, v: Vec<T>) -> Self {
        Self {
            first,
            rest: sparse_vector::new(v),
        }
    }
    pub fn new3(v: Vec<T>) -> Self {
        Self {
            first: T::zero(),
            rest: sparse_vector::new(v),
        }
    }

    // accumulation_vector<T>& operator=(&other:accumulation_vector<T>) = default;
    // accumulation_vector<T>& operator=(accumulation_vector<T> &&other) = default;

    // bool operator==(&other:accumulation_vector<T>) const;

    // bool is_fully_accumulated() const;

    // usize domain_size() const;
    // usize size() const;
    //  pub fn size_in_bits(&self)->usize;

    //
    // accumulation_vector<T> accumulate_chunk(it_begin:&Vec<FieldT>::const_iterator
    //                                         it_end:&Vec<FieldT>::const_iterator
    //                                         offset:usize) const;
}

// use crate::common::data_structures::accumulation_vector;

//  Implementation of interfaces for an accumulation vector.

impl<T: AccumulationVectorConfig> PartialEq for accumulation_vector<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.rest == other.rest
    }
}

impl<T: AccumulationVectorConfig> accumulation_vector<T> {
    pub fn is_fully_accumulated(&self) -> bool {
        return self.rest.empty();
    }

    pub fn domain_size(&self) -> usize {
        return self.rest.domain_size();
    }

    pub fn size(&self) -> usize {
        return self.rest.domain_size();
    }

    pub fn size_in_bits(&self) -> usize {
        let first_size_in_bits = T::size_in_bits();
        let rest_size_in_bits = self.rest.size_in_bits();
        first_size_in_bits + rest_size_in_bits
    }

    pub fn accumulate_chunk<
        FieldT: std::ops::Mul<Output = FieldT> + Clone + ffec::scalar_multiplication::multiexp::AsBigint,
    >(
        &self,
        it: &[FieldT],
        offset: usize,
    ) -> Self {
        let acc_result = self.rest.accumulate::<FieldT>(it, offset);
        let new_first: T = self.first.clone() + acc_result.0;
        Self::new(new_first, acc_result.1)
    }
}

use std::fmt;
impl<T: AccumulationVectorConfig> fmt::Display for accumulation_vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.first, self.rest,
        )
    }
}

//
// std::istream& operator>>(std::istream& in, accumulation_vector<T> &v)
// {
//     in >> v.first;
//     ffec::consume_OUTPUT_NEWLINE(in);
//     in >> v.rest;
//     ffec::consume_OUTPUT_NEWLINE(in);

//     return in;
// }
