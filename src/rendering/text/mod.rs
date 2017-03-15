use na::{Vec2, Vec4, Mat2, Diag, ToHomogeneous};
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

pub const OPEN_SANS: &'static[u8] = include_bytes!("OpenSans.ttf");

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vec2<f64>, //Bottom Left
    pub scale: Vec2<f64>, // Applied First
    pub transform: Mat2<f64>, //Applied Second
    pub color: Vec4<f64>
}

pub trait RenderText<'a> {
    fn render(
        &mut self,
        target:&mut glium::Frame,
        font: &Font<'a>,
        cache: &mut rusttype::gpu_cache::Cache,
        cache_tex: &glium::texture::Texture2d,
        program: &glium::Program,
        display: &glium::backend::glutin_backend::GlutinFacade);

    fn get_vertices(
        &self,
        glyphs:  Vec<PositionedGlyph<'a>>,
        cache: &rusttype::gpu_cache::Cache,
        glyph_scale: Scale
    ) -> Vec<TextVertex>;
}

impl<'a> RenderText<'a> for PlainText {
    fn render(&mut self,
              target:&mut glium::Frame,
              font: &Font<'a>,
              cache: &mut rusttype::gpu_cache::Cache,
              cache_tex: &glium::texture::Texture2d,
              program: &glium::Program,
              display: &glium::backend::glutin_backend::GlutinFacade)
    {
        let (width, height) = target.get_dimensions();
        let dpi_factor = display.get_window().unwrap().hidpi_factor();

        let glyph_scale = Scale::uniform(256.0 * dpi_factor);
        
        let glyphs = layout_paragraph(font, glyph_scale, &self.content);
        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }
        
        cache.cache_queued(|rect, data| {
            cache_tex.main_level().write(glium::Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height()
            }, glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8
            });
        },
        1).unwrap();
        
        let uniforms = uniform! {
            tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            screen_width: width,
            screen_height: height,
            aspect_ratio: width as f32 / height as f32
        };

        let vertices = self.get_vertices(
            glyphs,
            cache,
            glyph_scale);

        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &vertices).unwrap();

        target.draw(&vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &program, &uniforms,
                    &glium::DrawParameters {
                        blend: glium::Blend::alpha_blending(),
                        ..Default::default()
                    }).unwrap();

    }

    fn get_vertices(
        &self,
        glyphs: Vec<PositionedGlyph<'a>>,
        cache: &rusttype::gpu_cache::Cache,
        glyph_scale: Scale
    ) -> Vec<TextVertex>
    {
        let color = [self.color.x as f32,
                     self.color.y as f32,
                     self.color.z as f32,
                     self.color.w as f32];
        let glyph_positions: Vec<[f32; 2]> = glyphs
            .iter()
            .filter_map(|g| {
                if let Ok(Some((_, screen_rect))) = cache.rect_for(0, g) {
                    Some([(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                          (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0])
                } else {
                    None
                }}).collect();

        let mut average_glyph_pos: [f32; 2] = glyph_positions
            .iter()
            .fold([0.0, 0.0], |acc, rect| 
                [acc[0] + rect[0], acc[1] + rect[1]]
            );
        average_glyph_pos = [average_glyph_pos[0] / (glyph_positions.len() as f32),
                             average_glyph_pos[1] / (glyph_positions.len() as f32)];
        
        let global_pos = [self.position.x as f32 ,self.position.y as f32];
        glyphs.iter().filter_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let actual_length = screen_rect.max.x - screen_rect.min.x;
                let actual_height = screen_rect.max.y - screen_rect.min.y;
                let screen_rect_pos = [(screen_rect.min.x + screen_rect.max.x) as f32 / 2.0,
                                       (screen_rect.min.y + screen_rect.max.y) as f32 / 2.0];
                let corrected_screen_rect_pos = [screen_rect_pos[0] - average_glyph_pos[0],
                                                 screen_rect_pos[1] - average_glyph_pos[1]];
                let text_rect_width_clip = (uv_rect.max.x - uv_rect.min.x) * 0.00;
                let text_rect_height_clip = (uv_rect.max.y - uv_rect.min.y) * 0.00;

                println!("{:?}", uv_rect);
                Some(TextVertex {
                    length: actual_length as f32,
                    height: actual_height as f32,
                    local_position: [corrected_screen_rect_pos[0], corrected_screen_rect_pos[1]],
                    position: global_pos,
                    tex_coords_min: [uv_rect.min.x + text_rect_width_clip, uv_rect.min.y + text_rect_height_clip],
                    tex_coords_max: [uv_rect.max.x - text_rect_width_clip, uv_rect.max.y - text_rect_height_clip],
                    scale: [self.scale.x as f32 * 100.0 / glyph_scale.x, self.scale.y as f32 * 100.0/ glyph_scale.y],
                    transform: mat2_64_to_32(*self.transform.as_ref()),
                    colour: color,
                })
            } else {
                None
            }
        }).collect()
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

pub struct TextProcessor<'a, T: RenderText<'a>> {
    pub text_objects: Option<Vec<T>>,
    text_cache: rusttype::gpu_cache::Cache,
    program: glium::Program,
    cache_texture: glium::texture::Texture2d,
    font: Font<'a>,
}

impl<'a, T: RenderText<'a>> TextProcessor<'a, T> {
    pub fn new(display: Box<glium::backend::glutin_backend::GlutinFacade>) -> Self {
        let (screen_width, screen_height) = display.get_window().unwrap().get_inner_size().unwrap();
        let dpi_factor = display.get_window().unwrap().hidpi_factor();

        let (cache_width, cache_height) = (512 * dpi_factor as u32, 512 * dpi_factor as u32);
        let cache = rusttype::gpu_cache::Cache::new(cache_width, cache_height, 0.1, 0.1);
        let cache_tex = glium::texture::Texture2d::with_format(
            &*display,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![0u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap).unwrap();

        TextProcessor {
            text_objects: None,
            text_cache: cache,
            cache_texture: cache_tex,
            program: shaders::make_program_from_shaders(get_text_shaders(), &display),
            font: FontCollection::from_bytes(OPEN_SANS).into_font().unwrap(),
        }
    }

    pub fn push_text(&mut self, text: T) {
        if let Some(ref mut buffer) = self.text_objects {
            buffer.push(text);
        }
        else {
            self.text_objects = Some(vec![text]);
        }
    }

    pub fn draw_text_at_target(&mut self,
                               target: &mut glium::Frame,
                               display: &glium::backend::glutin_backend::GlutinFacade) {
        let buffer = self.text_objects.take();
        if let Some(buffer) = buffer {
            for mut render_text in buffer {
                render_text.render(target,
                                   &self.font,
                                   &mut self.text_cache,
                                   &self.cache_texture,
                                   &self.program,
                                   display);
            }
        }
    }
}

fn get_text_shaders() -> shaders::Shaders {
    shaders::Shaders::VertexGeometryFragment(
        include_str!("text.vs"),
        include_str!("text.ges"),
        include_str!("text.fs"))
}

fn layout_paragraph<'a>(font: &'a Font,
                        scale: Scale,
                        text: &str) -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {},
                _ => {}
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let glyph = base_glyph.scaled(scale).positioned(caret);
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

