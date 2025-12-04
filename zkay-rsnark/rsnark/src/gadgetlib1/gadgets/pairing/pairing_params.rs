//  Declaration of selector for the pairing gadget.

/**
 * The interfaces of pairing gadgets are templatized via the parameter
 * ec_ppT. When used, the interfaces must be invoked with
 * a particular parameter choice; let 'my_ec_pp' denote this choice.
 *
 * Moreover, one must provide a template specialization for the class
 * pairing_selector (below), containing typedefs for the typenames
 * - FieldT
 * - FqeT
 * - FqkT
 * - Fqe_variable_type;
 * - Fqe_mul_gadget_type
 * - Fqe_mul_by_lc_gadget_type
 * - Fqe_sqr_gadget_type
 * - Fqk_variable_type
 * - Fqk_mul_gadget_type
 * - Fqk_special_mul_gadget_type
 * - Fqk_sqr_gadget_type
 * - other_curve_type
 * - e_over_e_miller_loop_gadget_type
 * - e_times_e_over_e_miller_loop_gadget_type
 * - final_exp_gadget_type
 * and also containing a static constant
 * - const constexpr ffec::bigint<m> pairing_loop_count
 *
 * For example, if you want to use the types my_Field, my_Fqe, etc,
 * then you would do as follows. First declare a new type:
 *
 *   pub struct my_ec_pp;
 *
 * Second, specialize pairing_selector<ec_ppT> for the
 * case ec_ppT = my_ec_pp, type  the above types:
 *
 *   
 *   pub struct pairing_selector<my_ec_pp> {
 *       type FieldT=my_Field;
 *       type FqeT=my_Fqe;
 *       type FqkT=my_Fqk;
 *       type Fqe_variable_type=my_Fqe_variable_type;
 *       type Fqe_mul_gadget_type=my_Fqe_mul_gadget_type;
 *       type Fqe_mul_by_lc_gadget_type=my_Fqe_mul_by_lc_gadget_type;
 *       type Fqe_sqr_gadget_type=my_Fqe_sqr_gadget_type;
 *       type Fqk_variable_type=my_Fqk_variable_type;
 *       type Fqk_mul_gadget_type=my_Fqk_mul_gadget_type;
 *       type Fqk_special_mul_gadget_type=my_Fqk_special_mul_gadget_type;
 *       type Fqk_sqr_gadget_type=my_Fqk_sqr_gadget_type;
 *       type other_curve_type=my_other_curve_type;
 *       type e_over_e_miller_loop_gadget_type=my_e_over_e_miller_loop_gadget_type;
 *       type e_times_e_over_e_miller_loop_gadget_type=my_e_times_e_over_e_miller_loop_gadget_type;
 *       type final_exp_gadget_type=my_final_exp_gadget_type;
 *       static pairing_loop_count:&constexpr ffec::bigint<...> = ...;
 *   };
 *
 * Having done the above, my_ec_pp can be used as a template parameter.
 *
 * See mnt_pairing_params.hpp for examples for the case of fixing
 * ec_ppT to "MNT4" and "MNT6".
 *
 */

// pub struct pairing_selector;

/**
 * Below are various template aliases (used for convenience).
 */
pub trait pairing_selector<my_ec_pp> {
    type FieldT;
    type FqeT;
    type FqkT;
    type Fqe_variable_type;
    type Fqe_mul_gadget_type;
    type Fqe_mul_by_lc_gadget_type;
    type Fqe_sqr_gadget_type;
    type Fqk_variable_type;
    type Fqk_mul_gadget_type;
    type Fqk_special_mul_gadget_type;
    type Fqk_sqr_gadget_type;
    type other_curve_type;
    type e_over_e_miller_loop_gadget_type;
    type e_times_e_over_e_miller_loop_gadget_type;
    type final_exp_gadget_type;
    const pairing_loop_count: u128;
}

pub type FqkT<ppT, P> = <P as pairing_selector<ppT>>::FqkT; // TODO: better name when stable
pub type Fqe_variable<ppT, P> = <P as pairing_selector<ppT>>::Fqe_variable_type;
pub type Fqe_mul_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqe_mul_gadget_type;
pub type Fqe_mul_by_lc_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqe_mul_by_lc_gadget_type;
pub type Fqe_sqr_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqe_sqr_gadget_type;
pub type Fqk_variable<ppT, P> = <P as pairing_selector<ppT>>::Fqk_variable_type;
pub type Fqk_mul_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqk_mul_gadget_type;
pub type Fqk_special_mul_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqk_special_mul_gadget_type;
pub type Fqk_sqr_gadget<ppT, P> = <P as pairing_selector<ppT>>::Fqk_sqr_gadget_type;
pub type other_curve<ppT, P> = <P as pairing_selector<ppT>>::other_curve_type;
pub type e_over_e_miller_loop_gadget<ppT, P> =
    <P as pairing_selector<ppT>>::e_over_e_miller_loop_gadget_type;
pub type e_times_e_over_e_miller_loop_gadget<ppT, P> =
    <P as pairing_selector<ppT>>::e_times_e_over_e_miller_loop_gadget_type;
pub type final_exp_gadget<ppT, P> = <P as pairing_selector<ppT>>::final_exp_gadget_type;

//#endif // PAIRING_PARAMS_HPP_
