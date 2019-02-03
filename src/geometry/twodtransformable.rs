use ::geometry::Point;

pub trait TwoDTransformable {
    fn shift_by(&mut self, shift: Point);
    fn rotate(&mut self, rot_angle: f64);
    fn get_center(&self) -> Point;
    fn scale_by(&mut self, scale_factor: f64); // Should scale relative to center
}

pub trait HasTwoDTransformable {
    type Transformable: TwoDTransformable;

    fn get_transformable(&self) -> &Self::Transformable;
    fn get_transformable_mut(&mut self) -> &mut Self::Transformable;
}