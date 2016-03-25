use std::cell::RefCell;
use std::iter::{Repeat, repeat};
use geometry;
use geometry::{circle, con_poly, line, HasAngle, QuadSoln};
use na::normalize;
use na::Vec2;

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

#[derive(Clone)]
pub enum CollDetails {
    None,
    Circ(Vec2<f64>), // Collision direction
    ConPolyInfo
}

#[derive(Clone)]
pub enum ConPolyInfo {
    LineInfo(usize, f64), // Line number and position on line => (0,1)
    CornerInfo(usize, Vec2<f64>) // Corner number and striking direction
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
            (CollObj::ConPoly(n1, p1), CollObj::Circ(n2, p2)) => circ_poly_coll(n2, p2, n1, p1),
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
    let circ2_next_rel = circ2_next.shift_by(shift1 * -1.0);
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

#[allow(unused_variables)]
pub fn circ_poly_coll<T: Clone>(circ_next: circle::Circle, circ_prev: circle::Circle, poly_next: con_poly::ConPoly, poly_prev: con_poly::ConPoly)
                         -> (CollResults<T>, CollResults<T>) {
    panic!("not implemented!");
}

#[allow(unused_variables)]
pub fn poly_poly_coll<T: Clone>(poly1_next: con_poly::ConPoly, poly1_prev: con_poly::ConPoly, poly2_next: con_poly::ConPoly, poly2_prev: con_poly::ConPoly)
                         -> (CollResults<T>, CollResults<T>) {
    panic!("not implemented!");
}
