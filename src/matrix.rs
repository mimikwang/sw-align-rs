#[derive(Debug)]
pub struct Matrix {
    values: Vec<Option<f64>>,
    width: usize,
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            values: vec![None; height * width],
            width,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Result<Option<f64>, &'static str> {
        let index = self.index(row, col);
        let value = self.values.get(index).ok_or("out of bounds")?;
        Ok(*value)
    }

    pub fn set(&mut self, row: usize, col: usize, new: Option<f64>) -> Result<(), &'static str> {
        let index = self.index(row, col);
        let value = self.values.get_mut(index).ok_or("out of bounds")?;
        *value = new;
        Ok(())
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn print(&self) -> Result<(), &'static str> {
        for i in 0..self.values.len() {
            print!("{:?} ", self.values.get(i).unwrap_or(&Some(0.0)).unwrap_or(0.0));
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
        Ok(())
    }
}
