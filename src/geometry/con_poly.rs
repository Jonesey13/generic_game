use na::{Vector1, Vector2, Rotation2, norm, dot};
use num::Zero;
use geometry::average_vec2;
use geometry::line::Line;
use geometry::vect::get_normal_2d;
use std::f64::consts;
use std::iter::{Repeat, repeat};

/// A convex polygon for collision detection
#[derive(Clone, Debug)]
pub struct ConPoly {
    pub corners: Vec<Vector2<f64>>, // defined anticlockwise
}

impl ConPoly {
    /// Length <=> x-axis, Height <=> y-axis
    pub fn new_from_rect(length:f64, height:f64, pos: Vector2<f64>, rot: Rotation2<f64>) -> ConPoly {
        let xshift = Vector2::<f64>::new(length, 0.0);
        let yshift = Vector2::<f64>::new(0.0, height);
        let bottom_left = Vector2::zero();
        let bottom_right = bottom_left + xshift;
        let top_right = bottom_left + xshift + yshift;
        let top_left = bottom_left + yshift;
        let mut corners = vec![bottom_left, bottom_right, top_right, top_left];

        let average = average_vec2(corners.clone());
        for mut vector in corners.iter_mut() {
            *vector = rot * (*vector - average) + pos;
        }
        ConPoly {
            corners: corners
        }
    }

    pub fn new_from_lines(lines: Vec<Line>) -> ConPoly {
        ConPoly {
            corners: lines.iter().map(|line| {line.beg}).collect()
        }
    }

    pub fn new (corners: Vec<Vector2<f64>>) -> ConPoly {
        ConPoly {
            corners: corners
        }
    }

    pub fn shift_by(&mut self, shift: Vector2<f64>) {
        for corner in self.corners.iter_mut() {
            *corner = *corner + shift;
        }
    }

    pub fn shifted_by(&self, shift: Vector2<f64>) -> ConPoly {
        let mut out = self.clone();
        out.shift_by(shift);
        out
    }

    // Normals face outwards
    pub fn normals(&self) -> Vec<Vector2<f64>> {
        let corners_it_shift = self.corners.iter().cloned().cycle().skip(1);
        self.corners.iter().cloned().zip(corners_it_shift).map(|(beg, end)| {-get_normal_2d(end - beg)}).collect()
    }

    pub fn get_normal(&self, index: usize) -> Vector2<f64> {
        self.normals()[index]
    }

    pub fn sides(&self) -> Vec<Line> {
        self.sides_iter().map(|(beg, end)| {Line::new_ref(beg, end)}).take(self.total_sides()).collect()
    }

    pub fn get_side(&self, index: usize) -> Option<Line> {
        self.sides_iter().nth(index).and_then(|(beg, end)| {Some(Line::new_ref(beg, end))})
    }

    fn sides_iter<'a>(&'a self) -> Box<Iterator<Item=(&'a Vector2<f64>, &'a Vector2<f64>)> + 'a> {
        let corners_it_shift = self.corners.iter().cycle().skip(1);
        Box::new(self.corners.iter().zip(corners_it_shift).map(|(beg, end)| {(beg, end)}))
    }

    /// Given a corner on the ConPoly, get the two adjacent sides as lines
    pub fn get_adjacent_sides(&self, corner_index: usize) -> Option<(Line, usize, Line, usize)> {
        let indices = match corner_index {
            0 => [self.total_sides() - 1, corner_index],
            _ => [corner_index - 1, corner_index],
        };
        match (self.get_side(indices[0]), self.get_side(indices[1])) {
            (Some(val1), Some(val2)) => Some((val1, indices[0], val2, indices[1])),
            _ => None
        }
    }

    pub fn total_sides(&self) -> usize {
        self.corners.len()
    }

    pub fn get_corner_lines(&self, poly2: &ConPoly) -> Vec<Line> {
        self.corners.iter().cloned()
        .zip(poly2.corners.iter().cloned())
        .map(|(beg, end)| { Line::new(beg, end) })
        .collect()
    }

    pub fn get_shift(&self, other: &ConPoly) -> Vector2<f64> {
        other.corners[0] - self.corners[0]
    }

    pub fn get_at_time(&self, other: &ConPoly, time: f64) -> ConPoly {
        let mut out = self.clone();
        let shift = self.get_shift(&other);
        out.shift_by(shift * time);
        out
    }

    pub fn interior_point_check(&self, point: Vector2<f64>) -> Option<Vector2<f64>> {
        let mut outside = false;
        let mut correction = Vector2::zero();
        
        for (&side, &normal) in self.sides().iter().zip(self.normals().iter()) {
            let overlap = dot(&(point - side.beg), &normal);
            if overlap > 0.0 {
                correction -= overlap * normal;
                outside = true;
            }
        }
        match outside {
            true => Some(correction),
            false => None
        }
    }
}

#[test]
fn point_inside_poly_test() {
    let corners = vec![
        Vector2::new(-1.0, -1.0),
        Vector2::new(1.0, -1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(-1.0, 1.0)

    ];
    let test_poly = ConPoly::new(corners);

    let test_point = Vector2::new(1.2,0.2);

    let overlap = test_poly.interior_point_check(test_point);

    assert!(overlap.is_some());

    assert!(norm(&(overlap.unwrap() - Vector2::new(-0.2, 0.0))) < 0.00001);
}
