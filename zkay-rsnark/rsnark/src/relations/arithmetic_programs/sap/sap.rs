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
// use  <memory>

use fqfft::evaluation_domain::evaluation_domain;



/* forward declaration */
// 
// class sap_witness;

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
pub struct sap_instance<FieldT> {
// //private:
num_variables:    usize,
degree:    usize,
num_inputs:    usize,

// //public:
domain:    RcCell<fqfft::evaluation_domain<FieldT> >,

A_in_Lagrange_basis:    Vec<HashMap<usize, FieldT> >,
C_in_Lagrange_basis:    Vec<HashMap<usize, FieldT> >,
}

//     sap_instance(const RcCell<fqfft::evaluation_domain<FieldT> > &domain,
//                  const usize num_variables,
//                  const usize degree,
//                  const usize num_inputs,
//                  const Vec<HashMap<usize, FieldT> > &A_in_Lagrange_basis,
//                  const Vec<HashMap<usize, FieldT> > &C_in_Lagrange_basis);

//     sap_instance(const RcCell<fqfft::evaluation_domain<FieldT> > &domain,
//                  const usize num_variables,
//                  const usize degree,
//                  const usize num_inputs,
//                  Vec<HashMap<usize, FieldT> > &&A_in_Lagrange_basis,
//                  Vec<HashMap<usize, FieldT> > &&C_in_Lagrange_basis);

//     sap_instance(const sap_instance<FieldT> &other) = default;
//     sap_instance(sap_instance<FieldT> &&other) = default;
//     sap_instance& operator=(const sap_instance<FieldT> &other) = default;
//     sap_instance& operator=(sap_instance<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(const sap_witness<FieldT> &witness) const;
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
pub struct sap_instance_evaluation<FieldT>{
//private:
num_variables:    usize,
degree:    usize,
num_inputs:    usize,
//public:
domain:    RcCell<fqfft::evaluation_domain<FieldT> >,

t:    FieldT,

Ht:    Vec<FieldT>, At:Vec<FieldT>, Ct:Vec<FieldT>,

Zt:    FieldT,
}

//     sap_instance_evaluation(const RcCell<fqfft::evaluation_domain<FieldT> > &domain,
//                             const usize num_variables,
//                             const usize degree,
//                             const usize num_inputs,
//                             const FieldT &t,
//                             const Vec<FieldT> &At,
//                             const Vec<FieldT> &Ct,
//                             const Vec<FieldT> &Ht,
//                             const FieldT &Zt);
//     sap_instance_evaluation(const RcCell<fqfft::evaluation_domain<FieldT> > &domain,
//                             const usize num_variables,
//                             const usize degree,
//                             const usize num_inputs,
//                             const FieldT &t,
//                             Vec<FieldT> &&At,
//                             Vec<FieldT> &&Ct,
//                             Vec<FieldT> &&Ht,
//                             const FieldT &Zt);

//     sap_instance_evaluation(const sap_instance_evaluation<FieldT> &other) = default;
//     sap_instance_evaluation(sap_instance_evaluation<FieldT> &&other) = default;
//     sap_instance_evaluation& operator=(const sap_instance_evaluation<FieldT> &other) = default;
//     sap_instance_evaluation& operator=(sap_instance_evaluation<FieldT> &&other) = default;

//     usize num_variables() const;
//     usize degree() const;
//     usize num_inputs() const;

//     bool is_satisfied(const sap_witness<FieldT> &witness) const;
// };

/**
 * A SAP witness.
 */
// 
pub struct sap_witness<FieldT>{
//private:
num_variables:    usize,
degree:    usize,
num_inputs:    usize,

//public:
d2:    FieldT,d1:FieldT,

coefficients_for_ACs:    Vec<FieldT>,
coefficients_for_H:    Vec<FieldT>,
}

//     sap_witness(const usize num_variables,
//                 const usize degree,
//                 const usize num_inputs,
//                 const FieldT &d1,
//                 const FieldT &d2,
//                 const Vec<FieldT> &coefficients_for_ACs,
//                 const Vec<FieldT> &coefficients_for_H);

//     sap_witness(const usize num_variables,
//                 const usize degree,
//                 const usize num_inputs,
//                 const FieldT &d1,
//                 const FieldT &d2,
//                 const Vec<FieldT> &coefficients_for_ACs,
//                 Vec<FieldT> &&coefficients_for_H);

//     sap_witness(const sap_witness<FieldT> &other) = default;
//     sap_witness(sap_witness<FieldT> &&other) = default;
//     sap_witness& operator=(const sap_witness<FieldT> &other) = default;
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



impl<FieldT> sap_instance<FieldT>{

pub fn new(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                   num_variables:usize,
                                   degree:usize,
                                   num_inputs:usize,
                                   A_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >,
                                   C_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >) ->Self
   
{
 Self{num_variables,
    degree,
    num_inputs,
    domain,
    A_in_Lagrange_basis,
    C_in_Lagrange_basis}
}


pub fn new2(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                   num_variables:usize,
                                   degree:usize,
                                   num_inputs:usize,
A_in_Lagrange_basis:                                   Vec<HashMap<usize, FieldT> >,
C_in_Lagrange_basis:                                   Vec<HashMap<usize, FieldT> >) ->Self
   
{
 Self{num_variables,
    degree,
    num_inputs,
    domain,
    A_in_Lagrange_basis,
    C_in_Lagrange_basis}
}


pub fn num_variables()->usize
{
    return num_variables;
}


pub fn degree()->usize
{
    return degree;
}


pub fn num_inputs()->usize
{
    return num_inputs;
}


