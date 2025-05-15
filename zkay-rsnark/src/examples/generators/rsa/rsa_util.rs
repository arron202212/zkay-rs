/**
 * Utility methods to extract sample randomness used by standard implementations
 * for RSA Encryption. In absence of test vectors, the extracted randomness is
 * used to test our RSA gadgets to make sure the RSA circuits match the standard
 * implementations.
 *
 */

pub struct RSAUtil;
impl RSAUtil {
    pub fn extractRSARandomness1_5(
        cipherText: Vec<byte>,
        privateKey: RSAPrivateKey,
    ) -> Vec<Vec<byte>> {
        let modulus = privateKey.getModulus();
        let keySize = modulus.bitLength();
        let d = privateKey.getPrivateExponent();

        let cipherTextPadded = vec![byte::default(); cipherText.length + 1];
        System.arraycopy(cipherText, 0, cipherTextPadded, 1, cipherText.length);
        cipherTextPadded[0] = 0;

        let c = BigInteger::new(cipherText);
        c = BigInteger::new(cipherTextPadded);
        let product = BigInteger.ONE;
        for i in (0..=keySize - 1).rev() {
            product = product.multiply(product).modulo(modulus);
            let bit = d.testBit(i);
            if bit {
                product = product.multiply(c).modulo(modulus);
            }
        }

        //		println!("After decryption manually = "
        //				+ product.toString(16));

        let paddedPlaintext = product.toByteArray();
        if paddedPlaintext.length != keySize / 8 - 1 {
            println!("Error");
            return None;
        }
        let plaintext = None;
        let randomness = None;

        if paddedPlaintext[0] != 2 {
            println!("Error");
        } else {
            for i in 1..keySize / 8 - 2 {
                if paddedPlaintext[i] != 0 {
                    continue;
                } else {
                    plaintext = vec![byte::default(); (keySize / 8 - 2) - i];
                    randomness = vec![byte::default(); i - 1];
                    plaintext.clone_from_slice(&paddedPlaintext[i + 1..]);
                    randomness.clone_from_slice(&paddedPlaintext[1..]);

                    break;
                }
            }
        }
        let result = vec![vec![plaintext, randomness]];
        return result;
    }

    fn intToByteArray(value: i32) -> Vec<byte> {
        return vec![ (byte) (value >>> 24), (byte) (value >>> 16),
				(byte) (value >>> 8), (byte) value ];
    }

    fn mgf(array: Vec<byte>, maskLen: i32, hlen: i32) -> Vec<byte> {
        let v = vec![];
        for i in 0..=(maskLen * 1.0 / hlen).ceil() as i32 - 1 {
            let c = intToByteArray(i);
            let hash = None;

            hash = MessageDigest.getInstance("SHA-256");

            hash.update(concat(array, c));
            let digest = hash.digest();
            hash.reset();
            v = concat(v, digest);
        }
        return v;
    }

    fn concat(a1: Vec<byte>, a2: Vec<byte>) -> Vec<byte> {
        let l = a1.length + a2.length;
        let mut result = vec![byte::default(); l];
        for i in 0..a1.length {
            result[i] = a1[i];
        }
        for i in 0..a2.length {
            result[i + a1.length] = a2[i];
        }
        return result;
    }

    pub fn extractRSAOAEPSeed(cipherText: Vec<byte>, privateKey: RSAPrivateKey) -> Vec<Vec<byte>> {
        let modulus = privateKey.getModulus();
        let keySize = modulus.bitLength();
        let d = privateKey.getPrivateExponent();

        let cipherTextPadded = vec![byte::default(); cipherText.length + 1];
        cipherTextPadded[1..1 + cipherText.length].clone_from_slice(&cipherText);
        cipherTextPadded[0] = 0;

        let c = BigInteger::new(cipherText);
        c = BigInteger::new(cipherTextPadded);

        let product = BigInteger.ONE;
        for i in (0..=keySize - 1).rev() {
            product = product.multiply(product).modulo(modulus);
            let bit = d.testBit(i);
            if bit {
                product = product.multiply(c).modulo(modulus);
            }
        }

        let hlen = 32;
        let maskedDBLength = keySize / 8 - hlen - 1;

        let encodedMessageBytes = product.toByteArray();

        if encodedMessageBytes.length > keySize / 8 {
            encodedMessageBytes = encodedMessageBytes[1..].to_vec();
        } else {
            while (encodedMessageBytes.length < keySize / 8) {
                encodedMessageBytes = concat(vec![0], encodedMessageBytes);
            }
        }

        let maskedSeed = encodedMessageBytes[1..hlen + 2].to_vec();
        let maskedDb = encodedMessageBytes[hlen + 1..].to_vec();

        let seedMask = mgf(maskedDb, hlen, hlen);
        let mut seed = seedMask.clone();
        for i in 0..hlen {
            seed[i] ^= maskedSeed[i];
        }

        let mut dbMask = mgf(seed, keySize / 8 - hlen - 1, hlen);
        dbMask = dbMask[..keySize / 8 - hlen - 1].to_vec();

        let DB = vec![byte::default(); dbMask.length + 1]; // appending a zero to the left, to avoid sign issues in the BigInteger
        DB[1..1 + maskedDBLength].clone_from_slice(&maskedDb);
        for i in 0..maskedDBLength {
            DB[i + 1] ^= dbMask[i];
        }
        //		let dbInt = BigInteger::new(DB);

        let shift1 = 0;
        while (DB[shift1] == 0) {
            shift1 += 1;
        }
        let idx = 32 + shift1;
        while (DB[idx] == 0) {
            idx += 1;
        }
        let plaintext = DB[idx + 1..].to_vec();
        let result = vec![vec![plaintext, seed]];
        return result;
    }
}
