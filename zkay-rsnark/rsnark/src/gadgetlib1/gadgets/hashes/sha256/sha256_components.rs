// Declaration of interfaces for gadgets for the SHA256 message schedule and round function.

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
use crate::gadgetlib1::pb_variable::pb_coeff_sum;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::r1cs_constraint;
use crate::relations::variable::{linear_combination, variable};
use ffec::FieldTConfig;
use ffec::common::utils::bit_vector;
use ffec::field_utils::field_utils::convert_field_element_to_bit_vector;
use parking_lot::Mutex;
use rccell::RcCell;
use std::marker::PhantomData;

pub const SHA256_digest_size: usize = 256;
pub const SHA256_block_size: usize = 512;

//
// pb_linear_combination_array<FieldT,PB> SHA256_default_IV(pb:RcCell<protoboard<FieldT,PB>> );
#[derive(Clone, Default)]
pub struct sha256_message_schedule_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget
    pub W_bits: Vec<pb_variable_array<FieldT, PB>>,
    pub pack_W: Vec<RcCell<packing_gadgets<FieldT, PB>>>,

    pub sigma0: Vec<variable<FieldT, pb_variable>>,
    pub sigma1: Vec<variable<FieldT, pb_variable>>,
    pub compute_sigma0: Vec<RcCell<small_sigma_gadgets<FieldT, PB>>>,
    pub compute_sigma1: Vec<RcCell<small_sigma_gadgets<FieldT, PB>>>,
    pub unreduced_W: Vec<variable<FieldT, pb_variable>>,
    pub mod_reduce_W: Vec<RcCell<lastbits_gadgets<FieldT, PB>>>,

    pub M: pb_variable_array<FieldT, PB>,
    pub packed_W: pb_variable_array<FieldT, PB>,
}
#[derive(Clone, Default)]
pub struct sha256_round_function_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget
    pub sigma0: variable<FieldT, pb_variable>,
    pub sigma1: variable<FieldT, pb_variable>,
    pub compute_sigma0: RcCell<big_sigma_gadgets<FieldT, PB>>,
    pub compute_sigma1: RcCell<big_sigma_gadgets<FieldT, PB>>,
    pub choice: variable<FieldT, pb_variable>,
    pub majority: variable<FieldT, pb_variable>,
    pub compute_choice: RcCell<choice_gadgets<FieldT, PB>>,
    pub compute_majority: RcCell<majority_gadgets<FieldT, PB>>,
    pub packed_d: variable<FieldT, pb_variable>,
    pub pack_d: RcCell<packing_gadgets<FieldT, PB>>,
    pub packed_h: variable<FieldT, pb_variable>,
    pub pack_h: RcCell<packing_gadgets<FieldT, PB>>,
    pub unreduced_new_a: variable<FieldT, pb_variable>,
    pub unreduced_new_e: variable<FieldT, pb_variable>,
    pub mod_reduce_new_a: RcCell<lastbits_gadgets<FieldT, PB>>,
    pub mod_reduce_new_e: RcCell<lastbits_gadgets<FieldT, PB>>,
    pub packed_new_a: variable<FieldT, pb_variable>,
    pub packed_new_e: variable<FieldT, pb_variable>,

    pub a: pb_linear_combination_array<FieldT, PB>,
    pub b: pb_linear_combination_array<FieldT, PB>,
    pub c: pb_linear_combination_array<FieldT, PB>,
    pub d: pb_linear_combination_array<FieldT, PB>,
    pub e: pb_linear_combination_array<FieldT, PB>,
    pub f: pb_linear_combination_array<FieldT, PB>,
    pub g: pb_linear_combination_array<FieldT, PB>,
    pub h: pb_linear_combination_array<FieldT, PB>,
    pub W: variable<FieldT, pb_variable>,
    pub K: i64,
    pub new_a: pb_linear_combination_array<FieldT, PB>,
    pub new_e: pb_linear_combination_array<FieldT, PB>,
}

