use na::Vec2;
use std::fmt;
use geometry::line::Line;

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

    pub fn shift_by(&mut self, shift: Vec2<f64>) {
        self.center = self.center + shift;
    }

    pub fn shifted_by(&self, shift: Vec2<f64>) -> Circle {
        let mut out = self.clone();
        out.shift_by(shift);
        out
    }

    pub fn get_movement_line(&self, other: &Circle) -> Line {
        Line::new(self.center, other.center)
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle: radius: {}, center: {{ x: {}, y: {} }}", self.rad, self.center.x, self.center.y)
    }
}
