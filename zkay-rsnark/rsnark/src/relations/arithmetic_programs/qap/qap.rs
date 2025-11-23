use crate::relations::FieldTConfig;
use ffec::algebra::scalar_multiplication::multiexp::inner_product;
/** @file
*****************************************************************************

Declaration of interfaces for a QAP ("Quadratic Arithmetic Program").

QAPs are defined in \[GGPR13].

References:

\[GGPR13]:
"Quadratic span programs and succinct NIZKs without PCPs",
Rosario Gennaro, Craig Gentry, Bryan Parno, Mariana Raykova,
EUROCRYPT 2013,
<http://eprint.iacr.org/2012/215>

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef QAP_HPP_
// #define QAP_HPP_

// use  <map>
//
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use rccell::RcCell;
use std::collections::HashMap;
/* forward declaration */
//
// pub struct qap_witness;

/**
 * A QAP instance.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs; and
 * - coefficients of the A,B,C polynomials in the Lagrange basis.
 *
 * There is no need to store the Z polynomial because it is uniquely
 * determined by the domain (as Z is its vanishing polynomial).
 */
//
pub struct qap_instance<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    domain: RcCell<ED>,

    A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    B_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
}
// impl qap_instance<FieldT>{
//     qap_instance(domain:&RcCell<ED>
//                  num_variables:usize
//                  degree:usize
//                  num_inputs:usize
//                  A_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >
//                  B_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >
//                  &C_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >);

//     qap_instance(domain:&RcCell<ED>
//                  num_variables:usize
//                  degree:usize
//                  num_inputs:usize
//                 Vec<HashMap<usize, FieldT> > &&A_in_Lagrange_basis,
//                 Vec<HashMap<usize, FieldT> > &&B_in_Lagrange_basis,
//                 Vec<HashMap<usize, FieldT> > &&C_in_Lagrange_basis);

//     qap_instance(&other:qap_instance<FieldT>) = default;
//     qap_instance(qap_instance<FieldT> &&other) = default;
//     qap_instance& operator=(&other:qap_instance<FieldT>) = default;
//     qap_instance& operator=(qap_instance<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(&witness:qap_witness<FieldT>) const;
// }

/**
 * A QAP instance evaluation is a QAP instance that is evaluated at a field element t.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs;
 * - a field element t;
 * - evaluations of the A,B,C (and Z) polynomials at t;
 * - evaluations of all monomials of t;
 * - counts about how many of the above evaluations are in fact non-zero.
 */
//
pub struct qap_instance_evaluation<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    domain: RcCell<ED>,

    t: FieldT,

    At: Vec<FieldT>,
    Bt: Vec<FieldT>,
    Ct: Vec<FieldT>,
    Ht: Vec<FieldT>,

    Zt: FieldT,
}

//     qap_instance_evaluation(domain:&RcCell<ED>
//                             num_variables:usize
//                             degree:usize
//                             num_inputs:usize
//                             t:&FieldT
//                             At:&Vec<FieldT>
//                             Bt:&Vec<FieldT>
//                             Ct:&Vec<FieldT>
//                             Ht:&Vec<FieldT>
//                             &Zt:FieldT);
//     qap_instance_evaluation(domain:&RcCell<ED>
//                             num_variables:usize
//                             degree:usize
//                             num_inputs:usize
//                             t:&FieldT
//                            Vec<FieldT> &&At,
//                            Vec<FieldT> &&Bt,
//                            Vec<FieldT> &&Ct,
//                            Vec<FieldT> &&Ht,
//                             &Zt:FieldT);

//     qap_instance_evaluation(&other:qap_instance_evaluation<FieldT>) = default;
//     qap_instance_evaluation(qap_instance_evaluation<FieldT> &&other) = default;
//     qap_instance_evaluation& operator=(&other:qap_instance_evaluation<FieldT>) = default;
//     qap_instance_evaluation& operator=(qap_instance_evaluation<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(&witness:qap_witness<FieldT>) const;
// };

/**
 * A QAP witness.
 */
//
pub struct qap_witness<FieldT> {
    //
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    d1: FieldT,
    d2: FieldT,
    d3: FieldT,

    coefficients_for_ABCs: Vec<FieldT>,
    coefficients_for_H: Vec<FieldT>,
}

//     qap_witness(num_variables:usize
//                 degree:usize
//                 num_inputs:usize
//                 d1:&FieldT
//                 d2:&FieldT
//                 d3:&FieldT
//                 coefficients_for_ABCs:&Vec<FieldT>
//                 &coefficients_for_H:Vec<FieldT>);

//     qap_witness(num_variables:usize
//                 degree:usize
//                 num_inputs:usize
//                 d1:&FieldT
//                 d2:&FieldT
//                 d3:&FieldT
//                 coefficients_for_ABCs:&Vec<FieldT>
//                Vec<FieldT> &&coefficients_for_H);

//     qap_witness(&other:qap_witness<FieldT>) = default;
//     qap_witness(qap_witness<FieldT> &&other) = default;
//     qap_witness& operator=(&other:qap_witness<FieldT>) = default;
//     qap_witness& operator=(qap_witness<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;
// };

