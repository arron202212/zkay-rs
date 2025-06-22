use crate::circuit::config::config::Configs;
use crate::circuit::eval::circuit_evaluator::CircuitEvaluator;
use crate::circuit::structure::circuit_generator::{CircuitGenerator,CircuitGeneratorIQ,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use examples::gadgets::hash::merkle_tree_path_gadget;
use examples::gadgets::hash::subset_sum_hash_gadget;
use crate::util::util::{Util,BigInteger};

pub struct MerkleTreeMembershipCircuitGenerator {
    publicRootWires: Vec<Option<WireType>>,
    intermediateHasheWires: Vec<Option<WireType>>,
    directionSelector: WireType,
    leafWires: Vec<Option<WireType>>,

    treeHeight: i32,

    merkleTreeGadget: MerkleTreePathGadget,
}
impl MerkleTreeMembershipCircuitGenerator {
    const leafNumOfWords: i32 = 10;
    const leafWordBitWidth: i32 = 32;
    const hashDigestDimension: i32 = SubsetSumHashGadget.DIMENSION;
    pub fn new(circuitName: String, treeHeight: i32) -> Self {
        super(circuitName);
        self.treeHeight = treeHeight;
    }
}
impl CircuitGenerator for MerkleTreeMembershipCircuitGenerator {
    fn buildCircuit() {
        //  declare inputs 
        publicRootWires = createInputWireArray(hashDigestDimension, "Input Merkle Tree Root");
        intermediateHasheWires =
            createProverWitnessWireArray(hashDigestDimension * treeHeight, "Intermediate Hashes");
        directionSelector = createProverWitnessWire("Direction selector");
        leafWires = createProverWitnessWireArray(leafNumOfWords, "Secret Leaf");

        // connect gadget
        merkleTreeGadget = MerkleTreePathGadget::new(
            directionSelector,
            leafWires,
            intermediateHasheWires,
            leafWordBitWidth,
            treeHeight,
        );
        let actualRoot = merkleTreeGadget.getOutputWires();

        /** Now compare the actual root with the pub  known root **/
        let errorAccumulator = get_zero_wire();
        for i in 0..hashDigestDimension {
            let diff = actualRoot[i].sub(publicRootWires[i]);
            let check = diff.checkNonZero();
            errorAccumulator = errorAccumulator.add(check);
        }

        makeOutputArray(actualRoot, "Computed Root");

        /** Expected mismatch here if the sample input below is tried**/
        makeOutput(errorAccumulator.checkNonZero(), "Error if NON-zero");
    }

    pub fn generateSampleInput(circuitEvaluator: CircuitEvaluator) {
        for i in 0..hashDigestDimension {
            circuitEvaluator.setWireValue(
                publicRootWires[i],
                Util::nextRandomBigInteger(Configs.field_prime),
            );
        }

        circuitEvaluator.setWireValue(directionSelector, Util::nextRandomBigInteger(treeHeight));
        for i in 0..hashDigestDimension * treeHeight {
            circuitEvaluator.setWireValue(
                intermediateHasheWires[i],
                Util::nextRandomBigInteger(Configs.field_prime),
            );
        }

        for i in 0..leafNumOfWords {
            circuitEvaluator.setWireValue(leafWires[i], Integer.MAX_VALUE);
        }
    }


}
    pub fn main(args: Vec<String>) {
        let mut generator = MerkleTreeMembershipCircuitGenerator::new("tree_64", 64);
        generator.generateCircuit();
        generator.evalCircuit();
        generator.prepFiles();
        generator.runLibsnark();
    }