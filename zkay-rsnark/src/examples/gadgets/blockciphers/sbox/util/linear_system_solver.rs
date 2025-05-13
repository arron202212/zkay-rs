

use circuit::config::config;

/**
 * Solves a linear system of equations over a finite field.
 * 
 * Used for efficient representation of AES S-box gadget
 */

pub struct LinearSystemSolver {

	pub   BigInteger prime = Config.FIELD_PRIME;

	 Vec<Vec<BigInteger>> mat;
	 i32 numRows, numCols;

	pub  LinearSystemSolver(Vec<Vec<BigInteger>> mat) {
		self.mat = mat;
		numRows = mat.length;
		numCols = mat[0].length;
	}

	pub   solveInPlace() {

		// https://www.csun.edu/~panferov/math262/262_rref.pdf
		// https://www.math.purdue.edu/~shao92/documents/Algorithm%20REF.pdf
		guassJordan();
		rref();
	}

	  guassJordan() {
		for colIdx in 0..numCols{
			i32 pivotRowIdx = rowIdx;
			while (pivotRowIdx < numRows
					&& mat[pivotRowIdx][colIdx].equals(BigInteger.ZERO)) {
				pivotRowIdx+=1;
			}
			if pivotRowIdx == numRows
				continue;

			// swap
			Vec<BigInteger> tmp = mat[pivotRowIdx];
			mat[pivotRowIdx] = mat[rowIdx];
			mat[rowIdx] = tmp;

			pivotRowIdx = rowIdx;

			// dividing by pivot
			BigInteger invF = inverse(mat[pivotRowIdx][colIdx]);
			for j in 0..numCols {
				mat[pivotRowIdx][j] = mat[pivotRowIdx][j].multiply(invF).mod(
						prime);
			}

			for k in pivotRowIdx..numRows{
				BigInteger f = negate(mat[k][colIdx]);
				for j in 0..numCols {
					mat[k][j] = mat[k][j].add(mat[pivotRowIdx][j].multiply(f));
					mat[k][j] = mat[k][j].mod(prime);
				}
			}

		}
	}

	  rref() {
		for rowIdx in (0..=numRows - 1).rev()
			i32 pivotColIdx = 0;
			while (pivotColIdx < numCols
					&& mat[rowIdx][pivotColIdx].equals(BigInteger.ZERO)) {
				pivotColIdx+=1;
			}
			if pivotColIdx == numCols
				continue;

			for k in (0..=rowIdx - 1).rev()
				BigInteger f = mat[k][pivotColIdx];
				for j in 0..numCols {
					mat[k][j] = mat[k][j]
							.add(negate(mat[rowIdx][j].multiply(f)));
					mat[k][j] = mat[k][j].mod(prime);
				}
			}
		}
	}

	  BigInteger negate(BigInteger x) {
		return (prime.subtract(x.mod(prime))).mod(prime);
	}

	  BigInteger inverse(BigInteger x) {
		return (x.mod(prime)).modInverse(prime);
	}

}
