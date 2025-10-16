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
// use  <memory>

use fqfft::evaluation_domain::evaluation_domain;
use std::collections::HashMap;
use rccell::RcCell;

/* forward declaration */
// 
// class qap_witness;

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
pub struct qap_instance<FieldT> {
// private:
    num_variables:usize,
    degree:usize,
    num_inputs:usize,


domain:    RcCell<fqfft::evaluation_domain<FieldT> >,

A_in_Lagrange_basis:   Vec<HashMap<usize, FieldT> >,
B_in_Lagrange_basis:   Vec<HashMap<usize, FieldT> >,
C_in_Lagrange_basis:   Vec<HashMap<usize, FieldT> >,
}
// impl qap_instance<FieldT>{
//     qap_instance(domain:&RcCell<fqfft::evaluation_domain<FieldT> >
//                  num_variables:usize
//                  degree:usize
//                  num_inputs:usize
//                  A_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >
//                  B_in_Lagrange_basis:&Vec<HashMap<usize, FieldT> >
//                  &C_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >);

//     qap_instance(domain:&RcCell<fqfft::evaluation_domain<FieldT> >
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
pub struct  qap_instance_evaluation<FieldT> {
// private:
    num_variables:usize,
    degree:usize,
    num_inputs:usize,

domain:    RcCell<fqfft::evaluation_domain<FieldT> >,

    t:FieldT,

     At:Vec<FieldT>, Bt:Vec<FieldT>, Ct:Vec<FieldT>, Ht:Vec<FieldT>,

    Zt:FieldT,
}

//     qap_instance_evaluation(domain:&RcCell<fqfft::evaluation_domain<FieldT> >
//                             num_variables:usize
//                             degree:usize
//                             num_inputs:usize
//                             t:&FieldT
//                             At:&Vec<FieldT>
//                             Bt:&Vec<FieldT>
//                             Ct:&Vec<FieldT>
//                             Ht:&Vec<FieldT>
//                             &Zt:FieldT);
//     qap_instance_evaluation(domain:&RcCell<fqfft::evaluation_domain<FieldT> >
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
// private:
    num_variables:usize,
    degree:usize,
    num_inputs:usize,


     d1:FieldT, d2:FieldT, d3:FieldT,

coefficients_for_ABCs:   Vec<FieldT>,
coefficients_for_H:   Vec<FieldT>,
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
use fqfft::evaluation_domain::evaluation_domain;


impl<FieldT> qap_instance<FieldT>{

pub fn new(domain:&RcCell<fqfft::evaluation_domain<FieldT> >,
                                   num_variables:usize,
                                   degree:usize,
                                   num_inputs:usize,
                                   A_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >,
                                   B_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >,
                                   C_in_Lagrange_basis:Vec<HashMap<usize, FieldT> >) ->Self
   
{
   Self {num_variables,
    degree,
    num_inputs,
    domain,
    A_in_Lagrange_basis,
    B_in_Lagrange_basis,
    C_in_Lagrange_basis}
}


pub fn qap_instance(domain:&RcCell<fqfft::evaluation_domain<FieldT> >,
                                   num_variables:usize,
                                   degree:usize,
                                   num_inputs:usize,
A_in_Lagrange_basis:                                  Vec<HashMap<usize, FieldT> >,
B_in_Lagrange_basis:                                  Vec<HashMap<usize, FieldT> >,
C_in_Lagrange_basis:                                  Vec<HashMap<usize, FieldT> >) ->Self
  
{
  Self{num_variables,
    degree,
    num_inputs,
    domain,
    A_in_Lagrange_basis,
    B_in_Lagrange_basis,
    C_in_Lagrange_basis}
}


 pub fn num_variables() ->usize
{
    return num_variables_;
}


 pub fn degree() ->usize
{
    return degree_;
}


 pub fn num_inputs() ->usize
{
    return num_inputs_;
}


 pub fn is_satisfied(witness:&qap_witness<FieldT>) ->bool
{
    let t =FieldT::random_element( );

   let At= vec![FieldT::zero();self.num_variables()+1];
   let  Bt= vec![FieldT::zero();self.num_variables()+1];
   let Ct= vec![FieldT::zero();self.num_variables()+1];
   let Ht= vec![FieldT::zero();self.degree()+1];

    let  Zt =self.domain.compute_vanishing_polynomial(t);

    let  u =self.domain.evaluate_all_lagrange_polynomials(t);

    for i in 0..self.num_variables()+1
    {
        for el in & A_in_Lagrange_basis[i]
        {
            At[i] += u[el.first] * el.second;
        }

        for el in  &B_in_Lagrange_basis[i]
        {
            Bt[i] += u[el.first] * el.second;
        }

        for el in  &C_in_Lagrange_basis[i]
        {
            Ct[i] += u[el.first] * el.second;
        }
    }

    let mut  ti = FieldT::one();
    for i in 0..self.degree()+1
    {
        Ht[i] = ti;
        ti *= t;
    }

    let eval_qap_inst=qap_instance_evaluation::<FieldT>::new(self.domain,
                                                        self.num_variables(),
                                                        self.degree(),
                                                        self.num_inputs(),
                                                        t,
                                                        At,
                                                        Bt,
                                                        Ct,
                                                        Ht,
                                                        Zt);
    return eval_qap_inst.is_satisfied(witness);
}
}

