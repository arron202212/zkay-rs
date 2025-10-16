// /** @file
//  *****************************************************************************

//  Declaration of complex domain data type.

//  *****************************************************************************
//  * @author     This file is part of libff, developed by SCIPR Lab
//  *             and contributors (see AUTHORS).
//  * @copyright  MIT license (see LICENSE file)
//  *****************************************************************************/

// #ifndef DOUBLE_HPP_
// #define DOUBLE_HPP_

// #include <complex>
use num_complex::{Complex,ComplexFloat};

// #include <libff/algebra/fields/bigint.hpp>

// namespace libff {

  pub struct Double 
  {
//     public:
       val:Complex<f64>,
  }

//       Double();

//       Double(f64 real);

//       Double(f64 real, f64 imag);

//       Double(Complex<f64> num);

//       static unsigned add_cnt;
//       static unsigned sub_cnt;
//       static unsigned mul_cnt;
//       static unsigned inv_cnt;

//       Double operator+(const Double &other) const;
//       Double operator-(const Double &other) const;
//       Double operator*(const Double &other) const;
//       Double operator-() const;

//       Double& operator+=(const Double &other);
//       Double& operator-=(const Double &other);
//       Double& operator*=(const Double &other);

//       bool operator==(const Double &other) const;
//       bool operator!=(const Double &other) const;

//       bool operator<(const Double &other) const;
//       bool operator>(const Double &other) const;

//       Double operator^(const bigint<1> power) const;
//       Double operator^(const size_t power) const;

//       bigint<1> as_bigint() const;
//       unsigned long as_ulong() const;
//       Double inverse() const;
//       Double squared() const;

//       static Double one();
//       static Double zero();
//       static Double random_element();
//       static Double geometric_generator();
//       static Double arithmetic_generator();

//       static Double multiplicative_generator;
//       static Double root_of_unity; // See get_root_of_unity() in field_utils
//       static size_t s;
//   };
// } // libff

// #endif // DOUBLE_HPP_

/** @file
 *****************************************************************************
 Implementation of complex domain data type.
 *****************************************************************************
 * @author     This file is part of libff, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/
//#include <cmath>
//#include <complex>

//#include <math.h>

use crate::algebra::field_utils::bigint::bigint;
// use crate::common::f64;

// namespace libff {

// using std::size_t;
impl Double{
pub fn new()->Self
{
   Self{ val : Complex::<f64>::new(0.0,0.0)}
}

pub fn new_real( real:f64)->Self
{
   Self{ val : Complex::<f64>::new(real, 0.0)}
}

pub fn new_real_imag( real:f64,  imag:f64)->Self
{
   Self{ val : Complex::<f64>::new(real, imag)}
}

pub fn new_complex( num:Complex<f64>)->Self
{
   Self {val : num}
}

// unsigned pub fn add_cnt = 0;
// unsigned pub fn sub_cnt = 0;
// unsigned pub fn mul_cnt = 0;
// unsigned pub fn inv_cnt = 0;

pub fn inverse(&self) ->Self
{
// #ifdef PROFILE_OP_COUNTS
    // ++inv_cnt;
//#endif

    return Self::new_complex(Complex::<f64>::new(1.0,0.0) / self.val.clone());
}

pub fn as_bigint(&self) ->bigint<1> 
{
    return bigint::<{1}>::new( self.val.re() as u64);
}

pub fn as_ulong(&self) ->u64
{
      self.val.re().round() as u64
}

pub fn squared(&self) ->Self
{
    return Self::new_complex(self.val.clone() * self.val.clone());
}

pub fn one()->Self
{
    return Self::new_real(1.0);
}

pub fn zero()->Self
{
    return Self::new_real(0.0);
}

pub fn random_element()->Self
{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    return Self::new_real((rng.r#gen::<i64>()  % 1001)  as f64);
}

pub fn geometric_generator()->Self
{
    return Self::new_real(2.0);
}

pub fn arithmetic_generator()->Self
{
    return Self::new_real(1.0);
}

// pub fn multiplicative_generator = Double(2);

// } // namespace libff
}



// pub fn operator+(const Double &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++add_cnt;
// //#endif

//     return Self::new(val + other.val);
// }

// pub fn operator-(const Double &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++sub_cnt;
// //#endif

//     return Self::new(val - other.val);
// }

// pub fn operator*(const Double &other) const
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++mul_cnt;
// //#endif

//     return Self::new(val * other.val);
// }

// pub fn operator-() const
// {
//     if val.imag() == 0 {
//         return Self::new(-val.real());
//     }

//     return Self::new(-val.real(), -val.imag());
// }

// Double& pub fn operator+=(const Double &other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++add_cnt;
// //#endif

//     this->val = Complex::<f64>::new(val + other.val);
//     return *this;
// }

// Double& pub fn operator-=(const Double &other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++sub_cnt;
// //#endif

//     this->val = Complex::<f64>::new(val - other.val);
//     return *this;
// }

// Double& pub fn operator*=(const Double &other)
// {
// // #ifdef PROFILE_OP_COUNTS
//     ++mul_cnt;
// //#endif

//     this->val *= Complex::<f64>::new(other.val);
//     return *this;
// }

// bool pub fn operator==(const Double &other) const
// {
//     return (std::abs(val.real() - other.val.real()) < 0.000001)
//         && (std::abs(val.imag() - other.val.imag()) < 0.000001);
// }

// bool pub fn operator!=(const Double &other) const
// {
//     return !(*this == other);
// }

// bool pub fn operator<(const Double &other) const
// {
//     return (val.real() < other.val.real());
// }

// bool pub fn operator>(const Double &other) const
// {
//     return (val.real() > other.val.real());
// }

// pub fn operator^(const bigint<1> power) const
// {
//     return Self::new(pow(val, power.as_ulong()));
// }

// pub fn operator^(const size_t power) const
// {
//     return Self::new(pow(val, power));
// }
