/** @file
 *****************************************************************************

 Declaration of interfaces for profiling constraints.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef CONSTRAINT_PROFILING_HPP_
// #define CONSTRAINT_PROFILING_HPP_

// use  <cstddef>
// use  <map>
// use  <string>
// use  <vector>



extern size_t constraint_profiling_indent;

struct constraint_profiling_entry {
    size_t indent;
    std::string annotation;
    size_t count;
};

extern std::vector<constraint_profiling_entry> constraint_profiling_table;

// #define PROFILE_CONSTRAINTS(pb, annotation)                             \
    for (size_t _num_constraints_before = pb.num_constraints(), _iter = (++constraint_profiling_indent, 0), _cp_pos = constraint_profiling_table.size(); \
         _iter == 0;                                                    \
         constraint_profiling_table.insert(constraint_profiling_table.begin() + _cp_pos, constraint_profiling_entry{--constraint_profiling_indent, annotation, pb.num_constraints() - _num_constraints_before}), \
         _iter = 1)

size_t PRINT_CONSTRAINT_PROFILING(); // returns # of top level constraints



//#endif // CONSTRAINT_PROFILING_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for profiling constraints.

 See constraint_profiling.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use ffec::common::profiling;

use crate::gadgetlib1::constraint_profiling;



size_t constraint_profiling_indent = 0;
std::vector<constraint_profiling_entry> constraint_profiling_table;

size_t PRINT_CONSTRAINT_PROFILING()
{
    size_t accounted = 0;
    ffec::print_indent();
    print!("Constraint profiling:\n");
    for ent in &constraint_profiling_table
    {
        if ent.indent == 0
        {
            accounted += ent.count;
        }

        ffec::print_indent();
        for i in 0..ent.indent
        {
            print!("  ");
        }
        print!("* Number of constraints in [%s]: {}\n", ent.annotation.c_str(), ent.count);
    }

    constraint_profiling_table.clear();
    constraint_profiling_indent = 0;

    return accounted;
}

}
