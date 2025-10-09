/** @file
 *****************************************************************************

 Declaration of public-parameter selector for the R1CS SEppzkSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// //#ifndef R1CS_SE_PPZKSNARK_PARAMS_HPP_
// // #define R1CS_SE_PPZKSNARK_PARAMS_HPP_

use ffec::algebra::curves::public_params;

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;

// 

/**
 * Below are various template aliases (used for convenience).
 */


type r1cs_se_ppzksnark_constraint_system<ppT> = r1cs_constraint_system<ffec::Fr<ppT> >;


type r1cs_se_ppzksnark_primary_input<ppT> = r1cs_primary_input<ffec::Fr<ppT> >;


type r1cs_se_ppzksnark_auxiliary_input<ppT> = r1cs_auxiliary_input<ffec::Fr<ppT> >;

// 

// //#endif // R1CS_SE_PPZKSNARK_PARAMS_HPP_