pub const SHA256_K: [u64; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub const SHA256_H: [u64; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub fn SHA256_default_IV<FieldT: FieldTConfig, PB: PBConfig>(
    pb: &RcCell<protoboard<FieldT, PB>>,
) -> pb_linear_combination_array<FieldT, PB> {
    let mut result = pb_linear_combination_array::<FieldT, PB>::default();
    result.contents.reserve(SHA256_digest_size);

    for i in 0..SHA256_digest_size {
        let iv_val = (SHA256_H[i / 32] >> (31 - (i % 32))) & 1;

        let mut iv_element =
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default();
        iv_element.assign(
            pb,
            &(variable::<FieldT, pb_variable>::from(iv_val as usize * ONE).into()),
        );
        iv_element.evaluate_pb(pb);

        result.contents.push(iv_element);
    }

    return result;
}

pub type sha256_message_schedule_gadgets<FieldT, PB> =
    gadget<FieldT, PB, sha256_message_schedule_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_message_schedule_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        M: pb_variable_array<FieldT, PB>,
        packed_W: pb_variable_array<FieldT, PB>,
        annotation_prefix: String,
    ) -> sha256_message_schedule_gadgets<FieldT, PB> {
        let mut W_bits = vec![pb_variable_array::<FieldT, PB>::default(); 64];
        let mut pack_W = vec![RcCell::new(packing_gadgets::<FieldT, PB>::default()); 16];
        for i in 0..16 {
            W_bits[i] = pb_variable_array::<FieldT, PB>::new(
                M.iter()
                    .rev()
                    .skip((15 - i) * 32)
                    .take(32)
                    .cloned()
                    .collect::<Vec<_>>(),
            );
            pack_W[i] = RcCell::new(packing_gadget::<FieldT, PB>::new(
                pb.clone(),
                W_bits[i].clone().into(),
                packed_W[i].clone().into(),
                prefix_format!(annotation_prefix, " pack_W_{}", i),
            ));
        }

        /* NB: some of those will be un-allocated */
        let mut sigma0 = vec![variable::<FieldT, pb_variable>::default(); 64];
        let mut sigma1 = vec![variable::<FieldT, pb_variable>::default(); 64];
        let mut compute_sigma0 =
            vec![RcCell::new(small_sigma_gadgets::<FieldT, PB>::default()); 64];
        let mut compute_sigma1 =
            vec![RcCell::new(small_sigma_gadgets::<FieldT, PB>::default()); 64];
        let mut unreduced_W = vec![variable::<FieldT, pb_variable>::default(); 64];
        let mut mod_reduce_W = vec![RcCell::new(lastbits_gadgets::<FieldT, PB>::default()); 64];

        for i in 16..64 {
            /* allocate result variables for sigma0/sigma1 invocations */
            sigma0[i].allocate(&pb, prefix_format!(annotation_prefix, " sigma0_{}", i));
            sigma1[i].allocate(&pb, prefix_format!(annotation_prefix, " sigma1_{}", i));

            /* compute sigma0/sigma1 */
            compute_sigma0[i] = RcCell::new(small_sigma_gadget::<FieldT, PB>::new(
                pb.clone(),
                W_bits[i - 15].clone(),
                sigma0[i].clone(),
                7,
                18,
                3,
                prefix_format!(annotation_prefix, " compute_sigma0_{}", i),
            ));
            compute_sigma1[i] = RcCell::new(small_sigma_gadget::<FieldT, PB>::new(
                pb.clone(),
                W_bits[i - 2].clone(),
                sigma1[i].clone(),
                17,
                19,
                10,
                prefix_format!(annotation_prefix, " compute_sigma1_{}", i),
            ));

            /* unreduced_W = sigma0(W_{i-15}) + sigma1(W_{i-2}) + W_{i-7} + W_{i-16} before modulo 2^32 */
            unreduced_W[i].allocate(&pb, prefix_format!(annotation_prefix, " unreduced_W_{}", i));

            /* allocate the bit representation of packed_W[i] */
            W_bits[i].allocate(&pb, 32, prefix_format!(annotation_prefix, " W_bits_{}", i));

            /* and finally reduce this into packed and bit representations */
            mod_reduce_W[i] = RcCell::new(lastbits_gadget::<FieldT, PB>::new(
                pb.clone(),
                unreduced_W[i].clone(),
                32 + 2,
                packed_W[i].clone(),
                W_bits[i].clone().into(),
                prefix_format!(annotation_prefix, " mod_reduce_W_{}", i),
            ));
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                W_bits,
                pack_W,
                sigma0,
                sigma1,
                compute_sigma0,
                compute_sigma1,
                unreduced_W,
                mod_reduce_W,
                M,
                packed_W,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_message_schedule_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..16 {
            self.t.pack_W[i].borrow().generate_r1cs_constraints(false); // do not enforce bitness here; caller be aware.
        }

        for i in 16..64 {
            self.t.compute_sigma0[i]
                .borrow()
                .generate_r1cs_constraints();
            self.t.compute_sigma1[i]
                .borrow()
                .generate_r1cs_constraints();

            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    FieldT::from(1).into(),
                    (self.t.sigma0[i].clone()
                        + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                            self.t.sigma1[i].clone(),
                        )
                        + self.t.packed_W[i - 16].clone()
                        + self.t.packed_W[i - 7].clone())
                    .into(),
                    self.t.unreduced_W[i].clone().into(),
                ),
                prefix_format!(self.annotation_prefix, " unreduced_W_{}", i),
            );

            self.t.mod_reduce_W[i].borrow().generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..16 {
            self.t.pack_W[i].borrow().generate_r1cs_witness_from_bits();
        }

        for i in 16..64 {
            self.t.compute_sigma0[i].borrow().generate_r1cs_witness();
            self.t.compute_sigma1[i].borrow().generate_r1cs_witness();

            *self.pb.borrow_mut().val_ref(&self.t.unreduced_W[i]) =
                self.pb.borrow().val(&self.t.sigma0[i])
                    + self.pb.borrow().val(&self.t.sigma1[i])
                    + self.pb.borrow().val(&self.t.packed_W[i - 16])
                    + self.pb.borrow().val(&self.t.packed_W[i - 7]);
            self.t.mod_reduce_W[i].borrow().generate_r1cs_witness();
        }
    }
}

