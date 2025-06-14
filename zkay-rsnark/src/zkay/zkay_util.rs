use crate::circuit::structure::wire_type::WireType;
use crate::circuit::structure::wire_array;
use crate::util::util::{Util,BigInteger};

pub struct ZkayUtil;
impl ZkayUtil {
    pub const ZKAY_RESTRICT_EVERYTHING: bool = false; // if set to true for debugging, each typed wire constructor restricts bitwidth (rather than just  inputs)

    pub fn reverseBytes(bitArray: WireArray, targetWordBits: i32) -> Vec<Option<WireType>> {
        return WireArray::new(Util::reverseBytes(bitArray.asArray()))
            .packBitsIntoWords(targetWordBits);
    }

    pub fn unsignedBytesToBigInt(bytes: Vec<byte>) -> BigInteger {
        let signum = 0;
        for b in bytes {
            if b != 0 {
                signum = 1;
                break;
            }
        }
        BigInteger::new(signum, bytes)
    }

    pub fn unsignedBigintToBytes(val: BigInteger) -> Vec<byte> {
        let b = val.toByteArray();
        let mut ret;
        if b[0] == 0 && b.len() > 1 {
            ret = vec![byte::default(); b.len() - 1];
            ret[..b.len() - 1].clone_from_slice(&b[1..]);
        } else {
            ret = b;
        }
        ret
    }

    pub fn unsignedBigintToBytes(val: BigInteger, byteCount: i32) -> Vec<byte> {
        let t = unsignedBigintToBytes(val);
        assert!(
            t.len() <= byteCount,
            "Value too large to fit into {byteCount} bytes"
        );
        let ret = vec![byte::default(); byteCount];
        ret[byteCount - t.len()..byteCount].clone_from_slice(&t);
        ret
    }

    pub fn runZkayJsnarkInterface() {
        let p = runcomand(vec![
            "../libsnark/build/libsnark/zkay_interface/run_snark",
            "keygen",
            ".",
            ".",
            "1",
        ]);
        //println!(
            "\n-----------------------------------RUNNING LIBSNARK KEYGEN -----------------------------------------"
        );
        let input = BufReader::new(p.getInputStream());
        let buf = StringBuilder::new();
        for line in input.lines() {
            buf.append(line).append("\n");
        }

        //println!(buf.toString());

        let p = runcomand(vec![
            "../libsnark/build/libsnark/zkay_interface/run_snark",
            "proofgen",
            ".",
            "proof.out",
            ".",
            "1",
            "1",
        ]);
        //println!(
            "\n-----------------------------------RUNNING LIBSNARK PROOFGEN -----------------------------------------"
        );
        let input = BufferedReader::new(InputStreamReader::new(p.getInputStream()));
        let buf = StringBuilder::new();
        for line in input.lines() {
            buf.append(line).append("\n");
        }

        //println!(buf.toString());
    }
}
