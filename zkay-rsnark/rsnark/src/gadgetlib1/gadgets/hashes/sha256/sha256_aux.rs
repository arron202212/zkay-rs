// Declaration of interfaces for auxiliary gadgets for the SHA256 gadget.

use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::basic_gadgets::{
    generate_boolean_r1cs_constraint, packing_gadget, packing_gadgets,
};
use crate::gadgetlib1::gadgets::hashes::hash_io::{
    block_variable, block_variables, digest_variable, digest_variables,
};
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

#[derive(Clone, Default)]
pub struct lastbits_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<FieldT>
    X: variable<FieldT, pb_variable>,
    X_bits: usize,
    result: variable<FieldT, pb_variable>,
    result_bits: pb_linear_combination_array<FieldT, PB>,
    full_bits: pb_linear_combination_array<FieldT, PB>,
    unpack_bits: RcCell<packing_gadgets<FieldT, PB>>,
    pack_result: RcCell<packing_gadgets<FieldT, PB>>,
}

#[derive(Clone, Default)]
pub struct XOR3_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // : public gadget<FieldT>
    tmp: variable<FieldT, pb_variable>,
    A: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    B: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    C: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    assume_C_is_zero: bool,
    out: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    _t: PhantomData<PB>,
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
#[derive(Clone, Default)]
pub struct small_sigma_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget
    W: pb_variable_array<FieldT, PB>,
    result: variable<FieldT, pb_variable>,
    result_bits: pb_variable_array<FieldT, PB>,
    compute_bits: Vec<RcCell<XOR3_gadgets<FieldT, PB>>>,
    pack_result: RcCell<packing_gadgets<FieldT, PB>>,
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
#[derive(Clone, Default)]
pub struct big_sigma_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget
    W: pb_linear_combination_array<FieldT, PB>,
    result: variable<FieldT, pb_variable>,
    result_bits: pb_variable_array<FieldT, PB>,
    compute_bits: Vec<RcCell<XOR3_gadgets<FieldT, PB>>>,
    pack_result: RcCell<packing_gadgets<FieldT, PB>>,
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
#[derive(Clone, Default)]
pub struct choice_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    // // : public gadget
    result_bits: pb_variable_array<FieldT, PB>,
    X: pb_linear_combination_array<FieldT, PB>,
    Y: pb_linear_combination_array<FieldT, PB>,
    Z: pb_linear_combination_array<FieldT, PB>,
    result: variable<FieldT, pb_variable>,
    pack_result: RcCell<packing_gadgets<FieldT, PB>>,
}

/* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */
#[derive(Clone, Default)]
pub struct majority_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //  : public gadget
    result_bits: pb_variable_array<FieldT, PB>,
    pack_result: RcCell<packing_gadgets<FieldT, PB>>,
    X: pb_linear_combination_array<FieldT, PB>,
    Y: pb_linear_combination_array<FieldT, PB>,
    Z: pb_linear_combination_array<FieldT, PB>,
    result: variable<FieldT, pb_variable>,
}
pub type lastbits_gadgets<FieldT, PB> = gadget<FieldT, PB, lastbits_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> lastbits_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        X: variable<FieldT, pb_variable>,
        X_bits: usize,
        result: variable<FieldT, pb_variable>,
        result_bits: pb_linear_combination_array<FieldT, PB>,
        annotation_prefix: String,
    ) -> lastbits_gadgets<FieldT, PB> {
        let mut full_bits = result_bits.clone();
        for i in result_bits.len()..X_bits {
            let mut full_bits_overflow = variable::<FieldT, pb_variable>::default();
            full_bits_overflow.allocate(&pb, prefix_format!(annotation_prefix, " full_bits_{}", i));
            full_bits.contents.push(full_bits_overflow.into());
        }

        let unpack_bits = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            full_bits.clone(),
            X.clone().into(),
            prefix_format!(annotation_prefix, " unpack_bits"),
        ));
        let pack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            result_bits.clone(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " pack_result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                X_bits,
                result,
                result_bits,
                full_bits,
                unpack_bits,
                pack_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> lastbits_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        self.t.unpack_bits.borrow().generate_r1cs_constraints(true);
        self.t.pack_result.borrow().generate_r1cs_constraints(false);
    }

    pub fn generate_r1cs_witness(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.t
            .unpack_bits
            .borrow()
            .generate_r1cs_witness_from_packed();
        self.t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}

