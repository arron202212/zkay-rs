
use util::util;
use circuit::config::config;
use circuit::eval::circuit_evaluator;
use circuit::structure::circuit_generator;
use circuit::structure::wire;
use examples::gadgets::hash::merkle_tree_path_gadget;
use examples::gadgets::hash::subset_sum_hash_gadget;

pub struct MerkleTreeMembershipCircuitGenerator extends CircuitGenerator {

	 publicRootWires:Vec<Wire>,
	 intermediateHasheWires:Vec<Wire>,
	 directionSelector:Wire,
	 leafWires:Vec<Wire>,

	 treeHeight:i32,
	

	 merkleTreeGadget:MerkleTreePathGadget,
	}
impl  MerkleTreeMembershipCircuitGenerator{
	 i32 leafNumOfWords = 10;
	 i32 leafWordBitWidth = 32;
 i32 hashDigestDimension = SubsetSumHashGadget.DIMENSION;
	pub  fn new(circuitName:String, i32 treeHeight)  ->Self{
		super(circuitName);
		self.treeHeight = treeHeight;
	}

	}
impl Gadget for MerkleTreeMembershipCircuitGenerator{
	  fn buildCircuit() {
		
		/** declare inputs **/
		
		publicRootWires = createInputWireArray(hashDigestDimension, "Input Merkle Tree Root");
		intermediateHasheWires = createProverWitnessWireArray(hashDigestDimension * treeHeight, "Intermediate Hashes");
		directionSelector = createProverWitnessWire("Direction selector");
		leafWires = createProverWitnessWireArray(leafNumOfWords, "Secret Leaf");

		/** connect gadget **/

		merkleTreeGadget = MerkleTreePathGadget::new(
				directionSelector, leafWires, intermediateHasheWires, leafWordBitWidth, treeHeight);
		let actualRoot = merkleTreeGadget.getOutputWires();
		
		/** Now compare the actual root with the pub  known root **/
		let errorAccumulator = getZeroWire();
		for i in 0..hashDigestDimension{
			let diff = actualRoot[i].sub(publicRootWires[i]);
			let check = diff.checkNonZero();
			errorAccumulator = errorAccumulator.add(check);
		}
		
		makeOutputArray(actualRoot, "Computed Root");
		
		/** Expected mismatch here if the sample input below is tried**/
		makeOutput(errorAccumulator.checkNonZero(), "Error if NON-zero");
		
	}

	
	pub   generateSampleInput(circuitEvaluator:CircuitEvaluator) {
		
		for i in 0..hashDigestDimension {
			circuitEvaluator.setWireValue(publicRootWires[i], Util::nextRandomBigInteger(Config.FIELD_PRIME));
		}
		
		circuitEvaluator.setWireValue(directionSelector, Util::nextRandomBigInteger(treeHeight));
		for i in 0..hashDigestDimension*treeHeight {
			circuitEvaluator.setWireValue(intermediateHasheWires[i],  Util::nextRandomBigInteger(Config.FIELD_PRIME));
		}
		
		for i in 0..leafNumOfWords{
			circuitEvaluator.setWireValue(leafWires[i], Integer.MAX_VALUE);
		}
		
	}
	
	
	pub    main(args:Vec<String>)  {
		
		let generator = MerkleTreeMembershipCircuitGenerator::new("tree_64", 64);
		generator.generateCircuit();
		generator.evalCircuit();
		generator.prepFiles();
		generator.runLibsnark();		
	}

	
}
