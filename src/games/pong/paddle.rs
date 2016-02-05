use na::{Vec2, Vec4};

#[derive(Clone)]
pub struct Paddle {
    pub length: f64,
    pub width: f64,
    pub color: Vec4<f64>
}

impl Paddle {
    pub fn new(size: Vec2<f64>, color: Vec4<f64>) -> Self {
        Paddle {
            length: size.x,
            width: size.y,
            color: color
        }
    }
}
