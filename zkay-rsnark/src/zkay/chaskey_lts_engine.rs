#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
#![allow(warnings, unused)]
use bytebuffer::ByteBuffer;

#[derive(Debug, Clone)]
pub struct CipherParameters {
    pub key: Vec<u8>,
}
impl CipherParameters {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }
    pub fn instance_of(&self, s: &str) -> bool {
        true
    }
    pub fn getKey(&self) -> &Vec<u8> {
        &self.key
    }
}

#[derive(Debug, Clone)]
pub struct ChaskeyLTSEngine {
    pub enc: bool,
    pub key: Vec<i32>,
}
//BlockCipher for
impl ChaskeyLTSEngine {
    pub fn new(encrypt: bool, cipherParameters: CipherParameters) -> Self {
        assert!(
            cipherParameters.instance_of("CipherParameters")
                && cipherParameters.getKey().len() == 16,
            "{},",
            cipherParameters.getKey().len()
        );

        let mut key = vec![0; 4];
        // ByteBuffer
        //     .wrap((cipherParameters).getKey())
        //     .order(ByteOrder.LITTLE_ENDIAN)
        //     .asIntBuffer()
        //     .get(key);
        Self { enc: encrypt, key }
    }

    pub fn getAlgorithmName() -> &'static str {
        "chaskey_lts_128"
    }

    pub fn getBlockSize() -> i32 {
        16
    }

    pub fn processBlock(&self, ins: &Vec<u8>, inOff: i32, out: &Vec<u8>, outOff: i32) -> i32 {
        let mut v = vec![0; 4];
        // ByteBuffer
        //     .wrap(ins, inOff, 16)
        //     .order(ByteOrder.LITTLE_ENDIAN)
        //     .asIntBuffer()
        //     .get(v);

        v[0] ^= self.key[0];
        v[1] ^= self.key[1];
        v[2] ^= self.key[2];
        v[3] ^= self.key[3];

        if self.enc {
            for round in 0..16 {
                v[0] += v[1];
                v[1] = v[1].rotate_left(5) ^ v[0];
                v[0] = v[0].rotate_left(16);

                v[2] += v[3];
                v[3] = v[3].rotate_left(8);
                v[3] ^= v[2];

                v[0] += v[3];
                v[3] = v[3].rotate_left(13);
                v[3] ^= v[0];

                v[2] += v[1];
                v[1] = v[1].rotate_left(7) ^ v[2];
                v[2] = v[2].rotate_left(16);
            }
        } else {
            for round in 0..16 {
                v[2] = v[2].rotate_right(16);
                v[1] = v[1] ^ v[2].rotate_right(7);
                v[2] -= v[1];

                v[3] ^= v[0];
                v[3] = v[3].rotate_right(13);
                v[0] -= v[3];

                v[3] ^= v[2];
                v[3] = v[3].rotate_right(8);
                v[2] -= v[3];

                v[0] = v[0].rotate_right(16);
                v[1] = v[1] ^ v[0].rotate_right(5);
                v[0] -= v[1];
            }
        }

        v[0] ^= self.key[0];
        v[1] ^= self.key[1];
        v[2] ^= self.key[2];
        v[3] ^= self.key[3];

        // ByteBuffer
        //     .wrap(out, outOff, 16)
        //     .order(ByteOrder.LITTLE_ENDIAN)
        //     .asIntBuffer()
        //     .put(v);
        16
    }

    pub fn reset() {
        // There are no state modifications -> nothing to do here
    }
}
