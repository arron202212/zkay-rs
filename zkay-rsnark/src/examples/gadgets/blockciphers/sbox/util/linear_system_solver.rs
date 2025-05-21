use crate::circuit::config::config::Configs;

/**
 * Solves a linear system of equations over a finite field.
 *
 * Used for efficient representation of AES S-box gadget
 */

pub struct LinearSystemSolver {
    mat: Vec<Vec<BigInteger>>,
    numCols: i32,
    numRows: i32,
}
impl LinearSystemSolver {
    const prime: BigInteger = Configs.get().unwrap().field_prime;
    pub fn new(mat: Vec<Vec<BigInteger>>) -> Self {
        self.mat = mat;
        numRows = mat.len();
        numCols = mat[0].length;
    }

    pub fn solveInPlace() {
        // https://www.csun.edu/~panferov/math262/262_rref.pdf
        // https://www.math.purdue.edu/~shao92/documents/Algorithm%20REF.pdf
        guassJordan();
        rref();
    }

    fn guassJordan() {
        for colIdx in 0..numCols {
            let pivotRowIdx = rowIdx;
            while (pivotRowIdx < numRows && mat[pivotRowIdx][colIdx]==BigInteger::ZERO) {
                pivotRowIdx += 1;
            }
            if pivotRowIdx == numRows {
                continue;
            }

            // swap
            let tmp = mat[pivotRowIdx];
            mat[pivotRowIdx] = mat[rowIdx];
            mat[rowIdx] = tmp;

            pivotRowIdx = rowIdx;

            // dividing by pivot
            let invF = inverse(mat[pivotRowIdx][colIdx]);
            for j in 0..numCols {
                mat[pivotRowIdx][j] = mat[pivotRowIdx][j].mul(invF).rem(.clone());
            }

            for k in pivotRowIdx..numRows {
                let f = negate(mat[k][colIdx]);
                for j in 0..numCols {
                    mat[k][j] = mat[k][j].add(mat[pivotRowIdx][j].mul(f));
                    mat[k][j] = mat[k][j].rem(prime.clone());
                }
            }
        }
    }

    fn rref() {
        for rowIdx in (0..=numRows - 1).rev() {
            let pivotColIdx = 0;
            while (pivotColIdx < numCols && mat[rowIdx][pivotColIdx]==BigInteger::ZERO) {
                pivotColIdx += 1;
            }
            if pivotColIdx == numCols {
                continue;
            }

            for k in (0..=rowIdx - 1).rev() {
                let f = mat[k][pivotColIdx];
                for j in 0..numCols {
                    mat[k][j] = mat[k][j].clone().add(mat[rowIdx][j].mul(f).neg());
                    mat[k][j] = mat[k][j].clone().rem(prime.clone());
                }
            }
        }
    }

    fn negate(x: BigInteger) -> BigInteger {
        return (prime.sub(x.rem(prime.clone()))).rem(prime.clone());
    }

    fn inverse(x: BigInteger) -> BigInteger {
        return (x.rem(prime.clone())).mod_inv(prime.clone());
    }
}
