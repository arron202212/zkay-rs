// Declaration of interfaces for the TinyRAM ALU arithmetic gadgets.

// These gadget check the correct execution of arithmetic TinyRAM instructions.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    DefaultDualVariableGadget, comparison_gadget, disjunction_gadget, dual_variable_gadget,
    dual_variable_gadgets, generate_boolean_r1cs_constraint, inner_product_gadget,
    loose_multiplexing_gadget, packing_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::ArithmeticGadgetConfig;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::word_variable_gadgets;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::{
    tinyram_comparison_gadget, tinyram_disjunction_gadget, tinyram_loose_multiplexing_gadget,
    tinyram_packing_gadget,
};
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;

use crate::relations::ram_computations::rams::tinyram::tinyram_aux::tinyram_opcode;
use crate::relations::ram_computations::rams::tinyram::tinyram_aux::{
    generate_tinyram_prelude, tinyram_architecture_params, tinyram_program,
};
use crate::relations::ram_computations::rams::{
    ram_params::ArchitectureParamsTypeConfig,
    tinyram::tinyram_aux::{
        tinyram_opcodes_control_flow, tinyram_opcodes_register, tinyram_opcodes_stall,
    },
};
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use ffec::common::profiling::print_time;
use ffec::common::utils::{from_twos_complement, log2, to_twos_complement};
use rccell::RcCell;
use std::marker::PhantomData;

/* arithmetic gadgets */
#[derive(Clone, Default)]
pub struct ALU_arithmetic_gadget<FieldT: FieldTConfig, T: Default + Clone> {
    // : public tinyram_standard_gadget<FieldT>
    pub opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pub desval: word_variable_gadgets<FieldT>,
    pub arg1val: word_variable_gadgets<FieldT>,
    pub arg2val: word_variable_gadgets<FieldT>,
    pub flag: variable<FieldT, pb_variable>,
    pub result: variable<FieldT, pb_variable>,
    pub result_flag: variable<FieldT, pb_variable>,
    pub t: T,
}

