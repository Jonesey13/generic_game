use super::CollisionObjectState;
use super::CollisionObjectDetails;
use rendering::Renderable;
use na::{Vector2, Vector4};
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
    Point(Vector2<f64>)
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

    pub fn render_collision_details(&self, object_details: CollisionObjectDetails, colour: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<Renderable>> {
        match (self, object_details) {
            (&CollisionObject::None, CollisionObjectDetails::None) => vec![],
            (&CollisionObject::Circ(ref circle), CollisionObjectDetails::Circ(dir)) => circle.render_collision_details(dir, colour, depth, fixed),
            (&CollisionObject::ConPoly(ref con_poly), CollisionObjectDetails::ConPoly(ref poly_info)) => con_poly.render_collision_details(poly_info.clone(), colour, depth, fixed),
            (&CollisionObject::Line(line), CollisionObjectDetails::Line(line_info)) => line.render_collision_details(line_info, colour, depth, fixed),
            (&CollisionObject::Point(point), CollisionObjectDetails::Point(dir)) => Point::new(point).render_collision_details(dir, colour, depth, fixed),
            _ => vec![]
        }
    }

    pub fn render(&self, colour: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<Renderable>> {
        match self {
            &CollisionObject::None => vec![],
            &CollisionObject::Circ(ref circle) => circle.to_renderables(colour, depth, fixed),
            &CollisionObject::ConPoly(ref con_poly) => con_poly.to_renderables(colour, depth, fixed),
            &CollisionObject::Line(line) => line.to_renderables(colour, depth, fixed),
            &CollisionObject::Point(point) => Point::new(point).to_renderables(colour, depth, fixed),
        }
    }
}