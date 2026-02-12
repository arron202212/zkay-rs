// Declaration of interfaces for the TinyRAM memory masking gadget.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{dual_variable_gadget, inner_product_gadget};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::ArithmeticGadgetConfig;
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::tinyram_protoboard::{
    SubTinyRamGadgetConfig, tinyram_gadget, tinyram_protoboard, tinyram_standard_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::word_variable_gadget::{
    doubleword_variable_gadget, doubleword_variable_gadgets, word_variable_gadget,
};
use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::tinyram_inner_product_gadget;
use crate::gadgetlib1::pb_variable::{
    pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_sum,
};
use crate::gadgetlib1::pb_variable::{pb_variable, pb_variable_array};
use crate::gadgetlib1::protoboard::{ProtoboardConfig, protoboard};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::ram_computations::rams::ram_params::ArchitectureParamsTypeConfig;
use crate::relations::variable::linear_combination;
use crate::relations::variable::variable;
use ffec::FieldTConfig;
use rccell::RcCell;
use std::marker::PhantomData;
/**
 * The memory masking gadget checks if a specified part of a double
 * word is correctly modified. In TinyRAM CPU checker we use this to
 * implement byte addressing and word addressing for the memory that
 * consists of double words.
 *
 * More precisely, memory masking gadgets takes the following
 * arguments:
 *
 * dw_contents_prev, dw_contents_next -- the contents of the memory
 *
 * double word before and after the access
 *
 * access_is_word -- a boolean indicating if access is word
 *
 * access_is_byte -- a boolean indicating if access is byte
 *
 * subaddress -- an integer specifying which byte (if access_is_byte=1)
 * or word (if access_is_byte=1) this access is operating on
 *
 * subcontents -- contents of the byte, resp., word to be operated on
 *
 * Memory masking gadget enforces that dw_contents_prev is equal to
 * dw_contents_next everywhere, except subaddres-th byte (if
 * access_is_byte = 1), or MSB(subaddress)-th word (if access_is_word =
 * 1). The corresponding byte, resp., word in dw_contents_next is
 * required to equal subcontents.
 *
 * Note that indexing MSB(subaddress)-th word is the same as indexing
 * the word specified by subaddress expressed in bytes and aligned to
 * the word boundary by rounding the subaddress down.
 *
 * Requirements: The caller is required to perform bounds checks on
 * subcontents. The caller is also required to ensure that exactly one
 * of access_is_word and access_is_byte is set to 1.
 */
#[derive(Clone, Default)]
pub struct memory_masking_gadget<FieldT: FieldTConfig> {
    // : public tinyram_standard_gadget<FieldT>
    shift: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    is_word0: variable<FieldT, pb_variable>,
    is_word1: variable<FieldT, pb_variable>,
    is_subaddress: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    is_byte: pb_variable_array<FieldT, tinyram_protoboard<FieldT>>,
    masked_out_word0: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    masked_out_word1: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    masked_out_bytes: pb_linear_combination_array<FieldT, tinyram_protoboard<FieldT>>,
    get_masked_out_dw_contents_prev: RcCell<tinyram_inner_product_gadget<FieldT>>,
    masked_out_dw_contents_prev: variable<FieldT, pb_variable>,
    expected_dw_contents_next: variable<FieldT, pb_variable>,
    dw_contents_prev: doubleword_variable_gadgets<FieldT>,
    subaddress: gadget<
        FieldT,
        tinyram_protoboard<FieldT>,
        dual_variable_gadget<FieldT, tinyram_protoboard<FieldT>, word_variable_gadget<FieldT>>,
    >,
    subcontents: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    access_is_word: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    access_is_byte: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    dw_contents_next: doubleword_variable_gadgets<FieldT>,
    // memory_masking_gadget(tinyram_protoboard<FieldT> &pb,
    //                       dw_contents_prev:doubleword_variable_gadgets<FieldT>,
    //                       subaddress:gadget<FieldT,tinyram_protoboard<FieldT>,dual_variable_gadget<FieldT,tinyram_protoboard<FieldT>,word_variable_gadget<FieldT>>>,
    //                       subcontents:linear_combination<FieldT,pb_variable,pb_linear_combination>,
    //                       access_is_word:linear_combination<FieldT,pb_variable,pb_linear_combination>,
    //                       access_is_byte:linear_combination<FieldT,pb_variable,pb_linear_combination>,
    //                       dw_contents_next:doubleword_variable_gadgets<FieldT>,
    //                       annotation_prefix:String="");
    // pub fn  generate_r1cs_constraints();
    // pub fn  generate_r1cs_witness();
}

// use crate::gadgetlib1::gadgets::cpu_checkers::tinyram::components::memory_masking_gadget;

// use crate::gadgetlib1::gadget::gadget;

pub type memory_masking_gadgets<FieldT> = gadget<
    FieldT,
    tinyram_protoboard<FieldT>,
    tinyram_gadget<FieldT, tinyram_standard_gadget<FieldT, memory_masking_gadget<FieldT>>>,
>;

impl<FieldT: FieldTConfig> memory_masking_gadget<FieldT> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, tinyram_protoboard<FieldT>>>,
        dw_contents_prev: doubleword_variable_gadgets<FieldT>,
        subaddress: gadget<
            FieldT,
            tinyram_protoboard<FieldT>,
            dual_variable_gadget<FieldT, tinyram_protoboard<FieldT>, word_variable_gadget<FieldT>>,
        >,
        subcontents: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        access_is_word: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        access_is_byte: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        dw_contents_next: doubleword_variable_gadgets<FieldT>,
        annotation_prefix: String,
    ) -> memory_masking_gadgets<FieldT> {
        /*
          Indicator variables for access being to word_0, word_1, and
          byte_0, byte_1, ...

          We use little-endian indexing here (least significant
          bit/byte/word has the smallest address).
        */
        let mut is_word0 = variable::<FieldT, pb_variable>::default();
        is_word0.allocate(&pb, format!("{} is_word0", annotation_prefix));
        let mut is_word1 = variable::<FieldT, pb_variable>::default();
        is_word1.allocate(&pb, format!("{} is_word1", annotation_prefix));
        let mut is_subaddress = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        is_subaddress.allocate(
            &pb,
            2 * pb.borrow().t.ap.bytes_in_word(),
            format!("{} is_sub_address", annotation_prefix),
        );
        let mut is_byte = pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        is_byte.allocate(
            &pb,
            2 * pb.borrow().t.ap.bytes_in_word(),
            format!("{} is_byte", annotation_prefix),
        );

        /*
          Get value of the dw_contents_prev for which the specified entity
          is masked out to be zero. E.g. the value of masked_out_bytes[3]
          will be the same as the value of dw_contents_prev, when 3rd
          (0-indexed) byte is set to all zeros.
        */
        let mut masked_out_word0 =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        masked_out_word0.assign(
            &pb,
            &(pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                &pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    dw_contents_prev.t.bits.contents[pb.borrow().t.ap.w..2 * pb.borrow().t.ap.w]
                        .to_vec(),
                )
                .into(),
            ) * (FieldT::from(2) ^ pb.borrow().t.ap.w)),
        );
        let mut masked_out_word1 =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();

        masked_out_word1.assign(
            &pb,
            &pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                &pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                    dw_contents_prev.t.bits.contents[..pb.borrow().t.ap.w].to_vec(),
                )
                .into(),
            ),
        );
        let mut masked_out_bytes =
            pb_linear_combination_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        masked_out_bytes.contents.resize(
            2 * pb.borrow().t.ap.bytes_in_word(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );

        for i in 0..2 * pb.borrow().t.ap.bytes_in_word() {
            /* just subtract out the byte to be masked */
            masked_out_bytes[i].assign(
                &pb,
                &(linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    dw_contents_prev.t.packed.clone(),
                ) - pb_packing_sum::<FieldT, tinyram_protoboard<FieldT>>(
                    &pb_variable_array::<FieldT, tinyram_protoboard<FieldT>>::new(
                        dw_contents_prev.t.bits.contents[8 * i..8 * (i + 1)].to_vec(),
                    )
                    .into(),
                ) * (FieldT::from(2) ^ (8 * i))),
            );
        }

        /*
          Define masked_out_dw_contents_prev to be the correct masked out
          contents for the current access type.
        */

        let mut masked_out_indicators =
            pb_linear_combination_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        masked_out_indicators.contents.push(is_word0.clone().into());
        masked_out_indicators.contents.push(is_word1.clone().into());
        masked_out_indicators
            .contents
            .extend::<pb_linear_combination_array<FieldT, tinyram_protoboard<FieldT>>>(
                is_byte.clone().into(),
            );

        let mut masked_out_results =
            pb_linear_combination_array::<FieldT, tinyram_protoboard<FieldT>>::default();
        masked_out_results
            .contents
            .push(masked_out_word0.clone().into());
        masked_out_results
            .contents
            .push(masked_out_word1.clone().into());
        masked_out_results.contents.extend(masked_out_bytes.clone());
        let mut masked_out_dw_contents_prev = variable::<FieldT, pb_variable>::default();
        masked_out_dw_contents_prev.allocate(
            &pb,
            format!("{} masked_out_dw_contents_prev", annotation_prefix),
        );
        let mut get_masked_out_dw_contents_prev = RcCell::new(inner_product_gadget::<
            FieldT,
            tinyram_protoboard<FieldT>,
        >::new(
            pb.clone(),
            masked_out_indicators.clone(),
            masked_out_results.clone(),
            masked_out_dw_contents_prev.clone(),
            format!("{} get_masked_out_dw_contents_prev", annotation_prefix),
        ));

        /*
         Define shift so that masked_out_dw_contents_prev + shift * subcontents = dw_contents_next
        */
        let mut shift_lc = linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
            is_word0.clone(),
        ) * FieldT::from(1)
            + is_word1.clone() * (FieldT::from(2) ^ pb.borrow().t.ap.w);
        for i in 0..2 * pb.borrow().t.ap.bytes_in_word() {
            shift_lc = shift_lc.clone() + is_byte[i].clone() * (FieldT::from(2) ^ (8 * i));
        }
        let mut shift = linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        shift.assign(&pb, &shift_lc);
        tinyram_standard_gadget::<FieldT, Self>::new(
            pb,
            annotation_prefix,
            Self {
                shift,
                is_word0,
                is_word1,
                is_subaddress,
                is_byte,
                masked_out_word0,
                masked_out_word1,
                masked_out_bytes,
                get_masked_out_dw_contents_prev,
                masked_out_dw_contents_prev,
                expected_dw_contents_next: variable::<FieldT, pb_variable>::default(),
                dw_contents_prev,
                subaddress,
                subcontents,
                access_is_word,
                access_is_byte,
                dw_contents_next,
            },
        )
    }
}
impl<FieldT: FieldTConfig> SubTinyRamGadgetConfig for memory_masking_gadget<FieldT> {}
impl<FieldT: FieldTConfig> ArithmeticGadgetConfig<FieldT> for memory_masking_gadgets<FieldT> {
    fn generate_r1cs_constraints(&self) {
        /* get indicator variables for is_subaddress[i] by adding constraints
        is_subaddress[i] * (subaddress - i) = 0 and \sum_i is_subaddress[i] = 1 */
        for i in 0..2 * self.pb.borrow().t.ap.bytes_in_word() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.t.t.is_subaddress[i].clone().into(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.t.t.subaddress.t.packed.clone(),
                    ) - variable::<FieldT, pb_variable>::from(i),
                    FieldT::from(0).into(),
                ),
                format!("{} is_subaddress_{}", self.annotation_prefix, i),
            );
        }
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                pb_sum::<FieldT, tinyram_protoboard<FieldT>, pb_variable>(
                    &self.t.t.t.is_subaddress.clone().into(),
                ),
                FieldT::from(1).into(),
            ),
            format!("{} is_subaddress", self.annotation_prefix),
        );

        /* get indicator variables is_byte_X */
        for i in 0..2 * self.pb.borrow().t.ap.bytes_in_word() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.t.t.access_is_byte.clone().into(),
                    self.t.t.t.is_subaddress[i].clone().into(),
                    self.t.t.t.is_byte[i].clone().into(),
                ),
                format!("{} is_byte_{}", self.annotation_prefix, i),
            );
        }

        /* get indicator variables is_word_0/is_word_1 */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.access_is_word.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    FieldT::from(1),
                ) - self.t.t.t.subaddress.t.bits[self.pb.borrow().t.ap.subaddr_len() - 1].clone(),
                self.t.t.t.is_word0.clone().into(),
            ),
            format!("{} is_word_0", self.annotation_prefix),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.access_is_word.clone().into(),
                self.t.t.t.subaddress.t.bits[self.pb.borrow().t.ap.subaddr_len() - 1]
                    .clone()
                    .into(),
                self.t.t.t.is_word1.clone().into(),
            ),
            format!("{} is_word_1", self.annotation_prefix),
        );

        /* compute masked_out_dw_contents_prev */
        self.t
            .t
            .t
            .get_masked_out_dw_contents_prev
            .borrow_mut()
            .generate_r1cs_constraints();

        /*
          masked_out_dw_contents_prev + shift * subcontents = dw_contents_next
        */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.t.t.shift.clone().into(),
                self.t.t.t.subcontents.clone().into(),
                linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                    self.t.t.t.dw_contents_next.t.packed.clone(),
                ) - self.t.t.t.masked_out_dw_contents_prev.clone(),
            ),
            format!("{} mask_difference", self.annotation_prefix),
        );
    }

    fn generate_r1cs_witness(&self) {
        /* get indicator variables is_subaddress */
        for i in 0..2 * self.pb.borrow().t.ap.bytes_in_word() {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.is_subaddress[i]) =
                if (self.pb.borrow().val(&self.t.t.t.subaddress.t.packed) == FieldT::from(i)) {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
        }

        /* get indicator variables is_byte_X */
        for i in 0..2 * self.pb.borrow().t.ap.bytes_in_word() {
            *self.pb.borrow_mut().val_ref(&self.t.t.t.is_byte[i]) =
                self.pb.borrow().val(&self.t.t.t.is_subaddress[i])
                    * self.pb.borrow().lc_val(&self.t.t.t.access_is_byte);
        }

        /* get indicator variables is_word_0/is_word_1 */
        *self.pb.borrow_mut().val_ref(&self.t.t.t.is_word0) = (FieldT::one()
            - self
                .pb
                .borrow()
                .val(&self.t.t.t.subaddress.t.bits[self.pb.borrow().t.ap.subaddr_len() - 1]))
            * self.pb.borrow().lc_val(&self.t.t.t.access_is_word);
        *self.pb.borrow_mut().val_ref(&self.t.t.t.is_word1) = self
            .pb
            .borrow()
            .val(&self.t.t.t.subaddress.t.bits[self.pb.borrow().t.ap.subaddr_len() - 1])
            * self.pb.borrow().lc_val(&self.t.t.t.access_is_word);

        /* calculate shift and masked out words/bytes */
        self.t.t.t.shift.evaluate_pb(&self.pb);
        self.t.t.t.masked_out_word0.evaluate_pb(&self.pb);
        self.t.t.t.masked_out_word1.evaluate_pb(&self.pb);
        self.t.t.t.masked_out_bytes.evaluate(&self.pb);

        /* get masked_out dw/word0/word1/bytes */
        self.t
            .t
            .t
            .get_masked_out_dw_contents_prev
            .borrow()
            .generate_r1cs_witness();

        /* compute dw_contents_next */
        *self
            .pb
            .borrow_mut()
            .val_ref(&self.t.t.t.dw_contents_next.t.packed) = self
            .pb
            .borrow()
            .val(&self.t.t.t.masked_out_dw_contents_prev)
            + self.pb.borrow().lc_val(&self.t.t.t.shift)
                * self.pb.borrow().lc_val(&self.t.t.t.subcontents);
        self.t
            .t
            .t
            .dw_contents_next
            .generate_r1cs_witness_from_packed();
    }
}
