

use util::util;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::augmenter::pinocchio_gadget;
use examples::gadgets::hash::sha256_gadget;

/**
 * This circuit generator augments a second-price auction circuit (produced by Pinocchio's compiler) 
 * with SHA-256 gadgets on each input and output value. 
 *
 */

pub struct AugmentedAuctionCircuitGenerator extends CircuitGenerator {

	// each value is assumed to be a 64-bit value
	 Vec<Wire> secretInputValues;
	 Vec<Wire> secretOutputValues; 

	// randomness vectors for each participant (each random vector is 7 64-bit words)
	 Vec<Vec<Wire>> secretInputRandomness;
	 Vec<Vec<Wire>> secretOutputRandomness; 
	
	 String pathToCompiledCircuit;
	 i32 numParties; // includes the auction manager + the participants
	
	pub  AugmentedAuctionCircuitGenerator(String circuitName, String pathToCompiledCircuit, i32 numParticipants) {
		super(circuitName);
		self.pathToCompiledCircuit  = pathToCompiledCircuit;
		self.numParties = numParticipants + 1;
	}

	
	  fn buildCircuit() {

		secretInputValues = createProverWitnessWireArray(numParties - 1); // the manager has a zero input (no need to commit to it)
		secretInputRandomness = vec![Wire::default();numParties - 1][];
		secretOutputRandomness = vec![Wire::default();numParties][];
		for i in 0..numParties - 1{
			secretInputRandomness[i] =   createProverWitnessWireArray(7);
			secretOutputRandomness[i] =   createProverWitnessWireArray(7);
		}
		secretOutputRandomness[numParties-1] =   createProverWitnessWireArray(7);

		// instantiate a Pinocchio gadget for the auction circuit
		PinocchioGadget auctionGagdet = PinocchioGadget::new(Util::concat(zeroWire, secretInputValues), pathToCompiledCircuit);
		Vec<Wire> outputs = auctionGagdet.getOutputWires();
		
		// ignore the last output for this circuit which carries the index of the winner (not needed for this example)
		secretOutputValues = Arrays.copyOfRange(outputs, 0, outputs.length - 1);
		
		// augment the input side
		for i in 0..numParties - 1{
			SHA256Gadget g = SHA256Gadget::new(Util::concat(secretInputValues[i], secretInputRandomness[i]), 64, 64, false, false);
			makeOutputArray(g.getOutputWires(), "Commitment for party # " + i + "'s input balance.");
		}
		
		// augment the output side
		for i in 0..numParties{
			// adapt the output values to 64-bit values (adaptation is needed due to the way Pinocchio's compiler handles subtractions) 
			secretOutputValues[i] = secretOutputValues[i].getBitWires(64*2).packAsBits(64);
			SHA256Gadget g = SHA256Gadget::new(Util::concat(secretOutputValues[i], secretOutputRandomness[i]), 64, 64, false, false);
			makeOutputArray(g.getOutputWires(), "Commitment for party # " + i + "'s output balance.");
		}
	}

	
	pub   generateSampleInput(CircuitEvaluator evaluator) {
		
		for i in 0..numParties - 1{
			evaluator.setWireValue(secretInputValues[i], Util::nextRandomBigInteger(63));
		}		
		
		for i in 0..numParties - 1{
			for(Wire w:secretInputRandomness[i]){
				evaluator.setWireValue(w, Util::nextRandomBigInteger(64));
			}
		}
		for i in 0..numParties{
			for(Wire w:secretOutputRandomness[i]){
				evaluator.setWireValue(w, Util::nextRandomBigInteger(64));
			}
		}
	}
	
	
	pub    main(args:Vec<String>)  {
		AugmentedAuctionCircuitGenerator generator = AugmentedAuctionCircuitGenerator::new("augmented_auction_10", "auction_10.arith", 10);
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();	
	}

}
