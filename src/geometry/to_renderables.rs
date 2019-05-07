use crate::rendering::*;

pub trait ToRenderables {
    fn to_renderables(&self, color: Color, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>>;
}