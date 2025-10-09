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

use  <cstddef>
use  <vector>



class integer_permutation {
private:
    std::vector<size_t> contents; /* offset by min_element */

public:
    size_t min_element;
    size_t max_element;

    integer_permutation(const size_t size = 0);
    integer_permutation(const size_t min_element, const size_t max_element);

    integer_permutation& operator=(const integer_permutation &other) = default;

    size_t size() const;
    bool operator==(const integer_permutation &other) const;

    void set(const size_t position, const size_t value);
    size_t get(const size_t position) const;

    bool is_valid() const;
    integer_permutation inverse() const;
    integer_permutation slice(const size_t slice_min_element, const size_t slice_max_element) const;

    /* Similarly to std::next_permutation this transforms the current
    integer permutation into the next lexicographically ordered
    permutation; returns false if the last permutation was reached and
    this is now the identity permutation on [min_element .. max_element] */
    bool next_permutation();

    void random_shuffle();
};



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

use  <algorithm>
use  <cassert>
use  <numeric>
use  <unordered_set>

use crate::common::data_structures::integer_permutation;



integer_permutation::integer_permutation(const size_t size) :
    min_element(0), max_element(size-1)
{
    contents.resize(size);
    std::iota(contents.begin(), contents.end(), 0);
}

integer_permutation::integer_permutation(const size_t min_element, const size_t max_element) :
    min_element(min_element), max_element(max_element)
{
    assert!(min_element <= max_element);
    const size_t size = max_element - min_element + 1;
    contents.resize(size);
    std::iota(contents.begin(), contents.end(), min_element);
}

size_t integer_permutation::size() const
{
    return max_element - min_element + 1;
}

bool integer_permutation::operator==(const integer_permutation &other) const
{
    return (self.min_element == other.min_element &&
            self.max_element == other.max_element &&
            self.contents == other.contents);
}

void integer_permutation::set(const size_t position, const size_t value)
{
    assert!(min_element <= position && position <= max_element);
    contents[position - min_element] = value;
}

size_t integer_permutation::get(const size_t position) const
{
    assert!(min_element <= position && position <= max_element);
    return contents[position - min_element];
}


bool integer_permutation::is_valid() const
{
    std::unordered_set<size_t> elems;

    for (auto &el : contents)
    {
        if (el < min_element || el > max_element || elems.find(el) != elems.end())
        {
            return false;
        }

        elems.insert(el);
    }

    return true;
}

integer_permutation integer_permutation::inverse() const
{
    integer_permutation result(min_element, max_element);

    for (size_t position = min_element; position <= max_element; ++position)
    {
        result.contents[self.contents[position - min_element] - min_element] = position;
    }

// #ifdef DEBUG
    assert!(result.is_valid());
//#endif

    return result;
}

integer_permutation integer_permutation::slice(const size_t slice_min_element, const size_t slice_max_element) const
{
    assert!(min_element <= slice_min_element && slice_min_element <= slice_max_element && slice_max_element <= max_element);
    integer_permutation result(slice_min_element, slice_max_element);
    std::copy(self.contents.begin() + (slice_min_element - min_element),
              self.contents.begin() + (slice_max_element - min_element) + 1,
              result.contents.begin());
// #ifdef DEBUG
    assert!(result.is_valid());
//#endif

    return result;
}

bool integer_permutation::next_permutation()
{
    return std::next_permutation(contents.begin(), contents.end());
}

void integer_permutation::random_shuffle()
{
    return std::random_shuffle(contents.begin(), contents.end());
}


