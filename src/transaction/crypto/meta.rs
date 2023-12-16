use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
       pub  static ref cryptoparams:HashMap<String,HashMap<&'static str,i32>>=
    HashMap::from([
    (String::from("dummy"), HashMap::from([
        ("key_bits", 248),
        ("cipher_payload_bytes", 31),
        ("cipher_chunk_size", 31),
        ("symmetric", 0),
        ("rnd_bytes", 31),
        ("rnd_chunk_size", 31),
        ("enc_signed_as_unsigned", 1),
    ])),
     (String::from("dummy-hom"), HashMap::from([
        ("key_bits", 248),
        ("cipher_payload_bytes", 32),
        ("cipher_chunk_size", 32),
        ("symmetric", 0),
        ("rnd_bytes", 32),
        ("rnd_chunk_size", 32),
        ("enc_signed_as_unsigned", 0),
    ])),

     (String::from("rsa-oaep"),  HashMap::from([
        ("key_bits", 2048),
        ("cipher_payload_bytes", 256),
        ("cipher_chunk_size", 29),
        ("symmetric", 0),
        ("rnd_bytes", 32),
        ("rnd_chunk_size", 16),
        ("enc_signed_as_unsigned", 1),
   ])),
     (String::from("rsa-pkcs1.5"),  HashMap::from([
        ("key_bits", 2048),
        ("cipher_payload_bytes", 256),
        ("cipher_chunk_size", 29),
        ("symmetric", 0),
        ("rnd_bytes", 221), //// for 256 - 3 - plainbytes (32 byte plaintext, for now fixed)
        ("rnd_chunk_size", 28),
        ("enc_signed_as_unsigned", 1),
    ])),
     (String::from("ecdh-aes"),  HashMap::from([
        ("key_bits", 253),
        ("cipher_payload_bytes", 48),// 128bit iv + 256 bit ciphertext
        ("cipher_chunk_size", 24),
        ("symmetric", 1),
        ("rnd_bytes", 0),// included in cipher text
        ("rnd_chunk_size", 0),
        ("enc_signed_as_unsigned", 1),
    ])),

     (String::from("ecdh-chaskey"),  HashMap::from([
        ("key_bits", 253),
        ("cipher_payload_bytes", 48),// 128bit iv + 256 bit ciphertext
        ("cipher_chunk_size", 24),
        ("symmetric", 1),
        ("rnd_bytes", 0),// included in cipher text
        ("rnd_chunk_size", 0),
        ("enc_signed_as_unsigned", 1),
  ])),
   // WARNING, Not cryptographically secure. Values retained for developer sanity.
   // Recommended values,
   // - key_bits, 2048
   // - cipher_payload_bytes, 4096 // 8
   // - rnd_bytes, 2048 // 8
    ( String::from("paillier"),  HashMap::from([
        ("key_bits", 320), // 320-bit n
        ("cipher_payload_bytes", 640 / 8), // cipher is mod n^2, thus at most twice the bit length
        ("cipher_chunk_size", 120 / 8), // LongElement.CHUNK_SIZE / sizeof(byte)
        ("symmetric", 0),
        ("rnd_bytes", 320 / 8), // random value mod n, thus same size as n
        ("rnd_chunk_size", 120 / 8), // LongElement.CHUNK_SIZE / sizeof(byte)
        ("enc_signed_as_unsigned", 0),
    ])),

    (String::from("elgamal"),  HashMap::from([
        ("key_bits", 2*254),                 // two BabyJubJub coordinates (fit into 254 bits each)
        ("cipher_payload_bytes", 128),       // four BabyJubJub coordinates
        ("cipher_chunk_size", 32),           // one BabyJubJub coordinate
        ("symmetric", 0),
        ("rnd_bytes", 32),                   // one element from the BabyJubJub scalar field
        ("rnd_chunk_size", 32),
        ("enc_signed_as_unsigned", 0),
    ])),
    ]);
}
