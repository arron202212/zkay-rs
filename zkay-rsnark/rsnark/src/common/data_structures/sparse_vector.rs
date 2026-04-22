// Declaration of interfaces for a sparse vector.

use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::field_utils::BigInteger;
use ffec::scalar_multiplication::multiexp::{KCConfig, multi_exp, multi_exp_method};
use ffec::{FieldTConfig, PpConfig, Zero};

// /**
//  * A sparse vector is a list of indices along with corresponding values.
//  * The indices are selected from the set {0,1,...,domain_size-1}.
//  */
#[derive(Default, Clone)]
pub struct sparse_vector<T: PpConfig> {
    pub indices: Vec<usize>,
    pub values: Vec<T>,
    pub domain_size_: usize,
}

impl<T: PpConfig> sparse_vector<T> {
    pub fn new(v: Vec<T>) -> Self {
        let domain_size_ = v.len();
        Self {
            values: v,
            domain_size_,
            indices: (0..domain_size_).collect(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.values.len() == self.indices.len() && self.values.len() <= self.domain_size_ {
            return false;
        }

        for i in 0..self.indices.len() {
            if self.indices[i] >= self.indices[i + 1] {
                return false;
            }
        }

        if !self.indices.is_empty() && self.indices[self.indices.len() - 1] >= self.domain_size_ {
            return false;
        }

        true
    }

    pub fn empty(&self) -> bool {
        self.indices.is_empty()
    }

    pub fn domain_size(&self) -> usize {
        self.domain_size_
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn size_in_bits(&self) -> usize {
        self.indices.len() * (std::mem::size_of::<usize>() * 8 + T::size_in_bits())
    }

    pub fn accumulate<FieldT: FieldTConfig>(
        &self,
        it: &[FieldT],
        offset: usize,
    ) -> (T, sparse_vector<T>) {
        let mut chunks = 1;

        let mut accumulated_value = T::zero();
        let mut resulting_vector = sparse_vector::<T>::default();
        resulting_vector.domain_size_ = self.domain_size_;

        let mut range_len = it.len();
        let mut in_block = false;
        let mut first_pos = -1;
        let mut last_pos = -1;

        for i in 0..self.indices.len() {
            let matching_pos = (offset <= self.indices[i] && self.indices[i] < offset + range_len);
            let mut copy_over;

            if in_block {
                if matching_pos && last_pos == i as isize - 1 {
                    // block can be extended, do it
                    last_pos = i as isize;
                    copy_over = false;
                } else {
                    // block has ended here
                    in_block = false;
                    copy_over = true;

                    accumulated_value = accumulated_value
                        + multi_exp::<T, FieldT, { multi_exp_method::multi_exp_method_bos_coster }>(
                            &self.values[first_pos as usize..last_pos as usize + 1],
                            &it[(self.indices[first_pos as usize] - offset)
                                ..(self.indices[last_pos as usize] - offset) + 1],
                            chunks,
                        );
                }
            } else {
                if matching_pos {
                    // block can be started
                    first_pos = i as isize;
                    last_pos = i as isize;
                    in_block = true;
                    copy_over = false;
                } else {
                    copy_over = true;
                }
            }

            if copy_over {
                resulting_vector.indices.push(self.indices[i]);
                resulting_vector.values.push(self.values[i].clone());
            }
        }

        if in_block {
            accumulated_value = accumulated_value
                + multi_exp::<T, FieldT, { multi_exp_method::multi_exp_method_bos_coster }>(
                    &self.values[first_pos as usize..last_pos as usize + 1],
                    &it[(self.indices[first_pos as usize] - offset)
                        ..(self.indices[last_pos as usize] - offset) + 1],
                    chunks,
                );
        }

        (accumulated_value, resulting_vector)
    }
}

use std::ops::Index;
impl<T: PpConfig> Index<usize> for sparse_vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        let it = self.indices.partition_point(|&v| v < idx);
        if it != self.indices.len() && it == idx {
            &self.values[it]
        } else {
            &self.values[0]
        }
    }
}

impl<T: PpConfig> PartialEq for sparse_vector<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.domain_size_ != other.domain_size_ {
            return false;
        }

        let (mut this_pos, mut other_pos) = (0, 0);
        while this_pos < self.indices.len() && other_pos < other.indices.len() {
            if self.indices[this_pos] == other.indices[other_pos] {
                if self.values[this_pos] != other.values[other_pos] {
                    return false;
                }
                this_pos += 1;
                other_pos += 1;
            } else if self.indices[this_pos] < other.indices[other_pos] {
                if !self.values[this_pos].is_zero() {
                    return false;
                }
                this_pos += 1;
            } else {
                if !other.values[other_pos].is_zero() {
                    return false;
                }
                other_pos += 1;
            }
        }

        //at least one of the vectors has been exhausted, so other must be empty
        while this_pos < self.indices.len() {
            if !self.values[this_pos].is_zero() {
                return false;
            }
            this_pos += 1;
        }

        while other_pos < other.indices.len() {
            if !other.values[other_pos].is_zero() {
                return false;
            }
            other_pos += 1;
        }

        true
    }
}

impl<T: PpConfig> PartialEq<Vec<T>> for sparse_vector<T> {
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.domain_size_ < other.len() {
            return false;
        }

        let mut j = 0;
        for i in 0..other.len() {
            if self.indices[j] == i {
                if self.values[j] != other[j] {
                    return false;
                }
                j += 1;
            } else {
                if !other[j].is_zero() {
                    return false;
                }
            }
        }

        true
    }
}

use std::fmt;

impl<T: PpConfig> fmt::Display for sparse_vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}{}\n{}",
            self.domain_size_,
            self.indices.len(),
            self.indices
                .iter()
                .map(|i| format!("{i}\n"))
                .collect::<String>(),
            self.values.len(),
            self.values
                .iter()
                .map(|i| format!("{i}{OUTPUT_NEWLINE}"))
                .collect::<String>(),
        )
    }
}
