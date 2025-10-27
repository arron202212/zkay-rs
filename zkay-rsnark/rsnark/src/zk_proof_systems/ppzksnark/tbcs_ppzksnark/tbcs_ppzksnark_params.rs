/** @file
 *****************************************************************************

 Declaration of public-parameter selector for the TBCS ppzkSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef TBCS_PPZKSNARK_PARAMS_HPP_
// #define TBCS_PPZKSNARK_PARAMS_HPP_

use crate::relations::circuit_satisfaction_problems/tbcs/tbcs;



/**
 * Below are various typedefs aliases (used for uniformity with other proof systems).
 */

type tbcs_ppzksnark_circuit=tbcs_circuit;

type tbcs_ppzksnark_primary_input=tbcs_primary_input;

type tbcs_ppzksnark_auxiliary_input=tbcs_auxiliary_input;



//#endif // TBCS_PPZKSNARK_PARAMS_HPP_
