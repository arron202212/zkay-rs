// Declaration of interfaces for the exponentiation gadget.
use crate::gadgetlib1::gadget::gadget;
use crate::gadgetlib1::gadgets::pairing::pairing_params::{
    Fpk_mul_gadgetT, Fpk_sqr_gadgetT, Fpk_variableT, M, MulTConfig, SqrTConfig, VariableTConfig,
    pairing_selector, ppTConfig,
};
use crate::gadgetlib1::protoboard::{PBConfig, ProtoboardConfig, protoboard};
use crate::prefix_format;
use ffec::algebra::field_utils::bigint::bigint;
use ffec::scalar_multiplication::wnaf::find_wnaf;
use ffec::{FieldTConfig, One, PpConfig};
use rccell::RcCell;
use std::marker::PhantomData;

/**
 * The exponentiation gadget verifies field exponentiation in the field F_{p^k}.
 *
 * Note that the power is a constant (i.e., hardcoded into the gadget).
 */

// pub type Fpk_variableT<FpkT,P> = <FpkT as ppTConfig>::Fpk_variableT;
// pub type Fpk_mul_gadgetT<FpkT,P> = <FpkT as ppTConfig>::Fpk_mul_gadgetT;
// pub type Fpk_sqr_gadgetT<FpkT,P> = <FpkT as ppTConfig>::Fpk_sqr_gadgetT;
type FieldT<FpkT> = <FpkT as ppTConfig>::my_Fp;
// pub type Fqk_variable<FpkT> = <FpkT as ppTConfig>::Fpk_variableT;
// pub type Fqk_mul_gadget<FpkT> = <FpkT as ppTConfig>::Fpk_mul_gadgetT;
// pub type Fqk_sqr_gadget<FpkT> = <FpkT as ppTConfig>::Fpk_sqr_gadgetT;

#[derive(Clone, Default)]
pub struct exponentiation_gadget<FpkT: ppTConfig> {
    // : gadget<FpkT::my_Fp>
    // type FieldT=FpkT::my_Fp;
    pub NAF: Vec<i64>,

    pub intermediate: Vec<RcCell<Fpk_variableT<FpkT>>>,
    pub addition_steps: Vec<RcCell<Fpk_mul_gadgetT<FpkT>>>,
    pub subtraction_steps: Vec<RcCell<Fpk_mul_gadgetT<FpkT>>>,
    pub doubling_steps: Vec<RcCell<Fpk_sqr_gadgetT<FpkT>>>,

    pub elt: Fpk_variableT<FpkT>,
    pub power: bigint<M>,
    pub result: Fpk_variableT<FpkT>,

    pub intermed_count: usize,
    pub add_count: usize,
    pub sub_count: usize,
    pub dbl_count: usize,
}

pub type exponentiation_gadgets<FpkT> =
    gadget<<FpkT as ppTConfig>::FieldT, <FpkT as ppTConfig>::PB, exponentiation_gadget<FpkT>>;
