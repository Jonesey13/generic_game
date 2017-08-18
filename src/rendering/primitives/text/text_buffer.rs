use na::{Vector2, Vector4, Matrix2};
use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype;
use rusttype::Rect;
use glium;
use glium::{Surface, Frame, DrawParameters, Depth, DepthTest};
use glium::backend::glutin_backend::GlutinFacade;
use std::borrow::Cow;
use super::{RenderText};
use rendering;
use rendering::shaders;
use rendering::glium_buffer::GliumBuffer;
use rendering::render_by_shaders::GliumPrimitive;
use games::view_details;
use std::sync::Mutex;
use debug::*;

pub const OPEN_SANS: &'static[u8] = include_bytes!("OpenSans.ttf");

lazy_static! {
    static ref OPEN_SANS_FONT: Mutex<Font<'static>> = Mutex::new(FontCollection::from_bytes(OPEN_SANS).into_font().unwrap());
}

pub struct TextBuffer<'a, T: RenderText> {
    vertices: Vec<T::TextVert>,
    text_cache: rusttype::gpu_cache::Cache,
    program: glium::Program,
    cache_tex: glium::texture::Texture2d,
    font: Font<'a>,
    glyph_scale: f32
}

impl<'a, T: RenderText> TextBuffer<'a, T> {
    pub fn new(display: Box<GlutinFacade>) -> Self {
        let dpi_factor = display.get_window().unwrap().hidpi_factor();

        let (cache_width, cache_height) = (10000 * dpi_factor as u32, 10000 * dpi_factor as u32);
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

        TextBuffer {
            vertices: Vec::new(),
            text_cache: cache,
            cache_tex: cache_tex,
            program: shaders::make_program_from_shaders(T::get_shaders(), &display),
            font: FontCollection::from_bytes(OPEN_SANS).into_font().unwrap(),
            glyph_scale: 128.0 * dpi_factor
        }
    }

    fn render<Unif: glium::uniforms::Uniforms>(
        vertices: &Vec<T::TextVert>,
        program: &glium::Program,
        target: &mut Frame,
        display: &GlutinFacade,
        _: &DrawParameters,
        uniforms: &Unif)
    {
        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &vertices).unwrap();

        target.draw(&vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::Points),
                    &program,
                    uniforms,
                    &glium::DrawParameters {
                        blend: glium::Blend::alpha_blending(),
                        depth: Depth {
                            test: DepthTest::IfLessOrEqual,
                            write: true,..Default::default()
                        },
                        ..Default::default()
                    }).unwrap();

    }
}

impl<'a, T: RenderText> GliumBuffer<T> for TextBuffer<'a, T> {
    fn draw_at_target<Unif: glium::uniforms::Uniforms>(
        &mut self,
        target: &mut Frame,
        display: &GlutinFacade,
        view_details: view_details::ViewDetails,
        draw_params: &DrawParameters,
        _: &Unif
    ) {
        if !self.vertices.is_empty() {
            let (width, height) = target.get_dimensions();
            
            let aspect_ratio = width as f64 / height as f64;

            let cache_tex = &self.cache_tex;
            let v_metrics = self.font.v_metrics(Scale::uniform(self.glyph_scale));
            let uniforms = uniform! {
                tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
                screen_width: width,
                screen_height: height,
                max_char_height_pix: v_metrics.ascent - v_metrics.descent,
                aspect_ratio: aspect_ratio as f32,
                world_view: rendering::glium_renderer::GliumRenderer::create_worldview_mat(view_details, aspect_ratio),      
            };

            Self::render(&self.vertices, &self.program, target, display, draw_params, &uniforms);
        }
    }

    fn load_renderable(&mut self, text: T) {
        debug_clock_start("Render::glium_load::text");
        if text.get_content().len() == 0 {return; }
        let glyph_scale = Scale::uniform(self.glyph_scale);

        debug_clock_start("Render::glium_load::text::layout_paragraph");
        let glyphs = layout_paragraph(&self.font, glyph_scale, &text.get_content());
        debug_clock_stop("Render::glium_load::text::layout_paragraph");

        debug_clock_start("Render::glium_load::text::queue_glyph");
        for glyph in &glyphs {
            self.text_cache.queue_glyph(0, glyph);
        }
        debug_clock_stop("Render::glium_load::text::queue_glyph");

        let cache_tex = &mut self.cache_tex;
        let mut text_cache = &mut self.text_cache;

        debug_clock_start("Render::glium_load::text::queue_cache");
        text_cache.cache_queued(
            |rect, data| {
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
            4).unwrap();
        debug_clock_stop("Render::glium_load::text::queue_cache");

        debug_clock_start("Render::glium_load::text::glyph_pos_data");
        let glyph_pos_data: Vec<(Rect<f32>, Rect<i32>)> = glyphs
            .iter()
            .filter_map(|g| {
                if let Ok(Some(pos_data)) = text_cache.rect_for(0, g) {
                    Some(pos_data)
                } else {
                    None
                }}).collect();
        debug_clock_stop("Render::glium_load::text::glyph_pos_data");                

        debug_clock_start("Render::glium_load::text::get_vertices");
        let mut vertices = text.get_vertices(glyph_pos_data);
        self.vertices.append(&mut vertices);
        debug_clock_stop("Render::glium_load::text::get_vertices");

        debug_clock_stop("Render::glium_load::text");
    }

    fn get_vertices(&mut self) -> &mut Vec<T::TextVert> {
        &mut self.vertices
    }

    fn flush_buffer(&mut self) {
        self.vertices = Vec::new();
    }
}

fn layout_paragraph<'a>(font: &'a Font,
                        scale: Scale,
                        text: &str) -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent;
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
