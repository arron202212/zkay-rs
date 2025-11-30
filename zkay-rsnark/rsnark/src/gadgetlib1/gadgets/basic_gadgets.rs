use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::pb_variable::{
    ONE, pb_linear_combination, pb_linear_combination_array, pb_packing_sum, pb_variable,
    pb_variable_array,
};
use crate::gadgetlib1::protoboard::{PBConfig, protoboard};
use crate::relations::FieldTConfig;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
    r1cs_variable_assignment,
};
use crate::relations::variable::{linear_combination, variable};
use ffec::common::utils::div_ceil;
use ffec::field_utils::bigint::bigint;
use rccell::RcCell;
use std::marker::PhantomData;
pub fn FMT(s: &String, c: &str) {}

#[macro_export]
macro_rules! prefix_format {
    ("",$fmt:expr $(, $($arg:tt)*)?) => {
        format!($fmt, $($($arg)*)?)
    };
    ($prefix:expr, $fmt:expr $(, $($arg:tt)*)?) => {
            format!(concat!("{}", $fmt),$prefix,$($($arg)*)?)
    };

}

/* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
//
// pub fn generate_boolean_r1cs_constraint(pb:&RcCell<protoboard<FieldT,PB>> , lc:&linear_combination<FieldT,pb_variable,pb_linear_combination>, annotation_prefix:&String);

//
// pub fn generate_r1cs_equals_const_constraint(pb:&RcCell<protoboard<FieldT,PB>> , lc:&linear_combination<FieldT,pb_variable,pb_linear_combination>, annotation_prefix:&FieldT& c,  String);
#[derive(Clone, Default)]
pub struct packing_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    /* no internal variables */
    pub bits: pb_linear_combination_array<FieldT, PB>,
    pub packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
}

impl<FieldT: FieldTConfig, PB: PBConfig> packing_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        bits: pb_linear_combination_array<FieldT, PB>,
        packed: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { bits, packed })
    }

    // pub fn generate_r1cs_constraints(enforce_bitness:bool);
    // /* adds constraint result = \sum  bits[i] * 2^i */
    // pub fn generate_r1cs_witness_from_packed();
    // pub fn generate_r1cs_witness_from_bits();
}

#[derive(Clone, Default)]
pub struct multipacking_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub packers: Vec<gadget<FieldT, PB, packing_gadget<FieldT, PB>>>,
    pub bits: pb_linear_combination_array<FieldT, PB>,
    pub packed_vars: pb_linear_combination_array<FieldT, PB>,
    pub chunk_size: usize,
    pub num_chunks: usize,
    pub last_chunk_size: usize,
}
// impl multipacking_gadget {
//     multipacking_gadget(pb:&RcCell<protoboard<FieldT,PB>> ,
//                         bits:&pb_linear_combination_array<FieldT,PB>,
//                         packed_vars:&pb_linear_combination_array<FieldT,PB>,
//                         chunk_size:usize,
//                         annotation_prefix:&String);
//     // pub fn generate_r1cs_constraints(enforce_bitness:bool);
//     // pub fn generate_r1cs_witness_from_packed();
//     // pub fn generate_r1cs_witness_from_bits();
// }

#[derive(Clone, Default)]
pub struct field_vector_copy_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub source: pb_variable_array<FieldT, PB>,
    pub target: pb_variable_array<FieldT, PB>,
    pub do_copy: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    // field_vector_copy_gadget(pb:&RcCell<protoboard<FieldT,PB>> ,
    //                          source:&pb_variable_array<FieldT,PB>,
    //                          target:&pb_variable_array<FieldT,PB>,
    //                          do_copy:&linear_combination<FieldT,pb_variable,pb_linear_combination>,
    //                          annotation_prefix:&String);
    // pub fn generate_r1cs_constraints(&self);
    // pub fn generate_r1cs_witness(&self);
}

#[derive(Clone, Default)]
pub struct bit_vector_copy_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub source_bits: pb_variable_array<FieldT, PB>,
    pub target_bits: pb_variable_array<FieldT, PB>,
    pub do_copy: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub packed_source: pb_variable_array<FieldT, PB>,
    pub packed_target: pb_variable_array<FieldT, PB>,
    pub pack_source: RcCell<gadget<FieldT, PB, multipacking_gadget<FieldT, PB>>>,
    pub pack_target: RcCell<gadget<FieldT, PB, multipacking_gadget<FieldT, PB>>>,
    pub copier: RcCell<gadget<FieldT, PB, field_vector_copy_gadget<FieldT, PB>>>,
    pub chunk_size: usize,
    pub num_chunks: usize,
    // bit_vector_copy_gadget(pb:&RcCell<protoboard<FieldT,PB>> ,
    //                        source_bits:&pb_variable_array<FieldT,PB>,
    //                        target_bits:&pb_variable_array<FieldT,PB>,
    //                        do_copy:&linear_combination<FieldT,pb_variable,pb_linear_combination>,
    //                        chunk_size:usize,
    //                        annotation_prefix:&String);
    // pub fn generate_r1cs_constraints(enforce_source_bitness:bool, enforce_target_bitness:bool);
    // pub fn generate_r1cs_witness(&self);
}

