use std::cmp::Ordering::{Equal, Less, Greater};
use geometry;
use geometry::{circle, con_poly, line, HasAngle, DualSoln, Poly, poly};
use geometry::circle::Circle;
use geometry::con_poly::ConPoly;
use geometry::line::Line;
use na::{normalize, Vector2, dot, abs};
use super::{CollResults, CollDetails};
use super::collision_details::ConPolyInfo;

static EPSILON: f64 = 0.0001;

pub fn circ_point_coll<T: Clone>(circ_next: &Circle, circ_prev: &Circle, point_next: &Vector2<f64>, point_prev: &Vector2<f64>)
                         -> (CollResults<T>, CollResults<T>) {
    // Make the circle stationary
    let circ_shift = circ_next.center - circ_prev.center;
    let point_next_rel = point_next + circ_shift * -1.0;
    let point_timeline_rel = Line::new(point_prev.clone(), point_next_rel);
    let coll_soln = geometry::line_circle_intersect(&point_timeline_rel, circ_prev);
    if let Some(time) = coll_soln.smallest_within_zero_one() {
        if coll_soln.both_positive() {
            let collision_dir = normalize(&(point_timeline_rel.get_point(time) - circ_prev.center));
            return (CollResults::collided(CollDetails::Circ(collision_dir), time),
                    CollResults::collided(CollDetails::Point(-collision_dir), time));
        }
    }
    (CollResults::no_collision(), CollResults::no_collision())
}

