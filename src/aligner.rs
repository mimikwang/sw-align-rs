use crate::matrix::Matrix;

const GAP_PENALTY: f64 = 2.0;

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

    fn init(&mut self) -> Result<(), &'static str> {
        for i in 0..=self.seq0.len() {
            self.matrix.set(i, 0, Some(0.0))?;
        }

        for j in 0..=self.seq1.len() {
            self.matrix.set(0, j, Some(0.0))?;
        }

        Ok(())
    }

    pub fn build(&mut self) -> Result<(), &'static str> {
        self.init()?;

        for i in 1..=self.seq0.len() {
            let b0 = self.seq0.get(i - 1).ok_or("does not exist")?;
            for j in 1..=self.seq1.len() {
                let b1 = self.seq1.get(j - 1).ok_or("does not exist")?;

                let n0 = self.matrix.get(i - 1, j - 1)?.ok_or("does not exist")?
                    + Self::substitution(b0, b1);

                let n1 = (0..i)
                    .map(|ind| {
                        let val = self.matrix.get(ind, j)?;
                        Ok(val.unwrap_or(0.0) - GAP_PENALTY)
                    })
                    .collect::<Result<Vec<f64>, &'static str>>()?
                    .into_iter()
                    .fold(f64::NEG_INFINITY, f64::max);

                let n2 = (0..j)
                    .map(|ind| {
                        let val = self.matrix.get(i, ind)?;
                        Ok(val.unwrap_or(0.0) - GAP_PENALTY)
                    })
                    .collect::<Result<Vec<f64>, &'static str>>()?
                    .into_iter()
                    .fold(f64::NEG_INFINITY, f64::max);

                let picked = [n0, n1, n2, 0.0].into_iter().fold(f64::NEG_INFINITY, f64::max);
                self.matrix.set(i, j, Some(picked))?;
            }
        }

        Ok(())
    }

    fn substitution(b0: &u8, b1: &u8) -> f64 {
        if b0 == b1 {
            return 3.0;
        }

        -3.0
    }

    pub fn print(&self) -> Result<(), &'static str> {
        self.matrix.print()
    }
}
