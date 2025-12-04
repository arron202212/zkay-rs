/** @file
*****************************************************************************

Declaration of interfaces for Fp6 gadgets.

The gadgets verify field arithmetic in Fp6 = Fp3[Y]/(Y^2-X) where
Fp3 = Fp[X]/(X^3-non_residue) and non_residue is in Fp.

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef FP6_GADGETS_HPP_
// #define FP6_GADGETS_HPP_
use crate::gadgetlib1::gadget;
use crate::gadgetlib1::gadgets::fields::fp2_gadgets;
use crate::gadgetlib1::gadgets::fields::fp3_gadgets;

/**
 * Gadget that represents an Fp6 variable.
 */

pub struct Fp6_variable {
    // : public gadget<Fp6T::my_Fp>
    // type FieldT=Fp6T::my_Fp;
    // type Fp3T=Fp6T::my_Fpe;
    c0: Fp3_variable<Fp3T>,
    c1: Fp3_variable<Fp3T>,
}

/**
 * Gadget that creates constraints for Fp6 multiplication.
 */

pub struct Fp6_mul_gadget {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp3T=Fp6T::my_Fpe;
    A: Fp6_variable<Fp6T>,
    B: Fp6_variable<Fp6T>,
    result: Fp6_variable<Fp6T>,

    v0_c0: pb_linear_combination<FieldT>,
    v0_c1: pb_linear_combination<FieldT>,
    v0_c2: pb_linear_combination<FieldT>,

    Ac0_plus_Ac1_c0: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1_c1: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1_c2: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1: RcCell<Fp3_variable<Fp3T>>,

    v0: RcCell<Fp3_variable<Fp3T>>,
    v1: RcCell<Fp3_variable<Fp3T>>,

    Bc0_plus_Bc1_c0: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1_c1: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1_c2: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1: RcCell<Fp3_variable<Fp3T>>,

    result_c1_plus_v0_plus_v1_c0: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1_c1: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1_c2: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1: RcCell<Fp3_variable<Fp3T>>,

    compute_v0: RcCell<Fp3_mul_gadget<Fp3T>>,
    compute_v1: RcCell<Fp3_mul_gadget<Fp3T>>,
    compute_result_c1: RcCell<Fp3_mul_gadget<Fp3T>>,
}

/**
 * Gadget that creates constraints for Fp6 multiplication by a Fp6 element B for which B.c0.c0 = B.c0.c1 = 0.
 */

pub struct Fp6_mul_by_2345_gadget {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp3T=Fp6T::my_Fpe;
    A: Fp6_variable<Fp6T>,
    B: Fp6_variable<Fp6T>,
    result: Fp6_variable<Fp6T>,

    v0_c0: pb_linear_combination<FieldT>,
    v0_c1: pb_linear_combination<FieldT>,
    v0_c2: pb_linear_combination<FieldT>,

    Ac0_plus_Ac1_c0: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1_c1: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1_c2: pb_linear_combination<FieldT>,
    Ac0_plus_Ac1: RcCell<Fp3_variable<Fp3T>>,

    v0: RcCell<Fp3_variable<Fp3T>>,
    v1: RcCell<Fp3_variable<Fp3T>>,

    Bc0_plus_Bc1_c0: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1_c1: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1_c2: pb_linear_combination<FieldT>,
    Bc0_plus_Bc1: RcCell<Fp3_variable<Fp3T>>,

    result_c1_plus_v0_plus_v1_c0: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1_c1: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1_c2: pb_linear_combination<FieldT>,
    result_c1_plus_v0_plus_v1: RcCell<Fp3_variable<Fp3T>>,

    compute_v1: RcCell<Fp3_mul_gadget<Fp3T>>,
    compute_result_c1: RcCell<Fp3_mul_gadget<Fp3T>>,
}

/**
 * Gadget that creates constraints for Fp6 squaring.
 */

pub struct Fp6_sqr_gadget {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    A: Fp6_variable<Fp6T>,
    result: Fp6_variable<Fp6T>,

    mul: RcCell<Fp6_mul_gadget<Fp6T>>,
}

/**
 * Gadget that creates constraints for Fp6 cyclotomic squaring
 */

pub struct Fp6_cyclotomic_sqr_gadget {
    // : public gadget<Fp6T::my_Fp>
    //     type FieldT=Fp6T::my_Fp;
    //     type Fp2T=Fp6T::my_Fp2;
    A: Fp6_variable<Fp6T>,
    result: Fp6_variable<Fp6T>,

