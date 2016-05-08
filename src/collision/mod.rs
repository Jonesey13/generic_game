use std::cell::RefCell;
use std::iter::{Repeat, repeat};
use std::cmp::Ordering::{Equal, Less, Greater};
use geometry;
use geometry::{circle, con_poly, line, HasAngle, DualSoln};
use na::{normalize, Vec2, dot};
use utils::debug::*;

pub trait Collidable {
    type Data: Clone;
    fn get_collision_object(&self) -> CollObj { CollObj::None }
    fn get_collision_results(&self) -> CollResults<Self::Data>;
    fn set_collision_results(&mut self, CollResults<Self::Data>);
    fn get_collision_time(&mut self) -> Option<f64> {self.get_collision_results().time}
    fn has_collided(&self) -> bool { self.get_collision_results().collided }
    fn get_collision_details(&self) -> Option<CollDetails> { self.get_collision_results().details }
    fn get_collision_data(&self) -> Self::Data;
}

pub enum CollObj {
    None,
    Circ(circle::Circle, circle::Circle),
    ConPoly(con_poly::ConPoly, con_poly::ConPoly)
}

#[derive(Clone, Debug)]
pub enum CollDetails {
    None,
    Circ(Vec2<f64>), // Collision direction, outward from object
    ConPoly(ConPolyInfo)
}

#[derive(Clone, Debug)]
pub enum ConPolyInfo {
    LineInfo(usize, f64), // Line number and position on line => (0,1)
    CornerInfo(usize, Vec2<f64>) // Corner number and striking direction (outward)
}

#[derive(Clone)]
pub struct CollResults<T: Clone> {
    collided: bool,
    details: Option<CollDetails>,
    pub time: Option<f64>,
    data: Option<T>
}

impl<T: Clone> CollResults<T> {
    pub fn no_collision() -> CollResults<T> {
        CollResults {
            collided: false,
            details: None,
            time: None,
            data: None,
        }
    }

    pub fn collided(details: CollDetails, time: f64) -> CollResults<T> {
        CollResults {
            collided: true,
            details: Some(details),
            time: Some(time),
            data: None,
        }
    }
}

pub struct Collider;

impl Collider {
    pub fn process_all<T: Clone> (&mut self, mut objects: Vec<&mut Collidable<Data=T>>) {
        loop {
            if let Some((first, rest)) = objects.split_last_mut() {
                for mut second in rest {
                    Collider::process_pair(first, second);
                }
            }
            else {
                break;
            }
            objects.pop();
        }
    }

    fn process_pair<T: Clone> (first: &mut&mut Collidable<Data=T>, second: &mut&mut Collidable<Data=T>) {
        let coll_object1 = first.get_collision_object();
        let coll_object2 = second.get_collision_object();
        let (mut results1, mut results2): (CollResults<T>, CollResults<T>) = match (coll_object1, coll_object2) {
            (CollObj::Circ(n1, p1), CollObj::Circ(n2, p2)) => circ_circ_coll(n1, p1, n2, p2),
            (CollObj::Circ(n1, p1), CollObj::ConPoly(n2, p2)) => circ_poly_coll(n1, p1, n2, p2),
            (CollObj::ConPoly(n1, p1), CollObj::Circ(n2, p2)) => {let res = circ_poly_coll(n2, p2, n1, p1); (res.1, res.0)},
            (CollObj::ConPoly(n1, p1), CollObj::ConPoly(n2, p2)) => poly_poly_coll(n1, p1, n2, p2),
            _ => return,
        };

        let data1 = first.get_collision_data();
        let data2 = second.get_collision_data();
        results1.data = Some(data2);
        results2.data = Some(data1);
        first.set_collision_results(results1);
        second.set_collision_results(results2);
    }
}

pub fn circ_circ_coll<T: Clone>(circ1_next: circle::Circle, circ1_prev: circle::Circle, circ2_next: circle::Circle, circ2_prev: circle::Circle)
                         -> (CollResults<T>, CollResults<T>) {
    // Make circ1 stationary
    let shift1 = circ1_next.center - circ1_prev.center;
    let circ2_next_rel = circ2_next.shifted_by(shift1 * -1.0);
    let shift2_rel = circ2_next_rel.center - circ2_prev.center;
    let rad_tot = circ1_prev.rad + circ2_prev.rad;
    let circ2_line = line::Line::new(circ2_prev.center, circ2_next_rel.center);
    let coll_soln = geometry::line_circle_intersect(circ2_line, circle::Circle::new(rad_tot, circ1_prev.center));
    if let Some(time) = coll_soln.smallest_within_zero_one() {
        if coll_soln.both_positive() {
            let circ2_collision_center = circ2_prev.center + shift2_rel * time;
            let collision_dir = normalize(&(circ2_collision_center - circ1_prev.center));
            return (CollResults::collided(CollDetails::Circ(collision_dir), time),
                    CollResults::collided(CollDetails::Circ(collision_dir * -1.0), time));
        }
    }
    (CollResults::no_collision(), CollResults::no_collision())
}

