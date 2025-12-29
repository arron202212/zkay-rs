// Generic signature interface for ADSNARK.


use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params;

pub struct kpT<ppT> {
    sk: r1cs_ppzkadsnark_skT<ppT>,
    vk: r1cs_ppzkadsnark_vkT<ppT>,
}


//  pub fn sigGen( )->kpT<ppT>;


// r1cs_ppzkadsnark_sigT<ppT> sigSign(sk:&r1cs_ppzkadsnark_skT<ppT>, label:&labelT, Lambda:&ffec::G2<snark_pp<ppT>>);


// bool sigVerif(vk:&r1cs_ppzkadsnark_vkT<ppT>, label:&labelT, Lambda:&ffec::G2<snark_pp<ppT>>, sig:&r1cs_ppzkadsnark_sigT<ppT>);


// bool sigBatchVerif(vk:&r1cs_ppzkadsnark_vkT<ppT>, labels:&Vec<labelT>, Lambdas:&Vec<ffec::G2<snark_pp<ppT>>>, sigs:&Vec<r1cs_ppzkadsnark_sigT<ppT>>);


