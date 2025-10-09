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

use libsnark::relations::arithmetic_programs::sap::sap;
use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



/**
 * Helper function to find evaluation domain that will be used by the reduction
 * for a given R1CS instance.
 */
// template<typename FieldT>
// std::shared_ptr<libfqfft::evaluation_domain<FieldT> > r1cs_to_sap_get_domain(&cs:r1cs_constraint_system<FieldT>);

/**
 * Instance map for the R1CS-to-QAP reduction.
 */
// template<typename FieldT>
// sap_instance<FieldT> r1cs_to_sap_instance_map(&cs:r1cs_constraint_system<FieldT>);

/**
 * Instance map for the R1CS-to-QAP reduction followed by evaluation of the resulting QAP instance.
 */
// template<typename FieldT>
// sap_instance_evaluation<FieldT> r1cs_to_sap_instance_map_with_evaluation(cs:&r1cs_constraint_system<FieldT>
//                                                                          &t:FieldT);

/**
 * Witness map for the R1CS-to-QAP reduction.
 *
 * The witness map takes zero knowledge into account when d1,d2 are random.
 */
// template<typename FieldT>
// sap_witness<FieldT> r1cs_to_sap_witness_map(cs:&r1cs_constraint_system<FieldT>
//                                             primary_input:&r1cs_primary_input<FieldT>
//                                             auxiliary_input:&r1cs_auxiliary_input<FieldT>
//                                             d1:&FieldT
//                                             &d2:FieldT);



// use libsnark::reductions::r1cs_to_sap::r1cs_to_sap;

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

use ffec::common::profiling;
use ffec::common::utils;
use fqfft::evaluation_domain::get_evaluation_domain;



/**
 * Helper function to multiply a field element by 4 efficiently
 */

 pub fn times_four< FieldT>( x:FieldT)->FieldT
{
    let times_two = x + x;
    return times_two + times_two;
}

/**
 * Helper function to find evaluation domain that will be used by the reduction
 * for a given R1CS instance.
 */

pub fn r1cs_to_sap_get_domain< FieldT>(&cs:r1cs_constraint_system<FieldT>)->std::shared_ptr<libfqfft::evaluation_domain<FieldT> > 
{
    /*
     * the SAP instance will have:
     * - two constraints for every constraint in the original constraint system
     * - two constraints for every public input, except the 0th, which
     *   contributes just one extra constraint
     * see comments in r1cs_to_sap_instance_map for details on where these
     * constraints come from.
     */
    return libfqfft::get_evaluation_domain::<FieldT>(2 * cs.num_constraints() + 2 * cs.num_inputs() + 1);
}

/**
 * Instance map for the R1CS-to-SAP reduction.
 */
