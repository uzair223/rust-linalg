use super::Vector;

impl Vector {
    pub fn norm(&self) -> f64 {
        self.iter().fold(0., |acc, x| acc + x * x).sqrt()
    }
}
