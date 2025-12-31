// Declaration of interfaces for the exponentiation gadget.
use crate::gadgetlib1::gadget::gadget;

use crate::gadgetlib1::gadgets::curves::{M, MulTConfig, SqrTConfig, VariableTConfig, ppTConfig};
use crate::gadgetlib1::protoboard::PBConfig;
use crate::gadgetlib1::protoboard::protoboard;
use crate::prefix_format;
use ffec::FieldTConfig;
use ffec::One;
use ffec::algebra::field_utils::bigint::bigint;
use ffec::scalar_multiplication::wnaf::find_wnaf;
use rccell::RcCell;
use std::marker::PhantomData;

/**
 * The exponentiation gadget verifies field exponentiation in the field F_{p^k}.
 *
 * Note that the power is a constant (i.e., hardcoded into the gadget).
 */
//
// pub trait ppTConfig<FieldT: FieldTConfig, PB: PBConfig>:
//     One
//     + Default
//     + Clone
//     + std::cmp::PartialEq
//     + for<'a> std::ops::BitXor<&'a ffec::field_utils::bigint::bigint<M>, Output = Self>
// {
//     type Fpk_variableT: VariableTConfig<FieldT, PB, Self>;
//     type Fpk_mul_gadgetT: MulTConfig<FieldT, PB, Self::Fpk_variableT>;
//     type Fpk_sqr_gadgetT: SqrTConfig<FieldT, PB, Self::Fpk_variableT>;
//     const M: usize;
//     fn inverse(&self) -> Self;
//     fn random_element() -> Self;
//     fn Frobenius_map(&self, power: usize) -> Self;
//     fn to_field(&self) -> FieldT;
//     fn c0(&self) -> FieldT;
//     fn c1(&self) -> FieldT;
//     fn c2(&self) -> FieldT;
// }

pub type Fpk_variableT<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_variableT;
pub type Fpk_mul_gadgetT<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_mul_gadgetT;
pub type Fpk_sqr_gadgetT<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_sqr_gadgetT;

pub type Fqk_variable<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_variableT;
pub type Fqk_mul_gadget<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_mul_gadgetT;
pub type Fqk_sqr_gadget<FpkT, FieldT, PB> = <FpkT as ppTConfig<FieldT, PB>>::Fpk_sqr_gadgetT;

// const M: usize = 4; //<FieldT, PB,FpkT>:usize= <FpkT as ppTConfig<FieldT, PB>>::M;
// pub trait VariableTConfig<FieldT: FieldTConfig, PB: PBConfig, FpkT>: Default + Clone {
//     fn get_element(&self) -> FpkT;
//     fn new(pb: RcCell<protoboard<FieldT, PB>>, annotation_prefix: String) -> Self;
//     fn new2(pb: RcCell<protoboard<FieldT, PB>>, f: FpkT, annotation_prefix: String) -> Self;
//     fn generate_r1cs_constraints(&self);
//     fn generate_r1cs_witness(&self, f: &FpkT);
// }
// pub trait MulTConfig<FieldT: FieldTConfig, PB: PBConfig, Fpk_variableT>: Default + Clone {
//     fn new(
//         pb: RcCell<protoboard<FieldT, PB>>,
//         v: Fpk_variableT<FieldT,PB,FpkT>,
//         v2: Fpk_variableT<FieldT,PB,FpkT>,
//         v3: Fpk_variableT<FieldT,PB,FpkT>,
//         annotation_prefix: String,
//     ) -> Self;
//     fn generate_r1cs_constraints(&self);
//     fn generate_r1cs_witness(&self);
// }
// pub trait SqrTConfig<FieldT: FieldTConfig, PB: PBConfig, Fpk_variableT>: Default + Clone {
//     fn new(
//         pb: RcCell<protoboard<FieldT, PB>>,
//         s: RcCell<Fpk_variableT<FieldT,PB,FpkT>>,
//         s2: Fpk_variableT<FieldT,PB,FpkT>,
//         annotation_prefix: String,
//     ) -> Self;
//     fn generate_r1cs_constraints(&self);
//     fn generate_r1cs_witness(&self);
// }

#[derive(Clone, Default)]
pub struct exponentiation_gadget<FpkT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig> {
    // : gadget<FpkT::my_Fp>
    // type FieldT=FpkT::my_Fp;
    NAF: Vec<i64>,

    intermediate: Vec<RcCell<Fpk_variableT<FpkT, FieldT, PB>>>,
    addition_steps: Vec<RcCell<Fpk_mul_gadgetT<FpkT, FieldT, PB>>>,
    subtraction_steps: Vec<RcCell<Fpk_mul_gadgetT<FpkT, FieldT, PB>>>,
    doubling_steps: Vec<RcCell<Fpk_sqr_gadgetT<FpkT, FieldT, PB>>>,

