use std::cell::RefCell;
use std::iter::{Repeat, repeat};
use std::cmp::Ordering::{Equal, Less, Greater};
use geometry;
use geometry::{circle, con_poly, line, HasAngle, DualSoln, Poly, poly};
use geometry::circle::Circle;
use geometry::con_poly::ConPoly;
use geometry::line::Line;
use na::{normalize, Vector2, dot, abs};
use debug::*;
pub mod collision_object;
pub mod collision_object_wrapper;
pub mod collision_test_game;
pub mod collision_logic;
pub mod collision_object_results;
pub mod collision_results;
pub mod collision_object_details;
pub mod collision_details;
pub mod collider;

pub use self::collision_object::CollObj;
pub use self::collision_test_game::CollisionTestGame;
pub use self::collision_test_game::builder::CollisionTestBuilder;
pub use self::collision_object_results::CollisionObjectResults;
pub use self::collision_results::CollisionResults;
pub use self::collision_object_details::{CollisionObjectDetails, ConPolyInfo, LineInfo, LineSide};
pub use self::collision_details::CollisionDetails;
pub use self::collider::Collider;

pub trait Collidable {
    type Data: Clone;
    fn get_collision_objects(&self) -> Vec<CollisionObjectState> { vec![] }
    fn get_collision_object_results(&self) -> CollisionObjectResults<Self::Data>;
    fn set_collision_object_results(&mut self, CollisionObjectResults<Self::Data>);
    fn get_collision_time(&mut self) -> Option<f64> {self.get_collision_object_results().time}
    fn has_collided(&self) -> bool { self.get_collision_object_results().collided }
    fn get_collision_details(&self) -> Option<CollisionObjectDetails> { self.get_collision_object_results().details }
    fn get_collision_data(&self) -> Self::Data;
}

#[derive(Clone)]
pub enum CollisionObjectState {
    None,
    Circ(Circle, Circle),
    ConPoly(ConPoly, ConPoly),
    Line(Line, Line),
    Point(Vector2<f64>, Vector2<f64>)
}