#[derive(Clone, Default)]
pub struct dual_variable_gadget<FieldT: FieldTConfig, PB: PBConfig, T> {
    pub consistency_check: RcCell<gadget<FieldT, PB, packing_gadget<FieldT, PB>>>,
    pub packed: variable<FieldT, pb_variable>,
    pub bits: pb_variable_array<FieldT, PB>,
    pub t: T,
}
impl<FieldT: FieldTConfig, PB: PBConfig, T> dual_variable_gadget<FieldT, PB, T> {
    pub fn new(
        mut pb: RcCell<protoboard<FieldT, PB>>,
        width: usize,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, PB, Self> {
        let mut packed = variable::<FieldT, pb_variable>::default();
        packed.allocate(&pb, prefix_format!(annotation_prefix, " packed"));
        let mut bits = pb_variable_array::<FieldT, PB>::default();
        bits.allocate(&pb, width, &prefix_format!(annotation_prefix, " bits"));
        let consistency_check = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            bits.clone().into(),
            packed.clone().into(),
            prefix_format!(annotation_prefix, " consistency_check"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                consistency_check,
                packed,
                bits,
                t,
            },
        )
    }

    pub fn new_with_bits(
        pb: RcCell<protoboard<FieldT, PB>>,
        bits: pb_variable_array<FieldT, PB>,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, PB, Self> {
        let mut packed = variable::<FieldT, pb_variable>::default();
        packed.allocate(&pb, prefix_format!(annotation_prefix, " packed"));
        let consistency_check = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            bits.clone().into(),
            packed.clone().into(),
            prefix_format!(annotation_prefix, " consistency_check"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                consistency_check,
                packed,
                bits,
                t,
            },
        )
    }

    pub fn new_with_width(
        pb: RcCell<protoboard<FieldT, PB>>,
        packed: variable<FieldT, pb_variable>,
        width: usize,
        annotation_prefix: String,
        t: T,
    ) -> gadget<FieldT, PB, Self> {
        let mut bits = pb_variable_array::<FieldT, PB>::default();
        bits.allocate(&pb, width, &prefix_format!(annotation_prefix, " bits"));
        let consistency_check = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            bits.clone().into(),
            packed.clone().into(),
            prefix_format!(annotation_prefix, " consistency_check"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                consistency_check,
                packed,
                bits,
                t,
            },
        )
    }

    // pub fn generate_r1cs_constraints(enforce_bitness:bool);
    // pub fn generate_r1cs_witness_from_packed();
    // pub fn generate_r1cs_witness_from_bits();
}

/*
  the gadgets below are Fp specific:
  I * X = R
  (1-R) * X = 0

  if X = 0 then R = 0
  if X != 0 then R = 1 and I = X^{-1}
*/
#[derive(Clone, Default)]
pub struct disjunction_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub inv: variable<FieldT, pb_variable>,
    pub inputs: pb_variable_array<FieldT, PB>,
    pub output: variable<FieldT, pb_variable>,
}
impl<FieldT: FieldTConfig, PB: PBConfig> disjunction_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        inputs: pb_variable_array<FieldT, PB>,
        output: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut inv = variable::<FieldT, pb_variable>::default();
        assert!(inputs.len() >= 1);
        inv.allocate(&pb, prefix_format!(annotation_prefix, " inv"));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                inv,
                inputs,
                output,
            },
        )
    }

    // pub fn generate_r1cs_constraints(&self);
    // pub fn generate_r1cs_witness(&self);
}
#[derive(Clone, Default)]
pub struct conjunction_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub inv: variable<FieldT, pb_variable>,
    pub inputs: pb_variable_array<FieldT, PB>,
    pub output: variable<FieldT, pb_variable>,
}
impl<FieldT: FieldTConfig, PB: PBConfig> conjunction_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        inputs: pb_variable_array<FieldT, PB>,
        output: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut inv = variable::<FieldT, pb_variable>::default();
        assert!(inputs.len() >= 1);
        inv.allocate(&pb, prefix_format!(annotation_prefix, " inv"));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                inv,
                inputs,
                output,
            },
        )
    }

    //     pub fn generate_r1cs_constraints(&self);
    //     pub fn generate_r1cs_witness(&self);
}
#[derive(Clone, Default)]
pub struct comparison_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    pub alpha: pb_variable_array<FieldT, PB>,
    pub alpha_packed: variable<FieldT, pb_variable>,
    pub pack_alpha: RcCell<gadget<FieldT, PB, packing_gadget<FieldT, PB>>>,
    pub all_zeros_test: RcCell<gadget<FieldT, PB, disjunction_gadget<FieldT, PB>>>,
    pub not_all_zeros: variable<FieldT, pb_variable>,
    pub n: usize,
    pub A: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub B: linear_combination<FieldT, pb_variable, pb_linear_combination>,
    pub less: variable<FieldT, pb_variable>,
    pub less_or_eq: variable<FieldT, pb_variable>,
}
impl<FieldT: FieldTConfig, PB: PBConfig> comparison_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        n: usize,
        A: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        B: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        less: variable<FieldT, pb_variable>,
        less_or_eq: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut alpha = pb_variable_array::<FieldT, PB>::default();
        alpha.allocate(&pb, n, &prefix_format!(annotation_prefix, " alpha"));
        alpha.contents.push(less_or_eq.clone()); // alpha[n] is less_or_eq
        let mut alpha_packed = variable::<FieldT, pb_variable>::default();
        alpha_packed.allocate(&pb, prefix_format!(annotation_prefix, " alpha_packed"));
        let mut not_all_zeros = variable::<FieldT, pb_variable>::default();
        not_all_zeros.allocate(&pb, prefix_format!(annotation_prefix, " not_all_zeros"));

        let pack_alpha = RcCell::new(packing_gadget::<FieldT, PB>::new(
            pb.clone(),
            alpha.clone().into(),
            alpha_packed.clone().into(),
            prefix_format!(annotation_prefix, " pack_alpha"),
        ));

        let all_zeros_test = RcCell::new(disjunction_gadget::<FieldT, PB>::new(
            pb.clone(),
            pb_variable_array::<FieldT, PB>::new(alpha.contents[..n].to_vec()),
            not_all_zeros.clone(),
            prefix_format!(annotation_prefix, " all_zeros_test"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                alpha,
                alpha_packed,
                pack_alpha,
                all_zeros_test,
                not_all_zeros,
                n,
                A,
                B,
                less,
                less_or_eq,
            },
        )
    }

    // pub fn generate_r1cs_constraints(&self);
    // pub fn generate_r1cs_witness(&self);
}
#[derive(Clone, Default)]
pub struct inner_product_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    /* S_i = \sum_{k=0}^{i+1} A[i] * B[i] */
    pub S: pb_variable_array<FieldT, PB>,
    pub A: pb_linear_combination_array<FieldT, PB>,
    pub B: pb_linear_combination_array<FieldT, PB>,
    pub result: variable<FieldT, pb_variable>,
}
impl<FieldT: FieldTConfig, PB: PBConfig> inner_product_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        A: pb_linear_combination_array<FieldT, PB>,
        B: pb_linear_combination_array<FieldT, PB>,
        result: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        //  gadget<FieldT>(&pb, annotation_prefix),A,B,result
        assert!(A.len() >= 1);
        assert!(A.len() == B.len());
        let mut S = pb_variable_array::<FieldT, PB>::default();
        S.allocate(&pb, A.len() - 1, &prefix_format!(annotation_prefix, " S"));
        gadget::<FieldT, PB, Self>::new(pb, annotation_prefix, Self { S, A, B, result })
    }

    // pub fn generate_r1cs_constraints(&self);
    // pub fn generate_r1cs_witness(&self);
}

