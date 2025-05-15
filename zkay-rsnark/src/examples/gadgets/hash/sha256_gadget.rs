use circuit::operations::gadget;
use circuit::structure::wire;
use circuit::structure::wire_array;
use util::util;

pub struct SHA256Gadget {
    unpaddedInputs: Vec<Wire>,

    bitwidthPerInputElement: i32,
    totalLengthInBytes: i32,

    numBlocks: i32,
    binaryOutput: bool,
    paddingRequired: bool,

    preparedInputBits: Vec<Wire>,
    output: Vec<Wire>,
}
impl SHA256Gadget {
    const H: Vec<i64> = vec![
        0x6a09e667L,
        0xbb67ae85L,
        0x3c6ef372L,
        0xa54ff53aL,
        0x510e527fL,
        0x9b05688cL,
        0x1f83d9abL,
        0x5be0cd19L,
    ];

    const K: Vec<i64> = vec![
        0x428a2f98L,
        0x71374491L,
        0xb5c0fbcfL,
        0xe9b5dba5L,
        0x3956c25bL,
        0x59f111f1L,
        0x923f82a4L,
        0xab1c5ed5L,
        0xd807aa98L,
        0x12835b01L,
        0x243185beL,
        0x550c7dc3L,
        0x72be5d74L,
        0x80deb1feL,
        0x9bdc06a7L,
        0xc19bf174L,
        0xe49b69c1L,
        0xefbe4786L,
        0x0fc19dc6L,
        0x240ca1ccL,
        0x2de92c6fL,
        0x4a7484aaL,
        0x5cb0a9dcL,
        0x76f988daL,
        0x983e5152L,
        0xa831c66dL,
        0xb00327c8L,
        0xbf597fc7L,
        0xc6e00bf3L,
        0xd5a79147L,
        0x06ca6351L,
        0x14292967L,
        0x27b70a85L,
        0x2e1b2138L,
        0x4d2c6dfcL,
        0x53380d13L,
        0x650a7354L,
        0x766a0abbL,
        0x81c2c92eL,
        0x92722c85L,
        0xa2bfe8a1L,
        0xa81a664bL,
        0xc24b8b70L,
        0xc76c51a3L,
        0xd192e819L,
        0xd6990624L,
        0xf40e3585L,
        0x106aa070L,
        0x19a4c116L,
        0x1e376c08L,
        0x2748774cL,
        0x34b0bcb5L,
        0x391c0cb3L,
        0x4ed8aa4aL,
        0x5b9cca4fL,
        0x682e6ff3L,
        0x748f82eeL,
        0x78a5636fL,
        0x84c87814L,
        0x8cc70208L,
        0x90befffaL,
        0xa4506cebL,
        0xbef9a3f7L,
        0xc67178f2L,
    ];
    pub fn new(
        ins: Vec<Wire>,
        bitWidthPerInputElement: i32,
        totalLengthInBytes: i32,
        binaryOutput: bool,
        paddingRequired: bool,
        desc: Vec<String>,
    ) -> Self {
        super(desc);
        assert!(
            totalLengthInBytes * 8 <= ins.len() * bitWidthPerInputElement
                && totalLengthInBytes * 8 >= (ins.len() - 1) * bitWidthPerInputElement,
            "Inconsistent Length Information"
        );

        assert!(
            paddingRequired
                || totalLengthInBytes % 64 == 0
                || ins.len() * bitWidthPerInputElement == totalLengthInBytes,
            "When padding is not forced, totalLengthInBytes % 64 must be zero."
        );

        self.unpaddedInputs = ins;
        self.bitwidthPerInputElement = bitWidthPerInputElement;
        self.totalLengthInBytes = totalLengthInBytes;
        self.binaryOutput = binaryOutput;
        self.paddingRequired = paddingRequired;

        buildCircuit();
    }
}
impl Gadget for SHA256Gadget {
    fn buildCircuit() {
        // pad if needed
        prepare();

        let outDigest = vec![Wire::default(); 8];
        let hWires = vec![Wire::default(); H.len()];
        for i in 0..H.len() {
            hWires[i] = generator.createConstantWire(H[i]);
        }

        for blockNum in 0..numBlocks {
            let mut wsSplitted = vec![vec![]; 64];
            let mut w = vec![Wire::default(); 64];

            for i in 0..64 {
                if i < 16 {
                    wsSplitted[i] = Util::reverseBytes(Arrays.copyOfRange(
                        preparedInputBits,
                        blockNum * 512 + i * 32,
                        blockNum * 512 + (i + 1) * 32,
                    ));

                    w[i] = WireArray::new(wsSplitted[i]).packAsBits(32);
                } else {
                    let t1 = w[i - 15].rotateRight(32, 7);
                    let t2 = w[i - 15].rotateRight(32, 18);
                    let t3 = w[i - 15].shiftRight(32, 3);
                    let s0 = t1.xorBitwise(t2, 32);
                    s0 = s0.xorBitwise(t3, 32);

                    let t4 = w[i - 2].rotateRight(32, 17);
                    let t5 = w[i - 2].rotateRight(32, 19);
                    let t6 = w[i - 2].shiftRight(32, 10);
                    let s1 = t4.xorBitwise(t5, 32);
                    s1 = s1.xorBitwise(t6, 32);

                    w[i] = w[i - 16].add(w[i - 7]);
                    w[i] = w[i].add(s0).add(s1);
                    w[i] = w[i].trimBits(34, 32);
                }
            }

            let a = hWires[0];
            let b = hWires[1];
            let c = hWires[2];
            let d = hWires[3];
            let e = hWires[4];
            let f = hWires[5];
            let g = hWires[6];
            let h = hWires[7];

            for i in 0..64 {
                let t1 = e.rotateRight(32, 6);
                let t2 = e.rotateRight(32, 11);
                let t3 = e.rotateRight(32, 25);
                let s1 = t1.xorBitwise(t2, 32);
                s1 = s1.xorBitwise(t3, 32);

                let ch = computeCh(e, f, g, 32);

                let t4 = a.rotateRight(32, 2);
                let t5 = a.rotateRight(32, 13);
                let t6 = a.rotateRight(32, 22);
                let s0 = t4.xorBitwise(t5, 32);
                s0 = s0.xorBitwise(t6, 32);

                let mut maj;
                // since after each iteration, SHA256 does c = b; and b = a;, we can make use of that to save multiplications in maj computation.
                // To do this, we make use of the caching feature, by just changing the order of wires sent to maj(). Caching will take care of the rest.
                if i % 2 == 1 {
                    maj = computeMaj(c, b, a, 32);
                } else {
                    maj = computeMaj(a, b, c, 32);
                }

                let temp1 = w[i].add(K[i]).add(s1).add(h).add(ch);

                let temp2 = maj.add(s0);

                h = g;
                g = f;
                f = e;
                e = temp1.add(d);
                e = e.trimBits(35, 32);

                d = c;
                c = b;
                b = a;
                a = temp2.add(temp1);
                a = a.trimBits(35, 32);
            }

            hWires[0] = hWires[0].add(a).trimBits(33, 32);
            hWires[1] = hWires[1].add(b).trimBits(33, 32);
            hWires[2] = hWires[2].add(c).trimBits(33, 32);
            hWires[3] = hWires[3].add(d).trimBits(33, 32);
            hWires[4] = hWires[4].add(e).trimBits(33, 32);
            hWires[5] = hWires[5].add(f).trimBits(33, 32);
            hWires[6] = hWires[6].add(g).trimBits(33, 32);
            hWires[7] = hWires[7].add(h).trimBits(33, 32);
        }

        outDigest[0] = hWires[0];
        outDigest[1] = hWires[1];
        outDigest[2] = hWires[2];
        outDigest[3] = hWires[3];
        outDigest[4] = hWires[4];
        outDigest[5] = hWires[5];
        outDigest[6] = hWires[6];
        outDigest[7] = hWires[7];

        if !binaryOutput {
            output = outDigest;
        } else {
            output = vec![Wire::default(); 8 * 32];
            for i in 0..8 {
                let bits = outDigest[i].getBitWires(32).asArray();
                for j in 0..32 {
                    output[j + i * 32] = bits[j];
                }
            }
        }
    }

