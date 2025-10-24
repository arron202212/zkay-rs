/** @file
 *****************************************************************************

 Declaration of interfaces for a permutation of the integers in {min_element,...,max_element}.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef INTEGER_PERMUTATION_HPP_
// #define INTEGER_PERMUTATION_HPP_

// use  <cstddef>
// use  <vector>



pub struct integer_permutation {
// private:
contents:    std::vector<size_t>, /* offset by min_element */

// 
min_element:    size_t,
max_element:    size_t,

    // integer_permutation(size:size_t = 0);
    // integer_permutation(min_element:size_t, max_element:size_t);

    // integer_permutation& operator=(const integer_permutation &other) = default;

    // size_t size() const;
    // bool operator==(const integer_permutation &other) const;

    // void set(position:size_t, value:size_t);
    // size_t get(position:size_t) const;

    // bool is_valid() const;
    // integer_permutation inverse() const;
    // integer_permutation slice(slice_min_element:size_t, slice_max_element:size_t) const;

    // /* Similarly to std::next_permutation this transforms the current
    // integer permutation into the next lexicographically ordered
    // permutation; returns false if the last permutation was reached and
    // this is now the identity permutation on [min_element .. max_element] */
    // bool next_permutation();

    // void random_shuffle();
}



//#endif // INTEGER_PERMUTATION_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a permutation of the integers in {min_element,...,max_element}.

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


impl integer_permutation{
pub fn new(size:size_t) ->Self
{
    contents.resize(size);
    std::iota(contents.begin(), contents.end(), 0);
    Self{min_element:0, max_element:size-1}
}

pub fn new2(min_element:size_t, max_element:size_t) ->Self
    
{
    assert!(min_element <= max_element);
    let size = max_element - min_element + 1;
    contents.resize(size);
    std::iota(contents.begin(), contents.end(), min_element);
    Self{min_element, max_element}
}

 pub fn size() ->size_t
{
    return max_element - min_element + 1;
}


pub fn set(position:size_t, value:size_t)
{
    assert!(min_element <= position && position <= max_element);
    contents[position - min_element] = value;
}

 pub fn get(position:size_t) ->size_t
{
    assert!(min_element <= position && position <= max_element);
    return contents[position - min_element];
}


 pub fn is_valid() ->bool
{
    let mut  elems=HashSet::new();

    for el in &contents
    {
        if el < min_element || el > max_element || elems.contains(el)
        {
            return false;
        }

        elems.insert(el);
    }

    return true;
}

 pub fn inverse() ->Self
{
     let mut result=integer_permutation::new(min_element, max_element);

    for position in min_element..=max_element
    {
        result.contents[self.contents[position - min_element] - min_element] = position;
    }

// #ifdef DEBUG
    // assert!(result.is_valid());
//#endif

    return result;
}

 pub fn slice(slice_min_element:size_t, slice_max_element:size_t) ->Self
{
    assert!(min_element <= slice_min_element && slice_min_element <= slice_max_element && slice_max_element <= max_element);
    let mut  result=integer_permutation::new(slice_min_element, slice_max_element);
    // std::copy(self.contents.begin() + (slice_min_element - min_element),
    //           self.contents.begin() + (slice_max_element - min_element) + 1,
    //           result.contents.begin());
    result.contents.insert(0,self.contents[slice_min_element - min_element]);
// #ifdef DEBUG
    // assert!(result.is_valid());
//#endif

    return result;
}

 pub fn next_permutation()->bool
{
    return std::next_permutation(contents.begin(), contents.end());
}

pub fn random_shuffle()
{
    return std::random_shuffle(contents.begin(), contents.end());
}


}



// bool pub fn operator==(const integer_permutation &other) const
// {
//     return (self.min_element == other.min_element &&
//             self.max_element == other.max_element &&
//             self.contents == other.contents);
// }