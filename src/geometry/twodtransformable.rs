use crate::geometry::Point;

pub trait TwoDTransformable {
    fn shift_by(&mut self, shift: Point);
    fn rotate_at_center(&mut self, rot_angle: f64);
    fn rotate_at_origin(&mut self, rot_angle: f64);
    fn get_center(&self) -> Point;
    fn scale_by(&mut self, scale_factor: f64); // Should scale relative to center
}

pub trait HasTwoDTransformable {
    type Transformable: TwoDTransformable;

    fn get_transformable(&self) -> &Self::Transformable;
    fn get_transformable_mut(&mut self) -> &mut Self::Transformable;
}

impl<T, S> TwoDTransformable for T
    where T : HasTwoDTransformable<Transformable=S>,
          S : TwoDTransformable
{
    fn shift_by(&mut self, shift: Point) {
        self.get_transformable_mut().shift_by(shift);
    }

    fn rotate_at_center(&mut self, rot_angle: f64) {
        self.get_transformable_mut().rotate_at_center(rot_angle);
    }

    fn rotate_at_origin(&mut self, rot_angle: f64) {
        self.get_transformable_mut().rotate_at_origin(rot_angle);
    }

    fn get_center(&self) -> Point {
        self.get_transformable().get_center()
    }

    fn scale_by(&mut self, scale_factor: f64) {
        self.get_transformable_mut().scale_by(scale_factor);
    }
}