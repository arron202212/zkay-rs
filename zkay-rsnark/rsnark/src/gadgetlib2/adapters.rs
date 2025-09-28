/** @file
 *****************************************************************************
 Declaration of an adapter to POD types for interfacing to SNARKs
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_ADAPTERS_HPP_
#define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_ADAPTERS_HPP_

use  <map>
use  <tuple>
use  <utility>

use  <libsnark/gadgetlib2/constraint.hpp>
use  <libsnark/gadgetlib2/pp.hpp>
use  <libsnark/gadgetlib2/protoboard.hpp>
use  <libsnark/gadgetlib2/variable.hpp>

using gadgetlib2::LinearTerm;
using gadgetlib2::LinearCombination;
using gadgetlib2::Constraint;
using gadgetlib2::ConstraintSystem;
using gadgetlib2::VariableAssignment;
using gadgetlib2::Protoboard;
using gadgetlib2::FElem;


namespace gadgetlib2 {

/**
 * This class is a temporary hack for quick integration of Fp constraints with ppsnark. It is the
 * IDDQD of classes and has "god mode" friend access to many of the gadgetlib classes. This will
 * be refactored out in the future. --Shaul
 */
class GadgetLibAdapter {
public:
    type unsigned long variable_index_t;
    type gadgetlib2::Fp Fp_elem_t;
    type ::std::pair<variable_index_t, Fp_elem_t> linear_term_t;
    type ::std::vector<linear_term_t> sparse_vec_t;
    type ::std::pair<sparse_vec_t, Fp_elem_t> linear_combination_t;
    type ::std::tuple<linear_combination_t,
                         linear_combination_t,
                         linear_combination_t> constraint_t;
    type ::std::vector<constraint_t> constraint_sys_t;
    type ::std::map<variable_index_t, Fp_elem_t> assignment_t;
    type ::std::pair<constraint_sys_t, assignment_t> protoboard_t;

    GadgetLibAdapter() {};

    linear_term_t convert(const LinearTerm& lt) const;
    linear_combination_t convert(const LinearCombination& lc) const;
    constraint_t convert(const Constraint& constraint) const;
    constraint_sys_t convert(const ConstraintSystem& constraint_sys) const;
    assignment_t convert(const VariableAssignment& assignment) const;
    static void resetVariableIndex(); ///< Resets variable index to 0 to make variable indices deterministic.
                                      //TODO: Kill GadgetLibAdapter::resetVariableIndex()
    static size_t getNextFreeIndex(){return Variable::nextFreeIndex_;}
    protoboard_t convert(const Protoboard& pb) const;
    Fp_elem_t convert(FElem fElem) const;
    static size_t getVariableIndex(const Variable& v){return v.index_;}
};

bool operator==(const GadgetLibAdapter::linear_combination_t& lhs,
                const GadgetLibAdapter::linear_term_t& rhs);

}

#endif // LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_ADAPTERS_HPP_
/** @file
 *****************************************************************************
 Implementation of an adapter for interfacing to SNARKs.
 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <libsnark/gadgetlib2/adapters.hpp>

using gadgetlib2::Variable;
using gadgetlib2::Rank1Constraint;

namespace gadgetlib2 {

type GadgetLibAdapter GLA;

GLA::linear_term_t GLA::convert(const LinearTerm& lt) const {
    const variable_index_t var = lt.variable_.index_;
    const Fp_elem_t coeff = convert(lt.coeff_);
    return{ var, coeff };
}

GLA::linear_combination_t GLA::convert(const LinearCombination& lc) const {
    sparse_vec_t sparse_vec;
    sparse_vec.reserve(lc.linearTerms_.size());
    for (auto lt : lc.linearTerms_) {
        sparse_vec.emplace_back(convert(lt));
    }
    const Fp_elem_t offset = convert(lc.constant_);
    return{ sparse_vec, offset };
}

GLA::constraint_t GLA::convert(const Constraint& constraint) const {
    const auto rank1_constraint = dynamic_cast<const Rank1Constraint&>(constraint);
    return constraint_t(convert(rank1_constraint.a()),
        convert(rank1_constraint.b()),
        convert(rank1_constraint.c()));
}

GLA::constraint_sys_t GLA::convert(const ConstraintSystem& constraint_sys) const {
    constraint_sys_t retval;
    retval.reserve(constraint_sys.constraintsPtrs_.size());
    for (auto constraintPtr : constraint_sys.constraintsPtrs_) {
        retval.emplace_back(convert(*constraintPtr));
    }
    return retval;
}

GLA::assignment_t GLA::convert(const VariableAssignment& assignment) const {
    assignment_t retval;
    for (const auto assignmentPair : assignment) {
        const variable_index_t var = assignmentPair.first.index_;
        const Fp_elem_t elem = convert(assignmentPair.second);
        retval[var] = elem;
    }
    return retval;
}

void GLA::resetVariableIndex() { // This is a hack, used for testing
    Variable::nextFreeIndex_ = 0;
}

/***TODO: Remove reliance of GadgetLibAdapter conversion on global variable indices, and the resulting limit of single protoboard instance at a time.
This limitation is to prevent a logic bug that may occur if the variables used are given different indices in different generations of the same constraint system.
The indices are assigned on the Variable constructor, using the global variable nextFreeIndex. Thus, creating two protoboards in the same program may cause
unexpected behavior when converting.
Moreover, the bug will create more variables than needed in the converted system, e.g. if variables 0,1,3,4 were used in the gadgetlib2
generated system, then the conversion will create a new r1cs system with variables 0,1,2,3,4 and assign variable 2 the value zero
(when converting the assignment).
Everything should be fixed soon.
If you are sure you know what you are doing, you can comment out the ASSERT line.
*/
GLA::protoboard_t GLA::convert(const Protoboard& pb) const {
	//GADGETLIB_ASSERT(pb.numVars()==getNextFreeIndex(), "Some Variables were created and not used, or, more than one protoboard was used.");
    return protoboard_t(convert(pb.constraintSystem()), convert(pb.assignment()));
}

GLA::Fp_elem_t GLA::convert(FElem fElem) const {
    using gadgetlib2::R1P_Elem;
    fElem.promoteToFieldType(gadgetlib2::R1P); // convert fElem from FConst to R1P_Elem
    const R1P_Elem* pR1P = dynamic_cast<R1P_Elem*>(fElem.elem_.get());
    return pR1P->elem_;
}

bool operator==(const GLA::linear_combination_t& lhs,
    const GLA::linear_term_t& rhs) {
    return lhs.first.size() == 1 &&
        lhs.first.at(0) == rhs &&
        lhs.second == Fp(0);
}

}
