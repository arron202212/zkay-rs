use crate::circuit::operations::gadget;
use crate::circuit::structure::circuit_generator::{CircuitGenerator,CircuitGeneratorIQ,CircuitGeneratorExtend,getActiveCircuitGenerator};
use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use examples::gadgets::blockciphers::sbox::aes_s_box_compute_gadget;
use examples::gadgets::blockciphers::sbox::aes_s_box_gadget_optimized1;
use examples::gadgets::blockciphers::sbox::aes_s_box_gadget_optimized2;
use examples::gadgets::blockciphers::sbox::aes_s_box_naive_lookup_gadget;

pub enum SBoxOption {
    LINEAR_SCAN,
    COMPUTE,
    OPTIMIZED1,
    OPTIMIZED2,
}

/**
 * Implements an AES 128-bit block cipher. The gadget applies an improved
 * read-only memory lookup from xjsnark (to appear) to reduce the cost of the
 * S-box access. (See the sbox package for the improved lookup implementation)
 *
 */
pub struct AES128CipherGadget {
    //
    plaintext: Vec<Option<WireType>>,   // array of 16 bytes
    expandedKey: Vec<Option<WireType>>, // array of 176 bytes (call expandKey(..))
    ciphertext: Vec<Option<WireType>>,  // array of 16 bytes
}
impl AES128CipherGadget {
    const nb: i32 = 4;
    const nk: i32 = 4;
    const nr: i32 = 6 + nk;
    pub const sBoxOption: SBoxOption = SBoxOption.OPTIMIZED2;
    pub const RCon: Vec<i32> = vec![
        0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
    ];

    pub const SBox: Vec<i32> = vec![
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
    /**
     *
     * @param inputs
     *            : array of 16 bytes (each wire represents a byte)
     * @param expandedKey
     *            : array of 176 bytes (each wire represents a byte) -- call
     *            expandKey() to get it
     */

    pub fn new(inputs: Vec<Option<WireType>>, expandedKey: Vec<Option<WireType>>, desc: &Option<String>) -> Self {
        super(desc);
        assert!(
            inputs.len() == 4 * nb && expandedKey.len() == 4 * nb * (nr + 1),
            "Invalid Input"
        );

        self.plaintext = inputs;
        self.expandedKey = expandedKey;
        buildCircuit();
    }
}
impl Gadget for AES128CipherGadget {
    // follows the outline in http://www.cs.utsa.edu/~wagner/laws/AESintro.html
    fn buildCircuit() {
        ciphertext = vec![None; 4 * nb];
        let state = vec![None; 4][nb];
        let i = 0;
        for j in 0..nb {
            for k in 0..4 {
                state[k][j] = plaintext[i += 1];
            }
        }

        state = addRoundKey(state, 0, nb - 1);
        let round = 0;
        for round in 1..nr {
            subBytes(state);
            state = shiftRows(state);
            state = mixColumns(state);
            state = addRoundKey(state, round * nb * 4, (round + 1) * nb * 4 - 1);
        }

        subBytes(state);
        state = shiftRows(state);
        state = addRoundKey(state, round * nb * 4, (round + 1) * nb * 4 - 1);

        let mut i = 0;
        for j in 0..nb {
            for k in 0..4 {
                ciphertext[i] = state[k][j];
                i += 1;
            }
        }
    }

    fn subBytes(state: Vec<Vec<Option<WireType>>>) {
        for i in 0..state.len() {
            for j in 0..state[i].len() {
                state[i][j] = randomAccess(generator, state[i][j]);
            }
        }
    }

    fn mixColumns(state: Vec<Vec<Option<WireType>>>) -> Vec<Vec<Option<WireType>>> {
        let mut a = vec![None; 4];
        for c in 0..4 {
            for i in 0..4 {
                a[i] = state[i][c];
            }
            state[0][c] = galoisMulConst(a[0], 2)
                .xorWireArray(galoisMulConst(a[1], 3))
                .xorWireArray(a[2].getBitWires(8))
                .xorWireArray(a[3].getBitWires(8))
                .packAsBits(None,None,);

            state[1][c] = a[0]
                .getBitWires(8)
                .xorWireArray(galoisMulConst(a[1], 2))
                .xorWireArray(galoisMulConst(a[2], 3))
                .xorWireArray(a[3].getBitWires(8))
                .packAsBits(None,None,);

            state[2][c] = a[0]
                .getBitWires(8)
                .xorWireArray(a[1].getBitWires(8))
                .xorWireArray(galoisMulConst(a[2], 2))
                .xorWireArray(galoisMulConst(a[3], 3))
                .packAsBits(None,None,);
            state[3][c] = galoisMulConst(a[0], 3)
                .xorWireArray(a[1].getBitWires(8))
                .xorWireArray(a[2].getBitWires(8))
                .xorWireArray(galoisMulConst(a[3], 2))
                .packAsBits(None,None,);
        }
        state
    }

