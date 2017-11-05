use super::CollisionObjectState;
use super::CollisionObjectDetails;
use rendering::Renderable;
use na::Vector4;

pub trait CollObj {
    fn get_object_pair(&self, other: &Self) -> CollisionObjectState;
    
    #[allow(unused_variables)]
    fn render_collision_details(&self, CollisionObjectDetails, colour: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<Renderable>> {vec![]}
}