#[derive(Clone, Default)]
pub struct loose_multiplexing_gadget<FieldT: FieldTConfig, PB: PBConfig> {
    //   this implements loose multiplexer:
    //   index not in bounds -> success_flag = 0
    //   index in bounds && success_flag = 1 -> result is correct
    //   however if index is in bounds we can also set success_flag to 0 (and then result will be forced to be 0)
    pub alpha: pb_variable_array<FieldT, PB>,
    pub compute_result: RcCell<gadget<FieldT, PB, inner_product_gadget<FieldT, PB>>>,
    pub arr: pb_linear_combination_array<FieldT, PB>,
    pub index: variable<FieldT, pb_variable>,
    pub result: variable<FieldT, pb_variable>,
    pub success_flag: variable<FieldT, pb_variable>,
}
impl<FieldT: FieldTConfig, PB: PBConfig> loose_multiplexing_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        arr: pb_linear_combination_array<FieldT, PB>,
        index: variable<FieldT, pb_variable>,
        result: variable<FieldT, pb_variable>,
        success_flag: variable<FieldT, pb_variable>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let mut alpha = pb_variable_array::<FieldT, PB>::default();
        alpha.allocate(&pb, arr.len(), &prefix_format!(annotation_prefix, " alpha"));
        let compute_result = RcCell::new(inner_product_gadget::<FieldT, PB>::new(
            pb.clone(),
            alpha.clone().into(),
            arr.clone().into(),
            result.clone(),
            prefix_format!(annotation_prefix, " compute_result"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                alpha,
                compute_result,
                arr,
                index,
                result,
                success_flag,
            },
        )
    }

    // pub fn generate_r1cs_constraints(&self);
    // pub fn generate_r1cs_witness(&self);
}

use ffec::common::profiling::print_time;
use ffec::common::utils;

//
pub fn generate_boolean_r1cs_constraint<FieldT: FieldTConfig, PB: PBConfig>(
    pb: &RcCell<protoboard<FieldT, PB>>,
    lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    annotation_prefix: String,
) {
    /* forces lc to take value 0 or 1 by adding constraint lc * (1-lc) = 0 */
    pb.borrow_mut().add_r1cs_constraint(
        r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
            lc.clone(),
            -lc.clone() + 1,
            0.into(),
        ),
        prefix_format!(annotation_prefix, " boolean_r1cs_constraint"),
    );
}

pub fn generate_r1cs_equals_const_constraint<FieldT: FieldTConfig, PB: PBConfig>(
    pb: &RcCell<protoboard<FieldT, PB>>,
    lc: &linear_combination<FieldT, pb_variable, pb_linear_combination>,
    c: &FieldT,
    annotation_prefix: String,
) {
    pb.borrow_mut().add_r1cs_constraint(
        r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
            1.into(),
            lc.clone(),
            c.clone().into(),
        ),
        prefix_format!(annotation_prefix, " constness_constraint"),
    );
}

impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, packing_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        /* adds constraint result = \sum  bits[i] * 2^i */
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                1.into(),
                pb_packing_sum::<FieldT, PB>(&self.t.bits),
                self.t.packed.clone(),
            ),
            prefix_format!(self.annotation_prefix, " packing_constraint"),
        );

        if enforce_bitness {
            for i in 0..self.t.bits.len() {
                generate_boolean_r1cs_constraint::<FieldT, PB>(
                    &self.pb,
                    &self.t.bits[i],
                    prefix_format!(self.annotation_prefix, " bitness_{}", i),
                );
            }
        }
    }

    pub fn generate_r1cs_witness_from_packed(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.t.packed.evaluate_pb(&self.pb);
        assert!(
            self.pb
                .borrow()
                .lc_val(&self.t.packed)
                .as_bigint::<{ FieldT::num_limbs as usize }>()
                .num_bits()
                <= self.t.bits.len()
        ); // `bits` is large enough to represent this packed value
        self.t
            .bits
            .fill_with_bits_of_field_element(&self.pb, &self.pb.borrow().lc_val(&self.t.packed));
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        self.t.bits.evaluate(&self.pb);
        *self.pb.borrow_mut().lc_val_ref(&self.t.packed) =
            self.t.bits.get_field_element_from_bits(&self.pb);
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> multipacking_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        bits: pb_linear_combination_array<FieldT, PB>,
        packed_vars: pb_linear_combination_array<FieldT, PB>,
        chunk_size: usize,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let num_chunks = (div_ceil(bits.len(), chunk_size).unwrap());
        let last_chunk_size = (bits.len() - (num_chunks - 1) * chunk_size);
        assert!(packed_vars.len() == num_chunks);
        let mut packers = vec![];
        for i in 0..num_chunks {
            packers.push(packing_gadget::<FieldT, PB>::new(
                pb.clone(),
                pb_linear_combination_array::<FieldT, PB>::new(
                    bits.contents[i * chunk_size..std::cmp::min((i + 1) * chunk_size, bits.len())]
                        .to_vec(),
                ),
                packed_vars[i].clone(),
                prefix_format!(annotation_prefix, " packers_{}", i),
            ));
        }
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                packers,
                bits,
                packed_vars,
                chunk_size,
                num_chunks,
                last_chunk_size,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, multipacking_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        for i in 0..self.t.num_chunks {
            self.t.packers[i].generate_r1cs_constraints(enforce_bitness);
        }
    }

    pub fn generate_r1cs_witness_from_packed(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        for i in 0..self.t.num_chunks {
            self.t.packers[i].generate_r1cs_witness_from_packed();
        }
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        for i in 0..self.t.num_chunks {
            self.t.packers[i].generate_r1cs_witness_from_bits();
        }
    }
}

pub fn multipacking_num_chunks<FieldT: FieldTConfig, PB: PBConfig>(num_bits: usize) -> usize {
    div_ceil(num_bits, FieldT::capacity()).unwrap()
}
impl<FieldT: FieldTConfig, PB: PBConfig> field_vector_copy_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        source: pb_variable_array<FieldT, PB>,
        target: pb_variable_array<FieldT, PB>,
        do_copy: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        // gadget<FieldT>(&pb, annotation_prefix),source,target,do_copy
        assert!(source.len() == target.len());
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                source,
                target,
                do_copy,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, field_vector_copy_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.source.len() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.do_copy.clone(),
                    linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.source[i].clone(),
                    ) - linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(
                        self.t.target[i].clone(),
                    ),
                    0.into(),
                ),
                prefix_format!(self.annotation_prefix, " copying_check_{}", i),
            );
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.do_copy.evaluate_pb(&self.pb);
        assert!(
            self.pb.borrow().lc_val(&self.t.do_copy) == FieldT::one()
                || self.pb.borrow().lc_val(&self.t.do_copy) == FieldT::zero()
        );
        if self.pb.borrow().lc_val(&self.t.do_copy) != FieldT::zero() {
            for i in 0..self.t.source.len() {
                *self.pb.borrow_mut().val_ref(&self.t.target[i]) =
                    self.pb.borrow().val(&self.t.source[i]);
            }
        }
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig> bit_vector_copy_gadget<FieldT, PB> {
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        source_bits: pb_variable_array<FieldT, PB>,
        target_bits: pb_variable_array<FieldT, PB>,
        do_copy: linear_combination<FieldT, pb_variable, pb_linear_combination>,
        chunk_size: usize,
        annotation_prefix: String,
    ) -> gadget<FieldT, PB, Self> {
        let num_chunks = (div_ceil(source_bits.len(), chunk_size).unwrap());
        assert!(source_bits.len() == target_bits.len());
        let mut packed_source = pb_variable_array::<FieldT, PB>::default();
        packed_source.allocate(
            &pb,
            num_chunks,
            &prefix_format!(annotation_prefix, " packed_source"),
        );
        let pack_source = RcCell::new(multipacking_gadget::<FieldT, PB>::new(
            pb.clone(),
            source_bits.clone().into(),
            packed_source.clone().into(),
            chunk_size,
            prefix_format!(annotation_prefix, " pack_source"),
        ));
        let mut packed_target = pb_variable_array::<FieldT, PB>::default();
        packed_target.allocate(
            &pb,
            num_chunks,
            &prefix_format!(annotation_prefix, " packed_target"),
        );
        let pack_target = RcCell::new(multipacking_gadget::<FieldT, PB>::new(
            pb.clone(),
            target_bits.clone().into(),
            packed_target.clone().into(),
            chunk_size,
            prefix_format!(annotation_prefix, " pack_target"),
        ));

        let copier = RcCell::new(field_vector_copy_gadget::<FieldT, PB>::new(
            pb.clone(),
            packed_source.clone(),
            packed_target.clone(),
            do_copy.clone(),
            prefix_format!(annotation_prefix, " copier"),
        ));
        gadget::<FieldT, PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                source_bits,
                target_bits,
                do_copy,
                packed_source,
                packed_target,
                pack_source,
                pack_target,
                copier,
                chunk_size,
                num_chunks,
            },
        )
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, bit_vector_copy_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(
        &self,
        enforce_source_bitness: bool,
        enforce_target_bitness: bool,
    ) {
        self.t
            .pack_source
            .borrow()
            .generate_r1cs_constraints(enforce_source_bitness);
        self.t
            .pack_target
            .borrow()
            .generate_r1cs_constraints(enforce_target_bitness);

        self.t.copier.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.do_copy.evaluate_pb(&self.pb);
        assert!(
            self.pb.borrow().lc_val(&self.t.do_copy) == FieldT::zero()
                || self.pb.borrow().lc_val(&self.t.do_copy) == FieldT::one()
        );
        if self.pb.borrow().lc_val(&self.t.do_copy) == FieldT::one() {
            for i in 0..self.t.source_bits.len() {
                *self.pb.borrow_mut().val_ref(&self.t.target_bits[i]) =
                    self.pb.borrow().val(&self.t.source_bits[i]);
            }
        }

        self.t
            .pack_source
            .borrow()
            .generate_r1cs_witness_from_bits();
        self.t
            .pack_target
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}

