pub mod line;
pub mod circle;
pub mod vect;
pub mod con_poly;
use na::{Vec2, Norm, dot};
use num::Zero;

/// For the line beg <=> t=0 and end <=> t=1
/// Results are expressed as points on the line parameterised in t
pub fn line_circle_intersect (line: line::Line, circ: circle::Circle) -> QuadSoln {
    let shifted_line = line.shift_by(circ.center * -1.0);
    line_center_circle_intersect(shifted_line, circ.rad)
}

/// Circle is assumed to be centered on the origin
/// For the line beg <=> t=0 and end <=> t=1
/// Results are expressed as points on the line parameterised in t
pub fn line_center_circle_intersect (line: line::Line, circ_rad: f64) -> QuadSoln {
    let a = line.get_direction().sqnorm();
    let b = 2.0 * dot(&line.beg, &line.get_direction());
    let c = line.beg.sqnorm() - circ_rad * circ_rad;
    solve_quadratic(a, b, c)
}

pub fn solve_quadratic(a: f64, b: f64, c:f64) -> QuadSoln {
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return QuadSoln::None;
    }
    else {
        let first = -b / (2.0*a);
        let second = discriminant.sqrt() / (2.0*a);
        return QuadSoln::Two(first - second, first + second);
    }
}

pub enum QuadSoln {
    None,
    Two(f64, f64), // Ascending Order
}

impl QuadSoln {
    /// Returns smallest of two in [0,1] (if any)
    pub fn smallest_within_zero_one(&self) -> Option<f64> {
        match self {
            &QuadSoln::Two(x, y) if in_zero_one(x) && in_zero_one(y) && x < y => Some(x),
            &QuadSoln::Two(x, y) if in_zero_one(x) && in_zero_one(y) && y < x => Some(y),
            &QuadSoln::Two(x, y) if in_zero_one(x) && !in_zero_one(y) => Some(x),
            &QuadSoln::Two(x, y) if !in_zero_one(x) && in_zero_one(y) => Some(y),
            &QuadSoln::Two(_, _) => None,
            &QuadSoln::None => None
        }
    }

    pub fn both_positive(&self) -> bool {
        match self {
            &QuadSoln::Two(x, y) => x >= 0.0 && y >= 0.0,
            &QuadSoln::None => false
        }
    }
}

fn in_zero_one(x: f64) -> bool { x >= 0.0 && x <= 1.0 }

/// For the line beg <=> t=0 and end <=> t=1
/// For the two values in the QuadSoln the first float corresponds to a point on
/// the first line and the second float the second line
pub fn line_line_intersect_2d(line1: line::Line, line2: line::Line) -> QuadSoln {
    let dir1 = line1.get_direction();
    let dir2 = line2.get_direction();
    let normal1 = line1.get_normal();
    let normal2 = line2.get_normal();
    if dot(&dir1, &normal1) != 0.0 {
        let t1 = dot(&(line2.beg - line1.beg), &normal2) / dot(&dir1, &normal2);
        let t2 = dot(&(line1.beg - line2.beg), &normal1) / dot(&dir2, &normal1);
        return QuadSoln::Two(t1, t2);
    }
    QuadSoln::None
}

pub trait HasAngle {
    fn get_angle(&self) -> f64;
}

impl HasAngle for Vec2<f64> {
    fn get_angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

pub trait FromAngle {
    fn from_angle(f64) -> Self;
}

impl FromAngle for Vec2<f64> {
    fn from_angle(angle: f64) -> Self {
        Vec2::new(angle.cos(), angle.sin())
    }
}
