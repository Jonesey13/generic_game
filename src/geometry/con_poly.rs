use na::{Vec1, Vec2, Rot2, normalize};
use num::Zero;
use geometry::average_vec2;
use geometry::line::Line;
use geometry::vect::get_normal_2d;
use std::f64::consts;
use std::iter::{Repeat, repeat};

/// A convex polygon for collision detection
#[derive(Clone, Debug)]
pub struct ConPoly {
    pub corners: Vec<Vec2<f64>>, // defined anticlockwise
}

impl ConPoly {
    /// Length <=> x-axis, Height <=> y-axis
    pub fn new_from_rect(length:f64, height:f64, pos: Vec2<f64>, rot: Rot2<f64>) -> ConPoly {
        let xshift = Vec2::<f64>::new(length, 0.0);
        let yshift = Vec2::<f64>::new(0.0, height);
        let bottom_left = Vec2::zero();
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

    pub fn shift_by(&mut self, shift: Vec2<f64>) {
        for corner in self.corners.iter_mut() {
            *corner = *corner + shift;
        }
    }

    pub fn shifted_by(&self, shift: Vec2<f64>) -> ConPoly {
        let mut out = self.clone();
        out.shift_by(shift);
        out
    }

    pub fn normals(&self) -> Vec<Vec2<f64>> {
        let corners_it_shift = self.corners.iter().cloned().cycle().skip(1);
        self.corners.iter().cloned().zip(corners_it_shift).map(|(beg, end)| {-get_normal_2d(end - beg)}).collect()
    }

    pub fn get_side(&self, index: usize) -> Line {
        let beg = self.corners[index];
        let end = self.corners.iter().cloned().cycle().skip(1).nth(index).unwrap();
        Line::new(beg, end)
    }

    pub fn get_normal(&self, index: usize) -> Vec2<f64> {
        -get_normal_2d(self.get_side(index).get_direction())
    }

    pub fn total_sides(&self) -> usize {
        self.corners.len()
    }
}