pub fn circ_circ_coll<T: Clone>(circ1_next: &Circle, circ1_prev: &Circle, circ2_next: &Circle, circ2_prev: &Circle)
                         -> (CollResults<T>, CollResults<T>) {
    // Make circ1 stationary
    let shift1 = circ1_next.center - circ1_prev.center;
    let circ2_next_rel = circ2_next.shifted_by(shift1 * -1.0);
    let shift2_rel = circ2_next_rel.center - circ2_prev.center;
    let rad_tot = circ1_prev.rad + circ2_prev.rad;
    let circ2_line = Line::new(circ2_prev.center, circ2_next_rel.center);
    let coll_soln = geometry::line_circle_intersect(&circ2_line, &Circle::new(rad_tot, circ1_prev.center));
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

pub fn circ_poly_coll<T: Clone, P: Sized + Clone + Poly>(circ_next: &Circle, circ_prev: &Circle, poly_next: &P, poly_prev: &P)
                                -> (CollResults<T>, CollResults<T>) {
    let earliest_corner = circ_poly_coll_corners(circ_next, circ_prev, poly_next, poly_prev);

    let earliest_side = circ_poly_coll_sides(circ_next, circ_prev, poly_next, poly_prev);

    match (earliest_corner, earliest_side) {
        (None, None) => (CollResults::no_collision(), CollResults::no_collision()),
        (Some((c_det, p_det, t)), None) => (CollResults::collided(c_det, t), CollResults::collided(p_det, t)),
        (None, Some((c_det, p_det, t))) => (CollResults::collided(c_det, t), CollResults::collided(p_det, t)),
        (Some((c_det1, p_det1, t1)), Some((c_det2, p_det2, t2))) => {
            if t1 < t2 + EPSILON {
                return (CollResults::collided(c_det1, t1), CollResults::collided(p_det1, t1));
            }
            else {
                return (CollResults::collided(c_det2, t2), CollResults::collided(p_det2, t2));
            }
        }
    }
}

fn circ_poly_coll_corners<P: Poly + Sized + Clone>(circ_next: &Circle, circ_prev: &Circle, poly_next: &P, poly_prev: &P)
                          -> Option<(CollDetails, CollDetails, f64)> {
    // Change frame of reference so that the Circle appears to be fixed
    let circ_shift = circ_next.center - circ_prev.center;
    let poly_next_rel = poly::get_shifted(poly_next, circ_shift * - 1.0);

    let mut collisions: Vec<(usize, f64, Vector2<f64>)> = Vec::new(); //corner index, time, circle collision dir

    for (index, corner_line) in (0..poly_prev.total_sides()).zip(poly_prev.get_corner_lines(&poly_next_rel)) {
        let corner_coll_soln = geometry::line_circle_intersect(&corner_line, &circ_prev);

        if let (true, Some(time)) = (corner_coll_soln.both_positive(), corner_coll_soln.smallest_within_zero_one()) {
            let coll_dir = normalize(&(corner_line.get_point(time) - circ_prev.center));
            collisions.push((index, time, coll_dir));
        }
    }

    collisions.sort_by(|&(_, time1, _), &(_, time2, _)| {time1.partial_cmp(&time2).unwrap_or(Equal) });

    match collisions.iter().cloned().nth(0) {
        None => None,
        Some((index, time, coll_dir)) => {
            let circ_details = CollDetails::Circ(coll_dir);
            let poly_details = CollDetails::ConPoly(ConPolyInfo::CornerInfo(index, coll_dir * -1.0));
            Some((circ_details, poly_details, time))
        }
    }
}

fn circ_poly_coll_sides<P: Poly> (circ_next: &Circle, circ_prev: &Circle, poly_next: &P, poly_prev: &P)
                        -> Option<(CollDetails, CollDetails, f64)> {
    // Side checks next - requires polygon to be stationary
    let poly_shift = poly_next.get_corners()[0] - poly_prev.get_corners()[0];
    let circ_next_rel = circ_next.shifted_by(poly_shift * - 1.0);

    let mut collisions: Vec<(usize, f64, f64)> = Vec::new();

    for ((index, side), normal) in (0..poly_prev.total_sides()).zip(poly_prev.sides()).zip(poly_prev.normals()) {
        let circ_line = circ_prev.get_movement_line(&circ_next_rel).shifted_by(- normal * circ_prev.rad);
        let intersect = geometry::line_line_intersect_2d(&circ_line, &side);

        if let (DualSoln::Two(time, side_pos),
                true,
                true)
            =  (intersect,
                intersect.both_within_zero_one(),
                dot(&circ_line.get_direction(), &normal) < 0.0) {
            collisions.push((index, time, side_pos));
        }
    }

    collisions.sort_by(|&(_, time1, _), &(_, time2, _)| {time1.partial_cmp(&time2).unwrap_or(Equal) });

    match collisions.iter().cloned().nth(0) {
        None => None,
        Some((index, time, side_pos)) => {
            let circ_details = CollDetails::Circ(poly_prev.get_normal(index) * -1.0);
            let poly_details = CollDetails::ConPoly(ConPolyInfo::LineInfo(index, side_pos));
            Some((circ_details, poly_details, time))
        }
    }
}

pub fn poly_poly_coll<T: Clone, P1: Poly + Clone, P2: Poly + Clone>(poly1_next: &P1, poly1_prev: &P1, poly2_next: &P2, poly2_prev: &P2)
                         -> (CollResults<T>, CollResults<T>) {
    let earliest_corner_collision = earliest_corner_collision(poly1_next, poly1_prev, poly2_next, poly2_prev);

    let mut side_collision: Option<(CollDetails, CollDetails, f64)> = None;

    if let Some((results1, results2, time)) = earliest_corner_collision.clone() {
        let poly1_at_collision = poly::get_at_time(poly1_prev, poly1_prev.get_shift(poly1_next), time);
        let poly2_at_collision = poly::get_at_time(poly2_prev, poly2_prev.get_shift(poly2_next), time);
        side_collision = match (results1, results2) {
            (CollDetails::ConPoly(ConPolyInfo::CornerInfo(corner_index, _)), CollDetails::ConPoly(ConPolyInfo::LineInfo(side_index, _)))
                => poly_poly_coll_sides(&poly1_at_collision, &poly2_at_collision, corner_index, side_index, time),
            (CollDetails::ConPoly(ConPolyInfo::LineInfo(side_index, _)), CollDetails::ConPoly(ConPolyInfo::CornerInfo(corner_index, _)))
                => poly_poly_coll_sides(&poly2_at_collision, &poly1_at_collision, corner_index, side_index, time)
                .and_then(|(p2_det, p1_det, time)| {Some((p1_det, p2_det, time))}),
            _ => None
        };
    }

    match side_collision.or(earliest_corner_collision) {
        None => (CollResults::no_collision(), CollResults::no_collision()),
        Some((p1_det, p2_det, time)) => (CollResults::collided(p1_det, time), CollResults::collided(p2_det, time))
    }
}

fn earliest_corner_collision<P1: Poly + Clone, P2: Poly + Clone>(poly1_next: &P1, poly1_prev: &P1, poly2_next: &P2, poly2_prev: &P2)
    -> Option<(CollDetails, CollDetails, f64)> {
    let poly1_corner_collision = poly_poly_coll_corners(poly1_next, poly1_prev, poly2_next, poly2_prev);
    let poly2_corner_collision = poly_poly_coll_corners(poly2_next, poly2_prev, poly1_next, poly1_prev)
        .and_then(|(poly2_details, poly1_details, time)| {Some((poly1_details, poly2_details, time))});

    match (poly1_corner_collision, poly2_corner_collision) {
        (None, None) => None,
        (Some(val), None) => Some(val),
        (None, Some(val)) => Some(val),
        (Some((p1_det1, p2_det1, time1)), Some((p1_det2, p2_det2, time2))) => match time1 < time2 {
            true => Some((p1_det1, p2_det1, time1)),
            false => Some((p1_det2, p2_det2, time2))
        }
    }
}

/// Check collisions of corners of obj1 on sides of obj2
fn poly_poly_coll_corners<P1: Poly + Clone, P2: Poly + Clone>(poly1_next: &P1, poly1_prev: &P1, poly2_next: &P2, poly2_prev: &P2)
                          -> Option<(CollDetails, CollDetails, f64)> {
    // We require poly2 to be stationary
    let poly2_shift = poly2_prev.get_shift(poly2_next);
    let poly1_next_rel = poly::get_shifted(poly1_next, poly2_shift * -1.0);
    let poly1_lines = poly1_prev.get_corner_lines(&poly1_next_rel);

    let earliest_poly1_corner_coll = points_side_coll(&poly1_lines, poly2_prev);

    if let Some((corner_index, side_index, time, side_pos)) = earliest_poly1_corner_coll {
        let poly1_details = CollDetails::ConPoly(ConPolyInfo::CornerInfo(corner_index, poly2_prev.get_normal(side_index) * -1.0));
        let poly2_details = CollDetails::ConPoly(ConPolyInfo::LineInfo(side_index, side_pos));
        return Some((poly1_details, poly2_details, time));
    }
    None
}

/// Check collisions of sides of poly1 on sides of poly2
/// (assuming a corner collision has already occured on corner corner_num of poly1
/// so that we don't have to worry about prev/next)
fn poly_poly_coll_sides(poly1: &Poly, poly2: &Poly, corner_num: usize, side_num: usize, time: f64)
                        -> Option<(CollDetails, CollDetails, f64)> {
    if let (Some((side1, index1, side2, index2)), Some(poly2_side))
        = (poly1.get_adjacent_sides(corner_num), poly2.get_side(side_num)) {
            if line_line_parallel(&side1, &poly2_side) {
                let poly1_details = CollDetails::ConPoly(ConPolyInfo::SideInfo(index1));
                let poly2_details = CollDetails::ConPoly(ConPolyInfo::SideInfo(side_num));
                return Some((poly1_details, poly2_details, time));
            }
            if line_line_parallel(&side2, &poly2_side) {
                let poly1_details = CollDetails::ConPoly(ConPolyInfo::SideInfo(index2));
                let poly2_details = CollDetails::ConPoly(ConPolyInfo::SideInfo(side_num));
                return Some((poly1_details, poly2_details, time));
            }
        }
    None
}

pub fn poly_point_coll<T: Clone>(poly_next: &Poly, poly_prev: &Poly, point_next: &Vector2<f64>, point_prev: &Vector2<f64>)
                         -> (CollResults<T>, CollResults<T>) {
    let poly_shift = poly_prev.get_shift(poly_next);

    let shifted_point_line = Line::new(*point_prev, point_next  - poly_shift);

    if let Some((corner_index, time, side_pos)) = point_side_coll(&shifted_point_line, poly_prev) {
        let poly_details = CollDetails::ConPoly(ConPolyInfo::LineInfo(corner_index, side_pos));
        let point_details = CollDetails::Point(-poly_prev.get_normal(corner_index));

        return (CollResults::collided(poly_details, time), CollResults::collided(point_details, time));
    }
    
    (CollResults::no_collision(), CollResults::no_collision())
}

/// Computes earliest collision of a set of points moving with a (static) polygon
/// Inputs: line_vec -> Vector of lines representing the paths of points
///         poly -> polygon
/// Output: Some(point number, side number, time of collision, side_position) <-> Collision occured
///         None <-> No collision
fn points_side_coll(lines: &Vec<Line>, poly: &Poly) -> Option<(usize, usize, f64, f64)> {

    let mut collisions: Vec<(usize, usize, f64, f64)> = Vec::new();

    for (line_index, line) in (0..lines.len()).zip(lines) {
        if let Some((corner_index, time, side_pos)) = point_side_coll(line, poly) {
            collisions.push((line_index, corner_index, time, side_pos));
        }
    }

    collisions.sort_by(|&(_, _, time1, _), &(_, _, time2, _)| {time1.partial_cmp(&time2).unwrap_or(Equal) });

    collisions.iter().cloned().nth(0)
}

/// Determines when and how the path of a point enters a polygon (outside -> inside)
/// Inputs: line -> represents start and end positions of the point
///         poly -> polygon
/// Output: Some(side number, time of collision, side position) <-> Collision occured
///         None <-> No collision
fn point_side_coll(line: &Line, poly: &Poly) -> Option<(usize, f64, f64)> {
    let mut collisions: Vec<(usize, f64, f64)> = Vec::new();

    for ((index, side), normal) in (0..poly.total_sides()).zip(poly.sides()).zip(poly.normals()) {
        let intersect = geometry::line_line_intersect_2d(&line, &side);

        if let (DualSoln::Two(time, side_pos),
                true,
                true)
            =  (intersect,
                intersect.both_within_zero_one(),
                dot(&line.get_direction(), &normal) < 0.0) {
            collisions.push((index, time, side_pos));
        }
    }

    collisions.sort_by(|&(_, time1, _), &(_, time2, _)| { time1.partial_cmp(&time2).unwrap_or(Equal) });

    collisions.iter().cloned().nth(0)
}

// Checks if two lines are parallel
fn line_line_parallel(line1: &Line, line2: &Line) -> bool {
    let line1_normal = line1.get_normal();
    let line2_dir = line2.get_direction();
    abs(&dot(&line1_normal, &line2_dir)) < EPSILON
}

fn line_line_overlap(line1: &Line, line2: &Line) -> bool {
    let line1_dir = line1.get_diff();
    let line2_normal = line2.get_unnormalized_normal();
    if abs(&dot(&line1_dir, &line2_normal)) < EPSILON {
        return dot(&(line2.end - line1.beg), &line1_dir) > 0.0
            && dot(&(line2.beg - line1.end), &line1_dir) < 0.0;
    }
    false
}
