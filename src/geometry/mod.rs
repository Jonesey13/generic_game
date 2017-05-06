pub mod line;
pub mod circle;
pub mod vect;
pub mod con_poly;
pub mod bezier_2d;
pub mod rect;
pub mod bezier_patch;
use na::{Vector2, norm, dot};
use num::Zero;

const EPSILON: f64 = 0.0000001;

/// For the line beg <=> t=0 and end <=> t=1
/// Results are expressed as points on the line parameterised in t
pub fn line_circle_intersect (line: &line::Line, circ: &circle::Circle) -> DualSoln {
    let shifted_line = line.shift_by(circ.center * -1.0);
    line_center_circle_intersect(&shifted_line, circ.rad)
}

/// Circle is assumed to be centered on the origin
/// For the line beg <=> t=0 and end <=> t=1
/// Results are expressed as points on the line parameterised in t
pub fn line_center_circle_intersect (line: &line::Line, circ_rad: f64) -> DualSoln {
    let a = line.get_diff().norm_squared();
    let b = 2.0 * dot(&line.beg, &line.get_diff());
    let c = line.beg.norm_squared() - circ_rad * circ_rad;
    solve_quadratic(a, b, c)
}

pub fn solve_quadratic(a: f64, b: f64, c:f64) -> DualSoln {
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return DualSoln::None;
    }
    else {
        let first = -b / (2.0*a);
        let second = discriminant.sqrt() / (2.0*a);
        return DualSoln::Two(first - second, first + second);
    }
}

pub fn average_vec2(vecs: Vec<Vector2<f64>>) -> Vector2<f64> {
    vecs.iter().fold(Vector2::zero(), |acc, &x| acc + x) / vecs.len() as f64
}

#[derive(Copy, Clone, Debug)]
pub enum DualSoln {
    None,
    Two(f64, f64),
}

impl DualSoln {
    /// Returns smallest of two in [0,1] (if any)
    pub fn smallest_within_zero_one(&self) -> Option<f64> {
        match self {
            &DualSoln::Two(x, y) if in_zero_one(x) && in_zero_one(y) && x < y => Some(x),
            &DualSoln::Two(x, y) if in_zero_one(x) && in_zero_one(y) && y < x => Some(y),
            &DualSoln::Two(x, y) if in_zero_one(x) && !in_zero_one(y) => Some(x),
            &DualSoln::Two(x, y) if !in_zero_one(x) && in_zero_one(y) => Some(y),
            &DualSoln::Two(_, _) => None,
            &DualSoln::None => None
        }
    }

    pub fn both_positive(&self) -> bool {
        match self {
            &DualSoln::Two(x, y) => x >= 0.0 && y >= 0.0,
            &DualSoln::None => false
        }
    }

    pub fn both_within_zero_one(&self) -> bool {
        match self {
            &DualSoln::Two(x, y) => in_zero_one(x) && in_zero_one(y),
            &DualSoln::None => false
        }
    }

    pub fn both_strictly_within_zero_one(&self) -> bool {
        match self {
            &DualSoln::Two(x, y) => in_zero_one_strict(x) && in_zero_one_strict(y),
            &DualSoln::None => false
        }
    }
}

fn in_zero_one(x: f64) -> bool { x >= 0.0 && x <= 1.0 }

fn in_zero_one_strict(x: f64) -> bool { x > 0.0 + EPSILON && x < 1.0 - EPSILON}

/// For the line beg <=> t=0 and end <=> t=1
/// For the two values in the DualSoln the first float corresponds to a point on
/// the first line and the second float the second line
pub fn line_line_intersect_2d(line1: &line::Line, line2: &line::Line) -> DualSoln {
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

pub trait HasAngle {
    fn get_angle(&self) -> f64;
}

impl HasAngle for Vector2<f64> {
    fn get_angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

pub trait FromAngle {
    fn from_angle(f64) -> Self;
}

impl FromAngle for Vector2<f64> {
    fn from_angle(angle: f64) -> Self {
        Vector2::new(angle.cos(), angle.sin())
    }
}

#[test]
fn line_line_intersect() {
    let line1 = line::Line::new(Vector2::new(-0.5, 0.0), Vector2::new(0.5, 0.0));
    let line2 = line::Line::new(Vector2::new(0.3, 1.0), Vector2::new(0.3, -1.0));
    let soln = line_line_intersect_2d(&line1, &line2);
    assert!(soln.both_within_zero_one(), "soln: {:?}", soln)
}
