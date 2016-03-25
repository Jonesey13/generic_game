use na::Vec2;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Circle{
    pub rad: f64,
    pub center: Vec2<f64>
}

impl Circle {
    pub fn new(rad: f64, center: Vec2<f64>) -> Circle {
        Circle{
            rad: rad,
            center: center
        }
    }

    pub fn shift_by(&self, move_vec: Vec2<f64>) -> Circle {
        Circle {
            rad: self.rad,
            center: self.center + move_vec
        }
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle: radius: {}, center: {{ x: {}, y: {} }}", self.rad, self.center.x, self.center.y)
    }
}
