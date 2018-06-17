use ::geometry::Point;

pub struct CartesianRectangle {
    center: Point,
    width: f64,
    height: f64
}

impl CartesianRectangle {
    pub fn x_bounds(&self) -> Point {
        Point::new(self.center.x - self.width / 2.0, self.center.x + self.width / 2.0)
    }

    pub fn y_bounds(&self) -> Point {
        Point::new(self.center.y - self.height / 2.0, self.center.y + self.height / 2.0)
    }
}