pub type ALU_arithmetic_gadgets<FieldT, T: Default + Clone> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, ALU_arithmetic_gadget<FieldT, T>>>,
>;
impl<FieldT: FieldTConfig, T: Default + Clone> SubTinyRamGadgetConfig
    for ALU_arithmetic_gadget<FieldT, T>
{
}
impl<FieldT: FieldTConfig, T: Default + Clone> ALU_arithmetic_gadget<FieldT, T> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
        t: T,
    ) -> ALU_arithmetic_gadgets<FieldT, T> {
        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                opcode_indicators,
                desval,
                arg1val,
                arg2val,
                flag,
                result,
                result_flag,
                t,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_and_gadget<FieldT: FieldTConfig> {
    // : public ALU_arithmetic_gadget<FieldT>
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    not_all_zeros: RcCell<tinyram_disjunction_gadget<FieldT>>,
    not_all_zeros_result: variable<FieldT, pb_variable>,
}

pub type ALU_and_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_and_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_and_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_and_gadgets<FieldT> {
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_bit", annotation_prefix),
        );
        let mut not_all_zeros_result = variable::<FieldT, pb_variable>::default();
        not_all_zeros_result.allocate(&pb, format!("{} not_all_zeros_result", annotation_prefix));

        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        let not_all_zeros = RcCell::new(
            disjunction_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word.clone().into(),
                not_all_zeros_result.clone().into(),
                format!("{}not_all_zeros", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                not_all_zeros_result,
                pack_result,
                not_all_zeros,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_or_gadget<FieldT: FieldTConfig> {
    // : public ALU_arithmetic_gadget<FieldT>
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    not_all_zeros: RcCell<tinyram_disjunction_gadget<FieldT>>,
    not_all_zeros_result: variable<FieldT, pb_variable>,
}
pub type ALU_or_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_or_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_or_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_or_gadgets<FieldT> {
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_bit", annotation_prefix),
        );
        let mut not_all_zeros_result = variable::<FieldT, pb_variable>::default();
        not_all_zeros_result.allocate(&pb, format!("{} not_all_zeros_result", annotation_prefix));

        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        let not_all_zeros = RcCell::new(
            disjunction_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word.clone().into(),
                not_all_zeros_result.clone().into(),
                format!("{}not_all_zeros", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                not_all_zeros_result,
                pack_result,
                not_all_zeros,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_xor_gadget<FieldT: FieldTConfig> {
    // : public ALU_arithmetic_gadget<FieldT>
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    not_all_zeros: RcCell<tinyram_disjunction_gadget<FieldT>>,
    not_all_zeros_result: variable<FieldT, pb_variable>,
}
pub type ALU_xor_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_xor_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_xor_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_xor_gadgets<FieldT> {
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_bit", annotation_prefix),
        );
        let mut not_all_zeros_result = variable::<FieldT, pb_variable>::default();
        not_all_zeros_result.allocate(&pb, format!("{} not_all_zeros_result", annotation_prefix));

        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        let not_all_zeros = RcCell::new(
            disjunction_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word.clone().into(),
                not_all_zeros_result.clone().into(),
                format!("{}not_all_zeros", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                not_all_zeros_result,
                pack_result,
                not_all_zeros,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_not_gadget<FieldT: FieldTConfig> {
    /* we do bitwise not, because we need to compute flag */
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    not_all_zeros: RcCell<tinyram_disjunction_gadget<FieldT>>,
    not_all_zeros_result: variable<FieldT, pb_variable>,
}
pub type ALU_not_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_not_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_not_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_not_gadgets<FieldT> {
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_bit", annotation_prefix),
        );
        let mut not_all_zeros_result = variable::<FieldT, pb_variable>::default();
        not_all_zeros_result.allocate(&pb, format!("{} not_all_zeros_result", annotation_prefix));

        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        let not_all_zeros = RcCell::new(
            disjunction_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word.clone().into(),
                not_all_zeros_result.clone().into(),
                format!("{}not_all_zeros", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                not_all_zeros_result,
                pack_result,
                not_all_zeros,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_add_gadget<FieldT: FieldTConfig> {
    addition_result: variable<FieldT, pb_variable>,
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    res_word_and_flag: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    unpack_addition: RcCell<tinyram_packing_gadget<FieldT>>,
}
pub type ALU_add_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_add_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_add_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_add_gadgets<FieldT> {
        let mut addition_result = variable::<FieldT, pb_variable>::default();
        addition_result.allocate(&pb, format!("{} addition_result", annotation_prefix));
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_word", annotation_prefix),
        );

        let mut res_word_and_flag = res_word.clone();
        res_word_and_flag.contents.push(result_flag.clone());

        let unpack_addition =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word_and_flag.clone().into(),
                addition_result.clone().into(),
                format!("{} unpack_addition", annotation_prefix),
            ));
        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                addition_result,
                pack_result,
                res_word_and_flag,
                unpack_addition,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_sub_gadget<FieldT: FieldTConfig> {
    intermediate_result: variable<FieldT, pb_variable>,
    negated_flag: variable<FieldT, pb_variable>,
    res_word: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    res_word_and_negated_flag: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    pack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    unpack_intermediate: RcCell<tinyram_packing_gadget<FieldT>>,
}
pub type ALU_sub_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_sub_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_sub_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_sub_gadgets<FieldT> {
        let mut intermediate_result = variable::<FieldT, pb_variable>::default();
        intermediate_result.allocate(&pb, format!("{} intermediate_result", annotation_prefix));
        let mut negated_flag = variable::<FieldT, pb_variable>::default();
        negated_flag.allocate(&pb, format!("{} negated_flag", annotation_prefix));
        let mut res_word = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        res_word.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} res_word", annotation_prefix),
        );

        let mut res_word_and_negated_flag = res_word.clone();
        res_word_and_negated_flag
            .contents
            .push(negated_flag.clone());

        let unpack_intermediate =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                res_word_and_negated_flag.clone().into(),
                intermediate_result.clone().into(),
                format!("{} unpack_intermediate", annotation_prefix),
            ));
        let pack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            res_word.clone().into(),
            result.clone().into(),
            format!("{} pack_result", annotation_prefix),
        ));
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self {
                res_word,
                intermediate_result,
                pack_result,
                negated_flag,
                res_word_and_negated_flag,
                unpack_intermediate,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_mov_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type ALU_mov_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_mov_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_mov_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_mov_gadgets<FieldT> {
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_cmov_gadget<FieldT: FieldTConfig> {
    _t: PhantomData<FieldT>,
}
pub type ALU_cmov_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_cmov_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_cmov_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_cmov_gadgets<FieldT> {
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            result,
            result_flag,
            annotation_prefix,
            Self { _t: PhantomData },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_cmp_gadget<FieldT: FieldTConfig> {
    comparator: tinyram_comparison_gadget<FieldT>,
    cmpe_result: variable<FieldT, pb_variable>,
    cmpe_result_flag: variable<FieldT, pb_variable>,
    cmpa_result: variable<FieldT, pb_variable>,
    cmpa_result_flag: variable<FieldT, pb_variable>,
    cmpae_result: variable<FieldT, pb_variable>,
    cmpae_result_flag: variable<FieldT, pb_variable>,
}
pub type ALU_cmp_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_cmp_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_cmp_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        cmpe_result: variable<FieldT, pb_variable>,
        cmpe_result_flag: variable<FieldT, pb_variable>,
        cmpa_result: variable<FieldT, pb_variable>,
        cmpa_result_flag: variable<FieldT, pb_variable>,
        cmpae_result: variable<FieldT, pb_variable>,
        cmpae_result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_cmp_gadgets<FieldT> {
        let comparator = comparison_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            pb.borrow().t.ap.w,
            arg2val.t.packed.clone().into(),
            arg1val.t.packed.clone().into(),
            cmpa_result_flag.clone().into(),
            cmpae_result_flag.clone().into(),
            format!("{} comparator", annotation_prefix),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            cmpa_result.clone(),
            cmpa_result_flag.clone(),
            annotation_prefix,
            Self {
                comparator,
                cmpe_result,
                cmpe_result_flag,
                cmpa_result,
                cmpa_result_flag,
                cmpae_result,
                cmpae_result_flag,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_cmps_gadget<FieldT: FieldTConfig> {
    negated_arg1val_sign: variable<FieldT, pb_variable>,
    negated_arg2val_sign: variable<FieldT, pb_variable>,
    modified_arg1: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    modified_arg2: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    packed_modified_arg1: variable<FieldT, pb_variable>,
    packed_modified_arg2: variable<FieldT, pb_variable>,
    pack_modified_arg1: RcCell<tinyram_packing_gadget<FieldT>>,
    pack_modified_arg2: RcCell<tinyram_packing_gadget<FieldT>>,
    comparator: RcCell<tinyram_comparison_gadget<FieldT>>,
    cmpg_result: variable<FieldT, pb_variable>,
    cmpg_result_flag: variable<FieldT, pb_variable>,
    cmpge_result: variable<FieldT, pb_variable>,
    cmpge_result_flag: variable<FieldT, pb_variable>,
}
pub type ALU_cmps_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_cmps_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_cmps_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        cmpg_result: variable<FieldT, pb_variable>,
        cmpg_result_flag: variable<FieldT, pb_variable>,
        cmpge_result: variable<FieldT, pb_variable>,
        cmpge_result_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_cmps_gadgets<FieldT> {
        let mut negated_arg1val_sign = variable::<FieldT, pb_variable>::default();
        negated_arg1val_sign.allocate(&pb, format!("{} negated_arg1val_sign", annotation_prefix));
        let mut negated_arg2val_sign = variable::<FieldT, pb_variable>::default();
        negated_arg2val_sign.allocate(&pb, format!("{} negated_arg2val_sign", annotation_prefix));
        let mut modified_arg1 = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            arg1val.t.bits.contents[..arg1val.t.bits.contents.len() - 1].to_vec(),
        );
        modified_arg1.contents.push(negated_arg1val_sign.clone());
        let mut modified_arg2 = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            arg2val.t.bits.contents[..arg1val.t.bits.contents.len() - 1].to_vec(),
        );
        modified_arg2.contents.push(negated_arg2val_sign.clone());
        let mut packed_modified_arg1 = variable::<FieldT, pb_variable>::default();
        packed_modified_arg1.allocate(&pb, format!("{} packed_modified_arg1", annotation_prefix));
        let mut packed_modified_arg2 = variable::<FieldT, pb_variable>::default();
        packed_modified_arg2.allocate(&pb, format!("{} packed_modified_arg2", annotation_prefix));
        let pack_modified_arg1 =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                modified_arg1.clone().into(),
                packed_modified_arg1.clone().into(),
                format!("{} pack_modified_arg1", annotation_prefix),
            ));
        let pack_modified_arg2 =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                modified_arg2.clone().into(),
                packed_modified_arg2.clone().into(),
                format!("{} pack_modified_arg2", annotation_prefix),
            ));
        let comparator = RcCell::new(
            comparison_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                pb.borrow().t.ap.w,
                packed_modified_arg2.clone().into(),
                packed_modified_arg1.clone().into(),
                cmpg_result_flag.clone().into(),
                cmpge_result_flag.clone().into(),
                format!("{} comparator", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            cmpg_result.clone(),
            cmpg_result_flag.clone(),
            annotation_prefix,
            Self {
                negated_arg1val_sign,
                negated_arg2val_sign,
                modified_arg1,
                modified_arg2,
                packed_modified_arg1,
                packed_modified_arg2,
                pack_modified_arg1,
                pack_modified_arg2,
                comparator,
                cmpg_result,
                cmpg_result_flag,
                cmpge_result,
                cmpge_result_flag,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_umul_gadget<FieldT: FieldTConfig> {
    mul_result: word_variable_gadgets<FieldT>,
    mull_bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    umulh_bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    result_flag: variable<FieldT, pb_variable>,
    pack_mull_result: RcCell<tinyram_packing_gadget<FieldT>>,
    pack_umulh_result: RcCell<tinyram_packing_gadget<FieldT>>,
    compute_flag: RcCell<tinyram_disjunction_gadget<FieldT>>,
    mull_result: variable<FieldT, pb_variable>,
    mull_flag: variable<FieldT, pb_variable>,
    umulh_result: variable<FieldT, pb_variable>,
    umulh_flag: variable<FieldT, pb_variable>,
}
pub type ALU_umul_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_umul_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_umul_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        mull_result: variable<FieldT, pb_variable>,
        mull_flag: variable<FieldT, pb_variable>,
        umulh_result: variable<FieldT, pb_variable>,
        umulh_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_umul_gadgets<FieldT> {
        let mul_result = word_variable_gadget::<FieldT>::new_with_width(
            pb.clone(),
            2 * pb.borrow().t.ap.w,
            format!("{} mul_result", annotation_prefix),
        );
        let mull_bits = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            mul_result.t.bits.contents[..pb.borrow().t.ap.w].to_vec(),
        );
        let umulh_bits = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            mul_result.t.bits.contents[pb.borrow().t.ap.w..2 * pb.borrow().t.ap.w].to_vec(),
        );
        let pack_mull_result =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                mull_bits.clone().into(),
                mull_result.clone().into(),
                format!("{} pack_mull_result", annotation_prefix),
            ));
        let pack_umulh_result =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                umulh_bits.clone().into(),
                umulh_result.clone().into(),
                format!("{} pack_umulh_result", annotation_prefix),
            ));
        let mut result_flag = variable::<FieldT, pb_variable>::default();
        result_flag.allocate(&pb, format!("{} result_flag", annotation_prefix));
        let compute_flag = RcCell::new(
            disjunction_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                umulh_bits.clone().into(),
                result_flag.clone().into(),
                format!("{} compute_flag", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            mull_result.clone(),
            mull_flag.clone(),
            annotation_prefix,
            Self {
                mul_result,
                mull_bits,
                umulh_bits,
                result_flag,
                pack_mull_result,
                pack_umulh_result,
                compute_flag,
                mull_result,
                mull_flag,
                umulh_result,
                umulh_flag,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_smul_gadget<FieldT: FieldTConfig> {
    mul_result: word_variable_gadgets<FieldT>,
    smulh_bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    top: variable<FieldT, pb_variable>,
    pack_top: RcCell<tinyram_packing_gadget<FieldT>>,
    is_top_empty_aux: variable<FieldT, pb_variable>,
    is_top_empty: variable<FieldT, pb_variable>,
    is_top_full_aux: variable<FieldT, pb_variable>,
    is_top_full: variable<FieldT, pb_variable>,
    result_flag: variable<FieldT, pb_variable>,
    pack_smulh_result: RcCell<tinyram_packing_gadget<FieldT>>,
    smulh_result: variable<FieldT, pb_variable>,
    smulh_flag: variable<FieldT, pb_variable>,
}
pub type ALU_smul_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_smul_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_smul_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        smulh_result: variable<FieldT, pb_variable>,
        smulh_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_smul_gadgets<FieldT> {
        let mul_result = word_variable_gadget::<FieldT>::new_with_width(
            pb.clone(),
            2 * pb.borrow().t.ap.w + 1,
            format!("{} mul_result", annotation_prefix),
        ); /* see witness map for explanation for 2w+1 */
        let smulh_bits = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
            mul_result.t.bits.contents[pb.borrow().t.ap.w..2 * pb.borrow().t.ap.w].to_vec(),
        );
        let pack_smulh_result =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                smulh_bits.clone().into(),
                smulh_result.clone().into(),
                format!("{} pack_smulh_result", annotation_prefix),
            ));
        let mut top = variable::<FieldT, pb_variable>::default();
        top.allocate(&pb, format!("{} top", annotation_prefix));
        let pack_top = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                mul_result.t.bits.contents[pb.borrow().t.ap.w - 1..2 * pb.borrow().t.ap.w].to_vec(),
            )
            .into(),
            top.clone().into(),
            format!("{} pack_top", annotation_prefix),
        ));
        let mut is_top_empty = variable::<FieldT, pb_variable>::default();
        is_top_empty.allocate(&pb, format!("{} is_top_empty", annotation_prefix));
        let mut is_top_empty_aux = variable::<FieldT, pb_variable>::default();
        is_top_empty_aux.allocate(&pb, format!("{} is_top_empty_aux", annotation_prefix));
        let mut is_top_full = variable::<FieldT, pb_variable>::default();
        is_top_full.allocate(&pb, format!("{} is_top_full", annotation_prefix));
        let mut is_top_full_aux = variable::<FieldT, pb_variable>::default();
        is_top_full_aux.allocate(&pb, format!("{} is_top_full_aux", annotation_prefix));
        let mut result_flag = variable::<FieldT, pb_variable>::default();
        result_flag.allocate(&pb, format!("{} result_flag", annotation_prefix));
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            smulh_result.clone(),
            smulh_flag.clone(),
            annotation_prefix,
            Self {
                mul_result,
                smulh_bits,
                top,
                pack_top,
                is_top_empty_aux,
                is_top_empty,
                is_top_full_aux,
                is_top_full,
                result_flag,
                pack_smulh_result,
                smulh_result,
                smulh_flag,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_divmod_gadget<FieldT: FieldTConfig> {
    /*
      <<<<<<< Updated upstream
      B * q + r = A_aux = A * B_nonzero
      q * (1-B_nonzero) = 0
      A<B_gadget<FieldT>(r < B, less=B_nonzero, leq=ONE)
      =======
      B * q + r = A

      r <= B
      >>>>>>> Stashed changes
    */
    B_inv: variable<FieldT, pb_variable>,
    B_nonzero: variable<FieldT, pb_variable>,
    A_aux: variable<FieldT, pb_variable>,
    r_less_B: RcCell<tinyram_comparison_gadget<FieldT>>,
    udiv_result: variable<FieldT, pb_variable>,
    udiv_flag: variable<FieldT, pb_variable>,
    umod_result: variable<FieldT, pb_variable>,
    umod_flag: variable<FieldT, pb_variable>,
}
pub type ALU_divmod_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_divmod_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_divmod_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        udiv_result: variable<FieldT, pb_variable>,
        udiv_flag: variable<FieldT, pb_variable>,
        umod_result: variable<FieldT, pb_variable>,
        umod_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_divmod_gadgets<FieldT> {
        let mut B_inv = variable::<FieldT, pb_variable>::default();
        B_inv.allocate(&pb, format!("{} B_inv", annotation_prefix));
        let mut B_nonzero = variable::<FieldT, pb_variable>::default();
        B_nonzero.allocate(&pb, format!("{} B_nonzer", annotation_prefix));
        let mut A_aux = variable::<FieldT, pb_variable>::default();
        A_aux.allocate(&pb, format!("{} A_aux", annotation_prefix));
        let r_less_B = RcCell::new(
            comparison_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                pb.borrow().t.ap.w,
                umod_result.clone().into(),
                arg2val.t.packed.clone().into(),
                B_nonzero.clone().into(),
                ONE.clone().into(),
                format!("{} r_less_B", annotation_prefix),
            ),
        );
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            udiv_result.clone(),
            udiv_flag.clone(),
            annotation_prefix,
            Self {
                B_inv,
                B_nonzero,
                A_aux,
                r_less_B,
                udiv_result,
                udiv_flag,
                umod_result,
                umod_flag,
            },
        )
    }
}

