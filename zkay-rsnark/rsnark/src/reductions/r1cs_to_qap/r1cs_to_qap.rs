/** @file
 *****************************************************************************

 Declaration of interfaces for a R1CS-to-QAP reduction, that is, constructing
 a QAP ("Quadratic Arithmetic Program") from a R1CS ("Rank-1 Constraint System").

 QAPs are defined in \[GGPR13], and constructed for R1CS also in \[GGPR13].

 The implementation of the reduction follows, extends, and optimizes
 the efficient approach described in Appendix E of \[BCGTV13].

 References:

 \[BCGTV13]
 "SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
 Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
 CRYPTO 2013,
 <http://eprint.iacr.org/2013/507>

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

// //#ifndef R1CS_TO_QAP_HPP_
// // #define R1CS_TO_QAP_HPP_

use crate::relations::arithmetic_programs::qap::qap;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



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



use crate::reductions::r1cs_to_qap::r1cs_to_qap;

// //#endif // R1CS_TO_QAP_HPP_


/** @file
 *****************************************************************************

 Implementation of interfaces for a R1CS-to-QAP reduction.

 See r1cs_to_qap.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// //#ifndef R1CS_TO_QAP_TCC_
// // #define R1CS_TO_QAP_TCC_

use ffec::common::profiling;
use ffec::common::utils;
use fqfft::evaluation_domain::get_evaluation_domain;



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
pub fn
 r1cs_to_qap_instance_map(cs:&r1cs_constraint_system<FieldT>)->qap_instance<FieldT>
{
    ffec::enter_block("Call to r1cs_to_qap_instance_map",false);

    let domain = libfqfft::get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() +1);

    let mut A_in_Lagrange_basis=Vec::with_capicity(cs.num_variables()+1);
    let mut B_in_Lagrange_basis=Vec::with_capicity(cs.num_variables()+1);
    let mut C_in_Lagrange_basis=Vec::with_capicity(cs.num_variables()+1);

    ffec::enter_block("Compute polynomials A, B, C in Lagrange basis");
    /**
     * add and process the constraints
     *     input_i * 0 = 0
     * to ensure soundness of input consistency
     */
    for i in 0..=cs.num_inputs()
    {
        A_in_Lagrange_basis[i][cs.num_constraints() + i] = FieldT::one();
    }
    /* process all other constraints */
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].a.terms.size()
        {
            A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index][i] +=
                cs.constraints[i].a.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].b.terms.size()
        {
            B_in_Lagrange_basis[cs.constraints[i].b.terms[j].index][i] +=
                cs.constraints[i].b.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].c.terms.size()
        {
            C_in_Lagrange_basis[cs.constraints[i].c.terms[j].index][i] +=
                cs.constraints[i].c.terms[j].coeff;
        }
    }
    ffec::leave_block("Compute polynomials A, B, C in Lagrange basis");

    ffec::leave_block("Call to r1cs_to_qap_instance_map");

    return qap_instance::<FieldT>::new(domain,
                                cs.num_variables(),
                                domain.m,
                                cs.num_inputs(),
                                A_in_Lagrange_basis,
                                B_in_Lagrange_basis,
                                C_in_Lagrange_basis);
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
pub fn 
 r1cs_to_qap_instance_map_with_evaluation(cs:&r1cs_constraint_system<FieldT>,
                                                                         t:&FieldT)->qap_instance_evaluation<FieldT>
{
    ffec::enter_block("Call to r1cs_to_qap_instance_map_with_evaluation");

    let domain = libfqfft::get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() +1);

    let (At, Bt, Ct, Ht)=(vec![ FieldT::zero();cs.num_variables()+1],vec![ FieldT::zero();cs.num_variables()+1],vec![ FieldT::zero();cs.num_variables()+1],Vec::with_capicity(domain.m+1));

    let  Zt =domain.compute_vanishing_polynomial(t);

    ffec::enter_block("Compute evaluations of A, B, C, H at t");
    let   u =domain.evaluate_all_lagrange_polynomials(t);
    /**
     * add and process the constraints
     *     input_i * 0 = 0
     * to ensure soundness of input consistency
     */
    for i in 0..=cs.num_inputs()
    {
        At[i] = u[cs.num_constraints() + i];
    }
    /* process all other constraints */
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].a.terms.size()
        {
            At[cs.constraints[i].a.terms[j].index] +=
                u[i]*cs.constraints[i].a.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].b.terms.size()
        {
            Bt[cs.constraints[i].b.terms[j].index] +=
                u[i]*cs.constraints[i].b.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].c.terms.size()
        {
            Ct[cs.constraints[i].c.terms[j].index] +=
                u[i]*cs.constraints[i].c.terms[j].coeff;
        }
    }

    let mut  ti = FieldT::one();
    for i in 0..domain.m+1
    {
        Ht.push(ti);
        ti *= t;
    }
    ffec::leave_block("Compute evaluations of A, B, C, H at t");

    ffec::leave_block("Call to r1cs_to_qap_instance_map_with_evaluation");

    return qap_instance_evaluation::<FieldT>::new(domain,
                                           cs.num_variables(),
                                           domain.m,
                                           cs.num_inputs(),
                                           t,
                                           At,
                                           Bt,
                                           Ct,
                                           Ht,
                                           Zt);
}