// use crate::relations::arithmetic_programs::qap::qap;

//#endif // QAP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a QAP ("Quadratic Arithmetic Program").

See qap.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef QAP_TCC_
// #define QAP_TCC_
use ffec::algebra::scalar_multiplication::multiexp;
use ffec::common::profiling;
use ffec::common::utils;

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> qap_instance<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        B_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            A_in_Lagrange_basis,
            B_in_Lagrange_basis,
            C_in_Lagrange_basis,
        }
    }

    pub fn new2(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        A_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        B_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
        C_in_Lagrange_basis: Vec<HashMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            A_in_Lagrange_basis,
            B_in_Lagrange_basis,
            C_in_Lagrange_basis,
        }
    }

    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    pub fn is_satisfied(&self, witness: &qap_witness<FieldT>) -> bool {
        let mut t = FieldT::random_element();

        let mut At = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Bt = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Ct = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Ht = vec![FieldT::zero(); self.degree() + 1];

        let mut Zt = self.domain.borrow().compute_vanishing_polynomial(&t);

        let u = self.domain.borrow().evaluate_all_lagrange_polynomials(&t);

        for i in 0..self.num_variables() + 1 {
            for el in &self.A_in_Lagrange_basis[i] {
                At[i] += u[*el.0].clone() * el.1.clone();
            }

            for el in &self.B_in_Lagrange_basis[i] {
                Bt[i] += u[*el.0].clone() * el.1.clone();
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

        let eval_qap_inst = qap_instance_evaluation::<FieldT, ED>::new(
            self.domain.clone(),
            self.num_variables(),
            self.degree(),
            self.num_inputs(),
            t,
            At,
            Bt,
            Ct,
            Ht,
            Zt,
        );
        return eval_qap_inst.is_satisfied(witness);
    }
}

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> qap_instance_evaluation<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        At: Vec<FieldT>,
        Bt: Vec<FieldT>,
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
            Bt,
            Ct,
            Ht,
            Zt,
        }
    }

    pub fn qap_instance_evaluation(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        At: Vec<FieldT>,
        Bt: Vec<FieldT>,
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
            Bt,
            Ct,
            Ht,
            Zt,
        }
    }

    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }

    pub fn is_satisfied(&self, witness: &qap_witness<FieldT>) -> bool {
        if self.num_variables() != witness.num_variables() {
            return false;
        }

        if self.degree() != witness.degree() {
            return false;
        }

        if self.num_inputs() != witness.num_inputs() {
            return false;
        }

        if self.num_variables() != witness.coefficients_for_ABCs.len() {
            return false;
        }

        if self.degree() + 1 != witness.coefficients_for_H.len() {
            return false;
        }

        if self.At.len() != self.num_variables() + 1
            || self.Bt.len() != self.num_variables() + 1
            || self.Ct.len() != self.num_variables() + 1
        {
            return false;
        }

        if self.Ht.len() != self.degree() + 1 {
            return false;
        }

        if self.Zt != self.domain.borrow().compute_vanishing_polynomial(&self.t) {
            return false;
        }

        let mut ans_A = self.At[0].clone() + witness.d1.clone() * self.Zt.clone();
        let mut ans_B = self.Bt[0].clone() + witness.d2.clone() * self.Zt.clone();
        let mut ans_C = self.Ct[0].clone() + witness.d3.clone() * self.Zt.clone();
        let mut ans_H = FieldT::zero();

        ans_A = ans_A
            + inner_product::<FieldT>(
                &self.At[1..1 + self.num_variables()],
                &witness.coefficients_for_ABCs[..self.num_variables()],
            );
        ans_B = ans_B
            + inner_product::<FieldT>(
                &self.Bt[1..1 + self.num_variables()],
                &witness.coefficients_for_ABCs[..self.num_variables()],
            );
        ans_C = ans_C
            + inner_product::<FieldT>(
                &self.Ct[1..1 + self.num_variables()],
                &witness.coefficients_for_ABCs[..self.num_variables()],
            );
        ans_H = ans_H
            + inner_product::<FieldT>(
                &self.Ht[..self.degree() + 1],
                &witness.coefficients_for_H[..self.degree() + 1],
            );

        if ans_A * ans_B.clone() - ans_C != ans_H * self.Zt.clone() {
            return false;
        }

        return true;
    }
}

impl<FieldT> qap_witness<FieldT> {
    pub fn new(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d1: FieldT,
        d2: FieldT,
        d3: FieldT,
        coefficients_for_ABCs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d1,
            d2,
            d3,
            coefficients_for_ABCs,
            coefficients_for_H,
        }
    }

    pub fn qap_witness(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d1: FieldT,
        d2: FieldT,
        d3: FieldT,
        coefficients_for_ABCs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d1,
            d2,
            d3,
            coefficients_for_ABCs,
            coefficients_for_H,
        }
    }

    pub fn num_variables(&self) -> usize {
        self.num_variables
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn num_inputs(&self) -> usize {
        self.num_inputs
    }
}

//#endif // QAP_TCC_