    fn computeMaj(a: Wire, b: Wire, c: Wire, numBits: i32) -> Wire {
        let result = vec![Wire::default(); numBits];
        let aBits = a.getBitWires(numBits).asArray();
        let bBits = b.getBitWires(numBits).asArray();
        let cBits = c.getBitWires(numBits).asArray();

        for i in 0..numBits {
            let t1 = aBits[i].mul(bBits[i]);
            let t2 = aBits[i].add(bBits[i]).add(t1.mul(-2));
            result[i] = t1.add(cBits[i].mul(t2));
        }
        return WireArray::new(result).packAsBits();
    }

    fn computeCh(a: Wire, b: Wire, c: Wire, numBits: i32) -> Wire {
        let result = vec![Wire::default(); numBits];

        let aBits = a.getBitWires(numBits).asArray();
        let bBits = b.getBitWires(numBits).asArray();
        let cBits = c.getBitWires(numBits).asArray();

        for i in 0..numBits {
            let t1 = bBits[i].sub(cBits[i]);
            let t2 = t1.mul(aBits[i]);
            result[i] = t2.add(cBits[i]);
        }
        return WireArray::new(result).packAsBits();
    }

    fn prepare() {
        numBlocks = (totalLengthInBytes * 1.0 / 64).ceil() as i32;
        let bits = WireArray::new(unpaddedInputs)
            .getBits(bitwidthPerInputElement)
            .asArray();
        let tailLength = totalLengthInBytes % 64;
        if paddingRequired {
            let mut pad;
            if (64 - tailLength >= 9) {
                pad = vec![Wire::default(); 64 - tailLength];
            } else {
                pad = vec![Wire::default(); 128 - tailLength];
            }
            numBlocks = (totalLengthInBytes + pad.len()) / 64;
            pad[0] = generator.createConstantWire(0x80);
            for i in 1..pad.len() - 8 {
                pad[i] = generator.getZeroWire();
            }
            let lengthInBits = totalLengthInBytes * 8;
            let lengthBits = vec![Wire::default(); 64];
            for i in 0..8 {
                pad[pad.len() - 1 - i] =
                    generator.createConstantWire((lengthInBits >> (8 * i)) & 0xFFL);
                let tmp = pad[pad.len() - 1 - i].getBitWires(8).asArray();
                lengthBits[(7 - i) * 8..(7 - i + 1) * 8].clone_from_slice(&tmp);
            }
            let totalNumberOfBits = numBlocks * 512;
            preparedInputBits = vec![generator.getZeroWire(); totalNumberOfBits];
            preparedInputBits[..totalLengthInBytes * 8].clone_from_slice(&bits);
            preparedInputBits[totalLengthInBytes * 8 + 7] = generator.getOneWire();
            preparedInputBits[preparedInputBits.len() - 64..].clone_from_slice(&lengthBits);
        } else {
            preparedInputBits = bits;
        }
    }

    /**
     * outputs digest as 32-bit words
     */

    pub fn getOutputWires() -> Vec<Wire> {
        return output;
    }
}
