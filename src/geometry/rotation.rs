use std::ops::Mul;
use ::geometry::*;

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

    fn generate_mat(&self) -> [[f64; 2]; 2] {
        return [
            [self.angle.cos(), -self.angle.sin()],
            [self.angle.sin(), self.angle.cos()]
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
