/** @file
*****************************************************************************

Declaration of specializations of pairing_selector<ppT> to
- pairing_selector<mnt4_pp>, and
- pairing_selector<mnt6_pp>.

See pairing_params.hpp .

*****************************************************************************
* @author     This file is part of libsnark, developed by SCIPR Lab
*             and contributors (see AUTHORS).
* @copyright  MIT license (see LICENSE file)
*****************************************************************************/
//#ifndef MNT_PAIRING_PARAMS_HPP_
// #define MNT_PAIRING_PARAMS_HPP_
use ff_curves::algebra::curves::mnt::mnt4::mnt4_pp;
use ff_curves::algebra::curves::mnt::mnt6::mnt6_pp;

use crate::gadgetlib1::gadgets::fields::fp2_gadgets;
use crate::gadgetlib1::gadgets::fields::fp3_gadgets::{Fp3_variable,Fp3_mul_gadget,Fp3_mul_by_lc_gadget,Fp3_sqr_gadget};
use crate::gadgetlib1::gadgets::fields::fp4_gadgets;
use crate::gadgetlib1::gadgets::fields::fp6_gadgets::{Fp6_variable,Fp6_mul_gadget,Fp6_mul_by_2345_gadget,Fp6_sqr_gadget};
use crate::gadgetlib1::gadgets::pairing::pairing_params;
use std::marker::PhantomData;
pub struct FrT<T>(PhantomData<T>);
pub type Fr<T>=FrT<T>;
pub struct FqeT<T>(PhantomData<T>);
pub type Fqe<T>=FqeT<T>;
pub struct FqkT<T>(PhantomData<T>);
pub type Fqk<T>=FqkT<T>;
/**
 * Specialization for MNT4.
 */

pub trait  pairing_selector4<mnt4_pp,mnt6_pp>  {
    type FieldT = Fr<mnt4_pp>;
    type FqeT = Fqe<mnt6_pp>;
    type FqkT = Fqk<mnt6_pp>;

    type Fqe_variable_type = Fp3_variable<FqeT>;
    type Fqe_mul_gadget_type = Fp3_mul_gadget<FqeT>;
    type Fqe_mul_by_lc_gadget_type = Fp3_mul_by_lc_gadget<FqeT>;
    type Fqe_sqr_gadget_type = Fp3_sqr_gadget<FqeT>;

    type Fqk_variable_type = Fp6_variable<FqkT>;
    type Fqk_mul_gadget_type = Fp6_mul_gadget<FqkT>;
    type Fqk_special_mul_gadget_type = Fp6_mul_by_2345_gadget<FqkT>;
    type Fqk_sqr_gadget_type = Fp6_sqr_gadget<FqkT>;

    type other_curve_type = mnt6_pp;

    type e_over_e_miller_loop_gadget_type = mnt_e_over_e_miller_loop_gadget<mnt4_pp>;
    type e_times_e_over_e_miller_loop_gadget_type =
        mnt_e_times_e_over_e_miller_loop_gadget<mnt4_pp>;
    type final_exp_gadget_type = mnt4_final_exp_gadget<mnt4_pp>;

    // static pairing_loop_count:&constexpr bigint<mnt6_Fr::num_limbs> = mnt6_ate_loop_count;
}

/**
 * Specialization for MNT6.
 */

pub trait pairing_selector6<mnt6_pp,mnt4_pp> {
    type FieldT = Fr<mnt6_pp>;

    type FqeT = Fqe<mnt4_pp>;
    type FqkT = Fqk<mnt4_pp>;

    type Fqe_variable_type = Fp2_variable<FqeT>;
    type Fqe_mul_gadget_type = Fp2_mul_gadget<FqeT>;
    type Fqe_mul_by_lc_gadget_type = Fp2_mul_by_lc_gadget<FqeT>;
    type Fqe_sqr_gadget_type = Fp2_sqr_gadget<FqeT>;

    type Fqk_variable_type = Fp4_variable<FqkT>;
    type Fqk_mul_gadget_type = Fp4_mul_gadget<FqkT>;
    type Fqk_special_mul_gadget_type = Fp4_mul_gadget<FqkT>;
    type Fqk_sqr_gadget_type = Fp4_sqr_gadget<FqkT>;

    type other_curve_type = mnt4_pp;

    type e_over_e_miller_loop_gadget_type = mnt_e_over_e_miller_loop_gadget<mnt6_pp>;
    type e_times_e_over_e_miller_loop_gadget_type =
        mnt_e_times_e_over_e_miller_loop_gadget<mnt6_pp>;
    type final_exp_gadget_type = mnt6_final_exp_gadget<mnt6_pp>;

    // static pairing_loop_count:&constexpr bigint<mnt4_Fr::num_limbs> = mnt4_ate_loop_count;
}

//#endif // MNT_PAIRING_PARAMS_HPP_
