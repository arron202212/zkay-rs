// Declaration of interfaces for a R1CS-to-SAP reduction, that is, constructing
// a SAP ("Square Arithmetic Program") from a R1CS ("Rank-1 Constraint System").

// SAPs are defined and constructed from R1CS in \[GM17].

// The implementation of the reduction follows, extends, and optimizes
// the efficient approach described in Appendix E of \[BCGTV13].

// References:

// \[BCGTV13]
// "SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge",
// Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer, Madars Virza,
// CRYPTO 2013,
// <http://eprint.iacr.org/2013/507>

// \[GM17]:
// "Snarky Signatures: Minimal Signatures of Knowledge from
//  Simulation-Extractable SNARKs",
// Jens Groth and Mary Maller,
// IACR-CRYPTO-2017,
// <https://eprint.iacr.org/2017/540>

use crate::relations::arithmetic_programs::qap::qap::{
    qap_instance, qap_instance_evaluation, qap_witness,
};
use crate::relations::arithmetic_programs::sap::sap::{
    sap_instance, sap_instance_evaluation, sap_witness,
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

use fqfft::evaluation_domain::{
    evaluation_domain::{EvaluationDomainConfig, EvaluationDomainType, evaluation_domain},
    get_evaluation_domain::get_evaluation_domain,
};
use rccell::RcCell;

use std::collections::BTreeMap;

use tracing::{Level, span};

// /**
//  * Helper function to multiply a field element by 4 efficiently
//  */
pub fn times_four<FieldT: FieldTConfig>(x: FieldT) -> FieldT {
    let times_two = x.clone() + x.clone();
    return times_two.clone() + times_two.clone();
}

//  * Helper function to find evaluation domain that will be used by the reduction
//  * for a given R1CS instance.

pub fn r1cs_to_sap_get_domain<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
) -> RcCell<EvaluationDomainType<FieldT>> {
    // /*
    //  * the SAP instance will have:
    //  * - two constraints for every constraint in the original constraint system
    //  * - two constraints for every public input, except the 0th, which
    //  *   contributes just one extra constraint
    //  * see comments in r1cs_to_sap_instance_map for details on where these
    //  * constraints come from.
    //  */
    get_evaluation_domain::<FieldT>(2 * cs.num_constraints() + 2 * cs.num_inputs() + 1).unwrap()
}

// /**
//  * Instance map for the R1CS-to-SAP reduction.
//  */
pub fn r1cs_to_sap_instance_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
) -> sap_instance<FieldT> {
    let span0 = span!(Level::TRACE, "Call to r1cs_to_sap_instance_map");
    let _=span0.enter();

    let domain = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut A_in_Lagrange_basis =
        Vec::<BTreeMap<usize, FieldT>>::with_capacity(sap_num_variables + 1);
    let mut C_in_Lagrange_basis =
        Vec::<BTreeMap<usize, FieldT>>::with_capacity(sap_num_variables + 1);

    let span = span!(Level::TRACE, "Compute polynomials A, C in Lagrange basis").entered();
    // /**
    //  * process R1CS constraints, converting a constraint of the form
    //  *   \sum a_i x_i * \sum b_i x_i = \sum c_i x_i
    //  * into two constraints
    //  *   (\sum (a_i + b_i) x_i)^2 = 4 \sum c_i x_i + x'_i
    //  *   (\sum (a_i - b_i) x_i)^2 = x'_i
    //  * where x'_i is an extra variable (a separate one for each original
    //  * constraint)
    //  *
    //  * this adds 2 * cs.num_constraints() constraints
    //  *   (numbered 0 .. 2 * cs.num_constraints() - 1)
    //  * and cs.num_constraints() extra variables
    //  *   (numbered cs.num_variables() + 1 .. cs.num_variables() + cs.num_constraints())
    //  */
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

    // /**
    //  * add and convert the extra constraints
    //  *     x_i * 1 = x_i
    //  * to ensure that the polynomials 0 .. cs.num_inputs() are linearly
    //  * independent from each other and the rest, which is required for security
    //  * proofs (see [GM17, p. 29])
    //  *
    //  * note that i = 0 is a special case, where this constraint is expressible
    //  * as x_0^2 = x_0,
    //  * whereas for every other i we introduce an extra variable x''_i and do
    //  *   (x_i + x_0)^2 = 4 x_i + x''_i
    //  *   (x_i - x_0)^2 = x''_i
    //  *
    //  * this adds 2 * cs.num_inputs() + 1 extra constraints
    //  *   (numbered 2 * cs.num_constraints() ..
    //  *             2 * cs.num_constraints() + 2 * cs.num_inputs())
    //  * and cs.num_inputs() extra variables
    //  *   (numbered cs.num_variables() + cs.num_constraints() + 1 ..
    //  *             cs.num_variables() + cs.num_constraints() + cs.num_inputs())
    //  */
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

    span.exit();

   

    sap_instance::<FieldT>::new(
        domain.clone(),
        sap_num_variables,
        domain.borrow().m(),
        cs.num_inputs(),
        A_in_Lagrange_basis,
        C_in_Lagrange_basis,
    )
}

