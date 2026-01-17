// Declaration of interfaces for a USCS-to-SSP reduction, that is, constructing
// a SSP ("Square Span Program") from a USCS ("boolean circuit with 2-input gates").

// SSPs are defined in \[DFGK14], and constructed for USCS also in \[DFGK14].

// The implementation of the reduction adapts to \[DFGK14], extends, and optimizes
// the efficient QAP-based approach described in Appendix E of \[BCGTV13].

// References:

// \[BCGTV13]
// "SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
// Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
// CRYPTO 2013,
// <http://eprint.iacr.org/2013/507>

// \[DFGK14]:
// "Square Span Programs with Applications to Succinct NIZK Arguments"
// George Danezis, Cedric Fournet, Jens Groth, Markulf Kohlweiss,
// ASIACRYPT 2014,
// <http://eprint.iacr.org/2014/718>

use crate::relations::arithmetic_programs::ssp::ssp::{
    ssp_instance, ssp_instance_evaluation, ssp_witness,
};
use crate::relations::constraint_satisfaction_problems::uscs::uscs::{
    uscs_auxiliary_input, uscs_constraint_system, uscs_primary_input,
};
use crate::relations::variable::{SubLinearCombinationConfig, SubVariableConfig};
use ffec::FieldTConfig;
use ffec::common::profiling::{enter_block, leave_block};
use ffec::common::utils;
use fqfft::evaluation_domain::{
    evaluation_domain::{EvaluationDomainConfig, evaluation_domain},
    get_evaluation_domain::get_evaluation_domain,
};
use std::collections::BTreeMap;
// /**
//  * Instance map for the USCS-to-SSP reduction.
//  */
// ssp_instance<FieldT> uscs_to_ssp_instance_map(cs:&uscs_constraint_system<FieldT>);

// /**
//  * Instance map for the USCS-to-SSP reduction followed by evaluation of the resulting SSP instance.
//  */
// ssp_instance_evaluation<FieldT> uscs_to_ssp_instance_map_with_evaluation(cs:&uscs_constraint_system<FieldT>,
//                                                                          t:&FieldT);

// /**
//  * Witness map for the USCS-to-SSP reduction.
//  *
//  * The witness map takes zero knowledge into account when d is random.
//  */
// ssp_witness<FieldT> uscs_to_ssp_witness_map(cs:&uscs_constraint_system<FieldT>,
//                                             primary_input:&uscs_primary_input<FieldT>,
//                                             auxiliary_input:&uscs_auxiliary_input<FieldT>,
//                                             d:&FieldT);

/**
 * Instance map for the USCS-to-SSP reduction.
 *
 * Namely, given a USCS constraint system cs, construct a SSP instance for which:
 *   V := (V_0(z),V_1(z),...,V_m(z))
 * where
 *   m = number of variables of the SSP
 * and
 *   each V_i is expressed in the Lagrange basis.
 */

pub fn uscs_to_ssp_instance_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &uscs_constraint_system<FieldT, SV, SLC>,
) -> ssp_instance<FieldT> {
    enter_block("Call to uscs_to_ssp_instance_map", false);

    let domain = get_evaluation_domain::<FieldT>(cs.num_constraints()).unwrap();

    enter_block("Compute polynomials V in Lagrange basis", false);
    let mut V_in_Lagrange_basis = vec![BTreeMap::new(); cs.num_variables() + 1];
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].terms.len() {
            *V_in_Lagrange_basis[cs.constraints[i].terms[j].index.index]
                .entry(i)
                .or_insert(FieldT::zero()) += cs.constraints[i].terms[j].coeff.clone();
        }
    }
    for i in cs.num_constraints()..domain.borrow().m() {
        *V_in_Lagrange_basis[0].entry(i).or_insert(FieldT::zero()) += FieldT::one();
    }
    leave_block("Compute polynomials V in Lagrange basis", false);

    leave_block("Call to uscs_to_ssp_instance_map", false);

    ssp_instance::<FieldT>::new(
        domain.clone(),
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        (V_in_Lagrange_basis),
    )
}

/**
 * Instance map for the USCS-to-SSP reduction followed by evaluation of the resulting SSP instance.
 *
 * Namely, given a USCS constraint system cs and a field element t, construct
 * a SSP instance (evaluated at t) for which:
 *   Vt := (V_0(t),V_1(t),...,V_m(t))
 *   Ht := (1,t,t^2,...,t^n)
 *   Zt := Z(t) = "vanishing polynomial of a certain set S, evaluated at t"
 * where
 *   m = number of variables of the SSP
 *   n = degree of the SSP
 */

