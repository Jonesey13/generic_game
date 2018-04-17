use std::ops::{Add, Mul};
use na::Vector2;
use num::Zero;
use geometry::Polynomial;

#[derive(Clone, Debug)]
pub struct Polynomial2d {
    coefficients: Vec<Vector2<f64>> // lowest to highest
}

impl Polynomial2d {
    pub fn new(coefficients: Vec<Vector2<f64>>) -> Self {
        Self {
            coefficients
        }
    }

    pub fn new_from_1d_polys(poly1: Polynomial, poly2: Polynomial) -> Self {
        let coefficients = poly1.get_coeffs().iter()
                            .zip(poly2.get_coeffs().iter())
            .map(|(coeff1, coeff2)| {Vector2::new(*coeff1, *coeff2)}).collect();

        Self {
            coefficients
        }
    }

    pub fn get_coeffs(&self) -> &Vec<Vector2<f64>> {
        &self.coefficients
    }

    pub fn new_x_minus(val: f64) -> Self {
        Self {
            coefficients: vec![-Vector2::new(val, val), Vector2::new(1.0, 1.0)]
        }
    }

    pub fn identity() -> Self {
        Self {
            coefficients: vec![Vector2::new(1.0, 1.0)]
        }
    }

    pub fn zero() -> Self {
        Self {
            coefficients: vec![Vector2::zero()]
        }
    }

    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn get_point(&self, time: f64) -> Vector2<f64> {
        let mut output = Vector2::zero();

        for (index, coeff) in self.coefficients.iter().cloned().enumerate() {
            output = output + coeff * time.powi(index as i32);
        }

        output
    }
}

impl Add<Polynomial2d> for Polynomial2d {
        type Output = Polynomial2d;

    fn add(self, rhs: Polynomial2d) -> Self::Output {
        let max_degree = self.get_degree().max(rhs.get_degree());

        let mut output_coeffs = vec![];

        for i in 0..max_degree + 1 {
            let coeff = self.coefficients.get(i).cloned().unwrap_or(Vector2::zero()) 
                        + rhs.coefficients.get(i).cloned().unwrap_or(Vector2::zero());
            output_coeffs.push(coeff);
        }

        Self {
            coefficients: output_coeffs
        }
    }
}

impl Mul<Polynomial2d> for Polynomial2d {
    type Output = Polynomial2d;

    fn mul(self, rhs: Polynomial2d) -> Self::Output {
        let max_degree = self.get_degree() + rhs.get_degree();

        let mut output_coeffs = vec![];

        for i in 0..max_degree + 1 {
            let mut coeff = Vector2::zero();
            for j in 0..i + 1 {
                let first_coeff = self.coefficients.get(j).cloned().unwrap_or(Vector2::zero());
                let second_coeff = rhs.coefficients.get(i-j).cloned().unwrap_or(Vector2::zero());
                coeff += Vector2::new(first_coeff.x * second_coeff.x, first_coeff.y * second_coeff.y);
            }
            output_coeffs.push(coeff);
        }

        Self {
            coefficients: output_coeffs
        }
    }
}

impl Mul<Polynomial2d> for f64 {
    type Output = Polynomial2d;

    fn mul(self, rhs: Polynomial2d) -> Self::Output {
        let new_coeffs = rhs.coefficients.iter().map(|c| {self * c}).collect();

        Polynomial2d {
            coefficients: new_coeffs
        }
    }
}