impl<FpkT: ppTConfig> exponentiation_gadget<FpkT> {
    pub fn new(
        pb: RcCell<protoboard<FpkT::FieldT, FpkT::PB>>,
        elt: Fpk_variableT<FpkT>,
        power: bigint<M>,
        result: Fpk_variableT<FpkT>,
        annotation_prefix: String,
    ) -> exponentiation_gadgets<FpkT> {
        let NAF = find_wnaf(1, &power.0);

        let mut intermed_count = 0;
        let mut add_count = 0;
        let mut sub_count = 0;
        let mut dbl_count = 0;

        let mut found_nonzero = false;
        for i in (0..=NAF.len() - 1).rev() {
            if found_nonzero {
                dbl_count += 1;
                intermed_count += 1;
            }

            if NAF[i] != 0 {
                found_nonzero = true;

                if NAF[i] > 0 {
                    add_count += 1;
                    intermed_count += 1;
                } else {
                    sub_count += 1;
                    intermed_count += 1;
                }
            }
        }

        let mut intermediate = vec![RcCell::new(Fpk_variableT::<FpkT>::default()); intermed_count];
        intermediate[0] = RcCell::new(Fpk_variableT::<FpkT>::new2(
            pb.clone(),
            FpkT::FieldT::one(),
            prefix_format!(annotation_prefix, " intermediate_0"),
        ));
        for i in 1..intermed_count {
            intermediate[i] = RcCell::new(Fpk_variableT::<FpkT>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " intermediate_{}", i),
            ));
        }
        let mut addition_steps = vec![RcCell::new(Fpk_mul_gadgetT::<FpkT>::default()); add_count];
        let mut subtraction_steps =
            vec![RcCell::new(Fpk_mul_gadgetT::<FpkT>::default()); sub_count];
        let mut doubling_steps = vec![RcCell::new(Fpk_sqr_gadgetT::<FpkT>::default()); dbl_count];

        let mut found_nonzero = false;

        let (mut dbl_id, mut add_id, mut sub_id, mut intermed_id) = (0, 0, 0, 0);

        for i in (0..=NAF.len() - 1).rev() {
            if found_nonzero {
                doubling_steps[dbl_id] = RcCell::new(Fpk_sqr_gadgetT::<FpkT>::new(
                    pb.clone(),
                    intermediate[intermed_id].clone(),
                    if intermed_id + 1 == intermed_count {
                        result.clone()
                    } else {
                        intermediate[intermed_id + 1].borrow().clone()
                    },
                    prefix_format!(annotation_prefix, " doubling_steps_{}", dbl_count),
                ));
                intermed_id += 1;
                dbl_id += 1;
            }

            if NAF[i] != 0 {
                found_nonzero = true;

                if NAF[i] > 0 {
                    /* next = cur * elt */
                    addition_steps[add_id] = RcCell::new(Fpk_mul_gadgetT::<FpkT>::new(
                        pb.clone(),
                        intermediate[intermed_id].borrow().clone(),
                        elt.clone(),
                        if intermed_id + 1 == intermed_count {
                            result.clone()
                        } else {
                            intermediate[intermed_id + 1].borrow().clone()
                        },
                        prefix_format!(annotation_prefix, " addition_steps_{}", dbl_count),
                    ));
                    add_id += 1;
                    intermed_id += 1;
                } else {
                    /* next = cur / elt, i.e. next * elt = cur */
                    subtraction_steps[sub_id] = RcCell::new(Fpk_mul_gadgetT::<FpkT>::new(
                        pb.clone(),
                        if intermed_id + 1 == intermed_count {
                            result.clone()
                        } else {
                            intermediate[intermed_id + 1].borrow().clone()
                        },
                        elt.clone(),
                        intermediate[intermed_id].borrow().clone(),
                        prefix_format!(annotation_prefix, " subtraction_steps_{}", dbl_count),
                    ));
                    sub_id += 1;
                    intermed_id += 1;
                }
            }
        }
        gadget::<FpkT::FieldT, FpkT::PB, Self>::new(
            pb,
            annotation_prefix,
            Self {
                elt,
                power,
                result,
                NAF,
                intermediate,
                addition_steps,
                subtraction_steps,
                doubling_steps,
                intermed_count,
                add_count,
                sub_count,
                dbl_count,
            },
        )
    }
}
impl<FpkT: ppTConfig> exponentiation_gadgets<FpkT> {
    pub fn generate_r1cs_constraints(&self) {
        for i in 0..self.t.add_count {
            self.t.addition_steps[i]
                .borrow()
                .generate_r1cs_constraints();
        }

        for i in 0..self.t.sub_count {
            self.t.subtraction_steps[i]
                .borrow()
                .generate_r1cs_constraints();
        }

        for i in 0..self.t.dbl_count {
            self.t.doubling_steps[i]
                .borrow()
                .generate_r1cs_constraints();
        }
    }

    pub fn generate_r1cs_witness(&self) {
        self.t.intermediate[0]
            .borrow()
            .generate_r1cs_witness(&FpkT::FieldT::one());

        let mut found_nonzero = false;
        let mut dbl_id = 0;
        let mut add_id = 0;
        let mut sub_id = 0;
        let mut intermed_id = 0;

        for i in (0..=self.t.NAF.len() - 1).rev() {
            if found_nonzero {
                self.t.doubling_steps[dbl_id]
                    .borrow()
                    .generate_r1cs_witness();
                intermed_id += 1;
                dbl_id += 1;
            }

            if self.t.NAF[i] != 0 {
                found_nonzero = true;

                if self.t.NAF[i] > 0 {
                    self.t.addition_steps[add_id]
                        .borrow()
                        .generate_r1cs_witness();
                    intermed_id += 1;
                    add_id += 1;
                } else {
                    let cur_val = self.t.intermediate[intermed_id].borrow().get_element();
                    let elt_val = self.t.elt.get_element();
                    let next_val = cur_val * elt_val.inverse();

                    (if intermed_id + 1 == self.t.intermed_count {
                        self.t.result.clone()
                    } else {
                        self.t.intermediate[intermed_id + 1].borrow().clone()
                    })
                    .generate_r1cs_witness(&next_val.to_field::<FpkT::FieldT>());

                    self.t.subtraction_steps[sub_id]
                        .borrow()
                        .generate_r1cs_witness();

                    intermed_id += 1;
                    sub_id += 1;
                }
            }
        }
    }
}

pub fn test_exponentiation_gadget<FpkT: ppTConfig>(power: &bigint<M>, annotation: &String) {
    // type FieldT = FpkT::my_Fp;

    let mut pb = RcCell::new(protoboard::<FpkT::FieldT, FpkT::PB>::default());
    let mut x = Fpk_variableT::<FpkT>::new(pb.clone(), "x".to_owned());
    let mut x_to_power = Fpk_variableT::<FpkT>::new(pb.clone(), "x_to_power".to_owned());
    let mut exp_gadget = exponentiation_gadget::<FpkT>::new(
        pb.clone(),
        x.clone(),
        power.clone(),
        x_to_power.clone(),
        "exp_gadget".to_owned(),
    );
    exp_gadget.generate_r1cs_constraints();

    for i in 0..10 {
        let x_val = FpkT::FieldT::random_element();
        x.generate_r1cs_witness(&x_val.to_field::<FpkT::FieldT>());
        exp_gadget.generate_r1cs_witness();
        let res = x_to_power.get_element();
        assert!(pb.borrow().is_satisfied());
        assert!(res == (x_val ^ power.as_ulong() as usize));
    }
    print!(
        "number of constraints for {}_exp = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
    print!("exponent was: ");
    power.print();
}
