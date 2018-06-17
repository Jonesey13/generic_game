pub mod text_buffer;
mod plain_text;
pub use self::text_buffer::TextBuffer;
pub use self::plain_text::{PlainText, TextAlign};

use na::{Vector4, Matrix2};
use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::Surface;
use std::borrow::Cow;
use games::view_details;
use rendering::primitives::StandardPrimitive;
use rendering::render_by_shaders::GliumStandardPrimitive;
use rendering::shaders::Shaders;

pub trait RenderText {
    type TextVert: glium::vertex::Vertex;

    fn get_shaders() -> Shaders;

    fn get_vertices(
        &self,
        glyph_pos_data: Vec<(Rect<f32>, Rect<i32>)>
    ) -> Vec<Self::TextVert>;

    fn get_content(&self) -> &String;
}

impl<T: RenderText> GliumStandardPrimitive for T {
    type Vertex = T::TextVert;

    fn get_shaders() -> Shaders {
        T::get_shaders()
    }

    fn get_vertex(self) -> Vec<Self::Vertex> {panic!("Should be using RenderText's get_vertices()!")}
}

