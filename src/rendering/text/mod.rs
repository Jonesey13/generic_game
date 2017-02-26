use na::{Vec2, Vec4, Mat2, Diag, ToHomogeneous};
use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::Surface;
use arrayvec;
use std::borrow::Cow;
use super::shaders;
use super::conversion_tools::mat4_64_to_32;
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

    fn build_vertex_buffer(
        &self,
        screen_width: u32,
        screen_height: u32,
        glyphs:  Vec<PositionedGlyph<'a>>,
        cache: &rusttype::gpu_cache::Cache
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
        let rt_scale = rusttype::Scale {
            x: self.scale.x as f32,
            y: self.scale.y as f32,
        };
        
        let glyphs = layout_paragraph(font, rt_scale, width, &self.content);
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
        }).unwrap();
        
        let uniforms = uniform! {
            tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        let vertices = self.build_vertex_buffer(width, height, glyphs, cache);

        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &vertices).unwrap();

        target.draw(&vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms,
                    &glium::DrawParameters {
                        blend: glium::Blend::alpha_blending(),
                        ..Default::default()
                    }).unwrap();

    }

    fn build_vertex_buffer(
        &self,
        screen_width: u32,
        screen_height: u32,
        glyphs:  Vec<PositionedGlyph<'a>>,
        cache: &rusttype::gpu_cache::Cache
    ) -> Vec<TextVertex>
    {
        let color = [self.color.x as f32,
                     self.color.y as f32,
                     self.color.z as f32,
                     self.color.w as f32];
        let origin = point(0.0, 0.0);
        glyphs.iter().flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin
                        + (vector(screen_rect.min.x as f32 / screen_width as f32 - 0.5,
                                  1.0 - screen_rect.min.y as f32 / screen_height as f32 - 0.5)) * 2.0,
                    max: origin
                        + (vector(screen_rect.max.x as f32 / screen_width as f32 - 0.5,
                                  1.0 - screen_rect.max.y as f32 / screen_height as f32 - 0.5)) * 2.0
                };
                arrayvec::ArrayVec::<[TextVertex; 6]>::from([
                    TextVertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour: color
                    },
                    TextVertex {
                        position: [gl_rect.min.x,  gl_rect.min.y],
                        tex_coords: [uv_rect.min.x, uv_rect.min.y],
                        colour: color
                    },
                    TextVertex {
                        position: [gl_rect.max.x,  gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour: color
                    },
                    TextVertex {
                        position: [gl_rect.max.x,  gl_rect.min.y],
                        tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        colour: color },
                    TextVertex {
                        position: [gl_rect.max.x, gl_rect.max.y],
                        tex_coords: [uv_rect.max.x, uv_rect.max.y],
                        colour: color
                    },
                    TextVertex {
                        position: [gl_rect.min.x, gl_rect.max.y],
                        tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        colour: color
                    }])
            } else {
                arrayvec::ArrayVec::new()
            }
        }).collect()
    }

}

impl Renderable for PlainText {
    fn get_type(&self) -> RenderType { RenderType::Text(self.clone()) }
}

#[derive(Copy, Clone)]
pub struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    colour: [f32; 4]
}

implement_vertex!(TextVertex, position, tex_coords, colour);

pub struct TextProcessor<'a, T: RenderText<'a>> {
    pub text_objects: Option<Vec<T>>,
    text_cache: rusttype::gpu_cache::Cache,
    program: glium::Program,
    cache_texture: glium::texture::Texture2d,
    font: Font<'a>,
}

impl<'a, T: RenderText<'a>> TextProcessor<'a, T> {
    pub fn new(display: Box<glium::backend::glutin_backend::GlutinFacade>) -> Self {
        let dpi_factor = display.get_window().unwrap().hidpi_factor();

        let (cache_width, cache_height) = (512 * dpi_factor as u32, 512 * dpi_factor as u32);
        let cache = rusttype::gpu_cache::Cache::new(cache_width, cache_height, 0.1, 0.1);
        let cache_tex = glium::texture::Texture2d::with_format(
            &*display,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
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
    shaders::Shaders::VertexFragment(
        include_str!("text.vs"),
        include_str!("text.fs"))
}

fn layout_paragraph<'a>(font: &'a Font,
                        scale: Scale,
                        width: u32,
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
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