impl<FieldT: FieldTConfig, PB: PBConfig, T>
    gadget<FieldT, PB, dual_variable_gadget<FieldT, PB, T>>
{
    pub fn generate_r1cs_constraints(&self, enforce_bitness: bool) {
        self.t
            .consistency_check
            .borrow()
            .generate_r1cs_constraints(enforce_bitness);
    }

    pub fn generate_r1cs_witness_from_packed(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.t
            .consistency_check
            .borrow()
            .generate_r1cs_witness_from_packed();
    }

    pub fn generate_r1cs_witness_from_bits(&self) {
        self.t
            .consistency_check
            .borrow()
            .generate_r1cs_witness_from_bits();
    }
}
impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, disjunction_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        /* inv * sum = output */
        let (mut a1, mut b1, mut c1) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a1.add_term_with_variable(self.t.inv.clone());
        for i in 0..self.t.inputs.len() {
            b1.add_term_with_variable(self.t.inputs[i].clone());
        }
        c1.add_term_with_variable(self.t.output.clone());

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a1, b1, c1),
            prefix_format!(self.annotation_prefix, " inv*sum=output"),
        );

        /* (1-output) * sum = 0 */
        let (mut a2, mut b2, mut c2) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a2.add_term_with_index(ONE);
        a2.add_term(self.t.output.index, -1);
        for i in 0..self.t.inputs.len() {
            b2.add_term_with_variable(self.t.inputs[i].clone());
        }
        c2.add_term(ONE, 0);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a2, b2, c2),
            prefix_format!(self.annotation_prefix, " (1-output)*sum=0"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        let mut sum = FieldT::zero();

        for i in 0..self.t.inputs.len() {
            sum += self.pb.borrow().val(&self.t.inputs[i]);
        }

        if sum.is_zero() {
            *self.pb.borrow_mut().val_ref(&self.t.inv) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.output) = FieldT::zero();
        } else {
            *self.pb.borrow_mut().val_ref(&self.t.inv) = sum.inverse();
            *self.pb.borrow_mut().val_ref(&self.t.output) = FieldT::one();
        }
    }
}

pub fn test_disjunction_gadget<FieldT: FieldTConfig, PB: PBConfig>(n: usize) {
    print!("testing disjunction_gadget on all {} bit strings\n", n);

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut inputs = pb_variable_array::<FieldT, PB>::default();
    inputs.allocate(&pb, n, "inputs");

    let mut output = variable::<FieldT, pb_variable>::default();
    output.allocate(&pb, "output".to_owned());

    let mut d = disjunction_gadget::<FieldT, PB>::new(
        pb.clone(),
        inputs.clone(),
        output.clone(),
        "d".to_owned(),
    );
    d.generate_r1cs_constraints();

    for w in 0..1usize << n {
        for j in 0..n {
            *pb.borrow_mut().val_ref(&inputs[j]) =
                FieldT::from(if w & (1usize << j) != 0 { 1i64 } else { 0 });
        }

        d.generate_r1cs_witness();

        // #ifdef DEBUG
        print!("positive test for {}\n", w);
        //#endif
        assert!(
            pb.borrow().val(&output)
                == if w != 0 {
                    FieldT::one()
                } else {
                    FieldT::zero()
                }
        );
        assert!(pb.borrow().is_satisfied());

        // #ifdef DEBUG
        print!("negative test for {}\n", w);
        //#endif
        *pb.borrow_mut().val_ref(&output) = (if w != 0 {
            FieldT::zero()
        } else {
            FieldT::one()
        });
        assert!(!pb.borrow().is_satisfied());
    }

    print_time("disjunction tests successful");
}

impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, conjunction_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        /* inv * (n-sum) = 1-output */
        let (mut a1, mut b1, mut c1) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a1.add_term_with_variable(self.t.inv.clone());
        b1.add_term(ONE, self.t.inputs.len() as i64);
        for i in 0..self.t.inputs.len() {
            b1.add_term(self.t.inputs[i].index, -1);
        }
        c1.add_term_with_index(ONE);
        c1.add_term(self.t.output.index, -1);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a1, b1, c1),
            prefix_format!(self.annotation_prefix, " inv*(n-sum)=(1-output)"),
        );

        /* output * (n-sum) = 0 */
        let (mut a2, mut b2, mut c2) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );

        a2.add_term_with_variable(self.t.output.clone());
        b2.add_term(ONE, self.t.inputs.len() as i64);
        for i in 0..self.t.inputs.len() {
            b2.add_term(self.t.inputs[i].index, -1);
        }
        c2.add_term(ONE, 0);

        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a2, b2, c2),
            prefix_format!(self.annotation_prefix, " output*(n-sum)=0"),
        );
    }

    pub fn generate_r1cs_witness(&self) {
        let mut sum = FieldT::from(self.t.inputs.len());

        for i in 0..self.t.inputs.len() {
            sum -= self.pb.borrow().val(&self.t.inputs[i]);
        }

        if sum.is_zero() {
            *self.pb.borrow_mut().val_ref(&self.t.inv) = FieldT::zero();
            *self.pb.borrow_mut().val_ref(&self.t.output) = FieldT::one();
        } else {
            *self.pb.borrow_mut().val_ref(&self.t.inv) = sum.inverse();
            *self.pb.borrow_mut().val_ref(&self.t.output) = FieldT::zero();
        }
    }
}

