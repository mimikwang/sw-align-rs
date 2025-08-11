use crate::matrix::Matrix;

const GAP_PENALTY: i32 = 2;
const MATCH: i32 = 3;
const MISMATCH: i32 = -3;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Trace {
    #[default]
    Stop,
    Top,
    Left,
    Diag,
}

pub struct Aligner {
    matrix: Matrix<i32>,
    traceback: Matrix<Trace>,
    seq0: Vec<u8>,
    seq1: Vec<u8>,

    print0: String,
    print1: String,
}

impl Aligner {
    pub fn new(seq0: Vec<u8>, seq1: Vec<u8>) -> Self {
        let width = seq0.len() + 1;
        let height = seq1.len() + 1;
        Self {
            matrix: Matrix::new(width, height),
            traceback: Matrix::new(width, height),
            seq0,
            seq1,
            print0: "".into(),
            print1: "".into(),
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

                let picked = [n_diag, n_top, n_left, 0]
                    .into_iter()
                    .enumerate()
                    .max_by_key(|(_, val)| *val)
                    .ok_or("bad")?;
                let trace = match picked.0 {
                    0 => Trace::Diag,
                    1 => Trace::Top,
                    2 => Trace::Left,
                    _ => Trace::Stop,
                };
                self.matrix.set(i, j, picked.1)?;
                self.traceback.set(i, j, trace)?;
            }
        }
        self.trace()?;
        Ok(())
    }

    fn trace(&mut self) -> Result<(), &'static str> {
        let (max_row, max_col, _) = self.matrix.max();
        let mut print0 = vec![];
        let mut print1 = vec![];
        let mut row = max_row;
        let mut col = max_col;
        loop {
            let trace = self.traceback.get(row, col)?;
            if trace == Trace::Stop {
                break;
            }
            let b0 = self.seq0.get(col - 1).ok_or("not found")?;
            let b1 = self.seq1.get(row - 1).ok_or("not found")?;
            

            match trace {
                Trace::Diag => {
                    print0.push(*b0);
                    print1.push(*b1);
                    row -= 1;
                    col -= 1;

                },
                Trace::Top => {
                    print0.push(b'-');
                    print1.push(*b1);
                    row -= 1;
                },
                Trace::Left => {
                    print0.push(*b0);
                    print1.push(b'-');
                    col -= 1;
                },
                _ => {},
            }
        }
        print0.reverse();
        print1.reverse();
        self.print0 = String::from_utf8(print0).map_err(|_| "")?;
        self.print1 = String::from_utf8(print1).map_err(|_| "")?;
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
        writeln!(f, "{}", self.print0)?;
        writeln!(f, "{}", self.print1)?;
        Ok(())
    }
}