#[derive(Clone, Default)]
pub struct ALU_shr_shl_gadget<FieldT: FieldTConfig> {
    reversed_input: variable<FieldT, pb_variable>,
    pack_reversed_input: RcCell<tinyram_packing_gadget<FieldT>>,
    barrel_right_internal: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    shifted_out_bits: Vec<pb_variable_array<FieldT, tinyram_protoboard<FieldT>>>,
    is_oversize_shift: variable<FieldT, pb_variable>,
    check_oversize_shift: RcCell<tinyram_disjunction_gadget<FieldT>>,
    result: variable<FieldT, pb_variable>,
    result_bits: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    unpack_result: RcCell<tinyram_packing_gadget<FieldT>>,
    reversed_result: variable<FieldT, pb_variable>,
    pack_reversed_result: RcCell<tinyram_packing_gadget<FieldT>>,
    shr_result: variable<FieldT, pb_variable>,
    shr_flag: variable<FieldT, pb_variable>,
    shl_result: variable<FieldT, pb_variable>,
    shl_flag: variable<FieldT, pb_variable>,
    logw: usize,
}
pub type ALU_shr_shl_gadgets<FieldT> = ALU_arithmetic_gadgets<FieldT, ALU_shr_shl_gadget<FieldT>>;
impl<FieldT: FieldTConfig> ALU_shr_shl_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        opcode_indicators: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
        desval: word_variable_gadgets<FieldT>,
        arg1val: word_variable_gadgets<FieldT>,
        arg2val: word_variable_gadgets<FieldT>,
        flag: variable<FieldT, pb_variable>,
        shr_result: variable<FieldT, pb_variable>,
        shr_flag: variable<FieldT, pb_variable>,
        shl_result: variable<FieldT, pb_variable>,
        shl_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> ALU_shr_shl_gadgets<FieldT> {
        let logw = log2(pb.borrow().t.ap.w);
        let mut reversed_input = variable::<FieldT, pb_variable>::default();
        reversed_input.allocate(&pb, format!("{} reversed_input", annotation_prefix));
        let pack_reversed_input =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    arg1val.t.bits.contents.iter().rev().cloned().collect(),
                )
                .into(),
                reversed_input.clone().into(),
                format!("{} pack_reversed_input", annotation_prefix),
            ));
        let mut barrel_right_internal =
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        barrel_right_internal.allocate(
            &pb,
            logw + 1,
            format!("{} barrel_right_internal", annotation_prefix),
        );

        let mut shifted_out_bits =
            vec![pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default(); logw];
        for i in 0..logw {
            shifted_out_bits[i].allocate(
                &pb,
                1usize << i,
                format!("{} shifted_out_bits_{}", annotation_prefix, i),
            );
        }
        let mut is_oversize_shift = variable::<FieldT, pb_variable>::default();
        is_oversize_shift.allocate(&pb, format!("{} is_oversize_shift", annotation_prefix));
        let check_oversize_shift = RcCell::new(disjunction_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                arg2val.t.bits.contents[logw..].to_vec(),
            ),
            is_oversize_shift.clone().into(),
            format!("{} check_oversize_shift", annotation_prefix),
        ));
        let mut result = variable::<FieldT, pb_variable>::default();
        result.allocate(&pb, format!("{} result", annotation_prefix));
        let mut result_bits = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        result_bits.allocate(
            &pb,
            pb.borrow().t.ap.w,
            format!("{} result_bits", annotation_prefix),
        );
        let unpack_result = RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
            pb.clone(),
            result_bits.clone().into(),
            result.clone().into(), //barrel_right_internal[logw],
            format!("{} unpack_result", annotation_prefix),
        ));
        let mut reversed_result = variable::<FieldT, pb_variable>::default();
        reversed_result.allocate(&pb, format!("{} reversed_result", annotation_prefix));
        let pack_reversed_result =
            RcCell::new(packing_gadget::<FieldT, tinyram_protoboard<FieldT>>::new(
                pb.clone(),
                pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    result_bits.contents.iter().rev().cloned().collect(),
                )
                .into(),
                reversed_result.clone().into(),
                format!("{} pack_reversed_result", annotation_prefix),
            ));
        ALU_arithmetic_gadget::<FieldT, Self>::new(
            pb,
            opcode_indicators,
            desval,
            arg1val,
            arg2val,
            flag,
            shr_result.clone(),
            shr_flag.clone(),
            annotation_prefix,
            Self {
                reversed_input,
                pack_reversed_input,
                barrel_right_internal,
                shifted_out_bits,
                is_oversize_shift,
                check_oversize_shift,
                result,
                result_bits,
                unpack_result,
                reversed_result,
                pack_reversed_result,
                shr_result,
                shr_flag,
                shl_result,
                shl_flag,
                logw,
            },
        )
    }
}

/* the code here is full of template lambda magic, but it is better to
have limited presence of such code than to have code duplication in
testing functions, which basically do the same thing: brute force
the range of inputs which different success predicates */

//
type initializer_fn<T, FieldT: FieldTConfig> = fn(
    &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>, // pb
    &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,  // opcode_indicators
    &word_variable_gadgets<FieldT>,                          // desval
    &word_variable_gadgets<FieldT>,                          // arg1val
    &word_variable_gadgets<FieldT>,                          // arg2val
    &variable<FieldT, pb_variable>,                          // flag
    &variable<FieldT, pb_variable>,                          // result
    &variable<FieldT, pb_variable>,                          // result_flag
) -> T;

pub fn brute_force_arithmetic_gadget<
    FieldT: FieldTConfig,
    T: ArithmeticGadgetConfig<FieldT>,
    ResFn,
    FlagFn,
