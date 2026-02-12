// Declaration of interfaces for a R1CS-to-QAP reduction, that is, constructing
// a QAP ("Quadratic Arithmetic Program") from a R1CS ("Rank-1 Constraint System").

// QAPs are defined in \[GGPR13], and constructed for R1CS also in \[GGPR13].

// The implementation of the reduction follows, extends, and optimizes
// the efficient approach described in Appendix E of \[BCGTV13].

// References:

// \[BCGTV13]
// "SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
// Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
// CRYPTO 2013,
// <http://eprint.iacr.org/2013/507>

// \[GGPR13]:
// "Quadratic span programs and succinct NIZKs without PCPs",
// Rosario Gennaro, Craig Gentry, Bryan Parno, Mariana Raykova,
// EUROCRYPT 2013,
// <http://eprint.iacr.org/2012/215>

use crate::relations::arithmetic_programs::qap::qap::{
    qap_instance, qap_instance_evaluation, qap_witness,
};
use crate::relations::circuit_satisfaction_problems::bacs::bacs::{
    bacs_auxiliary_input, bacs_circuit, bacs_primary_input,
};
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
use fqfft::evaluation_domain::{
    evaluation_domain::{EvaluationDomainConfig, evaluation_domain},
    get_evaluation_domain::get_evaluation_domain,
};
/**
 * Instance map for the R1CS-to-QAP reduction.
 */
// pub fn
// qap_instance<FieldT> r1cs_to_qap_instance_map(&cs:r1cs_constraint_system<FieldT>);

/**
 * Instance map for the R1CS-to-QAP reduction followed by evaluation of the resulting QAP instance.
 */
// pub fn
// qap_instance_evaluation<FieldT> r1cs_to_qap_instance_map_with_evaluation(cs:&r1cs_constraint_system<FieldT>
//                                                                          &t:FieldT);

/**
 * Witness map for the R1CS-to-QAP reduction.
 *
 * The witness map takes zero knowledge into account when d1,d2,d3 are random.
 */
// pub fn
// qap_witness<FieldT> r1cs_to_qap_witness_map(cs:&r1cs_constraint_system<FieldT>
//                                             primary_input:&r1cs_primary_input<FieldT>
//                                             auxiliary_input:&r1cs_auxiliary_input<FieldT>
//                                             d1:&FieldT
//                                             d2:&FieldT
//                                             &d3:FieldT);

// use crate::reductions::r1cs_to_qap::r1cs_to_qap;
use std::collections::HashMap;

/**
 * Instance map for the R1CS-to-QAP reduction.
 *
 * Namely, given a R1CS constraint system cs, construct a QAP instance for which:
 *   A := (A_0(z),A_1(z),...,A_m(z))
 *   B := (B_0(z),B_1(z),...,B_m(z))
 *   C := (C_0(z),C_1(z),...,C_m(z))
 * where
 *   m = number of variables of the QAP
 * and
 *   each A_i,B_i,C_i is expressed in the Lagrange basis.
 */
pub fn r1cs_to_qap_instance_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
) -> qap_instance<FieldT> {
    enter_block("Call to r1cs_to_qap_instance_map", false);

    let domain =
        get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() + 1).unwrap();

    let mut A_in_Lagrange_basis =
        Vec::<HashMap<usize, FieldT>>::with_capacity(cs.num_variables() + 1);
    let mut B_in_Lagrange_basis =
        Vec::<HashMap<usize, FieldT>>::with_capacity(cs.num_variables() + 1);
    let mut C_in_Lagrange_basis =
        Vec::<HashMap<usize, FieldT>>::with_capacity(cs.num_variables() + 1);

    enter_block("Compute polynomials A, B, C in Lagrange basis", false);
    /**
     * add and process the constraints
     *     input_i * 0 = 0
     * to ensure soundness of input consistency
     */
    for i in 0..=cs.num_inputs() {
        A_in_Lagrange_basis[i].insert(cs.num_constraints() + i, FieldT::one());
    }
    /* process all other constraints */
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].a.terms.len() {
            *A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index.index]
                .entry(i)
                .or_insert(FieldT::zero()) += cs.constraints[i].a.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].b.terms.len() {
            *B_in_Lagrange_basis[cs.constraints[i].b.terms[j].index.index]
                .entry(i)
                .or_insert(FieldT::zero()) += cs.constraints[i].b.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].c.terms.len() {
            *C_in_Lagrange_basis[cs.constraints[i].c.terms[j].index.index]
                .entry(i)
                .or_insert(FieldT::zero()) += cs.constraints[i].c.terms[j].coeff.clone();
        }
    }
    leave_block("Compute polynomials A, B, C in Lagrange basis", false);

    leave_block("Call to r1cs_to_qap_instance_map", false);

    qap_instance::<FieldT>::new(
        domain.clone(),
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        A_in_Lagrange_basis,
        B_in_Lagrange_basis,
        C_in_Lagrange_basis,
    )
}

