use crate::relations::arithmetic_programs::qap::qap::{
    qap_instance, qap_instance_evaluation, qap_witness,
};
use crate::relations::arithmetic_programs::sap::sap::{
    sap_instance, sap_instance_evaluation, sap_witness,
};
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_primary_input,
};
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs::{
    r1cs_auxiliary_input, r1cs_constraint, r1cs_constraint_system, r1cs_primary_input,
    r1cs_variable_assignment,
};
use crate::relations::variable::{
    SubLinearCombinationConfig, SubVariableConfig, linear_combination,
};
use ffec::FieldTConfig;
use ffec::common::profiling;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::common::utils;
use ffec::common::utils::FMT;
use fqfft::evaluation_domain::get_evaluation_domain::get_evaluation_domain;
use fqfft::evaluation_domain::{evaluation_domain::evaluation_domain, get_evaluation_domain};
use rccell::RcCell;
/** @file
*****************************************************************************

Declaration of interfaces for a R1CS-to-SAP reduction, that is, constructing
a SAP ("Square Arithmetic Program") from a R1CS ("Rank-1 Constraint System").

SAPs are defined and constructed from R1CS in \[GM17].

The implementation of the reduction follows, extends, and optimizes
the efficient approach described in Appendix E of \[BCGTV13].

References:

\[BCGTV13]
"SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
CRYPTO 2013,
<http://eprint.iacr.org/2013/507>

\[GM17]:
"Snarky Signatures: Minimal Signatures of Knowledge from
 Simulation-Extractable SNARKs",
Jens Groth and Mary Maller,
IACR-CRYPTO-2017,
<https://eprint.iacr.org/2017/540>

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef R1CS_TO_SAP_HPP_
// // #define R1CS_TO_SAP_HPP_
use std::collections::HashMap;
/**
 * Helper function to find evaluation domain that will be used by the reduction
 * for a given R1CS instance.
 */
//
// RcCell<evaluation_domain<FieldT> > r1cs_to_sap_get_domain(&cs:r1cs_constraint_system<FieldT>);

/**
 * Instance map for the R1CS-to-QAP reduction.
 */
//
// sap_instance<FieldT> r1cs_to_sap_instance_map(&cs:r1cs_constraint_system<FieldT>);

/**
 * Instance map for the R1CS-to-QAP reduction followed by evaluation of the resulting QAP instance.
 */
//
// sap_instance_evaluation<FieldT> r1cs_to_sap_instance_map_with_evaluation(cs:&r1cs_constraint_system<FieldT>
//                                                                          &t:FieldT);

/**
 * Witness map for the R1CS-to-QAP reduction.
 *
 * The witness map takes zero knowledge into account when d1,d2 are random.
 */
//
// sap_witness<FieldT> r1cs_to_sap_witness_map(cs:&r1cs_constraint_system<FieldT>
//                                             primary_input:&r1cs_primary_input<FieldT>
//                                             auxiliary_input:&r1cs_auxiliary_input<FieldT>
//                                             d1:&FieldT
//                                             &d2:FieldT);

// use crate::reductions::r1cs_to_sap::r1cs_to_sap;

// //#endif // R1CS_TO_SAP_HPP_
/** @file
*****************************************************************************

Implementation of interfaces for a R1CS-to-SAP reduction.

See r1cs_to_qap.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
// //#ifndef R1CS_TO_SAP_TCC_
// // #define R1CS_TO_SAP_TCC_

/**
 * Helper function to multiply a field element by 4 efficiently
 */

pub fn times_four<FieldT: FieldTConfig>(x: FieldT) -> FieldT {
    let times_two = x.clone() + x.clone();
    return times_two.clone() + times_two.clone();
}

/**
 * Helper function to find evaluation domain that will be used by the reduction
 * for a given R1CS instance.
 */

pub fn r1cs_to_sap_get_domain<
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
) -> RcCell<ED> {
    /*
     * the SAP instance will have:
     * - two constraints for every constraint in the original constraint system
     * - two constraints for every public input, except the 0th, which
     *   contributes just one extra constraint
     * see comments in r1cs_to_sap_instance_map for details on where these
     * constraints come from.
     */
    return get_evaluation_domain::<FieldT, ED>(2 * cs.num_constraints() + 2 * cs.num_inputs() + 1)
        .unwrap();
}