pub fn 
r1cs_to_sap_instance_map< FieldT>(&cs:r1cs_constraint_system<FieldT>)->sap_instance<FieldT> 
{
    ffec::enter_block("Call to r1cs_to_sap_instance_map");

    let  domain =
        r1cs_to_sap_get_domain(cs);

    let  sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut A_in_Lagrange_basis=Vec::with_capicity(sap_num_variables + 1);
    let mut C_in_Lagrange_basis=Vec::with_capicity(sap_num_variables + 1);

    ffec::enter_block("Compute polynomials A, C in Lagrange basis");
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
    let  extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].a.terms.size()
        {
            A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index][2 * i] +=
                cs.constraints[i].a.terms[j].coeff;
            A_in_Lagrange_basis[cs.constraints[i].a.terms[j].index][2 * i + 1] +=
                cs.constraints[i].a.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].b.terms.size()
        {
            A_in_Lagrange_basis[cs.constraints[i].b.terms[j].index][2 * i] +=
                cs.constraints[i].b.terms[j].coeff;
            A_in_Lagrange_basis[cs.constraints[i].b.terms[j].index][2 * i + 1] -=
                cs.constraints[i].b.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].c.terms.size()
        {
            C_in_Lagrange_basis[cs.constraints[i].c.terms[j].index][2 * i] +=
                  times_four(cs.constraints[i].c.terms[j].coeff);
        }

        C_in_Lagrange_basis[extra_var_offset + i][2 * i] += FieldT::one();
        C_in_Lagrange_basis[extra_var_offset + i][2 * i + 1] += FieldT::one();
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

    let  extra_constr_offset = 2 * cs.num_constraints();
    let extra_var_offset2 = cs.num_variables() + cs.num_constraints();
    
    //   NB: extra variables start at (extra_var_offset2 + 1), because i starts at
        // 1 below
    

    A_in_Lagrange_basis[0][extra_constr_offset] = FieldT::one();
    C_in_Lagrange_basis[0][extra_constr_offset] = FieldT::one();

    for i in 1..=cs.num_inputs()
    {
        A_in_Lagrange_basis[i][extra_constr_offset + 2 * i - 1] += FieldT::one();
        A_in_Lagrange_basis[0][extra_constr_offset + 2 * i - 1] += FieldT::one();
        C_in_Lagrange_basis[i][extra_constr_offset + 2 * i - 1] +=
            times_four(FieldT::one());
        C_in_Lagrange_basis[extra_var_offset2 + i][extra_constr_offset + 2 * i - 1] += FieldT::one();

        A_in_Lagrange_basis[i][extra_constr_offset + 2 * i] += FieldT::one();
        A_in_Lagrange_basis[0][extra_constr_offset + 2 * i] -= FieldT::one();
        C_in_Lagrange_basis[extra_var_offset2 + i][2 * cs.num_constraints() + 2 * i] += FieldT::one();
    }

    ffec::leave_block("Compute polynomials A, C in Lagrange basis");

    ffec::leave_block("Call to r1cs_to_sap_instance_map");

    return sap_instance::<FieldT>::new(domain,
                                sap_num_variables,
                                domain.m,
                                cs.num_inputs(),
                                A_in_Lagrange_basis,
                                 C_in_Lagrange_basis);
}

/**
 * Instance map for the R1CS-to-SAP reduction followed by evaluation
 * of the resulting QAP instance.
 */
pub fn 
 r1cs_to_sap_instance_map_with_evaluation< FieldT>(cs:&r1cs_constraint_system<FieldT>,
                                                                         &t:FieldT)->sap_instance_evaluation<FieldT>
{
    ffec::enter_block("Call to r1cs_to_sap_instance_map_with_evaluation");

    let  domain =
        r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let  (mut At, mut Ct,mut Ht)=(vec![FieldT::zero();sap_num_variables + 1],vec![FieldT::zero();sap_num_variables + 1],Vec::with_capicity(domain.m+1));

    let Zt =domain.compute_vanishing_polynomial(t);

    ffec::enter_block("Compute evaluations of A, C, H at t");
    let u =domain.evaluate_all_lagrange_polynomials(t);
    /**
     * add and process all constraints as in r1cs_to_sap_instance_map
     */
    let extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints()
    {
        for j in 0..cs.constraints[i].a.terms.size()
        {
            At[cs.constraints[i].a.terms[j].index] +=
                u[2 * i] * cs.constraints[i].a.terms[j].coeff;
            At[cs.constraints[i].a.terms[j].index] +=
                u[2 * i + 1] * cs.constraints[i].a.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].b.terms.size()
        {
            At[cs.constraints[i].b.terms[j].index] +=
                u[2 * i] * cs.constraints[i].b.terms[j].coeff;
            At[cs.constraints[i].b.terms[j].index] -=
                u[2 * i + 1] * cs.constraints[i].b.terms[j].coeff;
        }

        for j in 0..cs.constraints[i].c.terms.size()
        {
            Ct[cs.constraints[i].c.terms[j].index] +=
                times_four(u[2 * i] * cs.constraints[i].c.terms[j].coeff);
        }

        Ct[extra_var_offset + i] += u[2 * i];
        Ct[extra_var_offset + i] += u[2 * i + 1];
    }

    let extra_constr_offset = 2 * cs.num_constraints();
    let extra_var_offset2 = cs.num_variables() + cs.num_constraints();

    At[0] += u[extra_constr_offset];
    Ct[0] += u[extra_constr_offset];

    for i in 1..=cs.num_inputs()
    {
        At[i] += u[extra_constr_offset + 2 * i - 1];
        At[0] += u[extra_constr_offset + 2 * i - 1];
        Ct[i] += times_four(u[extra_constr_offset + 2 * i - 1]);
        Ct[extra_var_offset2 + i] += u[extra_constr_offset + 2 * i - 1];

        At[i] += u[extra_constr_offset + 2 * i];
        At[0] -= u[extra_constr_offset + 2 * i];
        Ct[extra_var_offset2 + i] += u[extra_constr_offset + 2 * i];
    }

    let ti = FieldT::one();
    for i in 0..domain.m+1
    {
        Ht.push(ti);
        ti *= t;
    }
    ffec::leave_block("Compute evaluations of A, C, H at t");

    ffec::leave_block("Call to r1cs_to_sap_instance_map_with_evaluation");

    return sap_instance_evaluation::<FieldT>(domain,
                                           sap_num_variables,
                                           domain.m,
                                           cs.num_inputs(),
                                           t,
                                           (At),
                                           (Ct),
                                           (Ht),
                                           Zt);
}

