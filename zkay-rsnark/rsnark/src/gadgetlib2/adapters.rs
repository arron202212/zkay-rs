//  Declaration of an adapter to POD types for interfacing to SNARKs
use crate::gadgetlib2::constraint::{Constraint, ConstraintSystem, Rank1Constraint};
use crate::gadgetlib2::pp::Fp;
use crate::gadgetlib2::protoboard::Protoboard;
use crate::gadgetlib2::variable::{
    FElem, FieldType, LinearCombination, LinearTerm, Variable, VariableAssignment, nextFreeIndex_,
};
use crate::relations::FieldTConfig;
use std::collections::HashMap;
use std::sync::atomic::{self, AtomicUsize, Ordering};

pub trait ConvertConfig<T, R> {
    fn convert(t: T) -> R;
}

// /**
//  * This pub struct is a temporary hack for quick integration of Fp constraints with ppsnark. It is the
//  * IDDQD of classes and has "god mode" friend access to many of the gadgetlib classes. This will
//  * be refactored out in the future. --Shaul
//  */
pub trait GadgetLibAdapter {
    type variable_index_t = u64;
    type Fp_elem_t = Fp;
    type linear_term_t = (Self::variable_index_t, Self::Fp_elem_t);
    type sparse_vec_t = Vec<Self::linear_term_t>;
    type linear_combination_t = (Self::sparse_vec_t, Self::Fp_elem_t);
    type constraint_t = (
        Self::linear_combination_t,
        Self::linear_combination_t,
        Self::linear_combination_t,
    );
    type constraint_sys_t = Vec<Self::constraint_t>;
    type assignment_t = HashMap<Self::variable_index_t, Self::Fp_elem_t>;
    type protoboard_t = (Self::constraint_sys_t, Self::assignment_t);

    //    fn  convert(lt:&LinearTerm)->linear_term_t;
    //    fn  convert(lc:&LinearCombination)->linear_combination_t;
    //    fn  convert(constraint:&Constraint)->constraint_t;
    //    fn  convert(constraint_sys:&ConstraintSystem)->constraint_sys_t;
    //    fn  convert(assignment:&VariableAssignment)->assignment_t;
    fn resetVariableIndex();
    ///< Resets variable index to 0 to make variable indices deterministic.
    //TODO: Kill GadgetLibAdapter::resetVariableIndex()
    fn getNextFreeIndex() -> usize {
        nextFreeIndex_.load(Ordering::Relaxed)
    }
    //    fn  convert(pb:&Protoboard)->protoboard_t;
    //    fn  convert(fElem:FElem)->Fp_elem_t;
    fn getVariableIndex(v: &Variable) -> usize {
        v.index_ as _
    }
}

pub struct GLA;

// pub type GLA=GadgetLibAdapter;
impl GadgetLibAdapter for GLA {
    type variable_index_t = u64;
    type Fp_elem_t = Fp;
    type linear_term_t = (Self::variable_index_t, Self::Fp_elem_t);
    type sparse_vec_t = Vec<Self::linear_term_t>;
    type linear_combination_t = (Self::sparse_vec_t, Self::Fp_elem_t);
    type constraint_t = (
        Self::linear_combination_t,
        Self::linear_combination_t,
        Self::linear_combination_t,
    );
    type constraint_sys_t = Vec<Self::constraint_t>;
    type assignment_t = HashMap<Self::variable_index_t, Self::Fp_elem_t>;
    type protoboard_t = (Self::constraint_sys_t, Self::assignment_t);
    fn resetVariableIndex() {
        // This is a hack, used for testing
        // Variable::nextFreeIndex_ = 0;
        nextFreeIndex_.store(0, Ordering::Relaxed);
    }
}

pub type variable_index_t = <GLA as GadgetLibAdapter>::variable_index_t;
pub type Fp_elem_t = <GLA as GadgetLibAdapter>::Fp_elem_t;
pub type linear_term_t = <GLA as GadgetLibAdapter>::linear_term_t;
pub type sparse_vec_t = <GLA as GadgetLibAdapter>::sparse_vec_t;
pub type linear_combination_t = <GLA as GadgetLibAdapter>::linear_combination_t;
pub type constraint_t = <GLA as GadgetLibAdapter>::constraint_t;
pub type constraint_sys_t = <GLA as GadgetLibAdapter>::constraint_sys_t;
pub type assignment_t = <GLA as GadgetLibAdapter>::assignment_t;
pub type protoboard_t = <GLA as GadgetLibAdapter>::protoboard_t;