pub fn test_conjunction_gadget<FieldT: FieldTConfig, PB: PBConfig>(n: usize) {
    print!("testing conjunction_gadget on all {} bit strings\n", n);

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut inputs = pb_variable_array::<FieldT, PB>::default();
    inputs.allocate(&pb, n, "inputs");

    let mut output = variable::<FieldT, pb_variable>::default();
    output.allocate(&pb, "output".to_owned());

    let mut c = conjunction_gadget::<FieldT, PB>::new(
        pb.clone(),
        inputs.clone(),
        output.clone(),
        "c".to_owned(),
    );
    c.generate_r1cs_constraints();

    for w in 0..1usize << n {
        for j in 0..n {
            *pb.borrow_mut().val_ref(&inputs[j]) = if w & (1usize << j) != 0 {
                FieldT::one()
            } else {
                FieldT::zero()
            };
        }

        c.generate_r1cs_witness();

        // #ifdef DEBUG
        print!("positive test for {}\n", w);
        //#endif
        assert!(
            pb.borrow().val(&output)
                == if w == (1usize << n) - 1 {
                    FieldT::one()
                } else {
                    FieldT::zero()
                }
        );
        assert!(pb.borrow().is_satisfied());

        // #ifdef DEBUG
        print!("negative test for {}\n", w);
        //#endif
        *pb.borrow_mut().val_ref(&output) = if w == (1usize << n) - 1 {
            FieldT::zero()
        } else {
            FieldT::one()
        };
        assert!(!pb.borrow().is_satisfied());
    }

    print_time("conjunction tests successful");
}

impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, comparison_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        /*
         packed(alpha) = 2^n + B - A

         not_all_zeros = \bigvee_{i=0}^{n-1} alpha_i

         if B - A > 0, then 2^n + B - A > 2^n,
             so alpha_n = 1 and not_all_zeros = 1
         if B - A = 0, then 2^n + B - A = 2^n,
             so alpha_n = 1 and not_all_zeros = 0
         if B - A < 0, then 2^n + B - A \in {0, 1, \ldots, 2^n-1},
             so alpha_n = 0

         therefore alpha_n = less_or_eq and alpha_n * not_all_zeros = less
        */

        /* not_all_zeros to be Boolean, alpha_i are Boolean by packing gadget */
        generate_boolean_r1cs_constraint::<FieldT, PB>(
            &self.pb,
            &self.t.not_all_zeros.clone().into(),
            prefix_format!(self.annotation_prefix, " not_all_zeros"),
        );

        /* constraints for packed(alpha) = 2^n + B - A */
        self.t.pack_alpha.borrow().generate_r1cs_constraints(true);
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                1.into(),
                self.t.B.clone() + (FieldT::from(2) ^ self.t.n) - self.t.A.clone(),
                self.t.alpha_packed.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " main_constraint"),
        );

        /* compute result */
        self.t.all_zeros_test.borrow().generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                self.t.less_or_eq.clone().into(),
                self.t.not_all_zeros.clone().into(),
                self.t.less.clone().into(),
            ),
            prefix_format!(self.annotation_prefix, " less"),
        );
    }

    pub fn generate_r1cs_witness(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        self.t.A.evaluate_pb(&self.pb);
        self.t.B.evaluate_pb(&self.pb);

        /* unpack 2^n + B - A into alpha_packed */
        *self.pb.borrow_mut().val_ref(&self.t.alpha_packed) = (FieldT::from(2) ^ self.t.n)
            + self.pb.borrow().lc_val(&self.t.B)
            - self.pb.borrow().lc_val(&self.t.A);
        self.t
            .pack_alpha
            .borrow()
            .generate_r1cs_witness_from_packed();

        /* compute result */
        self.t.all_zeros_test.borrow().generate_r1cs_witness();
        *self.pb.borrow_mut().val_ref(&self.t.less) =
            self.pb.borrow().val(&self.t.less_or_eq) * self.pb.borrow().val(&self.t.not_all_zeros);
    }
}

pub fn test_comparison_gadget<FieldT: FieldTConfig, PB: PBConfig>(n: usize)
where
    [(); { FieldT::num_limbs as usize }]:,
{
    print!("testing comparison_gadget on all {} bit inputs\n", n);

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let (mut A, mut B, mut less, mut less_or_eq) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );
    A.allocate(&pb, "A".to_owned());
    B.allocate(&pb, "B".to_owned());
    less.allocate(&pb, "less".to_owned());
    less_or_eq.allocate(&pb, "less_or_eq".to_owned());

    let mut cmp = comparison_gadget::<FieldT, PB>::new(
        pb.clone(),
        n,
        A.clone().into(),
        B.clone().into(),
        less.clone(),
        less_or_eq.clone(),
        "cmp".to_owned(),
    );
    cmp.generate_r1cs_constraints();

    for a in 0..1usize << n {
        for b in 0..1usize << n {
            *pb.borrow_mut().val_ref(&A) = FieldT::from(a);
            *pb.borrow_mut().val_ref(&B) = FieldT::from(b);

            cmp.generate_r1cs_witness();

            // #ifdef DEBUG
            print!("positive test for {} < {}\n", a, b);
            //#endif
            assert!(pb.borrow().val(&less) == if a < b { FieldT::one() } else { FieldT::zero() });
            assert!(
                *pb.borrow_mut().val_ref(&less_or_eq)
                    == if a <= b {
                        FieldT::one()
                    } else {
                        FieldT::zero()
                    }
            );
            assert!(pb.borrow().is_satisfied());
        }
    }

    print_time("comparison tests successful");
}

impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, inner_product_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        /*
          S_i = \sum_{k=0}^{i+1} A[i] * B[i]
          S[0] = A[0] * B[0]
          S[i+1] - S[i] = A[i] * B[i]
        */
        for i in 0..self.t.A.len() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT,pb_variable,pb_linear_combination>::new(
                    self.t.A[i].clone(),
                    self.t.B[i].clone(),
                    (if i == self.t.A.len() - 1 {
                        linear_combination::<FieldT, pb_variable, pb_linear_combination>::from(self.t.result.clone())
                    } else {
                         (if i == 0 { linear_combination::<FieldT,pb_variable,pb_linear_combination>::from(0)*linear_combination::<FieldT,pb_variable,pb_linear_combination>::from(FieldT::one())} else { -linear_combination::<FieldT,pb_variable,pb_linear_combination>::from(self.t.S[i - 1].clone()) })+self.t.S[i].clone()
                    }),
                ),
                prefix_format!(self.annotation_prefix, " S_{}", i),
            );
        }
    }

    pub fn generate_r1cs_witness(&self) {
        let mut total = FieldT::zero();
        for i in 0..self.t.A.len() {
            self.t.A[i].evaluate_pb(&self.pb);
            self.t.B[i].evaluate_pb(&self.pb);

            total += self.pb.borrow().lc_val(&self.t.A[i]) * self.pb.borrow().lc_val(&self.t.B[i]);
            *self.pb.borrow_mut().val_ref(
                &(if i == self.t.A.len() - 1 {
                    self.t.result.clone()
                } else {
                    self.t.S[i].clone()
                }),
            ) = total.clone();
        }
    }
}

