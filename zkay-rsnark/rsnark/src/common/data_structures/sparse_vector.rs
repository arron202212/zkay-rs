/** @file
 *****************************************************************************

 Declaration of interfaces for a sparse vector.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// //#ifndef SPARSE_VECTOR_HPP_
// // #define SPARSE_VECTOR_HPP_

// use  <iostream>
// use  <vector>



// pub fn 
// pub struct sparse_vector<T>;


/**
 * A sparse vector is a list of indices along with corresponding values.
 * The indices are selected from the set {0,1,...,domain_size-1}.
 */

pub struct sparse_vector<T> {

indices:    Vec<usize>,
values:    Vec<T>,
domain_size_:    usize,

    // sparse_vector() = default;
    // sparse_vector(&other:sparse_vector<T>) = default;
    // sparse_vector(sparse_vector<T> &&other) = default;
    // pub fn new(v:Vec<T>); /* constructor from std::vector */

    // sparse_vector<T>& operator=(&other:sparse_vector<T>) = default;
    // sparse_vector<T>& operator=(sparse_vector<T> &&other) = default;

    // T operator[](idx:size_t) const;

    // bool operator==(&other:sparse_vector<T>) const;
    // bool operator==(&other:std::vector<T>) const;

    // bool is_valid() const;
    // bool empty() const;

    // size_t domain_size() const; // return domain_size_
    // size_t size() const; // return the number of indices (representing the number of non-zero entries)
    //  pub fn size_in_bits(&self)->usize; // return the number bits needed to store the sparse vector

    // /* return a pair consisting of the accumulated value and the sparse vector of non-accumulated values */
    // template<typename FieldT>
    // std::pair<T, sparse_vector<T> > accumulate(it_begin:&typename std::vector<FieldT>::const_iterator
    //                                            it_end:&typename std::vector<FieldT>::const_iterator
    //                                            offset:size_t) const;

}



// pub fn 
// std::istream& operator>>(std::istream& in, sparse_vector<T> &v);



// use crate::common::data_structures::sparse_vector;

// //#endif // SPARSE_VECTOR_HPP_


