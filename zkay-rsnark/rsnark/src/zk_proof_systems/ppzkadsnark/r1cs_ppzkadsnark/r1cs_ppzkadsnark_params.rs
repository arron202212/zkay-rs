/** @file
 *****************************************************************************

 Declaration of public-parameter selector for the R1CS ppzkADSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef R1CS_PPZKADSNARK_PARAMS_HPP_
// #define R1CS_PPZKADSNARK_PARAMS_HPP_

use ffec::algebra::curves::public_params;

use crate::relations::constraint_satisfaction_problems::r1cs::r1cs;



pub struct labelT {
// 
    label_bytes:[u8;16],
    // labelT() {};
}

/**
 * Below are various template aliases (used for convenience).
 */


type snark_pp<r1cs_ppzkadsnark_ppT> =  r1cs_ppzkadsnark_ppT::snark_pp;


type r1cs_ppzkadsnark_constraint_system<r1cs_ppzkadsnark_ppT> = r1cs_constraint_system<Fr<snark_pp<r1cs_ppzkadsnark_ppT>>>;


type r1cs_ppzkadsnark_primary_input<r1cs_ppzkadsnark_ppT> = r1cs_primary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>> >;


type r1cs_ppzkadsnark_auxiliary_input<r1cs_ppzkadsnark_ppT> = r1cs_auxiliary_input<Fr<snark_pp<r1cs_ppzkadsnark_ppT>> >;


type r1cs_ppzkadsnark_skT<r1cs_ppzkadsnark_ppT> =  r1cs_ppzkadsnark_ppT::skT;


type r1cs_ppzkadsnark_vkT<r1cs_ppzkadsnark_ppT> =  r1cs_ppzkadsnark_ppT::vkT;


type r1cs_ppzkadsnark_sigT<r1cs_ppzkadsnark_ppT> =  r1cs_ppzkadsnark_ppT::sigT;


type r1cs_ppzkadsnark_prfKeyT<r1cs_ppzkadsnark_ppT> =  r1cs_ppzkadsnark_ppT::prfKeyT;




//#endif // R1CS_PPZKADSNARK_PARAMS_HPP_

