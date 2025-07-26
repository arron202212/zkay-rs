#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use std::ops::{Mul,Add,Sub,Div,Rem};
use crate::util::util::{BigInteger, Util};
/**
 * Utility methods to extract sample randomness used by standard implementations
 * for RSA Encryption. In absence of test vectors, the extracted randomness is
 * used to test our RSA gadgets to make sure the RSA circuits match the standard
 * implementations.
 *
 */

pub struct RSAUtil;
impl RSAUtil {
    pub fn extractRSARandomness1_5(cipherText: Vec<u8>, privateKey: Vec<u8>) -> Vec<Vec<u8>> {
        let modulus = BigInteger::from(7); //privateKey.getModulus();
        let keySize = modulus.bits();
        let d = BigInteger::from(1); //privateKey.getPrivateExponent();

        let mut cipherTextPadded = vec![0; cipherText.len() + 1];
        cipherTextPadded[1..cipherText.len()].clone_from_slice(&cipherText[0..]);
        cipherTextPadded[0] = 0;

        let mut c = BigInteger::from_signed_bytes_be(&cipherText);
        c = BigInteger::from_signed_bytes_be(&cipherTextPadded);
        let mut product = Util::one();
        for i in (0..=keySize - 1).rev() {
            product = product.clone().mul(&product).rem(&modulus);
            let bit = d.bit(i);
            if bit {
                product = product.mul(&c).rem(&modulus);
            }
        }

        //		//println!("After decryption manually = "
        //				+ product.toString(16));

        let paddedPlaintext = product.to_bytes_be().1.clone();
        if paddedPlaintext.len() != keySize as usize / 8 - 1 {
            println!("Error");
           return  vec![]
        }
        let mut plaintext = vec![];
        let mut randomness = vec![];

        if paddedPlaintext[0] != 2 {
            println!("Error");
        } else {
            for i in 1..keySize as usize/ 8 - 2 {
                if paddedPlaintext[i] == 0 {
                    plaintext = vec![0; (keySize as usize / 8 - 2) - i];
                    randomness = vec![0; i - 1];
                    plaintext.clone_from_slice(&paddedPlaintext[i + 1..]);
                    randomness.clone_from_slice(&paddedPlaintext[1..]);

                    break;
                }
            }
        }
        let result = vec![plaintext, randomness];
        result
    }

    fn intToByteArray(value: i32) -> Vec<u8> {
        vec![(value >> 24) as u8, (value >> 16) as u8, (value >> 8) as u8, value as u8]
    }

    fn mgf(array:&Vec<u8>, maskLen: i32, hlen: i32) -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=(maskLen  as f64 / hlen as f64).ceil() as i32 - 1 {
            let c = Self::intToByteArray(i);
            // let hash = None;

            // hash = MessageDigest.getInstance("SHA-256");

            // hash.update(concat(array, c));
            let digest = vec![];//hash.digest();
            // hash.reset();
            v = Self::concat(v, digest);
        }
        v
    }

    fn concat(a1: Vec<u8>, a2: Vec<u8>) -> Vec<u8> {
        let l = a1.len() + a2.len();
        let mut result = vec![0; l];
        for i in 0..a1.len() {
            result[i] = a1[i];
        }
        for i in 0..a2.len() {
            result[i + a1.len()] = a2[i];
        }
        result
    }

    pub fn extractRSAOAEPSeed(cipherText: &Vec<u8>, privateKey: &Vec<u8>) -> Vec<Vec<u8>> {
        let modulus = BigInteger::from_signed_bytes_be(&privateKey);//.getModulus();
        let keySize = modulus.bits() as usize;
        let d = BigInteger::from(1);//privateKey.getPrivateExponent();

        let mut cipherTextPadded = vec![0; cipherText.len() + 1];
        cipherTextPadded[1..1 + cipherText.len()].clone_from_slice(&cipherText);
        cipherTextPadded[0] = 0;

        let mut c = BigInteger::from_signed_bytes_be(&cipherText);
        c = BigInteger::from_signed_bytes_be(&cipherTextPadded);

        let mut product = Util::one();
        for i in (0..=keySize - 1).rev() {
            product = product.clone().mul(&product).rem(&modulus);
            let bit = d.bit(i as u64);
            if bit {
                product = product.mul(&c).rem(&modulus);
            }
        }

        let hlen = 32usize;
        let maskedDBLength = keySize / 8 - hlen - 1;

        let mut encodedMessageBytes = product.to_bytes_be().1.clone();

        if encodedMessageBytes.len() > keySize / 8 {
            encodedMessageBytes = encodedMessageBytes[1..].to_vec();
        } else {
            while (encodedMessageBytes.len() < keySize / 8) {
                encodedMessageBytes = Self::concat(vec![0], encodedMessageBytes);
            }
        }

        let maskedSeed = encodedMessageBytes[1..hlen + 2].to_vec();
        let maskedDb = encodedMessageBytes[hlen + 1..].to_vec();

        let seedMask = Self::mgf(&maskedDb, hlen as i32, hlen as i32);
        let mut seed = seedMask.clone();
        for i in 0..hlen {
            seed[i] ^= maskedSeed[i];
        }

        let mut dbMask = Self::mgf(&seed, keySize as i32 / 8 - hlen as i32- 1, hlen  as i32);
        dbMask = dbMask[..keySize / 8 - hlen - 1].to_vec();

        let mut DB = vec![0; dbMask.len() + 1]; // appending a zero to the left, to avoid sign issues in the BigInteger
        DB[1..1 + maskedDBLength].clone_from_slice(&maskedDb);
        for i in 0..maskedDBLength {
            DB[i + 1] ^= dbMask[i];
        }
        //		let dbInt = BigInteger::new(DB);

        let mut shift1 = 0;
        while (DB[shift1] == 0) {
            shift1 += 1;
        }
        let mut idx = 32 + shift1;
        while (DB[idx] == 0) {
            idx += 1;
        }
        let plaintext = DB[idx + 1..].to_vec();
        let result = vec![plaintext, seed];
        result
    }
}
