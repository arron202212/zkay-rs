// Declaration of interfaces for a sparse vector.

use ffec::PpConfig;
use ffec::common::serialization::OUTPUT_NEWLINE;
use ffec::scalar_multiplication::multiexp::{multi_exp, multi_exp_method};
// pub trait SparseVectorConfig:
//     Default
//     + std::fmt::Display
//     + std::cmp::PartialEq
//     + std::ops::Add<Output = Self>
//     + ffec::Zero
//     + Clone
//     + std::ops::Sub<Output = Self>
//     + ffec::scalar_multiplication::wnaf::Config
// {
//     fn size_in_bits() -> usize;
//     // fn zero()->Self;
// }

/**
 * A sparse vector is a list of indices along with corresponding values.
 * The indices are selected from the set {0,1,...,domain_size-1}.
 */
#[derive(Default, Clone)]
pub struct sparse_vector<T: PpConfig> {
    pub indices: Vec<usize>,
    pub values: Vec<T>,
    pub domain_size_: usize,
    // sparse_vector() = default;
    // sparse_vector(&other:sparse_vector<T>) = default;
    // sparse_vector(sparse_vector<T> &&other) = default;
    // pub fn new(v:Vec<T>); /* constructor from Vec */

    // sparse_vector<T>& operator=(&other:sparse_vector<T>) = default;
    // sparse_vector<T>& operator=(sparse_vector<T> &&other) = default;

    // T operator[](idx:usize) const;

    // bool operator==(&other:sparse_vector<T>) const;
    // bool operator==(&other:Vec<T>) const;

    // bool is_valid() const;
    // bool empty() const;

    // usize domain_size() const; // return domain_size_
    // usize size() const; // return the number of indices (representing the number of non-zero entries)
    //  pub fn size_in_bits(&self)->usize; // return the number bits needed to store the sparse vector

    // /* return a pair consisting of the accumulated value and the sparse vector of non-accumulated values */
    //
    // std::pair<T, sparse_vector<T> > accumulate(it_begin:&Vec<FieldT>::const_iterator
    //                                            it_end:&Vec<FieldT>::const_iterator
    //                                            offset:usize) const;
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

        return true;
    }

    pub fn empty(&self) -> bool {
        return self.indices.is_empty();
    }

    pub fn domain_size(&self) -> usize {
        return self.domain_size_;
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }

    pub fn size_in_bits(&self) -> usize {
        self.indices.len() * (std::mem::size_of::<usize>() * 8 + T::size_in_bits())
    }

    pub fn accumulate<FieldT: PpConfig>(
        &self,
        it: &[FieldT],
        offset: usize,
    ) -> (T, sparse_vector<T>) {
        // // #ifdef MULTICORE
        //     override:usize chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
        // #else
        let mut chunks = 1;
        // //#endif

        let mut accumulated_value = T::zero();
        let mut resulting_vector = sparse_vector::<T>::default();
        resulting_vector.domain_size_ = self.domain_size_;

        let mut range_len = it.len();
        let mut in_block = false;
        let mut first_pos = -1;
        let mut last_pos = -1; // g++ -flto emits unitialized warning, even though in_block guards for such cases.

        for i in 0..self.indices.len() {
            let matching_pos = (offset <= self.indices[i] && self.indices[i] < offset + range_len);
            // print!("i = {}, pos[i] = {}, offset = {}, w_size = {}\n", i, indices[i], offset, w_size);
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

                    // // #ifdef DEBUG
                    //                 ffec::print_indent(); print!("doing multiexp for w_{} ... w_{}\n", indices[first_pos], indices[last_pos]);
                    // //#endif
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
            // // #ifdef DEBUG
            //         ffec::print_indent(); print!("doing multiexp for w_{} ... w_{}\n", indices[first_pos], indices[last_pos]);
            // //#endif
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

// pub fn
// T sparse_vector<T>::operator[](idx:usize) const
// {
//     auto it = std::lower_bound(indices.begin(), indices.end(), idx);
//     return if (it != indices.end() && *it == idx) {values[it - indices.begin()]} else{T()};
// }

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

        /* at least one of the vectors has been exhausted, so other must be empty */
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

        return true;
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

        return true;
    }
}

// pub fn
// bool sparse_vector<T>::operator==(&other:Vec<T>) const
// {
//     if self.domain_size_ < other.len()
//     {
//         return false;
//     }

//     usize j = 0;
//     for i in 0..other.len()
//     {
//         if self.indices[j] == i
//         {
//             if self.values[j] != other[j]
//             {
//                 return false;
//             }
//             j+=1;
//         }
//         else
//         {
//             if !other[j].is_zero()
//             {
//                 return false;
//             }
//         }
//     }

//     return true;
// }

use std::fmt;

impl<ppT: PpConfig> fmt::Display for sparse_vector<ppT> {
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

// pub fn
// std::istream& operator>>(std::istream& in, sparse_vector<T> &v)
// {
//     in >> self.domain_size_;
//     ffec::consume_newline(in);

//     usize s;
//     in >> s;
//     ffec::consume_newline(in);
//     self.indices.resize(s);
//     for i in 0..s
//     {
//         in >> self.indices[i];
//         ffec::consume_newline(in);
//     }

//     self.values.clear();
//     in >> s;
//     ffec::consume_newline(in);
//     self.values.reserve(s);

//     for i in 0..s
//     {
//         T t;
//         in >> t;
//         ffec::consume_OUTPUT_NEWLINE(in);
//         self.values.push(t);
//     }

//     return in;
// }
