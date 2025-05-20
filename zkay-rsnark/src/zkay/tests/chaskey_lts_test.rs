use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::CircuitGenerator;
use crate::circuit::structure::wire_type::WireType;

use zkay::*;
use zkay::crypto::crypto_backend;


pub struct ChaskeyLtsTest {
    // Chaskey lts test vectors from FELICS
    // https://www.cryptolux.org/index.php/FELICS

     Vec<byte> key = {
            (byte) 0x56, (byte) 0x09, (byte) 0xe9, (byte) 0x68,
            (byte) 0x5f, (byte) 0x58, (byte) 0xe3, (byte) 0x29,
            (byte) 0x40, (byte) 0xec, (byte) 0xec, (byte) 0x98,
            (byte) 0xc5, (byte) 0x22, (byte) 0x98, (byte) 0x2f
    };
     Vec<byte> plain = {
        (byte) 0xb8, (byte) 0x23, (byte) 0x28, (byte) 0x26,
        (byte) 0xfd, (byte) 0x5e, (byte) 0x40, (byte) 0x5e,
        (byte) 0x69, (byte) 0xa3, (byte) 0x01, (byte) 0xa9,
        (byte) 0x78, (byte) 0xea, (byte) 0x7a, (byte) 0xd8
    };
     Vec<byte> cipher = {
        (byte) 0xd5, (byte) 0x60, (byte) 0x8d, (byte) 0x4d,
        (byte) 0xa2, (byte) 0xbf, (byte) 0x34, (byte) 0x7b,
        (byte) 0xab, (byte) 0xf8, (byte) 0x77, (byte) 0x2f,
        (byte) 0xdf, (byte) 0xed, (byte) 0xde, (byte) 0x07
    };

    
    pub   byteBigintConversionTest() {
        let b = ZkayUtil.unsignedBytesToBigInt(plain);
        let o = ZkayUtil.unsignedBigintToBytes(b, plain.len());
        Assert.assertArrayEquals("Array bigint conversion does not preserve values", o, plain);

        b = ZkayUtil.unsignedBytesToBigInt(cipher);
        o = ZkayUtil.unsignedBigintToBytes(b, cipher.len());
        Assert.assertArrayEquals("Array bigint conversion does not preserve values", o, cipher);

        let zero_arr = vec![byte::default();16];
        b = ZkayUtil.unsignedBytesToBigInt(zero_arr);
        o = ZkayUtil.unsignedBigintToBytes(b, zero_arr.len());
        Assert.assertArrayEquals("Array bigint conversion does not preserve values", o, zero_arr);
    }

    
    pub   chaskeyLtsTest() {
        let crypto = ChaskeyLTSEngine::new();

        // Test encrypt
        crypto.init(true, KeyParameter::new(key));
        let out = vec![byte::default();16];
        crypto.processBlock(plain, 0, out, 0);
        Assert.assertArrayEquals("Wrong encryption output", cipher, out);

        crypto.reset();

        // Test decrypt
        crypto.init(false, KeyParameter::new(key));
        crypto.processBlock(out, 0, out, 0);
        Assert.assertArrayEquals("Wrong decryption output", plain, out);
    }

    
    pub   cbcChaskeyOutputSameAsGadgetTest() throws InvalidCipherTextException {
        // Define inputs
        let key = BigInteger::new("b2e21df10a222a69ee1e6a2d60465f4c", 16);
        let iv = BigInteger::new("f2c605c86352cea9fcaf88f12eba6371", 16);
        let plain = BigInteger::new("6d60ad00cd9efa16841c842876fd4dc9f0fba1eb9e1ce623a83f45483a221f9", 16);

        // Compute encryption via jsnark gadget
        CircuitGenerator cgen = CircuitGenerator::new("cbcchaskey") {
            
              fn buildCircuit() {
                let plainwire = TypedWire::new(createConstantWire(plain), ZkayType.ZkUint(256), "plaintext");
                let ivwire = createConstantWire(iv);
                let keywire = createConstantWire(key);

                makeOutputArray(ZkayCBCSymmetricEncGadget::new(plainwire, keywire, ivwire,
                        ZkayCBCSymmetricEncGadget.CipherType.CHASKEY).getOutputWires());
            }

            
            pub  fn generateSampleInput(CircuitEvaluator evaluator) {

            }
        };
        cgen.generateCircuit();
        cgen.evalCircuit();
        let evaluator = CircuitEvaluator::new(cgen);
        evaluator.evaluate();
        let outwires = cgen.getOutWires();
        let outs = vec![BigInteger::default();outwires.size()];
        for i in 0..outs.len() {
            outs[i] = evaluator.getWireValue(outwires.get(i));
        }


        // Compute encryption via CbcChaskey implementation
        let iv_bytes = ZkayUtil.unsignedBigintToBytes(iv, 16);
        Vec<byte> result = ChaskeyLtsCbc.crypt(true, ZkayUtil.unsignedBigintToBytes(key, 16),
                                            iv_bytes, ZkayUtil.unsignedBigintToBytes(plain, 32));


        // Convert output to format produced by gadget (iv included, packed 248bit values in reverse order)
        let iv_cipher = vec![byte::default();16 + result.len()];
        System.arraycopy(iv_bytes, 0, iv_cipher, 0, iv_bytes.len());
        System.arraycopy(result, 0, iv_cipher, iv_bytes.len(), result.len());

        let chunk_size = CryptoBackend.Symmetric.CIPHER_CHUNK_SIZE / 8;
        let first_chunk_size = iv_cipher.len() % chunk_size;
        let bigints = new ArrayList<>();
        if first_chunk_size != 0 {
            let chunk = Arrays.copyOfRange(iv_cipher, 0, first_chunk_size);
            bigints.add(ZkayUtil.unsignedBytesToBigInt(chunk));
        }
        for i in first_chunk_size..iv_cipher.len() - first_chunk_size{
            let chunk = Arrays.copyOfRange(iv_cipher, i, i + chunk_size);
            bigints.add(ZkayUtil.unsignedBytesToBigInt(chunk));
        }
        Collections.reverse(bigints);

        // Check if both are equal
        Assert.assertArrayEquals(outs, bigints.toArray());
    }
}
