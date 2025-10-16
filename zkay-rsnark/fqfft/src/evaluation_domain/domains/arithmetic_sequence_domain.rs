/** @file
 *****************************************************************************

 Declaration of interfaces for the "arithmetic sequence" evaluation domain.

 These functions use an arithmetic sequence of size m to perform evaluation.

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ARITHMETIC_SEQUENCE_DOMAIN_HPP
// #define ARITHMETIC_SEQUENCE_DOMAIN_HPP

use crate::evaluation_domain::evaluation_domain;

// //namespace libfqfft {

//   
  pub struct arithmetic_sequence_domain<FieldT>  {
//   : public evaluation_domain<FieldT>
precomputation_sentinel:    bool,
subproduct_tree:    Vec<Vec<Vec<FieldT> > >,
arithmetic_sequence:    Vec<FieldT>,
arithmetic_generator:    FieldT,
m:usize,
  }
//     void do_precomputation();

//     arithmetic_sequence_domain(const size_t m);

//     void FFT(a:&Vec<FieldT>);
//     void iFFT(a:&Vec<FieldT>);
//     void cosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     void icosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(const size_t idx);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     void add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>);
//     void divide_by_Z_on_coset(P:&Vec<FieldT>);

//   };

// //} // libfqfft

// use crate::evaluation_domain::domains::arithmetic_sequence_domain.tcc;

//#endif // ARITHMETIC_SEQUENCE_DOMAIN_HPP


/** @file
 *****************************************************************************

 Implementation of interfaces for the "arithmetic sequence" evaluation domain.

 See arithmetic_sequence_domain.hpp .

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef ARITHMETIC_SEQUENCE_DOMAIN_TCC_
// #define ARITHMETIC_SEQUENCE_DOMAIN_TCC_

use crate::evaluation_domain::domains::basic_radix2_domain_aux::_multiply_by_coset;
use crate::polynomial_arithmetic::basis_change::{monomial_to_newton_basis,compute_subproduct_tree,newton_to_monomial_basis};

use crate::polynomial_arithmetic::basic_operations::_polynomial_multiplication;

use ffec::common::utils::log2;
 use num_traits::One;
use std::ops::Sub;
// #ifdef MULTICORE
//#include <omp.h>
//#endif

// //namespace libfqfft {
impl<FieldT:num_traits::Zero+Clone+ std::ops::SubAssign+ std::default::Default+std::convert::From<i32>+ std::ops::Sub<Output = FieldT>+std::convert::From<usize>+ std::ops::AddAssign+One+std::ops::Neg<Output = FieldT>+  std::ops::MulAssign+ std::ops::BitXor<usize>+ std::ops::Mul<Output = FieldT>+ std::cmp::PartialEq> arithmetic_sequence_domain<FieldT>
{
pub fn new(m:usize)->eyre::Result<Self>
{// : evaluation_domain<FieldT>(m)
  if m <= 1{ eyre::bail!("arithmetic(): expected m > 1");}
//   if FieldT::arithmetic_generator() == FieldT::zero()
//     {eyre::bail!("arithmetic(): expected FieldT::arithmetic_generator() != FieldT::zero()");}

 Ok( Self{precomputation_sentinel : false,
        subproduct_tree:    vec![],
arithmetic_sequence:   vec![],
arithmetic_generator:    FieldT::zero(),
    m,
    })

}

// 
pub fn FFT(&mut self,a:&mut Vec<FieldT>)->eyre::Result<()>
{
  if a.len() != self.m{ eyre::bail!("arithmetic: expected a.len() == self.m");}

  if !self.precomputation_sentinel { self.do_precomputation();}

  /* Monomial to Newton */
  monomial_to_newton_basis(a, &self.subproduct_tree, self.m);
  
  /* Newton to Evaluation */
  let mut  S=vec![FieldT::zero();self.m]; /* i! * arithmetic_generator */
  S[0] = FieldT::one();

  let mut  factorial = FieldT::one();
  for i in 1..self.m
  {
    // factorial *= FieldT::from(i);
    // S[i] = (factorial * self.arithmetic_generator).inverse();
  }

  _polynomial_multiplication(&mut a.clone(), a, &S);
  a.resize(self.m,FieldT::zero());

// #ifdef MULTICORE
//   //#pragma omp parallel for
//#endif
//   for i in 0..self.m
//   {
//     a[i] *= S[i].inverse();
//   }
    Ok(())
}


pub fn iFFT(&mut self,a:&mut Vec<FieldT>)->eyre::Result<()>
{
  if a.len() != self.m {eyre::bail!("arithmetic: expected a.len() == self.m");}
  
  if !self.precomputation_sentinel {self.do_precomputation();}

  /* Interpolation to Newton */
  let mut  S=vec![FieldT::zero();self.m]; /* i! * arithmetic_generator */
  S[0] = FieldT::one();

  let mut  W=vec![FieldT::zero();self.m];
  W[0] = a[0].clone() * S[0].clone();

  let mut  factorial = FieldT::one();
  for i in 1..self.m
  {
    // factorial *= FieldT(i);
    // S[i] = (factorial * self.arithmetic_generator).inverse();
    W[i] = a[i].clone() * S[i].clone();
    if i % 2 == 1 {S[i] = -S[i].clone();}
  }

  _polynomial_multiplication(a, &W, &S);
  a.resize(self.m,FieldT::zero());

  /* Newton to Monomial */
  newton_to_monomial_basis(a, &self.subproduct_tree, self.m);
    Ok(())
}


