use crate::Result;
use crate::errors::ERR_OUT_OF_BOUNDS;

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Clone + Copy + Default,
{
    values: Vec<T>,
    width: usize,
}

impl<T> Matrix<T>
where
    T: Clone + Copy + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![T::default(); height * width],
            width,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<T> {
        let index = self.index(row, col);
        let value = self.values.get(index).ok_or(ERR_OUT_OF_BOUNDS)?;
        Ok(*value)
    }

    pub fn set(&mut self, row: usize, col: usize, new: T) -> Result<()> {
        let index = self.index(row, col);
        let value = self.values.get_mut(index).ok_or(ERR_OUT_OF_BOUNDS)?;
        *value = new;
        Ok(())
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy + Default + Ord,
{
    pub fn max(&self) -> (usize, usize, T) {
        let max = self
            .values
            .iter()
            .enumerate()
            .max_by_key(|(_, val)| **val)
            .unwrap();
        let col = max.0 % self.width;
        let row = max.0 / self.width;
        (row, col, *max.1)
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: Clone + Copy + Default + std::fmt::Debug,
{
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
