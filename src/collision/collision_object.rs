use super::CollisionObjectState;
use super::CollisionObjectDetails;
use rendering::StandardRenderable;
use na::{Vector4};
use geometry::{Circle, ConPoly, Line, Point, ToRenderables};

pub trait ToCollisionObjects {
    fn to_collision_objects(&self) -> Vec<CollisionObject>;
}

#[derive(Clone)]
pub enum CollisionObject {
    None,
    Circ(Circle),
    ConPoly(ConPoly),
    Line(Line),
    Point(Point)
}

impl CollisionObject {
    pub fn build_state(self, other: Self) -> CollisionObjectState {
        match (self, other) {
            (CollisionObject::None, CollisionObject::None) => CollisionObjectState::None,
            (CollisionObject::Circ(circ1), CollisionObject::Circ(circ2)) => CollisionObjectState::Circ(circ1, circ2),
            (CollisionObject::ConPoly(con_poly1), CollisionObject::ConPoly(con_poly2)) => CollisionObjectState::ConPoly(con_poly1, con_poly2),
            (CollisionObject::Line(line1), CollisionObject::Line(line2)) => CollisionObjectState::Line(line1, line2),
            (CollisionObject::Point(point1), CollisionObject::Point(point2)) => CollisionObjectState::Point(point1, point2),
            _ => panic!("Collision Objects Do Not Match: Cannot build state!")
        }
    }

    pub fn render_collision_details(&self, object_details: CollisionObjectDetails, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>> {
        match (self, object_details) {
            (&CollisionObject::None, CollisionObjectDetails::None) => vec![],
            (&CollisionObject::Circ(ref circle), CollisionObjectDetails::Circ(dir)) => circle.render_collision_details(dir, color, depth, fixed),
            (&CollisionObject::ConPoly(ref con_poly), CollisionObjectDetails::ConPoly(ref poly_info)) => con_poly.render_collision_details(poly_info.clone(), color, depth, fixed),
            (&CollisionObject::Line(line), CollisionObjectDetails::Line(line_info)) => line.render_collision_details(line_info, color, depth, fixed),
            (&CollisionObject::Point(point), CollisionObjectDetails::Point(dir)) => point.render_collision_details(dir, color, depth, fixed),
            _ => vec![]
        }
    }

    pub fn render(&self, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>> {
        match self {
            &CollisionObject::None => vec![],
            &CollisionObject::Circ(ref circle) => circle.to_renderables(color, depth, fixed),
            &CollisionObject::ConPoly(ref con_poly) => con_poly.to_renderables(color, depth, fixed),
            &CollisionObject::Line(line) => line.to_renderables(color, depth, fixed),
            &CollisionObject::Point(point) => point.to_renderables(color, depth, fixed),
        }
    }
}