
/**
 * Utility methods to extract sample randomness used by standard implementations
 * for RSA Encryption. In absence of test vectors, the extracted randomness is
 * used to test our RSA gadgets to make sure the RSA circuits match the standard
 * implementations.
 *
 */

pub struct RSAUtil {

	pub   Vec<Vec<byte>> extractRSARandomness1_5(cipherText:Vec<byte>,
			RSAPrivateKey privateKey) {

		BigInteger modulus = privateKey.getModulus();
		i32 keySize = modulus.bitLength();
		BigInteger d = privateKey.getPrivateExponent();

		Vec<byte> cipherTextPadded = vec![byte::default();cipherText.length + 1];
		System.arraycopy(cipherText, 0, cipherTextPadded, 1, cipherText.length);
		cipherTextPadded[0] = 0;

		BigInteger c = BigInteger::new(cipherText);
		c = BigInteger::new(cipherTextPadded);
		BigInteger product = BigInteger.ONE;
		for i in (0..=keySize - 1).rev()
			product = product.multiply(product).mod(modulus);
			bool bit = d.testBit(i);
			if bit
				product = product.multiply(c).mod(modulus);
		}

//		println!("After decryption manually = "
//				+ product.toString(16));

		Vec<byte> paddedPlaintext = product.toByteArray();
		if paddedPlaintext.length != keySize / 8 - 1 {
			println!("Error");
			return null;
		}
		Vec<byte> plaintext = null;
		Vec<byte> randomness = null;

		if paddedPlaintext[0] != 2 {
			println!("Error");
		} else {
			for i in 1..keySize / 8 - 2{
				if paddedPlaintext[i] != 0 {
					continue;
				} else {
					plaintext = vec![byte::default();(keySize / 8 - 2) - i];
					randomness = vec![byte::default();i - 1];
					System.arraycopy(paddedPlaintext, i + 1, plaintext, 0,
							plaintext.length);
					System.arraycopy(paddedPlaintext, 1, randomness, 0,
							randomness.length);

					break;
				}
			}
		}
		Vec<Vec<byte>> result = vec![byte::default();][] { plaintext, randomness };
		return result;
	}

	  Vec<byte> intToByteArray(i32 value) {
		return vec![byte::default();] { (byte) (value >>> 24), (byte) (value >>> 16),
				(byte) (value >>> 8), (byte) value };
	}

	  Vec<byte> mgf(array:Vec<byte>, i32 maskLen, i32 hlen) {

		Vec<byte> v = vec![byte::default();0];
		for i in 0..=((i32) Math.ceil(maskLen * 1.0 / hlen)) - 1{
			Vec<byte> c = intToByteArray(i);
			MessageDigest hash = null;
			try {
				hash = MessageDigest.getInstance("SHA-256");
			} catch (Exception e) {
				e.printStackTrace();
			}
			hash.update(concat(array, c));
			Vec<byte> digest = hash.digest();
			hash.reset();
			v = concat(v, digest);
		}
		return v;
	}

	  Vec<byte> concat(a1:Vec<byte>, a2:Vec<byte>) {
		i32 l = a1.length + a2.length;
		Vec<byte> result = vec![byte::default();l];
		for i in 0..a1.length {
			result[i] = a1[i];
		}
		for i in 0..a2.length {
			result[i + a1.length] = a2[i];
		}
		return result;
	}

	pub   Vec<Vec<byte>> extractRSAOAEPSeed(cipherText:Vec<byte>,
			RSAPrivateKey privateKey) {

		BigInteger modulus = privateKey.getModulus();
		i32 keySize = modulus.bitLength();
		BigInteger d = privateKey.getPrivateExponent();

		Vec<byte> cipherTextPadded = vec![byte::default();cipherText.length + 1];
		System.arraycopy(cipherText, 0, cipherTextPadded, 1, cipherText.length);
		cipherTextPadded[0] = 0;

		BigInteger c = BigInteger::new(cipherText);
		c = BigInteger::new(cipherTextPadded);

		BigInteger product = BigInteger.ONE;
		for i in (0..=keySize - 1).rev()
			product = product.multiply(product).mod(modulus);
			bool bit = d.testBit(i);
			if bit
				product = product.multiply(c).mod(modulus);
		}

		i32 hlen = 32;
		i32 maskedDBLength = keySize / 8 - hlen - 1;

		Vec<byte> encodedMessageBytes = product.toByteArray();

		if encodedMessageBytes.length > keySize / 8 {
			encodedMessageBytes = Arrays.copyOfRange(encodedMessageBytes, 1,
					encodedMessageBytes.length);
		} else {
			while (encodedMessageBytes.length < keySize / 8) {
				encodedMessageBytes = concat(vec![byte::default();] { 0 },
						encodedMessageBytes);
			}
		}

		Vec<byte> maskedSeed = Arrays
				.copyOfRange(encodedMessageBytes, 1, hlen + 1);
		Vec<byte> maskedDb = Arrays.copyOfRange(encodedMessageBytes, hlen + 1,
				encodedMessageBytes.length);

		Vec<byte> seedMask = mgf(maskedDb, hlen, hlen);
		Vec<byte> seed = Arrays.copyOf(seedMask, hlen);
		for i in 0..hlen {
			seed[i] ^= maskedSeed[i];
		}

		Vec<byte> dbMask = mgf(seed, keySize / 8 - hlen - 1, hlen);
		dbMask= Arrays.copyOf(dbMask, keySize/8-hlen-1);

		Vec<byte> DB = vec![byte::default();dbMask.length + 1]; // appending a zero to the left, to avoid sign issues in the BigInteger
		System.arraycopy(maskedDb, 0, DB, 1, maskedDBLength);
		for i in 0..maskedDBLength {
			DB[i + 1] ^= dbMask[i];
		}
//		BigInteger dbInt = BigInteger::new(DB);

		i32 shift1 = 0;
		while (DB[shift1] == 0) {
			shift1+=1;
		}
		i32 idx = 32 + shift1;
		while (DB[idx] == 0) {
			idx+=1;
		}
		Vec<byte> plaintext = Arrays.copyOfRange(DB, idx + 1, DB.length);
		Vec<Vec<byte>> result = vec![byte::default();][] { plaintext, seed };
		return result;
	}

}
