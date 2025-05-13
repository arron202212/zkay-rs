
use circuit::structure::wire;

pub struct Util {

	// seeded by 1 for testing purposes
	 Random rand = Random::new(1);
}
impl Util{
	pub   Vec<BigInteger> split(x:BigInteger , chunkSize:i32 ) {
		i32 numChunks = Math.max(1, (x.bitLength() + chunkSize - 1) / chunkSize); // ceil(x.bitLength() / chunkSize)
		return split(x, numChunks, chunkSize);
	}

	pub   Vec<BigInteger> split(x:BigInteger , numChunks:i32 , chunkSize:i32 ) {
		Vec<BigInteger> chunks = vec![BigInteger::default();numChunks];
		BigInteger mask = BigInteger.ONE.shiftLeft(chunkSize).subtract(BigInteger.ONE);
		for i in 0..numChunks {
			chunks[i] = x.shiftRight(chunkSize * i).and(mask);
		}
		return chunks;
	}

	pub   BigInteger combine(table:Vec<BigInteger>, blocks:Vec<Wire>, bitwidth:i32 ) {
		BigInteger sum = BigInteger.ZERO;
		for i in 0..blocks.length {
			if table[blocks[i].getWireId()] == null {
				continue;
			}
			sum = sum.add(table[blocks[i].getWireId()].shiftLeft(bitwidth * i));
		}
		return sum;
	}

	pub   BigInteger group(list:Vec<BigInteger>, width:i32 ) {
		BigInteger x = BigInteger.ZERO;
		for i in 0..list.length {
			x = x.add(list[i].shiftLeft(width * i));
		}
		return x;
	}

	pub   Vec<i32> concat(a1:Vec<i32>, a2:Vec<i32>) {
		Vec<i32> all = vec![i32::default();a1.length + a2.length];
		for i in 0..all.length {
			all[i] = i < a1.length  { a1[i] }else { a2[i - a1.length]};
		}
		return all;
	}

	pub   Vec<Wire> concat(a1:Vec<Wire>, a2:Vec<Wire>) {
		Vec<Wire> all = vec![Wire::default();a1.length + a2.length];
		for i in 0..all.length {
			all[i] = i < a1.length  { a1[i] }else { a2[i - a1.length]};
		}
		return all;
	}

	pub   Vec<Wire> concat(w:Wire , a:Vec<Wire>) {
		Vec<Wire> all = vec![Wire::default();1 + a.length];
		for i in 0..all.length {
			all[i] = i < 1  { w }else { a[i - 1]};
		}
		return all;
	}

	pub   Vec<i32> concat(Vec<Vec<i32>> arrays) {
		i32 sum = 0;
		for array in  arrays {
			sum += array.length;
		}
		Vec<i32> all = vec![i32::default();sum];
		i32 idx = 0;
		for array in  arrays {
			for a in array {
				all[idx+=1] = a;
			}
		}
		return all;
	}

	pub   Vec<BigInteger> randomBigIntegerArray(num:i32 , n:BigInteger ) {
		Vec<BigInteger> result = vec![BigInteger::default();num];
		for i in 0..num {
			result[i] = nextRandomBigInteger(n);
		}
		return result;
	}

	pub   BigInteger nextRandomBigInteger(n:BigInteger ) {
		BigInteger result = BigInteger::new(n.bitLength(), rand);
		while (result.compareTo(n) >= 0) {
			result = BigInteger::new(n.bitLength(), rand);
		}
		return result;
	}

	pub   Vec<BigInteger> randomBigIntegerArray(num:i32 , numBits:i32 ) {
		Vec<BigInteger> result = vec![BigInteger::default();num];
		for i in 0..num {
			result[i] = nextRandomBigInteger(numBits);
		}
		return result;
	}

	pub   BigInteger nextRandomBigInteger(numBits:i32 ) {
		return BigInteger::new(numBits, rand);
	}

	pub   String getDesc(desc:Vec<String>) {
		if desc.length == 0 {
			return "";
		} else {
			return desc[0];
		}
	}

	pub   List<Integer> parseSequenceLists(s:String ) {
		List<Integer> list = new ArrayList<>();
		Vec<String> chunks = s.split(",");
		for chunk in chunks {
			if chunk.equals("")
				continue;
			i32 lower = Integer.parseInt(chunk.split(":")[0]);
			i32 upper = Integer.parseInt(chunk.split(":")[1]);
			for i in lower..=upper{
				list.add(i);
			}
		}
		return list;
	}

	pub   Vec<Wire> reverseBytes(inBitWires:Vec<Wire>) {
		Vec<Wire> outs = Arrays.copyOf(inBitWires, inBitWires.length);
		i32 numBytes = inBitWires.length / 8;
		for i in 0..numBytes / 2{
			i32 other = numBytes - i - 1;
			for j in 0..8 {
				Wire temp = outs[i * 8 + j];
				outs[i * 8 + j] = outs[other * 8 + j];
				outs[other * 8 + j] = temp;
			}
		}
		return outs;
	}

	pub   String arrayToString(a:Vec<i32>, separator:String ) {
		StringBuilder s = StringBuilder::new();
		for i in 0..a.length - 1{
			s.append(a[i]).append(separator);
		}
		s.append(a[a.length - 1]);
		return s.toString();
	}

	pub   String arrayToString(a:Vec<Wire>, separator:String ) {
		StringBuilder s = StringBuilder::new();
		for i in 0..a.length - 1{
			s.append(a[i]).append(separator);
		}
		s.append(a[a.length - 1]);
		return s.toString();
	}

	pub   bool isBinary(v:BigInteger ) {
		return v.equals(BigInteger.ZERO) || v.equals(BigInteger.ONE);
	}

	pub   String padZeros(s:String , l:i32 ) {
		return format!("%" + l + "s",s).replace(' ', '0');
	}

	// Computation is cheap, keeping lots of BigIntegers in memory likely isn't, so use a weak hash map
	  Map<Integer, BigInteger> maxValueCache = Collections.synchronizedMap(new WeakHashMap<>());
	pub   BigInteger computeMaxValue(numBits:i32 ) {
		return maxValueCache.computeIfAbsent(numBits, i -> BigInteger.ONE.shiftLeft(i).subtract(BigInteger.ONE));
	}

	  Map<Integer, BigInteger> boundCache = Collections.synchronizedMap(new WeakHashMap<>());
	pub   BigInteger computeBound(numBits:i32 ) {
		return boundCache.computeIfAbsent(numBits, i -> BigInteger.ONE.shiftLeft(numBits));
	}

	pub   Vec<Wire> padWireArray(a:Vec<Wire>, length:i32 , p:Wire ) {
		if a.length == length {
			return a;
		} else if a.length > length {
			println!("No padding needed!");
			return a;
		} else {
			Vec<Wire> newArray = vec![Wire::default();length];
			System.arraycopy(a, 0, newArray, 0, a.length);
			for k in a..length{
				newArray[k] = p;
			}
			return newArray;
		}
	}

	pub   BigInteger mod(x:BigInteger , m:BigInteger ) {
		if x.signum() >= 0 && x.compareTo(m) < 0 {
			return x; // In range, 'mod' is no-op, but creates new BigInteger
		} else {
			return x.mod(m);
		}
	}
}