pub fn cosetFFT(&mut self,a:&mut Vec<FieldT>, g:&FieldT)->eyre::Result<()>
{
  _multiply_by_coset(a, g);
  self.FFT(a)
}


pub fn icosetFFT(&mut self,a:&mut Vec<FieldT>, g:&FieldT)->eyre::Result<()>
{
  self.iFFT(a);
//   _multiply_by_coset(a, g.inverse());
    Ok(())
}


pub fn evaluate_all_lagrange_polynomials(&mut self,t:&FieldT)->Vec<FieldT>
{
  /* Compute Lagrange polynomial of size m, with m+1 points (x_0, y_0), ... ,(x_m, y_m) */
  /* Evaluate for x = t */
  /* Return coeffs for each l_j(x) = (l / l_i[j]) * w[j] */

  if !self.precomputation_sentinel {self.do_precomputation();}

  /**
   * If t equals one of the arithmetic progression values,
   * then output 1 at the right place, and 0 elsewhere.
   */
  for i in 0..self.m
  {
    if &self.arithmetic_sequence[i] == t // i.e., t equals self.arithmetic_sequence[i]
    {
      let mut  res=vec![FieldT::zero();self.m];
      res[i] = FieldT::one();
      return res;
    }
  }

  /**
   * Otherwise, if t does not equal any of the arithmetic progression values,
   * then compute each Lagrange coefficient.
   */
  let mut  l=vec![FieldT::zero();self.m];
  let l0:FieldT = t.clone() - self.arithmetic_sequence[0].clone();
l[0] =l0;
  let mut  l_vanish = l[0].clone();
  let mut  g_vanish = FieldT::one();

  for i in 1..self.m
  {
    // l[i] = t - self.arithmetic_sequence[i];
    // l_vanish *= l[i];
    // g_vanish *= -self.arithmetic_sequence[i];
  }

  let mut  w=vec![FieldT::zero();self.m];
//   w[0] = g_vanish.inverse() * (self.arithmetic_generator^(self.m-1));
  
//   l[0] = l_vanish * l[0].inverse() * w[0];
  for i in 1..self.m
  {
    let mut  num = self.arithmetic_sequence[i-1].clone() - self.arithmetic_sequence[self.m-1].clone();
    // w[i] = w[i-1] * num * self.arithmetic_sequence[i].inverse();
    // l[i] = l_vanish * l[i].inverse() * w[i];
  }

  return l;
}


pub fn get_domain_element(&mut self,idx:usize)->FieldT
{
  if !self.precomputation_sentinel {self.do_precomputation();}

  return self.arithmetic_sequence[idx].clone();
}


pub fn compute_vanishing_polynomial(&mut self,t:&FieldT)->FieldT
{
  if !self.precomputation_sentinel{self.do_precomputation();}

  /* Notes: Z = prod_{i = 0 to m} (t - a[i]) */
  let mut  Z = FieldT::one();
  for i in 0..self.m
  {
    let tt:FieldT= t.clone() - self.arithmetic_sequence[i].clone();
    Z *= tt;
  }
  return Z;
}


pub fn add_poly_Z(&mut self,coeff:&FieldT, H:&mut Vec<FieldT>)->eyre::Result<()>
{
  if H.len() != self.m+1{ eyre::bail!("arithmetic: expected H.len() == self.m+1");}

  if !self.precomputation_sentinel{self.do_precomputation();}

  let mut  x=vec![FieldT::zero();2];
  x[0] = -self.arithmetic_sequence[0].clone();
  x[1] = FieldT::one();

  let mut  t=vec![FieldT::zero();2];

  for i in 1..self.m+1
  {
    t[0] = -self.arithmetic_sequence[i].clone();
    t[1] = FieldT::one();
    let xx=x.clone();
    _polynomial_multiplication(&mut x, &xx, &t);

  }

// #ifdef MULTICORE
//   //#pragma omp parallel for
//#endif
  for i in 0..self.m+1
  {
    H[i] += (x[i].clone() * coeff.clone());
  }
    Ok(())
}


pub fn divide_by_Z_on_coset(&self,P:&Vec<FieldT>)
{
  let coset = self.arithmetic_generator.clone(); /* coset in arithmetic sequence? */
//   let  Z_inverse_at_coset = self.compute_vanishing_polynomial(&coset).inverse();
//   for i in 0..self.m
//   {
//     P[i] *= Z_inverse_at_coset.clone();
//   }
}


pub fn do_precomputation(&mut self,)
{
  compute_subproduct_tree(log2(self.m), &mut self.subproduct_tree);

//   self.arithmetic_generator = FieldT::arithmetic_generator();

  self.arithmetic_sequence = vec![FieldT::zero();self.m];
  for i in 0..self.m
  {
    self.arithmetic_sequence[i] = self.arithmetic_generator.clone() * FieldT::from(i);
  }

  self.precomputation_sentinel = true;
}
}
// //} // libfqfft

//#endif // ARITHMETIC_SEQUENCE_DOMAIN_TCC_