    a: RcCell<Fp2_variable<Fp2T>>,
    b: RcCell<Fp2_variable<Fp2T>>,
    c: RcCell<Fp2_variable<Fp2T>>,

    asq_c0: pb_linear_combination<FieldT>,
    asq_c1: pb_linear_combination<FieldT>,

    bsq_c0: pb_linear_combination<FieldT>,
    bsq_c1: pb_linear_combination<FieldT>,

    csq_c0: pb_linear_combination<FieldT>,
    csq_c1: pb_linear_combination<FieldT>,

    asq: RcCell<Fp2_variable<Fp2T>>,
    bsq: RcCell<Fp2_variable<Fp2T>>,
    csq: RcCell<Fp2_variable<Fp2T>>,

    compute_asq: RcCell<Fp2_sqr_gadget<Fp2T>>,
    compute_bsq: RcCell<Fp2_sqr_gadget<Fp2T>>,
    compute_csq: RcCell<Fp2_sqr_gadget<Fp2T>>,
}

// use crate::gadgetlib1::gadgets::fields::fp6_gadgets;

//#endif // FP6_GADGETS_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for Fp6 gadgets.

See fp6_gadgets.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/

//#ifndef FP6_GADGETS_TCC_
// #define FP6_GADGETS_TCC_

impl Fp6_variable<Fp6T> {
    pub fn new(pb: &RcCell<protoboard<FieldT>>, annotation_prefix: &String) -> Self {
        Self {
            // gadget<FieldT>(&pb, annotation_prefix),
            c0: Fp3_variable::<Fp3T>::new(pb, FMT(annotation_prefix, " c0")),
            c1: Fp3_variable::<Fp3T>::new(pb, FMT(annotation_prefix, " c1")),
        }
    }

    pub fn new2(pb: &RcCell<protoboard<FieldT>>, el: &Fp6T, annotation_prefix: &String) -> Self {
        //  gadget<FieldT>(&pb, annotation_prefix),
        Self {
            c0: Fp3_variable::<Fp3T>::new(pb, el.c0, FMT(annotation_prefix, " c0")),
            c1: Fp3_variable::<Fp3T>::new(pb, el.c1, FMT(annotation_prefix, " c1")),
        }
    }

    pub fn new3(
        pb: &RcCell<protoboard<FieldT>>,
        c0: &Fp3_variable<Fp3T>,
        c1: &Fp3_variable<Fp3T>,
        annotation_prefix: &String,
    ) -> Self {
        // gadget<FieldT>(&pb, annotation_prefix),
        Self { c0, c1 }
    }

    pub fn generate_r1cs_equals_const_constraints(el: &Fp6T) {
        c0.generate_r1cs_equals_const_constraints(el.c0);
        c1.generate_r1cs_equals_const_constraints(el.c1);
    }

    pub fn generate_r1cs_witness(el: &Fp6T) {
        c0.generate_r1cs_witness(el.c0);
        c1.generate_r1cs_witness(el.c1);
    }

    pub fn get_element() -> Fp6T {
        let mut el = Fp6T::new();
        el.c0 = c0.get_element();
        el.c1 = c1.get_element();
        return el;
    }

    pub fn Frobenius_map(power: usize) -> Fp6_variable<Fp6T> {
        let (new_c0c0, new_c0c1, new_c0c2, new_c1c0, new_c1c1, new_c1c2) = (
            pb_linear_combination::<FieldT>::new(),
            pb_linear_combination::<FieldT>::new(),
            pb_linear_combination::<FieldT>::new(),
            pb_linear_combination::<FieldT>::new(),
            pb_linear_combination::<FieldT>::new(),
            pb_linear_combination::<FieldT>::new(),
        );
        new_c0c0.assign(self.pb, c0.c0);
        new_c0c1.assign(self.pb, c0.c1 * Fp3T::Frobenius_coeffs_c1[power % 3]);
        new_c0c2.assign(self.pb, c0.c2 * Fp3T::Frobenius_coeffs_c2[power % 3]);
        new_c1c0.assign(self.pb, c1.c0 * Fp6T::Frobenius_coeffs_c1[power % 6]);
        new_c1c1.assign(
            self.pb,
            c1.c1 * (Fp6T::Frobenius_coeffs_c1[power % 6] * Fp3T::Frobenius_coeffs_c1[power % 3]),
        );
        new_c1c2.assign(
            self.pb,
            c1.c2 * (Fp6T::Frobenius_coeffs_c1[power % 6] * Fp3T::Frobenius_coeffs_c2[power % 3]),
        );

        return Fp6_variable::<Fp6T>(
            self.pb,
            Fp3_variable::<Fp3T>(
                self.pb,
                new_c0c0,
                new_c0c1,
                new_c0c2,
                FMT(self.annotation_prefix, " Frobenius_map_c0"),
            ),
            Fp3_variable::<Fp3T>(
                self.pb,
                new_c1c0,
                new_c1c1,
                new_c1c2,
                FMT(self.annotation_prefix, " Frobenius_map_c1"),
            ),
            FMT(self.annotation_prefix, " Frobenius_map"),
        );
    }

