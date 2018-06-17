use ::geometry::Point;

pub trait TwoDTransformable {
    fn shift_by(&mut self, shift: Point);
    fn rotate(&mut self, rot_angle: f64);
}