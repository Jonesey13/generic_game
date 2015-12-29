use na::Vec3;
use super::renderables::{Renderable, RenderVertex};
use super::shaders::Shaders;
use glium;
use glium::index::PrimitiveType;
use super::conversion_tools::*;

#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
    pub pos: Vec3<f64>,
}

impl Renderable for Circle {
    fn get_shaders(&self) -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("circle.vs"),
            include_str!("circle.tcs"),
            include_str!("circle.tes"),
            include_str!("circle.fs"))
    }

    fn get_vertex(&self) -> RenderVertex {
        RenderVertex::Circ(self.clone().into())
    }

    fn get_primitive_type(&self) -> PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CircleVertex {
    pub radius: f32,
    pub pos: [f32; 3],
}

implement_vertex!(CircleVertex, radius, pos);

impl From<Circle> for CircleVertex {
    fn from(rect: Circle) -> Self {
        CircleVertex {
            radius: rect.radius as f32,
            pos: vec3_64_to_32(*rect.pos.as_ref()),
        }
    }
}
