// import math
// import re

use crate::transaction::crypto::meta::cryptoparams;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CryptoParams {
    crypto_name: String,
}
fn title(data: String) -> String {
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
    fn new(self, crypto_name: String) -> Self {
        Self { crypto_name }
    }

    // fn __eq__(self, other)
    //     return isinstance(other, CryptoParams) and self.crypto_name == other.crypto_name

    // fn __hash__(&self)
    //     return self.crypto_name.__hash__()

    fn identifier_name(&self) -> String {
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

    fn key_bits(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"key_bits"]
    }

    fn key_bytes(&self) -> i32 {
        ((self.key_bits() + 7) / 8) as _
    }

    fn key_len(&self) -> i32 {
        if self.is_symmetric_cipher() {
            1
        } else {
            (self.key_bytes() + self.cipher_chunk_size() - 1) / self.cipher_chunk_size()
        }
    }

    fn rnd_bytes(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"rnd_bytes"]
    }

    fn rnd_chunk_size(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"rnd_chunk_size"]
    }

    fn randomness_len(&self) -> i32 {
        if self.is_symmetric_cipher() {
            0
        } else {
            (self.rnd_bytes() + self.rnd_chunk_size() - 1) / self.rnd_chunk_size()
        }
    }

    fn cipher_bytes_payload(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"cipher_payload_bytes"]
    }

    fn is_symmetric_cipher(&self) -> bool {
        cryptoparams[&self.crypto_name][&"symmetric"] != 0
    }

    fn cipher_payload_len(&self) -> i32 {
        (self.cipher_bytes_payload() + self.cipher_chunk_size() - 1) / self.cipher_chunk_size()
    }

    fn cipher_len(&self) -> i32 {
        self.cipher_payload_len()
            + if self.is_symmetric_cipher()
              { 1} //Additional uint to store sender address
        else
            {0}
    }

    fn cipher_chunk_size(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"cipher_chunk_size"]
    }

    fn enc_signed_as_unsigned(&self) -> i32 {
        cryptoparams[&self.crypto_name][&"enc_signed_as_unsigned"]
    }
}
