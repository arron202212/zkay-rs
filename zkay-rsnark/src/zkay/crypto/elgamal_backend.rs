use crate::circuit::operations::gadget::GadgetConfig;
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use zkay::*;

pub struct ElgamalBackend;

impl ElgamalBackend {
    const EC_COORD_BITS: i32 = 254; // a BabyJubJub affine coordinate fits into 254 bits

    const KEY_CHUNK_SIZE: i32 = 256; // needs to be a multiple of 8

    const RND_CHUNK_SIZE: i32 = 256;

    fn ElgamalBackend(keyBits: i32) {
        //super(keyBits);

        // pub  key must be a BabyJubJub point (two coordinates)
        assert!(keyBits == 2 * EC_COORD_BITS, "pub  key size mismatch");
    }
}
impl Asymmetric for ElgamalBackend {
    fn getKeyChunkSize() -> i32 {
        KEY_CHUNK_SIZE
    }

    fn usesDecryptionGadget() -> bool {
        // randomness is not extractable from an ElGamal ciphertext, so need a separate
        // gadget for decryption
        true
    }

    fn addKey(keyName: String, keyWires: Vec<Option<WireType>>) {
        // elgamal does not require a bit-representation of the pub  key, so store it directly
        keys.put(keyName, WireArray::new(keyWires));
    }

    fn createEncryptionGadget(
        plain: TypedWire,
        keyName: String,
        random: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        let pkArray = getKeyArray(keyName);
        let pk = ZkayBabyJubJubGadget::new().JubJubPoint(pkArray.get(0), pkArray.get(1));
        let randomArray = WireArray::new(random).getBits(RND_CHUNK_SIZE).asArray();
        assert!(
            plain.zkay_type.bitwidth <= 32,
            "plaintext must be at most 32 bits for elgamal backend"
        );
        return ZkayElgamalEncGadget::new(
            plain.wire.getBitWires(plain.zkay_type.bitwidth).asArray(),
            pk,
            randomArray,
        );
    }

    fn createDecryptionGadget(
        plain: TypedWire,
        cipher: Vec<Option<WireType>>,
        pkName: String,
        sk: Vec<Option<WireType>>,
        desc: &Option<String>,
    ) -> Gadget {
        let pkArray = getKeyArray(pkName);
        let pk = ZkayBabyJubJubGadget::new().JubJubPoint(pkArray.get(0), pkArray.get(1));
        let c1 = ZkayBabyJubJubGadget::new().JubJubPoint(cipher[0], cipher[1]);
        let c2 = ZkayBabyJubJubGadget::new().JubJubPoint(cipher[2], cipher[3]);
        let skBits = WireArray::new(sk).getBits(RND_CHUNK_SIZE).asArray();
        ZkayElgamalDecGadget::new(pk, skBits, c1, c2, plain.wire)
    }

    fn toTypedWireArray(wires: Vec<Option<WireType>>, name: String) -> Vec<TypedWire> {
        let typedWires = vec![TypedWire::default(); wires.len()];
        let uint256 = ZkayType.ZkUint(256);
        for i in 0..wires.len() {
            typedWires[i] = TypedWire::new(wires[i], uint256, name);
        }
        typedWires
    }

    fn fromTypedWireArray(typedWires: Vec<TypedWire>) -> Vec<Option<WireType>> {
        let wires = vec![None; typedWires.len()];
        let uint256 = ZkayType.ZkUint(256);
        for i in 0..typedWires.len() {
            ZkayType.checkType(uint256, typedWires[i].zkay_type);
            wires[i] = typedWires[i].wire;
        }
        wires
    }

    fn parseJubJubPoint(wire: Vec<Option<WireType>>, offset: i32) -> JubJubPoint {
        ZkayBabyJubJubGadget::new().JubJubPoint(wire[offset], wire[offset + 1])
    }

