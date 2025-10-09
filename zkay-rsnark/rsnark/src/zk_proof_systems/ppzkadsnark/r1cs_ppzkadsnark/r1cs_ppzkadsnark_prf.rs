/** @file
 *****************************************************************************

 Generic PRF interface for ADSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef PRF_HPP_
// #define PRF_HPP_

use libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/r1cs_ppzkadsnark_params;



template <typename ppT>
r1cs_ppzkadsnark_prfKeyT<ppT> prfGen();

template<typename ppT>
ffec::Fr<snark_pp<ppT>> prfCompute(const r1cs_ppzkadsnark_prfKeyT<ppT> &key, const labelT &label);



//#endif // PRF_HPP_
