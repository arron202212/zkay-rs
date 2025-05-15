
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;
use examples::gadgets::blockciphers::sbox.aes_s_box_gadget_optimized2;


pub struct AES128_Test  {

	
	pub   testCase1() {
		
		// key: "2b7e151628aed2a6abf7158809cf4f3c"
		// plaintext: "ae2d8a571e03ac9c9eb76fac45af8e51"
		// ciphertext: "f5d3d58503b9699de785895a96fdbaaf"
		
		// testing all available sBox implementations
for sboxOption in AES128CipherGadget.SBoxOption.values()
			
			AES128CipherGadget.sBoxOption = sboxOption;
			CircuitGenerator generator = CircuitGenerator::new("AES128_Test1_"+sboxOption) {
	
				 Vec<Wire> plaintext; // 16 bytes
				 Vec<Wire> key; // 16 bytes
				 Vec<Wire> ciphertext; // 16 bytes
	
				
				  fn buildCircuit() {
					plaintext = createInputWireArray(16);
					key = createInputWireArray(16);
let expandedKey = AES128CipherGadget.expandKey(key);
					ciphertext = AES128CipherGadget::new(plaintext, expandedKey)
							.getOutputWires();
					makeOutputArray(ciphertext);
				}
	
				
				pub  fn generateSampleInput(CircuitEvaluator evaluator) {
	
					BigInteger keyV = BigInteger::new(
							"2b7e151628aed2a6abf7158809cf4f3c", 16);
					BigInteger msgV = BigInteger::new(
							"ae2d8a571e03ac9c9eb76fac45af8e51", 16);
	
let keyArray = keyV.toByteArray();
let msgArray = msgV.toByteArray();
					msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
							msgArray.length);
					keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
							keyArray.length);
	
					for i in 0..plaintext.length {
						evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
					}
					for i in 0..key.length {
						evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
					}
				}
			};
	
			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();
	
			BigInteger result = BigInteger::new("f5d3d58503b9699de785895a96fdbaaf",
					16);
		
let resultArray = result.toByteArray();
			resultArray = Arrays.copyOfRange(resultArray, resultArray.length - 16,
					resultArray.length);
	
			for i in 0..16 {
				assertEquals(evaluator.getWireValue(cipherText.get(i)),
						BigInteger.valueOf((resultArray[i] + 256) % 256));
			}
		}
	}
	
	
	
	pub   testCase2() {
		
		// key: "2b7e151628aed2a6abf7158809cf4f3c"
		// plaintext: "6bc1bee22e409f96e93d7e117393172a"
		// ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"
		
		// testing all available sBox implementations
for sboxOption in AES128CipherGadget.SBoxOption.values()
			
			AES128CipherGadget.sBoxOption = sboxOption;
			CircuitGenerator generator = CircuitGenerator::new("AES128_Test2_"+sboxOption) {
	
				 Vec<Wire> plaintext; // 16 bytes
				 Vec<Wire> key; // 16 bytes
				 Vec<Wire> ciphertext; // 16 bytes
	
				
				  fn buildCircuit() {
					plaintext = createInputWireArray(16);
					key = createInputWireArray(16);
let expandedKey = AES128CipherGadget.expandKey(key);
					ciphertext = AES128CipherGadget::new(plaintext, expandedKey)
							.getOutputWires();
					makeOutputArray(ciphertext);
				}
	
				
				pub  fn generateSampleInput(CircuitEvaluator evaluator) {
	
					BigInteger keyV = BigInteger::new(
							"2b7e151628aed2a6abf7158809cf4f3c", 16);
					BigInteger msgV = BigInteger::new(
							"6bc1bee22e409f96e93d7e117393172a", 16);
	
let keyArray = keyV.toByteArray();
let msgArray = msgV.toByteArray();
					msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
							msgArray.length);
					keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
							keyArray.length);
	
					for i in 0..plaintext.length {
						evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
					}
					for i in 0..key.length {
						evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
					}
				}
			};
	
			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();
	
			BigInteger result = BigInteger::new("3ad77bb40d7a3660a89ecaf32466ef97",
					16);
	
			// expected output:0xf5d3d58503b9699de785895a96fdbaaf
	
let resultArray = result.toByteArray();
			resultArray = Arrays.copyOfRange(resultArray, resultArray.length - 16,
					resultArray.length);
	
			for i in 0..16 {
				assertEquals(evaluator.getWireValue(cipherText.get(i)),
						BigInteger.valueOf((resultArray[i] + 256) % 256));
			}
		}
	}

	
	pub   testCase3() {
		
		// key: "2b7e151628aed2a6abf7158809cf4f3c"
		// plaintext: "6bc1bee22e409f96e93d7e117393172a"
		// ciphertext: "3ad77bb40d7a3660a89ecaf32466ef97"
		
		// testing all available sBox implementations
for sboxOption in AES128CipherGadget.SBoxOption.values()
			
			AES128CipherGadget.sBoxOption = sboxOption;
			CircuitGenerator generator = CircuitGenerator::new("AES128_Test3_"+sboxOption) {
	
				 Vec<Wire> plaintext; // 16 bytes
				 Vec<Wire> key; // 16 bytes
				 Vec<Wire> ciphertext; // 16 bytes
	
				
				  fn buildCircuit() {
					plaintext = createInputWireArray(16);
					key = createInputWireArray(16);
let expandedKey = AES128CipherGadget.expandKey(key);
					ciphertext = AES128CipherGadget::new(plaintext, expandedKey)
							.getOutputWires();
					makeOutputArray(ciphertext);
				}
	
				
				pub  fn generateSampleInput(CircuitEvaluator evaluator) {
	
					BigInteger keyV = BigInteger::new(
							"2b7e151628aed2a6abf7158809cf4f3c", 16);
					BigInteger msgV = BigInteger::new(
							"30c81c46a35ce411e5fbc1191a0a52ef", 16);
	
let keyArray = keyV.toByteArray();
let msgArray = msgV.toByteArray();
					msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
							msgArray.length);
					keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
							keyArray.length);
	
					for i in 0..plaintext.length {
						evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
					}
					for i in 0..key.length {
						evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
					}
				}
			};
	
			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();
	
			BigInteger result = BigInteger::new("43b1cd7f598ece23881b00e3ed030688",
					16);
	
