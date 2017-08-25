use super::CollObjPair;
use super::CollDetails;
use rendering::Renderable;
use na::Vector4;

pub trait CollObj {
    fn get_object_pair(&self, other: &Self) -> CollObjPair;
    
    fn render_collision_details(&self, CollDetails, colour: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<Renderable>> {vec![]}
}