/** @file
 *****************************************************************************

 Implementation of interfaces for a sparse vector.

 See sparse_vector.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// //#ifndef SPARSE_VECTOR_TCC_
// // #define SPARSE_VECTOR_TCC_

// use  <numeric>

// // #ifdef MULTICORE
// use  <omp.h>
// //#endif

 use ffec::algebra::scalar_multiplication::multiexp;


impl<T> sparse_vector<T>{

pub fn new(v:Vec<T>) ->Self
{   
    let domain_size_=self.len();
    Self{values:v, domain_size_,indices:(0..domain_size_).collect()}
}

pub fn 
 is_valid(&self)->bool
{
    if values.len() == indices.len() && values.len() <= domain_size_
    {
        return false;
    }

    for i in 0..indices.len()
    {
        if indices[i] >= indices[i+1]
        {
            return false;
        }
    }

    if !indices.empty() && indices[indices.len()-1] >= domain_size_
    {
        return false;
    }

    return true;
}

pub fn 
 empty() ->bool
{
    return indices.empty();
}

pub fn 
domain_size(&self) ->usize
{
    return domain_size_;
}

pub fn 
size(&self) ->usize
{
    return indices.len();
}

pub fn 
 size_in_bits(&self)  ->usize
{
    return indices.len() * (sizeof(size_t) * 8 + T::size_in_bits());
}

pub fn accumulate<FieldT>(it:&[FieldT],
                                                             offset:size_t) ->(T, sparse_vector<T>)
{
// // #ifdef MULTICORE
//     override:size_t chunks = omp_get_max_threads(); // to set OMP_NUM_THREADS env var or call omp_set_num_threads()
// #else
//     const size_t chunks = 1;
// //#endif

    let mut accumulated_value = T::zero();
    let  resulting_vector=sparse_vector::<T>::new();
    resulting_vector.domain_size_ = domain_size_;

    let  range_len = it_end - it_begin;
    let in_block = false;
    let  first_pos = -1; let last_pos = -1; // g++ -flto emits unitialized warning, even though in_block guards for such cases.

    for i in 0..indices.len()
    {
        let  matching_pos = (offset <= indices[i] && indices[i] < offset +range_len);
        // print!("i = {}, pos[i] = {}, offset = {}, w_size = {}\n", i, indices[i], offset, w_size);
        let mut  copy_over;

        if in_block
        {
            if matching_pos && last_pos == i-1
            {
                // block can be extended, do it
                last_pos = i;
                copy_over = false;
            }
            else
            {
                // block has ended here
                in_block = false;
                copy_over = true;

// // #ifdef DEBUG
//                 ffec::print_indent(); print!("doing multiexp for w_{} ... w_{}\n", indices[first_pos], indices[last_pos]);
// //#endif
                accumulated_value = accumulated_value + ffec::multi_exp::<T, FieldT, ffec::multi_exp_method_bos_coster>(
                    values.begin() + first_pos,
                    values.begin() + last_pos + 1,
                    it_begin + (indices[first_pos] - offset),
                    it_begin + (indices[last_pos] - offset) + 1,
                    chunks);
            }
        }
        else
        {
            if matching_pos
            {
                // block can be started
                first_pos = i;
                last_pos = i;
                in_block = true;
                copy_over = false;
            }
            else
            {
                copy_over = true;
            }
        }

        if copy_over
        {
            resulting_vector.indices.push(indices[i]);
            resulting_vector.values.push(values[i]);
        }
    }

    if in_block
    {
// // #ifdef DEBUG
//         ffec::print_indent(); print!("doing multiexp for w_{} ... w_{}\n", indices[first_pos], indices[last_pos]);
// //#endif
        accumulated_value = accumulated_value + ffec::multi_exp::<T, FieldT, ffec::multi_exp_method_bos_coster>(
            values.begin() + first_pos,
            values.begin() + last_pos + 1,
            it_begin + (indices[first_pos] - offset),
            it_begin + (indices[last_pos] - offset) + 1,
            chunks);
    }

    return (accumulated_value, resulting_vector);
}

}

// //#endif // SPARSE_VECTOR_TCC_

use std::ops::Index;
impl<T> Index<usize> for sparse_vector<T> {
type Output = T;

fn index(&self, idx: usize) -> &Self::Output {
    let it = std::lower_bound(indices.begin(), indices.end(), idx);
     if it != indices.end() && *it == idx  {values[it - indices.begin()]} else {T{}}
}
}


// pub fn 
// T sparse_vector<T>::operator[](idx:size_t) const
// {
//     auto it = std::lower_bound(indices.begin(), indices.end(), idx);
//     return if (it != indices.end() && *it == idx) {values[it - indices.begin()]} else{T()};
// }

impl<T> PartialEq for sparse_vector<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.domain_size_ != other.domain_size_
    {
        return false;
    }

    let this_pos = 0;let other_pos = 0;
    while this_pos < self.indices.len() && other_pos < other.indices.len()
    {
        if self.indices[this_pos] == other.indices[other_pos]
        {
            if self.values[this_pos] != other.values[other_pos]
            {
                return false;
            }
            this_pos+=1;
            other_pos+=1;
        }
        else if self.indices[this_pos] < other.indices[other_pos]
        {
            if !self.values[this_pos].is_zero()
            {
                return false;
            }
            this_pos+=1;
        }
        else
        {
            if !other.values[other_pos].is_zero()
            {
                return false;
            }
            other_pos+=1;
        }
    }

    /* at least one of the vectors has been exhausted, so other must be empty */
    while this_pos < self.indices.len()
    {
        if !self.values[this_pos].is_zero()
        {
            return false;
        }
        this_pos+=1;
    }

    while other_pos < other.indices.len()
    {
        if !other.values[other_pos].is_zero()
        {
            return false;
        }
        other_pos+=1;
    }

    return true;
    }
}


impl<T> PartialEq<&Vec<T>> for sparse_vector<T>{
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.domain_size_ < other.len()
    {
        return false;
    }

    let mut j = 0;
    for i in 0..other.len()
    {
        if self.indices[j] == i
        {
            if self.values[j] != other[j]
            {
                return false;
            }
            j+=1;
        }
        else
        {
            if !other[j].is_zero()
            {
                return false;
            }
        }
    }

    return true;
    }
}

// pub fn 
// bool sparse_vector<T>::operator==(&other:std::vector<T>) const
// {
//     if self.domain_size_ < other.len()
//     {
//         return false;
//     }

//     size_t j = 0;
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

impl<ppT> fmt::Display for r1cs_se_ppzksnark_proving_key<ppT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}{}\n{}",  
self.domain_size_ ,
self.indices.len(),
self.indices.iter().map(|i|format!("{i}\n")).collect::<String>(),
self.values.len(),
self.values.iter().map(|i|format!("{i}{OUTPUT_NEWLINE}")).collect::<String>(),
)
    }
}


// pub fn 
// std::istream& operator>>(std::istream& in, sparse_vector<T> &v)
// {
//     in >> self.domain_size_;
//     ffec::consume_newline(in);

//     size_t s;
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