 pub fn is_satisfied(witness:sap_witness<FieldT>) ->bool
{
    let  t = FieldT::random_element();

    let At =vec![FieldT::zero();self.num_variables()+1];
    let  Ct =vec![FieldT::zero();self.num_variables()+1];
    let  Ht=Vec::with_capacity(self.degree()+1);

    let  Zt = self.domain.compute_vanishing_polynomial(t);

    let  u = self.domain.evaluate_all_lagrange_polynomials(t);

    for  i in 0.. self.num_variables()+1
    {
        for el in &A_in_Lagrange_basis[i]
        {
            At[i] += u[el.first] * el.second;
        }

        for el in &C_in_Lagrange_basis[i]
        {
            Ct[i] += u[el.first] * el.second;
        }
    }

    let  ti = FieldT::one();
    for i in  0.. self.degree()+1
    {
        Ht[i] = ti;
        ti *= t;
    }

    let   eval_sap_inst=sap_instance_evaluation::<FieldT>::new(self.domain,
                                                        self.num_variables(),
                                                        self.degree(),
                                                        self.num_inputs(),
                                                        t,
                                                        At,
                                                        Ct,
                                                        Ht,
                                                        Zt);
    return eval_sap_inst.is_satisfied(witness);
}
}

impl<FieldT> sap_instance_evaluation<FieldT>{

pub fn new(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                                         num_variables:usize,
                                                         degree:usize,
                                                         num_inputs:usize,
                                                         t:FieldT,
                                                         At:Vec<FieldT>,
                                                         Ct:Vec<FieldT>,
                                                         Ht:Vec<FieldT>,
                                                         Zt:FieldT) ->Self

{
    Self{num_variables,
    degree,
    num_inputs,
    domain,
    t,
    At,
    Ct,
    Ht,
    Zt}
}


pub fn new1(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                                         num_variables:usize,
                                                         degree:usize,
                                                         num_inputs:usize,
                                                         t:FieldT,
At:                                                         Vec<FieldT>,
Ct:                                                         Vec<FieldT>,
Ht:                                                         Vec<FieldT>,
                                                         Zt:FieldT) ->Self
  
{
 Self{ num_variables,
    degree,
    num_inputs,
    domain,
    t,
    At,
    Ct,
    Ht,
    Zt}
}


pub fn num_variables()->usize
{
    return num_variables;
}


pub fn degree()->usize
{
    return degree;
}


pub fn num_inputs()->usize
{
    return num_inputs;
}


 pub fn is_satisfied(witness:sap_witness<FieldT>) ->bool
{
    if self.num_variables() != witness.num_variables()
    {
        return false;
    }

    if self.degree() != witness.degree()
    {
        return false;
    }

    if self.num_inputs() != witness.num_inputs()
    {
        return false;
    }

    if self.num_variables() != witness.coefficients_for_ACs.size()
    {
        return false;
    }

    if self.degree()+1 != witness.coefficients_for_H.size()
    {
        return false;
    }

    if self.At.size() != self.num_variables()+1 || self.Ct.size() != self.num_variables()+1
    {
        return false;
    }

    if self.Ht.size() != self.degree()+1
    {
        return false;
    }

    if self.Zt != self.domain.compute_vanishing_polynomial(self.t)
    {
        return false;
    }

    let ans_A = self.At[0] + witness.d1*self.Zt;
    let ans_C = self.Ct[0] + witness.d2*self.Zt;
    let ans_H = FieldT::zero();

    ans_A = ans_A + ffec::inner_product::<FieldT>(self.At.begin()+1,
                                                 self.At.begin()+1+self.num_variables(),
                                                 witness.coefficients_for_ACs.begin(),
                                                 witness.coefficients_for_ACs.begin()+self.num_variables());
    ans_C = ans_C + ffec::inner_product::<FieldT>(self.Ct.begin()+1,
                                                 self.Ct.begin()+1+self.num_variables(),
                                                 witness.coefficients_for_ACs.begin(),
                                                 witness.coefficients_for_ACs.begin()+self.num_variables());
    ans_H = ans_H + ffec::inner_product::<FieldT>(self.Ht.begin(),
                                                 self.Ht.begin()+self.degree()+1,
                                                 witness.coefficients_for_H.begin(),
                                                 witness.coefficients_for_H.begin()+self.degree()+1);

    if ans_A * ans_A - ans_C != ans_H * self.Zt
    {
        return false;
    }

    return true;
}
}
impl sap_witness<FieldT>{

pub fn new(num_variables:usize,
                                 degree:usize,
                                 num_inputs:usize,
                                 d1:FieldT,
                                 d2:FieldT,
                                 coefficients_for_ACs:Vec<FieldT>,
                                 coefficients_for_H:Vec<FieldT>) ->Self
   
{
 Self{num_variables,
    degree,
    num_inputs,
    d1,
    d2,
    coefficients_for_ACs,
    coefficients_for_H}
}


pub fn new2(num_variables:usize,
                                 degree:usize,
                                 num_inputs:usize,
                                 d1:FieldT,
                                 d2:FieldT,
                                 coefficients_for_ACs:Vec<FieldT>,
coefficients_for_H:                                 Vec<FieldT>) ->Self
 
{
   Self{
    num_variables,
    degree,
    num_inputs,
    d1,
    d2,
    coefficients_for_ACs,
    coefficients_for_H
    }
}



pub fn num_variables()->usize
{
    return num_variables;
}


pub fn degree()->usize
{
    return degree;
}


pub fn num_inputs()->usize
{
    return num_inputs;
}


}

//#endif // SAP_TCC_