    fn galoisMulConst(wire: WireType, mut i: i32) -> WireArray {
        let p = generator.get_zero_wire();
        let counter;
        let hiBitSet;

        for counter in 0..8 {
            if (i & 1) != 0 {
                p = p.xorBitwise(wire, 8);
            }
            i >>= 1;
            if i == 0 {}
            hiBitSet = wire.getBitWires(8).get(7);
            wire = wire.shl(8, 1);
            let tmp = wire.xorBitwise(generator.createConstantWire(0x1bL), 8);
            wire = wire.add(hiBitSet.mul(tmp.sub(wire)));
        }
        p.getBitWires(8)
    }

    fn shiftRows(state: Vec<Vec<Option<WireType>>>) -> Vec<Vec<Option<WireType>>> {
        let mut newState = vec![vec![None; nb]; 4];
        newState[0] = state[0][..nb].to_vec();
        for j in 0..nb {
            newState[1][j] = state[1][(j + 1) % nb];
            newState[2][j] = state[2][(j + 2) % nb];
            newState[3][j] = state[3][(j + 3) % nb];
        }
        newState
    }

    fn addRoundKey(state: Vec<Vec<Option<WireType>>>, from: i32, to: i32) -> Vec<Vec<Option<WireType>>> {
        let newState = vec![vec![None; nb]; 4];
        let idx = 0;
        for j in 0..nb {
            for i in 0..4 {
                newState[i][j] = state[i][j].xorBitwise(expandedKey[from + idx], 8);
                idx += 1;
            }
        }
        newState
    }

    pub fn getOutputWires() -> Vec<Option<WireType>> {
        ciphertext
    }

    // key is a 16-byte array. Each wire represents a byte.
    pub fn expandKey(key: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let w = vec![vec![None; 4]; nb * (nr + 1)];
        let mut temp;
        let mut i = 0;
        while (i < nk) {
            w[i] = vec![key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
            i += 1;
        }

        let mut generator = CircuitGenerator.getActiveCircuitGenerator();
        i = nk;
        while (i < nb * (nr + 1)) {
            temp = w[i - 1];
            if i % nk == 0 {
                temp = subWord(generator, rotateWord(generator, temp));
                temp[0] = temp[0].xorBitwise(generator.createConstantWire(RCon[i / nk]), 8);
            } else if nk > 6 && (i % nk) == 4 {
                temp = subWord(generator, temp);
            }

            for v in 0..4 {
                w[i][v] = w[i - nk][v].xorBitwise(temp[v], 8);
            }

            i += 1;
        }
        let expanded = vec![None; nb * (nr + 1) * 4];
        let idx = 0;
        for k in 0..nb * (nr + 1) {
            for i in 0..4 {
                expanded[idx += 1] = w[k][i];
            }
        }

        expanded
    }

    fn subWord(generator:Box<dyn CGConfig+Send+Sync>, w: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        for i in 0..w.len() {
            w[i] = randomAccess(generator, w[i]);
        }
        w
    }

    fn rotateWord(generator:Box<dyn CGConfig+Send+Sync>, w: Vec<Option<WireType>>) -> Vec<Option<WireType>> {
        let newW = vec![None; w.len()];
        for j in 0..w.len() {
            newW[j] = w[(j + 1) % w.len()];
        }
        newW
    }

    fn randomAccess(generator:Box<dyn CGConfig+Send+Sync>, wire: WireType) -> WireType {
        let g = None;
        match sBoxOption {
            LINEAR_SCAN => g = AESSBoxNaiveLookupGadget::new(wire),

            COMPUTE => g = AESSBoxComputeGadget::new(wire),

            OPTIMIZED1 => g = AESSBoxGadgetOptimized1::new(wire),

            OPTIMIZED2 => g = AESSBoxGadgetOptimized2::new(wire),
            _ => {}
        }

        g.getOutputWires()[0]
    }
}
