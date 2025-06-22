
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{CGConfig,CircuitGenerator,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::diffie_hellman_key_exchange::field_extension_dh_key_exchange;

/**
 * Tests Key Exchange via Field Extension Gadget (DHKeyExchangeGadget.java) 
 * Parameters used here assumes ~80-bit security
 */

pub struct FieldExtensionDHKeyExchange_Test  {

	
	// This is a very simple example for testing purposes. To see how key exchange gadgets could be used, 
	// check the HybridEncryptionCircuitGenerator
	
	// The sage script to compute the sample case is commented in the end of the file.
	
	
	pub   testHardcodedKeys() {
		
		CircuitGenerator generator = CircuitGenerator::new("FieldExtension_Test1") {

let mu = 4;
let omega = 7;
let exponentBitlength = 397;
			
			 Vec<Option<WireType>> exponentBits;
			
			
			  fn buildCircuit() {
				
				exponentBits = createInputWireArray(exponentBitlength, "exponent");

let g = vec![None;mu];
let h = vec![None;mu];

				// Hardcode the base and the other party's key (suitable when keys are not expected to change)
				g[0] = createConstantWire(BigInteger::new("16377448892084713529161739182205318095580119111576802375181616547062197291263"));
				g[1] = createConstantWire(BigInteger::new("13687683608888423916085091250849188813359145430644908352977567823030408967189"));
				g[2] = createConstantWire(BigInteger::new("12629166084120705167185476169390021031074363183264910102253898080559854363106"));
				g[3] = createConstantWire(BigInteger::new("19441276922979928804860196077335093208498949640381586557241379549605420212272"));

				h[0] = createConstantWire(BigInteger::new("8252578783913909531884765397785803733246236629821369091076513527284845891757"));
				h[1] = createConstantWire(BigInteger::new("20829599225781884356477513064431048695774529855095864514701692089787151865093"));
				h[2] = createConstantWire(BigInteger::new("1540379511125324102377803754608881114249455137236500477169164628692514244862"));
				h[3] = createConstantWire(BigInteger::new("1294177986177175279602421915789749270823809536595962994745244158374705688266"));

				FieldExtensionDHKeyExchange fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(g, h, exponentBits,
						omega, "");

let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
				makeOutputArray(g_to_s, "DH Key Exchange Output");
let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
				makeOutputArray(h_to_s, "Derived Secret Key");
			}

			
			pub  fn generateSampleInput(CircuitEvaluator evaluator) {
let exponent = BigInteger::new("151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(exponentBits[i],if  exponent.bit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let output = generator.get_out_wires();

		assertEquals(evaluator.getWireValue(output.get(0)), BigInteger::new("9327289243415079515318132023689497171271904433099600200400859968177425894580"));
		assertEquals(evaluator.getWireValue(output.get(1)), BigInteger::new("21312311033900790023937954575527091756377215260488498667283640904465223526236"));
		assertEquals(evaluator.getWireValue(output.get(2)), BigInteger::new("19883079534945520345012965173409210670280801176341700376612297932480562491904"));
		assertEquals(evaluator.getWireValue(output.get(3)), BigInteger::new("11262499765857836098986663841690204003097813561305051025968110590253003094192"));
		
		assertEquals(evaluator.getWireValue(output.get(4)), BigInteger::new("2202294410438304085016660740566673536814787951643742901558895317916637664703"));
		assertEquals(evaluator.getWireValue(output.get(5)), BigInteger::new("18724398730888665000453307259637219298475373267590805228665739285983831525279"));
		assertEquals(evaluator.getWireValue(output.get(6)), BigInteger::new("21875304682329937834628267681832507202983143541480299478306965773109713498819"));
		assertEquals(evaluator.getWireValue(output.get(7)), BigInteger::new("12006400062454647262588139453308241334465382550157910424084838650858146672647"));
	
	}
	
	
	pub   testVariableKeys() {
		
		CircuitGenerator generator = CircuitGenerator::new("FieldExtension_Test2") {

let mu = 4;
let omega = 7;
let exponentBitlength = 397;
			
			 Vec<Option<WireType>> exponentBits;
			 Vec<Option<WireType>> g;
			 Vec<Option<WireType>> h;

			
			  fn buildCircuit() {
				
				exponentBits = createInputWireArray(exponentBitlength, "exponent");

				g = createInputWireArray(mu);
				h = createInputWireArray(mu);

				FieldExtensionDHKeyExchange fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(g, h, exponentBits,
						omega, "");

let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
				makeOutputArray(g_to_s, "DH Key Exchange Output");
let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
				makeOutputArray(h_to_s, "Derived Secret Key");
			}

			
			pub  fn generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(g[0],BigInteger::new("16377448892084713529161739182205318095580119111576802375181616547062197291263"));
				evaluator.setWireValue(g[1],BigInteger::new("13687683608888423916085091250849188813359145430644908352977567823030408967189"));
				evaluator.setWireValue(g[2],BigInteger::new("12629166084120705167185476169390021031074363183264910102253898080559854363106"));
				evaluator.setWireValue(g[3],BigInteger::new("19441276922979928804860196077335093208498949640381586557241379549605420212272"));

				evaluator.setWireValue(h[0],BigInteger::new("8252578783913909531884765397785803733246236629821369091076513527284845891757"));
				evaluator.setWireValue(h[1],BigInteger::new("20829599225781884356477513064431048695774529855095864514701692089787151865093"));
				evaluator.setWireValue(h[2],BigInteger::new("1540379511125324102377803754608881114249455137236500477169164628692514244862"));
				evaluator.setWireValue(h[3],BigInteger::new("1294177986177175279602421915789749270823809536595962994745244158374705688266"));

let exponent = BigInteger::new("151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(exponentBits[i],if  exponent.bit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let output = generator.get_out_wires();


		
		assertEquals(evaluator.getWireValue(output.get(0)), BigInteger::new("9327289243415079515318132023689497171271904433099600200400859968177425894580"));
		assertEquals(evaluator.getWireValue(output.get(1)), BigInteger::new("21312311033900790023937954575527091756377215260488498667283640904465223526236"));
		assertEquals(evaluator.getWireValue(output.get(2)), BigInteger::new("19883079534945520345012965173409210670280801176341700376612297932480562491904"));
		assertEquals(evaluator.getWireValue(output.get(3)), BigInteger::new("11262499765857836098986663841690204003097813561305051025968110590253003094192"));
		
		assertEquals(evaluator.getWireValue(output.get(4)), BigInteger::new("2202294410438304085016660740566673536814787951643742901558895317916637664703"));
		assertEquals(evaluator.getWireValue(output.get(5)), BigInteger::new("18724398730888665000453307259637219298475373267590805228665739285983831525279"));
		assertEquals(evaluator.getWireValue(output.get(6)), BigInteger::new("21875304682329937834628267681832507202983143541480299478306965773109713498819"));
		assertEquals(evaluator.getWireValue(output.get(7)), BigInteger::new("12006400062454647262588139453308241334465382550157910424084838650858146672647"));
	
	}
	
	
	
	pub   testInputValidation() {
		
		CircuitGenerator generator = CircuitGenerator::new("FieldExtension_Test3") {

let mu = 4;
let omega = 7;
let exponentBitlength = 397;
			
			 Vec<Option<WireType>> exponentBits;
			 Vec<Option<WireType>> g;
			 Vec<Option<WireType>> h;

			
			  fn buildCircuit() {
				
				exponentBits = createInputWireArray(exponentBitlength, "exponent");

				g = createInputWireArray(mu);
				h = createInputWireArray(mu);

				FieldExtensionDHKeyExchange fieldExtensionDHKeyExchange = FieldExtensionDHKeyExchange::new(g, h, exponentBits,
						omega, "");

				// provide prime order subgroup
				fieldExtensionDHKeyExchange.validateInputs(BigInteger::new("566003748421165623973140684210338877916630960782201693595769129706864925719318115473892932098619423042929922932476493069"));
				
let g_to_s = fieldExtensionDHKeyExchange.getOutputPublicValue();
				makeOutputArray(g_to_s, "DH Key Exchange Output");
let h_to_s = fieldExtensionDHKeyExchange.getSharedSecret();
				makeOutputArray(h_to_s, "Derived Secret Key");
			}

			
			pub  fn generateSampleInput(CircuitEvaluator evaluator) {
				evaluator.setWireValue(g[0],BigInteger::new("16377448892084713529161739182205318095580119111576802375181616547062197291263"));
				evaluator.setWireValue(g[1],BigInteger::new("13687683608888423916085091250849188813359145430644908352977567823030408967189"));
				evaluator.setWireValue(g[2],BigInteger::new("12629166084120705167185476169390021031074363183264910102253898080559854363106"));
				evaluator.setWireValue(g[3],BigInteger::new("19441276922979928804860196077335093208498949640381586557241379549605420212272"));

				evaluator.setWireValue(h[0],BigInteger::new("8252578783913909531884765397785803733246236629821369091076513527284845891757"));
				evaluator.setWireValue(h[1],BigInteger::new("20829599225781884356477513064431048695774529855095864514701692089787151865093"));
				evaluator.setWireValue(h[2],BigInteger::new("1540379511125324102377803754608881114249455137236500477169164628692514244862"));
				evaluator.setWireValue(h[3],BigInteger::new("1294177986177175279602421915789749270823809536595962994745244158374705688266"));

let exponent = BigInteger::new("151828783241023778037546088811142494551372361892819281986925142448620047716812787162715261182186261271525615616651551515");
				for i in 0..exponentBitlength{
					evaluator.setWireValue(exponentBits[i],if  exponent.bit(i) {1}else {0});
				}
			}
		};

		generator.generateCircuit();
		generator.evalCircuit();
let evaluator = generator.getCircuitEvaluator();
let output = generator.get_out_wires();


		
		assertEquals(evaluator.getWireValue(output.get(0)), BigInteger::new("9327289243415079515318132023689497171271904433099600200400859968177425894580"));
		assertEquals(evaluator.getWireValue(output.get(1)), BigInteger::new("21312311033900790023937954575527091756377215260488498667283640904465223526236"));
		assertEquals(evaluator.getWireValue(output.get(2)), BigInteger::new("19883079534945520345012965173409210670280801176341700376612297932480562491904"));
		assertEquals(evaluator.getWireValue(output.get(3)), BigInteger::new("11262499765857836098986663841690204003097813561305051025968110590253003094192"));
		
		assertEquals(evaluator.getWireValue(output.get(4)), BigInteger::new("2202294410438304085016660740566673536814787951643742901558895317916637664703"));
		assertEquals(evaluator.getWireValue(output.get(5)), BigInteger::new("18724398730888665000453307259637219298475373267590805228665739285983831525279"));
		assertEquals(evaluator.getWireValue(output.get(6)), BigInteger::new("21875304682329937834628267681832507202983143541480299478306965773109713498819"));
		assertEquals(evaluator.getWireValue(output.get(7)), BigInteger::new("12006400062454647262588139453308241334465382550157910424084838650858146672647"));
	
	}
	
	/* Sage Script generating the above values:
		F.<x> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617)[]
		K.<a> = GF(21888242871839275222246405745257275088548364400416034343698204186575808495617**4, name='a', modulus=x^4-7)
		
		base = 16377448892084713529161739182205318095580119111576802375181616547062197291263*a^0 + 13687683608888423916085091250849188813359145430644908352977567823030408967189*a^1 + 12629166084120705167185476169390021031074363183264910102253898080559854363106*a^2 + 19441276922979928804860196077335093208498949640381586557241379549605420212272*a^3
		h = 1294177986177175279602421915789749270823809536595962994745244158374705688266*a^3 + 1540379511125324102377803754608881114249455137236500477169164628692514244862*a^2 + 20829599225781884356477513064431048695774529855095864514701692089787151865093*a + 8252578783913909531884765397785803733246236629821369091076513527284845891757
		
		baseOrder = base.multiplicative_order()
		hOrder = h.multiplicative_order()
		print(baseOrder)
		print(hOrder)
		print(is_prime(baseOrder))
		
		secret = 15403795111253241023778037546088811142494551372365004771691646286925142448620047716
		base_to_secret = base^secret
		h_to_secret = h^secret
		print(base_to_secret)
		print(h_to_secret)
	 */
}
