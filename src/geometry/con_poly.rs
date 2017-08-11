use na::{Vector1, Vector2, Rotation2, norm, dot};
use num::Zero;
use geometry::average_vec2;
use geometry::line::Line;
use geometry::vect::get_normal_2d;
use geometry::Poly;
use std::f64::consts;
use std::iter::{Repeat, repeat};

/// A convex polygon for collision detection
#[derive(Clone, Debug)]
pub struct ConPoly {
    pub corners: Vec<Vector2<f64>>, // defined anticlockwise
}

impl Poly for ConPoly {
    fn get_corners(&self) -> Vec<Vector2<f64>> {
        self.corners.clone()
    }

    fn set_corners(&mut self, corners: Vec<Vector2<f64>>) {
        self.corners = corners;
    }
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
