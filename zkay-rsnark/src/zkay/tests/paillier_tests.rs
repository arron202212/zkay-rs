

use circuit::auxiliary::long_element;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;

use util::util;
use zkay::zkay_paillier_dec_gadget;
use zkay::zkay_paillier_enc_gadget;
use zkay::zkay_paillier_fast_dec_gadget;
use zkay::zkay_paillier_fast_enc_gadget;



pub struct PaillierTests {

	@Test
	pub   testEncryptionExample() {
		BigInteger plain = BigInteger::new("42");
		BigInteger random = BigInteger::new("25");
		BigInteger n = BigInteger::new("9047");
		BigInteger generator = BigInteger::new("27");
		PaillierEncCircuitGenerator enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
		BigInteger cipher = enc.computeResult();
		assertEquals(BigInteger::new("45106492"), cipher);
	}

	@Test
	pub   testDecryptionExample() {
		BigInteger n = BigInteger::new("9047");
		BigInteger cipher = BigInteger::new("2587834");
		BigInteger lambda = BigInteger::new("4428");
		BigInteger mu = BigInteger::new("1680");
		PaillierDecCircuitGenerator dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
		BigInteger plain = dec.computeResult();
		assertEquals(BigInteger::new("55"), plain);
	}

	@Test
	pub   test256BitEncryption() {
		BigInteger plain = BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505");
		BigInteger random = BigInteger::new("66895129274476067543864711343178574027057505369800972938068894913816799963509");
		BigInteger n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		BigInteger generator = BigInteger::new("27");
		PaillierEncCircuitGenerator enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
		BigInteger cipher = enc.computeResult();
		assertEquals(BigInteger::new("3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963"), cipher);
	}

