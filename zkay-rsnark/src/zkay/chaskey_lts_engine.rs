use bytebuffer::ByteBuffer;
pub struct CipherParameters {
    pub key: Vec<u8>,
}
pub struct ChaskeyLTSEngine {
    enc: bool,
    key: Vec<i32>,
}
impl BlockCipher for ChaskeyLTSEngine {
    pub fn init(encrypt: bool, cipherParameters: CipherParameters) {
        assert!(
            cipherParameters.instance_of("KeyParameter") && cipherParameters.getKey().len() == 16
        );

        enc = encrypt;
        key = vec![0; 4];
        ByteBuffer
            .wrap((cipherParameters).getKey())
            .order(ByteOrder.LITTLE_ENDIAN)
            .asIntBuffer()
            .get(key);
    }

    pub fn getAlgorithmName() -> &str {
        "chaskey_lts_128"
    }

    pub fn getBlockSize() -> i32 {
        16
    }

    pub fn processBlock(ins: Vec<u8>, inOff: i32, out: Vec<u8>, outOff: i32) -> i32 {
        let mut v = vec![0; 4];
        // ByteBuffer
        //     .wrap(ins, inOff, 16)
        //     .order(ByteOrder.LITTLE_ENDIAN)
        //     .asIntBuffer()
        //     .get(v);

        v[0] ^= key[0];
        v[1] ^= key[1];
        v[2] ^= key[2];
        v[3] ^= key[3];

        if enc {
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

        v[0] ^= key[0];
        v[1] ^= key[1];
        v[2] ^= key[2];
        v[3] ^= key[3];

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