    pub fn evaluate() {
        c0.evaluate();
        c1.evaluate();
    }
}
impl Fp6_mul_gadget<Fp6T> {
    pub fn new(
        pb: &RcCell<protoboard<FieldT>>,
        A: &Fp6_variable<Fp6T>,
        B: &Fp6_variable<Fp6T>,
        result: &Fp6_variable<Fp6T>,
        annotation_prefix: &String,
    ) -> Self {
        /*
            Karatsuba multiplication for Fp6 as a quadratic extension of Fp3:
                v0 = A.c0 * B.c0
                v1 = A.c1 * B.c1
                result.c0 = v0 + non_residue * v1
                result.c1 = (A.c0 + A.c1) * (B.c0 + B.c1) - v0 - v1
            where "non_residue * elem" := (non_residue * elem.c2, elem.c0, elem.c1)

            Enforced with 3 Fp3_mul_gadget's that ensure that:
                A.c1 * B.c1 = v1
                A.c0 * B.c0 = v0
                (A.c0+A.c1)*(B.c0+B.c1) = result.c1 + v0 + v1

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */
        v1 = RcCell::new(Fp3_variable::<Fp3T>::new(pb, FMT(annotation_prefix, " v1")));

        compute_v1 = RcCell::new(Fp3_mul_gadget::<Fp3T>::new(
            pb,
            A.c1,
            B.c1,
            *v1,
            FMT(annotation_prefix, " compute_v1"),
        ));

        v0_c0.assign(&pb, result.c0.c0 - Fp6T::non_residue * v1.c2);
        v0_c1.assign(&pb, result.c0.c1 - v1.c0);
        v0_c2.assign(&pb, result.c0.c2 - v1.c1);
        v0 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            v0_c0,
            v0_c1,
            v0_c2,
            FMT(annotation_prefix, " v0"),
        ));

        compute_v0 = RcCell::new(Fp3_mul_gadget::<Fp3T>::new(
            pb,
            A.c0,
            B.c0,
            *v0,
            FMT(annotation_prefix, " compute_v0"),
        ));

        Ac0_plus_Ac1_c0.assign(&pb, A.c0.c0 + A.c1.c0);
        Ac0_plus_Ac1_c1.assign(&pb, A.c0.c1 + A.c1.c1);
        Ac0_plus_Ac1_c2.assign(&pb, A.c0.c2 + A.c1.c2);
        Ac0_plus_Ac1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            Ac0_plus_Ac1_c0,
            Ac0_plus_Ac1_c1,
            Ac0_plus_Ac1_c2,
            FMT(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        Bc0_plus_Bc1_c0.assign(&pb, B.c0.c0 + B.c1.c0);
        Bc0_plus_Bc1_c1.assign(&pb, B.c0.c1 + B.c1.c1);
        Bc0_plus_Bc1_c2.assign(&pb, B.c0.c2 + B.c1.c2);
        Bc0_plus_Bc1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            Bc0_plus_Bc1_c0,
            Bc0_plus_Bc1_c1,
            Bc0_plus_Bc1_c2,
            FMT(annotation_prefix, " Bc0_plus_Bc1"),
        ));

        result_c1_plus_v0_plus_v1_c0.assign(&pb, result.c1.c0 + v0.c0 + v1.c0);
        result_c1_plus_v0_plus_v1_c1.assign(&pb, result.c1.c1 + v0.c1 + v1.c1);
        result_c1_plus_v0_plus_v1_c2.assign(&pb, result.c1.c2 + v0.c2 + v1.c2);
        result_c1_plus_v0_plus_v1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            result_c1_plus_v0_plus_v1_c0,
            result_c1_plus_v0_plus_v1_c1,
            result_c1_plus_v0_plus_v1_c2,
            FMT(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        compute_result_c1 = RcCell::new(Fp3_mul_gadget::<Fp3T>::new(
            pb,
            *Ac0_plus_Ac1,
            *Bc0_plus_Bc1,
            *result_c1_plus_v0_plus_v1,
            FMT(annotation_prefix, " compute_result_c1"),
        ));
        // gadget<FieldT>(&pb, annotation_prefix),
        Self { A, B, result }
    }

    pub fn generate_r1cs_constraints() {
        compute_v0.generate_r1cs_constraints();
        compute_v1.generate_r1cs_constraints();
        compute_result_c1.generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness() {
        compute_v0.generate_r1cs_witness();
        compute_v1.generate_r1cs_witness();

        Ac0_plus_Ac1_c0.evaluate(self.pb);
        Ac0_plus_Ac1_c1.evaluate(self.pb);
        Ac0_plus_Ac1_c2.evaluate(self.pb);

        Bc0_plus_Bc1_c0.evaluate(self.pb);
        Bc0_plus_Bc1_c1.evaluate(self.pb);
        Bc0_plus_Bc1_c2.evaluate(self.pb);

        compute_result_c1.generate_r1cs_witness();

        let Aval = A.get_element();
        let Bval = B.get_element();
        let Rval = Aval * Bval;

        result.generate_r1cs_witness(Rval);

        result_c1_plus_v0_plus_v1_c0.evaluate(self.pb);
        result_c1_plus_v0_plus_v1_c1.evaluate(self.pb);
        result_c1_plus_v0_plus_v1_c2.evaluate(self.pb);

        compute_result_c1.generate_r1cs_witness();
    }
}

