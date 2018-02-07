use na::{Vector1, Vector2, Vector3, Vector4, Rotation2, Matrix2, convert};
use num::Zero;
use rendering::primitives::Primitive;
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;

#[derive(Copy, Clone)]
pub struct TextureRect {
    pub length: f64,  /// x-axis
    pub height: f64,  /// y-axis
    pub rot: Rotation2<f64>,
    pub pos: Vector3<f64>,
    pub texture_corner: Vector3<f64>,
    pub texture_dim: Vector2<f64>,
    pub fixed: bool
}

impl TextureRect {
    pub fn new_regular(
            length: f64, 
            height: f64, 
            pos: Vector3<f64>, 
            texture_corner: Vector3<f64>,
            texture_dim: Vector2<f64>,
            fixed: bool
        ) -> TextureRect {
        TextureRect {
            length,
            height,
            rot: Rotation2::new(0.0),
            pos,
            texture_corner,
            texture_dim,
            fixed
        }
    }

    pub fn new_with_rotation(
            length: f64, 
            height: f64, 
            pos: Vector3<f64>,
            rotation: Rotation2<f64>,
            texture_corner: Vector3<f64>,
            texture_dim: Vector2<f64>,
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

impl GliumPrimitive for TextureRect {
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
            rot: *convert::<_, Matrix2<f32>>(*rect.rot.matrix()).as_ref(),
            pos: *convert::<_, Vector3<f32>>(rect.pos).as_ref(),
            texture_corner: *convert::<_, Vector3<f32>>(rect.texture_corner).as_ref(),
            texture_dim: *convert::<_, Vector2<f32>>(rect.texture_dim).as_ref(),
            fixed_pos: rect.fixed as u32
        }
    }
}
