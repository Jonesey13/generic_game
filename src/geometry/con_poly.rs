use na::{Vector1, Vector2, Vector3, Vector4, Rotation2, norm, dot};
use num::Zero;
use geometry::average_vec2;
use geometry::vect::get_normal_2d;
use geometry::Poly;
use std::f64::consts;
use std::iter::{Repeat, repeat};
use geometry::{TwoDTransformable, ToRenderable, Point, Line};
use rendering;
use rendering::{Renderable, Polygon};
use collision::{CollObj, CollisionObjectState};
use collision;

/// A (formally convex) polygon for collision detection
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
        for vector in corners.iter_mut() {
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

    pub fn get_average(&self) -> Vector2<f64> {
        self.corners.iter().fold(Vector2::zero(), |acc, v| {acc + v}) / self.corners.len() as f64
    }
}

impl TwoDTransformable for ConPoly {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        for corner in &mut self.corners {
            *corner = *corner + shift
        }
    }

    fn rotate(&mut self, rot_angle: f64) {
        let rot_mat = Rotation2::new(rot_angle);
        let center = self.get_average();
        for corner in &mut self.corners {
            *corner = rot_mat * (*corner - center) + center;
        }
    }
}

impl ToRenderable for ConPoly {
    fn to_renderable(&self, colour: Vector4<f64>, depth: f64, fixed: bool) -> Box<Renderable> {
        Box::new(Polygon::new_regular(self.corners.clone(), self.get_average(), Vector3::new(0.0, 0.0, depth), colour, fixed))
    }
}

impl CollObj for ConPoly {
    fn get_object_pair(&self, other: &Self) -> CollisionObjectState {
        CollisionObjectState::ConPoly(self.clone(), other.clone())
    }

    fn render_collision_details(&self, collision_details: collision::CollisionObjectDetails, colour: Vector4<f64>, depth: f64, fixed: bool) 
    -> Vec<Box<Renderable>> {
        let coll_location = match collision_details {
            collision::CollisionObjectDetails::ConPoly(loc) => loc,
            _ => return vec![]
        };

        let location_renderable: Box<ToRenderable> = match coll_location {
            collision::ConPolyInfo::LineInfo(side, pos) => Box::new(Point::new(self.get_side(side).unwrap().get_point(pos))),
            collision::ConPolyInfo::CornerInfo(num, _) => Box::new(Point::new(self.get_corners()[num])),
            collision::ConPolyInfo::SideInfo(side) => Box::new(self.get_side(side).unwrap()),
        };

        let (coll_pos, coll_dir) = match coll_location {
            collision::ConPolyInfo::LineInfo(side, pos) => (self.get_side(side).unwrap().get_point(pos), self.get_normal(side)),
            collision::ConPolyInfo::CornerInfo(num, dir) => (self.get_corners()[num], dir),
            collision::ConPolyInfo::SideInfo(side) => (self.get_side(side).unwrap().get_point(0.5), self.get_normal(side)),
        };

        let direction_renderable: Box<rendering::Renderable> = Box::new(
            rendering::Arrow::new_for_coll_test(
                    coll_pos,
                    coll_dir,
                    colour,
                    depth,
                    fixed
            )
        );

        vec![
            location_renderable.to_renderable(colour, depth, fixed),
            direction_renderable
        ]
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
