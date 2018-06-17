use na::{Vector3, Vector4, convert};
use num::Zero;
use rendering::primitives::StandardPrimitive;
use rendering::render_by_shaders::GliumStandardPrimitive;
use rendering::shaders::Shaders;
use ::geometry::*;
use glium;

#[derive(Copy, Clone)]
pub struct CirclePart {
    pub radial_dim: Point,
    pub angular_dim: Point,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl GliumStandardPrimitive for CirclePart {
    type Vertex = CircleVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("circle_part.vs"),
            include_str!("circle_part.tcs"),
            include_str!("circle_part.tes"),
            include_str!("circle_part.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CircleVertex {
    pub radial_dim: [f32; 2],
    pub angular_dim: [f32; 2],
    pub pos: [f32; 3],
    pub color: [f32; 4],
    pub fixed_pos: u32
}

implement_vertex!(CircleVertex, radial_dim, angular_dim, pos, color, fixed_pos);

impl From<CirclePart> for CircleVertex {
    fn from(circ: CirclePart) -> Self {
        CircleVertex {
            radial_dim: circ.radial_dim.into(),
            angular_dim: circ.angular_dim.into(),
            pos: *convert::<_, Vector3<f32>>(circ.pos).as_ref(),
            color: *convert::<_, Vector4<f32>>(circ.color).as_ref(),
            fixed_pos: circ.fixed as u32
        }
    }
}
