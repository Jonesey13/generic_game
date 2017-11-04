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
pub mod coll_obj;
pub mod coll_obj_wrapper;
pub mod collision_test_game;
pub mod collision_logic;
pub mod collision_results;
pub mod collision_details;
pub mod collider;

pub use self::coll_obj::CollObj;
pub use self::collision_test_game::CollisionTestGame;
pub use self::collision_test_game::builder::CollisionTestBuilder;
pub use self::collision_results::CollResults;
pub use self::collision_details::{CollDetails, ConPolyInfo, LineInfo, LineSide};
pub use self::collider::Collider;

pub trait Collidable {
    type Data: Clone;
    fn get_collision_object(&self) -> CollObjPair { CollObjPair::None }
    fn get_collision_results(&self) -> CollResults<Self::Data>;
    fn set_collision_results(&mut self, CollResults<Self::Data>);
    fn get_collision_time(&mut self) -> Option<f64> {self.get_collision_results().time}
    fn has_collided(&self) -> bool { self.get_collision_results().collided }
    fn get_collision_details(&self) -> Option<CollDetails> { self.get_collision_results().details }
    fn get_collision_data(&self) -> Self::Data;
}

pub enum CollObjPair {
    None,
    Circ(Circle, Circle),
    ConPoly(ConPoly, ConPoly),
    Line(Line, Line),
    Point(Vector2<f64>, Vector2<f64>)
}

