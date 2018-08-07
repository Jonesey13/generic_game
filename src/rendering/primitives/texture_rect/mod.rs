use ::rendering::*;
use ::geometry::*;

#[derive(Copy, Clone)]
pub struct TextureRect {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation,
    pub pos: Point3,
    pub texture_corner: Point3,
    pub texture_dim: Point,
    pub fixed: bool
}

impl TextureRect {
    pub fn new_regular(
            length: f64, 
            height: f64, 
            pos: Point3, 
            texture_corner: Point3,
            texture_dim: Point,
            fixed: bool
        ) -> TextureRect {
        TextureRect {
            length,
            height,
            rot: Rotation::new(0.0),
            pos,
            texture_corner,
            texture_dim,
            fixed
        }
    }

    pub fn new_with_rotation(
            length: f64, 
            height: f64, 
            pos: Point3,
            rotation: Rotation,
            texture_corner: Point3,
            texture_dim: Point,
            fixed: bool
        ) -> TextureRect {
        TextureRect {
            length,
            height,
            rot: rotation,
            pos,
            texture_corner,
            texture_dim,
            fixed
        }
    }
}

impl GliumStandardPrimitive for TextureRect {
    type Vertex = TextureRectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("texture_rect.vs"),
            include_str!("texture_rect.ges"),
            include_str!("texture_rect.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }
}

#[derive(Copy, Clone, Debug)]
pub struct TextureRectVertex {
    pub length: f32,
    pub height: f32,
    pub rot: [[f32; 2]; 2],
    pub pos: [f32; 3],
    pub texture_corner: [f32; 3],
    pub texture_dim: [f32; 2],
    pub fixed_pos: u32
}

implement_vertex!(TextureRectVertex, length, height, rot, pos, texture_corner, texture_dim, fixed_pos);

impl From<TextureRect> for TextureRectVertex {
    fn from(rect: TextureRect) -> Self {
        TextureRectVertex {
            length: rect.length as f32,
            height: rect.height as f32,
            rot: rect.rot.get_matrix_f32(),
            pos: rect.pos.into(),
            texture_corner: rect.texture_corner.into(),
            texture_dim: rect.texture_dim.into(),
            fixed_pos: rect.fixed as u32
        }
    }
}
