// Declaration of interfaces for top-level SHA256 gadgets.

use crate::common::data_structures::merkle_tree;

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::hash_io::{
    block_variable, block_variables, digest_variable, digest_variables,
};
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_aux::{
    big_sigma_gadget, big_sigma_gadgets, choice_gadget, choice_gadgets, lastbits_gadget,
    lastbits_gadgets, majority_gadget, majority_gadgets, small_sigma_gadget, small_sigma_gadgets,
};
 use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components::{SHA256_digest_size,SHA256_block_size,SHA256_default_IV,SHA256_K,sha256_message_schedule_gadgets,sha256_message_schedule_gadget,sha256_round_function_gadgets,sha256_round_function_gadget};
use crate::gadgetlib1::gadgets::hashes::sha256::sha256_components;
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::common::utils::bit_vector;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

/**
 * Gadget for the SHA256 compression function.
 */

pub struct sha256_compression_function_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>
pub  round_a: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_b: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_c: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_d: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_e: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_f: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_g: Vec<pb_linear_combination_array<FieldT, PB>>,
pub  round_h: Vec<pb_linear_combination_array<FieldT, PB>>,

pub  packed_W: pb_variable_array<FieldT, PB>,
pub  message_schedule: RcCell<sha256_message_schedule_gadgets<FieldT, PB>>,
pub  round_functions: Vec<sha256_round_function_gadgets<FieldT, PB>>,

pub  unreduced_output: pb_variable_array<FieldT, PB>,
pub  reduced_output: pb_variable_array<FieldT, PB>,
pub  reduce_output: Vec<lastbits_gadgets<FieldT, PB>>,

pub  prev_output: pb_linear_combination_array<FieldT, PB>,
pub  new_block: pb_variable_array<FieldT, PB>,
pub  output: digest_variables<FieldT, PB>,
}

/**
 * Gadget for the SHA256 compression function, viewed as a 2-to-1 hash
 * function, and using the same initialization vector as in SHA256
 * specification. Thus, any collision for
 * sha256_two_to_one_hash_gadget trivially extends to a collision for
 * full SHA256 (by appending the same padding).
 */

type hash_value_type = bit_vector;
// type merkle_authentication_path_type = merkle_authentication_path;
pub struct sha256_two_to_one_hash_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //gadget<FieldT>
 pub    f: RcCell<sha256_compression_function_gadgets<FieldT, PB>>,
}

