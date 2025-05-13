
use circuit::config::config;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;


/**
 * A Merkle tree authentication gadget using the subsetsum hash function
 * 
 */

pub struct MerkleTreePathGadget extends Gadget {

	  i32 digestWidth = SubsetSumHashGadget.DIMENSION;

	 i32 treeHeight;
	 Wire directionSelectorWire;
	 Vec<Wire> directionSelectorBits;
	 Vec<Wire> leafWires;
	 Vec<Wire> intermediateHashWires;
	 Vec<Wire> outRoot;

	 i32 leafWordBitWidth;

	pub  MerkleTreePathGadget(Wire directionSelectorWire, leafWires:Vec<Wire>, intermediateHasheWires:Vec<Wire>,
			i32 leafWordBitWidth, i32 treeHeight, desc:Vec<String>) {

		super(desc);
		self.directionSelectorWire = directionSelectorWire;
		self.treeHeight = treeHeight;
		self.leafWires = leafWires;
		self.intermediateHashWires = intermediateHasheWires;
		self.leafWordBitWidth = leafWordBitWidth;

		buildCircuit();

	}

	  fn buildCircuit() {

		directionSelectorBits = directionSelectorWire.getBitWires(treeHeight).asArray();

		// Apply CRH to leaf data
		Vec<Wire> leafBits = WireArray::new(leafWires).getBits(leafWordBitWidth).asArray();
		SubsetSumHashGadget subsetSumGadget = SubsetSumHashGadget::new(leafBits, false);
		Vec<Wire> currentHash = subsetSumGadget.getOutputWires();

		// Apply CRH across tree path guided by the direction bits
		for i in 0..treeHeight {
			Vec<Wire> inHash = vec![Wire::default();2 * digestWidth];
			for j in 0..digestWidth {
				Wire temp = currentHash[j].sub(intermediateHashWires[i * digestWidth + j]);
				Wire temp2 = directionSelectorBits[i].mul(temp);
				inHash[j] = intermediateHashWires[i * digestWidth + j].add(temp2);
			}
			for j in digestWidth..2 * digestWidth{
				Wire temp = currentHash[j - digestWidth].add(intermediateHashWires[i * digestWidth + j - digestWidth]);
				inHash[j] = temp.sub(inHash[j - digestWidth]);
			}

			Vec<Wire> nextInputBits = WireArray::new(inHash).getBits(Config.LOG2_FIELD_PRIME).asArray();
			subsetSumGadget = SubsetSumHashGadget::new(nextInputBits, false);
			currentHash = subsetSumGadget.getOutputWires();
		}
		outRoot = currentHash;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return outRoot;
	}

}