impl<FieldT> qap_instance_evaluation<FieldT>{

pub fn new(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                                         num_variables:usize,
                                                         degree:usize,
                                                         num_inputs:usize,
                                                         t:FieldT,
                                                         At:Vec<FieldT>,
                                                         Bt:Vec<FieldT>,
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
    Bt,
    Ct,
    Ht,
    Zt}
}


pub fn qap_instance_evaluation(domain:RcCell<fqfft::evaluation_domain<FieldT> >,
                                                         num_variables:usize,
                                                         degree:usize,
                                                         num_inputs:usize,
                                                         t:FieldT,
At:                                                        Vec<FieldT>,
Bt:                                                        Vec<FieldT>,
Ct:                                                        Vec<FieldT>,
Ht:                                                        Vec<FieldT>,
                                                         Zt:FieldT) ->Self
   
{
 Self{num_variables,
    degree,
    num_inputs,
    domain,
    t,
    At,
    Bt,
    Ct,
    Ht,
    Zt}
}


 pub fn num_variables()  ->usize
{
    return num_variables_;
}


 pub fn degree()  ->usize
{
    return degree_;
}


 pub fn num_inputs() ->usize
{
    return num_inputs_;
}


 pub fn is_satisfied(&witness:qap_witness<FieldT>) ->bool
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

    if self.num_variables() != witness.coefficients_for_ABCs.size()
    {
        return false;
    }

    if self.degree()+1 != witness.coefficients_for_H.size()
    {
        return false;
    }

    if self.At.size() != self.num_variables()+1 || self.Bt.size() != self.num_variables()+1 || self.Ct.size() != self.num_variables()+1
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
    let ans_B = self.Bt[0] + witness.d2*self.Zt;
    let ans_C = self.Ct[0] + witness.d3*self.Zt;
    let ans_H = FieldT::zero();

    ans_A = ans_A + ffec::inner_product::<FieldT>(self.At.begin()+1,
                                                 self.At.begin()+1+self.num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+self.num_variables());
    ans_B = ans_B + ffec::inner_product::<FieldT>(self.Bt.begin()+1,
                                                 self.Bt.begin()+1+self.num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+self.num_variables());
    ans_C = ans_C + ffec::inner_product::<FieldT>(self.Ct.begin()+1,
                                                 self.Ct.begin()+1+self.num_variables(),
                                                 witness.coefficients_for_ABCs.begin(),
                                                 witness.coefficients_for_ABCs.begin()+self.num_variables());
    ans_H = ans_H + ffec::inner_product::<FieldT>(self.Ht.begin(),
                                                 self.Ht.begin()+self.degree()+1,
                                                 witness.coefficients_for_H.begin(),
                                                 witness.coefficients_for_H.begin()+self.degree()+1);

    if ans_A * ans_B - ans_C != ans_H * self.Zt
    {
        return false;
    }

    return true;
}
}

impl<FieldT> qap_witness<FieldT>{

pub fn new(num_variables:usize,
                                 degree:usize,
                                 num_inputs:usize,
                                 d1:FieldT,
                                 d2:FieldT,
                                 d3:FieldT,
                                 coefficients_for_ABCs:Vec<FieldT>,
                                 coefficients_for_H:Vec<FieldT>) ->Self
   
{
 Self{num_variables,
    degree,
    num_inputs,
    d1,
    d2,
    d3,
    coefficients_for_ABCs,
    coefficients_for_H}
}


pub fn qap_witness(num_variables:usize,
                                 degree:usize,
                                 num_inputs:usize,
                                 d1:FieldT,
                                 d2:FieldT,
                                 d3:FieldT,
                                 coefficients_for_ABCs:Vec<FieldT>,
coefficients_for_H:                                Vec<FieldT>) ->Self
    
{
Self{num_variables,
    degree,
    num_inputs,
    d1,
    d2,
    d3,
    coefficients_for_ABCs,
    coefficients_for_H}
}



 pub fn num_variables() ->usize
{
    return num_variables_;
}


 pub fn degree() ->usize
{
    return degree_;
}


 pub fn num_inputs() ->usize
{
    return num_inputs_;
}



}

//#endif // QAP_TCC_
