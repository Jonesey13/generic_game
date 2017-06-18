use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    start: f64,
    end: f64
}

impl Interval {
    pub fn new (start: f64, end: f64) -> Interval {
        Interval {
            start,
            end
        }
    }

    pub fn get_start(&self) -> f64 {
       self.start
    }

    pub fn get_end(&self) -> f64 {
       self.end
    }

    pub fn contains(&self, value: f64) -> bool {
        self.start < value && self.end > value
    }

    pub fn point_to_regularised(&self, point: f64) -> f64 {
        (point - self.start) / (self.end - self.start)
    }

    pub fn get_intermediate_point(&self, t: f64) -> f64 {
        self.start * (1.0 - t) + self.end * t
    }

    /// Fits a point in this interval to another interval
    pub fn fit_point_to(&self, point: f64, another: Interval) -> f64 {
        another.get_intermediate_point(self.point_to_regularised(point))
    }
}

impl Mul<f64> for Interval {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Interval::new(self.start * rhs, self.end * rhs)
    }
}
