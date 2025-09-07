#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
//#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use crate::util::util::{BigInteger, Util};
use std::ops::{Add, Div, Mul, Rem, Sub};

//  * Utility methods to extract sample randomness used by standard implementations
//  * for RSA Encryption. In absence of test vectors, the extracted randomness is
//  * used to test our RSA gadgets to make sure the RSA circuits match the standard
//  * implementations.

pub struct RSAUtil;
impl RSAUtil {
    pub fn extract_rsa_randomness1_5(cipher_text: &Vec<u8>, private_key: &Vec<u8>) -> Vec<Vec<u8>> {
        let modulus = BigInteger::from(7); //private_key.getModulus();
        let key_size = modulus.bits();
        let d = BigInteger::from(1); //private_key.getPrivateExponent();

        let mut cipher_text_padded = vec![0; cipher_text.len() + 1];
        cipher_text_padded[1..cipher_text.len()].clone_from_slice(&cipher_text[0..]);
        cipher_text_padded[0] = 0;

        let mut c = BigInteger::from_signed_bytes_be(&cipher_text);
        c = BigInteger::from_signed_bytes_be(&cipher_text_padded);
        let mut product = Util::one();
        for i in (0..=key_size - 1).rev() {
            product = product.clone().mul(&product).rem(&modulus);
            let bit = d.bit(i);
            if bit {
                product = product.mul(&c).rem(&modulus);
            }
        }

        //		//println!("After decryption manually = "
        //				+ product.toString(16));

        let padded_plain_text = product.to_bytes_be().1.clone();
        if padded_plain_text.len() != key_size as usize / 8 - 1 {
            println!("Error");
            return vec![];
        }
        let mut plaintext = vec![];
        let mut randomness = vec![];

        if padded_plain_text[0] != 2 {
            println!("Error");
        } else {
            for i in 1..key_size as usize / 8 - 2 {
                if padded_plain_text[i] == 0 {
                    plaintext = vec![0; (key_size as usize / 8 - 2) - i];
                    randomness = vec![0; i - 1];
                    plaintext.clone_from_slice(&padded_plain_text[i + 1..]);
                    randomness.clone_from_slice(&padded_plain_text[1..]);

                    break;
                }
            }
        }
        let result = vec![plaintext, randomness];
        result
    }

    fn int_to_byte_array(value: i32) -> Vec<u8> {
        vec![
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ]
    }

    fn mgf(array: &Vec<u8>, maskLen: i32, hlen: i32) -> Vec<u8> {
        let mut v = vec![];
        for i in 0..=(maskLen as f64 / hlen as f64).ceil() as i32 - 1 {
            let c = Self::int_to_byte_array(i);
            // let hash = None;

            // hash = MessageDigest.getInstance("SHA-256");

            // hash.update(concat(array, c));
            let digest = vec![]; //hash.digest();
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

    pub fn extract_rsa_oaep_seed(cipher_text: &Vec<u8>, private_key: &Vec<u8>) -> Vec<Vec<u8>> {
        let modulus = BigInteger::from_signed_bytes_be(&private_key); //.getModulus();
        let key_size = modulus.bits() as usize;
        let d = BigInteger::from(1); //private_key.getPrivateExponent();

        let mut cipher_text_padded = vec![0; cipher_text.len() + 1];
        cipher_text_padded[1..1 + cipher_text.len()].clone_from_slice(&cipher_text);
        cipher_text_padded[0] = 0;

        let mut c = BigInteger::from_signed_bytes_be(&cipher_text);
        c = BigInteger::from_signed_bytes_be(&cipher_text_padded);

        let mut product = Util::one();
        for i in (0..=key_size - 1).rev() {
            product = product.clone().mul(&product).rem(&modulus);
            let bit = d.bit(i as u64);
            if bit {
                product = product.mul(&c).rem(&modulus);
            }
        }

        let hlen = 32usize;
        let masked_db_length = key_size / 8 - hlen - 1;

        let mut encoded_message_bytes = product.to_bytes_be().1.clone();

        if encoded_message_bytes.len() > key_size / 8 {
            encoded_message_bytes = encoded_message_bytes[1..].to_vec();
        } else {
            while (encoded_message_bytes.len() < key_size / 8) {
                encoded_message_bytes = Self::concat(vec![0], encoded_message_bytes);
            }
        }

        let masked_seed = encoded_message_bytes[1..hlen + 2].to_vec();
        let masked_db = encoded_message_bytes[hlen + 1..].to_vec();

        let seed_mask = Self::mgf(&masked_db, hlen as i32, hlen as i32);
        let mut seed = seed_mask.clone();
        for i in 0..hlen {
            seed[i] ^= masked_seed[i];
        }

        let mut db_mask = Self::mgf(&seed, key_size as i32 / 8 - hlen as i32 - 1, hlen as i32);
        db_mask = db_mask[..key_size / 8 - hlen - 1].to_vec();

        let mut db = vec![0; db_mask.len() + 1]; // appending a zero to the left, to avoid sign issues in the BigInteger
        db[1..1 + masked_db_length].clone_from_slice(&masked_db);
        for i in 0..masked_db_length {
            db[i + 1] ^= db_mask[i];
        }
        //		let dbInt = BigInteger::new(db);

        let mut shift1 = 0;
        while (db[shift1] == 0) {
            shift1 += 1;
        }
        let mut idx = 32 + shift1;
        while (db[idx] == 0) {
            idx += 1;
        }
        let plaintext = db[idx + 1..].to_vec();
        let result = vec![plaintext, seed];
        result
    }
}
