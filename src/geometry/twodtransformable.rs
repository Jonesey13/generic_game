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

impl<T> TwoDTransformable for HasTwoDTransformable<Transformable=T>
    where T : TwoDTransformable
{
    fn shift_by(&mut self, shift: Point) {
        self.get_transformable_mut().shift_by(shift);
    }

    fn rotate(&mut self, rot_angle: f64) {
        self.get_transformable_mut().rotate(rot_angle);
    }

    fn get_center(&self) -> Point {
        self.get_transformable().get_center()
    }

    fn scale_by(&mut self, scale_factor: f64) {
        self.get_transformable_mut().scale_by(scale_factor);
    }
}