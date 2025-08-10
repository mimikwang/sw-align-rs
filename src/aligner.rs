use crate::matrix::Matrix;

const GAP_PENALTY: i32 = 2;
const MATCH: i32 = 3;
const MISMATCH: i32 = -3;

pub struct Aligner {
    matrix: Matrix,
    seq0: Vec<u8>,
    seq1: Vec<u8>,
}

impl Aligner {
    pub fn new(seq0: Vec<u8>, seq1: Vec<u8>) -> Self {
        Self {
            matrix: Matrix::new(seq0.len() + 1, seq1.len() + 1),
            seq0,
            seq1,
        }
    }

    pub fn build(&mut self) -> Result<(), &'static str> {
        for j in 1..=self.seq0.len() {
            let b0 = self.seq0.get(j - 1).ok_or("does not exist")?;

            for i in 1..=self.seq1.len() {
                let b1 = self.seq1.get(i - 1).ok_or("does not exist")?;

                let n_diag = self.matrix.get(i - 1, j - 1)? + Self::substitution(b0, b1);
                let n_top = self.matrix.get(i - 1, j)? - GAP_PENALTY;
                let n_left = self.matrix.get(i, j - 1)? - GAP_PENALTY;

                let picked = [n_diag, n_top, n_left, 0].into_iter().max().ok_or("bad")?;
                self.matrix.set(i, j, picked)?;
            }
        }

        Ok(())
    }

    fn substitution(b0: &u8, b1: &u8) -> i32 {
        if b0 == b1 {
            return MATCH;
        }
        MISMATCH
    }
}

impl std::fmt::Display for Aligner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}