/**
 * Instance map for the R1CS-to-SAP reduction.
 */
pub fn r1cs_to_sap_instance_map<
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
) -> sap_instance<FieldT, ED> {
    enter_block("Call to r1cs_to_sap_instance_map", false);

    let domain = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut A_in_Lagrange_basis =
        Vec::<HashMap<usize, FieldT>>::with_capacity(sap_num_variables + 1);
    let mut C_in_Lagrange_basis =
        Vec::<HashMap<usize, FieldT>>::with_capacity(sap_num_variables + 1);

    enter_block("Compute polynomials A, C in Lagrange basis", false);
    /**
     * process R1CS constraints, converting a constraint of the form
     *   \sum a_i x_i * \sum b_i x_i = \sum c_i x_i
     * into two constraints
     *   (\sum (a_i + b_i) x_i)^2 = 4 \sum c_i x_i + x'_i
     *   (\sum (a_i - b_i) x_i)^2 = x'_i
     * where x'_i is an extra variable (a separate one for each original
     * constraint)
     *
     * this adds 2 * cs.num_constraints() constraints
     *   (numbered 0 .. 2 * cs.num_constraints() - 1)
     * and cs.num_constraints() extra variables
     *   (numbered cs.num_variables() + 1 .. cs.num_variables() + cs.num_constraints())
     */
    let extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].a.terms.len() {
            *A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index.index]
                .entry(2 * i)
                .or_insert(FieldT::zero()) += cs.constraints[i].a.terms[j].coeff.clone();
            *A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index.index]
                .entry(2 * i + 1)
                .or_insert(FieldT::zero()) += cs.constraints[i].a.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].b.terms.len() {
            *A_in_Lagrange_basis[cs.constraints[i].b.terms[j].index.index]
                .entry(2 * i)
                .or_insert(FieldT::zero()) += cs.constraints[i].b.terms[j].coeff.clone();
            *A_in_Lagrange_basis[cs.constraints[i].b.terms[j].index.index]
                .entry(2 * i + 1)
                .or_insert(FieldT::zero()) -= cs.constraints[i].b.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].c.terms.len() {
            *C_in_Lagrange_basis[cs.constraints[i].c.terms[j].index.index]
                .entry(2 * i)
                .or_insert(FieldT::zero()) +=
                times_four(cs.constraints[i].c.terms[j].coeff.clone());
        }

        *C_in_Lagrange_basis[extra_var_offset + i]
            .entry(2 * i)
            .or_insert(FieldT::zero()) += FieldT::one();
        *C_in_Lagrange_basis[extra_var_offset + i]
            .entry(2 * i + 1)
            .or_insert(FieldT::zero()) += FieldT::one();
    }

    /**
     * add and convert the extra constraints
     *     x_i * 1 = x_i
     * to ensure that the polynomials 0 .. cs.num_inputs() are linearly
     * independent from each other and the rest, which is required for security
     * proofs (see [GM17, p. 29])
     *
     * note that i = 0 is a special case, where this constraint is expressible
     * as x_0^2 = x_0,
     * whereas for every other i we introduce an extra variable x''_i and do
     *   (x_i + x_0)^2 = 4 x_i + x''_i
     *   (x_i - x_0)^2 = x''_i
     *
     * this adds 2 * cs.num_inputs() + 1 extra constraints
     *   (numbered 2 * cs.num_constraints() ..
     *             2 * cs.num_constraints() + 2 * cs.num_inputs())
     * and cs.num_inputs() extra variables
     *   (numbered cs.num_variables() + cs.num_constraints() + 1 ..
     *             cs.num_variables() + cs.num_constraints() + cs.num_inputs())
     */
    let extra_constr_offset = 2 * cs.num_constraints();
    let extra_var_offset2 = cs.num_variables() + cs.num_constraints();

    //   NB: extra variables start at (extra_var_offset2 + 1), because i starts at
    // 1 below

    A_in_Lagrange_basis[0].insert(extra_constr_offset, FieldT::one());
    C_in_Lagrange_basis[0].insert(extra_constr_offset, FieldT::one());

    for i in 1..=cs.num_inputs() {
        *A_in_Lagrange_basis[i]
            .entry(extra_constr_offset + 2 * i - 1)
            .or_insert(FieldT::zero()) += FieldT::one();
        *A_in_Lagrange_basis[0]
            .entry(extra_constr_offset + 2 * i - 1)
            .or_insert(FieldT::zero()) += FieldT::one();
        *C_in_Lagrange_basis[i]
            .entry(extra_constr_offset + 2 * i - 1)
            .or_insert(FieldT::zero()) += times_four(FieldT::one());
        *C_in_Lagrange_basis[extra_var_offset2 + i]
            .entry(extra_constr_offset + 2 * i - 1)
            .or_insert(FieldT::zero()) += FieldT::one();

        *A_in_Lagrange_basis[i]
            .entry(extra_constr_offset + 2 * i)
            .or_insert(FieldT::zero()) += FieldT::one();
        *A_in_Lagrange_basis[0]
            .entry(extra_constr_offset + 2 * i)
            .or_insert(FieldT::zero()) -= FieldT::one();
        *C_in_Lagrange_basis[extra_var_offset2 + i]
            .entry(2 * cs.num_constraints() + 2 * i)
            .or_insert(FieldT::zero()) += FieldT::one();
    }

    leave_block("Compute polynomials A, C in Lagrange basis", false);

    leave_block("Call to r1cs_to_sap_instance_map", false);

    return sap_instance::<FieldT, ED>::new(
        domain,
        sap_num_variables,
        ED::M,
        cs.num_inputs(),
        A_in_Lagrange_basis,
        C_in_Lagrange_basis,
    );
}

