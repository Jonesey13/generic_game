use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::Surface;
use std::borrow::Cow;
use games::view_details;
use na::{Vector2, Vector3, Vector4, Matrix2};
use na;
use super::RenderText;
use rendering::{shaders, Renderable};

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vector3<f64>,
    pub scale: Vector2<f64>, // Applied First
    pub transform: Matrix2<f64>, //Applied Second
    pub color: Vector4<f64>,
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
        let far_left_pos = glyph_pos_data[0].1.min.x as f32;
        
        let global_pos = [self.position.x as f32 ,self.position.y as f32, self.position.z as f32];
        glyph_pos_data.iter().map(|&(uv_rect, screen_rect)| {
            let actual_length = screen_rect.max.x - screen_rect.min.x;
            let actual_height = screen_rect.max.y - screen_rect.min.y;
            let screen_rect_pos = [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                                   (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0];
            let corrected_screen_rect_pos = match self.align {
                TextAlign::Center => [screen_rect_pos[0] - average_glyph_pos[0],
                                      screen_rect_pos[1] - average_glyph_pos[1]],
                TextAlign::Left => [screen_rect_pos[0] - far_left_pos,
                                      screen_rect_pos[1] - average_glyph_pos[1]],
            };
            
            TextVertex {
                length: actual_length as f32,
                height: actual_height as f32,
                local_position: [corrected_screen_rect_pos[0], corrected_screen_rect_pos[1]],
                position: global_pos,
                tex_coords_min: [uv_rect.min.x, uv_rect.min.y],
                tex_coords_max: [uv_rect.max.x, uv_rect.max.y],
                scale: [self.scale.x as f32, self.scale.y as f32 ],
                transform: *na::convert::<_, Matrix2<f32>>(self.transform).as_ref(),
                colour: color,
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
        let scale = Vector2::new(height, height);
        let color = Vector4::new(1.0, 1.0, 1.0, 1.0);

        PlainText {
            content,
            scale,
            position,
            transform: Matrix2::identity(),
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
    Center,
    Left
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
    colour: [f32; 4],
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
    colour, 
    fixed_pos
);