pub type XOR3_gadgets<FieldT, PB> = gadget<FieldT, PB, XOR3_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> XOR3_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        B: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        C: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        assume_C_is_zero: bool,
        out: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> XOR3_gadgets<FieldT, PB> {
        let mut tmp = variable::<FieldT, pb_variable>::default();
        if !assume_C_is_zero {
            tmp.allocate(&pb, prefix_format!(annotation_prefix, " tmp"));
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                tmp,
                A,
                B,
                C,
                assume_C_is_zero,
                out,
                _t: PhantomData,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> XOR3_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        /*
          tmp = A + B - 2AB i.e. tmp = A xor B
          out = tmp + C - 2tmp C i.e. out = tmp xor C
        */
        if self.t.assume_C_is_zero {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    (self.t.A.clone() * 2).into(),
                    self.t.B.clone(),
                    self.t.A.clone() + self.t.B.clone() - self.t.out.clone(),
                ),
                prefix_format!(self.annotation_prefix, " implicit_tmp_equals_out"),
            );
        } else {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    (self.t.A.clone() * 2).into(),
                    self.t.B.clone(),
                    self.t.A.clone() + self.t.B.clone() - self.t.tmp.clone(),
                ),
                prefix_format!(self.annotation_prefix, " tmp"),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    (self.t.tmp.clone() * 2).into(),
                    self.t.C.clone(),
                    self.t.tmp.clone() + self.t.C.clone() - self.t.out.clone(),
                ),
                prefix_format!(self.annotation_prefix, " out"),
            );
        }
    }

    pub fn generate_r1cs_witness(&self) {
        if self.t.assume_C_is_zero {
            *self.pb.borrow_mut().lc_val_ref(&self.t.out) = self.pb.borrow().lc_val(&self.t.A)
                + self.pb.borrow().lc_val(&self.t.B)
                - FieldT::from(2)
                    * self.pb.borrow().lc_val(&self.t.A)
                    * self.pb.borrow().lc_val(&self.t.B);
        } else {
            *self
                .pb
                .borrow_mut()
                .lc_val_ref(&(self.t.tmp.clone().into())) = self.pb.borrow().lc_val(&self.t.A)
                + self.pb.borrow().lc_val(&self.t.B)
                - FieldT::from(2)
                    * self.pb.borrow().lc_val(&self.t.A)
                    * self.pb.borrow().lc_val(&self.t.B);
            *self.pb.borrow_mut().lc_val_ref(&self.t.out) = self.pb.borrow().val(&self.t.tmp)
                + self.pb.borrow().lc_val(&self.t.C)
                - FieldT::from(2)
                    * self.pb.borrow().val(&self.t.tmp)
                    * self.pb.borrow().lc_val(&self.t.C);
        }
    }
}
// #define SHA256_GADGET_ROTR(A, i, k) A[((i)+(k)) % 32]
macro_rules! SHA256_GADGET_ROTR {
    ($A: expr,$i:expr,$k:expr) => {
        $A[(($i) + ($k)) % 32]
    };
}

