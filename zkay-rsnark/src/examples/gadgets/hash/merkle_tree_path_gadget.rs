
use circuit::config::config;
use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;


/**
 * A Merkle tree authentication gadget using the subsetsum hash function
 * 
 */

pub struct MerkleTreePathGadget  {

	  

	 treeHeight:i32,
	 directionSelectorWire:Wire,
	 directionSelectorBits:Vec<Wire>,
	 leafWires:Vec<Wire>,
	 intermediateHashWires:Vec<Wire>,
	 outRoot:Vec<Wire>,

	 leafWordBitWidth:i32,
}
impl  MerkleTreePathGadget{
let digestWidth = SubsetSumHashGadget.DIMENSION;
	pub  fn new(directionSelectorWire:Wire, leafWires:Vec<Wire>, intermediateHasheWires:Vec<Wire>,
			i32 leafWordBitWidth, i32 treeHeight, desc:Vec<String>) {

		super(desc);
		self.directionSelectorWire = directionSelectorWire;
		self.treeHeight = treeHeight;
		self.leafWires = leafWires;
		self.intermediateHashWires = intermediateHasheWires;
		self.leafWordBitWidth = leafWordBitWidth;

		buildCircuit();

	}
}
impl Gadget for MerkleTreePathGadget{
	  fn buildCircuit() {

		directionSelectorBits = directionSelectorWire.getBitWires(treeHeight).asArray();

		// Apply CRH to leaf data
		let leafBits = WireArray::new(leafWires).getBits(leafWordBitWidth).asArray();
		let subsetSumGadget = SubsetSumHashGadget::new(leafBits, false);
		let currentHash = subsetSumGadget.getOutputWires();

		// Apply CRH across tree path guided by the direction bits
		for i in 0..treeHeight {
			let inHash = vec![Wire::default();2 * digestWidth];
			for j in 0..digestWidth {
				let temp = currentHash[j].sub(intermediateHashWires[i * digestWidth + j]);
				let temp2 = directionSelectorBits[i].mul(temp);
				inHash[j] = intermediateHashWires[i * digestWidth + j].add(temp2);
			}
			for j in digestWidth..2 * digestWidth{
				let temp = currentHash[j - digestWidth].add(intermediateHashWires[i * digestWidth + j - digestWidth]);
				inHash[j] = temp.sub(inHash[j - digestWidth]);
			}

			let nextInputBits = WireArray::new(inHash).getBits(Config.LOG2_FIELD_PRIME).asArray();
			subsetSumGadget = SubsetSumHashGadget::new(nextInputBits, false);
			currentHash = subsetSumGadget.getOutputWires();
		}
		outRoot = currentHash;
	}

	
	 pub  fn getOutputWires()->Vec<Wire>  {
		return outRoot;
	}

}
