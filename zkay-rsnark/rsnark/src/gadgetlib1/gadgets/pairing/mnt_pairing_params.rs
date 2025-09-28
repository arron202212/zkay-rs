/** @file
 *****************************************************************************

 Declaration of specializations of pairing_selector<ppT> to
 - pairing_selector<libff::mnt4_pp>, and
 - pairing_selector<libff::mnt6_pp>.

 See pairing_params.hpp .

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef MNT_PAIRING_PARAMS_HPP_
#define MNT_PAIRING_PARAMS_HPP_

use  <libff/algebra/curves/mnt/mnt4/mnt4_pp.hpp>
use  <libff/algebra/curves/mnt/mnt6/mnt6_pp.hpp>

use  <libsnark/gadgetlib1/gadgets/fields/fp2_gadgets.hpp>
use  <libsnark/gadgetlib1/gadgets/fields/fp3_gadgets.hpp>
use  <libsnark/gadgetlib1/gadgets/fields/fp4_gadgets.hpp>
use  <libsnark/gadgetlib1/gadgets/fields/fp6_gadgets.hpp>
use  <libsnark/gadgetlib1/gadgets/pairing/pairing_params.hpp>

namespace libsnark {

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
class pairing_selector<libff::mnt4_pp> {
public:
    type libff::Fr<libff::mnt4_pp> FieldT;
    type libff::Fqe<libff::mnt6_pp> FqeT;
    type libff::Fqk<libff::mnt6_pp> FqkT;

    type Fp3_variable<FqeT> Fqe_variable_type;
    type Fp3_mul_gadget<FqeT> Fqe_mul_gadget_type;
    type Fp3_mul_by_lc_gadget<FqeT> Fqe_mul_by_lc_gadget_type;
    type Fp3_sqr_gadget<FqeT> Fqe_sqr_gadget_type;

    type Fp6_variable<FqkT> Fqk_variable_type;
    type Fp6_mul_gadget<FqkT> Fqk_mul_gadget_type;
    type Fp6_mul_by_2345_gadget<FqkT> Fqk_special_mul_gadget_type;
    type Fp6_sqr_gadget<FqkT> Fqk_sqr_gadget_type;

    type libff::mnt6_pp other_curve_type;

    type mnt_e_over_e_miller_loop_gadget<libff::mnt4_pp> e_over_e_miller_loop_gadget_type;
    type mnt_e_times_e_over_e_miller_loop_gadget<libff::mnt4_pp> e_times_e_over_e_miller_loop_gadget_type;
    type mnt4_final_exp_gadget<libff::mnt4_pp> final_exp_gadget_type;

    static const constexpr libff::bigint<libff::mnt6_Fr::num_limbs> &pairing_loop_count = libff::mnt6_ate_loop_count;
};

/**
 * Specialization for MNT6.
 */
template<>
class pairing_selector<libff::mnt6_pp> {
public:
    type libff::Fr<libff::mnt6_pp> FieldT;

    type libff::Fqe<libff::mnt4_pp> FqeT;
    type libff::Fqk<libff::mnt4_pp> FqkT;

    type Fp2_variable<FqeT> Fqe_variable_type;
    type Fp2_mul_gadget<FqeT> Fqe_mul_gadget_type;
    type Fp2_mul_by_lc_gadget<FqeT> Fqe_mul_by_lc_gadget_type;
    type Fp2_sqr_gadget<FqeT> Fqe_sqr_gadget_type;

    type Fp4_variable<FqkT> Fqk_variable_type;
    type Fp4_mul_gadget<FqkT> Fqk_mul_gadget_type;
    type Fp4_mul_gadget<FqkT> Fqk_special_mul_gadget_type;
    type Fp4_sqr_gadget<FqkT> Fqk_sqr_gadget_type;

    type libff::mnt4_pp other_curve_type;

    type mnt_e_over_e_miller_loop_gadget<libff::mnt6_pp> e_over_e_miller_loop_gadget_type;
    type mnt_e_times_e_over_e_miller_loop_gadget<libff::mnt6_pp> e_times_e_over_e_miller_loop_gadget_type;
    type mnt6_final_exp_gadget<libff::mnt6_pp> final_exp_gadget_type;

    static const constexpr libff::bigint<libff::mnt4_Fr::num_limbs> &pairing_loop_count = libff::mnt4_ate_loop_count;
};

} // libsnark

#endif // MNT_PAIRING_PARAMS_HPP_