impl Fp6_mul_by_2345_gadget<Fp6T> {
    pub fn new(
        pb: &RcCell<protoboard<FieldT>>,
        A: &Fp6_variable<Fp6T>,
        B: &Fp6_variable<Fp6T>,
        result: &Fp6_variable<Fp6T>,
        annotation_prefix: &String,
    ) -> Self {
        /*
            Karatsuba multiplication for Fp6 as a quadratic extension of Fp3:
                v0 = A.c0 * B.c0
                v1 = A.c1 * B.c1
                result.c0 = v0 + non_residue * v1
                result.c1 = (A.c0 + A.c1) * (B.c0 + B.c1) - v0 - v1
            where "non_residue * elem" := (non_residue * elem.c2, elem.c0, elem.c1)

            We know that B.c0.c0 = B.c0.c1 = 0

            Enforced with 2 Fp3_mul_gadget's that ensure that:
                A.c1 * B.c1 = v1
                (A.c0+A.c1)*(B.c0+B.c1) = result.c1 + v0 + v1

            And one multiplication (three direct constraints) that enforces A.c0 * B.c0
            = v0, where B.c0.c0 = B.c0.c1 = 0.

            Note that (u + v * X + t * X^2) * (0 + 0 * X + z * X^2) =
            (v * z * non_residue + t * z * non_residue * X + u * z * X^2)

            Reference:
                "Multiplication and Squaring on Pairing-Friendly Fields"
                Devegili, OhEigeartaigh, Scott, Dahab
        */
        v1 = RcCell::new(Fp3_variable::<Fp3T>::new(pb, FMT(annotation_prefix, " v1")));
        compute_v1 = RcCell::new(Fp3_mul_gadget::<Fp3T>::new(
            pb,
            A.c1,
            B.c1,
            *v1,
            FMT(annotation_prefix, " compute_v1"),
        ));

        /* we inline result.c0 in v0 as follows: v0 = (result.c0.c0 - Fp6T::non_residue * v1->c2, result.c0.c1 - v1->c0, result.c0.c2 - v1->c1) */
        v0 = RcCell::new(Fp3_variable::<Fp3T>::new(pb, FMT(annotation_prefix, " v0")));

        Ac0_plus_Ac1_c0.assign(&pb, A.c0.c0 + A.c1.c0);
        Ac0_plus_Ac1_c1.assign(&pb, A.c0.c1 + A.c1.c1);
        Ac0_plus_Ac1_c2.assign(&pb, A.c0.c2 + A.c1.c2);
        Ac0_plus_Ac1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            Ac0_plus_Ac1_c0,
            Ac0_plus_Ac1_c1,
            Ac0_plus_Ac1_c2,
            FMT(annotation_prefix, " Ac0_plus_Ac1"),
        ));

        Bc0_plus_Bc1_c0.assign(&pb, B.c0.c0 + B.c1.c0);
        Bc0_plus_Bc1_c1.assign(&pb, B.c0.c1 + B.c1.c1);
        Bc0_plus_Bc1_c2.assign(&pb, B.c0.c2 + B.c1.c2);
        Bc0_plus_Bc1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            Bc0_plus_Bc1_c0,
            Bc0_plus_Bc1_c1,
            Bc0_plus_Bc1_c2,
            FMT(annotation_prefix, " Bc0_plus_Bc1"),
        ));

        result_c1_plus_v0_plus_v1_c0.assign(&pb, result.c1.c0 + v0.c0 + v1.c0);
        result_c1_plus_v0_plus_v1_c1.assign(&pb, result.c1.c1 + v0.c1 + v1.c1);
        result_c1_plus_v0_plus_v1_c2.assign(&pb, result.c1.c2 + v0.c2 + v1.c2);
        result_c1_plus_v0_plus_v1 = RcCell::new(Fp3_variable::<Fp3T>::new(
            pb,
            result_c1_plus_v0_plus_v1_c0,
            result_c1_plus_v0_plus_v1_c1,
            result_c1_plus_v0_plus_v1_c2,
            FMT(annotation_prefix, " result_c1_plus_v0_plus_v1"),
        ));

        compute_result_c1 = RcCell::new(Fp3_mul_gadget::<Fp3T>::new(
            pb,
            *Ac0_plus_Ac1,
            *Bc0_plus_Bc1,
            *result_c1_plus_v0_plus_v1,
            FMT(annotation_prefix, " compute_result_c1"),
        ));
        // gadget<FieldT>(&pb, annotation_prefix),
        Self { A, B, result }
    }

    pub fn generate_r1cs_constraints() {
        compute_v1.generate_r1cs_constraints();
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(
                A.c0.c1,
                Fp3T::non_residue * B.c0.c2,
                result.c0.c0 - Fp6T::non_residue * v1.c2,
            ),
            FMT(self.annotation_prefix, " v0.c0"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(A.c0.c2, Fp3T::non_residue * B.c0.c2, result.c0.c1 - v1.c0),
            FMT(self.annotation_prefix, " v0.c1"),
        );
        self.pb.borrow_mut().add_r1cs_constraint(
            r1cs_constraint::<FieldT>(A.c0.c0, B.c0.c2, result.c0.c2 - v1.c1),
            FMT(self.annotation_prefix, " v0.c2"),
        );
        compute_result_c1.generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness() {
        compute_v1.generate_r1cs_witness();

        let A_c0_val = A.c0.get_element();
        let B_c0_val = B.c0.get_element();
        assert!(B_c0_val.c0.is_zero());
        assert!(B_c0_val.c1.is_zero());

        let v0_val = A_c0_val * B_c0_val;
        v0.generate_r1cs_witness(v0_val);

        Ac0_plus_Ac1_c0.evaluate(self.pb);
        Ac0_plus_Ac1_c1.evaluate(self.pb);
        Ac0_plus_Ac1_c2.evaluate(self.pb);

        Bc0_plus_Bc1_c0.evaluate(self.pb);
        Bc0_plus_Bc1_c1.evaluate(self.pb);
        Bc0_plus_Bc1_c2.evaluate(self.pb);

        compute_result_c1.generate_r1cs_witness();

        let Aval = A.get_element();
        let Bval = B.get_element();
        let Rval = Aval * Bval;

        result.generate_r1cs_witness(Rval);

        result_c1_plus_v0_plus_v1_c0.evaluate(self.pb);
        result_c1_plus_v0_plus_v1_c1.evaluate(self.pb);
        result_c1_plus_v0_plus_v1_c2.evaluate(self.pb);

        compute_result_c1.generate_r1cs_witness();
    }
}

