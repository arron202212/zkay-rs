use ffec::common::profiling::print_indent;

pub struct constraint_profiling_entry {
    indent: usize,
    annotation: String,
    count: usize,
}

// extern Vec<constraint_profiling_entry> constraint_profiling_table;

// #define PROFILE_CONSTRAINTS(pb, annotation)                             \;
// for (usize _num_constraints_before = pb.num_constraints(), _iter = (++constraint_profiling_indent, 0), _cp_pos = constraint_profiling_table.len(); \
//      _iter == 0;                                                    \
//      constraint_profiling_table.insert(constraint_profiling_table.begin() + _cp_pos, constraint_profiling_entry{--constraint_profiling_indent, annotation, pb.num_constraints() - _num_constraints_before}), \
//      _iter = 1)

// usize PRINT_CONSTRAINT_PROFILING(); // returns # of top level constraints

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

// use crate::gadgetlib1::constraint_profiling;

// usize constraint_profiling_indent = 0;
// Vec<constraint_profiling_entry> constraint_profiling_table;

pub fn PRINT_CONSTRAINT_PROFILING() -> usize {
    let mut constraint_profiling_table = Vec::<constraint_profiling_entry>::new();
    let mut accounted = 0;
    print_indent();
    print!("Constraint profiling:\n");
    for ent in &constraint_profiling_table {
        if ent.indent == 0 {
            accounted += ent.count;
        }

        print_indent();
        for i in 0..ent.indent {
            print!("  ");
        }
        print!(
            "* Number of constraints in [{}]: {}\n",
            ent.annotation, ent.count
        );
    }

    constraint_profiling_table.clear();
    let constraint_profiling_indent = 0;

    return accounted;
}

// }