	@Test
	pub   test256BitDecryption() {
		BigInteger n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		BigInteger cipher = BigInteger::new("3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963");
		BigInteger lambda = BigInteger::new("35852839167575522071857348954969382050854184697828828629810896599748215236036");
		BigInteger mu = BigInteger::new("38822179779668243734206910236945399376867932682990009748733172869327079310544");
		PaillierDecCircuitGenerator dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
		BigInteger plain = dec.computeResult();
		assertEquals(BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505"), plain);
	}

	@Test
	pub   test256BitFastEncryption() {
		BigInteger plain = BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505");
		BigInteger random = BigInteger::new("66895129274476067543864711343178574027057505369800972938068894913816799963509");
		BigInteger n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		PaillierFastEncCircuitGenerator enc = PaillierFastEncCircuitGenerator::new("Paillier Enc", n, plain, random);
		BigInteger cipher = enc.computeResult();
		assertEquals(BigInteger::new("3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136"), cipher);
	}

	@Test
	pub   test256BitFastDecryption() {
		BigInteger n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		BigInteger lambda = BigInteger::new("71705678335151044143714697909938764101708369395657657259621793199496430472072");
		BigInteger cipher = BigInteger::new("3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136");
		PaillierFastDecCircuitGenerator dec = PaillierFastDecCircuitGenerator::new("Paillier Dec", n, lambda, cipher);
		BigInteger plain = dec.computeResult();
		assertEquals(BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505"), plain);
	}

	// Don't look. Here lies the Land of Copy & Paste

	  class PaillierEncCircuitGenerator extends CircuitGenerator {

		 BigInteger plain;
		 BigInteger random;
		 BigInteger n;
		 BigInteger generator;

		 LongElement plainWire;
		 LongElement randomWire;
		 LongElement nWire;
		 LongElement generatorWire;

		 PaillierEncCircuitGenerator(String name, BigInteger plain, BigInteger random,
		                                    BigInteger n, BigInteger generator) {
			super(name);
			self.plain = plain;
			self.random = random;
			self.n = n;
			self.generator = generator;
		}

		
		  fn buildCircuit() {
			plainWire = createLongElementInput(max(plain.bitLength(), 1), "plain");
			randomWire = createLongElementInput(max(random.bitLength(), 1), "random");
			i32 nBits = max(n.bitLength(), 1);
			nWire = createLongElementInput(nBits, "n");
			generatorWire = createLongElementInput(max(generator.bitLength(), 1), "generator");
			ZkayPaillierEncGadget enc = ZkayPaillierEncGadget::new(nWire, nBits, generatorWire, plainWire, randomWire);
			makeOutputArray(enc.getOutputWires(), "cipher");
		}

		
		pub   generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(plainWire, plain, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(randomWire, random, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(generatorWire, generator, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			long t1 = System.nanoTime();
			generateCircuit();
			long t2 = System.nanoTime();
			double ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			CircuitEvaluator evaluator = getCircuitEvaluator();
			Vec<BigInteger> outValues = evaluator.getWiresValues(getOutWires().toArray(vec![Wire::default();0]));
			return Util::group(outValues, LongElement.CHUNK_BITWIDTH);
		}
	}

	  class PaillierDecCircuitGenerator extends CircuitGenerator {

		 BigInteger cipher;
		 BigInteger n;
		 BigInteger lambda;
		 BigInteger mu;

		 LongElement cipherWire;
		 LongElement nWire;
		 LongElement lambdaWire;
		 LongElement muWire;

		 PaillierDecCircuitGenerator(String name, BigInteger cipher, BigInteger n,
		                                    BigInteger lambda, BigInteger mu) {
			super(name);
			self.cipher = cipher;
			self.n = n;
			self.lambda = lambda;
			self.mu = mu;
		}

		
		  fn buildCircuit() {
			cipherWire = createLongElementInput(max(cipher.bitLength(), 1), "cipher");
			i32 nBits = max(n.bitLength(), 1);
			nWire = createLongElementInput(nBits, "n");
			lambdaWire = createLongElementInput(max(lambda.bitLength(), 1), "lambda");
			muWire = createLongElementInput(max(mu.bitLength(), 1), "mu");
			ZkayPaillierDecGadget dec = ZkayPaillierDecGadget::new(nWire, nBits, lambdaWire, muWire, cipherWire);
			makeOutputArray(dec.getOutputWires(), "plain");
		}

		
		pub   generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(cipherWire, cipher, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(lambdaWire, lambda, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(muWire, mu, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			long t1 = System.nanoTime();
			generateCircuit();
			long t2 = System.nanoTime();
			double ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			CircuitEvaluator evaluator = getCircuitEvaluator();
			Vec<BigInteger> outValues = evaluator.getWiresValues(getOutWires().toArray(vec![Wire::default();0]));
			return Util::group(outValues, LongElement.CHUNK_BITWIDTH);
		}
	}

	  class PaillierFastEncCircuitGenerator extends CircuitGenerator {

		 BigInteger n;
		 BigInteger plain;
		 BigInteger random;

		 LongElement nWire;
		 LongElement plainWire;
		 LongElement randomWire;

		 PaillierFastEncCircuitGenerator(String name, BigInteger n, BigInteger plain, BigInteger random) {
			super(name);
			self.n = n;
			self.plain = plain;
			self.random = random;
		}

		
		  fn buildCircuit() {
			i32 nBits = max(n.bitLength(), 1);
			nWire = createLongElementInput(nBits, "n");
			plainWire = createLongElementInput(max(plain.bitLength(), 1), "plain");
			randomWire = createLongElementInput(max(random.bitLength(), 1), "random");
			ZkayPaillierFastEncGadget enc = ZkayPaillierFastEncGadget::new(nWire, nBits, plainWire, randomWire);
			makeOutputArray(enc.getOutputWires(), "cipher");
		}

		
		pub   generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(plainWire, plain, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(randomWire, random, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			long t1 = System.nanoTime();
			generateCircuit();
			long t2 = System.nanoTime();
			double ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			CircuitEvaluator evaluator = getCircuitEvaluator();
			Vec<BigInteger> outValues = evaluator.getWiresValues(getOutWires().toArray(vec![Wire::default();0]));
			return Util::group(outValues, LongElement.CHUNK_BITWIDTH);
		}
	}

	  class PaillierFastDecCircuitGenerator extends CircuitGenerator {

		 BigInteger n;
		 BigInteger lambda;
		 BigInteger cipher;

		 LongElement nWire;
		 LongElement lambdaWire;
		 LongElement cipherWire;

		 PaillierFastDecCircuitGenerator(String name, BigInteger n, BigInteger lambda, BigInteger cipher) {
			super(name);
			self.n = n;
			self.lambda = lambda;
			self.cipher = cipher;
		}

		
		  fn buildCircuit() {
			i32 nBits = max(n.bitLength(), 1);
			nWire = createLongElementInput(nBits, "n");
			lambdaWire = createLongElementInput(max(lambda.bitLength(), 1), "lambda");
			cipherWire = createLongElementInput(max(cipher.bitLength(), 1), "cipher");
			ZkayPaillierFastDecGadget dec = ZkayPaillierFastDecGadget::new(nWire, nBits, lambdaWire, cipherWire);
			makeOutputArray(dec.getOutputWires(), "plain");
		}

		
		pub   generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(lambdaWire, lambda, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(cipherWire, cipher, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			long t1 = System.nanoTime();
			generateCircuit();
			long t2 = System.nanoTime();
			double ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			CircuitEvaluator evaluator = getCircuitEvaluator();
			Vec<BigInteger> outValues = evaluator.getWiresValues(getOutWires().toArray(vec![Wire::default();0]));
			return Util::group(outValues, LongElement.CHUNK_BITWIDTH);
		}
	}
}
