use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::Surface;
use std::borrow::Cow;
use games::view_details;
use na::{Vector3};
use na;
use super::RenderText;
use rendering::*;
use ::geometry::Point;

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vector3<f64>,
    pub scale: Point, // Applied First
    pub transform: [[f64; 2]; 2], //Applied Second
    pub color: Color,
    pub fixed: bool,
    pub align: TextAlign
}

impl RenderText for PlainText {
    type TextVert = TextVertex;
    
    fn get_vertices(
        &self,
        glyph_pos_data: Vec<(Rect<f32>, Rect<i32>)>
    ) -> Vec<Self::TextVert>
    {
        let color = [self.color.r as f32,
                     self.color.g as f32,
                     self.color.b as f32,
                     self.color.a as f32];
        let glyph_positions: Vec<[f32; 2]> = glyph_pos_data
            .iter()
            .map(|&(_, screen_rect)| {
                [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                 (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0]
                }).collect();

        let average_glyph_pos: [f32; 2] = 
            [(glyph_positions[0][0] + glyph_positions[glyph_positions.len() - 1][0]) / 2.0,
             (glyph_positions[0][1] + glyph_positions[glyph_positions.len() - 1][1]) / 2.0];
        let far_left_pos = glyph_pos_data[0].1.min.x as f32;
        
        let global_pos = [self.position.x as f32 ,self.position.y as f32, self.position.z as f32];
        glyph_pos_data.iter().map(|&(uv_rect, screen_rect)| {
            let actual_length = screen_rect.max.x - screen_rect.min.x;
            let actual_height = screen_rect.max.y - screen_rect.min.y;
            let screen_rect_pos = [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                                   (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0];
            let corrected_screen_rect_pos = match self.align {
                TextAlign::Centered => [screen_rect_pos[0] - average_glyph_pos[0],
                                      screen_rect_pos[1] - average_glyph_pos[1]],
                TextAlign::LeftBaseLine => [screen_rect_pos[0] - far_left_pos,
                                      screen_rect_pos[1]],
                TextAlign::BaseLine => [screen_rect_pos[0] - average_glyph_pos[0],
                                      screen_rect_pos[1]],
            };
            
            TextVertex {
                length: actual_length as f32,
                height: actual_height as f32,
                local_position: [corrected_screen_rect_pos[0], corrected_screen_rect_pos[1]],
                position: global_pos,
                tex_coords_min: [uv_rect.min.x, uv_rect.min.y],
                tex_coords_max: [uv_rect.max.x, uv_rect.max.y],
                scale: [self.scale.x as f32, self.scale.y as f32],
                transform: [[self.transform[0][0] as f32, self.transform[0][1] as f32],
                            [self.transform[1][0] as f32, self.transform[1][1] as f32]],
                color: color,
                fixed_pos: self.fixed as u32
                }
            } 
        ).collect()
    }

    fn get_shaders() -> shaders::Shaders {
        shaders::Shaders::VertexGeometryFragment(
            include_str!("text.vs"),
            include_str!("text.ges"),
            include_str!("text.fs"))
    }

    fn get_content(&self) -> &String {&self.content}
}

impl PlainText {
    pub fn new_simple_white(content: String, height: f64, position: Vector3<f64>, align: TextAlign) -> PlainText {
        let scale = Point::new(height, height);
        let color = Color::new(1.0, 1.0, 1.0, 1.0);

        PlainText {
            content,
            scale,
            position,
            transform: [[1.0, 0.0], [0.0, 1.0]],
            color,
            fixed: true,
            align
        }
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.content.chars().fold(1, |mut acc, char| {if char == '\r' {acc += 1; } acc })
    }

    pub fn truncate_to_line(&mut self, line: usize) {
        self.content = self.content.split('\r').take(line).fold("".to_owned(), |acc, s| {acc + "\r" + s});
    }
}

#[derive(Copy, Clone)]
pub enum TextAlign {
    Centered,
    BaseLine,
    LeftBaseLine
}

#[derive(Copy, Clone)]
pub struct TextVertex {
    length: f32,
    height: f32,
    local_position: [f32; 2],
    position: [f32; 3],
    tex_coords_min: [f32; 2],
    tex_coords_max: [f32; 2],
    scale: [f32; 2],
    transform: [[f32; 2]; 2],
    color: [f32; 4],
    fixed_pos: u32
}

implement_vertex!(
    TextVertex, 
    length, 
    height, 
    local_position, 
    position, 
    tex_coords_min, 
    tex_coords_max, 
    scale, 
    transform, 
    color, 
    fixed_pos
);