    elt: Fpk_variableT<FpkT, FieldT, PB>,
    power: bigint<M>,
    result: Fpk_variableT<FpkT, FieldT, PB>,

    intermed_count: usize,
    add_count: usize,
    sub_count: usize,
    dbl_count: usize,
    _t: PhantomData<(FpkT, FieldT, PB)>,
}

pub type exponentiation_gadgets<FpkT, FieldT, PB> =
    gadget<FieldT, PB, exponentiation_gadget<FpkT, FieldT, PB>>;
impl<FpkT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    exponentiation_gadget<FpkT, FieldT, PB>
{
    pub fn new(
        pb: RcCell<protoboard<FieldT, PB>>,
        elt: Fpk_variableT<FpkT, FieldT, PB>,
        power: bigint<M>,
        result: Fpk_variableT<FpkT, FieldT, PB>,
        annotation_prefix: String,
    ) -> exponentiation_gadgets<FpkT, FieldT, PB> {
        let NAF = find_wnaf(1, &power);

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

        let mut intermediate =
            vec![RcCell::new(Fpk_variableT::<FpkT, FieldT, PB>::default()); intermed_count];
        intermediate[0] = RcCell::new(Fpk_variableT::<FpkT, FieldT, PB>::new2(
            pb.clone(),
            FieldT::one(),
            prefix_format!(annotation_prefix, " intermediate_0"),
        ));
        for i in 1..intermed_count {
            intermediate[i] = RcCell::new(Fpk_variableT::<FpkT, FieldT, PB>::new(
                pb.clone(),
                prefix_format!(annotation_prefix, " intermediate_{}", i),
            ));
        }
        let mut addition_steps =
            vec![RcCell::new(Fpk_mul_gadgetT::<FpkT, FieldT, PB>::default()); add_count];
        let mut subtraction_steps =
            vec![RcCell::new(Fpk_mul_gadgetT::<FpkT, FieldT, PB>::default()); sub_count];
        let mut doubling_steps =
            vec![RcCell::new(Fpk_sqr_gadgetT::<FpkT, FieldT, PB>::default()); dbl_count];

        let mut found_nonzero = false;

        let (mut dbl_id, mut add_id, mut sub_id, mut intermed_id) = (0, 0, 0, 0);

        for i in (0..=NAF.len() - 1).rev() {
            if found_nonzero {
                doubling_steps[dbl_id] = RcCell::new(Fpk_sqr_gadgetT::<FpkT, FieldT, PB>::new(
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
                    addition_steps[add_id] = RcCell::new(Fpk_mul_gadgetT::<FpkT, FieldT, PB>::new(
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
                    subtraction_steps[sub_id] =
                        RcCell::new(Fpk_mul_gadgetT::<FpkT, FieldT, PB>::new(
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
        gadget::<FieldT, PB, Self>::new(
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
                _t: PhantomData,
            },
        )
    }
}
impl<FpkT: ppTConfig<FieldT, PB>, FieldT: FieldTConfig, PB: PBConfig>
    exponentiation_gadgets<FpkT, FieldT, PB>
{
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
            .generate_r1cs_witness(&FieldT::one());

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
                    .generate_r1cs_witness(&next_val.to_field());

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

pub fn test_exponentiation_gadget<
    FieldT: FieldTConfig,
    PB: PBConfig,
    FpkT: ppTConfig<FieldT, PB>,
>(
    power: &bigint<M>,
    annotation: &String,
) {
    // type FieldT = FpkT::my_Fp;

    let mut pb = RcCell::new(protoboard::<FieldT, PB>::default());
    let mut x = Fpk_variableT::<FpkT, FieldT, PB>::new(pb.clone(), "x".to_owned());
    let mut x_to_power =
        Fpk_variableT::<FpkT, FieldT, PB>::new(pb.clone(), "x_to_power".to_owned());
    let mut exp_gadget = exponentiation_gadget::<FpkT, FieldT, PB>::new(
        pb.clone(),
        x.clone(),
        power.clone(),
        x_to_power.clone(),
        "exp_gadget".to_owned(),
    );
    exp_gadget.generate_r1cs_constraints();

    for i in 0..10 {
        let x_val = FpkT::random_element();
        x.generate_r1cs_witness(&x_val.to_field());
        exp_gadget.generate_r1cs_witness();
        let res = x_to_power.get_element();
        assert!(pb.borrow().is_satisfied());
        assert!(res == (x_val ^ power));
    }
    print!(
        "number of constraints for {}_exp = {}\n",
        annotation,
        pb.borrow().num_constraints()
    );
    print!("exponent was: ");
    power.print();
}
