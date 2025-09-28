/** @file
 *****************************************************************************

 This file defines default_r1cs_ppzkadsnark_pp based on the elliptic curve
 choice selected in ec_pp.hpp.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

#ifndef R1CS_PPZKADSNARK_PP_HPP_
#define R1CS_PPZKADSNARK_PP_HPP_

use  <libsnark/common/default_types/r1cs_ppzksnark_pp.hpp>
use  <libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/prf/aes_ctr_prf.hpp>
use  <libsnark/zk_proof_systems/ppzkadsnark/r1cs_ppzkadsnark/examples/signature/ed25519_signature.hpp>

namespace libsnark {

	class default_r1cs_ppzkadsnark_pp {
	public:
		type default_r1cs_ppzksnark_pp snark_pp;
		type ed25519_skT skT;
		type ed25519_vkT vkT;
    	type ed25519_sigT sigT;
    	type aesPrfKeyT prfKeyT;

    	static void init_public_params();
	};

};  // libsnark

#endif // R1CS_PPZKADSNARK_PP_HPP_
/** @file
 *****************************************************************************

 This file provides the initialization methods for the default ADSNARK params.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

use  <libsnark/common/default_types/r1cs_ppzkadsnark_pp.hpp>

namespace libsnark {

void default_r1cs_ppzkadsnark_pp::init_public_params()
{
    snark_pp::init_public_params();
}

} // libsnark