pub fn uscs_to_ssp_instance_map_with_evaluation<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &uscs_constraint_system<FieldT, SV, SLC>,
    t: &FieldT,
) -> ssp_instance_evaluation<FieldT> {
    enter_block("Call to uscs_to_ssp_instance_map_with_evaluation", false);

    let domain = get_evaluation_domain::<FieldT>(cs.num_constraints()).unwrap();

    let mut Vt = vec![FieldT::zero(); cs.num_variables() + 1];
    let mut Ht = vec![FieldT::zero(); domain.borrow().m() + 1];

    let Zt = domain.borrow_mut().compute_vanishing_polynomial(t);

    enter_block("Compute evaluations of V and H at t", false);
    let u = domain.borrow_mut().evaluate_all_lagrange_polynomials(t);
    for i in 0..cs.num_constraints() {
        for j in 0..cs.constraints[i].terms.len() {
            Vt[cs.constraints[i].terms[j].index.index] +=
                u[i].clone() * cs.constraints[i].terms[j].coeff.clone();
        }
    }
    for i in cs.num_constraints()..domain.borrow().m() {
        Vt[0] += u[i].clone(); /* dummy constraint: 1^2 = 1 */
    }
    let mut ti = FieldT::one();
    for i in 0..domain.borrow().m() + 1 {
        Ht[i] = ti.clone();
        ti *= t.clone();
    }
    leave_block("Compute evaluations of V and H at t", false);

    leave_block("Call to uscs_to_ssp_instance_map_with_evaluation", false);

    ssp_instance_evaluation::<FieldT>::new(
        domain.clone(),
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        t.clone(),
        Vt,
        Ht,
        Zt,
    )
}

/**
 * Witness map for the USCS-to-SSP reduction.
 *
 * The witness map takes zero knowledge into account when d is random.
 *
 * More precisely, compute the coefficients
 *     h_0,h_1,...,h_n
 * of the polynomial
 *     H(z)->Self= (V(z)^2-1)/Z(z)
 * where
 *   V(z)->Self= V_0(z) + \sum_{k=1}^{m} w_k V_k(z) + d * Z(z)
 *   Z(z)->Self= "vanishing polynomial of set S"
 * and
 *   m = number of variables of the SSP
 *   n = degree of the SSP
 *
 * This is done as follows:
 *  (1) compute evaluations of V on S = {sigma_1,...,sigma_n}
 *  (2) compute coefficients of V
 *  (3) compute evaluations of V on T = "coset of S"
 *  (4) compute evaluation of H on T
 *  (5) compute coefficients of H
 *  (6) patch H to account for d (i.e., add coefficients of the polynomial 2*d*V(z) + d*d*Z(z) )
 *
 * The code below is not as simple as the above high-level description due to
 * some reshuffling to save space.
 */

pub fn uscs_to_ssp_witness_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &uscs_constraint_system<FieldT, SV, SLC>,
    primary_input: &uscs_primary_input<FieldT>,
    auxiliary_input: &uscs_auxiliary_input<FieldT>,
    d: &FieldT,
) -> ssp_witness<FieldT> {
    enter_block("Call to uscs_to_ssp_witness_map", false);

    /* sanity check */

    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let mut full_variable_assignment: Vec<_> = primary_input
        .iter()
        .chain(auxiliary_input)
        .cloned()
        .collect();

    let domain = get_evaluation_domain::<FieldT>(cs.num_constraints()).unwrap();

    enter_block("Compute evaluation of polynomial V on set S", false);
    let mut aA = vec![FieldT::zero(); domain.borrow().m()];
    assert!(domain.borrow().m() >= cs.num_constraints());
    for i in 0..cs.num_constraints() {
        aA[i] += cs.constraints[i].evaluate(&full_variable_assignment);
    }
    for i in cs.num_constraints()..domain.borrow().m() {
        aA[i] += FieldT::one();
    }
    leave_block("Compute evaluation of polynomial V on set S", false);

    enter_block("Compute coefficients of polynomial V", false);
    domain.borrow_mut().iFFT(&mut aA);
    leave_block("Compute coefficients of polynomial V", false);

    enter_block("Compute ZK-patch", false);
    let mut coefficients_for_H = vec![FieldT::zero(); domain.borrow().m() + 1];
    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    /* add coefficients of the polynomial 2*d*V(z) + d*d*Z(z) */
    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] = FieldT::from(2i64) * d.clone() * aA[i].clone();
    }
    domain
        .borrow_mut()
        .add_poly_Z(&d.squared(), &mut coefficients_for_H);
    leave_block("Compute ZK-patch", false);

    enter_block("Compute evaluation of polynomial V on set T", false);
    domain
        .borrow_mut()
        .cosetFFT(&mut aA, &FieldT::multiplicative_generator());
    leave_block("Compute evaluation of polynomial V on set T", false);

    enter_block("Compute evaluation of polynomial H on set T", false);
    let mut H_tmp = aA.clone(); // can overwrite aA because it is not used later
    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    for i in 0..domain.borrow().m() {
        H_tmp[i] = aA[i].squared() - FieldT::one();
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
    // #ifdef MULTICORE
    //#pragma omp parallel for
    //#endif
    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] += H_tmp[i].clone();
    }
    leave_block("Compute sum of H and ZK-patch", false);

    leave_block("Call to uscs_to_ssp_witness_map", false);

    return ssp_witness::<FieldT>::new(
        cs.num_variables(),
        domain.borrow().m(),
        cs.num_inputs(),
        d.clone(),
        full_variable_assignment,
        coefficients_for_H,
    );
}
