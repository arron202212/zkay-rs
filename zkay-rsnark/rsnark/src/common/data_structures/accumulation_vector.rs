// Declaration of interfaces for an accumulation vector.
use crate::common::data_structures::sparse_vector::sparse_vector;
use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::{One, PpConfig, Zero};

/**
 * An accumulation vector comprises an accumulation value and a sparse vector.
 * The method "accumulate_chunk" allows one to accumlate portions of the sparse
 * vector into the accumualation value.
 */
// pub trait AccumulationVectorConfig:
//     Clone
//     + Default
//     + std::cmp::PartialEq
//     + std::fmt::Display
//     + std::ops::Add
//     + Sized
// {
//     // fn zero()->Self;
//     // fn size_in_bits()->usize;
// }
#[derive(Clone, Default)]
pub struct accumulation_vector<KC: KCConfig> {
    pub first: KC::T,
    pub rest: sparse_vector<KC>,
}
impl<KC: KCConfig> From<Vec<KC::T>> for accumulation_vector<KC> {
    fn from(v: Vec<KC::T>) -> Self {
        Self {
            first: KC::T::zero(),
            rest: sparse_vector::new(v),
        }
    }
}

impl<KC: KCConfig> accumulation_vector<KC> {
    // accumulation_vector() = default;
    // accumulation_vector(&other:accumulation_vector<T>) = default;
    // accumulation_vector(accumulation_vector<T> &&other) = default;
    pub fn new(first: KC::T, rest: sparse_vector<KC>) -> Self {
        Self { first, rest }
    }
    pub fn new_with_vec(first: KC::T, v: Vec<KC::T>) -> Self {
        Self {
            first,
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

impl<KC: KCConfig> PartialEq for accumulation_vector<KC> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.rest == other.rest
    }
}

impl<KC: KCConfig> accumulation_vector<KC> {
    pub fn is_fully_accumulated(&self) -> bool {
        self.rest.empty()
    }

    pub fn domain_size(&self) -> usize {
        self.rest.domain_size()
    }

    pub fn len(&self) -> usize {
        self.rest.domain_size()
    }

    pub fn size_in_bits(&self) -> usize {
        let first_size_in_bits = KC::T::size_in_bits();
        let rest_size_in_bits = self.rest.size_in_bits();
        first_size_in_bits + rest_size_in_bits
    }

    pub fn accumulate_chunk(&self, it: &[KC::FieldT], offset: usize) -> Self {
        let acc_result = self.rest.accumulate::<KC::FieldT>(it, offset);
        let new_first: KC::T = self.first.clone() + acc_result.0;
        Self::new(new_first, acc_result.1)
    }
}

use std::fmt;
impl<KC: KCConfig> fmt::Display for accumulation_vector<KC> {
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
