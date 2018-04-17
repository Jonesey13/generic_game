use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub struct Polynomial {
    coefficients: Vec<f64> // lowest to highest
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self {
            coefficients
        }
    }

    pub fn get_coeffs(&self) -> &Vec<f64> {
        &self.coefficients
    }

    pub fn new_x_minus(val: f64) -> Self {
        Self {
            coefficients: vec![-val, 1.0]
        }
    }

    pub fn identity() -> Self {
        Self {
            coefficients: vec![1.0]
        }
    }

    pub fn zero() -> Self {
        Self {
            coefficients: vec![0.0]
        }
    }

    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn get_point(&self, time: f64) -> f64 {
        let mut output = 0.0;

        for (index, coeff) in self.coefficients.iter().cloned().enumerate() {
            output = output + coeff * time.powi(index as i32);
        }

        output
    }
}

impl Add<Polynomial> for Polynomial {
        type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        let max_degree = self.get_degree().max(rhs.get_degree());

        let mut output_coeffs = vec![];

        for i in 0..max_degree + 1 {
            let coeff = self.coefficients.get(i).cloned().unwrap_or_default() 
                        + rhs.coefficients.get(i).cloned().unwrap_or_default();
            output_coeffs.push(coeff);
        }

        Self {
            coefficients: output_coeffs
        }
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let max_degree = self.get_degree() + rhs.get_degree();

        let mut output_coeffs = vec![];

        for i in 0..max_degree + 1 {
            let mut coeff = 0.0;
            for j in 0..i + 1 {
                coeff += self.coefficients.get(j).cloned().unwrap_or_default() 
                        * rhs.coefficients.get(i-j).cloned().unwrap_or_default();
            }
            output_coeffs.push(coeff);
        }

        Self {
            coefficients: output_coeffs
        }
    }
}

impl Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let new_coeffs = rhs.coefficients.iter().map(|c| {self * c}).collect();

        Polynomial {
            coefficients: new_coeffs
        }
    }
}