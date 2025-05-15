use circuit::structure::wire;

pub struct Util {
    // seeded by 1 for testing purposes
    rand: Random,
}
impl Default for Util {
    fn default() -> Self {
        Self {
            rand: Random::new(1),
        }
    }
}
impl Util {
    pub fn split(x: BigInteger, chunkSize: i32) -> Vec<BigInteger> {
        let numChunks = zkay_type(1, (x.bitLength() + chunkSize - 1) / chunkSize); // ceil(x.bitLength() / chunkSize)
        return split(x, numChunks, chunkSize);
    }

    pub fn split(x: BigInteger, numChunks: i32, chunkSize: i32) -> Vec<BigInteger> {
        let chunks = vec![BigInteger::default(); numChunks];
        let mask = BigInteger.ONE.shiftLeft(chunkSize).subtract(BigInteger.ONE);
        for i in 0..numChunks {
            chunks[i] = x.shiftRight(chunkSize * i).and(mask);
        }
        return chunks;
    }

    pub fn combine(table: Vec<BigInteger>, blocks: Vec<Wire>, bitwidth: i32) -> BigInteger {
        let sum = BigInteger.ZERO;
        for i in 0..blocks.length {
            if table[blocks[i].getWireId()] == None {
                continue;
            }
            sum = sum.add(table[blocks[i].getWireId()].shiftLeft(bitwidth * i));
        }
        return sum;
    }

    pub fn group(list: Vec<BigInteger>, width: i32) -> BigInteger {
        let x = BigInteger.ZERO;
        for i in 0..list.length {
            x = x.add(list[i].shiftLeft(width * i));
        }
        return x;
    }

    pub fn concat(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
        let all = vec![i32::default(); a1.length + a2.length];
        for i in 0..all.length {
            all[i] = if i < a1.length {
                a1[i]
            } else {
                a2[i - a1.length]
            };
        }
        return all;
    }

    pub fn concat(a1: Vec<Wire>, a2: Vec<Wire>) -> Vec<Wire> {
        let all = vec![Wire::default(); a1.length + a2.length];
        for i in 0..all.length {
            all[i] = if i < a1.length {
                a1[i]
            } else {
                a2[i - a1.length]
            };
        }
        return all;
    }

    pub fn concat(w: Wire, a: Vec<Wire>) -> Vec<Wire> {
        let all = vec![Wire::default(); 1 + a.length];
        for i in 0..all.length {
            all[i] = if i < 1 { w } else { a[i - 1] };
        }
        return all;
    }

    pub fn concat(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let sum = 0;
        for array in arrays {
            sum += array.length;
        }
        let all = vec![i32::default(); sum];
        let idx = 0;
        for array in arrays {
            for a in array {
                all[idx] = a;
                idx += 1;
            }
        }
        return all;
    }

    pub fn randomBigIntegerArray(num: i32, n: BigInteger) -> Vec<BigInteger> {
        let result = vec![BigInteger::default(); num];
        for i in 0..num {
            result[i] = nextRandomBigInteger(n);
        }
        return result;
    }

    pub fn nextRandomBigInteger(n: BigInteger) -> BigInteger {
        let result = BigInteger::new(n.bitLength(), rand);
        while (result.compareTo(n) >= 0) {
            result = BigInteger::new(n.bitLength(), rand);
        }
        return result;
    }

    pub fn randomBigIntegerArray(num: i32, numBits: i32) -> Vec<BigInteger> {
        let result = vec![BigInteger::default(); num];
        for i in 0..num {
            result[i] = nextRandomBigInteger(numBits);
        }
        return result;
    }

    pub fn nextRandomBigInteger(numBits: i32) -> BigInteger {
        return BigInteger::new(numBits, rand);
    }

    pub fn getDesc(desc: Vec<String>) -> String {
        if desc.length == 0 {
            return "";
        } else {
            return desc[0];
        }
    }

    pub fn parseSequenceLists(s: String) -> List<Integer> {
        let list = Vec::new();
        let chunks = s.split(",");
        for chunk in chunks {
            if chunk.equals("") {
                continue;
            }
            let c: Vec<_> = chunk.split(":").collect();
            let lower = c[0].parse::<i32>().unwrap();
            let upper = c[1].parse::<i32>().unwrap();
            for i in lower..=upper {
                list.add(i);
            }
        }
        return list;
    }

    pub fn reverseBytes(inBitWires: Vec<Wire>) -> Vec<Wire> {
        let outs = inBitWires.clone();
        let numBytes = inBitWires.length / 8;
        for i in 0..numBytes / 2 {
            let other = numBytes - i - 1;
            for j in 0..8 {
                let temp = outs[i * 8 + j];
                outs[i * 8 + j] = outs[other * 8 + j];
                outs[other * 8 + j] = temp;
            }
        }
        return outs;
    }

    pub fn arrayToString(a: Vec<i32>, separator: String) -> String {
        let s = String::new();
        for i in 0..a.length - 1 {
            s.push_str(a[i]).push_str(separator);
        }
        s.push_str(a[a.length - 1]);
        return s;
    }

    pub fn arrayToString(a: Vec<Wire>, separator: String) -> String {
        let s = String::new();
        for i in 0..a.length - 1 {
            s.push_str(a[i]).push_str(separator);
        }
        s.push_str(a[a.length - 1]);
        return s;
    }

    pub fn isBinary(v: BigInteger) -> bool {
        return v.equals(BigInteger.ZERO) || v.equals(BigInteger.ONE);
    }

    pub fn padZeros(s: String, l: i32) -> String {
        return format!("%" + l + "s", s).replace(' ', '0');
    }

    // Computation is cheap, keeping lots of BigIntegers in memory likely isn't, so use a weak hash map

    pub fn computeMaxValue(numBits: i32) -> BigInteger {
        let maxValueCache = HashMap::new();
        return maxValueCache.computeIfAbsent(
            numBits,
            i - BigInteger.ONE.shiftLeft(i).subtract(BigInteger.ONE),
        );
    }

    pub fn computeBound(numBits: i32) -> BigInteger {
        let boundCache = HashMap::new();
        return boundCache.computeIfAbsent(numBits, i - BigInteger.ONE.shiftLeft(numBits));
    }

    pub fn padWireArray(a: Vec<Wire>, length: i32, p: Wire) -> Vec<Wire> {
        if a.length == length {
            return a;
        } else if a.length > length {
            println!("No padding needed!");
            return a;
        } else {
            let newArray = vec![Wire::default(); length];
            newArray[..a.length].clone_from_slice(&a);
            for k in a..length {
                newArray[k] = p;
            }
            return newArray;
        }
    }

    pub fn modulo(x: BigInteger, m: BigInteger) -> BigInteger {
        if x.signum() >= 0 && x.compareTo(m) < 0 {
            return x; // In range, 'mod' is no-op, but creates new BigInteger
        } else {
            return x.modulo(m);
        }
    }
}
