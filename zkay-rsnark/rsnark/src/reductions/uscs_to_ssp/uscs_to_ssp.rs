/** @file
 *****************************************************************************

 Declaration of interfaces for a USCS-to-SSP reduction, that is, constructing
 a SSP ("Square Span Program") from a USCS ("boolean circuit with 2-input gates").

 SSPs are defined in \[DFGK14], and constructed for USCS also in \[DFGK14].

 The implementation of the reduction adapts to \[DFGK14], extends, and optimizes
 the efficient QAP-based approach described in Appendix E of \[BCGTV13].

 References:

 \[BCGTV13]
 "SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
 Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
 CRYPTO 2013,
 <http://eprint.iacr.org/2013/507>

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

//#ifndef USCS_TO_SSP_HPP_
// #define USCS_TO_SSP_HPP_

use crate::relations::arithmetic_programs::ssp::ssp;
use crate::relations::constraint_satisfaction_problems/uscs/uscs;



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



// use libsnark/reductions/uscs_to_ssp/uscs_to_ssp;

//#endif // USCS_TO_SSP_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for a USCS-to-SSP reduction.

 See uscs_to_ssp.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_TO_SSP_TCC_
// #define USCS_TO_SSP_TCC_

use common::profiling;
use common::utils;
use fqfft::evaluation_domain::get_evaluation_domain;



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

pub fn uscs_to_ssp_instance_map<FieldT> (cs:&uscs_constraint_system<FieldT>)->ssp_instance<FieldT> 
{
    enter_block("Call to uscs_to_ssp_instance_map");

    let domain = libfqfft::get_evaluation_domain::<FieldT>(cs.num_constraints());

    enter_block("Compute polynomials V in Lagrange basis");
    let mut V_in_Lagrange_basis=vec![BTreeMap::new();cs.num_variables()+1];
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].terms.len()
        {
            V_in_Lagrange_basis[cs.constraints[i].terms[j].index][i] += cs.constraints[i].terms[j].coeff;
        }
    }
    for i in cs.num_constraints()..domain.m
    {
        V_in_Lagrange_basis[0][i] += FieldT::one();
    }
    leave_block("Compute polynomials V in Lagrange basis");

    leave_block("Call to uscs_to_ssp_instance_map");

    return ssp_instance::<FieldT>::new(domain,
                                cs.num_variables(),
                                domain.m,
                                cs.num_inputs(),
                                (V_in_Lagrange_basis));
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

 pub fn uscs_to_ssp_instance_map_with_evaluation<FieldT>(cs:&uscs_constraint_system<FieldT>,
                                                                         t:&FieldT)->ssp_instance_evaluation<FieldT>
{
    enter_block("Call to uscs_to_ssp_instance_map_with_evaluation");

    let  domain = libfqfft::get_evaluation_domain<FieldT>(cs.num_constraints());

    let mut  Vt=vec![FieldT::default();cs.num_variables()+1, FieldT::zero()];
    let mut  Ht=vec![FieldT::default();domain.m+1];

    let Zt= domain.compute_vanishing_polynomial(t);

    enter_block("Compute evaluations of V and H at t");
    let  u = domain.evaluate_all_lagrange_polynomials(t);
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].terms.len()
        {
            Vt[cs.constraints[i].terms[j].index] += u[i]*cs.constraints[i].terms[j].coeff;
        }
    }
    for i in cs.num_constraints()..domain.m
    {
        Vt[0] += u[i]; /* dummy constraint: 1^2 = 1 */
    }
    let mut  ti = FieldT::one();
    for i in 0..domain.m+1
    {
        Ht[i] = ti;
        ti *= t;
    }
    leave_block("Compute evaluations of V and H at t");

    leave_block("Call to uscs_to_ssp_instance_map_with_evaluation");

    return ssp_instance_evaluation::<FieldT>::new(domain,
                                           cs.num_variables(),
                                           domain.m,
                                           cs.num_inputs(),
                                           t,
                                           Vt,
                                           Ht,
                                           Zt);
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

 pub fn uscs_to_ssp_witness_map<FieldT>(cs:&uscs_constraint_system<FieldT>,
                                            primary_input:&uscs_primary_input<FieldT>,
                                            auxiliary_input:&uscs_auxiliary_input<FieldT>,
                                            d:&FieldT)->ssp_witness<FieldT>
{
    enter_block("Call to uscs_to_ssp_witness_map");

    /* sanity check */

    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let mut  full_variable_assignment = primary_input;
    full_variable_assignment.extend(auxiliary_input);

    let  domain = libfqfft::get_evaluation_domain<FieldT>(cs.num_constraints());

    enter_block("Compute evaluation of polynomial V on set S");
    let mut aA=vec![FieldT::zero();domain.m];
    assert!(domain.m >= cs.num_constraints());
    for i in 0..cs.num_constraints()
    {
        aA[i] += cs.constraints[i].evaluate(full_variable_assignment);
    }
    for i in cs.num_constraints()..domain.m
    {
        aA[i] += FieldT::one();
    }
    leave_block("Compute evaluation of polynomial V on set S");

    enter_block("Compute coefficients of polynomial V");
    domain.iFFT(aA);
    leave_block("Compute coefficients of polynomial V");

    enter_block("Compute ZK-patch");
    let mut  coefficients_for_H=vec![FieldT::zero();domain.m+1];
// #ifdef MULTICORE
//#pragma omp parallel for
//#endif
    /* add coefficients of the polynomial 2*d*V(z) + d*d*Z(z) */
    for i in 0..domain.m
    {
        coefficients_for_H[i] = FieldT(2)*d*aA[i];
    }
    domain.add_poly_Z(d.squared(), coefficients_for_H);
    leave_block("Compute ZK-patch");

    enter_block("Compute evaluation of polynomial V on set T");
    domain.cosetFFT(aA, FieldT::multiplicative_generator);
    leave_block("Compute evaluation of polynomial V on set T");

    enter_block("Compute evaluation of polynomial H on set T");
    let  mut H_tmp = aA; // can overwrite aA because it is not used later
// #ifdef MULTICORE
//#pragma omp parallel for
//#endif
    for i in 0..domain.m
    {
        H_tmp[i] = aA[i].squared()-FieldT::one();
    }

    enter_block("Divide by Z on set T");
    domain.divide_by_Z_on_coset(H_tmp);
    leave_block("Divide by Z on set T");

    leave_block("Compute evaluation of polynomial H on set T");

    enter_block("Compute coefficients of polynomial H");
    domain.icosetFFT(H_tmp, FieldT::multiplicative_generator);
    leave_block("Compute coefficients of polynomial H");

    enter_block("Compute sum of H and ZK-patch");
// #ifdef MULTICORE
//#pragma omp parallel for
//#endif
    for i in 0..domain.m
    {
        coefficients_for_H[i] += H_tmp[i];
    }
    leave_block("Compute sum of H and ZK-patch");

    leave_block("Call to uscs_to_ssp_witness_map");

    return ssp_witness::<FieldT>::new(cs.num_variables(),
                               domain.m,
                               cs.num_inputs(),
                               d,
                               full_variable_assignment,
                               coefficients_for_H);
}



//#endif // USCS_TO_SSP_TCC_
