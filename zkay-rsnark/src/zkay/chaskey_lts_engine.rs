

pub struct ChaskeyLTSEngine 
{     enc:bool,
      key:Vec<i32>,
}
    impl BlockCipher for ChaskeyLTSEngine {
    pub  fn init(encrypt:bool , cipherParameters:CipherParameters )  {
            assert!( (cipherParameters.instanceof( KeyParameter)) && (cipherParameters).getKey().length == 16);
        
        enc = encrypt;
        key = vec![i32::default();4];
        ByteBuffer.wrap(((KeyParameter) cipherParameters).getKey()).order(ByteOrder.LITTLE_ENDIAN).asIntBuffer().get(key);
    }

    
 pub fn getAlgorithmName()->      String {
        return "chaskey_lts_128";
    }

    
    pub  fn getBlockSize()->  i32 {
        return 16;
    }

    
    pub   processBlock(in:Vec<byte>, inOff:i32 , out:Vec<byte>, outOff:i32 )->i32  {
let v =  vec![i32::default();4];
        ByteBuffer.wrap(in, inOff, 16).order(ByteOrder.LITTLE_ENDIAN).asIntBuffer().get(v);

        v[0] ^= key[0];
        v[1] ^= key[1];
        v[2] ^= key[2];
        v[3] ^= key[3];

        if enc {
            for round in 0..16{
                v[0] += v[1];
                v[1] = Integer.rotateLeft(v[1], 5) ^ v[0];
                v[0] = Integer.rotateLeft(v[0], 16);

                v[2] += v[3];
                v[3] = Integer.rotateLeft(v[3], 8);
                v[3] ^= v[2];

                v[0] += v[3];
                v[3] = Integer.rotateLeft(v[3], 13);
                v[3] ^= v[0];

                v[2] += v[1];
                v[1] = Integer.rotateLeft(v[1], 7) ^ v[2];
                v[2] = Integer.rotateLeft(v[2], 16);
            }
        }
        else {
            for round in 0..16{
                v[2] = Integer.rotateRight(v[2], 16);
                v[1] = Integer.rotateRight(v[1] ^ v[2], 7);
                v[2] -= v[1];

                v[3] ^= v[0];
                v[3] = Integer.rotateRight(v[3], 13);
                v[0] -= v[3];

                v[3] ^= v[2];
                v[3] = Integer.rotateRight(v[3], 8);
                v[2] -= v[3];

                v[0] = Integer.rotateRight(v[0], 16);
                v[1] = Integer.rotateRight(v[1] ^ v[0], 5);
                v[0] -= v[1];
            }
        }

        v[0] ^= key[0];
        v[1] ^= key[1];
        v[2] ^= key[2];
        v[3] ^= key[3];

        ByteBuffer.wrap(out, outOff, 16).order(ByteOrder.LITTLE_ENDIAN).asIntBuffer().put(v);
        return 16;
    }

    
    pub   reset() {
        // There are no state modifications -> nothing to do here
    }
}
