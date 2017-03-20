use na::{Vector2, norm};
use super::vect;

#[derive(Clone, Debug)]
pub struct Line {
    pub beg: Vector2<f64>,
    pub end: Vector2<f64>
}

impl Line {
    pub fn new(beg: Vector2<f64>, end: Vector2<f64>) -> Line {
        Line {
            beg: beg,
            end: end
        }
    }

    pub fn new_ref(beg: &Vector2<f64>, end: &Vector2<f64>) -> Line {
        Line {
            beg: *beg,
            end: *end
        }
    }

    // alpha = 0 => beg, alpha = 1 => end
    pub fn get_point(&self, alpha: f64) -> Vector2<f64> {
        self.beg * (1.0 - alpha) + self.end * alpha
    }

    pub fn get_diff(&self) -> Vector2<f64> {
        self.end - self.beg
    }

    pub fn get_direction(&self) -> Vector2<f64> {
        (self.end - self.beg).normalize()
    }

    pub fn get_normal(&self) -> Vector2<f64> {
        let dir = self.get_diff();
        vect::get_normal_2d(dir)
    }

    pub fn get_unnormalized_normal(&self) -> Vector2<f64> {
        let dir = self.get_diff();
        vect::get_rot90_2d(dir)
    }

    pub fn shift_by(&self, move_vec: Vector2<f64>) -> Line {
        Line {
            beg: self.beg + move_vec,
            end: self.end + move_vec
        }
    }
}
