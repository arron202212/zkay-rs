// Declaration of interfaces for (single and double) word gadgets.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::dual_variable_gadget;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::tinyram_protoboard;
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::protoboard;
use crate::relations::FieldTConfig;
use crate::relations::variable::variable;
use rccell::RcCell;
use std::marker::PhantomData;
//Holds both binary and field representaton of a word.

#[derive(Clone, Default)]
pub struct word_variable_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type word_variable_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    dual_variable_gadget<FieldT, tinyram_protoboard<FieldT>, word_variable_gadget<FieldT>>,
>;
impl<FieldT: FieldTConfig> word_variable_gadget<FieldT> {
    // : public dual_variable_gadget<FieldT>
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        annotation_prefix: String,
    ) -> word_variable_gadgets<FieldT> {
        let w = pb.borrow().t.ap.w;
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new(
            pb,
            w,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
    pub fn new_with_width(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        w: usize,
        annotation_prefix: String,
    ) -> word_variable_gadgets<FieldT> {
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new(
            pb,
            w,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
    pub fn new_with_bits(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        annotation_prefix: String,
    ) -> word_variable_gadgets<FieldT> {
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new_with_bits(
            pb,
            bits,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
    pub fn new_with_packed(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        packed: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> word_variable_gadgets<FieldT> {
        let w = pb.borrow().t.ap.w;
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new_with_width(
            pb,
            packed,
            w,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

pub type doubleword_variable_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    dual_variable_gadget<FieldT, tinyram_protoboard<FieldT>, doubleword_variable_gadget<FieldT>>,
>;
// Holds both binary and field representaton of a double word.
#[derive(Clone, Default)]
pub struct doubleword_variable_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
impl<FieldT: FieldTConfig> doubleword_variable_gadget<FieldT> {
    // : public dual_variable_gadget<FieldT>
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        annotation_prefix: String,
    ) -> doubleword_variable_gadgets<FieldT> {
        let w = pb.borrow().t.ap.w;
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new(
            pb,
            2 * w,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
    pub fn new_with_bits(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        annotation_prefix: String,
    ) -> doubleword_variable_gadgets<FieldT> {
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new_with_bits(
            pb,
            bits,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
    pub fn new_with_packed(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        packed: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> doubleword_variable_gadgets<FieldT> {
        let w = pb.borrow().t.ap.w;
        dual_variable_gadget::<FieldT, tinyram_protoboard<FieldT>, Self>::new_with_width(
            pb,
            packed,
            2 * w,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}
