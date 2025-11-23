// /** @file
//  *****************************************************************************

//  Declaration of interfaces for a SAP ("Square Arithmetic Program").

//  SAPs are defined in \[GM17].

//  References:

//  \[GM17]:
//  "Snarky Signatures: Minimal Signatures of Knowledge from
//   Simulation-Extractable SNARKs",
//  Jens Groth and Mary Maller,
//  IACR-CRYPTO-2017,
//  <https://eprint.iacr.org/2017/540>

//  *****************************************************************************
//  * @author     This file is part of libsnark, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/
//#ifndef SAP_HPP_
// #define SAP_HPP_

// use  <map>
//

use crate::relations::FieldTConfig;
use ffec::scalar_multiplication::multiexp::inner_product;
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use rccell::RcCell;
use std::collections::HashMap;
/* forward declaration */
//
// pub struct sap_witness;

/**
 * A SAP instance.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs; and
 * - coefficients of the A,C polynomials in the Lagrange basis.
 *
 * There is no need to store the Z polynomial because it is uniquely
 * determined by the domain (as Z is its vanishing polynomial).
 */
//
pub struct sap_instance<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    // //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    // //
    domain: RcCell<ED>,

    A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
}

//     sap_instance(domain:&RcCell<ED >,
//                  num_variables:usize,
//                  degree:usize,
//                  num_inputs:usize,
//                  A_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >,
//                  C_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >);

//     sap_instance(domain:&RcCell<ED >,
//                  num_variables:usize,
//                  degree:usize,
//                  num_inputs:usize,
//                  Vec<HashMap<usize, FieldT> > &&A_in_Lagrange_basis,
//                  Vec<HashMap<usize, FieldT> > &&C_in_Lagrange_basis);

//     sap_instance(other:&sap_instance<FieldT>) = default;
//     sap_instance(sap_instance<FieldT> &&other) = default;
//     sap_instance& operator=(other:&sap_instance<FieldT>) = default;
//     sap_instance& operator=(sap_instance<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(witness:&sap_witness<FieldT>) const;
// };

/**
 * A SAP instance evaluation is a SAP instance that is evaluated at a field element t.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs;
 * - a field element t;
 * - evaluations of the A,C (and Z) polynomials at t;
 * - evaluations of all monomials of t;
 * - counts about how many of the above evaluations are in fact non-zero.
 */
//
pub struct sap_instance_evaluation<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,
    //
    domain: RcCell<ED>,

    t: FieldT,

    Ht: Vec<FieldT>,
    At: Vec<FieldT>,
    Ct: Vec<FieldT>,

    Zt: FieldT,
}

//     sap_instance_evaluation(domain:&RcCell<ED >,
//                             num_variables:usize,
//                             degree:usize,
//                             num_inputs:usize,
//                             t:&FieldT,
//                             At:&Vec<FieldT>,
//                             Ct:&Vec<FieldT>,
//                             Ht:&Vec<FieldT>,
//                             Zt:&FieldT);
//     sap_instance_evaluation(domain:&RcCell<ED >,
//                             num_variables:usize,
//                             degree:usize,
//                             num_inputs:usize,
//                             t:&FieldT,
//                             Vec<FieldT> &&At,
//                             Vec<FieldT> &&Ct,
//                             Vec<FieldT> &&Ht,
//                             Zt:&FieldT);

//     sap_instance_evaluation(other:&sap_instance_evaluation<FieldT>) = default;
//     sap_instance_evaluation(sap_instance_evaluation<FieldT> &&other) = default;
//     sap_instance_evaluation& operator=(other:&sap_instance_evaluation<FieldT>) = default;
//     sap_instance_evaluation& operator=(sap_instance_evaluation<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(witness:&sap_witness<FieldT>) const;
// };

/**
 * A SAP witness.
 */
//
pub struct sap_witness<FieldT> {
    //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    //
    d2: FieldT,
    d1: FieldT,

    coefficients_for_ACs: Vec<FieldT>,
    coefficients_for_H: Vec<FieldT>,
}

//     sap_witness(num_variables:usize,
//                 degree:usize,
//                 num_inputs:usize,
//                 d1:&FieldT,
//                 d2:&FieldT,
//                 coefficients_for_ACs:&Vec<FieldT>,
//                 coefficients_for_H:&Vec<FieldT>);

//     sap_witness(num_variables:usize,
//                 degree:usize,
//                 num_inputs:usize,
//                 d1:&FieldT,
//                 d2:&FieldT,
//                 coefficients_for_ACs:&Vec<FieldT>,
//                 Vec<FieldT> &&coefficients_for_H);

//     sap_witness(other:&sap_witness<FieldT>) = default;
//     sap_witness(sap_witness<FieldT> &&other) = default;
//     sap_witness& operator=(other:&sap_witness<FieldT>) = default;
//     sap_witness& operator=(sap_witness<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;
// };

// use crate::relations::arithmetic_programs::sap::sap;

//#endif // SAP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a SAP ("Square Arithmetic Program").

