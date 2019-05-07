use std::ops::Mul;
use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Rotation {
    pub angle: f64
}

impl Rotation {
    pub fn new(angle: f64) -> Self {
        Self {
            angle
        }
    }

    pub fn get_matrix(&self) -> [[f64; 2]; 2] {
        return [
            [self.angle.cos(), self.angle.sin()],
            [-self.angle.sin(), self.angle.cos()]
        ]
    }

    pub fn get_matrix_f32(&self) -> [[f32; 2]; 2] {
        return [
            [self.angle.cos() as f32, self.angle.sin() as f32],
            [-self.angle.sin() as f32, self.angle.cos() as f32]
        ]
    }
}

impl Mul<Rotation> for Rotation {
    type Output = Rotation;

    fn mul(self, other: Rotation) -> Rotation {
        Self {
            angle: self.angle + other.angle
        }
    }
}

impl Mul<Point> for Rotation {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        Point::new(
            self.angle.cos() * point.x - self.angle.sin() * point.y,
            self.angle.sin() * point.x + self.angle.cos() * point.y
        )
    }
}