/**
 * Witness map for the R1CS-to-QAP reduction.
 *
 * The witness map takes zero knowledge into account when d1,d2,d3 are random.
 *
 * More precisely, compute the coefficients
 *     h_0,h_1,...,h_n
 * of the polynomial
 *     H(z) := (A(z)*B(z)-C(z))/Z(z)
 * where
 *   A(z) := A_0(z) + \sum_{k=1}^{m} w_k A_k(z) + d1 * Z(z)
 *   B(z) := B_0(z) + \sum_{k=1}^{m} w_k B_k(z) + d2 * Z(z)
 *   C(z) := C_0(z) + \sum_{k=1}^{m} w_k C_k(z) + d3 * Z(z)
 *   Z(z) := "vanishing polynomial of set S"
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
pub fn 
 r1cs_to_qap_witness_map(cs:&r1cs_constraint_system<FieldT>,
                                            primary_input:&r1cs_primary_input<FieldT>,
                                            auxiliary_input:&r1cs_auxiliary_input<FieldT>,
                                            d1:&FieldT,
                                            d2:&FieldT,
                                            d3:&FieldT)->qap_witness<FieldT>
{
    ffec::enter_block("Call to r1cs_to_qap_witness_map");

    /* sanity check */
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let  domain = libfqfft::get_evaluation_domain::<FieldT>(cs.num_constraints() + cs.num_inputs() +1);

    let mut  full_variable_assignment = primary_input.clone();
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());

    ffec::enter_block("Compute evaluation of polynomials A, B on set S");
    let ( aA , aB)=(vec![FieldT::zero(),domain.m],vec![FieldT::zero(),domain.m]);

    /* account for the additional constraints input_i * 0 = 0 */
    for i in 0..=cs.num_inputs()
    {
        aA[i+cs.num_constraints()] = if i > 0 { full_variable_assignment[i-1]} else {FieldT::one()};
    }
    /* account for all other constraints */
    for i in 0..cs.num_constraints()
    {
        aA[i] += cs.constraints[i].a.evaluate(full_variable_assignment);
        aB[i] += cs.constraints[i].b.evaluate(full_variable_assignment);
    }
    ffec::leave_block("Compute evaluation of polynomials A, B on set S");

    ffec::enter_block("Compute coefficients of polynomial A");
    domain.iFFT(aA);
    ffec::leave_block("Compute coefficients of polynomial A");

    ffec::enter_block("Compute coefficients of polynomial B");
    domain.iFFT(aB);
    ffec::leave_block("Compute coefficients of polynomial B");

    ffec::enter_block("Compute ZK-patch");
    let coefficients_for_H=vec![FieldT::zero();domain.m+1];
// // #ifdef MULTICORE
// //#pragma omp parallel for
// //#endif
    /* add coefficients of the polynomial (d2*A + d1*B - d3) + d1*d2*Z */
    for i in 0..domain.m
    {
        coefficients_for_H[i] = d2*aA[i] + d1*aB[i];
    }
    coefficients_for_H[0] -= d3;
    domain.add_poly_Z(d1*d2, coefficients_for_H);
    ffec::leave_block("Compute ZK-patch");

    ffec::enter_block("Compute evaluation of polynomial A on set T");
    domain.cosetFFT(aA, FieldT::multiplicative_generator);
    ffec::leave_block("Compute evaluation of polynomial A on set T");

    ffec::enter_block("Compute evaluation of polynomial B on set T");
    domain.cosetFFT(aB, FieldT::multiplicative_generator);
    ffec::leave_block("Compute evaluation of polynomial B on set T");

    ffec::enter_block("Compute evaluation of polynomial H on set T");
    let H_tmp = &aA; // can overwrite aA because it is not used later
// // #ifdef MULTICORE
// //#pragma omp parallel for
// //#endif
    for i in 0..domain.m
    {
        H_tmp[i] = aA[i]*aB[i];
    }
    // Vec<FieldT>().swap(aB); // destroy aB

    ffec::enter_block("Compute evaluation of polynomial C on set S");
    let  mut aC=vec![FieldT::zero();domain.m];
    for i in 0..cs.num_constraints()
    {
        aC[i] += cs.constraints[i].c.evaluate(full_variable_assignment);
    }
    ffec::leave_block("Compute evaluation of polynomial C on set S");

    ffec::enter_block("Compute coefficients of polynomial C");
    domain.iFFT(aC);
    ffec::leave_block("Compute coefficients of polynomial C");

    ffec::enter_block("Compute evaluation of polynomial C on set T");
    domain.cosetFFT(aC, FieldT::multiplicative_generator);
    ffec::leave_block("Compute evaluation of polynomial C on set T");

// // #ifdef MULTICORE
// //#pragma omp parallel for
// //#endif
    for i in 0..domain.m
    {
        H_tmp[i] = (H_tmp[i]-aC[i]);
    }

    ffec::enter_block("Divide by Z on set T");
    domain.divide_by_Z_on_coset(H_tmp);
    ffec::leave_block("Divide by Z on set T");

    ffec::leave_block("Compute evaluation of polynomial H on set T");

    ffec::enter_block("Compute coefficients of polynomial H");
    domain.icosetFFT(H_tmp, FieldT::multiplicative_generator);
    ffec::leave_block("Compute coefficients of polynomial H");

    ffec::enter_block("Compute sum of H and ZK-patch");
// // #ifdef MULTICORE
// //#pragma omp parallel for
// //#endif
    for i in 0..domain.m
    {
        coefficients_for_H[i] += H_tmp[i];
    }
    ffec::leave_block("Compute sum of H and ZK-patch");

    ffec::leave_block("Call to r1cs_to_qap_witness_map");

    return qap_witness::<FieldT>::new(cs.num_variables(),
                               domain.m,
                               cs.num_inputs(),
                               d1,
                               d2,
                               d3,
                               full_variable_assignment,
                               (coefficients_for_H));
}



// //#endif // R1CS_TO_QAP_TCC_
