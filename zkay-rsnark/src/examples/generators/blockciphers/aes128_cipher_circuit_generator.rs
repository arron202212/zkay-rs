

use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::blockciphers::aes128_cipher_gadget;


// A sample usage of the AES128 block cipher gadget
public class AES128CipherCircuitGenerator extends CircuitGenerator {

	private Wire[] inputs;
	private Wire[] key;
	private Wire[] outputs;
	private AES128CipherGadget gadget;

	public AES128CipherCircuitGenerator(String circuitName) {
		super(circuitName);
	}

	
	protected void buildCircuit() {
		inputs = createInputWireArray(16); // in bytes
		key = createInputWireArray(16); // in bytes

		Wire[] expandedKey = AES128CipherGadget.expandKey(key);
		gadget = new AES128CipherGadget(inputs, expandedKey, "");
		outputs = gadget.getOutputWires();
		for (Wire o : outputs) {
			makeOutput(o);
		}

	}

	
	public void generateSampleInput(CircuitEvaluator circuitEvaluator) {

		BigInteger keyV = new BigInteger("2b7e151628aed2a6abf7158809cf4f3c", 16);
		BigInteger msgV = new BigInteger("ae2d8a571e03ac9c9eb76fac45af8e51", 16);

		// expected output:0xf5d3d58503b9699de785895a96fdbaaf

		byte[] keyArray = keyV.toByteArray();
		byte[] msgArray = msgV.toByteArray();
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

	public static void main(String[] args)  {

		Config.hexOutputEnabled = true;
		AES128CipherCircuitGenerator generator = new AES128CipherCircuitGenerator(
				"AES_Circuit");
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();

	}
}