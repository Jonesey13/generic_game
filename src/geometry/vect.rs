use crate::geometry::*;

/// Anticlockwise rotation
pub fn get_normal_2d(vect: Point) -> Point  {
    Point::new(-vect.y, vect.x).normalized()
}

pub fn get_rot90_2d(vect: Point) -> Point  {
    Point::new(-vect.y, vect.x)
}
