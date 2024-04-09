#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
// import math
// import re

use crate::meta::CRYPTOPARAMS;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CryptoParams {
    pub crypto_name: String,
}
pub fn title(data: String) -> String {
    let mut result = String::new();
    let mut first = true;
    for value in data.chars() {
        if first {
            result.push(value.to_ascii_uppercase());
            first = false;
        } else {
            result.push(value);
            if !value.is_ascii_alphanumeric() {
                first = true;
            }
        }
    }
    result
}
// class CryptoParams:
impl CryptoParams {
    pub fn new(crypto_name: String) -> Self {
        Self { crypto_name }
    }

    // pub fn __eq__(self, other)
    //     return isinstance(other, CryptoParams) and self.crypto_name == other.crypto_name

    // pub fn __hash__(&self)
    //     return self.crypto_name.__hash__()

    pub fn identifier_name(&self) -> String {
        // re.sub("[^a-zA-Z0-9$_]", "_", self.crypto_name).title()
        use lazy_static::lazy_static;
        use regex::Regex;
        use std::borrow::Cow;
        lazy_static! {
            static ref ISO8601_DATE_REGEX: Regex = Regex::new(r"[^a-zA-Z0-9$_]").unwrap();
        }
        title(
            ISO8601_DATE_REGEX
                .replace_all(&self.crypto_name, "_")
                .to_string(),
        )
    }

    pub fn key_bits(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"key_bits"]
    }

    pub fn key_bytes(&self) -> i32 {
        ((self.key_bits() + 7) / 8) as _
    }

    pub fn key_len(&self) -> i32 {
        if self.is_symmetric_cipher() {
            1
        } else {
            (self.key_bytes() + self.cipher_chunk_size() - 1) / self.cipher_chunk_size()
        }
    }

    pub fn rnd_bytes(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"rnd_bytes"]
    }

    pub fn rnd_chunk_size(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"rnd_chunk_size"]
    }

    pub fn randomness_len(&self) -> Option<i32> {
        Some(if self.is_symmetric_cipher() {
            0
        } else {
            (self.rnd_bytes() + self.rnd_chunk_size() - 1) / self.rnd_chunk_size()
        })
    }

    pub fn cipher_bytes_payload(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"cipher_payload_bytes"]
    }

    pub fn is_symmetric_cipher(&self) -> bool {
        CRYPTOPARAMS[&self.crypto_name][&"symmetric"] != 0
    }

    pub fn cipher_payload_len(&self) -> i32 {
        (self.cipher_bytes_payload() + self.cipher_chunk_size() - 1) / self.cipher_chunk_size()
    }

    pub fn cipher_len(&self) -> i32 {
        self.cipher_payload_len()
            + if self.is_symmetric_cipher()
              { 1} //Additional uint to store sender address
        else
            {0}
    }

    pub fn cipher_chunk_size(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"cipher_chunk_size"]
    }

    pub fn enc_signed_as_unsigned(&self) -> i32 {
        CRYPTOPARAMS[&self.crypto_name][&"enc_signed_as_unsigned"]
    }
}
