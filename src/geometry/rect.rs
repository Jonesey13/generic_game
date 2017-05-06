use na::Vector2;

pub struct Rect {
    center: Vector2<f64>,
    width: f64,
    height: f64
}

impl Rect {
    pub fn x_bounds(&self) -> Vector2<f64> {
        Vector2::new(self.center.x - self.width / 2.0, self.center.x + self.width / 2.0)
    }

    pub fn y_bounds(&self) -> Vector2<f64> {
        Vector2::new(self.center.y - self.height / 2.0, self.center.y + self.height / 2.0)
    }
}
