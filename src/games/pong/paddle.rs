use na::{Vector2, Vector4};

#[derive(Clone)]
pub struct Paddle {
    pub length: f64,
    pub width: f64,
    pub color: Vector4<f64>
}

impl Paddle {
    pub fn new(size: Vector2<f64>, color: Vector4<f64>) -> Self {
        Paddle {
            length: size.x,
            width: size.y,
            color: color
        }
    }
}
