use crate::geometry::Point;

pub struct CartesianRectangle {
    center: Point,
    width: f64,
    height: f64
}

impl CartesianRectangle {
    pub fn new(center: Point, width: f64, height: f64) -> Self {
        Self {
            center,
            width,
            height
        }
    }

    pub fn x_bounds(&self) -> Point {
        Point::new(self.center.x - self.width / 2.0, self.center.x + self.width / 2.0)
    }

    pub fn y_bounds(&self) -> Point {
        Point::new(self.center.y - self.height / 2.0, self.center.y + self.height / 2.0)
    }

    pub fn contains_point(&self, point: Point) -> bool {
        let x_bounds = self.x_bounds();
        let y_bounds = self.y_bounds();

        point.x < x_bounds.y 
        && point.x > x_bounds.x
        && point.y < y_bounds.y
        && point.y > y_bounds.x
    }
}
