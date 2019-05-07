use crate::rendering::*;
use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct RectanglePrimitive {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation,
    pub pos: Point3,
    pub color: Color,
    pub fixed: bool
}

impl RectanglePrimitive {
    pub fn new_regular(
            length: f64, 
            height: f64, 
            pos: Point3, 
            color: Color,
            fixed: bool
        ) -> Self {
        Self {
            length,
            height,
            rot: Rotation::new(0.0),
            pos,
            color,
            fixed
        }
    }

    pub fn new_with_rotation(
            length: f64, 
            height: f64, 
            pos: Point3,
            rotation: Rotation,
            color: Color,
            fixed: bool
        ) -> Self {
        Self {
            length,
            height,
            rot: rotation,
            pos,
            color,
            fixed
        }
    }
}

impl GliumStandardPrimitive for RectanglePrimitive {
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

impl From<RectanglePrimitive> for RectangleVertex {
    fn from(rect: RectanglePrimitive) -> Self {
        RectangleVertex {
            length: rect.length as f32,
            height: rect.height as f32,
            rot: rect.rot.get_matrix_f32(),
            pos: rect.pos.into(),
            color: rect.color.get_array_f32(),
            fixed_pos: rect.fixed as u32
        }
    }
}
