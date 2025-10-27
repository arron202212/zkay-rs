/** @file
 *****************************************************************************

 Declaration of interfaces for the "basic radix-2" evaluation domain.

 Roughly, the domain has size m = 2^k and consists of the m-th roots of unity.

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_RADIX2_DOMAIN_HPP_
// #define BASIC_RADIX2_DOMAIN_HPP_

//#include <vector>

use crate::evaluation_domain::evaluation_domain;

//namespace libfqfft {

// 
pub struct basic_radix2_domain<FieldT> {
// : public evaluation_domain<FieldT> 

     pub omega:FieldT,
    pub m:usize,
}

//     basic_radix2_domain(m:usize);

//     pub fn  FFT(a:&Vec<FieldT>);
//     pub fn  iFFT(a:&Vec<FieldT>);
//     pub fn  cosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     pub fn  icosetFFT(a:&Vec<FieldT>, g:&FieldT);
//     Vec<FieldT> evaluate_all_lagrange_polynomials(t:&FieldT);
//     FieldT get_domain_element(idx:usize);
//     FieldT compute_vanishing_polynomial(t:&FieldT);
//     pub fn  add_poly_Z(coeff:&FieldT, H:&Vec<FieldT>);
//     pub fn  divide_by_Z_on_coset(P:&Vec<FieldT>);

// };

// //} // libfqfft

// use crate::evaluation_domain::domains::basic_radix2_domain.tcc;

//#endif // BASIC_RADIX2_DOMAIN_HPP_
/** @file
 *****************************************************************************

 Implementation of interfaces for the "basic radix-2" evaluation domain.

 See basic_radix2_domain.hpp .

 *****************************************************************************
 * @author     This file is part of libfqfft, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef BASIC_RADIX2_DOMAIN_TCC_
// #define BASIC_RADIX2_DOMAIN_TCC_

use ffec::algebra::field_utils::field_utils;
use ffec::common::double;
use ffec::common::utils::log2;
use crate::evaluation_domain::domains::basic_radix2_domain_aux::{ _basic_radix2_FFT,_basic_radix2_evaluate_all_lagrange_polynomials,_multiply_by_coset};
use ffec::algebra::field_utils::field_utils::get_root_of_unity_is_same_double;
use num_traits::One;
use std::ops::BitXor;
 use std::ops::Sub;
//namespace libfqfft {
impl<FieldT: std::default::Default + std::ops::Sub+One+ std::ops::MulAssign+ std::cmp::PartialEq+Clone+std::ops::Sub<Output = FieldT>+ std::ops::MulAssign+ std::ops::BitXor<Output = FieldT>+ std::convert::From<usize>+ num_traits::Zero+ std::ops::SubAssign+ std::ops::AddAssign<FieldT>>  basic_radix2_domain<FieldT> 
{

pub fn new( m:usize)->eyre::Result<Self>
{// : evaluation_domain<FieldT>(m)
    if m <= 1{eyre::bail!("basic_radix2(): expected m > 1");}

    if "FieldT"!= "Double"
    {
       let  logm = log2(m);
        // if logm > (FieldT::s){eyre::bail!("basic_radix2(): expected logm <= FieldT::s");}
    }

   Ok( Self { 
        omega : get_root_of_unity_is_same_double::<FieldT>(m),
        m,
        })
    // catch (const std::invalid_argument& e) { throw DomainSizeException(e.what()); }
}


pub fn FFT(&self,a:&mut Vec<FieldT>)->eyre::Result<()>
{
    if a.len() != self.m{eyre::bail!("basic_radix2: expected a.len() == self.m");}

    _basic_radix2_FFT(a, &self.omega);
    Ok(())
}


pub fn iFFT(&self,a:&Vec<FieldT>)->eyre::Result<()>
{
    if a.len() != self.m{eyre::bail!("basic_radix2: expected a.len() == self.m");}

    // _basic_radix2_FFT(a, self.omega.inverse());

    // let  sconst = FieldT::from(a.len()).inverse();
    // for i in 0..a.len()
    // {
    //     a[i] *= sconst;
    // }
    Ok(())
}


pub fn cosetFFT(&self,a:&mut Vec<FieldT>, g:&FieldT)->eyre::Result<()>
{
    _multiply_by_coset(a, g);
    self.FFT(a)
}


pub fn icosetFFT(&self,a:&Vec<FieldT>, g:&FieldT)->eyre::Result<()>
{
    self.iFFT(a);
    // _multiply_by_coset(a, g.inverse());
    Ok(())
}


pub fn evaluate_all_lagrange_polynomials(&self,t:&FieldT)->Vec<FieldT>
{
    return _basic_radix2_evaluate_all_lagrange_polynomials(self.m, t).unwrap();
}


pub fn get_domain_element(&self, idx:usize)->FieldT
{
    return self.omega.clone()^FieldT::from(idx);
}


pub fn compute_vanishing_polynomial(&self,t:&FieldT)->FieldT
{
    let tt:FieldT=t.clone();
    let mm:FieldT=self.m.into();
    let tm:FieldT=tt^mm;
     tm - FieldT::one()
}


pub fn add_poly_Z(&self,coeff:&FieldT, H:&mut Vec<FieldT>)->eyre::Result<()>
{
    if H.len() != self.m+1{eyre::bail!("basic_radix2: expected H.len() == self.m+1");}

    H[self.m] += coeff.clone();
    H[0] -= coeff.clone();
    Ok(())
}


pub fn divide_by_Z_on_coset(&self,P:&Vec<FieldT>)
{
    // let  coset = FieldT::multiplicative_generator.clone();
    // let  Z_inverse_at_coset = self.compute_vanishing_polynomial(coset).inverse();
    // for i in 0..self.m
    // {
    //     P[i] *= Z_inverse_at_coset;
    // }
}
}
//} // libfqfft

//#endif // BASIC_RADIX2_DOMAIN_TCC_