impl ConvertConfig<&LinearTerm, linear_term_t> for GLA {
    fn convert(lt: &LinearTerm) -> linear_term_t {
        let var = lt.variable_.index_;
        let coeff = Self::convert(&lt.coeff_);
        (var, coeff)
    }
}

impl ConvertConfig<&LinearCombination, linear_combination_t> for GLA {
    fn convert(lc: &LinearCombination) -> linear_combination_t {
        let mut sparse_vec = sparse_vec_t::with_capacity(lc.linearTerms_.len());
        // sparse_vec.reserve(lc.linearTerms_.len());
        for lt in &lc.linearTerms_ {
            sparse_vec.push(Self::convert(lt));
        }
        let offset = Self::convert(&lc.constant_);
        (sparse_vec, offset)
    }
}
impl ConvertConfig<&Constraint<Rank1Constraint>, constraint_t> for GLA {
    fn convert(constraint: &Constraint<Rank1Constraint>) -> constraint_t {
        let rank1_constraint = constraint;
        (
            Self::convert(rank1_constraint.t.a()),
            Self::convert(rank1_constraint.t.b()),
            Self::convert(rank1_constraint.t.c()),
        )
    }
}
impl ConvertConfig<&ConstraintSystem, constraint_sys_t> for GLA {
    fn convert(constraint_sys: &ConstraintSystem) -> constraint_sys_t {
        let mut retval = constraint_sys_t::with_capacity(constraint_sys.constraintsPtrs_.len());
        // retval.reserve(constraint_sys.constraintsPtrs_.len());
        for constraintPtr in &constraint_sys.constraintsPtrs_ {
            retval.push(Self::convert(
                constraintPtr.borrow().try_as_rank_1_ref().unwrap(),
            ));
        }
        return retval;
    }
}
impl ConvertConfig<&VariableAssignment, assignment_t> for GLA {
    fn convert(assignment: &VariableAssignment) -> assignment_t {
        let mut retval = assignment_t::new();
        for assignmentPair in assignment {
            let var = assignmentPair.0.index_;
            let elem = Self::convert(assignmentPair.1);
            retval.insert(var, elem);
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
        return (
            Self::convert(pb.constraintSystem()),
            Self::convert(pb.assignment()),
        );
    }
}
impl ConvertConfig<&FElem, Fp_elem_t> for GLA {
    fn convert(fElem: &FElem) -> Fp_elem_t {
        // using gadgetlib2::R1P_Elem;
        fElem.promoteToFieldType(&FieldType::R1P); // convert fElem from FConst to R1P_Elem
        fElem
            .elem_
            .borrow()
            .try_as_elem_ref()
            .unwrap()
            .elem_
            .clone()
    }
}

impl<FieldT: FieldTConfig> ConvertConfig<&FElem, FieldT> for GLA {
    fn convert(fElem: &FElem) -> FieldT {
        // using gadgetlib2::R1P_Elem;
        // fElem.promoteToFieldType(&FieldType::R1P); // convert fElem from FConst to R1P_Elem
        // fElem
        //     .elem_
        //     .borrow()
        //     .try_as_elem_ref()
        //     .unwrap()
        //     .elem_
        //     .clone()
        FieldT::default()
    }
}

// impl PartialEq <&linear_term_t> for linear_combination_t {
//     #[inline]
//     fn eq(&self, other: &linear_term_t) -> bool {
//         self.0.len() == 1 && &self.0[0] == other && self.1 == Fp::from(0i64)
//     }
// }

// bool operator==(lhs:linear_combination_t&,
//     rhs:&linear_term_t) {
//     return lhs.first.len() == 1 &&
//         lhs.first.at(0) == rhs &&
//         lhs.second == Fp(0);
// }

// }
