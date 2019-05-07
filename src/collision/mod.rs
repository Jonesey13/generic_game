use std::cell::RefCell;
use std::iter::{Repeat, repeat};
use std::cmp::Ordering::{Equal, Less, Greater};
use crate::geometry::*;
use crate::debug::*;
pub mod collision_object;
pub mod collidable_wrapper;
pub mod collision_test_game;
pub mod collision_logic;
pub mod collision_object_results;
pub mod collision_results;
pub mod collision_object_details;
pub mod collision_details;
pub mod collider;
pub mod collision_data_type;

pub use self::collision_object::{CollisionObject, ToCollisionObjects};
pub use self::collision_test_game::CollisionTestGame;
pub use self::collision_test_game::builder::CollisionTestBuilder;
pub use self::collision_object_results::CollisionObjectResults;
pub use self::collision_results::CollisionResults;
pub use self::collision_object_details::{CollisionObjectDetails, ConPolyInfo, LineInfo, LineSide};
pub use self::collision_details::CollisionDetails;
pub use self::collider::Collider;
pub use self::collision_data_type::CollisionDataType;

pub trait Collidable {
    type Data: Clone;
    fn get_collision_objects(&self) -> Vec<CollisionObjectState> { vec![] }
    fn get_earliest_collision_results(&self) -> Option<CollisionResults<Self::Data>>;
    fn add_collision_results(&mut self, _: CollisionResults<Self::Data>);
    fn get_own_collision_data(&self) -> Self::Data;
    fn resolve_collision_results(&mut self) {}

    fn get_earliest_collision_time(&mut self) -> Option<f64> {
        self.get_earliest_collision_results()
        .and_then(|res| {Some(res.details.time)})
    }
    fn has_collided(&self) -> bool { self.get_earliest_collision_results().is_some() }
    fn get_earliest_collision_details(&self) -> Option<CollisionDetails> { 
        self.get_earliest_collision_results()
        .and_then(|res| {Some(res.details)})
    }
    fn get_earliest_collision_data(&self) -> Option<Self::Data> { 
        self.get_earliest_collision_results()
        .and_then(|res| {Some(res.data)})
    }
}

#[derive(Clone)]
pub enum CollisionObjectState {
    None,
    Circ(Circle, Circle),
    ConPoly(ConPoly, ConPoly),
    Line(Line, Line),
    Point(Point, Point)
}