/**
 * Instance map for the R1CS-to-QAP reduction followed by evaluation of the resulting QAP instance.
 *
 * Namely, given a R1CS constraint system cs and a field element t, construct
 * a QAP instance (evaluated at t) for which:
 *   At := (A_0(t),A_1(t),...,A_m(t))
 *   Bt := (B_0(t),B_1(t),...,B_m(t))
 *   Ct := (C_0(t),C_1(t),...,C_m(t))
 *   Ht := (1,t,t^2,...,t^n)
 *   Zt := Z(t) = "vanishing polynomial of a certain set S, evaluated at t"
 * where
 *   m = number of variables of the QAP
 *   n = degree of the QAP
 */
pub fn r1cs_to_qap_instance_map_with_evaluation<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    t: &FieldT,
) -> qap_instance_evaluation<FieldT> {
    enter_block("Call to r1cs_to_qap_instance_map_with_evaluation", false);

    let domain =
        get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() + 1).unwrap();

    let (mut At, mut Bt, mut Ct, mut Ht) = (
        vec![FieldT::zero(); cs.num_variables() + 1],
        vec![FieldT::zero(); cs.num_variables() + 1],
        vec![FieldT::zero(); cs.num_variables() + 1],
        Vec::with_capacity(domain.borrow().m() + 1),
    );

    let Zt = domain.borrow_mut().compute_vanishing_polynomial(t);

    enter_block("Compute evaluations of A, B, C, H at t", false);
    let u = domain.borrow_mut().evaluate_all_lagrange_polynomials(t);
    /**
     * add and process the constraints
     *     input_i * 0 = 0
     * to ensure soundness of input consistency
     */
    for i in 0..=cs.num_inputs() {
        At[i] = u[cs.num_constraints() + i].clone();
    }
    /* process all other constraints */
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].a.terms.len() {
            At[cs.constraints[i].a.terms[j].index.index] +=
                u[i].clone() * cs.constraints[i].a.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].b.terms.len() {
            Bt[cs.constraints[i].b.terms[j].index.index] +=
                u[i].clone() * cs.constraints[i].b.terms[j].coeff.clone();
        }

        for j in 0..cs.constraints[i].c.terms.len() {
            Ct[cs.constraints[i].c.terms[j].index.index] +=
                u[i].clone() * cs.constraints[i].c.terms[j].coeff.clone();
        }
    }

    let mut ti = FieldT::one();
    for i in 0..domain.borrow().m() + 1 {
        Ht.push(ti.clone());
        ti *= t.clone();
    }
    leave_block("Compute evaluations of A, B, C, H at t", false);

    leave_block("Call to r1cs_to_qap_instance_map_with_evaluation", false);

    qap_instance_evaluation::<FieldT>::new(
        domain.clone(),
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        t.clone(),
        At,
        Bt,
        Ct,
        Ht,
        Zt,
    )
}

/**
 * Witness map for the R1CS-to-QAP reduction.
 *
 * The witness map takes zero knowledge into account when d1,d2,d3 are random.
 *
 * More precisely, compute the coefficients
 *     h_0,h_1,...,h_n
 * of the polynomial
 *     H(z)->Self= (A(z)*B(z)-C(z))/Z(z)
 * where
 *   A(z)->Self= A_0(z) + \sum_{k=1}^{m} w_k A_k(z) + d1 * Z(z)
 *   B(z)->Self= B_0(z) + \sum_{k=1}^{m} w_k B_k(z) + d2 * Z(z)
 *   C(z)->Self= C_0(z) + \sum_{k=1}^{m} w_k C_k(z) + d3 * Z(z)
 *   Z(z)->Self= "vanishing polynomial of set S"
 * and
 *   m = number of variables of the QAP
 *   n = degree of the QAP
 *
 * This is done as follows:
 *  (1) compute evaluations of A,B,C on S = {sigma_1,...,sigma_n}
 *  (2) compute coefficients of A,B,C
 *  (3) compute evaluations of A,B,C on T = "coset of S"
 *  (4) compute evaluation of H on T
 *  (5) compute coefficients of H
 *  (6) patch H to account for d1,d2,d3 (i.e., add coefficients of the polynomial (A d2 + B d1 - d3) + d1*d2*Z )
 *
 * The code below is not as simple as the above high-level description due to
 * some reshuffling to save space.
 */
