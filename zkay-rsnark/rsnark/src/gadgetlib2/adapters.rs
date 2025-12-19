// /** @file
//  *****************************************************************************
//  Declaration of an adapter to POD types for interfacing to SNARKs
//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
// //#ifndef LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_ADAPTERS_HPP_
// // #define LIBSNARK_GADGETLIB2_INCLUDE_GADGETLIB2_ADAPTERS_HPP_

// use  <map>
// use  <tuple>
// use  <utility>

// use crate::gadgetlib2::constraint;
// use crate::gadgetlib2::pp;
// use crate::gadgetlib2::protoboard;
// use crate::gadgetlib2::variable;

// // using gadgetlib2::LinearTerm;
// // using gadgetlib2::LinearCombination;
// // using gadgetlib2::Constraint;
// // using gadgetlib2::ConstraintSystem;
// // using gadgetlib2::VariableAssignment;
// // using gadgetlib2::Protoboard;
// // using gadgetlib2::FElem;

pub trait ConvertConfig<T, R> {
    fn convert(t: T) -> R;
}
// namespace gadgetlib2 {

// /**
//  * This pub struct is a temporary hack for quick integration of Fp constraints with ppsnark. It is the
//  * IDDQD of classes and has "god mode" friend access to many of the gadgetlib classes. This will
//  * be refactored out in the future. --Shaul
//  */
pub trait GadgetLibAdapter {
    type variable_index_t = u64;
    type Fp_elem_t = Fp;
    type linear_term_t = (variable_index_t, Fp_elem_t);
    type sparse_vec_t = Vec<linear_term_t>;
    type linear_combination_t = (sparse_vec_t, Fp_elem_t);
    type constraint_t = (
        linear_combination_t,
        linear_combination_t,
        linear_combination_t,
    );
    type constraint_sys_t = Vec<constraint_t>;
    type assignment_t = HashMap<variable_index_t, Fp_elem_t>;
    type protoboard_t = (constraint_sys_t, assignment_t);

    //    fn  convert(lt:&LinearTerm)->linear_term_t;
    //    fn  convert(lc:&LinearCombination)->linear_combination_t;
    //    fn  convert(constraint:&Constraint)->constraint_t;
    //    fn  convert(constraint_sys:&ConstraintSystem)->constraint_sys_t;
    //    fn  convert(assignment:&VariableAssignment)->assignment_t;
    fn resetVariableIndex();
    ///< Resets variable index to 0 to make variable indices deterministic.
    //TODO: Kill GadgetLibAdapter::resetVariableIndex()
    fn getNextFreeIndex() -> usize {
        return Variable::nextFreeIndex_;
    }
    //    fn  convert(pb:&Protoboard)->protoboard_t;
    //    fn  convert(fElem:FElem)->Fp_elem_t;
    fn getVariableIndex(v: &Variable) -> usize {
        return v.index_;
    }
}

// bool operator==(lhs:GadgetLibAdapter::linear_combination_t&,
//                 const GadgetLibAdapter::linear_term_t& rhs);

// }

// using gadgetlib2::Variable;
// using gadgetlib2::Rank1Constraint;

pub struct GLA;
impl GLA {
    pub fn new() -> Self {
        Self
    }
}
// type GLA=GadgetLibAdapter;
impl GadgetLibAdapter for GLA {
    fn resetVariableIndex() {
        // This is a hack, used for testing
        Variable::nextFreeIndex_ = 0;
    }
}

impl ConvertConfig<&LinearTerm, linear_term_t> for GLA {
    fn convert(lt: &LinearTerm) -> linear_term_t {
        let var = lt.variable_.index_;
        let coeff = convert(lt.coeff_);
        (var, coeff)
    }
}

impl ConvertConfig<&LinearCombination, linear_combination_t> for GLA {
    fn convert(lc: &LinearCombination) -> linear_combination_t {
        let mut sparse_vec = sparse_vec_t::with_capacity(lc.linearTerms_.len());
        // sparse_vec.reserve(lc.linearTerms_.len());
        for lt in &lc.linearTerms_ {
            sparse_vec.push(convert(lt));
        }
        let offset = convert(lc.constant_);
        (sparse_vec, offset)
    }
}
impl ConvertConfig<&Constraint, constraint_t> for GLA {
    fn convert(constraint: &Constraint) -> constraint_t {
        let rank1_constraint = &constraint;
        (
            convert(rank1_constraint.a()),
            convert(rank1_constraint.b()),
            convert(rank1_constraint.c()),
        )
    }
}
impl ConvertConfig<&ConstraintSystem, constraint_sys_t> for GLA {
    fn convert(constraint_sys: &ConstraintSystem) -> constraint_sys_t {
        let mut retval = constraint_sys_t::with_capacity(constraint_sys.constraintsPtrs_.len());
        // retval.reserve(constraint_sys.constraintsPtrs_.len());
        for constraintPtr in &constraint_sys.constraintsPtrs_ {
            retval.push(convert(*constraintPtr));
        }
        return retval;
    }
}
impl ConvertConfig<&VariableAssignment, assignment_t> for GLA {
    fn convert(assignment: &VariableAssignment) -> assignment_t {
        let mut retval = assignment_t::new();
        for assignmentPair in &assignment {
            let var = assignmentPair.first.index_;
            let elem = convert(assignmentPair.second);
            retval[var] = elem;
        }
        return retval;
    }
}

// /***TODO: Remove reliance of GadgetLibAdapter conversion on global variable indices, and the resulting limit of single protoboard instance at a time.
// This limitation is to prevent a logic bug that may occur if the variables used are given different indices in different generations of the same constraint system.
// The indices are assigned on the Variable constructor, using the global variable nextFreeIndex. Thus, creating two protoboards in the same program may cause
// unexpected behavior when converting.
// Moreover, the bug will create more variables than needed in the converted system, e.g. if variables 0,1,3,4 were used in the gadgetlib2
// generated system, then the conversion will create a new r1cs system with variables 0,1,2,3,4 and assign variable 2 the value zero
// (when converting the assignment).
// Everything should be fixed soon.
// If you are sure you know what you are doing, you can comment out the ASSERT line.
// */
impl ConvertConfig<&Protoboard, protoboard_t> for GLA {
    fn convert(pb: &Protoboard) -> protoboard_t {
        //GADGETLIB_ASSERT(pb.numVars()==getNextFreeIndex(), "Some Variables were created and not used, or, more than one protoboard was used.");
        return protoboard_t(convert(pb.constraintSystem()), convert(pb.assignment()));
    }
}
impl ConvertConfig<FElem, Fp_elem_t> for GLA {
    fn convert(fElem: FElem) -> Fp_elem_t {
        // using gadgetlib2::R1P_Elem;
        fElem.promoteToFieldType(gadgetlib2::R1P); // convert fElem from FConst to R1P_Elem
        let pR1P = fElem.elem_.get();
        return pR1P.elem_.clone();
    }
}

impl PartialEq for linear_combination_t {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == 1 && self.0.at(0) == rhs && self.1 == Fp::from(0)
    }
}

// bool operator==(lhs:linear_combination_t&,
//     rhs:&linear_term_t) {
//     return lhs.first.len() == 1 &&
//         lhs.first.at(0) == rhs &&
//         lhs.second == Fp(0);
// }

// }