/**
 * Instance map for the R1CS-to-SAP reduction followed by evaluation
 * of the resulting QAP instance.
 */
pub fn r1cs_to_sap_instance_map_with_evaluation<
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    t: &FieldT,
) -> sap_instance_evaluation<FieldT, ED> {
    enter_block("Call to r1cs_to_sap_instance_map_with_evaluation", false);

    let domain: RcCell<ED> = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let (mut At, mut Ct, mut Ht) = (
        vec![FieldT::zero(); sap_num_variables + 1],
        vec![FieldT::zero(); sap_num_variables + 1],
        Vec::with_capacity(ED::M + 1),
    );

    let Zt = domain.borrow().compute_vanishing_polynomial(t);

    enter_block("Compute evaluations of A, C, H at t", false);
    let u = domain.borrow().evaluate_all_lagrange_polynomials(t);
    /**
     * add and process all constraints as in r1cs_to_sap_instance_map
     */
    let extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].a.terms.len() {
            At[cs.constraints[i].a.terms[j].index.index] +=
                u[2 * i].clone() * cs.constraints[i].a.terms[j].coeff.clone();
            At[cs.constraints[i].a.terms[j].index.index] +=
                u[2 * i + 1].clone() * cs.constraints[i].a.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].b.terms.len() {
            At[cs.constraints[i].b.terms[j].index.index] +=
                u[2 * i].clone() * cs.constraints[i].b.terms[j].coeff.clone();
            At[cs.constraints[i].b.terms[j].index.index] -=
                u[2 * i + 1].clone() * cs.constraints[i].b.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].c.terms.len() {
            Ct[cs.constraints[i].c.terms[j].index.index] +=
                times_four(u[2 * i].clone() * cs.constraints[i].c.terms[j].coeff.clone());
        }

        Ct[extra_var_offset + i] += u[2 * i].clone();
        Ct[extra_var_offset + i] += u[2 * i + 1].clone();
    }

    let extra_constr_offset = 2 * cs.num_constraints();
    let extra_var_offset2 = cs.num_variables() + cs.num_constraints();

    At[0] += u[extra_constr_offset].clone();
    Ct[0] += u[extra_constr_offset].clone();

    for i in 1..=cs.num_inputs() {
        At[i] += u[extra_constr_offset + 2 * i - 1].clone();
        At[0] += u[extra_constr_offset + 2 * i - 1].clone();
        Ct[i] += times_four(u[extra_constr_offset + 2 * i - 1].clone());
        Ct[extra_var_offset2 + i] += u[extra_constr_offset + 2 * i - 1].clone();

        At[i] += u[extra_constr_offset + 2 * i].clone();
        At[0] -= u[extra_constr_offset + 2 * i].clone();
        Ct[extra_var_offset2 + i] += u[extra_constr_offset + 2 * i].clone();
    }

    let mut ti = FieldT::one();
    for i in 0..ED::M + 1 {
        Ht.push(ti.clone());
        ti *= t.clone();
    }
    leave_block("Compute evaluations of A, C, H at t", false);

    leave_block("Call to r1cs_to_sap_instance_map_with_evaluation", false);

    return sap_instance_evaluation::<FieldT, ED>::new(
        domain,
        sap_num_variables,
        ED::M,
        cs.num_inputs(),
        t.clone(),
        At,
        Ct,
        Ht,
        Zt,
    );
}