pub type sha256_compression_function_gadgets<FieldT, PB> =
    gadget<FieldT, PB, sha256_compression_function_gadget<FieldT, PB> >;
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_compression_function_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        prev_output: pb_linear_combination_array<FieldT, PB>,
        new_block: pb_variable_array<FieldT, PB>,
        output: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> sha256_compression_function_gadgets<FieldT, PB> {
        let mut round_a = vec![];
        let mut round_b = vec![];
        let mut round_c = vec![];
        let mut round_d = vec![];
        let mut round_e = vec![];
        let mut round_f = vec![];
        let mut round_g = vec![];
        let mut round_h = vec![];
        let mut packed_W = pb_variable_array::<FieldT, PB>::default();
        let mut round_functions = vec![];

        let mut unreduced_output = pb_variable_array::<FieldT, PB>::default();
        let mut reduced_output = pb_variable_array::<FieldT, PB>::default();
        let mut reduce_output = vec![];

        /* message schedule and inputs for it */
        packed_W.allocate(&pb, 64, &prefix_format!(annotation_prefix, " packed_W"));
        let message_schedule = RcCell::new(sha256_message_schedule_gadget::<FieldT,PB>::new(
            pb.clone(),
            new_block.clone(),
            packed_W.clone(),
            prefix_format!(annotation_prefix, " message_schedule"),
        ));

        /* initalize */
        round_a.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(7 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_b.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(6 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_c.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(5 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_d.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(4 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_e.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(3 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_f.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(2 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_g.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .skip(1 * 32)
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));
        round_h.push(pb_linear_combination_array::<FieldT,PB>::new(
            prev_output
                .iter()
                .rev()
                .take(32)
                .cloned()
                .collect::<Vec<_>>(),
        ));

        /* do the rounds */
        for i in 0..64 {
            round_h.push(round_g[i].clone());
            round_g.push(round_f[i].clone());
            round_f.push(round_e[i].clone());
            round_d.push(round_c[i].clone());
            round_c.push(round_b[i].clone());
            round_b.push(round_a[i].clone());

            let mut new_round_a_variables = pb_variable_array::<FieldT,PB>::default();
            new_round_a_variables.allocate(
                &pb,
                32,
                &prefix_format!(annotation_prefix, " new_round_a_variables_{}", i + 1),
            );
            round_a.push(new_round_a_variables.into());

            let mut new_round_e_variables = pb_variable_array::<FieldT,PB>::default();
            new_round_e_variables.allocate(
                &pb,
                32,
                &prefix_format!(annotation_prefix, " new_round_e_variables_{}", i + 1),
            );
            round_e.push(new_round_e_variables.into());

            round_functions.push(sha256_round_function_gadget::<FieldT, PB>::new(
                pb.clone(),
                round_a[i].clone(),
                round_b[i].clone(),
                round_c[i].clone(),
                round_d[i].clone(),
                round_e[i].clone(),
                round_f[i].clone(),
                round_g[i].clone(),
                round_h[i].clone(),
                packed_W[i].clone(),
                SHA256_K[i] as i64,
                round_a[i + 1].clone(),
                round_e[i + 1].clone(),
                prefix_format!(annotation_prefix, " round_functions_{}", i),
            ));
        }

        /* finalize */
        unreduced_output.allocate(
            &pb,
            8,
            &prefix_format!(annotation_prefix, " unreduced_output"),
        );
        reduced_output.allocate(&pb, 8, &prefix_format!(annotation_prefix, " reduced_output"));
        for i in 0..8 {
            reduce_output.push(lastbits_gadget::<FieldT, PB>::new(
                pb.clone(),
                unreduced_output[i].clone(),
                32 + 1,
                reduced_output[i].clone(),
                pb_variable_array::<FieldT, PB>::new(
                    output
                        .t.bits
                        .iter()
                        .rev()
                        .skip((7 - i) * 32)
                        .take(32)
                        .cloned()
                        .collect::<Vec<_>>(),
                ).into(),
                prefix_format!(annotation_prefix, " reduce_output_{}", i),
            ));
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                round_a,
                round_b,
                round_c,
                round_d,
                round_e,
                round_f,
                round_g,
                round_h,
                packed_W,
                message_schedule,
                round_functions,
                unreduced_output,
                reduced_output,
                reduce_output,
                prev_output,
                new_block,
                output,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_compression_function_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.message_schedule.borrow().generate_r1cs_constraints();
        for i in 0..64 {
            self.t.round_functions[i]
                .generate_r1cs_constraints();
        }

        for i in 0..4 {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    1.into(),
                    (self.t.round_functions[3 - i].t.packed_d.clone()
                        + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(self.t.round_functions[63 - i].t.packed_new_a.clone()))
                    .into(),
                    self.t.unreduced_output[i].clone().into(),
                ),
                prefix_format!(self.annotation_prefix, " unreduced_output_{}", i),
            );

            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    1.into(),
                    (self.t.round_functions[3 - i].t.packed_h.clone()
                        + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(self.t.round_functions[63 - i].t.packed_new_e.clone()))
                    .into(),
                    self.t.unreduced_output[4 + i].clone().into(),
                ),
                prefix_format!(self.annotation_prefix, " unreduced_output_{}", 4 + i),
            );
        }

        for i in 0..8 {
            self.t.reduce_output[i].generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self)  where [(); { FieldT::num_limbs as usize }]:{
        self.t.message_schedule.borrow().generate_r1cs_witness();

        // #ifdef DEBUG
        print!("Input:\n");
        for j in 0..16 {
            print!("{} ", self.pb.borrow().val(&self.t.packed_W[j]).as_ulong());
        }
        print!("\n");
        //#endif

        for i in 0..64 {
            self.t.round_functions[i].generate_r1cs_witness();
        }

        for i in 0..4 {
            *self.pb.borrow_mut().val_ref(&self.t.unreduced_output[i]) = self
                .pb
                .borrow()
                .val(&self.t.round_functions[3 - i].t.packed_d)
                + self
                    .pb
                    .borrow()
                    .val(&self.t.round_functions[63 - i].t.packed_new_a);
            *self
                .pb
                .borrow_mut()
                .val_ref(&self.t.unreduced_output[4 + i]) = self
                .pb
                .borrow()
                .val(&self.t.round_functions[3 - i].t.packed_h)
                + self
                    .pb
                    .borrow()
                    .val(&self.t.round_functions[63 - i].t.packed_new_e);
        }

        for i in 0..8 {
            self.t.reduce_output[i].generate_r1cs_witness();
        }

        // #ifdef DEBUG
        print!("Output:\n");
        for j in 0..8 {
            print!(
                "{} ",
                self.pb.borrow().val(&self.t.reduced_output[j]).as_ulong()
            );
        }
        print!("\n");
        //#endif
    }
}

pub type sha256_two_to_one_hash_gadgets<FieldT, PB> =
    gadget<FieldT, PB, sha256_two_to_one_hash_gadget<FieldT, PB> >;

impl<FieldT: FieldTConfig, PB: PBConfig> sha256_two_to_one_hash_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        left: digest_variables<FieldT, PB>,
        right: digest_variables<FieldT, PB>,
        output: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> sha256_two_to_one_hash_gadgets<FieldT, PB> {
        /* concatenate block = left || right */
        let mut block = pb_variable_array::<FieldT, PB>::new(
            left.t.bits.iter().chain(right.t.bits.iter()).cloned().collect(),
        );

        /* compute the hash itself */
        let f = RcCell::new(sha256_compression_function_gadget::<FieldT, PB>::new(
            pb.clone(),
            SHA256_default_IV::<FieldT, PB>(&pb),
            block.clone(),
            output.clone(),
            prefix_format!(annotation_prefix, " f"),
        ));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { f })
    }

    pub fn new2(
        pb: RcCell<protoboard<FieldT, PB>>,
        block_length: usize,
        input_block: block_variables<FieldT,PB>,
        output: digest_variables<FieldT, PB>,
        annotation_prefix: String,
    ) -> sha256_two_to_one_hash_gadgets<FieldT, PB>  {
        assert!(block_length == SHA256_block_size);
        assert!(input_block.t.bits.len() == block_length);
        let f = RcCell::new(sha256_compression_function_gadget::<FieldT, PB>::new(
            pb.clone(),
            SHA256_default_IV::<FieldT, PB>(&pb),
            input_block.t.bits.clone(),
            output.clone(),
            prefix_format!(annotation_prefix, " f"),
        ));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { f })
    }

    pub fn get_block_len() -> usize {
        return SHA256_block_size;
    }

    pub fn get_digest_len() -> usize {
        return SHA256_digest_size;
    }

    pub fn get_hash(input: &bit_vector) -> bit_vector  where [(); { FieldT::num_limbs as usize }]:{
        let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

        let mut input_variable =
            block_variable::<FieldT, PB>::new(pb.clone(), SHA256_block_size, "input".to_owned());
        let mut output_variable =
            digest_variable::<FieldT, PB>::new(pb.clone(), SHA256_digest_size, "output".to_owned());
        let mut f = sha256_two_to_one_hash_gadget::<FieldT, PB>::new2(
            pb.clone(),
            SHA256_block_size,
            input_variable.clone(),
            output_variable.clone(),
            "f".to_owned(),
        );

        input_variable.generate_r1cs_witness(input);
        f.generate_r1cs_witness();

        return output_variable.get_digest();
    }

    pub fn expected_constraints(ensure_output_bitness: bool) -> usize {
        //ffec::UNUSED(ensure_output_bitness);
        return 27280; /* hardcoded for now */
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_two_to_one_hash_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self, ensure_output_bitness: bool) {
        // //ffec::UNUSED(ensure_output_bitness);
        self.t.f.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) where [(); { FieldT::num_limbs as usize }]:{
        self.t.f.borrow().generate_r1cs_witness();
    }
}