See sap.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef SAP_TCC_
// #define SAP_TCC_
use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> sap_instance<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            A_in_Lagrange_basis,
            C_in_Lagrange_basis,
        }
    }

    pub fn new2(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            A_in_Lagrange_basis,
            C_in_Lagrange_basis,
        }
    }

    pub fn num_variables(&self) -> usize {
        return self.num_variables;
    }

    pub fn degree(&self) -> usize {
        return self.degree;
    }

    pub fn num_inputs(&self) -> usize {
        return self.num_inputs;
    }

    pub fn is_satisfied(&self, witness: sap_witness<FieldT>) -> bool {
        let mut t = FieldT::random_element();

        let mut At = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Ct = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Ht = Vec::with_capacity(self.degree() + 1);

        let mut Zt = self.domain.borrow().compute_vanishing_polynomial(&t);

        let u = self.domain.borrow().evaluate_all_lagrange_polynomials(&t);

        for i in 0..self.num_variables() + 1 {
            for el in &self.A_in_Lagrange_basis[i] {
                At[i] += u[*el.0].clone() * el.1.clone();
            }

            for el in &self.C_in_Lagrange_basis[i] {
                Ct[i] += u[*el.0].clone() * el.1.clone();
            }
        }

        let mut ti = FieldT::one();
        for i in 0..self.degree() + 1 {
            Ht[i] = ti.clone();
            ti *= t.clone();
        }

        let eval_sap_inst = sap_instance_evaluation::<FieldT, ED>::new(
            self.domain.clone(),
            self.num_variables(),
            self.degree(),
            self.num_inputs(),
            t,
            At,
            Ct,
            Ht,
            Zt,
        );
        return eval_sap_inst.is_satisfied(witness);
    }
}

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> sap_instance_evaluation<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        At: Vec<FieldT>,
        Ct: Vec<FieldT>,
        Ht: Vec<FieldT>,
        Zt: FieldT,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            t,
            At,
            Ct,
            Ht,
            Zt,
        }
    }

    pub fn new1(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        At: Vec<FieldT>,
        Ct: Vec<FieldT>,
        Ht: Vec<FieldT>,
        Zt: FieldT,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            t,
            At,
            Ct,
            Ht,
            Zt,
        }
    }

    pub fn num_variables(&self) -> usize {
        return self.num_variables;
    }

    pub fn degree(&self) -> usize {
        return self.degree;
    }

    pub fn num_inputs(&self) -> usize {
        return self.num_inputs;
    }

    pub fn is_satisfied(&self, witness: sap_witness<FieldT>) -> bool {
        if self.num_variables() != witness.num_variables() {
            return false;
        }

        if self.degree() != witness.degree() {
            return false;
        }

        if self.num_inputs() != witness.num_inputs() {
            return false;
        }

        if self.num_variables() != witness.coefficients_for_ACs.len() {
            return false;
        }

        if self.degree() + 1 != witness.coefficients_for_H.len() {
            return false;
        }

        if self.At.len() != self.num_variables() + 1 || self.Ct.len() != self.num_variables() + 1 {
            return false;
        }

        if self.Ht.len() != self.degree() + 1 {
            return false;
        }

        if self.Zt != self.domain.borrow().compute_vanishing_polynomial(&self.t) {
            return false;
        }

        let mut ans_A = self.At[0].clone() + witness.d1.clone() * self.Zt.clone();
        let mut ans_C = self.Ct[0].clone() + witness.d2.clone() * self.Zt.clone();
        let mut ans_H = FieldT::zero();

        ans_A = ans_A
            + inner_product::<FieldT>(
                &self.At[1..1 + self.num_variables()],
                &witness.coefficients_for_ACs[..self.num_variables()],
            );
        ans_C = ans_C
            + inner_product::<FieldT>(
                &self.Ct[1..1 + self.num_variables()],
                &witness.coefficients_for_ACs[..self.num_variables()],
            );
        ans_H = ans_H
            + inner_product::<FieldT>(
                &self.Ht[..self.degree() + 1],
                &witness.coefficients_for_H[..self.degree() + 1],
            );

        if ans_A.clone() * ans_A.clone() - ans_C != ans_H * self.Zt.clone() {
            return false;
        }

        return true;
    }
}
impl<FieldT> sap_witness<FieldT> {
    pub fn new(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d1: FieldT,
        d2: FieldT,
        coefficients_for_ACs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d1,
            d2,
            coefficients_for_ACs,
            coefficients_for_H,
        }
    }

    pub fn new2(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d1: FieldT,
        d2: FieldT,
        coefficients_for_ACs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d1,
            d2,
            coefficients_for_ACs,
            coefficients_for_H,
        }
    }

    pub fn num_variables(&self) -> usize {
        return self.num_variables;
    }

    pub fn degree(&self) -> usize {
        return self.degree;
    }

    pub fn num_inputs(&self) -> usize {
        return self.num_inputs;
    }
}

//#endif // SAP_TCC_
