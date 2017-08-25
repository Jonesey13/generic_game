use na::Vector2;

pub trait TwoDTransformable {
    fn shift_by(&mut self, shift: Vector2<f64>);
    fn rotate(&mut self, rot_angle: f64);
}