use na::{Vector1, Rotation, norm, dot};
use num::Zero;
use geometry::*;
use std::f64::consts;
use std::iter::{Repeat, repeat};

/// Refers to an implementation of a polygon
pub trait Poly : TwoDTransformable {
    fn get_corners(&self) -> Vec<Point>;
    fn set_corners(&mut self, corners: Vec<Point>); 

    // Normals face outwards
    fn normals(&self) -> Vec<Point> {
        let corners_it_shift = self.get_corners().into_iter().cycle().skip(1);
        self.get_corners().into_iter().zip(corners_it_shift).map(|(beg, end)| {-get_normal_2d(end - beg)}).collect()
    }

    fn get_normal(&self, index: usize) -> Point {
        self.normals()[index]
    }

    fn sides(&self) -> Vec<Line> {
        self.sides_iter().map(|(beg, end)| {Line::new(beg, end)}).take(self.total_sides()).collect()
    }

    fn get_side(&self, index: usize) -> Option<Line> {
        self.sides_iter().nth(index).and_then(|(beg, end)| {Some(Line::new(beg, end))})
    }

    fn sides_iter<'a>(&'a self) -> Box<Iterator<Item=(Point, Point)> + 'a> {
        let corners_it_shift = self.get_corners().into_iter().cycle().skip(1);
        Box::new(self.get_corners().into_iter().zip(corners_it_shift).map(|(beg, end)| {(beg, end)}))
    }

    /// Given a corner on the ConPoly, get the two adjacent sides as lines
    fn get_adjacent_sides(&self, corner_index: usize) -> Option<(Line, usize, Line, usize)> {
        let indices = match corner_index {
            0 => [self.total_sides() - 1, corner_index],
            _ => [corner_index - 1, corner_index],
        };
        match (self.get_side(indices[0]), self.get_side(indices[1])) {
            (Some(val1), Some(val2)) => Some((val1, indices[0], val2, indices[1])),
            _ => None
        }
    }

    fn total_sides(&self) -> usize {
        self.get_corners().len()
    }

    fn get_corner_lines(&self, other: &Poly) -> Vec<Line> {
        self.get_corners().into_iter()
        .zip(other.get_corners().into_iter())
        .map(|(beg, end)| { Line::new(beg, end) })
        .collect()
    }

    fn get_shift(&self, other: &Poly) -> Point {
        other.get_corners()[0] - self.get_corners()[0]
    }
}

pub fn get_at_time<T: Poly + Sized + Clone>(poly: &T, shift: Point, time: f64) -> T {
    let mut out = poly.clone();
    out.shift_by(time * shift);
    out
}

pub fn get_shifted<T: Poly + Sized + Clone>(poly: &T, shift: Point) -> T {
    let mut out = poly.clone();
    out.shift_by(shift);
    out
}