pub fn test_inner_product_gadget<FieldT: FieldTConfig, PB: PBConfig>(n: usize) {
    print!("testing inner_product_gadget on all {} bit strings\n", n);

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut A = pb_variable_array::<FieldT, PB>::default();
    A.allocate(&pb, n, "A");
    let mut B = pb_variable_array::<FieldT, PB>::default();
    B.allocate(&pb, n, "B");

    let mut result = variable::<FieldT, pb_variable>::default();
    result.allocate(&pb, "result".to_owned());

    let mut g = inner_product_gadget::<FieldT, PB>::new(
        pb.clone(),
        A.clone().into(),
        B.clone().into(),
        result.clone(),
        "g".to_owned(),
    );
    g.generate_r1cs_constraints();

    for i in 0..1usize << n {
        for j in 0..1usize << n {
            let mut correct = 0;
            for k in 0..n {
                *pb.borrow_mut().val_ref(&A[k]) = if i & (1usize << k) != 0 {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
                *pb.borrow_mut().val_ref(&B[k]) = if j & (1usize << k) != 0 {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
                correct += if (i & (1usize << k) != 0) && (j & (1usize << k) != 0) {
                    1
                } else {
                    0
                };
            }

            g.generate_r1cs_witness();
            // #ifdef DEBUG
            print!("positive test for ({}, {})\n", i, j);
            //#endif
            assert!(pb.borrow().val(&result) == FieldT::from(correct as i64));
            assert!(pb.borrow().is_satisfied());

            // #ifdef DEBUG
            print!("negative test for ({}, {})\n", i, j);
            //#endif
            *pb.borrow_mut().val_ref(&result) = FieldT::from(100 * n + 19);
            assert!(!pb.borrow().is_satisfied());
        }
    }

    print_time("inner_product_gadget tests successful");
}

impl<FieldT: FieldTConfig, PB: PBConfig> gadget<FieldT, PB, loose_multiplexing_gadget<FieldT, PB>> {
    pub fn generate_r1cs_constraints(&self) {
        /* \alpha_i (index - i) = 0 */
        for i in 0..self.t.arr.len() {
            self.pb.borrow_mut().add_r1cs_constraint(
                r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(
                    self.t.alpha[i].clone().into(),
                    FieldT::from(self.t.index.index - i).into(),
                    0.into(),
                ),
                prefix_format!(self.annotation_prefix, " alpha_{}", i),
            );
        }

        /* 1 * (\sum \alpha_i) = success_flag */
        let (mut a, mut b, mut c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );
        a.add_term_with_index(ONE);
        for i in 0..self.t.arr.len() {
            b.add_term_with_variable(self.t.alpha[i].clone());
        }
        c.add_term_with_variable(self.t.success_flag.clone());
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a, b, c),
            prefix_format!(self.annotation_prefix, " main_constraint"),
        );

        /* now success_flag is constrained to either 0 (if index is out of
        range) or \alpha_i. constrain it and \alpha_i to zero */
        generate_boolean_r1cs_constraint::<FieldT, PB>(
            &self.pb,
            &self.t.success_flag.clone().into(),
            prefix_format!(self.annotation_prefix, " success_flag"),
        );

        /* compute result */
        self.t.compute_result.borrow().generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness(&self)
    where
        [(); { FieldT::num_limbs as usize }]:,
    {
        /* assumes that idx can be fit in ulong; true for our purposes for now */
        let mut valint = self
            .pb
            .borrow()
            .val(&self.t.index)
            .as_bigint::<{ FieldT::num_limbs as usize }>();
        let mut idx = valint.as_ulong() as usize;
        let arrsize = bigint::<{ FieldT::num_limbs as usize }>::new(self.t.arr.len() as u64);

        if idx >= self.t.arr.len()
            || &valint.0.0[..FieldT::num_limbs as usize]
                >= &arrsize.0.0[..FieldT::num_limbs as usize]
        {
            for i in 0..self.t.arr.len() {
                *self.pb.borrow_mut().val_ref(&self.t.alpha[i]) = FieldT::zero();
            }

            *self.pb.borrow_mut().val_ref(&self.t.success_flag) = FieldT::zero();
        } else {
            for i in 0..self.t.arr.len() {
                *self.pb.borrow_mut().val_ref(&self.t.alpha[i]) = if i == idx {
                    FieldT::one()
                } else {
                    FieldT::zero()
                };
            }

            *self.pb.borrow_mut().val_ref(&self.t.success_flag) = FieldT::one();
        }

        self.t.compute_result.borrow().generate_r1cs_witness();
    }
}

pub fn test_loose_multiplexing_gadget<FieldT: FieldTConfig, PB: PBConfig>(n: usize)
where
    [(); { FieldT::num_limbs as usize }]:,
{
    print!(
        "testing loose_multiplexing_gadget on 2**{} variable<FieldT,pb_variable> array inputs\n",
        n
    );
    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());

    let mut arr = pb_variable_array::<FieldT, PB>::default();
    arr.allocate(&pb, 1usize << n, "arr");
    let (mut index, mut result, mut success_flag) = (
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
        variable::<FieldT, pb_variable>::default(),
    );
    index.allocate(&pb, "index".to_owned());
    result.allocate(&pb, "result".to_owned());
    success_flag.allocate(&pb, "success_flag".to_owned());

    let mut g = loose_multiplexing_gadget::<FieldT, PB>::new(
        pb.clone(),
        arr.clone().into(),
        index.clone(),
        result.clone(),
        success_flag.clone(),
        "g".to_owned(),
    );
    g.generate_r1cs_constraints();

    for i in 0..1usize << n {
        *pb.borrow_mut().val_ref(&arr[i]) = FieldT::from((19 * i) % (1usize << n));
    }

    for idx in -1..=(1i64 << n) {
        *pb.borrow_mut().val_ref(&index) = FieldT::from(idx);
        g.generate_r1cs_witness();

        if 0 <= idx && idx <= (1i64 << n) - 1 {
            print!("demuxing element {} (in bounds)\n", idx);
            assert!(pb.borrow().val(&result) == FieldT::from((19 * idx) % (1i64 << n)));
            assert!(pb.borrow().val(&success_flag) == FieldT::one());
            assert!(pb.borrow().is_satisfied());
            *pb.borrow_mut().val_ref(&result) -= FieldT::one();
            assert!(!pb.borrow().is_satisfied());
        } else {
            print!("demuxing element {} (out of bounds)\n", idx);
            assert!(pb.borrow().val(&success_flag) == FieldT::zero());
            assert!(pb.borrow().is_satisfied());
            *pb.borrow_mut().val_ref(&success_flag) = FieldT::one();
            assert!(!pb.borrow().is_satisfied());
        }
    }
    print!("loose_multiplexing_gadget tests successful\n");
}

pub fn create_linear_combination_constraints<
    FieldT: FieldTConfig,
    PB: PBConfig,
    VarT: VarTConfig<FieldT, PB>,
>(
    pb: &RcCell<protoboard<FieldT, PB>>,
    base: &Vec<FieldT>,
    v: &Vec<(VarT, FieldT)>,
    target: &VarT,
    annotation_prefix: String,
) {
    for i in 0..base.len() {
        let (mut a, mut b, mut c) = (
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
            linear_combination::<FieldT, pb_variable, pb_linear_combination>::default(),
        );

        a.add_term_with_index(ONE);
        b.add_term_with_field(ONE, base[i].clone());

        for p in v {
            b.add_term_with_field(p.0.all_vars()[i].index, p.1.clone());
        }

        c.add_term_with_variable(target.all_vars()[i].clone());

        pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT, pb_variable, pb_linear_combination>::new(a, b, c),
            prefix_format!(annotation_prefix, " linear_combination_{}", i),
        );
    }
}

pub fn create_linear_combination_witness<
    FieldT: FieldTConfig,
    PB: PBConfig,
    VarT: VarTConfig<FieldT, PB>,
>(
    pb: &RcCell<protoboard<FieldT, PB>>,
    base: &Vec<FieldT>,
    v: &Vec<(VarT, FieldT)>,
    target: &VarT,
) {
    for i in 0..base.len() {
        *pb.borrow_mut().val_ref(&target.all_vars()[i]) = base[i].clone();

        for p in v {
            *pb.borrow_mut().val_ref(&target.all_vars()[i]) +=
                p.1.clone() * pb.borrow().val(&p.0.all_vars()[i]);
        }
    }
}
pub trait VarTConfig<FieldT: FieldTConfig, PB: PBConfig> {
    fn all_vars(&self) -> &pb_variable_array<FieldT, PB>;
}
