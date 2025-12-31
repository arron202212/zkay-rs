use ffec::FieldTConfig;
use ffec::scalar_multiplication::multiexp::inner_product;
/** @file
*****************************************************************************

Declaration of interfaces for a SSP ("Square Span Program").

SSPs are defined in \[DFGK14].

References:

\[DFGK14]:
"Square Span Programs with Applications to Succinct NIZK Arguments"
George Danezis, Cedric Fournet, Jens Groth, Markulf Kohlweiss,
ASIACRYPT 2014,
<http://eprint.iacr.org/2014/718>

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef SSP_HPP_
// #define SSP_HPP_

// use  <map>
use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;
use rccell::RcCell;
use std::collections::BTreeMap;
/* forward declaration */

// pub struct ssp_witness;

/**
 * A SSP instance.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs; and
 * - coefficients of the V polynomials in the Lagrange basis.
 *
 * There is no need to store the Z polynomial because it is uniquely
 * determined by the domain (as Z is its vanishing polynomial).
 */

pub struct ssp_instance<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    domain: RcCell<ED>,

    V_in_Lagrange_basis: Vec<BTreeMap<usize, FieldT>>,
}

/**
 * A SSP instance evaluation is a SSP instance that is evaluated at a field element t.
 *
 * Specifically, the datastructure stores:
 * - a choice of domain (corresponding to a certain subset of the field);
 * - the number of variables, the degree, and the number of inputs;
 * - a field element t;
 * - evaluations of the V (and Z) polynomials at t;
 * - evaluations of all monomials of t.
 */

pub struct ssp_instance_evaluation<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> {
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    domain: RcCell<ED>,

    t: FieldT,

    Ht: Vec<FieldT>,
    Vt: Vec<FieldT>,

    Zt: FieldT,
}

/**
 * A SSP witness.
 */

pub struct ssp_witness<FieldT> {
    num_variables: usize,
    degree: usize,
    num_inputs: usize,

    d: FieldT,

    coefficients_for_Vs: Vec<FieldT>,
    coefficients_for_H: Vec<FieldT>,
}

// use crate::relations::arithmetic_programs::ssp::ssp;

//#endif // SSP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a SSP ("Square Span Program").

See ssp.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef SSP_TCC_
// #define SSP_TCC_
use ffec::algebra::scalar_multiplication::multiexp;
// use ffec::scalar_multiplication::multiexp::inner_product;
use ffec::common::profiling;
use ffec::common::utils;
// use fqfft::evaluation_domain::evaluation_domain::evaluation_domain;

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> ssp_instance<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        V_in_Lagrange_basis: Vec<BTreeMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            V_in_Lagrange_basis,
        }
    }

    pub fn new2(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        V_in_Lagrange_basis: Vec<BTreeMap<usize, FieldT>>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            V_in_Lagrange_basis,
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

    pub fn is_satisfied(&self, witness: &ssp_witness<FieldT>) -> bool {
        let t = FieldT::random_element();
        let mut Vt = vec![FieldT::zero(); self.num_variables() + 1];
        let mut Ht = vec![FieldT::zero(); self.degree() + 1];

        let mut Zt = self.domain.borrow().compute_vanishing_polynomial(&t);

        let u = self.domain.borrow().evaluate_all_lagrange_polynomials(&t);

        for i in 0..self.num_variables() + 1 {
            for el in &self.V_in_Lagrange_basis[i] {
                Vt[i] += u[*el.0].clone() * el.1.clone();
            }
        }

        let mut ti = FieldT::one();
        for i in 0..self.degree() + 1 {
            Ht[i] = ti.clone();
            ti *= t.clone();
        }

        let eval_ssp_inst = ssp_instance_evaluation::<FieldT, ED>::new(
            self.domain.clone(),
            self.num_variables(),
            self.degree(),
            self.num_inputs(),
            t,
            Vt,
            Ht,
            Zt,
        );
        return eval_ssp_inst.is_satisfied(witness);
    }
}

impl<FieldT: FieldTConfig, ED: evaluation_domain<FieldT>> ssp_instance_evaluation<FieldT, ED> {
    pub fn new(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        Vt: Vec<FieldT>,
        Ht: Vec<FieldT>,
        Zt: FieldT,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            t,
            Vt,
            Ht,
            Zt,
        }
    }

    pub fn new2(
        domain: RcCell<ED>,
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        t: FieldT,
        Vt: Vec<FieldT>,
        Ht: Vec<FieldT>,
        Zt: FieldT,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            domain,
            t,
            Vt,
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

    pub fn is_satisfied(&self, witness: &ssp_witness<FieldT>) -> bool {
        if self.num_variables() != witness.num_variables() {
            return false;
        }

        if self.degree() != witness.degree() {
            return false;
        }

        if self.num_inputs() != witness.num_inputs() {
            return false;
        }

        if self.num_variables() != witness.coefficients_for_Vs.len() {
            return false;
        }

        if self.degree() + 1 != witness.coefficients_for_H.len() {
            return false;
        }

        if self.Vt.len() != self.num_variables() + 1 {
            return false;
        }

        if self.Ht.len() != self.degree() + 1 {
            return false;
        }

        if self.Zt != self.domain.borrow().compute_vanishing_polynomial(&self.t) {
            return false;
        }

        let mut ans_V = self.Vt[0].clone() + witness.d.clone() * self.Zt.clone();
        let mut ans_H = FieldT::zero();

        ans_V = ans_V
            + inner_product::<FieldT>(
                &self.Vt[1..1 + self.num_variables()],
                &witness.coefficients_for_Vs[..self.num_variables()],
            );
        ans_H = ans_H
            + inner_product::<FieldT>(
                &self.Ht[..self.degree() + 1],
                &witness.coefficients_for_H[..self.degree() + 1],
            );

        if ans_V.squared() - FieldT::one() != ans_H * self.Zt.clone() {
            return false;
        }

        return true;
    }
}
impl<FieldT: FieldTConfig> ssp_witness<FieldT> {
    pub fn new(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d: FieldT,
        coefficients_for_Vs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d,
            coefficients_for_Vs,
            coefficients_for_H,
        }
    }

    pub fn new2(
        num_variables: usize,
        degree: usize,
        num_inputs: usize,
        d: FieldT,
        coefficients_for_Vs: Vec<FieldT>,
        coefficients_for_H: Vec<FieldT>,
    ) -> Self {
        Self {
            num_variables,
            degree,
            num_inputs,
            d,
            coefficients_for_Vs,
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

//#endif // SSP_TCC_
