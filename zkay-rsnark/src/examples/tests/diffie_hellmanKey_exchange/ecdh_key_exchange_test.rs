
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::diffie_hellman_key_exchange::ecdh_key_exchange_gadget;

/**
 * Tests Key Exchange via Elliptic curve Gadget (ECDHKeyExchangeGadget.java) 

 */

pub struct ECDHKeyExchange_Test extends TestCase {

	
	// The sage script to compute the sample case is commented in the end of the file.
	// TODO: Add more test cases
	
	@Test
	pub   testVariableInputCase() {
		
		CircuitGenerator generator = CircuitGenerator::new("ECDH_Test") {

			i32 exponentBitlength = ECDHKeyExchangeGadget.SECRET_BITWIDTH;			
			 Vec<Wire> secretBits;
			 Wire baseX;
			 Wire hX;
			
			
			  fn buildCircuit() {
				
				secretBits = createInputWireArray(exponentBitlength, "exponent");
				baseX = createInputWire();
				hX = createInputWire();
				

				ECDHKeyExchangeGadget keyExchangeGadget = 
						ECDHKeyExchangeGadget::new(baseX, hX, secretBits);

				makeOutput(keyExchangeGadget.getOutputPublicValue());		
				
				// Just for testing. In real scenarios, this should not be made pub 
				makeOutput(keyExchangeGadget.getSharedSecret());
				
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				
				evaluator.setWireValue(baseX, BigInteger::new("4"));
				evaluator.setWireValue(hX, BigInteger::new("21766081959050939664800904742925354518084319102596785077490863571049214729748"));
				
				BigInteger exponent = BigInteger::new("13867691842196510828352345865165018381161315605899394650350519162543016860992");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(secretBits[i],if  exponent.testBit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();
		ArrayList<Wire> output = generator.getOutWires();

		assertEquals(evaluator.getWireValue(output.get(0)), BigInteger::new("13458082339735734368462130456283583571822918321676509705348825437102113182254"));
		assertEquals(evaluator.getWireValue(output.get(1)), BigInteger::new("4167917227796707610764894996898236918915412447839980711033808347811701875717"));	
	}
	

	@Test
	pub   testHardcodedInputCase() {
		
		CircuitGenerator generator = CircuitGenerator::new("ECDH_Test2") {


			i32 exponentBitlength = ECDHKeyExchangeGadget.SECRET_BITWIDTH;			
			 Vec<Wire> secretBits;
			 Wire baseX;
			 Wire hX;
			
			
			  fn buildCircuit() {
				
				secretBits = createInputWireArray(exponentBitlength, "exponent");
				baseX = createConstantWire(BigInteger::new("4"));
				hX = createConstantWire(BigInteger::new("21766081959050939664800904742925354518084319102596785077490863571049214729748"));

				ECDHKeyExchangeGadget keyExchangeGadget = 
						ECDHKeyExchangeGadget::new(baseX, hX, secretBits);

				makeOutput(keyExchangeGadget.getOutputPublicValue());		
				
				// Just for testing. In real scenarios, this should not be made pub 
				makeOutput(keyExchangeGadget.getSharedSecret());
				
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				

				BigInteger exponent = BigInteger::new("13867691842196510828352345865165018381161315605899394650350519162543016860992");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(secretBits[i],if  exponent.testBit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
		CircuitEvaluator evaluator = generator.getCircuitEvaluator();
		ArrayList<Wire> output = generator.getOutWires();

		assertEquals(evaluator.getWireValue(output.get(0)), BigInteger::new("13458082339735734368462130456283583571822918321676509705348825437102113182254"));
		assertEquals(evaluator.getWireValue(output.get(1)), BigInteger::new("4167917227796707610764894996898236918915412447839980711033808347811701875717"));	
	}

	
	@Test
	pub   testInputValidation1() {
		
		CircuitGenerator generator = CircuitGenerator::new("ECDH_Test_InputValidation") {


			i32 exponentBitlength = ECDHKeyExchangeGadget.SECRET_BITWIDTH;			
			 Vec<Wire> secretBits;
			 Wire baseX;
			 Wire hX;
			
			
			  fn buildCircuit() {
				
				secretBits = createInputWireArray(exponentBitlength, "exponent");
				baseX = createInputWire();
				hX = createInputWire();
				

				ECDHKeyExchangeGadget keyExchangeGadget = 
						ECDHKeyExchangeGadget::new(baseX, hX, secretBits);

				keyExchangeGadget.validateInputs();
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				
				evaluator.setWireValue(baseX, BigInteger::new("4"));
				evaluator.setWireValue(hX, BigInteger::new("21766081959050939664800904742925354518084319102596785077490863571049214729748"));
				
				BigInteger exponent = BigInteger::new("13867691842196510828352345865165018381161315605899394650350519162543016860992");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(secretBits[i],if  exponent.testBit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();

		// if no exception get thrown we are ok
	}
	
	

	pub   testInputValidation2() {
		
		
		// try invalid input
		CircuitGenerator generator = CircuitGenerator::new("ECDH_Test_InputValidation2") {


			i32 exponentBitlength = ECDHKeyExchangeGadget.SECRET_BITWIDTH;			
			 Vec<Wire> secretBits;
			 Wire baseX;
			 Wire hX;
			
			
			  fn buildCircuit() {
				
				secretBits = createInputWireArray(exponentBitlength, "exponent");
				baseX = createInputWire();
				hX = createInputWire();


				ECDHKeyExchangeGadget keyExchangeGadget = 
						ECDHKeyExchangeGadget::new(baseX, baseX, hX, hX, secretBits);

				keyExchangeGadget.validateInputs();
			}

			
			pub   generateSampleInput(CircuitEvaluator evaluator) {
				
				// invalid
				evaluator.setWireValue(baseX, BigInteger::new("14"));
				evaluator.setWireValue(hX, BigInteger::new("21766081959050939664800904742925354518084319102596785077490863571049214729748"));
				
				BigInteger exponent = BigInteger::new("13867691842196510828352345865165018381161315605899394650350519162543016860992");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(secretBits[i],if  exponent.testBit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();

		// we expect an exception somewhere
		try{
			generator.evalCircuit();
			assertTrue(false);		
		} catch(Exception e){
			println!("Exception Expected!");
			assertTrue(true);
		}
		
		// TODO: test more error conditions
	}
	
	
	
//		Sage Script generating the above values:
//		
//		p = 21888242871839275222246405745257275088548364400416034343698204186575808495617
//		K.<a> = NumberField(x-1)
//		aa = 126932
//		E = EllipticCurve(GF(p),[0,aa,0,1,0])
//		print(E.order())
//		print(n(log(E.order(),2)))
//		print(n(log(2736030358979909402780800718157159386074658810754251464600343418943805806723,2)))
//		
//		secret = 13867691842196510828352345865165018381161315605899394650350519162543016860992
//		
//		base = E(4,  5854969154019084038134685408453962516899849177257040453511959087213437462470)
//		print(base*secret)
//		print(h*secret)
}
