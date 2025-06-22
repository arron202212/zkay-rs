

use crate::circuit::auxiliary::long_element;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;

use crate::util::util::{Util,BigInteger};
use zkay::zkay_paillier_dec_gadget;
use zkay::zkay_paillier_enc_gadget;
use zkay::zkay_paillier_fast_dec_gadget;
use zkay::zkay_paillier_fast_enc_gadget;



pub struct PaillierTests {

	
	pub   testEncryptionExample() {
		let plain = BigInteger::new("42");
		let random = BigInteger::new("25");
		let n = BigInteger::new("9047");
		let mut generator = BigInteger::new("27");
		let enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
		let cipher = enc.computeResult();
		assertEquals(BigInteger::new("45106492"), cipher);
	}

	
	pub   testDecryptionExample() {
		let n = BigInteger::new("9047");
		let cipher = BigInteger::new("2587834");
		let lambda = BigInteger::new("4428");
		let mu = BigInteger::new("1680");
		let dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
		let plain = dec.computeResult();
		assertEquals(BigInteger::new("55"), plain);
	}

	
	pub   test256BitEncryption() {
		let plain = BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505");
		let random = BigInteger::new("66895129274476067543864711343178574027057505369800972938068894913816799963509");
		let n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		let mut generator = BigInteger::new("27");
		let enc = PaillierEncCircuitGenerator::new("Paillier Enc", plain, random, n, generator);
		let cipher = enc.computeResult();
		assertEquals(BigInteger::new("3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963"), cipher);
	}

	
	pub   test256BitDecryption() {
		let n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		let cipher = BigInteger::new("3507594166975424775795724429703273237581693482251350761249288990776233360058698524194928568270852256828927631672223419615120374443722184016172266681685963");
		let lambda = BigInteger::new("35852839167575522071857348954969382050854184697828828629810896599748215236036");
		let mu = BigInteger::new("38822179779668243734206910236945399376867932682990009748733172869327079310544");
		let dec = PaillierDecCircuitGenerator::new("Paillier Dec", cipher, n, lambda, mu);
		let plain = dec.computeResult();
		assertEquals(BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505"), plain);
	}

	
	pub   test256BitFastEncryption() {
		let plain = BigInteger::new("58620521968995858419238449046464883186412581610038046858008683322252437292505");
		let random = BigInteger::new("66895129274476067543864711343178574027057505369800972938068894913816799963509");
		let n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		let enc = PaillierFastEncCircuitGenerator::new("Paillier Enc", n, plain, random);
		let cipher = enc.computeResult();
		assertEquals(BigInteger::new("3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136"), cipher);
	}

	
	pub   test256BitFastDecryption() {
		let n = BigInteger::new("71705678335151044143714697909938764102247769560297862447809589632641441407751");
		let lambda = BigInteger::new("71705678335151044143714697909938764101708369395657657259621793199496430472072");
		let cipher = BigInteger::new("3505470225408264473467386810920807437821858174488064393364776746993551415781505226520807868351169269605924531821264861279222635802527118722105662515867136");
		let dec = PaillierFastDecCircuitGenerator::new("Paillier Dec", n, lambda, cipher);
		let plain = dec.computeResult();
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
			plainWire = createLongElementInput(max(plain.bits(), 1), "plain");
			randomWire = createLongElementInput(max(random.bits(), 1), "random");
			let nBits = max(n.bits(), 1);
			nWire = createLongElementInput(nBits, "n");
			generatorWire = createLongElementInput(max(generator.bits(), 1), "generator");
			let enc = ZkayPaillierEncGadget::new(nWire, nBits, generatorWire, plainWire, randomWire);
			makeOutputArray(enc.getOutputWires(), "cipher");
		}

		
		pub  fn generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(plainWire, plain, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(randomWire, random, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(generatorWire, generator, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			let t1 = System.nanoTime();
			generateCircuit();
			let t2 = System.nanoTime();
			let ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			let evaluator = getCircuitEvaluator();
			let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None;0]));
			Util::group(outValues, LongElement.CHUNK_BITWIDTH)
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
			cipherWire = createLongElementInput(max(cipher.bits(), 1), "cipher");
			let nBits = max(n.bits(), 1);
			nWire = createLongElementInput(nBits, "n");
			lambdaWire = createLongElementInput(max(lambda.bits(), 1), "lambda");
			muWire = createLongElementInput(max(mu.bits(), 1), "mu");
			let dec = ZkayPaillierDecGadget::new(nWire, nBits, lambdaWire, muWire, cipherWire);
			makeOutputArray(dec.getOutputWires(), "plain");
		}

		
		pub  fn generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(cipherWire, cipher, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(lambdaWire, lambda, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(muWire, mu, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			let t1 = System.nanoTime();
			generateCircuit();
			let t2 = System.nanoTime();
			let ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			let evaluator = getCircuitEvaluator();
			let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None;0]));
			Util::group(outValues, LongElement.CHUNK_BITWIDTH)
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
			let nBits = max(n.bits(), 1);
			nWire = createLongElementInput(nBits, "n");
			plainWire = createLongElementInput(max(plain.bits(), 1), "plain");
			randomWire = createLongElementInput(max(random.bits(), 1), "random");
			let enc = ZkayPaillierFastEncGadget::new(nWire, nBits, plainWire, randomWire);
			makeOutputArray(enc.getOutputWires(), "cipher");
		}

		
		pub  fn generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(plainWire, plain, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(randomWire, random, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			let t1 = System.nanoTime();
			generateCircuit();
			let t2 = System.nanoTime();
			let ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			let evaluator = getCircuitEvaluator();
			let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None;0]));
			Util::group(outValues, LongElement.CHUNK_BITWIDTH)
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
			let nBits = max(n.bits(), 1);
			nWire = createLongElementInput(nBits, "n");
			lambdaWire = createLongElementInput(max(lambda.bits(), 1), "lambda");
			cipherWire = createLongElementInput(max(cipher.bits(), 1), "cipher");
			let dec = ZkayPaillierFastDecGadget::new(nWire, nBits, lambdaWire, cipherWire);
			makeOutputArray(dec.getOutputWires(), "plain");
		}

		
		pub  fn generateSampleInput(CircuitEvaluator evaluator) {
			evaluator.setWireValue(nWire, n, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(lambdaWire, lambda, LongElement.CHUNK_BITWIDTH);
			evaluator.setWireValue(cipherWire, cipher, LongElement.CHUNK_BITWIDTH);
		}

		pub  BigInteger computeResult() {
			let t1 = System.nanoTime();
			generateCircuit();
			let t2 = System.nanoTime();
			let ms = 1.e-6 * (t2 - t1);
			System.out.format("Building took %.3f ms\n", ms);
			evalCircuit();

			let evaluator = getCircuitEvaluator();
			let outValues = evaluator.getWiresValues(get_out_wires().toArray(vec![None;0]));
			Util::group(outValues, LongElement.CHUNK_BITWIDTH)
		}
	}
}
