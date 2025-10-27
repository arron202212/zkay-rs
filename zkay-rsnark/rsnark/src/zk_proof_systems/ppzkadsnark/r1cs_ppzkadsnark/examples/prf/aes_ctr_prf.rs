/** @file
 *****************************************************************************

 AES-Based PRF for ADSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

//#ifndef AESCTRPRF_HPP_
// #define AESCTRPRF_HPP_

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_prf;



pub struct  aesPrfKeyT {
// 
    key_bytes:[u8;32],
}
impl aesPrfKeyT{
pub fn new()->Self{
    Self{key_bytes:[0;32]}}}


//#endif // AESCTRPRF_HPP_
/** @file
 *****************************************************************************

 AES-Based PRF for ADSNARK.

 *****************************************************************************
 * @author     This file is part of libsnark, developed by SCIPR Lab
 *             and contributors (see AUTHORS).
 * @copyright  MIT license (see LICENSE file)
 *****************************************************************************/

// use  "gmp.h"
// use  "depends/libsnark-supercop/include/crypto_core_aes128encrypt.h"
// use  "depends/libsnark-supercop/include/randombytes.h"

use crate::common::default_types::r1cs_ppzkadsnark_pp;



// template <>
 pub fn prfGen<default_r1cs_ppzkadsnark_pp>()->aesPrfKeyT {
    let  key=aesPrfKeyT::new();
    // randombytes(key.key_bytes,32);
    return key;
}

// 
pub fn prfCompute<default_r1cs_ppzkadsnark_pp>(
    key:&aesPrfKeyT,  label:&labelT)->Fr<snark_pp<default_r1cs_ppzkadsnark_pp>>  {
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
}


