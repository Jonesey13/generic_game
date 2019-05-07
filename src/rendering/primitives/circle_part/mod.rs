use crate::rendering::*;
use crate::geometry::*;
use glium;

#[derive(Copy, Clone, Debug)]
pub struct CirclePart {
    pub radial_dim: Point,
    pub angular_dim: Point,
    pub pos: Point3,
    pub color: Color,
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
            pos: circ.pos.into(),
            color: circ.color.get_array_f32(),
            fixed_pos: circ.fixed as u32
        }
    }
}