    fn uninitZeroToIdentity(p: JubJubPoint) -> JubJubPoint {
        // Uninitialized values have a ciphertext of all zeroes, which is not a valid ElGamal cipher.
        // Instead, replace those values with the point at infinity (0, 1).
        let oneIfBothZero = p.x.checkNonZero().or(p.y.checkNonZero()).invAsBit();
        ZkayBabyJubJubGadget::new().JubJubPoint(p.x, p.y.add(oneIfBothZero))
    }
}
impl HomomorphicBackend for ElgamalBackend {
    fn doHomomorphicOp(
        lhs: HomomorphicInput,
        op: char,
        rhs: HomomorphicInput,
        keyName: String,
    ) -> Vec<TypedWire> {
        if (op == '+') || (op == '-') {
            // for (c1, c2) = Enc(m1, r1)
            //     (d1, d2) = Enc(m2, r2)
            //     e1 = c1 + d1
            //     e2 = c2 + d2
            // it is (e1, e2) = Enc(m1 + m2, r1 + r2)
            let outputName = "(" + lhs.getName() + ") + (" + rhs.getName() + ")";

            let lhs_twires = lhs.getCipher();
            let rhs_twires = rhs.getCipher();

            // sanity checks
            assert!(lhs_twires.len() == 4); // 4 BabyJubJub coordinates
            assert!(rhs_twires.len() == 4); // 4 BabyJubJub coordinates
            let lhs_wires = fromTypedWireArray(lhs_twires);
            let rhs_wires = fromTypedWireArray(rhs_twires);

            let c1 = parseJubJubPoint(lhs_wires, 0);
            let c2 = parseJubJubPoint(lhs_wires, 2);
            let d1 = parseJubJubPoint(rhs_wires, 0);
            let d2 = parseJubJubPoint(rhs_wires, 2);

            c1 = uninitZeroToIdentity(c1);
            c2 = uninitZeroToIdentity(c2);
            d1 = uninitZeroToIdentity(d1);
            d2 = uninitZeroToIdentity(d2);

            if op == '-' {
                d1.x = d1.x.neg();
                d2.x = d2.x.neg();
            }

            let gadget = ZkayElgamalAddGadget::new(c1, c2, d1, d2);
            toTypedWireArray(gadget.getOutputWires(), outputName)
        } else if op == '*' {
            let outputName = "(" + lhs.getName() + ") * (" + rhs.getName() + ")";

            let mut plain_wire;
            let mut cipher_twires;
            if lhs.isPlain() && rhs.isCipher() {
                plain_wire = lhs.getPlain();
                cipher_twires = rhs.getCipher();
            } else if lhs.isCipher() && rhs.isPlain() {
                cipher_twires = lhs.getCipher();
                plain_wire = rhs.getPlain();
            } else {
                panic!("Elgamal multiplication requires exactly 1 plaintext argument");
            }

            let cipher_wires = fromTypedWireArray(cipher_twires);
            let c1 = parseJubJubPoint(cipher_wires, 0);
            let c2 = parseJubJubPoint(cipher_wires, 2);

            c1 = uninitZeroToIdentity(c1);
            c2 = uninitZeroToIdentity(c2);

            let gadget =
                ZkayElgamalMulGadget::new(c1, c2, plain_wire.wire.getBitWires(32).asArray());
            toTypedWireArray(gadget.getOutputWires(), outputName)
        } else {
            panic!("Binary operation {op} not supported");
        }
    }

    fn doHomomorphicRerand(
        arg: Vec<TypedWire>,
        keyName: String,
        randomness: TypedWire,
    ) -> Vec<TypedWire> {
        let outputName = "rerand(" + arg[0].name + ")";

        // parse argument
        let arg_wires = fromTypedWireArray(arg);
        let c1 = parseJubJubPoint(arg_wires, 0);
        let c2 = parseJubJubPoint(arg_wires, 2);
        c1 = uninitZeroToIdentity(c1);
        c2 = uninitZeroToIdentity(c2);

        // parse key and randomness
        let pkArray = getKeyArray(keyName);
        let pk = ZkayBabyJubJubGadget::new().JubJubPoint(pkArray.get(0), pkArray.get(1));
        let randomArray = randomness.wire.getBitWires(RND_CHUNK_SIZE).asArray();

        // create gadget
        let gadget = ZkayElgamalRerandGadget::new(c1, c2, pk, randomArray);
        toTypedWireArray(gadget.getOutputWires(), outputName)
    }
}
