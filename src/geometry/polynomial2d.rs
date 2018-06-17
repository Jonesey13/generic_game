use std::ops::{Add, Mul};
use ::geometry::Point;
use geometry::Polynomial;

#[derive(Clone, Debug)]
pub struct Polynomial2d {
    coefficients: Vec<Point> // lowest to highest
}

impl Polynomial2d {
    pub fn new(coefficients: Vec<Point>) -> Self {
        Self {
            coefficients
        }
    }

    pub fn new_from_1d_polys(poly1: Polynomial, poly2: Polynomial) -> Self {
        let coefficients = poly1.get_coeffs().iter()
                            .zip(poly2.get_coeffs().iter())
            .map(|(coeff1, coeff2)| {Point::new(*coeff1, *coeff2)}).collect();

        Self {
            coefficients
        }
    }

    pub fn get_coeffs(&self) -> &Vec<Point> {
        &self.coefficients
    }

    pub fn new_x_minus(val: f64) -> Self {
        Self {
            coefficients: vec![-Point::new(val, val), Point::new(1.0, 1.0)]
        }
    }

    pub fn identity() -> Self {
        Self {
            coefficients: vec![Point::new(1.0, 1.0)]
        }
    }

    pub fn zero() -> Self {
        Self {
            coefficients: vec![Point::zero()]
        }
    }

    pub fn get_degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn get_point(&self, time: f64) -> Point {
        let mut output = Point::zero();

        for (index, coeff) in self.coefficients.iter().cloned().enumerate() {
            output = output + time.powi(index as i32) * coeff;
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
            let coeff = self.coefficients.get(i).cloned().unwrap_or(Point::zero()) 
                        + rhs.coefficients.get(i).cloned().unwrap_or(Point::zero());
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
            let mut coeff = Point::zero();
            for j in 0..i + 1 {
                let first_coeff = self.coefficients.get(j).cloned().unwrap_or(Point::zero());
                let second_coeff = rhs.coefficients.get(i-j).cloned().unwrap_or(Point::zero());
                coeff += Point::new(first_coeff.x * second_coeff.x, first_coeff.y * second_coeff.y);
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