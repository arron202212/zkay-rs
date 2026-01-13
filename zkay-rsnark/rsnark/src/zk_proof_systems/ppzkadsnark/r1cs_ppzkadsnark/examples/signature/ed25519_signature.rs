// Fast batch verification signature for ADSNARK.

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    SigTConfig, VkTConfig, labelT, ppzkadsnarkConfig, r1cs_ppzkadsnark_sigT, r1cs_ppzkadsnark_skT,
    r1cs_ppzkadsnark_vkT, snark_pp,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature::{
    SigConfig, kpT,
};
use ff_curves::G2;
use ffec::PpConfig;
use std::marker::PhantomData;

impl SigTConfig for ed25519_sigT {
    fn sig_bytes(&self) -> &[u8] {
        &self.sig_bytes
    }
    fn sig_bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.sig_bytes
    }
}
impl VkTConfig for ed25519_vkT {
    fn vk_bytes(&self) -> &[u8] {
        &self.vk_bytes
    }
}
#[derive(Default, Clone)]
pub struct ed25519_sigT {
    pub sig_bytes: Vec<u8>, //[u8; 64],
}
#[derive(Default, Clone)]
pub struct ed25519_vkT {
    pub vk_bytes: Vec<u8>, //[u8; 32],
}
#[derive(Default, Clone)]
pub struct ed25519_skT {
    pub sk_bytes: Vec<u8>, //[u8; 64],
}

// use  "depends/libsnark-supercop/include/crypto_sign.h"
// use crate::common::default_types::r1cs_ppzkadsnark_pp;

pub struct Ed25519<ppT: ppzkadsnarkConfig>(PhantomData<ppT>);

impl<ppT: ppzkadsnarkConfig> SigConfig<ppT> for Ed25519<ppT> {
    fn sigGen() -> kpT<ppT> {
        let keys = kpT::<ppT>::default();
        // crypto_sign_ed25519_amd64_51_30k_keypair(keys.vk.vk_bytes,keys.sk.sk_bytes);
        return keys;
    }

    fn sigSign(
        sk: &r1cs_ppzkadsnark_skT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
    ) -> r1cs_ppzkadsnark_sigT<ppT> {
        let mut sigma = r1cs_ppzkadsnark_sigT::<ppT>::default();
        let mut sigmalen = 0;
        let mut signature = vec![0; 64 + 16 + 320];
        let mut message = vec![0; 16 + 320];

        let mut Lambda_copy = G2::<snark_pp<ppT>>::from(Lambda.clone());
        Lambda_copy.to_affine_coordinates();

        for i in 0..16 {
            message[i] = label.label_bytes[i];
        }

        // More efficient way to get canonical point rep?
        let mut stream = String::new();
        // stream.rdbuf()->pubsetbuf(((char*)message)+16, 320);
        // stream << Lambda_copy;
        // let  written = stream.tellp();
        // while (written<320)
        // 	message[16+written++] = 0;

        // crypto_sign_ed25519_amd64_51_30k(signature, &sigmalen, message, 16 + 320, sk.sk_bytes);

        assert!(sigmalen == 64 + 16 + 320);

        for i in 0..64 {
            sigma.sig_bytes_mut()[i] = signature[i];
        }

        sigma
    }

    //
    fn sigVerif(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        label: &labelT,
        Lambda: &G2<snark_pp<ppT>>,
        sig: &r1cs_ppzkadsnark_sigT<ppT>,
    ) -> bool {
        let mut msglen = 0;
        let mut message = vec![0; 64 + 16 + 320];
        let mut signature = vec![0; 64 + 16 + 320];

        let mut Lambda_copy = G2::<snark_pp<ppT>>::from(Lambda.clone());
        Lambda_copy.to_affine_coordinates();

        for i in 0..64 {
            signature[i] = sig.sig_bytes()[i];
        }

        for i in 0..16 {
            signature[64 + i] = label.label_bytes[i];
        }

        // More efficient way to get canonical point rep?
        let mut stream = String::new();
        // stream.rdbuf()->pubsetbuf(((char*)signature)+64+16, 320);
        // stream << Lambda_copy;
        // usize written = stream.tellp();
        // while (written<320)
        // 	signature[64+16+written++] = 0;

        let res = 0;
        //     crypto_sign_ed25519_amd64_51_30k_open(
        //     message,
        //     &msglen,
        //     signature,
        //     64 + 16 + 320,
        //     vk.vk_bytes,
        // );
        res == 0
    }

    //
    fn sigBatchVerif(
        vk: &r1cs_ppzkadsnark_vkT<ppT>,
        labels: &Vec<labelT>,
        Lambdas: &Vec<G2<snark_pp<ppT>>>,
        sigs: &Vec<r1cs_ppzkadsnark_sigT<ppT>>,
    ) -> bool {
        let mut stream = String::new();

        assert!(labels.len() == Lambdas.len());
        assert!(labels.len() == sigs.len());

        let mut msglen = vec![0u64; labels.len()];
        let mut siglen = vec![0u64; labels.len()];
        let mut messages = vec![0; labels.len()];
        let mut signatures = vec![0; labels.len()];
        let mut pks = vec![vec![]; labels.len()];

        let mut pk_copy = vec![0; 32];
        for i in 0..32 {
            pk_copy[i] = vk.vk_bytes()[i];
        }

        let mut messagemem = vec![0; labels.len() * (64 + 16 + 320)];
        // assert!(messagemem != NULL);
        let mut signaturemem = vec![0; labels.len() * (64 + 16 + 320)];
        // assert!(signaturemem != NULL);

        for i in 0..labels.len() {
            siglen[i] = 64 + 16 + 320;
            messages[i] = messagemem[(64 + 16 + 320) * i];
            signatures[i] = signaturemem[(64 + 16 + 320) * i];
            pks[i] = pk_copy.clone();

            for j in 0..64 {
                signaturemem[i * (64 + 16 + 320) + j] = sigs[i].sig_bytes()[j];
            }

            for j in 0..16 {
                signaturemem[i * (64 + 16 + 320) + 64 + j] = labels[i].label_bytes[j];
            }

            // More efficient way to get canonical point rep?
            let mut Lambda_copy = G2::<snark_pp<ppT>>::from(Lambdas[i].clone());
            Lambda_copy.to_affine_coordinates();
            // stream.clear();
            // stream.rdbuf()->pubsetbuf((char*)(signaturemem+i*(64+16+320)+64+16), 320);
            // stream << Lambda_copy;
            // usize written = stream.tellp();
            // while (written<320)
            //     {signaturemem[i*(64+16+320)+64+16+written++] = 0;}
        }

        let res = 0;
        // crypto_sign_ed25519_amd64_51_30k_open_batch(
        //     messages,
        //     msglen,
        //     signatures,
        //     siglen,
        //     pks,
        //     labels.len(),
        // );

        res == 0
    }
}
