use na::{Vector2, norm};

/// Anticlockwise rotation
pub fn get_normal_2d(vect: Vector2<f64>) -> Vector2<f64>  {
    Vector2::new(-vect.y, vect.x).normalize()
}

pub fn get_rot90_2d(vect: Vector2<f64>) -> Vector2<f64>  {
    Vector2::new(-vect.y, vect.x)
}
