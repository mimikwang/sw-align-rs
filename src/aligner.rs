use crate::Result;
use crate::errors::{ERR_CATCH_ALL, ERR_NOT_FOUND};
use crate::matrix::Matrix;

const GAP_PENALTY: i32 = 2;
const MATCH: i32 = 3;
const MISMATCH: i32 = -3;
const GAP: u8 = b'-';

/// Back trace direction
#[derive(Clone, Default, PartialEq)]
enum Trace {
    #[default]
    Stop,
    Top,
    Left,
    Diag,
}

/// Aligner aligns two DNA sequences using the Smith-Waterman algorithm
pub struct Aligner {
    /// The scoring matrix
    matrix: Matrix<i32>,
    /// The traceback matrix
    traceback: Matrix<Trace>,

    /// The original sequence 0
    seq0: Vec<u8>,
    /// The original sequence 1
    seq1: Vec<u8>,

    /// The aligned sequence 0
    aligned0: String,
    /// The aligned sequence 1
    aligned1: String,
}

impl Aligner {
    /// Construct a new aligner
    pub fn new(seq0: Vec<u8>, seq1: Vec<u8>) -> Self {
        let width = seq0.len() + 1;
        let height = seq1.len() + 1;
        Self {
            matrix: Matrix::new(width, height),
            traceback: Matrix::new(width, height),
            seq0,
            seq1,
            aligned0: "".into(),
            aligned1: "".into(),
        }
    }

    /// Build the scoring matrix and run the traceback
    pub fn build(&mut self) -> Result<()> {
        for j in 1..=self.seq0.len() {
            let b0 = Self::get_base(&self.seq0, j - 1)?;

            for i in 1..=self.seq1.len() {
                let b1 = Self::get_base(&self.seq1, i - 1)?;

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

    /// Run the traceback to fill in aligned0 and aligned1
    fn trace(&mut self) -> Result<()> {
        let max_ind = self.matrix.max_index();
        let mut aligned0 = vec![];
        let mut aligned1 = vec![];
        let mut ind = max_ind;
        loop {
            let trace = self.traceback.get(ind.0, ind.1)?;
            if trace == &Trace::Stop {
                break;
            }
            let b0 = Self::get_base(&self.seq0, ind.1 - 1)?;
            let b1 = Self::get_base(&self.seq1, ind.0 - 1)?;

            Self::process_trace(
                &trace,
                (b0 , b1),
                &mut aligned0,
                &mut aligned1,
                &mut ind,
            )?;
        }
        aligned0.reverse();
        aligned1.reverse();
        self.aligned0 = String::from_utf8(aligned0).map_err(|_| ERR_CATCH_ALL)?;
        self.aligned1 = String::from_utf8(aligned1).map_err(|_| ERR_CATCH_ALL)?;
        Ok(())
    }

    fn substitution(b0: &u8, b1: &u8) -> i32 {
        if b0 == b1 {
            return MATCH;
        }
        MISMATCH
    }

    fn get_base(seq: &[u8], ind: usize) -> Result<&u8> {
        seq.get(ind).ok_or(ERR_NOT_FOUND)
    }

    fn process_trace(
        trace: &Trace,
        bases: (&u8, &u8),
        aligned0: &mut Vec<u8>,
        aligned1: &mut Vec<u8>,
        ind: &mut (usize, usize),
    ) -> Result<()> {
        match trace {
            Trace::Diag => {
                aligned0.push(*bases.0);
                aligned1.push(*bases.1);
                ind.0 -= 1;
                ind.1 -= 1;
            }
            Trace::Top => {
                aligned0.push(GAP);
                aligned1.push(*bases.1);
                ind.0 -= 1;
            }
            Trace::Left => {
                aligned0.push(*bases.0);
                aligned1.push(GAP);
                ind.1 -= 1;
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Display for Aligner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self.aligned0)?;
        writeln!(f, "{}", self.aligned1)?;
        Ok(())
    }
}
