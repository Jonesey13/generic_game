use std::f64::consts;
use std::iter::{Repeat, repeat};
use geometry::*;
use rendering::*;
use collision::{CollisionObject, CollisionObjectState, ToCollisionObjects};
use collision;

/// A (formally convex) polygon for collision detection
#[derive(Clone, Debug)]
pub struct ConPoly {
    pub corners: Vec<Point>, // defined anticlockwise
}

impl Poly for ConPoly {
    fn get_corners(&self) -> Vec<Point> {
        self.corners.clone()
    }

    fn set_corners(&mut self, corners: Vec<Point>) {
        self.corners = corners;
    }
}

impl ConPoly {
    pub fn new_from_rect(rect: Rectangle) -> ConPoly {
        let xshift = Point::new(rect.length, 0.0);
        let yshift = Point::new(0.0, rect.height);
        let bottom_left = Point::zero();
        let bottom_right = bottom_left + xshift;
        let top_right = bottom_left + xshift + yshift;
        let top_left = bottom_left + yshift;
        let mut corners = vec![bottom_left, bottom_right, top_right, top_left];

        let average = average_vec2(corners.clone());
        for vector in corners.iter_mut() {
            *vector = rect.rot * (*vector - average) + rect.pos;
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

    pub fn new (corners: Vec<Point>) -> ConPoly {
        ConPoly {
            corners: corners
        }
    }

    pub fn interior_point_check(&self, point: Point) -> Option<Point> {
        let mut outside = false;
        let mut correction = Point::zero();
        
        for (&side, &normal) in self.sides().iter().zip(self.normals().iter()) {
            let overlap = (point - side.beg).dot(&normal);
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

    pub fn get_average(&self) -> Point {
        (1.0 / self.corners.len() as f64) * self.corners.iter().fold(Point::zero(), |acc, v| {acc + v})
    }
}

impl TwoDTransformable for ConPoly {
    fn shift_by(&mut self, shift: Point) {
        for corner in &mut self.corners {
            *corner = *corner + shift
        }
    }

    fn rotate(&mut self, rot_angle: f64) {
        let rot_mat = Rotation::new(rot_angle);
        let center = self.get_average();
        for corner in &mut self.corners {
            *corner = rot_mat * (*corner - center) + center;
        }
    }
}

impl ToRenderables for ConPoly {
    fn to_renderables(&self, color: Color, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>> {
        vec![
            Box::new(Polygon::new_regular(self.corners.clone(), self.get_average(), Point3::new(0.0, 0.0, depth), color, fixed))
        ]
    }
}

impl ConPoly {
    pub fn render_collision_details(&self, coll_location: collision::ConPolyInfo, color: Color, depth: f64, fixed: bool) 
    -> Vec<Box<StandardRenderable>> {
        let location_renderable: Box<ToRenderables> = match coll_location {
            collision::ConPolyInfo::LineInfo(side, pos) => Box::new(self.get_side(side).unwrap().get_point(pos)),
            collision::ConPolyInfo::CornerInfo(num, _) => Box::new(self.get_corners()[num]),
            collision::ConPolyInfo::SideInfo(side) => Box::new(self.get_side(side).unwrap()),
        };

        let (coll_pos, coll_dir) = match coll_location {
            collision::ConPolyInfo::LineInfo(side, pos) => (self.get_side(side).unwrap().get_point(pos), self.get_normal(side)),
            collision::ConPolyInfo::CornerInfo(num, dir) => (self.get_corners()[num], dir),
            collision::ConPolyInfo::SideInfo(side) => (self.get_side(side).unwrap().get_point(0.5), self.get_normal(side)),
        };

        let direction_renderable: Box<StandardRenderable> = Box::new(
            Arrow::new_for_coll_test(
                    coll_pos,
                    coll_dir,
                    color,
                    depth,
                    fixed
            )
        );

        let mut renderables = location_renderable.to_renderables(color, depth, fixed);
        renderables.push(direction_renderable);
        renderables
    }    
}

impl ToCollisionObjects for ConPoly {
    fn to_collision_objects(&self) -> Vec<CollisionObject> {
        vec![
            CollisionObject::ConPoly(self.clone())
        ]
    }
}

#[test]
fn point_inside_poly_test() {
    let corners = vec![
        Point::new(-1.0, -1.0),
        Point::new(1.0, -1.0),
        Point::new(1.0, 1.0),
        Point::new(-1.0, 1.0)
    ];
    
    let test_poly = ConPoly::new(corners);

    let test_point = Point::new(1.2,0.2);

    let overlap = test_poly.interior_point_check(test_point);

    assert!(overlap.is_some());

    assert!((overlap.unwrap() - Point::new(-0.2, 0.0)).norm() < 0.00001);
}

impl From<Rectangle> for ConPoly {
    fn from(rect: Rectangle) -> Self {
        ConPoly::new_from_rect(rect)
    }
}