pub fn r1cs_to_qap_witness_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    primary_input: &r1cs_primary_input<FieldT>,
    auxiliary_input: &r1cs_auxiliary_input<FieldT>,
    d1: &FieldT,
    d2: &FieldT,
    d3: &FieldT,
) -> qap_witness<FieldT> {
    enter_block("Call to r1cs_to_qap_witness_map", false);

    /* sanity check */
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let domain =
        get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() + 1).unwrap();

    let mut full_variable_assignment: Vec<_> = primary_input
        .iter()
        .chain(auxiliary_input)
        .cloned()
        .collect();

    enter_block("Compute evaluation of polynomials A, B on set S", false);
    let (mut aA, mut aB) = (
        vec![FieldT::zero(); domain.borrow().m()],
        vec![FieldT::zero(); domain.borrow().m()],
    );

    /* account for the additional constraints input_i * 0 = 0 */
    for i in 0..=cs.num_inputs() {
        aA[i + cs.num_constraints()] = if i > 0 {
            full_variable_assignment[i - 1].clone()
        } else {
            FieldT::one()
        };
    }
    /* account for all other constraints */
    for i in 0..cs.num_constraints() {
        aA[i] += cs.constraints[i].a.evaluate(&full_variable_assignment);
        aB[i] += cs.constraints[i].b.evaluate(&full_variable_assignment);
    }
    leave_block("Compute evaluation of polynomials A, B on set S", false);

    enter_block("Compute coefficients of polynomial A", false);
    domain.borrow_mut().iFFT(&mut aA);
    leave_block("Compute coefficients of polynomial A", false);

    enter_block("Compute coefficients of polynomial B", false);
    domain.borrow_mut().iFFT(&mut aB);
    leave_block("Compute coefficients of polynomial B", false);

    enter_block("Compute ZK-patch", false);
    let mut coefficients_for_H = vec![FieldT::zero(); domain.borrow().m() + 1];
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    //
    /* add coefficients of the polynomial (d2*A + d1*B - d3) + d1*d2*Z */
    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] = d2.clone() * aA[i].clone() + d1.clone() * aB[i].clone();
    }
    coefficients_for_H[0] -= d3.clone();
    domain
        .borrow_mut()
        .add_poly_Z(&(d1.clone() * d2.clone()), &mut coefficients_for_H);
    leave_block("Compute ZK-patch", false);

    enter_block("Compute evaluation of polynomial A on set T", false);
    domain
        .borrow_mut()
        .cosetFFT(&mut aA, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial A on set T", false);

    enter_block("Compute evaluation of polynomial B on set T", false);
    domain
        .borrow_mut()
        .cosetFFT(&mut aB, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial B on set T", false);

    enter_block("Compute evaluation of polynomial H on set T", false);
    let mut H_tmp = aA.clone(); // can overwrite aA because it is not used later
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    //
    for i in 0..domain.borrow().m() {
        H_tmp[i] = aA[i].clone() * aB[i].clone();
    }
    // Vec<FieldT: FieldTConfig>().swap(aB); // destroy aB

    enter_block("Compute evaluation of polynomial C on set S", false);
    let mut aC = vec![FieldT::zero(); domain.borrow().m()];
    for i in 0..cs.num_constraints() {
        aC[i] += cs.constraints[i].c.evaluate(&full_variable_assignment);
    }
    leave_block("Compute evaluation of polynomial C on set S", false);

    enter_block("Compute coefficients of polynomial C", false);
    domain.borrow_mut().iFFT(&mut aC);
    leave_block("Compute coefficients of polynomial C", false);

    enter_block("Compute evaluation of polynomial C on set T", false);
    domain
        .borrow_mut()
        .cosetFFT(&mut aC, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial C on set T", false);

    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    //
    for i in 0..domain.borrow().m() {
        H_tmp[i] = (H_tmp[i].clone() - aC[i].clone());
    }

    enter_block("Divide by Z on set T", false);
    domain.borrow().divide_by_Z_on_coset(&mut H_tmp);
    leave_block("Divide by Z on set T", false);

    leave_block("Compute evaluation of polynomial H on set T", false);

    enter_block("Compute coefficients of polynomial H", false);
    domain
        .borrow_mut()
        .icosetFFT(&mut H_tmp, &FieldT::multiplicative_generator());
    leave_block("Compute coefficients of polynomial H", false);

    enter_block("Compute sum of H and ZK-patch", false);
    // // #ifdef MULTICORE
    // //#pragma omp parallel for
    //
    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] += H_tmp[i].clone();
    }
    leave_block("Compute sum of H and ZK-patch", false);

    leave_block("Call to r1cs_to_qap_witness_map", false);

    return qap_witness::<FieldT>::new(
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        d1.clone(),
        d2.clone(),
        d3.clone(),
        full_variable_assignment,
        (coefficients_for_H),
    );
}

//
