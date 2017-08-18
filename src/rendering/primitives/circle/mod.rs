use na::{Vector3, Vector4, convert};
use num::Zero;
use rendering::primitives::Primitive;
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;
use glium;

#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>
}

impl GliumPrimitive for Circle {
    type Vertex = CircleVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("circle.vs"),
            include_str!("circle.tcs"),
            include_str!("circle.tes"),
            include_str!("circle.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
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
            pos: *convert::<_, Vector3<f32>>(rect.pos).as_ref(),
            color: *convert::<_, Vector4<f32>>(rect.color).as_ref()
        }
    }
}