/**
 * Witness map for the R1CS-to-SAP reduction.
 *
 * The witness map takes zero knowledge into account when d1, d2 are random.
 *
 * More precisely, compute the coefficients
 *     h_0,h_1,...,h_n
 * of the polynomial
 *     H(z) := (A(z)*A(z)-C(z))/Z(z)
 * where
 *   A(z) := A_0(z) + \sum_{k=1}^{m} w_k A_k(z) + d1 * Z(z)
 *   C(z) := C_0(z) + \sum_{k=1}^{m} w_k C_k(z) + d2 * Z(z)
 *   Z(z) := "vanishing polynomial of set S"
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
pub fn 
 r1cs_to_sap_witness_map< FieldT>(cs:&r1cs_constraint_system<FieldT>,
                                            primary_input:&r1cs_primary_input<FieldT>,
                                            auxiliary_input:&r1cs_auxiliary_input<FieldT>,
                                            d1:&FieldT,
                                            d2:&FieldT)->sap_witness<FieldT>
{
    ffec::enter_block("Call to r1cs_to_sap_witness_map");

    /* sanity check */
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let  domain =
        r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut  full_variable_assignment = primary_input.clone();
    full_variable_assignment.insert(full_variable_assignment.end(), auxiliary_input.begin(), auxiliary_input.end());
    /**
     * we need to generate values of all the extra variables that we added
     * during the reduction
     *
     * note: below, we pass full_variable_assignment into the .evaluate()
     * method of the R1CS constraints. however, these extra variables shouldn't
     * be a problem, because .evaluate() only accesses the variables that are
     * actually used in the constraint.
     */
    for i in 0..cs.num_constraints()
    {
        /**
         * this is variable (extra_var_offset + i), an extra variable
         * we introduced that is not present in the input.
         * its value is (a - b)^2
         */
        let mut extra_var = cs.constraints[i].a.evaluate(full_variable_assignment) -
            cs.constraints[i].b.evaluate(full_variable_assignment);
        extra_var = extra_var * extra_var;
        full_variable_assignment.push_back(extra_var);
    }
    for i in 1..=cs.num_inputs()
    {
        /**
         * this is variable (extra_var_offset2 + i), an extra variable
         * we introduced that is not present in the input.
         * its value is (x_i - 1)^2
         */
        let mut  extra_var = full_variable_assignment[i - 1] - FieldT::one();
        extra_var = extra_var * extra_var;
        full_variable_assignment.push_back(extra_var);
    }

    ffec::enter_block("Compute evaluation of polynomial A on set S");
    let mut aA=vec![FieldT::zero();domain.m];

    /* account for all constraints, as in r1cs_to_sap_instance_map */
    for i in 0..cs.num_constraints()
    {
        aA[2 * i] += cs.constraints[i].a.evaluate(full_variable_assignment);
        aA[2 * i] += cs.constraints[i].b.evaluate(full_variable_assignment);

        aA[2 * i + 1] += cs.constraints[i].a.evaluate(full_variable_assignment);
        aA[2 * i + 1] -= cs.constraints[i].b.evaluate(full_variable_assignment);
    }

    let extra_constr_offset = 2 * cs.num_constraints();

    aA[extra_constr_offset] += FieldT::one();

    for i in 1..=cs.num_inputs()
    {
        aA[extra_constr_offset + 2 * i - 1] += full_variable_assignment[i - 1];
        aA[extra_constr_offset + 2 * i - 1] += FieldT::one();

        aA[extra_constr_offset + 2 * i] += full_variable_assignment[i - 1];
        aA[extra_constr_offset + 2 * i] -= FieldT::one();
    }

    ffec::leave_block("Compute evaluation of polynomial A on set S");

    ffec::enter_block("Compute coefficients of polynomial A");
    domain.iFFT(aA);
    ffec::leave_block("Compute coefficients of polynomial A");

    ffec::enter_block("Compute ZK-patch");
    let coefficients_for_H=vec![FieldT::zero();domain.m+1];