>(
    w: usize,
    opcode: usize,
    initializer: initializer_fn<T, FieldT>,
    res_function: ResFn,
    flag_function: FlagFn,
) where
    ResFn: Fn(usize, bool, usize, usize) -> usize,
    FlagFn: Fn(usize, bool, usize, usize) -> bool,
{
    /* parameters for res_function and flag_function are both desval, flag, arg1val, arg2val */

    print!("testing on all {} bit inputs\n", w);

    let mut ap = tinyram_architecture_params::new(w, 16);
    let mut P = tinyram_program::default();
    P.instructions = generate_tinyram_prelude(&ap);
    let mut pb = RcCell::new(tinyram_protoboard::<FieldT>::new(ap.clone())); //, P.len(), 0, 10);

    let mut opcode_indicators = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
    opcode_indicators.allocate(&pb, 1usize << ap.opcode_width(), "opcode_indicators");
    for i in 0..1usize << ap.opcode_width() {
        *pb.borrow_mut().val_ref(&opcode_indicators[i]) = if i == opcode {
            FieldT::one()
        } else {
            FieldT::zero()
        }
    }

    let mut desval = word_variable_gadget::<FieldT>::new(pb.clone(), "desval".to_owned());
    desval.generate_r1cs_constraints(true);
    let mut arg1val = word_variable_gadget::<FieldT>::new(pb.clone(), "arg1val".to_owned());
    arg1val.generate_r1cs_constraints(true);
    let mut arg2val = word_variable_gadget::<FieldT>::new(pb.clone(), "arg2val".to_owned());
    arg2val.generate_r1cs_constraints(true);
    let mut flag = variable::<FieldT, pb_variable>::default();
    flag.allocate(&pb, "flag".to_owned());
    let mut result = variable::<FieldT, pb_variable>::default();
    result.allocate(&pb, "result".to_owned());
    let mut result_flag = variable::<FieldT, pb_variable>::default();
    result_flag.allocate(&pb, "result_flag".to_owned());

    let mut g = T::default();
    g = initializer(
        &pb,
        &opcode_indicators,
        &desval,
        &arg1val,
        &arg2val,
        &flag,
        &result,
        &result_flag,
    );
    g.generate_r1cs_constraints();

    for des in 0..(1usize << w) {
        *pb.borrow_mut().val_ref(&desval.t.packed) = FieldT::from(des);
        desval.generate_r1cs_witness_from_packed();

        for f in 0..=1 {
            *pb.borrow_mut().val_ref(&flag) = if f != 0 {
                FieldT::one()
            } else {
                FieldT::zero()
            };

            for arg1 in 0..(1usize << w) {
                *pb.borrow_mut().val_ref(&arg1val.t.packed) = FieldT::from(arg1);
                arg1val.generate_r1cs_witness_from_packed();

                for arg2 in 0..(1usize << w) {
                    *pb.borrow_mut().val_ref(&arg2val.t.packed) = FieldT::from(arg2);
                    arg2val.generate_r1cs_witness_from_packed();

                    let res = res_function(des, f != 0, arg1, arg2);
                    let res_f = flag_function(des, f != 0, arg1, arg2);
                    // #ifdef DEBUG
                    print!(
                        "with the following parameters: flag = {}
                           , desval = {} ({})
                           , arg1val = {} ({})
                           , arg2val = {} ({})
                           . expected result: {} ({}), expected flag: {}\n",
                        f,
                        des,
                        from_twos_complement(des, w),
                        arg1,
                        from_twos_complement(arg1, w),
                        arg2,
                        from_twos_complement(arg2, w),
                        res,
                        from_twos_complement(res, w),
                        res_f
                    );

                    g.generate_r1cs_witness();
                    // #ifdef DEBUG
                    print!("result: ");
                    pb.borrow().val(&result).print();
                    print!("flag: ");
                    pb.borrow().val(&result_flag).print();

                    assert!(pb.borrow().is_satisfied());
                    assert!(pb.borrow().val(&result) == FieldT::from(res));
                    assert!(
                        pb.borrow().val(&result_flag)
                            == (if res_f { FieldT::one() } else { FieldT::zero() })
                    );
                }
            }
        }
    }
}

/* and */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_and_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                    vec![self.t.t.t.arg1val.t.bits[i].clone().into()],
                    vec![self.t.t.t.arg2val.t.bits[i].clone().into()],
                    vec![self.t.t.t.t.res_word[i].clone().into()],
                ),
                format!("{} res_word_{}", self.annotation_prefix, i),
            );
        }

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .not_all_zeros
            .borrow()
            .generate_r1cs_constraints();

        /* result_flag = 1 - not_all_zeros = result is 0^w */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.not_all_zeros_result.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            let b1 = self.pb.borrow().val(&self.t.t.t.arg1val.t.bits[i]) == FieldT::one();
            let b2 = self.pb.borrow().val(&self.t.t.t.arg2val.t.bits[i]) == FieldT::one();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.res_word[i]) = if b1 && b2 {
                FieldT::one()
            } else {
                FieldT::zero()
            }
        }

        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t.t.t.t.not_all_zeros.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            FieldT::one() - self.pb.borrow().val(&self.t.t.t.t.not_all_zeros_result);
    }
}

pub fn test_ALU_and_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting and test");
    brute_force_arithmetic_gadget::<FieldT, ALU_and_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_AND.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_and_gadgets<FieldT> {
            return ALU_and_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_and_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return x & y;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x & y) == 0;
        },
    );
    print_time("and tests successful");
}

/* or */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_or_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                    vec![
                        variable::<FieldT, pb_variable>::from(ONE).into(),
                        (self.t.t.t.arg1val.t.bits[i].clone() * (-1).into()).into(),
                    ],
                    vec![
                        variable::<FieldT, pb_variable>::from(ONE).into(),
                        (self.t.t.t.arg2val.t.bits[i].clone() * (-1).into()).into(),
                    ],
                    vec![
                        variable::<FieldT, pb_variable>::from(ONE).into(),
                        (self.t.t.t.t.res_word[i].clone() * (-1).into()).into(),
                    ],
                ),
                format!("{} res_word_{}", self.annotation_prefix, i),
            );
        }

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .not_all_zeros
            .borrow()
            .generate_r1cs_constraints();

        /* result_flag = 1 - not_all_zeros = result is 0^w */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.not_all_zeros_result.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            let b1 = self.pb.borrow().val(&self.t.t.t.arg1val.t.bits[i]) == FieldT::one();
            let b2 = self.pb.borrow().val(&self.t.t.t.arg2val.t.bits[i]) == FieldT::one();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.res_word[i]) = if b1 || b2 {
                FieldT::one()
            } else {
                FieldT::zero()
            }
        }

        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t.t.t.t.not_all_zeros.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            FieldT::one() - self.pb.borrow().val(&self.t.t.t.t.not_all_zeros_result);
    }
}
pub fn test_ALU_or_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting or test");
    brute_force_arithmetic_gadget::<FieldT, ALU_or_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_OR.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_or_gadgets<FieldT> {
            return ALU_or_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_or_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return x | y;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x | y) == 0;
        },
    );
    print_time("or tests successful");
}

/* xor */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_xor_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            /* a = b ^ c <=> a = b + c - 2*b*c, (2*b)*c = b+c - a */
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                    vec![(self.t.t.t.arg1val.t.bits[i].clone() * 2.into()).into()],
                    vec![self.t.t.t.arg2val.t.bits[i].clone().into()],
                    vec![
                        self.t.t.t.arg1val.t.bits[i].clone().into(),
                        self.t.t.t.arg2val.t.bits[i].clone().into(),
                        (self.t.t.t.t.res_word[i].clone() * (-1).into()).into(),
                    ],
                ),
                format!("{} res_word_{}", self.annotation_prefix, i),
            );
        }

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .not_all_zeros
            .borrow()
            .generate_r1cs_constraints();

        /* result_flag = 1 - not_all_zeros = result is 0^w */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.not_all_zeros_result.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            let b1 = self.pb.borrow().val(&self.t.t.t.arg1val.t.bits[i]) == FieldT::one();
            let b2 = self.pb.borrow().val(&self.t.t.t.arg2val.t.bits[i]) == FieldT::one();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.res_word[i]) = if b1 ^ b2 {
                FieldT::one()
            } else {
                FieldT::zero()
            }
        }

        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t.t.t.t.not_all_zeros.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            FieldT::one() - self.pb.borrow().val(&self.t.t.t.t.not_all_zeros_result);
    }
}
pub fn test_ALU_xor_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting xor test");
    brute_force_arithmetic_gadget::<FieldT, ALU_xor_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_XOR.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_xor_gadgets<FieldT> {
            return ALU_xor_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_xor_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return x ^ y;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x ^ y) == 0;
        },
    );
    print_time("xor tests successful");
}

/* not */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_not_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                    vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                    vec![
                        variable::<FieldT, pb_variable>::from(ONE).into(),
                        (self.t.t.t.arg2val.t.bits[i].clone() * (-1).into()).into(),
                    ],
                    vec![self.t.t.t.t.res_word[i].clone().into()],
                ),
                format!("{} res_word_{}", self.annotation_prefix, i),
            );
        }

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .not_all_zeros
            .borrow()
            .generate_r1cs_constraints();

        /* result_flag = 1 - not_all_zeros = result is 0^w */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.not_all_zeros_result.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        for i in 0..self.pb.borrow().t.ap.w {
            let b2 = self.pb.borrow().val(&self.t.t.t.arg2val.t.bits[i]) == FieldT::one();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.res_word[i]) =
                if !b2 { FieldT::one() } else { FieldT::zero() }
        }

        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t.t.t.t.not_all_zeros.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            FieldT::one() - self.pb.borrow().val(&self.t.t.t.t.not_all_zeros_result);
    }
}

pub fn test_ALU_not_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting not test");
    brute_force_arithmetic_gadget::<FieldT, ALU_not_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_NOT.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_not_gadgets<FieldT> {
            return ALU_not_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_not_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (1usize << w) - 1 - y;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return ((1usize << w) - 1 - y) == 0;
        },
    );
    print_time("not tests successful");
}

