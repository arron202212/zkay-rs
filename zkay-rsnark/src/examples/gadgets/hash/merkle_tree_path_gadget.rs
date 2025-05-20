use crate::circuit::config::config::Configs;
use crate::circuit::operations::gadget;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;

/**
 * A Merkle tree authentication gadget using the subsetsum hash function
 *
 */

pub struct MerkleTreePathGadget {
    treeHeight: i32,
    directionSelectorWire: WireType,
    directionSelectorBits: Vec<Option<WireType>>,
    leafWires: Vec<Option<WireType>>,
    intermediateHashWires: Vec<Option<WireType>>,
    outRoot: Vec<Option<WireType>>,

    leafWordBitWidth: i32,
}
impl MerkleTreePathGadget {
    const digestWidth: i32 = SubsetSumHashGadget.DIMENSION;
    pub fn new(
        directionSelectorWire: WireType,
        leafWires: Vec<Option<WireType>>,
        intermediateHasheWires: Vec<Option<WireType>>,
        leafWordBitWidth: i32,
        treeHeight: i32,
        desc: Vec<String>,
    ) -> Self {
        super(desc);
        self.directionSelectorWire = directionSelectorWire;
        self.treeHeight = treeHeight;
        self.leafWires = leafWires;
        self.intermediateHashWires = intermediateHasheWires;
        self.leafWordBitWidth = leafWordBitWidth;

        buildCircuit();
    }
}
impl Gadget for MerkleTreePathGadget {
    fn buildCircuit() {
        let directionSelectorBits = directionSelectorWire.getBitWires(treeHeight).asArray();

        // Apply CRH to leaf data
        let leafBits = WireArray::new(leafWires)
            .getBits(leafWordBitWidth)
            .asArray();
        let subsetSumGadget = SubsetSumHashGadget::new(leafBits, false);
        let currentHash = subsetSumGadget.getOutputWires();

        // Apply CRH across tree path guided by the direction bits
        for i in 0..treeHeight {
            let inHash = vec![WireType::default(); 2 * digestWidth];
            for j in 0..digestWidth {
                let temp = currentHash[j].sub(intermediateHashWires[i * digestWidth + j]);
                let temp2 = directionSelectorBits[i].mul(temp);
                inHash[j] = intermediateHashWires[i * digestWidth + j].add(temp2);
            }
            for j in digestWidth..2 * digestWidth {
                let temp = currentHash[j - digestWidth]
                    .add(intermediateHashWires[i * digestWidth + j - digestWidth]);
                inHash[j] = temp.sub(inHash[j - digestWidth]);
            }

            let nextInputBits = WireArray::new(inHash)
                .getBits(Config.log2_field_prime)
                .asArray();
            subsetSumGadget = SubsetSumHashGadget::new(nextInputBits, false);
            currentHash = subsetSumGadget.getOutputWires();
        }
        outRoot = currentHash;
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        return outRoot;
    }
}
