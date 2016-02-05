use na::Vec2;

#[derive(Clone)]
pub struct Line {
    pub beg: Vec2<f64>,
    pub end: Vec2<f64>
}

impl Line {
    pub fn new(beg: Vec2<f64>, end: Vec2<f64>) -> Line {
        Line {
            beg: beg,
            end: end
        }
    }

    // alpha = 0 => beg, alpha = 1 => end
    pub fn get_point(&self, alpha: f64) -> Vec2<f64> {
        self.beg * (1.0 - alpha) + self.end * alpha
    }
}
