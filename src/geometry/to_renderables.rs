use rendering::StandardRenderable;
use na::Vector4;

pub trait ToRenderables {
    fn to_renderables(&self, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>>;
}