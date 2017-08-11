use na::{Vector2, norm, dot};
use super::{vect, DualSoln, Poly} ;

#[derive(Copy, Clone, Debug)]
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

    pub fn get_lines_to(&self, other: Line) -> (Line, Line) {
        (Line::new(self.beg, other.beg), Line::new(self.end, other.end))
    }
}

impl Poly for Line {
    fn get_corners(&self) -> Vec<Vector2<f64>> {
        vec![self.beg, self.end]
    }
    fn set_corners(&mut self, corners: Vec<Vector2<f64>>) {
        self.beg = corners[0];
        self.end = corners[1];
    }
}

/// For the line beg <=> t=0 and end <=> t=1
/// For the two values in the DualSoln the first float corresponds to a point on
/// the first line and the second float the second line
pub fn line_line_intersect_2d(line1: &Line, line2: &Line) -> DualSoln {
    let dir1 = line1.get_diff();
    let dir2 = line2.get_diff();
    let normal1 = line1.get_normal();
    let normal2 = line2.get_normal();
    if dot(&dir1, &normal2) != 0.0 {
        let t1 = dot(&(line2.beg - line1.beg), &normal2) / dot(&dir1, &normal2);
        let t2 = dot(&(line1.beg - line2.beg), &normal1) / dot(&dir2, &normal1);
        return DualSoln::Two(t1, t2);
    }
    DualSoln::None
}

#[cfg(test)]
mod tests
{
    #[test]
    fn line_line_intersect() {
        let line1 = Line::new(Vector2::new(-0.5, 0.0), Vector2::new(0.5, 0.0));
        let line2 = Line::new(Vector2::new(0.3, 1.0), Vector2::new(0.3, -1.0));
        let soln = line_line_intersect_2d(&line1, &line2);
        assert!(soln.both_within_zero_one(), "soln: {:?}", soln)
    }
}