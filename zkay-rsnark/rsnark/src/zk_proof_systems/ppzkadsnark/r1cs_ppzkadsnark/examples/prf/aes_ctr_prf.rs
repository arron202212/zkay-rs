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



// use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
// use aes::Aes128;
// use num_bigint::BigUint;
// use ark_ff::{PrimeField, BigInteger};
// use ark_bn254::Fr; // 假设对应 C++ 中的 Fr 类型

// fn get_fr_from_label(label: &[u8; 16], key: &[u8; 32]) -> Fr {
//     // 1. 初始化 AES 实例
//     // C++ 代码中使用 key.key_bytes 作为 PRF 密钥
//     let cipher_prf = Aes128::new(GenericArray::from_slice(&key[0..16]));
    
//     // 2. 生成 seed_bytes (PRF 阶段)
//     let mut seed_bytes = GenericArray::clone_from_slice(label);
//     cipher_prf.encrypt_block(&mut seed_bytes);

//     // 3. 状态扩展 (类似 CTR 模式)
//     // C++ 中使用了 key.key_bytes + 16 作为扩展密钥
//     let cipher_ext = Aes128::new(GenericArray::from_slice(&key[16..32]));
//     let mut random_bytes = Vec::with_capacity(48); // 16 * 3 = 48 bytes

//     // 将 seed_bytes 转为 BigUint 用于自增操作
//     let mut aux = BigUint::from_bytes_be(&seed_bytes);

//     for _ in 0..3 {
//         let mut block = GenericArray::clone_from_slice(&aux.to_bytes_be());
//         // 如果 BigUint 转换出的字节不足 16 位，前面补 0 (对应 C++ 的 while 循环)
//         if block.len() < 16 {
//             let mut padded = [0u8; 16];
//             let offset = 16 - block.len();
//             padded[offset..].copy_from_slice(&block);
//             block = GenericArray::from(padded);
//         }

//         let mut encrypted_block = block;
//         cipher_ext.encrypt_block(&mut encrypted_block);
//         random_bytes.extend_from_slice(&encrypted_block);

//         // aux = aux + 1
//         aux += 1u32;
//     }

//     // 4. 将 48 字节大整数对 Fr 取模
//     let total_rand_int = BigUint::from_bytes_be(&random_bytes);
    
//     // 获取 Fr 的模数 (Modulus)
//     let fr_modulus: BigUint = Fr::MODULUS.into();
//     let result_int = total_rand_int % fr_modulus;

//     // 5. 转回 Fr 类型
//     Fr::from_be_bytes_mod_order(&result_int.to_bytes_be())
// }