pub type sha256_round_function_gadgets<FieldT, PB> =
    gadget<FieldT, PB, sha256_round_function_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> sha256_round_function_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        a: pb_linear_combination_array<FieldT, PB>,
        b: pb_linear_combination_array<FieldT, PB>,
        c: pb_linear_combination_array<FieldT, PB>,
        d: pb_linear_combination_array<FieldT, PB>,
        e: pb_linear_combination_array<FieldT, PB>,
        f: pb_linear_combination_array<FieldT, PB>,
        g: pb_linear_combination_array<FieldT, PB>,
        h: pb_linear_combination_array<FieldT, PB>,
        W: variable<FieldT, pb_variable>,
        K: i64,
        new_a: pb_linear_combination_array<FieldT, PB>,
        new_e: pb_linear_combination_array<FieldT, PB>,
        annotation_prefix: String,
    ) -> sha256_round_function_gadgets<FieldT, PB> {
        let mut sigma0 = variable::<FieldT, pb_variable>::default();
        let mut sigma1 = variable::<FieldT, pb_variable>::default();
        let mut choice = variable::<FieldT, pb_variable>::default();
        let mut majority = variable::<FieldT, pb_variable>::default();
        let mut packed_d = variable::<FieldT, pb_variable>::default();
        let mut packed_h = variable::<FieldT, pb_variable>::default();
        let mut unreduced_new_a = variable::<FieldT, pb_variable>::default();
        let mut unreduced_new_e = variable::<FieldT, pb_variable>::default();
        let mut packed_new_a = variable::<FieldT, pb_variable>::default();
        let mut packed_new_e = variable::<FieldT, pb_variable>::default();

        /* compute sigma0 and sigma1 */
        sigma0.allocate(&pb, prefix_format!(annotation_prefix, " sigma0"));
        sigma1.allocate(&pb, prefix_format!(annotation_prefix, " sigma1"));
        let compute_sigma0 = RcCell::new(big_sigma_gadget::<FieldT, PB>::new(
            pb.clone(),
            a.clone(),
            sigma0.clone(),
            2,
            13,
            22,
            prefix_format!(annotation_prefix, " compute_sigma0"),
        ));
        let compute_sigma1 = RcCell::new(big_sigma_gadget::<FieldT, PB>::new(
            pb.clone(),
            e.clone(),
            sigma1.clone(),
            6,
            11,
            25,
            prefix_format!(annotation_prefix, " compute_sigma1"),
        ));

        /* compute choice */
        choice.allocate(&pb, prefix_format!(annotation_prefix, " choice"));
        let compute_choice = RcCell::new(choice_gadget::<FieldT, PB>::new(
            pb.clone(),
            e.clone(),
            f.clone(),
            g.clone(),
            choice.clone(),
            prefix_format!(annotation_prefix, " compute_choice"),
        ));

        /* compute majority */
        majority.allocate(&pb, prefix_format!(annotation_prefix, " majority"));
        let compute_majority = RcCell::new(majority_gadget::<FieldT, PB>::new(
            pb.clone(),
            a.clone(),
            b.clone(),
            c.clone(),
            majority.clone(),
            prefix_format!(annotation_prefix, " compute_majority"),
        ));

        /* pack d */
        packed_d.allocate(&pb, prefix_format!(annotation_prefix, " packed_d"));
        let pack_d = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            d.clone(),
            packed_d.clone().into(),
            prefix_format!(annotation_prefix, " pack_d"),
        ));

        /* pack h */
        packed_h.allocate(&pb, prefix_format!(annotation_prefix, " packed_h"));
        let pack_h = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            h.clone(),
            packed_h.clone().into(),
            prefix_format!(annotation_prefix, " pack_h"),
        ));

        /* compute the actual results for the round */
        unreduced_new_a.allocate(&pb, prefix_format!(annotation_prefix, " unreduced_new_a"));
        unreduced_new_e.allocate(&pb, prefix_format!(annotation_prefix, " unreduced_new_e"));

        packed_new_a.allocate(&pb, prefix_format!(annotation_prefix, " packed_new_a"));
        packed_new_e.allocate(&pb, prefix_format!(annotation_prefix, " packed_new_e"));

        let mod_reduce_new_a = RcCell::new(lastbits_gadget::<FieldT, PB>::new(
            pb.clone(),
            unreduced_new_a.clone(),
            32 + 3,
            packed_new_a.clone(),
            new_a.clone(),
            prefix_format!(annotation_prefix, " mod_reduce_new_a"),
        ));
        let mod_reduce_new_e = RcCell::new(lastbits_gadget::<FieldT, PB>::new(
            pb.clone(),
            unreduced_new_e.clone(),
            32 + 3,
            packed_new_e.clone(),
            new_e.clone(),
            prefix_format!(annotation_prefix, " mod_reduce_new_e"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                sigma0,
                sigma1,
                compute_sigma0,
                compute_sigma1,
                choice,
                majority,
                compute_choice,
                compute_majority,
                packed_d,
                pack_d,
                packed_h,
                pack_h,
                unreduced_new_a,
                unreduced_new_e,
                mod_reduce_new_a,
                mod_reduce_new_e,
                packed_new_a,
                packed_new_e,
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                W,
                K,
                new_a,
                new_e,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> sha256_round_function_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.compute_sigma0.borrow().generate_r1cs_constraints();
        self.t.compute_sigma1.borrow().generate_r1cs_constraints();

        self.t.compute_choice.borrow().generate_r1cs_constraints();
        self.t.compute_majority.borrow().generate_r1cs_constraints();

        self.t.pack_d.borrow().generate_r1cs_constraints(false);
        self.t.pack_h.borrow().generate_r1cs_constraints(false);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                (self.t.packed_h.clone()
                    + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.sigma1.clone(),
                    )
                    + self.t.choice.clone()
                    + FieldT::from(self.t.K.clone())
                    + self.t.W.clone()
                    + self.t.sigma0.clone()
                    + self.t.majority.clone())
                .into(),
                self.t.unreduced_new_a.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " unreduced_new_a"),
        );

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                FieldT::from(1).into(),
                (self.t.packed_d.clone()
                    + linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.packed_h.clone(),
                    )
                    + self.t.sigma1.clone()
                    + self.t.choice.clone()
                    + FieldT::from(self.t.K.clone())
                    + self.t.W.clone())
                .into(),
                self.t.unreduced_new_e.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " unreduced_new_e"),
        );

        self.t.mod_reduce_new_a.borrow().generate_r1cs_constraints();
        self.t.mod_reduce_new_e.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.compute_sigma0.borrow().generate_r1cs_witness();
        self.t.compute_sigma1.borrow().generate_r1cs_witness();

        self.t.compute_choice.borrow().generate_r1cs_witness();
        self.t.compute_majority.borrow().generate_r1cs_witness();

        self.t.pack_d.borrow().generate_r1cs_witness_from_bits();
        self.t.pack_h.borrow().generate_r1cs_witness_from_bits();

        *self.pb.borrow_mut().val_ref(&self.t.unreduced_new_a) =
            self.pb.borrow().val(&self.t.packed_h)
                + self.pb.borrow().val(&self.t.sigma1)
                + self.pb.borrow().val(&self.t.choice)
                + FieldT::from(self.t.K)
                + self.pb.borrow().val(&self.t.W)
                + self.pb.borrow().val(&self.t.sigma0)
                + self.pb.borrow().val(&self.t.majority);
        *self.pb.borrow_mut().val_ref(&self.t.unreduced_new_e) =
            self.pb.borrow().val(&self.t.packed_d)
                + self.pb.borrow().val(&self.t.packed_h)
                + self.pb.borrow().val(&self.t.sigma1)
                + self.pb.borrow().val(&self.t.choice)
                + FieldT::from(self.t.K)
                + self.pb.borrow().val(&self.t.W);

        self.t.mod_reduce_new_a.borrow().generate_r1cs_witness();
        self.t.mod_reduce_new_e.borrow().generate_r1cs_witness();
    }
}
