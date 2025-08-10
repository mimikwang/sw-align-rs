use crate::matrix::Matrix;

pub struct Aligner {
    matrix: Matrix,
    seq0: Vec<u8>,
    seq1: Vec<u8>,
}

impl Aligner {
    pub fn new(seq0: Vec<u8>, seq1: Vec<u8>) -> Self {
        Self {
            matrix: Matrix::new(seq1.len() + 1, seq0.len() + 1),
            seq0,
            seq1,
        }
    }

    pub fn build(&mut self) -> Result<(), &'static str> {
        for i in 1..=self.seq0.len() {
            let b0 = self.seq0.get(i - 1).ok_or("does not exist")?;
            for j in 1..=self.seq1.len() {
                let b1 = self.seq1.get(j - 1).ok_or("does not exist")?;

                let n0 = self.matrix.get(i - 1, j - 1)? + Self::substitution(b0, b1);

                let n1 = (0..i)
                    .map(|ind| {
                        let val = self.matrix.get(ind, j)?;
                        Ok(val - Self::gap_penalty(i))
                    })
                    .collect::<Result<Vec<i32>, &'static str>>()?
                    .into_iter()
                    .max()
                    .ok_or("bad")?;

                let n2 = (0..j)
                    .map(|ind| {
                        let val = self.matrix.get(i, ind)?;
                        Ok(val - Self::gap_penalty(i))
                    })
                    .collect::<Result<Vec<i32>, &'static str>>()?
                    .into_iter()
                    .max()
                    .ok_or("bad")?;

                let picked = [n0, n1, n2, 0].into_iter().max().ok_or("bad")?;
                self.matrix.set(i, j, picked)?;
            }
        }

        Ok(())
    }

    fn substitution(b0: &u8, b1: &u8) -> i32 {
        if b0 == b1 {
            return 3;
        }

        -3
    }

    fn gap_penalty(k: usize) -> i32 {
        2
    }
}

impl std::fmt::Display for Aligner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}
