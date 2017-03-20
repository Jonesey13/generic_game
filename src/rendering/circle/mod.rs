use na::{Vector3, Vector4};
use num::Zero;
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::RenderByShaders;
use super::shaders::Shaders;
use glium;
use glium::index::PrimitiveType;
use super::conversion_tools::*;

#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>
}

impl Renderable for Circle {
    fn get_type(&self) -> RenderType { RenderType::Circ(self.clone()) }
}

impl RenderByShaders for Circle {
    type Vertex = CircleVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("circle.vs"),
            include_str!("circle.tcs"),
            include_str!("circle.tes"),
            include_str!("circle.fs"))
    }

    fn get_vertex(&self) -> Self::Vertex { self.clone().into() }

    fn get_primitive_type() -> PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CircleVertex {
    pub radius: f32,
    pub pos: [f32; 3],
    pub color: [f32; 4]
}

implement_vertex!(CircleVertex, radius, pos, color);

impl From<Circle> for CircleVertex {
    fn from(rect: Circle) -> Self {
        CircleVertex {
            radius: rect.radius as f32,
            pos: vec3_64_to_32(*rect.pos.as_ref()),
            color: vec4_64_to_32(*rect.color.as_ref())
        }
    }
}
