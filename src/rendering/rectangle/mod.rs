use na::{Vector1, Vector3, Vector4, Rotation2};
use num::Zero;
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::GliumRenderable;
use super::shaders::Shaders;
use super::conversion_tools::*;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation2<f64>,  /// anti-clockwise angle w.r.t. positive z-axis
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl Rectangle {
    pub fn new_regular(
            length: f64, 
            height: f64, 
            pos: Vector3<f64>, 
            color: Vector4<f64>
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation2::new(0.0),
            pos,
            color,
            fixed: false
        }
    }

    pub fn new_regular_fixed(
            length: f64, 
            height: f64, 
            pos: Vector3<f64>, 
            color: Vector4<f64>
        ) -> Rectangle {
        Rectangle {
            length,
            height,
            rot: Rotation2::new(0.0),
            pos,
            color,
            fixed: true
        }
    }
}

impl Renderable for Rectangle {
    fn get_type(&self) -> RenderType { RenderType::Rect(self.clone()) }
}

impl GliumRenderable for Rectangle {
    type Vertex = RectangleVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("rectangle.vs"),
            include_str!("rectangle.ges"),
            include_str!("rectangle.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }
}

#[derive(Copy, Clone, Debug)]
pub struct RectangleVertex {
    pub length: f32,
    pub height: f32,
    pub rot: [[f32; 2]; 2],
    pub pos: [f32; 3],
    pub color: [f32; 4],
    pub fixed_pos: u32
}

implement_vertex!(RectangleVertex, length, height, rot, pos, color, fixed_pos);

impl From<Rectangle> for RectangleVertex {
    fn from(rect: Rectangle) -> Self {
        RectangleVertex {
            length: rect.length as f32,
            height: rect.height as f32,
            rot: mat2_64_to_32(*rect.rot.matrix().as_ref()),
            pos: vec3_64_to_32(*rect.pos.as_ref()),
            color: vec4_64_to_32(*rect.color.as_ref()),
            fixed_pos: rect.fixed as u32
        }
    }
}
