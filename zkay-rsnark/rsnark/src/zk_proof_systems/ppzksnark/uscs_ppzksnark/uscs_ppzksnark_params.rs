/** @file
 *****************************************************************************

 Declaration of public-parameter selector for the USCS ppzkSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef USCS_PPZKSNARK_PARAMS_HPP_
// #define USCS_PPZKSNARK_PARAMS_HPP_

use ffec::algebra::curves::public_params;

use libsnark/relations/constraint_satisfaction_problems/uscs/uscs;



/**
 * Below are various template aliases (used for convenience).
 */

template<typename ppT>
using uscs_ppzksnark_constraint_system = uscs_constraint_system<ffec::Fr<ppT> >;

template<typename ppT>
using uscs_ppzksnark_primary_input = uscs_primary_input<ffec::Fr<ppT> >;

template<typename ppT>
using uscs_ppzksnark_auxiliary_input = uscs_auxiliary_input<ffec::Fr<ppT> >;



//#endif // USCS_PPZKSNARK_PARAMS_HPP_