/* add */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_add_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* addition_result = 1 * (arg1val + arg2val) */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    self.t.t.t.arg1val.t.packed.clone().into(),
                    self.t.t.t.arg2val.t.packed.clone().into(),
                ],
                vec![self.t.t.t.t.addition_result.clone().into()],
            ),
            format!("{} addition_result", self.annotation_prefix),
        );

        /* unpack into bits */
        self.t
            .t
            .t
            .t
            .unpack_addition
            .borrow()
            .generate_r1cs_constraints(true);

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.addition_result) =
            self.pb.borrow().val(&self.t.t.t.arg1val.t.packed)
                + self.pb.borrow().val(&self.t.t.t.arg2val.t.packed);
        self.t
            .t
            .t
            .t
            .unpack_addition
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}
pub fn test_ALU_add_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting add test");
    brute_force_arithmetic_gadget::<FieldT, ALU_add_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_ADD.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_add_gadgets<FieldT> {
            return ALU_add_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_add_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (x + y) % (1usize << w);
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x + y) >= (1usize << w);
        },
    );
    print_time("add tests successful");
}

/* sub */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_sub_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* intermediate_result = 2^w + (arg1val - arg2val) */
        let mut twoi = FieldT::one();

        let (mut a, mut b, mut c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );

        a.add_term(0, 1);
        for i in 0..self.pb.borrow().t.ap.w {
            twoi = twoi.clone() + twoi.clone();
        }
        b.add_term_with_field(0, twoi);
        b.add_term(self.t.t.t.arg1val.t.packed.index, 1);
        b.add_term(self.t.t.t.arg2val.t.packed.index, -1);
        c.add_term(self.t.t.t.t.intermediate_result.index, 1);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a, b, c),
            format!("{} main_constraint", self.annotation_prefix),
        );

        /* unpack into bits */
        self.t
            .t
            .t
            .t
            .unpack_intermediate
            .borrow()
            .generate_r1cs_constraints(true);

        /* generate result */
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.negated_flag.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        let mut twoi = FieldT::one();
        for i in 0..self.pb.borrow().t.ap.w {
            twoi = twoi.clone() + twoi.clone();
        }

        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.intermediate_result) = twoi
            + self.pb.borrow().val(&self.t.t.t.arg1val.t.packed)
            - self.pb.borrow().val(&self.t.t.t.arg2val.t.packed);
        self.t
            .t
            .t
            .t
            .unpack_intermediate
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .t
            .t
            .t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            FieldT::one() - self.pb.borrow().val(&self.t.t.t.t.negated_flag);
    }
}

pub fn test_ALU_sub_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting sub test");
    brute_force_arithmetic_gadget::<FieldT, ALU_sub_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_SUB.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_sub_gadgets<FieldT> {
            return ALU_sub_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_sub_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            let unsigned_result = ((1usize << w) + x - y) % (1usize << w);
            return unsigned_result;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            let msb = ((1usize << w) + x - y) >> w;
            return (msb == 0);
        },
    );
    print_time("sub tests successful");
}

/* mov */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_mov_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.arg2val.t.packed.clone().into()],
                vec![self.t.t.t.result.clone().into()],
            ),
            format!("{} mov_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.flag.clone().into()],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} mov_result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) =
            self.pb.borrow().val(&self.t.t.t.arg2val.t.packed);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            self.pb.borrow().val(&self.t.t.t.flag);
    }
}

pub fn test_ALU_mov_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting mov test");
    brute_force_arithmetic_gadget::<FieldT, ALU_mov_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_MOV.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_mov_gadgets<FieldT> {
            return ALU_mov_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_mov_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return y;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return f;
        },
    );
    print_time("mov tests successful");
}

/* cmov */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_cmov_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /*
          flag1 * arg2val + (1-flag1) * desval = result
          flag1 * (arg2val - desval) = result - desval
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.flag.clone().into()],
                vec![
                    self.t.t.t.arg2val.t.packed.clone().into(),
                    (self.t.t.t.desval.t.packed.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.result.clone().into(),
                    (self.t.t.t.desval.t.packed.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} cmov_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.flag.clone().into()],
                vec![self.t.t.t.result_flag.clone().into()],
            ),
            format!("{} cmov_result_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) =
            (if (self.pb.borrow().val(&self.t.t.t.flag) == FieldT::one()) {
                self.pb.borrow().val(&self.t.t.t.arg2val.t.packed)
            } else {
                self.pb.borrow().val(&self.t.t.t.desval.t.packed)
            });
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result_flag) =
            self.pb.borrow().val(&self.t.t.t.flag);
    }
}

pub fn test_ALU_cmov_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmov test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmov_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMOV.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmov_gadgets<FieldT> {
            return ALU_cmov_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_cmov_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize { return if f { y } else { des } },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return f;
        },
    );
    print_time("cmov tests successful");
}

/* unsigned comparison */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_cmp_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        self.t.t.t.t.comparator.generate_r1cs_constraints();
        /*
          cmpe = cmpae * (1-cmpa)
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.t.cmpae_result_flag.clone().into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.cmpa_result_flag.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.t.cmpe_result_flag.clone().into()],
            ),
            format!("{} cmpa_result_flag", self.annotation_prefix),
        );

        /* copy over results */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.desval.t.packed.clone().into()],
                vec![self.t.t.t.t.cmpe_result.clone().into()],
            ),
            format!("{} cmpe_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.desval.t.packed.clone().into()],
                vec![self.t.t.t.t.cmpa_result.clone().into()],
            ),
            format!("{} cmpa_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.desval.t.packed.clone().into()],
                vec![self.t.t.t.t.cmpae_result.clone().into()],
            ),
            format!("{} cmpae_result", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        self.t.t.t.t.comparator.generate_r1cs_witness();

        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpe_result) =
            self.pb.borrow().val(&self.t.t.t.desval.t.packed);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpa_result) =
            self.pb.borrow().val(&self.t.t.t.desval.t.packed);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpae_result) =
            self.pb.borrow().val(&self.t.t.t.desval.t.packed);

        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpe_result_flag) =
            (if (self.pb.borrow().val(&self.t.t.t.t.cmpae_result_flag) == FieldT::one())
                && (self.pb.borrow().val(&self.t.t.t.t.cmpa_result_flag) == FieldT::zero())
            {
                FieldT::one()
            } else {
                FieldT::zero()
            });
    }
}

pub fn test_ALU_cmpe_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmpe test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmp_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMPE.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmp_gadgets<FieldT> {
            let mut cmpa_result = variable::<FieldT, pb_variable>::default();
            cmpa_result.allocate(&pb, "cmpa_result".to_owned());
            let mut cmpa_result_flag = variable::<FieldT, pb_variable>::default();
            cmpa_result_flag.allocate(&pb, "cmpa_result_flag".to_owned());
            let mut cmpae_result = variable::<FieldT, pb_variable>::default();
            cmpae_result.allocate(&pb, "cmpae_result".to_owned());
            let mut cmpae_result_flag = variable::<FieldT, pb_variable>::default();
            cmpae_result_flag.allocate(&pb, "cmpae_result_flag".to_owned());
            return ALU_cmp_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                cmpa_result.clone(),
                cmpa_result_flag.clone(),
                cmpae_result.clone(),
                cmpae_result_flag.clone(),
                "ALU_cmp_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return des;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return x == y;
        },
    );
    print_time("cmpe tests successful");
}

pub fn test_ALU_cmpa_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmpa test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmp_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMPA.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmp_gadgets<FieldT> {
            let mut cmpe_result = variable::<FieldT, pb_variable>::default();
            cmpe_result.allocate(&pb, "cmpe_result".to_owned());
            let mut cmpe_result_flag = variable::<FieldT, pb_variable>::default();
            cmpe_result_flag.allocate(&pb, "cmpe_result_flag".to_owned());
            let mut cmpae_result = variable::<FieldT, pb_variable>::default();
            cmpae_result.allocate(&pb, "cmpae_result".to_owned());
            let mut cmpae_result_flag = variable::<FieldT, pb_variable>::default();
            cmpae_result_flag.allocate(&pb, "cmpae_result_flag".to_owned());
            return ALU_cmp_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                cmpe_result.clone(),
                cmpe_result_flag.clone(),
                result.clone(),
                result_flag.clone(),
                cmpae_result.clone(),
                cmpae_result_flag.clone(),
                "ALU_cmp_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return des;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return x > y;
        },
    );
    print_time("cmpa tests successful");
}

pub fn test_ALU_cmpae_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmpae test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmp_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMPAE.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmp_gadgets<FieldT> {
            let mut cmpe_result = variable::<FieldT, pb_variable>::default();
            cmpe_result.allocate(&pb, "cmpe_result".to_owned());
            let mut cmpe_result_flag = variable::<FieldT, pb_variable>::default();
            cmpe_result_flag.allocate(&pb, "cmpe_result_flag".to_owned());
            let mut cmpa_result = variable::<FieldT, pb_variable>::default();
            cmpa_result.allocate(&pb, "cmpa_result".to_owned());
            let mut cmpa_result_flag = variable::<FieldT, pb_variable>::default();
            cmpa_result_flag.allocate(&pb, "cmpa_result_flag".to_owned());
            return ALU_cmp_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                cmpe_result.clone(),
                cmpe_result_flag.clone(),
                cmpa_result.clone(),
                cmpa_result_flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_cmp_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return des;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return x >= y;
        },
    );
    print_time("cmpae tests successful");
}