// /**
//  * Instance map for the R1CS-to-SAP reduction followed by evaluation
//  * of the resulting QAP instance.
//  */
pub fn r1cs_to_sap_instance_map_with_evaluation<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    t: &FieldT,
) -> sap_instance_evaluation<FieldT> {
    let span0 = span!(
        Level::TRACE,
        "Call to r1cs_to_sap_instance_map_with_evaluation"
    )
    ;
    let _=span0.enter();

    let domain: RcCell<EvaluationDomainType<FieldT>> = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let (mut At, mut Ct, mut Ht) = (
        vec![FieldT::zero(); sap_num_variables + 1],
        vec![FieldT::zero(); sap_num_variables + 1],
        Vec::with_capacity(domain.borrow().m() + 1),
    );

    let Zt = domain.borrow_mut().compute_vanishing_polynomial(t);

    let span = span!(Level::TRACE, "Compute evaluations of A, C, H at t").entered();
    let u = domain.borrow_mut().evaluate_all_lagrange_polynomials(t);
    // /**
    //  * add and process all constraints as in r1cs_to_sap_instance_map
    //  */
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
    for i in 0..domain.borrow().m() + 1 {
        Ht.push(ti.clone());
        ti *= t.clone();
    }
    span.exit();

   

    sap_instance_evaluation::<FieldT>::new(
        domain.clone(),
        sap_num_variables,
        domain.borrow().m(),
        cs.num_inputs(),
        t.clone(),
        At,
        Ct,
        Ht,
        Zt,
    )
}

// /**
// * Witness map for the R1CS-to-SAP reduction.
// *
// * The witness map takes zero knowledge into account when d1, d2 are random.
// *
// * More precisely, compute the coefficients
// *     h_0,h_1,...,h_n
// * of the polynomial
// *     H(z)->Self= (A(z)*A(z)-C(z))/Z(z)
// * where
// *   A(z)->Self= A_0(z) + \sum_{k=1}^{m} w_k A_k(z) + d1 * Z(z)
// *   C(z)->Self= C_0(z) + \sum_{k=1}^{m} w_k C_k(z) + d2 * Z(z)
// *   Z(z)->Self= "vanishing polynomial of set S"
// * and
// *   m = number of variables of the SAP
// *   n = degree of the SAP
// *
// * This is done as follows:
// *  (1) compute evaluations of A,C on S = {sigma_1,...,sigma_n}
// *  (2) compute coefficients of A,C
// *  (3) compute evaluations of A,C on T = "coset of S"
// *  (4) compute evaluation of H on T
// *  (5) compute coefficients of H
// *  (6) patch H to account for d1,d2
//        (i.e., add coefficients of the polynomial (2*d1*A - d2 + d1^2 * Z))
// *
// * The code below is not as simple as the above high-level description due to
// * some reshuffling to save space.
// */
pub fn r1cs_to_sap_witness_map<
    FieldT: FieldTConfig,
    SV: SubVariableConfig,
    SLC: SubLinearCombinationConfig,