pub type small_sigma_gadgets<FieldT, PB> = gadget<FieldT, PB, small_sigma_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> small_sigma_gadget<FieldT, PB> {
    /* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        W: pb_variable_array<FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        rot1: usize,
        rot2: usize,
        shift: usize,
        annotation_prefix: String,
    ) -> small_sigma_gadgets<FieldT, PB> {
        let mut result_bits = pb_variable_array::<FieldT, PB>::default();
        result_bits.allocate(&pb, 32, &prefix_format!(annotation_prefix, " result_bits"));
        let compute_bits: Vec<_> = (0..32)
            .map(|i| {
                RcCell::new(XOR3_gadget::<FieldT, PB>::new(
                    pb.clone(),
                    SHA256_GADGET_ROTR!(W, i, rot1).clone().into(),
                    SHA256_GADGET_ROTR!(W, i, rot2).clone().into(),
                    if i + shift < 32 {
                        W[i + shift].clone().into()
                    } else {
                        variable::<FieldT, pb_variable>::from(ONE).into()
                    },
                    (i + shift >= 32),
                    result_bits[i].clone().into(),
                    prefix_format!(annotation_prefix, " compute_bits_{}", i),
                ))
            })
            .collect();
        let pack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            result_bits.clone().into(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " pack_result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                W,
                result,
                result_bits,
                compute_bits,
                pack_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> small_sigma_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..32 {
            self.t.compute_bits[i].borrow().generate_r1cs_constraints();
        }

        self.t.pack_result.borrow().generate_r1cs_constraints(false);
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..32 {
            self.t.compute_bits[i].borrow().generate_r1cs_witness();
        }

        self.t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}

pub type big_sigma_gadgets<FieldT, PB> = gadget<FieldT, PB, big_sigma_gadget<FieldT, PB>>;
impl<FieldT: FieldTConfig, PB: PBConfig> big_sigma_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        W: pb_linear_combination_array<FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        rot1: usize,
        rot2: usize,
        rot3: usize,
        annotation_prefix: String,
    ) -> big_sigma_gadgets<FieldT, PB> {
        let mut result_bits = pb_variable_array::<FieldT, PB>::default();
        result_bits.allocate(&pb, 32, &prefix_format!(annotation_prefix, " result_bits"));
        let compute_bits: Vec<_> = (0..32)
            .map(|i| {
                RcCell::new(XOR3_gadget::<FieldT, PB>::new(
                    pb.clone(),
                    SHA256_GADGET_ROTR!(W, i, rot1).clone(),
                    SHA256_GADGET_ROTR!(W, i, rot2).clone(),
                    SHA256_GADGET_ROTR!(W, i, rot3).clone(),
                    false,
                    result_bits[i].clone().into(),
                    prefix_format!(annotation_prefix, " compute_bits_{}", i),
                ))
            })
            .collect();

        let pack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            result_bits.clone().into(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " pack_result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                W,
                result,
                result_bits,
                compute_bits,
                pack_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> big_sigma_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..32 {
            self.t.compute_bits[i].borrow().generate_r1cs_constraints();
        }

        self.t.pack_result.borrow().generate_r1cs_constraints(false);
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..32 {
            self.t.compute_bits[i].borrow().generate_r1cs_witness();
        }

        self.t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}

pub type choice_gadgets<FieldT, PB> = gadget<FieldT, PB, choice_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> choice_gadget<FieldT, PB> {
    /* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        X: pb_linear_combination_array<FieldT, PB>,
        Y: pb_linear_combination_array<FieldT, PB>,
        Z: pb_linear_combination_array<FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> choice_gadgets<FieldT, PB> {
        let mut result_bits = pb_variable_array::<FieldT, PB>::default();
        result_bits.allocate(&pb, 32, &prefix_format!(annotation_prefix, " result_bits"));
        let pack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            result_bits.clone().into(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                Y,
                Z,
                result,
                result_bits,
                pack_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> choice_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..32 {
            /*
              result = x * y + (1-x) * z
              result - z = x * (y - z)
            */
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.X[i].clone(),
                    self.t.Y[i].clone() - self.t.Z[i].clone(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.result_bits[i].clone(),
                    ) - self.t.Z[i].clone(),
                ),
                prefix_format!(self.annotation_prefix, " result_bits_{}", i),
            );
        }
        self.t.pack_result.borrow().generate_r1cs_constraints(false);
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..32 {
            *self.pb.borrow_mut().val_ref(&self.t.result_bits[i]) =
                self.pb.borrow().lc_val(&self.t.X[i]) * self.pb.borrow().lc_val(&self.t.Y[i])
                    + (FieldT::one() - self.pb.borrow().lc_val(&self.t.X[i]))
                        * self.pb.borrow().lc_val(&self.t.Z[i]);
        }
        self.t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}

pub type majority_gadgets<FieldT, PB> = gadget<FieldT, PB, majority_gadget<FieldT, PB>>;

impl<FieldT: FieldTConfig, PB: PBConfig> majority_gadget<FieldT, PB> {
    /* Page 10 of http://csrc.nist.gov/publications/fips/fips180-4/fips-180-4.pdf */

    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        X: pb_linear_combination_array<FieldT, PB>,
        Y: pb_linear_combination_array<FieldT, PB>,
        Z: pb_linear_combination_array<FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> majority_gadgets<FieldT, PB> {
        let mut result_bits = pb_variable_array::<FieldT, PB>::default();
        result_bits.allocate(&pb, 32, &prefix_format!(annotation_prefix, " result_bits"));
        let pack_result = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            result_bits.clone().into(),
            result.clone().into(),
            prefix_format!(annotation_prefix, " result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                X,
                Y,
                Z,
                result,
                result_bits,
                pack_result,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> majority_gadgets<FieldT, PB> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..32 {
            /*
              2*result + aux = x + y + z
              x, y, z, aux -- bits
              aux = x + y + z - 2*result
            */
            generate_boolean_r1cs_constraint::<FieldT, PB>(
                &self.pb,
                &(self.t.result_bits[i].clone().into()),
                prefix_format!(self.annotation_prefix, " result_{}", i),
            );
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.X[i].clone() + self.t.Y[i].clone() + self.t.Z[i].clone()
                        - self.t.result_bits[i].clone() * 2,
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(1)
                        - (self.t.X[i].clone() + self.t.Y[i].clone() + self.t.Z[i].clone()
                            - self.t.result_bits[i].clone() * 2),
                    0.into(),
                ),
                prefix_format!(self.annotation_prefix, " result_bits_{}", i),
            );
        }
        self.t.pack_result.borrow().generate_r1cs_constraints(false);
    }

    pub fn generate_r1cs_witness(&self) {
        for i in 0..32 {
            let v = (self.pb.borrow().lc_val(&self.t.X[i])
                + self.pb.borrow().lc_val(&self.t.Y[i])
                + self.pb.borrow().lc_val(&self.t.Z[i]))
            .as_ulong();
            *self.pb.borrow_mut().val_ref(&self.t.result_bits[i]) = FieldT::from(v / 2);
        }

        self.t
            .pack_result
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}