/* signed comparison */
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_cmps_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* negate sign bits */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1].clone() * (-1).into())
                        .into(),
                ],
                vec![self.t.t.t.t.negated_arg1val_sign.clone().into()],
            ),
            format!("{} negated_arg1val_sign", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.arg2val.t.bits[self.pb.borrow().t.ap.w - 1].clone() * (-1).into())
                        .into(),
                ],
                vec![self.t.t.t.t.negated_arg2val_sign.clone().into()],
            ),
            format!("{} negated_arg2val_sign", self.annotation_prefix),
        );

        /* pack */
        self.t
            .t
            .t
            .t
            .pack_modified_arg1
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .pack_modified_arg2
            .borrow()
            .generate_r1cs_constraints(false);

        /* compare */
        self.t.t.t.t.comparator.borrow().generate_r1cs_constraints();

        /* copy over results */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.desval.t.packed.clone().into()],
                vec![self.t.t.t.t.cmpg_result.clone().into()],
            ),
            format!("{} cmpg_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.desval.t.packed.clone().into()],
                vec![self.t.t.t.t.cmpge_result.clone().into()],
            ),
            format!("{} cmpge_result", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        /* negate sign bits */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.negated_arg1val_sign) = FieldT::one()
            - self
                .pb
                .borrow()
                .val(&self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1]);
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.negated_arg2val_sign) = FieldT::one()
            - self
                .pb
                .borrow()
                .val(&self.t.t.t.arg2val.t.bits[self.pb.borrow().t.ap.w - 1]);

        /* pack */
        self.t
            .t
            .t
            .t
            .pack_modified_arg1
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .t
            .t
            .t
            .pack_modified_arg2
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* produce result */
        self.t.t.t.t.comparator.borrow().generate_r1cs_witness();

        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpg_result) =
            self.pb.borrow().val(&self.t.t.t.desval.t.packed);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.cmpge_result) =
            self.pb.borrow().val(&self.t.t.t.desval.t.packed);
    }
}
pub fn test_ALU_cmpg_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmpg test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmps_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMPG.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmps_gadgets<FieldT> {
            let mut cmpge_result = variable::<FieldT, pb_variable>::default();
            cmpge_result.allocate(&pb, "cmpge_result".to_owned());
            let mut cmpge_result_flag = variable::<FieldT, pb_variable>::default();
            cmpge_result_flag.allocate(&pb, "cmpge_result_flag".to_owned());
            return ALU_cmps_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                cmpge_result.clone(),
                cmpge_result_flag.clone(),
                "ALU_cmps_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return des;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (from_twos_complement(x, w) > from_twos_complement(y, w));
        },
    );
    print_time("cmpg tests successful");
}

pub fn test_ALU_cmpge_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting cmpge test");
    brute_force_arithmetic_gadget::<FieldT, ALU_cmps_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_CMPGE.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_cmps_gadgets<FieldT> {
            let mut cmpg_result = variable::<FieldT, pb_variable>::default();
            cmpg_result.allocate(&pb, "cmpg_result".to_owned());
            let mut cmpg_result_flag = variable::<FieldT, pb_variable>::default();
            cmpg_result_flag.allocate(&pb, "cmpg_result_flag".to_owned());
            return ALU_cmps_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                cmpg_result.clone(),
                cmpg_result_flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_cmps_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return des;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (from_twos_complement(x, w) >= from_twos_complement(y, w));
        },
    );
    print_time("cmpge tests successful");
}

impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_umul_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* do multiplication */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.arg1val.t.packed.clone().into()],
                vec![self.t.t.t.arg2val.t.packed.clone().into()],
                vec![self.t.t.t.t.mul_result.t.packed.clone().into()],
            ),
            format!("{} main_constraint", self.annotation_prefix),
        );
        self.t.t.t.t.mul_result.generate_r1cs_constraints(true);

        /* pack result */
        self.t
            .t
            .t
            .t
            .pack_mull_result
            .borrow()
            .generate_r1cs_constraints(false);
        self.t
            .t
            .t
            .t
            .pack_umulh_result
            .borrow()
            .generate_r1cs_constraints(false);

        /* compute flag */
        self.t
            .t
            .t
            .t
            .compute_flag
            .borrow()
            .generate_r1cs_constraints();

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.result_flag.clone().into()],
                vec![self.t.t.t.t.mull_flag.clone().into()],
            ),
            format!("{} mull_flag", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.result_flag.clone().into()],
                vec![self.t.t.t.t.umulh_flag.clone().into()],
            ),
            format!("{} umulh_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        /* do multiplication */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.mul_result.t.packed) =
            self.pb.borrow().val(&self.t.t.t.arg1val.t.packed)
                * self.pb.borrow().val(&self.t.t.t.arg2val.t.packed);
        self.t.t.t.t.mul_result.generate_r1cs_witness_from_packed();

        /* pack result */
        self.t
            .t
            .t
            .t
            .pack_mull_result
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .t
            .t
            .t
            .pack_umulh_result
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* compute flag */
        self.t.t.t.t.compute_flag.borrow().generate_r1cs_witness();

        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.mull_flag) =
            self.pb.borrow().val(&self.t.t.t.result_flag);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.umulh_flag) =
            self.pb.borrow().val(&self.t.t.t.result_flag);
    }
}
pub fn test_ALU_mull_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting mull test");
    brute_force_arithmetic_gadget::<FieldT, ALU_umul_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_MULL.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_umul_gadgets<FieldT> {
            let mut umulh_result = variable::<FieldT, pb_variable>::default();
            umulh_result.allocate(&pb, "umulh_result".to_owned());
            let mut umulh_flag = variable::<FieldT, pb_variable>::default();
            umulh_flag.allocate(&pb, "umulh_flag".to_owned());
            return ALU_umul_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                umulh_result.clone(),
                umulh_flag.clone(),
                "ALU_umul_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (x * y) % (1usize << w);
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return ((x * y) >> w) != 0;
        },
    );
    print_time("mull tests successful");
}

pub fn test_ALU_umulh_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting umulh test");
    brute_force_arithmetic_gadget::<FieldT, ALU_umul_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_UMULH.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_umul_gadgets<FieldT> {
            let mut mull_result = variable::<FieldT, pb_variable>::default();
            mull_result.allocate(&pb, "mull_result".to_owned());
            let mut mull_flag = variable::<FieldT, pb_variable>::default();
            mull_flag.allocate(&pb, "mull_flag".to_owned());
            return ALU_umul_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                mull_result.clone(),
                mull_flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_umul_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (x * y) >> w;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return ((x * y) >> w) != 0;
        },
    );
    print_time("umulh tests successful");
}

impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_smul_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* do multiplication */
        /*
          from two's complement: (packed - 2^w * bits[w-1])
          to two's complement: lower order bits of 2^{2w} + result_of_*
        */

        let (mut a, mut b, mut c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a.add_term(self.t.t.t.arg1val.t.packed.index, 1);
        a.add_term_with_field(
            self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1].index,
            -(FieldT::from(2) ^ self.pb.borrow().t.ap.w),
        );
        b.add_term(self.t.t.t.arg2val.t.packed.index, 1);
        b.add_term_with_field(
            self.t.t.t.arg2val.t.bits[self.pb.borrow().t.ap.w - 1].index,
            -(FieldT::from(2) ^ self.pb.borrow().t.ap.w),
        );
        c.add_term(self.t.t.t.t.mul_result.t.packed.index, 1);
        c.add_term_with_field(ONE, -(FieldT::from(2) ^ (2 * self.pb.borrow().t.ap.w)));
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a, b, c),
            format!("{} main_constraint", self.annotation_prefix),
        );

        self.t.t.t.t.mul_result.generate_r1cs_constraints(true);

        /* pack result */
        self.t
            .t
            .t
            .t
            .pack_smulh_result
            .borrow()
            .generate_r1cs_constraints(false);

        /* compute flag */
        self.t
            .t
            .t
            .t
            .pack_top
            .borrow()
            .generate_r1cs_constraints(false);

        /*
          the gadgets below are FieldT specific:
          I * X = (1-R)
          R * X = 0

          if X = 0 then R = 1
          if X != 0 then R = 0 and I = X^{-1}
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.t.is_top_empty_aux.clone().into()],
                vec![self.t.t.t.t.top.clone().into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.is_top_empty.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} I*X=1-R (is_top_empty)", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.t.is_top_empty.clone().into()],
                vec![self.t.t.t.t.top.clone().into()],
                vec![(variable::<FieldT, pb_variable>::from(ONE) * 0.into()).into()],
            ),
            format!("{} R*X=0 (is_top_full)", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.t.is_top_full_aux.clone().into()],
                vec![
                    self.t.t.t.t.top.clone().into(),
                    (variable::<FieldT, pb_variable>::from(ONE)
                        * (1i64 - (1i64 << (self.pb.borrow().t.ap.w + 1))).into())
                    .into(),
                ],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.is_top_full.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} I*X=1-R (is_top_full)", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![self.t.t.t.t.is_top_full.clone().into()],
                vec![
                    self.t.t.t.t.top.clone().into(),
                    (variable::<FieldT, pb_variable>::from(ONE)
                        * (1 - (1i64 << (self.pb.borrow().t.ap.w + 1))).into())
                    .into(),
                ],
                vec![(variable::<FieldT, pb_variable>::from(ONE) * 0.into()).into()],
            ),
            format!("{} R*X=0 (is_top_full)", self.annotation_prefix),
        );

        /* smulh_flag = 1 - (is_top_full + is_top_empty) */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.is_top_full.clone() * (-1).into()).into(),
                    (self.t.t.t.t.is_top_empty.clone() * (-1).into()).into(),
                ],
                vec![self.t.t.t.t.smulh_flag.clone().into()],
            ),
            format!("{} smulh_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        /* do multiplication */
        /*
          from two's complement: (packed - 2^w * bits[w-1])
          to two's complement: lower order bits of (2^{2w} + result_of_mul)
        */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.mul_result.t.packed) =
            (self.pb.borrow().val(&self.t.t.t.arg1val.t.packed)
                - (self
                    .pb
                    .borrow()
                    .val(&self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1])
                    * (FieldT::from(2) ^ self.pb.borrow().t.ap.w)))
                * (self.pb.borrow().val(&self.t.t.t.arg2val.t.packed)
                    - (self
                        .pb
                        .borrow()
                        .val(&self.t.t.t.arg2val.t.bits[self.pb.borrow().t.ap.w - 1])
                        * (FieldT::from(2) ^ self.pb.borrow().t.ap.w)))
                + (FieldT::from(2) ^ (2 * self.pb.borrow().t.ap.w));

        self.t.t.t.t.mul_result.generate_r1cs_witness_from_packed();

        /* pack result */
        self.t
            .t
            .t
            .t
            .pack_smulh_result
            .borrow()
            .generate_r1cs_witness_from_bits();

        /* compute flag */
        self.t
            .t
            .t
            .t
            .pack_top
            .borrow()
            .generate_r1cs_witness_from_bits();
        let topval = self.pb.borrow().val(&self.t.t.t.t.top).as_ulong();

        if topval == 0 {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_empty) = FieldT::one();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_empty_aux) = FieldT::zero();
        } else {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_empty) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_empty_aux) =
                self.pb.borrow().val(&self.t.t.t.t.top).inverse();
        }

        if topval == ((1usize << (self.pb.borrow().t.ap.w + 1)) - 1) {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_full) = FieldT::one();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_full_aux) = FieldT::zero();
        } else {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_full) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.is_top_full_aux) =
                (self.pb.borrow().val(&self.t.t.t.t.top)
                    - FieldT::from((1usize << (self.pb.borrow().t.ap.w + 1)) - 1))
                .inverse();
        }

        /* smulh_flag = 1 - (is_top_full + is_top_empty) */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.smulh_flag) = FieldT::one()
            - (self.pb.borrow().val(&self.t.t.t.t.is_top_full)
                + self.pb.borrow().val(&self.t.t.t.t.is_top_empty));
    }
}
pub fn test_ALU_smulh_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting smulh test");
    brute_force_arithmetic_gadget::<FieldT, ALU_smul_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_SMULH.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_smul_gadgets<FieldT> {
            return ALU_smul_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_smul_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            let res = to_twos_complement(
                (from_twos_complement(x, w) * from_twos_complement(y, w)) as i64,
                2 * w,
            );
            return res >> w;
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            let res = from_twos_complement(x, w) * from_twos_complement(y, w);
            let truncated_res = from_twos_complement(
                to_twos_complement(res as i64, 2 * w) & ((1usize << w) - 1),
                w,
            );
            return (res != truncated_res);
        },
    );
    print_time("smulh tests successful");
}

impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_divmod_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* B_inv * B = B_nonzero */
        let (mut a1, mut b1, mut c1) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a1.add_term(self.t.t.t.t.B_inv.index, 1);
        b1.add_term(self.t.t.t.arg2val.t.packed.index, 1);
        c1.add_term(self.t.t.t.t.B_nonzero.index, 1);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a1, b1, c1),
            format!("{} B_inv*B=B_nonzero", self.annotation_prefix),
        );

        /* (1-B_nonzero) * B = 0 */
        let (mut a2, mut b2, mut c2) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a2.add_term(ONE, 1);
        a2.add_term(self.t.t.t.t.B_nonzero.index, -1);
        b2.add_term(self.t.t.t.arg2val.t.packed.index, 1);
        c2.add_term(ONE, 0);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a2, b2, c2),
            format!("{} (1-B_nonzero)*B=0", self.annotation_prefix),
        );

        /* B * q + r = A_aux = A * B_nonzero */
        let (mut a3, mut b3, mut c3) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a3.add_term(self.t.t.t.arg2val.t.packed.index, 1);
        b3.add_term(self.t.t.t.t.udiv_result.index, 1);
        c3.add_term(self.t.t.t.t.A_aux.index, 1);
        c3.add_term(self.t.t.t.t.umod_result.index, -1);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a3, b3, c3),
            format!("{} B*q+r=A_aux", self.annotation_prefix),
        );

        let (mut a4, mut b4, mut c4) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a4.add_term(self.t.t.t.arg1val.t.packed.index, 1);
        b4.add_term(self.t.t.t.t.B_nonzero.index, 1);
        c4.add_term(self.t.t.t.t.A_aux.index, 1);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a4, b4, c4),
            format!("{} A_aux=A*B_nonzero", self.annotation_prefix),
        );

        /* q * (1-B_nonzero) = 0 */
        let (mut a5, mut b5, mut c5) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a5.add_term(self.t.t.t.t.udiv_result.index, 1);
        b5.add_term(ONE, 1);
        b5.add_term(self.t.t.t.t.B_nonzero.index, -1);
        c5.add_term(ONE, 0);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a5, b5, c5),
            format!("{} q*B_nonzero=0", self.annotation_prefix),
        );

        /* A<B_gadget<FieldT>(B, r, less=B_nonzero, leq=variable::<FieldT,pb_variable>::from(ONE).into()) */
        self.t.t.t.t.r_less_B.borrow().generate_r1cs_constraints();
    }

    fn generate_r1cs_witness(&self) {
        if self.pb.borrow().val(&self.t.t.t.arg2val.t.packed) == FieldT::zero() {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.B_inv) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.B_nonzero) = FieldT::zero();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.A_aux) = FieldT::zero();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.udiv_result) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.umod_result) = FieldT::zero();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.udiv_flag) = FieldT::one();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.umod_flag) = FieldT::one();
        } else {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.B_inv) =
                self.pb.borrow().val(&self.t.t.t.arg2val.t.packed).inverse();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.B_nonzero) = FieldT::one();

            let A = self
                .pb
                .borrow()
                .val(&self.t.t.t.arg1val.t.packed)
                .as_ulong();
            let B = self
                .pb
                .borrow()
                .val(&self.t.t.t.arg2val.t.packed)
                .as_ulong();

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.A_aux) =
                self.pb.borrow().val(&self.t.t.t.arg1val.t.packed);

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.udiv_result) = FieldT::from(A / B);
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.umod_result) = FieldT::from(A % B);

            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.udiv_flag) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.t.t.t.umod_flag) = FieldT::zero();
        }

        self.t.t.t.t.r_less_B.borrow().generate_r1cs_witness();
    }
}
pub fn test_ALU_udiv_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting udiv test");
    brute_force_arithmetic_gadget::<FieldT, ALU_divmod_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_UDIV.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_divmod_gadgets<FieldT> {
            let mut umod_result = variable::<FieldT, pb_variable>::default();
            umod_result.allocate(&pb, "umod_result".to_owned());
            let mut umod_flag = variable::<FieldT, pb_variable>::default();
            umod_flag.allocate(&pb, "umod_flag".to_owned());
            return ALU_divmod_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                umod_result.clone(),
                umod_flag.clone(),
                "ALU_divmod_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return if y == 0 { 0 } else { x / y };
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (y == 0);
        },
    );
    print_time("udiv tests successful");
}

