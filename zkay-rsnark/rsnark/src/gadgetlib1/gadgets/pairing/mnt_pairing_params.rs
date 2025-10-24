/** @file
 *****************************************************************************

 Declaration of specializations of pairing_selector<ppT> to
 - pairing_selector<ffec::mnt4_pp>, and
 - pairing_selector<ffec::mnt6_pp>.

 See pairing_params.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef MNT_PAIRING_PARAMS_HPP_
// #define MNT_PAIRING_PARAMS_HPP_

use ffec::algebra::curves::mnt::mnt4::mnt4_pp;
use ffec::algebra::curves::mnt::mnt6::mnt6_pp;

use crate::gadgetlib1::gadgets::fields/fp2_gadgets;
use crate::gadgetlib1::gadgets::fields/fp3_gadgets;
use crate::gadgetlib1::gadgets::fields/fp4_gadgets;
use crate::gadgetlib1::gadgets::fields/fp6_gadgets;
use crate::gadgetlib1::gadgets::pairing::pairing_params;



template<typename ppT>
class mnt_e_over_e_miller_loop_gadget;

template<typename ppT>
class mnt_e_times_e_over_e_miller_loop_gadget;

template<typename ppT>
class mnt4_final_exp_gadget;

template<typename ppT>
class mnt6_final_exp_gadget;

/**
 * Specialization for MNT4.
 */
template<>
class pairing_selector<ffec::mnt4_pp> {

    type ffec::Fr<ffec::mnt4_pp> FieldT;
    type ffec::Fqe<ffec::mnt6_pp> FqeT;
    type ffec::Fqk<ffec::mnt6_pp> FqkT;

    type Fp3_variable<FqeT> Fqe_variable_type;
    type Fp3_mul_gadget<FqeT> Fqe_mul_gadget_type;
    type Fp3_mul_by_lc_gadget<FqeT> Fqe_mul_by_lc_gadget_type;
    type Fp3_sqr_gadget<FqeT> Fqe_sqr_gadget_type;

    type Fp6_variable<FqkT> Fqk_variable_type;
    type Fp6_mul_gadget<FqkT> Fqk_mul_gadget_type;
    type Fp6_mul_by_2345_gadget<FqkT> Fqk_special_mul_gadget_type;
    type Fp6_sqr_gadget<FqkT> Fqk_sqr_gadget_type;

    type ffec::mnt6_pp other_curve_type;

    type mnt_e_over_e_miller_loop_gadget<ffec::mnt4_pp> e_over_e_miller_loop_gadget_type;
    type mnt_e_times_e_over_e_miller_loop_gadget<ffec::mnt4_pp> e_times_e_over_e_miller_loop_gadget_type;
    type mnt4_final_exp_gadget<ffec::mnt4_pp> final_exp_gadget_type;

    static const constexpr ffec::bigint<ffec::mnt6_Fr::num_limbs> &pairing_loop_count = ffec::mnt6_ate_loop_count;
};

/**
 * Specialization for MNT6.
 */
template<>
class pairing_selector<ffec::mnt6_pp> {

    type ffec::Fr<ffec::mnt6_pp> FieldT;

    type ffec::Fqe<ffec::mnt4_pp> FqeT;
    type ffec::Fqk<ffec::mnt4_pp> FqkT;

    type Fp2_variable<FqeT> Fqe_variable_type;
    type Fp2_mul_gadget<FqeT> Fqe_mul_gadget_type;
    type Fp2_mul_by_lc_gadget<FqeT> Fqe_mul_by_lc_gadget_type;
    type Fp2_sqr_gadget<FqeT> Fqe_sqr_gadget_type;

    type Fp4_variable<FqkT> Fqk_variable_type;
    type Fp4_mul_gadget<FqkT> Fqk_mul_gadget_type;
    type Fp4_mul_gadget<FqkT> Fqk_special_mul_gadget_type;
    type Fp4_sqr_gadget<FqkT> Fqk_sqr_gadget_type;

    type ffec::mnt4_pp other_curve_type;

    type mnt_e_over_e_miller_loop_gadget<ffec::mnt6_pp> e_over_e_miller_loop_gadget_type;
    type mnt_e_times_e_over_e_miller_loop_gadget<ffec::mnt6_pp> e_times_e_over_e_miller_loop_gadget_type;
    type mnt6_final_exp_gadget<ffec::mnt6_pp> final_exp_gadget_type;

    static const constexpr ffec::bigint<ffec::mnt4_Fr::num_limbs> &pairing_loop_count = ffec::mnt4_ate_loop_count;
};



//#endif // MNT_PAIRING_PARAMS_HPP_
