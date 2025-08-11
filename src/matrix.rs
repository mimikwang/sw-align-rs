//! Matrix struct to represent the scoring matrix and the traceback matrix

use crate::Result;
use crate::errors::ERR_OUT_OF_BOUNDS;

/// Data structure to represent a 2D matrix. This is used for the scoring matrix and the traceback
/// matrix.
#[derive(Debug)]
pub struct Matrix<T> {
    /// Values are stored in a vector
    values: Vec<T>,
    /// Width of the matrix - i.e. number of columns
    width: usize,
}

impl<T> Matrix<T>
where
    T: Clone + Default,
{
    /// Construct a matrix of size width x height, initialized with the default value.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![T::default(); height * width],
            width,
        }
    }

    /// Get the value at (row, col) for the matrix. Returns an error if it is out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Result<&T> {
        let index = self.index(row, col);
        let value = self.values.get(index).ok_or(ERR_OUT_OF_BOUNDS)?;
        Ok(value)
    }

    /// Set the value at (row, col) to new. Returns an error if it is out of bounds.
    pub fn set(&mut self, row: usize, col: usize, new: T) -> Result<()> {
        let index = self.index(row, col);
        let value = self.values.get_mut(index).ok_or(ERR_OUT_OF_BOUNDS)?;
        *value = new;
        Ok(())
    }

    /// Helper function to grab the index
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy + Ord,
{
    /// Grab the (row, col) index of the max value in the matrix
    pub fn max_index(&self) -> (usize, usize) {
        let max = self
            .values
            .iter()
            .enumerate()
            .max_by_key(|(_, val)| **val)
            .unwrap();
        let col = max.0 % self.width;
        let row = max.0 / self.width;
        (row, col)
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: Default + std::fmt::Debug,
{
    /// Format a matrix by printing the 2D matrix
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.values.len() {
            write!(f, "{:?} ", self.values.get(i).unwrap_or(&T::default()))?;
            if (i + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut mat = Matrix::<usize>::new(10, 10);
        assert_eq!(Ok(&0), mat.get(1, 1));
        assert_eq!(Ok(()), mat.set(9, 9, 5));
        assert_eq!(Ok(&5), mat.get(9, 9));
        assert_eq!(100, mat.values.len());
        assert_eq!(Err(ERR_OUT_OF_BOUNDS), mat.get(100, 100));
    }

    #[test]
    fn test_max_index() {
        let mut mat = Matrix::<usize>::new(4, 4);
        assert_eq!(Ok(()), mat.set(0, 0, 100));
        assert_eq!(Ok(()), mat.set(3, 2, 500));
        assert_eq!((3, 2), mat.max_index());
    }
}