pub fn test_ALU_umod_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting umod test");
    brute_force_arithmetic_gadget::<FieldT, ALU_divmod_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_UMOD.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_divmod_gadgets<FieldT> {
            let mut udiv_result = variable::<FieldT, pb_variable>::default();
            udiv_result.allocate(&pb, "udiv_result".to_owned());
            let mut udiv_flag = variable::<FieldT, pb_variable>::default();
            udiv_flag.allocate(&pb, "udiv_flag".to_owned());
            return ALU_divmod_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                udiv_result.clone(),
                udiv_flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_divmod_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return if y == 0 { 0 } else { x % y };
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (y == 0);
        },
    );
    print_time("umod tests successful");
}
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for ALU_shr_shl_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /*
          select the input for barrel shifter:

          r = arg1val * opcode_indicators[SHR] + reverse(arg1val) * (1-opcode_indicators[SHR])
          r - reverse(arg1val) = (arg1val - reverse(arg1val)) * opcode_indicators[SHR]
        */
        self.t
            .t
            .t
            .t
            .pack_reversed_input
            .borrow()
            .generate_r1cs_constraints(false);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    self.t.t.t.arg1val.t.packed.clone().into(),
                    (self.t.t.t.t.reversed_input.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.opcode_indicators
                        [tinyram_opcode::tinyram_opcode_SHR.clone() as usize]
                        .clone()
                        .into(),
                ],
                vec![
                    self.t.t.t.t.barrel_right_internal[0].clone().into(),
                    (self.t.t.t.t.reversed_input.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} select_arg1val_or_reversed", self.annotation_prefix),
        );

        /*
          do logw iterations of barrel shifts
        */
        for i in 0..self.t.t.t.t.logw {
            /* assert that shifted out part is bits */
            for j in 0..1usize << i {
                generate_boolean_r1cs_constraint::<FieldT, tinyram_protoboard<FieldT>>(
                    &self.pb,
                    &self.t.t.t.t.shifted_out_bits[i][j].clone().into(),
                    format!("{} shifted_out_bits_{}_{}", self.annotation_prefix, i, j),
                );
            }

            /*
              add main shifting constraint


              old_result =
              (shifted_result * 2^(i+1) + shifted_out_part) * need_to_shift +
              (shfited_result) * (1-need_to_shift)

              old_result - shifted_result = (shifted_result * (2^(i+1) - 1) + shifted_out_part) * need_to_shift
            */
            let (mut a, mut b, mut c) = (
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            );

            a.add_term_with_field(
                self.t.t.t.t.barrel_right_internal[i + 1].index,
                (FieldT::from(2) ^ (i + 1)) - FieldT::one(),
            );
            for j in 0..1usize << i {
                a.add_term_with_field(
                    self.t.t.t.t.shifted_out_bits[i][j].index,
                    (FieldT::from(2) ^ j),
                );
            }

            b.add_term(self.t.t.t.arg2val.t.bits[i].index, 1);

            c.add_term(self.t.t.t.t.barrel_right_internal[i].index, 1);
            c.add_term(self.t.t.t.t.barrel_right_internal[i + 1].index, -1);

            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a, b, c),
                format!("{} barrel_shift_{}", self.annotation_prefix, i),
            );
        }

        /*
          get result as the logw iterations or zero if shift was oversized

          result = (1-is_oversize_shift) * barrel_right_internal[logw]
        */
        self.t
            .t
            .t
            .t
            .check_oversize_shift
            .borrow()
            .generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    variable::<FieldT, pb_variable>::from(ONE).into(),
                    (self.t.t.t.t.is_oversize_shift.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.t.barrel_right_internal[self.t.t.t.t.logw]
                        .clone()
                        .into(),
                ],
                vec![self.t.t.t.result.clone().into()],
            ),
            format!("{} result", self.annotation_prefix),
        );

        /*
          get reversed result for SHL
        */
        self.t
            .t
            .t
            .t
            .unpack_result
            .borrow()
            .generate_r1cs_constraints(true);
        self.t
            .t
            .t
            .t
            .pack_reversed_result
            .borrow()
            .generate_r1cs_constraints(false);

        /*
          select the correct output:
          r = result * opcode_indicators[SHR] + reverse(result) * (1-opcode_indicators[SHR])
          r - reverse(result) = (result - reverse(result)) * opcode_indicators[SHR]
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    self.t.t.t.result.clone().into(),
                    (self.t.t.t.t.reversed_result.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.opcode_indicators
                        [tinyram_opcode::tinyram_opcode_SHR.clone() as usize]
                        .clone()
                        .into(),
                ],
                vec![
                    self.t.t.t.t.shr_result.clone().into(),
                    (self.t.t.t.t.reversed_result.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} shr_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![
                    self.t.t.t.result.clone().into(),
                    (self.t.t.t.t.reversed_result.clone() * (-1).into()).into(),
                ],
                vec![
                    self.t.t.t.opcode_indicators
                        [tinyram_opcode::tinyram_opcode_SHR.clone() as usize]
                        .clone()
                        .into(),
                ],
                vec![
                    self.t.t.t.t.shr_result.clone().into(),
                    (self.t.t.t.t.reversed_result.clone() * (-1).into()).into(),
                ],
            ),
            format!("{} shl_result", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![self.t.t.t.arg1val.t.bits[0].clone().into()],
                vec![self.t.t.t.t.shr_flag.clone().into()],
            ),
            format!("{} shr_flag", self.annotation_prefix),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new_with_vec(
                vec![variable::<FieldT, pb_variable>::from(ONE).into()],
                vec![
                    self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1]
                        .clone()
                        .into(),
                ],
                vec![self.t.t.t.t.shl_flag.clone().into()],
            ),
            format!("{} shl_flag", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        /* select the input for barrel shifter */
        self.t
            .t
            .t
            .t
            .pack_reversed_input
            .borrow()
            .generate_r1cs_witness_from_bits();

        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.t.barrel_right_internal[0]) =
            (if self.pb.borrow().val(
                &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_SHR.clone() as usize],
            ) == FieldT::one()
            {
                self.pb.borrow().val(&self.t.t.t.arg1val.t.packed)
            } else {
                self.pb.borrow().val(&self.t.t.t.t.reversed_input)
            });

        /*
          do logw iterations of barrel shifts.

          old_result =
          (shifted_result * 2^i + shifted_out_part) * need_to_shift +
          (shfited_result) * (1-need_to_shift)
        */

        for i in 0..self.t.t.t.t.logw {
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.t.t.t.barrel_right_internal[i + 1]) =
                if (self.pb.borrow().val(&self.t.t.t.arg2val.t.bits[i]) == FieldT::zero()) {
                    self.pb.borrow().val(&self.t.t.t.t.barrel_right_internal[i])
                } else {
                    FieldT::from(
                        self.pb
                            .borrow()
                            .val(&self.t.t.t.t.barrel_right_internal[i])
                            .as_ulong()
                            >> (i + 1),
                    )
                };

            self.t.t.t.t.shifted_out_bits[i].fill_with_bits_of_ulong(
                &self.pb,
                (self
                    .pb
                    .borrow()
                    .val(&self.t.t.t.t.barrel_right_internal[i])
                    .as_ulong()
                    % (2 << i)) as u64,
            );
        }

        /*
          get result as the logw iterations or zero if shift was oversized

          result = (1-is_oversize_shift) * barrel_right_internal[logw]
        */
        self.t
            .t
            .t
            .t
            .check_oversize_shift
            .borrow()
            .generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.t.t.result) = (FieldT::one()
            - self.pb.borrow().val(&self.t.t.t.t.is_oversize_shift))
            * self
                .pb
                .borrow()
                .val(&self.t.t.t.t.barrel_right_internal[self.t.t.t.t.logw]);

        /*
          get reversed result for SHL
        */
        self.t
            .t
            .t
            .t
            .unpack_result
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .t
            .t
            .t
            .pack_reversed_result
            .borrow()
            .generate_r1cs_witness_from_bits();

        /*
          select the correct output:
          r = result * opcode_indicators[SHR] + reverse(result) * (1-opcode_indicators[SHR])
          r - reverse(result) = (result - reverse(result)) * opcode_indicators[SHR]
        */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.shr_result) = if (self.pb.borrow().val(
            &self.t.t.t.opcode_indicators[tinyram_opcode::tinyram_opcode_SHR.clone() as usize],
        ) == FieldT::one())
        {
            self.pb.borrow().val(&self.t.t.t.result)
        } else {
            self.pb.borrow().val(&self.t.t.t.t.reversed_result)
        };

        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.shl_result) =
            self.pb.borrow().val(&self.t.t.t.t.shr_result);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.shr_flag) =
            self.pb.borrow().val(&self.t.t.t.arg1val.t.bits[0]);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.t.shl_flag) = self
            .pb
            .borrow()
            .val(&self.t.t.t.arg1val.t.bits[self.pb.borrow().t.ap.w - 1]);
    }
}
pub fn test_ALU_shr_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting shr test");
    brute_force_arithmetic_gadget::<FieldT, ALU_shr_shl_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_SHR.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_shr_shl_gadgets<FieldT> {
            let mut shl_result = variable::<FieldT, pb_variable>::default();
            shl_result.allocate(&pb, "shl_result".to_owned());
            let mut shl_flag = variable::<FieldT, pb_variable>::default();
            shl_flag.allocate(&pb, "shl_flag".to_owned());
            return ALU_shr_shl_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                result.clone(),
                result_flag.clone(),
                shl_result.clone(),
                shl_flag.clone(),
                "ALU_shr_shl_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (x >> y);
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x & 1) != 0;
        },
    );
    print_time("shr tests successful");
}

pub fn test_ALU_shl_gadget<FieldT: FieldTConfig>(w: usize) {
    print_time("starting shl test");
    brute_force_arithmetic_gadget::<FieldT, ALU_shr_shl_gadgets<FieldT>, _, _>(
        w,
        tinyram_opcode::tinyram_opcode_SHL.clone() as usize,
        |pb: &RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
         opcode_indicators: &pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
         desval: &word_variable_gadgets<FieldT>,
         arg1val: &word_variable_gadgets<FieldT>,
         arg2val: &word_variable_gadgets<FieldT>,
         flag: &variable<FieldT, pb_variable>,
         result: &variable<FieldT, pb_variable>,
         result_flag: &variable<FieldT, pb_variable>|
         -> ALU_shr_shl_gadgets<FieldT> {
            let mut shr_result = variable::<FieldT, pb_variable>::default();
            shr_result.allocate(&pb, "shr_result".to_owned());
            let mut shr_flag = variable::<FieldT, pb_variable>::default();
            shr_flag.allocate(&pb, "shr_flag".to_owned());
            return ALU_shr_shl_gadget::<FieldT>::new(
                pb.clone(),
                opcode_indicators.clone(),
                desval.clone(),
                arg1val.clone(),
                arg2val.clone(),
                flag.clone(),
                shr_result.clone(),
                shr_flag.clone(),
                result.clone(),
                result_flag.clone(),
                "ALU_shr_shl_gadget".to_owned(),
            );
        },
        |des: usize, f: bool, x: usize, y: usize| -> usize {
            return (x << y) & ((1usize << w) - 1);
        },
        |des: usize, f: bool, x: usize, y: usize| -> bool {
            return (x >> (w - 1)) != 0;
        },
    );
    print_time("shl tests successful");
}
