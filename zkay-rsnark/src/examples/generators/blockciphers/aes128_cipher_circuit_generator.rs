

use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;


// A sample usage of the AES128 block cipher gadget
pub struct AES128CipherCircuitGenerator extends CircuitGenerator {

	 Vec<Wire> inputs;
	 Vec<Wire> key;
	 Vec<Wire> outputs;
	 AES128CipherGadget gadget;

	pub  AES128CipherCircuitGenerator(String circuitName) {
		super(circuitName);
	}

	
	  fn buildCircuit() {
		inputs = createInputWireArray(16); // in bytes
		key = createInputWireArray(16); // in bytes

		Vec<Wire> expandedKey = AES128CipherGadget.expandKey(key);
		gadget = AES128CipherGadget::new(inputs, expandedKey, "");
		outputs = gadget.getOutputWires();
		for o in outputs {
			makeOutput(o);
		}

	}

	
	pub   generateSampleInput(CircuitEvaluator circuitEvaluator) {

		BigInteger keyV = BigInteger::new("2b7e151628aed2a6abf7158809cf4f3c", 16);
		BigInteger msgV = BigInteger::new("ae2d8a571e03ac9c9eb76fac45af8e51", 16);

		// expected output:0xf5d3d58503b9699de785895a96fdbaaf

		Vec<byte> keyArray = keyV.toByteArray();
		Vec<byte> msgArray = msgV.toByteArray();
		msgArray = Arrays.copyOfRange(msgArray, msgArray.length - 16,
				msgArray.length);
		keyArray = Arrays.copyOfRange(keyArray, keyArray.length - 16,
				keyArray.length);

		for i in 0..msgArray.length {
			circuitEvaluator.setWireValue(inputs[i], (msgArray[i] & 0xff));
		}

		for i in 0..keyArray.length {
			circuitEvaluator.setWireValue(key[i], (keyArray[i] & 0xff));
		}
	}

	pub    main(args:Vec<String>)  {

		Config.hexOutputEnabled = true;
		AES128CipherCircuitGenerator generator = AES128CipherCircuitGenerator::new(
				"AES_Circuit");
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();

	}
}