>(
    cs: &r1cs_constraint_system<FieldT, SV, SLC>,
    primary_input: &r1cs_primary_input<FieldT>,
    auxiliary_input: &r1cs_auxiliary_input<FieldT>,
    d1: &FieldT,
    d2: &FieldT,
) -> sap_witness<FieldT> {
    let span0 = span!(Level::TRACE, "Call to r1cs_to_sap_witness_map");
    let _=span0.enter();

    // sanity check
    assert!(cs.is_satisfied(primary_input, auxiliary_input));

    let domain: RcCell<EvaluationDomainType<FieldT>> = r1cs_to_sap_get_domain(cs);

    let sap_num_variables = cs.num_variables() + cs.num_constraints() + cs.num_inputs();

    let mut full_variable_assignment: Vec<_> = primary_input
        .iter()
        .chain(auxiliary_input)
        .cloned()
        .collect();

    // /**
    //  * we need to generate values of all the extra variables that we added
    //  * during the reduction
    //  *
    //  * note: below, we pass full_variable_assignment into the .evaluate()
    //  * method of the R1CS constraints. however, these extra variables shouldn't
    //  * be a problem, because .evaluate() only accesses the variables that are
    //  * actually used in the constraint.
    //  */
    for i in 0..cs.num_constraints() {
        // /**
        //  * this is variable (extra_var_offset + i), an extra variable
        //  * we introduced that is not present in the input.
        //  * its value is (a - b)^2
        //  */
        let mut extra_var = cs.constraints[i].a.evaluate(&full_variable_assignment)
            - cs.constraints[i].b.evaluate(&full_variable_assignment);
        extra_var = extra_var.clone() * extra_var.clone();
        full_variable_assignment.push(extra_var);
    }
    for i in 1..=cs.num_inputs() {
        // /**
        //  * this is variable (extra_var_offset2 + i), an extra variable
        //  * we introduced that is not present in the input.
        //  * its value is (x_i - 1)^2
        //  */
        let mut extra_var = full_variable_assignment[i - 1].clone() - FieldT::one();
        extra_var = extra_var.clone() * extra_var.clone();
        full_variable_assignment.push(extra_var);
    }

    let spanas = span!(Level::TRACE, "Compute evaluation of polynomial A on set S").entered();
    let mut aA = vec![FieldT::zero(); domain.borrow().m()];

    // account for all constraints, as in r1cs_to_sap_instance_map
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

    spanas.exit();

    let spana = span!(Level::TRACE, "Compute coefficients of polynomial A").entered();
    domain.borrow_mut().iFFT(&mut aA);
    spana.exit();

    let spanzp = span!(Level::TRACE, "Compute ZK-patch").entered();
    let mut coefficients_for_H = vec![FieldT::zero(); domain.borrow().m() + 1];

    // add coefficients of the polynomial (2*d1*A - d2) + d1*d1*Z
    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] = (d1.clone() * aA[i].clone()) + (d1.clone() * aA[i].clone());
    }
    coefficients_for_H[0] -= d2.clone();
    domain
        .borrow_mut()
        .add_poly_Z(&(d1.clone() * d1.clone()), &mut coefficients_for_H);
    spanzp.exit();

    let spanat = span!(Level::TRACE, "Compute evaluation of polynomial A on set T").entered();
    domain
        .borrow_mut()
        .cosetFFT(&mut aA, &FieldT::multiplicative_generator());
    spanat.exit();

    let spanht = span!(Level::TRACE, "Compute evaluation of polynomial H on set T").entered();
    let mut H_tmp = aA.clone(); // can overwrite aA because it is not used later

    for i in 0..domain.borrow().m() {
        H_tmp[i] = aA[i].clone() * aA[i].clone();
    }

    let spancs = span!(Level::TRACE, "Compute evaluation of polynomial C on set S").entered();
    let mut aC = vec![FieldT::zero(); domain.borrow().m()];
    // again, accounting for all constraints
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

    spancs.exit();

    let spanc = span!(Level::TRACE, "Compute coefficients of polynomial C").entered();
    domain.borrow_mut().iFFT(&mut aC);
    spanc.exit();

    let spanct = span!(Level::TRACE, "Compute evaluation of polynomial C on set T").entered();
    domain
        .borrow_mut()
        .cosetFFT(&mut aC, &FieldT::multiplicative_generator());
    spanct.exit();

    for i in 0..domain.borrow().m() {
        H_tmp[i] = (H_tmp[i].clone() - aC[i].clone());
    }

    let spanzt = span!(Level::TRACE, "Divide by Z on set T").entered();
    domain.borrow().divide_by_Z_on_coset(&mut H_tmp);
    spanzt.exit();

    spanht.exit();

    let spanh = span!(Level::TRACE, "Compute coefficients of polynomial H").entered();
    domain
        .borrow_mut()
        .icosetFFT(&mut H_tmp, &FieldT::multiplicative_generator());
    spanh.exit();

    let spanhp = span!(Level::TRACE, "Compute sum of H and ZK-patch").entered();

    for i in 0..domain.borrow().m() {
        coefficients_for_H[i] += H_tmp[i].clone();
    }
    spanhp.exit();

   

     sap_witness::<FieldT>::new(
        sap_num_variables,
        domain.borrow().m(),
        cs.num_inputs(),
        d1.clone(),
        d2.clone(),
        full_variable_assignment,
        (coefficients_for_H),
    )
}
