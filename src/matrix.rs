#[derive(Debug)]
pub struct Matrix {
    values: Vec<i32>,
    width: usize,
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            values: vec![0; height * width],
            width,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<i32, &'static str> {
        let index = self.index(row, col);
        let value = self.values.get(index).ok_or("out of bounds")?;
        Ok(*value)
    }

    pub fn set(&mut self, row: usize, col: usize, new: i32) -> Result<(), &'static str> {
        let index = self.index(row, col);
        let value = self.values.get_mut(index).ok_or("out of bounds")?;
        *value = new;
        Ok(())
    }

    fn index(&self, row: usize, col: usize) -> usize {
        col * self.width + row
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.values.len() {
            write!(f, "{:?} ", self.values.get(i).unwrap_or(&0))?;
            if (i + 1) % self.width == 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