impl Fp6_sqr_gadget<Fp6T> {
    pub fn new(
        pb: &RcCell<protoboard<FieldT>>,
        A: &Fp6_variable<Fp6T>,
        result: &Fp6_variable<Fp6T>,
        annotation_prefix: &String,
    ) -> Self {
        // We can't do better than 3 Fp3_mul_gadget's for squaring, so we just use multiplication.
        mul = RcCell::new(Fp6_mul_gadget::<Fp6T>::new(
            pb,
            A,
            A,
            result,
            FMT(annotation_prefix, " mul"),
        ));
        //  gadget<FieldT>(&pb, annotation_prefix),
        Self { A, result }
    }

    pub fn generate_r1cs_constraints() {
        mul.generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness() {
        mul.generate_r1cs_witness();
    }
}
impl Fp6_cyclotomic_sqr_gadget<Fp6T> {
    pub fn new(
        pb: &RcCell<protoboard<FieldT>>,
        A: &Fp6_variable<Fp6T>,
        result: &Fp6_variable<Fp6T>,
        annotation_prefix: &String,
    ) -> Self {
        /*
            my_Fp2 a = my_Fp2(c0.c0, c1.c1);
            my_Fp2 b = my_Fp2(c1.c0, c0.c2);
            my_Fp2 c = my_Fp2(c0.c1, c1.c2);

            my_Fp2 asq = a.squared();
            my_Fp2 bsq = b.squared();
            my_Fp2 csq = c.squared();

            result.c0.c0 = 3 * asq_a - 2 * a_a;
            result.c1.c1 = 3 * asq_b + 2 * a_b;

            result.c0.c1 = 3 * bsq_a - 2 * c_a;
            result.c1.c2 = 3 * bsq_b + 2 * c_b;

            result.c0.c2 = 3 * csq_a - 2 * b_b;
            result.c1.c0 = 3 * my_Fp3::non_residue * csq_b + 2 * b_a;

            return Fp6_2over3_model<n, mbodulus>(my_Fp3(A_a, C_a, B_b),
                                                 my_Fp3(B_a, A_b, C_b))
        */
        a = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            A.c0.c0,
            A.c1.c1,
            FMT(annotation_prefix, " a"),
        ));
        b = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            A.c1.c0,
            A.c0.c2,
            FMT(annotation_prefix, " b"),
        ));
        c = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            A.c0.c1,
            A.c1.c2,
            FMT(annotation_prefix, " c"),
        ));

        asq_c0.assign(&pb, (result.c0.c0 + 2 * a.c0) * FieldT(3).inverse());
        asq_c1.assign(&pb, (result.c1.c1 - 2 * a.c1) * FieldT(3).inverse());

        bsq_c0.assign(&pb, (result.c0.c1 + 2 * c.c0) * FieldT(3).inverse());
        bsq_c1.assign(&pb, (result.c1.c2 - 2 * c.c1) * FieldT(3).inverse());

        csq_c0.assign(&pb, (result.c0.c2 + 2 * b.c1) * FieldT(3).inverse());
        csq_c1.assign(
            &pb,
            (result.c1.c0 - 2 * b.c0) * (FieldT(3) * Fp2T::non_residue).inverse(),
        );

        asq = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            asq_c0,
            asq_c1,
            FMT(annotation_prefix, " asq"),
        ));
        bsq = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            bsq_c0,
            bsq_c1,
            FMT(annotation_prefix, " bsq"),
        ));
        csq = RcCell::new(Fp2_variable::<Fp2T>::new(
            pb,
            csq_c0,
            csq_c1,
            FMT(annotation_prefix, " csq"),
        ));

        compute_asq = RcCell::new(Fp2_sqr_gadget::<Fp2T>::new(
            pb,
            *a,
            *asq,
            FMT(annotation_prefix, " compute_asq"),
        ));
        compute_bsq = RcCell::new(Fp2_sqr_gadget::<Fp2T>::new(
            pb,
            *b,
            *bsq,
            FMT(annotation_prefix, " compute_bsq"),
        ));
        compute_csq = RcCell::new(Fp2_sqr_gadget::<Fp2T>::new(
            pb,
            *c,
            *csq,
            FMT(annotation_prefix, " compute_csq"),
        ));
        // gadget<FieldT>(&pb, annotation_prefix),
        Self { A, result }
    }

    pub fn generate_r1cs_constraints() {
        compute_asq.generate_r1cs_constraints();
        compute_bsq.generate_r1cs_constraints();
        compute_csq.generate_r1cs_constraints();
    }

    pub fn generate_r1cs_witness() {
        let Aval = A.get_element();
        let Rval = Aval.cyclotomic_squared();

        result.generate_r1cs_witness(Rval);

        asq.evaluate();
        bsq.evaluate();
        csq.evaluate();

        compute_asq.generate_r1cs_witness();
        compute_bsq.generate_r1cs_witness();
        compute_csq.generate_r1cs_witness();
    }
}

//#endif // FP6_GADGETS_TCC_
