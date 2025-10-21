/** @file
 *****************************************************************************

 Fast batch verification signature for ADSNARK.

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

//#ifndef ED25519SIG_HPP_
// #define ED25519SIG_HPP_

use crate::zk_proof_systems::ppzkadsnark::r1cs_ppzkadsnark::r1cs_ppzkadsnark_signature;



pub struct ed25519_sigT {
     sig_bytes:[u8;64],
}

pub struct ed25519_vkT {
     vk_bytes:[u8;32],
}

pub struct ed25519_skT {
     sk_bytes:[u8;64],
}



//#endif // ED25519SIG_HPP_
/** @file
 *****************************************************************************

 Fast batch verification signature for ADSNARK.

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

// use  "depends/libsnark-supercop/include/crypto_sign.h"

use crate::common::default_types::r1cs_ppzkadsnark_pp;



// template<>
pub fn sigGen<default_r1cs_ppzkadsnark_pp>()->kpT<default_r1cs_ppzkadsnark_pp>  {
   let  keys= kpT::<default_r1cs_ppzkadsnark_pp>::new();
    // crypto_sign_ed25519_amd64_51_30k_keypair(keys.vk.vk_bytes,keys.sk.sk_bytes);
    return keys;
}

// template<>
 pub fn sigSign<default_r1cs_ppzkadsnark_pp>(sk:&ed25519_skT , label:&labelT ,
                                                  Lambda:&ffec::G2<snark_pp<default_r1cs_ppzkadsnark_pp>> )->ed25519_sigT {
     let mut sigma=ed25519_sigT;
    let mut  sigmalen;
     let mut signature=vec![0;64+16+320];
    let  mut message=vec![0;16+320];

    let mut Lambda_copy=G2::<snark_pp::<default_r1cs_ppzkadsnark_pp>>::new(Lambda);
    Lambda_copy.to_affine_coordinates();

    for i in 0..16
        {message[i] = label.label_bytes[i];}

    // More efficient way to get canonical point rep?
    let mut  stream=String::new();
    // stream.rdbuf()->pubsetbuf(((char*)message)+16, 320);
    // stream << Lambda_copy;
    // let  written = stream.tellp();
    // while (written<320)
    // 	message[16+written++] = 0;
    
    crypto_sign_ed25519_amd64_51_30k(signature,&sigmalen,message,16+320,sk.sk_bytes);

    assert!(sigmalen == 64+16+320);

    for i in 0..64
        {sigma.sig_bytes[i] = signature[i];}

    return sigma;
}

// template<>
 pub fn sigVerif<default_r1cs_ppzkadsnark_pp>(vk:&ed25519_vkT , label:&labelT ,
                                           Lambda:&ffec::G2<snark_pp<default_r1cs_ppzkadsnark_pp>> ,
                                           sig:&ed25519_sigT )->bool{
    let mut msglen;
     let mut message=vec![0;64+16+320];
    let mut  signature=vec![0;64+16+320];

    let mut  Lambda_copy=G2::<snark_pp::<default_r1cs_ppzkadsnark_pp>>::new(Lambda);
    Lambda_copy.to_affine_coordinates();

    for i in 0..64
       { signature[i] = sig.sig_bytes[i];}

    for i in 0..16
        {signature[64+i] = label.label_bytes[i];}

    // More efficient way to get canonical point rep?
    let mut  stream=String::new();
    // stream.rdbuf()->pubsetbuf(((char*)signature)+64+16, 320);
    // stream << Lambda_copy;
    // size_t written = stream.tellp();
    // while (written<320)
    // 	signature[64+16+written++] = 0;

    let  res = crypto_sign_ed25519_amd64_51_30k_open(message,&msglen,signature,64+16+320,vk.vk_bytes);
    return (res==0);
}

// template<>
pub fn  sigBatchVerif<default_r1cs_ppzkadsnark_pp>(vk:&ed25519_vkT , labels:&Vec<labelT> ,
                                                Lambdas:&Vec<ffec::G2<snark_pp<default_r1cs_ppzkadsnark_pp>>> ,
                                                sigs:&Vec<ed25519_sigT> )->bool {
    let mut  stream=String::new();

    assert!(labels.len() == Lambdas.len());
    assert!(labels.len() == sigs.len());

    let mut msglen=vec![0u64;labels.len()];
    let mut siglen=vec![0u64;labels.len()];
     let mut messages=vec![0;labels.len()];
    let mut signatures=vec![0;labels.len()];
     let mut pks=vec![0;labels.len()];

     pk_copy[32];
    for i in 0..32{
        pk_copy[i] = vk.vk_bytes[i];
    }

    let mut messagemem = vec![0;labels.len()*(64+16+320)];
    // assert!(messagemem != NULL);
     let mut signaturemem = vec![0;labels.len()*(64+16+320)];
    // assert!(signaturemem != NULL);

    for i in 0..labels.len(){
        siglen[i] = 64+16+320;
        messages[i] = messagemem+(64+16+320)*i;
        signatures[i] = signaturemem+(64+16+320)*i;
        pks[i] = pk_copy;

        for j in 0..64{
            signaturemem[i*(64+16+320)+j] = sigs[i].sig_bytes[j];}

        for j in 0..16{
            signaturemem[i*(64+16+320)+64+j] = labels[i].label_bytes[j];}

        // More efficient way to get canonical point rep?
       	let mut  Lambda_copy=G2::<snark_pp::<default_r1cs_ppzkadsnark_pp>>::new(Lambdas[i]);
        Lambda_copy.to_affine_coordinates();
        // stream.clear();
        // stream.rdbuf()->pubsetbuf((char*)(signaturemem+i*(64+16+320)+64+16), 320);
        // stream << Lambda_copy;
        // size_t written = stream.tellp();
        // while (written<320)
        //     {signaturemem[i*(64+16+320)+64+16+written++] = 0;}

    }

    let  res = crypto_sign_ed25519_amd64_51_30k_open_batch(
        messages,msglen,
        signatures,siglen,
        pks,
        labels.len());

    return (res==0);
}


