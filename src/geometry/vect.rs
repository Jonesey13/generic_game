use na::{Vec2, Norm};

/// Anticlockwise rotation
pub fn get_normal_2d(vect: Vec2<f64>) -> Vec2<f64>  {
    Vec2::new(-vect.y, vect.x).normalize()
}

pub fn get_rot90_2d(vect: Vec2<f64>) -> Vec2<f64>  {
    Vec2::new(-vect.y, vect.x)
}
