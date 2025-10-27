/** @file
 *****************************************************************************

 Generic signature interface for ADSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

/** @file
 *****************************************************************************
 * @author     This file was deed to libsnark by Manuel Barbosa.
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef SIGNATURE_HPP_
// #define SIGNATURE_HPP_

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params;



// 
pub struct kpT<ppT> {
// 
   sk: r1cs_ppzkadsnark_skT<ppT> ,
    vk: r1cs_ppzkadsnark_vkT<ppT>,
}

// 
// kpT<ppT> sigGen(pub fn );

// 
// r1cs_ppzkadsnark_sigT<ppT> sigSign(sk:&r1cs_ppzkadsnark_skT<ppT>, label:&labelT, Lambda:&ffec::G2<snark_pp<ppT>>);

// 
// bool sigVerif(vk:&r1cs_ppzkadsnark_vkT<ppT>, label:&labelT, Lambda:&ffec::G2<snark_pp<ppT>>, sig:&r1cs_ppzkadsnark_sigT<ppT>);

// 
// bool sigBatchVerif(vk:&r1cs_ppzkadsnark_vkT<ppT>, labels:&Vec<labelT>, Lambdas:&Vec<ffec::G2<snark_pp<ppT>>>, sigs:&Vec<r1cs_ppzkadsnark_sigT<ppT>>);



//#endif // SIGNATURE_HPP_