/**
* Witness map for the R1CS-to-SAP reduction.
*
* The witness map takes zero knowledge into account when d1, d2 are random.
*
* More precisely, compute the coefficients
*     h_0,h_1,...,h_n
* of the polynomial
*     H(z)->Self= (A(z)*A(z)-C(z))/Z(z)
* where
*   A(z)->Self= A_0(z) + \sum_{k=1}^{m} w_k A_k(z) + d1 * Z(z)
*   C(z)->Self= C_0(z) + \sum_{k=1}^{m} w_k C_k(z) + d2 * Z(z)
*   Z(z)->Self= "vanishing polynomial of set S"
* and
*   m = number of variables of the SAP
*   n = degree of the SAP
*
* This is done as follows:
*  (1) compute evaluations of A,C on S = {sigma_1,...,sigma_n}
*  (2) compute coefficients of A,C
*  (3) compute evaluations of A,C on T = "coset of S"
*  (4) compute evaluation of H on T
*  (5) compute coefficients of H
*  (6) patch H to account for d1,d2
       (i.e., add coefficients of the polynomial (2*d1*A - d2 + d1^2 * Z))
*
* The code below is not as simple as the above high-level description due to
* some reshuffling to save space.
*/
pub fn r1cs_to_sap_witness_map<
    FieldT: FieldTConfig,
    ED: evaluation_domain<FieldT>,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    primary_input: &r1cs_primary_input<FieldT>,
    auxiliary_input: &r1cs_auxiliary_input<FieldT>,
    d1: &FieldT,
    d2: &FieldT,
) -> sap_witness<FieldT> {
    enter_block("Call to r1cs_to_sap_witness_map", false);

    /* sanity check */
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let domain: RcCell<ED> = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut full_variable_assignment: Vec<_> = primary_input
        .iter()
        .chain(auxiliary_input)
        .cloned()
        .collect();

    /**
     * we need to generate values of all the extra variables that we added
     * during the reduction
     *
     * note: below, we pass full_variable_assignment into the .evaluate()
     * method of the R1CS constraints. however, these extra variables shouldn't
     * be a problem, because .evaluate() only accesses the variables that are
     * actually used in the constraint.
     */
    for i in 0..cs.num_constraints() {
        /**
         * this is variable (extra_var_offset + i), an extra variable
         * we introduced that is not present in the input.
         * its value is (a - b)^2
         */
        let mut extra_var = cs.constraints[i].a.evaluate(&full_variable_assignment)
            - cs.constraints[i].b.evaluate(&full_variable_assignment);
        extra_var = extra_var.clone() * extra_var.clone();
        full_variable_assignment.push(extra_var);
    }
    for i in 1..=cs.num_inputs() {
        /**
         * this is variable (extra_var_offset2 + i), an extra variable
         * we introduced that is not present in the input.
         * its value is (x_i - 1)^2
         */
        let mut extra_var = full_variable_assignment[i - 1].clone() - FieldT::one();
        extra_var = extra_var.clone() * extra_var.clone();
        full_variable_assignment.push(extra_var);
    }

    enter_block("Compute evaluation of polynomial A on set S", false);
    let mut aA = vec![FieldT::zero(); ED::M];

    /* account for all constraints, as in r1cs_to_sap_instance_map */
    for i in 0..cs.num_constraints() {
        aA[2 * i] += cs.constraints[i].a.evaluate(&full_variable_assignment);
        aA[2 * i] += cs.constraints[i].b.evaluate(&full_variable_assignment);

        aA[2 * i + 1] += cs.constraints[i].a.evaluate(&full_variable_assignment);
        aA[2 * i + 1] -= cs.constraints[i].b.evaluate(&full_variable_assignment);
    }

    let extra_constr_offset = 2 * cs.num_constraints();

    aA[extra_constr_offset] += FieldT::one();

    for i in 1..=cs.num_inputs() {
        aA[extra_constr_offset + 2 * i - 1] += full_variable_assignment[i - 1].clone();
        aA[extra_constr_offset + 2 * i - 1] += FieldT::one();

        aA[extra_constr_offset + 2 * i] += full_variable_assignment[i - 1].clone();
        aA[extra_constr_offset + 2 * i] -= FieldT::one();
    }

    leave_block("Compute evaluation of polynomial A on set S", false);

    enter_block("Compute coefficients of polynomial A", false);
    domain.borrow().iFFT(&aA);
    leave_block("Compute coefficients of polynomial A", false);

    enter_block("Compute ZK-patch", false);
    let mut coefficients_for_H = vec![FieldT::zero(); ED::M + 1];
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    /* add coefficients of the polynomial (2*d1*A - d2) + d1*d1*Z */
    for i in 0..ED::M {
        coefficients_for_H[i] = (d1.clone() * aA[i].clone()) + (d1.clone() * aA[i].clone());
    }
    coefficients_for_H[0] -= d2.clone();
    domain
        .borrow()
        .add_poly_Z(&(d1.clone() * d1.clone()), &coefficients_for_H);
    leave_block("Compute ZK-patch", false);

    enter_block("Compute evaluation of polynomial A on set T", false);
    domain
        .borrow()
        .cosetFFT(&aA, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial A on set T", false);

    enter_block("Compute evaluation of polynomial H on set T", false);
    let mut H_tmp = aA.clone(); // can overwrite aA because it is not used later
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..ED::M {
        H_tmp[i] = aA[i].clone() * aA[i].clone();
    }

    enter_block("Compute evaluation of polynomial C on set S", false);
    let mut aC = vec![FieldT::zero(); ED::M];
    /* again, accounting for all constraints */
    let extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints() {
        aC[2 * i] += times_four(cs.constraints[i].c.evaluate(&full_variable_assignment));

        aC[2 * i] += full_variable_assignment[extra_var_offset + i - 1].clone();
        aC[2 * i + 1] += full_variable_assignment[extra_var_offset + i - 1].clone();
    }

    let extra_var_offset2 = cs.num_variables() + cs.num_constraints();
    aC[extra_constr_offset] += FieldT::one();

    for i in 1..=cs.num_inputs() {
        aC[extra_constr_offset + 2 * i - 1] += times_four(full_variable_assignment[i - 1].clone());

        aC[extra_constr_offset + 2 * i - 1] +=
            full_variable_assignment[extra_var_offset2 + i - 1].clone();
        aC[extra_constr_offset + 2 * i] +=
            full_variable_assignment[extra_var_offset2 + i - 1].clone();
    }

    leave_block("Compute evaluation of polynomial C on set S", false);

    enter_block("Compute coefficients of polynomial C", false);
    domain.borrow().iFFT(&aC);
    leave_block("Compute coefficients of polynomial C", false);

    enter_block("Compute evaluation of polynomial C on set T", false);
    domain
        .borrow()
        .cosetFFT(&aC, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial C on set T", false);

    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..ED::M {
        H_tmp[i] = (H_tmp[i].clone() - aC[i].clone());
    }

    enter_block("Divide by Z on set T", false);
    domain.borrow().divide_by_Z_on_coset(&H_tmp);
    leave_block("Divide by Z on set T", false);

    leave_block("Compute evaluation of polynomial H on set T", false);

    enter_block("Compute coefficients of polynomial H", false);
    domain
        .borrow()
        .icosetFFT(&H_tmp, &FieldT::multiplicative_generator());
    leave_block("Compute coefficients of polynomial H", false);

    enter_block("Compute sum of H and ZK-patch", false);
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    // //#endif
    for i in 0..ED::M {
        coefficients_for_H[i] += H_tmp[i].clone();
    }
    leave_block("Compute sum of H and ZK-patch", false);

    leave_block("Call to r1cs_to_sap_witness_map", false);

    return sap_witness::<FieldT>::new(
        sap_num_variables,
        ED::M,
        cs.num_inputs(),
        d1.clone(),
        d2.clone(),
        full_variable_assignment,
        (coefficients_for_H),
    );
}

// //#endif // R1CS_TO_SAP_TCC_
