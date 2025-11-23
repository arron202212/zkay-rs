/** @file
*****************************************************************************

Declaration of interfaces for a permutation of the integers in {self.min_element,...,self.max_element}.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef INTEGER_PERMUTATION_HPP_
// #define INTEGER_PERMUTATION_HPP_

// use  <cstddef>
//
use std::collections::HashSet;
#[derive(PartialEq)]
pub struct integer_permutation {
    //
    pub contents: Vec<usize>, /* offset by self.min_element */

    //
    pub min_element: usize,
    pub max_element: usize,
    // integer_permutation(size:usize = 0);
    // integer_permutation(self.min_element:usize, self.max_element:usize);

    // integer_permutation& operator=(other:&integer_permutation) = default;

    // usize size() const;
    // bool operator==(other:&integer_permutation) const;

    // pub fn  set(position:usize, value:usize);
    // usize get(position:usize) const;

    // bool is_valid() const;
    // integer_permutation inverse() const;
    // integer_permutation slice(slice_min_element:usize, slice_max_element:usize) const;

    // /* Similarly to std::next_permutation this transforms the current
    // integer permutation into the next lexicographically ordered
    // permutation; returns false if the last permutation was reached and
    // this is now the identity permutation on [self.min_element .. self.max_element] */
    // bool next_permutation();

    // pub fn  random_shuffle();
}

//#endif // INTEGER_PERMUTATION_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a permutation of the integers in {self.min_element,...,self.max_element}.

See integer_permutation.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

// use  <algorithm>
// use  <cassert>
// use  <numeric>
// use  <unordered_set>

// use crate::common::data_structures::integer_permutation;

impl integer_permutation {
    pub fn new(size: usize) -> Self {
        // contents.resize(size);
        // std::iota(contents.begin(), contents.end(), 0);
        Self {
            contents: vec![0; size],
            min_element: 0,
            max_element: size - 1,
        }
    }

    pub fn new2(min_element: usize, max_element: usize) -> Self {
        assert!(min_element <= max_element);
        let size = max_element - min_element + 1;
        // contents.resize(size);
        // std::iota(contents.begin(), contents.end(), self.min_element);
        Self {
            contents: vec![0; size],
            min_element,
            max_element,
        }
    }

    pub fn size(&self) -> usize {
        self.max_element - self.min_element + 1
    }

    pub fn set(&mut self, position: usize, value: usize) {
        assert!(self.min_element <= position && position <= self.max_element);
        self.contents[position - self.min_element] = value;
    }

    pub fn get(&self, position: usize) -> usize {
        assert!(self.min_element <= position && position <= self.max_element);
        self.contents[position - self.min_element]
    }

    pub fn is_valid(&self) -> bool {
        let mut elems = HashSet::new();

        for &el in &self.contents {
            if el < self.min_element || el > self.max_element || elems.contains(&el) {
                return false;
            }

            elems.insert(el);
        }

        true
    }

    pub fn inverse(&self) -> Self {
        let mut result = Self::new2(self.min_element, self.max_element);

        for position in self.min_element..=self.max_element {
            result.contents[self.contents[position - self.min_element] - self.min_element] =
                position;
        }

        // #ifdef DEBUG
        // assert!(result.is_valid());
        //#endif

        return result;
    }

    pub fn slice(&self, slice_min_element: usize, slice_max_element: usize) -> Self {
        assert!(
            self.min_element <= slice_min_element
                && slice_min_element <= slice_max_element
                && slice_max_element <= self.max_element
        );
        let mut result = Self::new2(slice_min_element, slice_max_element);
        // std::copy(self.contents.begin() + (slice_min_element - self.min_element),
        //           self.contents.begin() + (slice_max_element - self.min_element) + 1,
        //           result.contents.begin());
        result
            .contents
            .insert(0, self.contents[slice_min_element - self.min_element]);
        // #ifdef DEBUG
        // assert!(result.is_valid());
        //#endif

        return result;
    }

    pub fn next_permutation(&self) -> bool {
        //  std::next_permutation(contents.begin(), contents.end())
        false
    }

    pub fn random_shuffle(&self) {
        // return std::random_shuffle(contents.begin(), contents.end());
    }
}

// bool pub fn operator==(other:&integer_permutation) const
// {
//     return (self.min_element == other.self.min_element &&
//             self.max_element == other.self.max_element &&
//             self.contents == other.contents);
// }