let resultArray = result.toByteArray();
			resultArray = Arrays.copyOfRange(resultArray, resultArray.length - 16,
					resultArray.length);
	
			for i in 0..16 {
				assertEquals(evaluator.getWireValue(cipherText.get(i)),
						BigInteger.valueOf((resultArray[i] + 256) % 256));
			}
		}
	}


	
	pub   testCase4() {
		
		// key: "2b7e151628aed2a6abf7158809cf4f3c"
		// plaintext: "30c81c46a35ce411e5fbc1191a0a52ef"
		// ciphertext: "43b1cd7f598ece23881b00e3ed030688"
		
		// testing all available sBox implementations
for sboxOption in AES128CipherGadget.SBoxOption.values()
			
			AES128CipherGadget.sBoxOption = sboxOption;
			CircuitGenerator generator = CircuitGenerator::new("AES128_Test4_"+sboxOption) {
	
				 Vec<Wire> plaintext; // 16 bytes
				 Vec<Wire> key; // 16 bytes
				 Vec<Wire> ciphertext; // 16 bytes
	
				
				  fn buildCircuit() {
					plaintext = createInputWireArray(16);
					key = createInputWireArray(16);
let expandedKey = AES128CipherGadget.expandKey(key);
					ciphertext = AES128CipherGadget::new(plaintext, expandedKey)
							.getOutputWires();
					makeOutputArray(ciphertext);
				}
	
				
				pub  fn generateSampleInput(CircuitEvaluator evaluator) {
	
					BigInteger keyV = BigInteger::new(
							"2b7e151628aed2a6abf7158809cf4f3c", 16);
					BigInteger msgV = BigInteger::new(
							"f69f2445df4f9b17ad2b417be66c3710", 16);
	
let keyArray = keyV.toByteArray();
let msgArray = msgV.toByteArray();
					msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
							msgArray.length);
					keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
							keyArray.length);
	
					for i in 0..plaintext.length {
						evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
					}
					for i in 0..key.length {
						evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
					}
				}
			};
	
			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();
	
			BigInteger result = BigInteger::new("7b0c785e27e8ad3f8223207104725dd4",
					16);

			
let resultArray = result.toByteArray();
			resultArray = Arrays.copyOfRange(resultArray, resultArray.length - 16,
					resultArray.length);
	
			for i in 0..16 {
				assertEquals(evaluator.getWireValue(cipherText.get(i)),
						BigInteger.valueOf((resultArray[i] + 256) % 256));
			}
		}
	}
	
	
	pub   testCustomSboxImplementation() {
		
		
		AES128CipherGadget.sBoxOption = AES128CipherGadget.SBoxOption.OPTIMIZED2;
		for  b in  0..= 15; b{
			
			AESSBoxGadgetOptimized2.setBitCount(b);
			AESSBoxGadgetOptimized2.solveLinearSystems();
			CircuitGenerator generator = CircuitGenerator::new("AES128_Test_SBox_Parametrization_"+b) {
	
				 Vec<Wire> plaintext; // 16 bytes
				 Vec<Wire> key; // 16 bytes
				 Vec<Wire> ciphertext; // 16 bytes
	
				
				  fn buildCircuit() {
					plaintext = createInputWireArray(16);
					key = createInputWireArray(16);
let expandedKey = AES128CipherGadget.expandKey(key);
					ciphertext = AES128CipherGadget::new(plaintext, expandedKey)
							.getOutputWires();
					makeOutputArray(ciphertext);
				}
	
				
				pub  fn generateSampleInput(CircuitEvaluator evaluator) {
	
					BigInteger keyV = BigInteger::new(
							"2b7e151628aed2a6abf7158809cf4f3c", 16);
					BigInteger msgV = BigInteger::new(
							"f69f2445df4f9b17ad2b417be66c3710", 16);
	
let keyArray = keyV.toByteArray();
let msgArray = msgV.toByteArray();
					msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
							msgArray.length);
					keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
							keyArray.length);
	
					for i in 0..plaintext.length {
						evaluator.setWireValue(plaintext[i], (msgArray[i] & 0xff));
					}
					for i in 0..key.length {
						evaluator.setWireValue(key[i], (keyArray[i] & 0xff));
					}
				}
			};
	
			generator.generateCircuit();
			generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let cipherText = generator.getOutWires();
	
			BigInteger result = BigInteger::new("7b0c785e27e8ad3f8223207104725dd4",
					16);

			
let resultArray = result.toByteArray();
			resultArray = Arrays.copyOfRange(resultArray, resultArray.length - 16,
					resultArray.length);
	
			for i in 0..16 {
				assertEquals(evaluator.getWireValue(cipherText.get(i)),
						BigInteger.valueOf((resultArray[i] + 256) % 256));
			}
		}
	}


	
}
