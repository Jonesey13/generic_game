pub mod text_buffer;
pub use self::text_buffer::TextBuffer;

use na::{Vector2, Vector4, Matrix2};
use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::Surface;
use std::borrow::Cow;
use super::shaders;
use super::conversion_tools::mat2_64_to_32;
use super::renderables::{Renderable, RenderType};
use games::view_details;

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vector2<f64>, //Bottom Left
    pub scale: Vector2<f64>, // Applied First
    pub transform: Matrix2<f64>, //Applied Second
    pub color: Vector4<f64>
}

pub trait RenderText<'a> {
    fn get_shaders() -> super::shaders::Shaders;

    fn get_vertices(
        &self,
        glyph_pos_data: Vec<(Rect<f32>, Rect<i32>)>,
        glyph_scale: Scale
    ) -> Vec<TextVertex>;

    fn get_content(&self) -> &String;
}

impl<'a> RenderText<'a> for PlainText {
    fn get_vertices(
        &self,
        glyph_pos_data: Vec<(Rect<f32>, Rect<i32>)>,
        glyph_scale: Scale
    ) -> Vec<TextVertex>
    {
        let color = [self.color.x as f32,
                     self.color.y as f32,
                     self.color.z as f32,
                     self.color.w as f32];
        let glyph_positions: Vec<[f32; 2]> = glyph_pos_data
            .iter()
            .map(|&(_, screen_rect)| {
                [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                 (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0]
                }).collect();

        let mut average_glyph_pos: [f32; 2] = glyph_positions
            .iter()
            .fold([0.0, 0.0], |acc, rect| 
                [acc[0] + rect[0], acc[1] + rect[1]]
            );
        average_glyph_pos = [average_glyph_pos[0] / (glyph_positions.len() as f32),
                             average_glyph_pos[1] / (glyph_positions.len() as f32)];
        
        let global_pos = [self.position.x as f32 ,self.position.y as f32];
        glyph_pos_data.iter().map(|&(uv_rect, screen_rect)| {
            let actual_length = screen_rect.max.x - screen_rect.min.x;
            let actual_height = screen_rect.max.y - screen_rect.min.y;
            let screen_rect_pos = [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                                   (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0];
            let corrected_screen_rect_pos = [screen_rect_pos[0] - average_glyph_pos[0],
                                             screen_rect_pos[1] - average_glyph_pos[1]];
            let text_rect_width_clip = (uv_rect.max.x - uv_rect.min.x) * 0.00;
            let text_rect_height_clip = (uv_rect.max.y - uv_rect.min.y) * 0.00;
            
            TextVertex {
                length: actual_length as f32,
                height: actual_height as f32,
                local_position: [corrected_screen_rect_pos[0], corrected_screen_rect_pos[1]],
                position: global_pos,
                tex_coords_min: [uv_rect.min.x + text_rect_width_clip, uv_rect.min.y + text_rect_height_clip],
                tex_coords_max: [uv_rect.max.x - text_rect_width_clip, uv_rect.max.y - text_rect_height_clip],
                scale: [self.scale.x as f32 * 100.0 / glyph_scale.x, self.scale.y as f32 * 100.0/ glyph_scale.y],
                transform: mat2_64_to_32(*self.transform.as_ref()),
                colour: color,
                }
            } 
        ).collect()
    }
    
    // fn get_vertices(
    //     &self,
    //     glyphs: &Vec<PositionedGlyph<'a>>,
    //     cache: &rusttype::gpu_cache::Cache,
    //     glyph_scale: Scale
    // ) -> Vec<TextVertex>
    // {
    //     let color = [self.color.x as f32,
    //                  self.color.y as f32,
    //                  self.color.z as f32,
    //                  self.color.w as f32];
    //     let glyph_positions: Vec<[f32; 2]> = glyphs
    //         .iter()
    //         .filter_map(|g| {
    //             if let Ok(Some((_, screen_rect))) = cache.rect_for(0, g) {
    //                 Some([(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
    //                       (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0])
    //             } else {
    //                 None
    //             }}).collect();

    //     let mut average_glyph_pos: [f32; 2] = glyph_positions
    //         .iter()
    //         .fold([0.0, 0.0], |acc, rect| 
    //             [acc[0] + rect[0], acc[1] + rect[1]]
    //         );
    //     average_glyph_pos = [average_glyph_pos[0] / (glyph_positions.len() as f32),
    //                          average_glyph_pos[1] / (glyph_positions.len() as f32)];
        
    //     let global_pos = [self.position.x as f32 ,self.position.y as f32];
    //     glyphs.iter().filter_map(|g| {
    //         if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
    //             let actual_length = screen_rect.max.x - screen_rect.min.x;
    //             let actual_height = screen_rect.max.y - screen_rect.min.y;
    //             let screen_rect_pos = [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
    //                                    (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0];
    //             let corrected_screen_rect_pos = [screen_rect_pos[0] - average_glyph_pos[0],
    //                                              screen_rect_pos[1] - average_glyph_pos[1]];
    //             let text_rect_width_clip = (uv_rect.max.x - uv_rect.min.x) * 0.00;
    //             let text_rect_height_clip = (uv_rect.max.y - uv_rect.min.y) * 0.00;

    //             Some(TextVertex {
    //                 length: actual_length as f32,
    //                 height: actual_height as f32,
    //                 local_position: [corrected_screen_rect_pos[0], corrected_screen_rect_pos[1]],
    //                 position: global_pos,
    //                 tex_coords_min: [uv_rect.min.x + text_rect_width_clip, uv_rect.min.y + text_rect_height_clip],
    //                 tex_coords_max: [uv_rect.max.x - text_rect_width_clip, uv_rect.max.y - text_rect_height_clip],
    //                 scale: [self.scale.x as f32 * 100.0 / glyph_scale.x, self.scale.y as f32 * 100.0/ glyph_scale.y],
    //                 transform: mat2_64_to_32(*self.transform.as_ref()),
    //                 colour: color,
    //             })
    //         } else {
    //             None
    //         }
    //     }).collect()
    // }

    fn get_content(&self) -> &String {&self.content}

    fn get_shaders() -> super::shaders::Shaders {
        shaders::Shaders::VertexGeometryFragment(
            include_str!("text.vs"),
            include_str!("text.ges"),
            include_str!("text.fs"))
    }
}

impl Renderable for PlainText {
    fn get_type(&self) -> RenderType { RenderType::Text(self.clone()) }
}

#[derive(Copy, Clone)]
pub struct TextVertex {
    length: f32,
    height: f32,
    local_position: [f32; 2],
    position: [f32; 2],
    tex_coords_min: [f32; 2],
    tex_coords_max: [f32; 2],
    scale: [f32; 2],
    transform: [[f32; 2]; 2],
    colour: [f32; 4]
}

implement_vertex!(TextVertex, length, height, local_position, position, tex_coords_min, tex_coords_max, scale, transform, colour);
