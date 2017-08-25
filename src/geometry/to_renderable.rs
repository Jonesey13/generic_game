use rendering::Renderable;
use na::Vector4;

pub trait ToRenderable {
    fn to_renderable(&self, colour: Vector4<f64>, depth: f64, fixed: bool) -> Box<Renderable>;
}