// // #ifdef MULTICORE
// #pragma omp parallel for
// //#endif
    /* add coefficients of the polynomial (2*d1*A - d2) + d1*d1*Z */
    for i in 0..domain.m
    {
        coefficients_for_H[i] = (d1 * aA[i]) + (d1 * aA[i]);
    }
    coefficients_for_H[0] -= d2;
    domain.add_poly_Z(d1 * d1, coefficients_for_H);
    ffec::leave_block("Compute ZK-patch");

    ffec::enter_block("Compute evaluation of polynomial A on set T");
    domain.cosetFFT(aA, FieldT::multiplicative_generator);
    ffec::leave_block("Compute evaluation of polynomial A on set T");

    ffec::enter_block("Compute evaluation of polynomial H on set T");
    let mut H_tmp = &aA; // can overwrite aA because it is not used later
// // #ifdef MULTICORE
// #pragma omp parallel for
// //#endif
    for i in 0..domain.m
    {
        H_tmp[i] = aA[i]*aA[i];
    }

    ffec::enter_block("Compute evaluation of polynomial C on set S");
    let mut aC=vec![FieldT::zero();domain.m];
    /* again, accounting for all constraints */
    let  extra_var_offset = cs.num_variables() + 1;
    for i in 0..cs.num_constraints()
    {
        aC[2 * i] +=
            times_four(cs.constraints[i].c.evaluate(full_variable_assignment));

        aC[2 * i] += full_variable_assignment[extra_var_offset + i - 1];
        aC[2 * i + 1] += full_variable_assignment[extra_var_offset + i - 1];
    }

    let  extra_var_offset2 = cs.num_variables() + cs.num_constraints();
    aC[extra_constr_offset] += FieldT::one();

    for i in 1..=cs.num_inputs()
    {
        aC[extra_constr_offset + 2 * i - 1] +=
            times_four(full_variable_assignment[i - 1]);

        aC[extra_constr_offset + 2 * i - 1] +=
            full_variable_assignment[extra_var_offset2 + i - 1];
        aC[extra_constr_offset + 2 * i] +=
            full_variable_assignment[extra_var_offset2 + i - 1];
    }

    ffec::leave_block("Compute evaluation of polynomial C on set S");

    ffec::enter_block("Compute coefficients of polynomial C");
    domain.iFFT(aC);
    ffec::leave_block("Compute coefficients of polynomial C");

    ffec::enter_block("Compute evaluation of polynomial C on set T");
    domain.cosetFFT(aC, FieldT::multiplicative_generator);
    ffec::leave_block("Compute evaluation of polynomial C on set T");

// // #ifdef MULTICORE
// #pragma omp parallel for
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
// #pragma omp parallel for
// //#endif
    for i in 0..domain.m
    {
        coefficients_for_H[i] += H_tmp[i];
    }
    ffec::leave_block("Compute sum of H and ZK-patch");

    ffec::leave_block("Call to r1cs_to_sap_witness_map");

    return sap_witness::<FieldT>(sap_num_variables,
                               domain.m,
                               cs.num_inputs(),
                               d1,
                               d2,
                               full_variable_assignment,
                               (coefficients_for_H));
}



// //#endif // R1CS_TO_SAP_TCC_
