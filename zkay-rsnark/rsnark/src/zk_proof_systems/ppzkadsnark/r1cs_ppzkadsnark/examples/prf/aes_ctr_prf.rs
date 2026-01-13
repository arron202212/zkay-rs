// AES-Based PRF for ADSNARK.
use crate::common::default_types::r1cs_ppzkadsnark_pp::default_r1cs_ppzkadsnark_pp;
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_params::{
    labelT, ppzkadsnarkConfig, r1cs_ppzkadsnark_prfKeyT, snark_pp,
};
use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf::PrfConfig;
use ff_curves::Fr;
use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct aesPrfKeyT {
    pub key_bytes: [u8; 32],
}
impl aesPrfKeyT {
    pub fn new() -> Self {
        Self { key_bytes: [0; 32] }
    }
}

pub struct PrfAdsnark<ppT: ppzkadsnarkConfig>(PhantomData<ppT>);

impl<ppT: ppzkadsnarkConfig<prfKeyT = aesPrfKeyT>> PrfConfig<ppT> for PrfAdsnark<ppT> {
    // template <>
    fn prfGen() -> r1cs_ppzkadsnark_prfKeyT<ppT> {
        let key = aesPrfKeyT::default();
        // randombytes(key.key_bytes,32);
        key
    }

    //
    fn prfCompute(key: &r1cs_ppzkadsnark_prfKeyT<ppT>, label: &labelT) -> Fr<snark_pp<ppT>> {
        // unsigned char seed_bytes[16];
        // mpz_t aux,Fr_mod;
        // unsigned char random_bytes[16*3];
        // usize exp_len;

        // mpz_init (aux);
        // mpz_init (Fr_mod);

        // // compute random seed using AES as PRF
        // crypto_core_aes128encrypt_openssl(seed_bytes,label.label_bytes,key.key_bytes,NULL);

        // // use first 128 bits of output to seed AES-CTR
        // // PRG to expand to 3*128 bits
        // crypto_core_aes128encrypt_openssl(random_bytes,seed_bytes,key.key_bytes+16,NULL);

        // mpz_import(aux, 16, 0, 1, 0, 0, seed_bytes);
        // mpz_add_ui(aux,aux,1);
        // mpz_export(seed_bytes, &exp_len, 0, 1, 0, 0, aux);
        // while (exp_len < 16)
        //     seed_bytes[exp_len++] = 0;

        // crypto_core_aes128encrypt_openssl(random_bytes+16,seed_bytes,key.key_bytes+16,NULL);

        // mpz_add_ui(aux,aux,1);
        // mpz_export(seed_bytes, &exp_len, 0, 1, 0, 0, aux);
        // while (exp_len < 16)
        //     seed_bytes[exp_len++] = 0;

        // crypto_core_aes128encrypt_openssl(random_bytes+32,seed_bytes,key.key_bytes+16,NULL);

        // // see output as integer and reduce modulo r
        // mpz_import(aux, 16*3, 0, 1, 0, 0, random_bytes);
        // Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>::mod.to_mpz(Fr_mod);
        // mpz_mod(aux,aux,Fr_mod);

        // return Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>(
        //     bigint<Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>::num_limbs>(aux));
        Fr::<snark_pp<ppT>>::default()
    }
}
