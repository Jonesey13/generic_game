use na::{Vec3, Rot2};
use super::renderables::{Renderable, RenderVertex};
use super::shaders::Shaders;
use super::conversion_tools::*;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rot2<f64>,  /// anti-clockwise angle w.r.t. positive z-axis
    pub pos: Vec3<f64>,
}

impl Renderable for Rectangle {
    fn get_shaders(&self) -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("rectangle.vs"),
            include_str!("rectangle.ges"),
            include_str!("rectangle.fs"))
    }

    fn get_vertex(&self) -> RenderVertex {
        RenderVertex::Rect(self.clone().into())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RectangleVertex {
    pub length: f32,
    pub height: f32,
    pub rot: [[f32; 2]; 2],
    pub pos: [f32; 3],
}

implement_vertex!(RectangleVertex, length, height, rot, pos);

impl From<Rectangle> for RectangleVertex {
    fn from(rect: Rectangle) -> Self {
        RectangleVertex {
            length: rect.length as f32,
            height: rect.height as f32,
            rot: mat2_64_to_32(*rect.rot.submat().as_ref()),
            pos: vec3_64_to_32(*rect.pos.as_ref()),
        }
    }
}
