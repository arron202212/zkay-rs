// Declaration of interfaces for an accumulation vector.
use crate::common::data_structures::sparse_vector::sparse_vector;
use ffec::FieldTConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::field_utils::BigInteger;
use ffec::scalar_multiplication::multiexp::KCConfig;
use ffec::{One, PpConfig, Zero};

// /**
//  * An accumulation vector comprises an accumulation value and a sparse vector.
//  * The method "accumulate_chunk" allows one to accumlate portions of the sparse
//  * vector into the accumualation value.
//  */
#[derive(Clone, Default)]
pub struct accumulation_vector<T: PpConfig> {
    pub first: T,
    pub rest: sparse_vector<T>,
}
impl<T: PpConfig> From<Vec<T>> for accumulation_vector<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            first: T::zero(),
            rest: sparse_vector::new(v),
        }
    }
}

impl<T: PpConfig> accumulation_vector<T> {
    pub fn new(first: T, rest: sparse_vector<T>) -> Self {
        Self { first, rest }
    }
    pub fn new_with_vec(first: T, v: Vec<T>) -> Self {
        Self {
            first,
            rest: sparse_vector::new(v),
        }
    }
}

//  Implementation of interfaces for an accumulation vector.

impl<T: PpConfig> PartialEq for accumulation_vector<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.rest == other.rest
    }
}

impl<T: PpConfig> accumulation_vector<T> {
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
        let first_size_in_bits = T::size_in_bits();
        let rest_size_in_bits = self.rest.size_in_bits();
        first_size_in_bits + rest_size_in_bits
    }

    pub fn accumulate_chunk<FieldT: FieldTConfig>(&self, it: &[FieldT], offset: usize) -> Self {
        let acc_result = self.rest.accumulate(it, offset);
        let new_first: T = self.first.clone() + acc_result.0;
        Self::new(new_first, acc_result.1)
    }
}

use std::fmt;
impl<T: PpConfig> fmt::Display for accumulation_vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{OUTPUT_NEWLINE}{}{OUTPUT_NEWLINE}",
            self.first, self.rest,
        )
    }
}