pub fn circ_poly_coll<T: Clone>(circ_next: circle::Circle, circ_prev: circle::Circle,
                                poly_next: con_poly::ConPoly, poly_prev: con_poly::ConPoly)
                                -> (CollResults<T>, CollResults<T>) {
    let epsilon: f64 = 0.00001; // Used to distinguish between corner and side collisions.

    let earliest_corner = circ_poly_coll_corners(circ_next, circ_prev, poly_next.clone(), poly_prev.clone());

    let earliest_side = circ_poly_coll_sides(circ_next, circ_prev, poly_next, poly_prev);

    match (earliest_corner, earliest_side) {
        (None, None) => (CollResults::no_collision(), CollResults::no_collision()),
        (Some((c_det, p_det, t)), None) => (CollResults::collided(c_det, t), CollResults::collided(p_det, t)),
        (None, Some((c_det, p_det, t))) => (CollResults::collided(c_det, t), CollResults::collided(p_det, t)),
        (Some((c_det1, p_det1, t1)), Some((c_det2, p_det2, t2))) => {
            if t1 < t2 + epsilon {
                return (CollResults::collided(c_det1, t1), CollResults::collided(p_det1, t1));
            }
            else {
                return (CollResults::collided(c_det2, t2), CollResults::collided(p_det2, t2));
            }
        }
    }
}

fn circ_poly_coll_corners(circ_next: circle::Circle, circ_prev: circle::Circle,
                          poly_next: con_poly::ConPoly, poly_prev: con_poly::ConPoly)
                          -> Option<(CollDetails, CollDetails, f64)> {
    // Change frame of reference so that the Circle appears to be fixed
    let circ_shift = circ_next.center - circ_prev.center;
    let poly_next_rel = poly_next.shifted_by(circ_shift * - 1.0);

    let mut corner_solns = Vec::<Option<f64>>::new();

    for (corner_beg, corner_end) in poly_prev.corners.iter().zip(poly_next_rel.corners.iter()) {
        let corner_line = line::Line::new(*corner_beg, *corner_end);
        let corner_coll_soln = geometry::line_circle_intersect(corner_line, circ_prev);
        if corner_coll_soln.both_positive() {
            corner_solns.push(corner_coll_soln.smallest_within_zero_one());
        }
        else {
            corner_solns.push(None);
        }
    }

    let mut corner_solns_indexed: Vec<(usize, Option<f64>)> = (0..corner_solns.len()).zip(corner_solns.into_iter()).collect();

    corner_solns_indexed.sort_by(|&(_, a), &(_, b)| {
        match (a, b) {
            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(Equal),
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (None, None) => Equal
        }
    });

    match corner_solns_indexed[0] {
        (_, None) => None,
        (index, Some(t)) => {
            let corner_pos =  poly_prev.corners[index] * (1.0 - t) + poly_next_rel.corners[index] * t;
            let striking_dir = normalize(&(circ_prev.center - corner_pos));
            let circ_details = CollDetails::Circ(striking_dir * -1.0);
            let poly_details = CollDetails::ConPoly(ConPolyInfo::CornerInfo(index, striking_dir));
            Some((circ_details, poly_details, t))
        }
    }
}

fn circ_poly_coll_sides(circ_next: circle::Circle, circ_prev: circle::Circle,
                        poly_next: con_poly::ConPoly, poly_prev: con_poly::ConPoly)
                        -> Option<(CollDetails, CollDetails, f64)> {
    // Side checks next - requires polygon to be stationary
    let poly_shift = poly_next.corners[0] - poly_prev.corners[0];
    let circ_next_rel = circ_next.shifted_by(poly_shift * - 1.0);

    let mut side_solns = Vec::<Option<(f64, f64)>>::new();

    for ind in 0..poly_prev.total_sides() {
        let corner_line = poly_prev.get_side(ind);
        let normal = poly_prev.get_normal(ind);
        let circle_line = circ_prev.get_movement_line(&circ_next_rel).shift_by(- normal * circ_prev.rad);
        let circle_mov_dir = circle_line.get_direction();
        let side_coll_soln = geometry::line_line_intersect_2d(corner_line, circle_line);
        if let (DualSoln::Two(line_pos, time), true, true)
            = (side_coll_soln, side_coll_soln.both_within_zero_one(), dot(&circle_mov_dir, &normal) < 0.0) {
            side_solns.push(Some((line_pos, time)));
        }
        else {
            side_solns.push(None);
        }
    }

    let mut side_solns_indexed: Vec<(usize, Option<(f64,f64)>)> = (0..side_solns.len()).zip(side_solns.into_iter()).collect();

    side_solns_indexed.sort_by(|&(_, a), &(_, b)| {
        match (a, b) {
            (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(Equal),
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (None, None) => Equal
        }
    });

    match side_solns_indexed[0] {
        (_, None) => None,
        (index, Some((line_pos, t))) => {
            let circ_details = CollDetails::Circ(poly_prev.get_normal(index) * -1.0);
            let poly_details = CollDetails::ConPoly(ConPolyInfo::LineInfo(index, line_pos));
            Some((circ_details, poly_details, t))
        }
    }
}

#[allow(unused_variables)]
pub fn poly_poly_coll<T: Clone>(poly1_next: con_poly::ConPoly, poly1_prev: con_poly::ConPoly, poly2_next: con_poly::ConPoly, poly2_prev: con_poly::ConPoly)
                         -> (CollResults<T>, CollResults<T>) {
    panic!("not implemented!");
}
