use na::{Vec2, Vec4, Mat2, Diag, ToHomogeneous};
use unicode_normalization;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use rusttype::Rect;
use arrayvec;
use super::conversion_tools::mat4_64_to_32;
use super::renderables::{Renderable, RenderType};

pub static OPEN_SANS: &'static[u8] = include_bytes!("OpenSans.ttf");
pub static font = FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vec2<f64>, //Bottom Left
    pub scale: Vec2<f64>, // Applied First
    pub transform: Mat2<f64>, //Applied Second
    pub color: Vec4<f64>
}

pub trait RenderText {
    fn render(&mut self, target: &mut glium::Frame);
}

impl RenderText for PlainText {
    fn render(&mut self, target: &mut glium::Frame) {
        let cache_tex = glium::texture::Texture2d::with_format(
        &display,
        glium::texture::RawImage2d {
            data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
            width: cache_width,
            height: cache_height,
            format: glium::texture::ClientFormat::U8
        },
        glium::texture::UncompressedFloatFormat::U8,
        glium::texture::MipmapsOption::NoMipmap).unwrap();
        

        
        // let render_text = glium_text::TextDisplay::new(txt_sys, font_texture, &self.content);
        // let scale_mat = Mat2::from_diag(&self.scale);
        // let scale_and_transform = self.transform * scale_mat;
        // let mut mat = scale_and_transform.to_homogeneous().to_homogeneous();
        // mat.m14 = self.position.x;
        // mat.m24 = self.position.y;
        // let color = (self.color.x as f32, self.color.y as f32, self.color.z as f32, self.color.w as f32);
        // glium_text::draw(&render_text, txt_sys, target, mat4_64_to_32(*mat.as_ref()), color);
    }
}

impl Renderable for PlainText {
    fn get_type(&self) -> RenderType { RenderType::Txt(self.clone()) }
}

#[derive(Copy, Clone)]
struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    colour: [f32; 4]
}

implement_vertex!(TextVertex, position, tex_coords, colour);

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

fn get_vertex_buffer(display, glyphs, cache,) -> glium::VertexBuffer {
    let colour = [0.0, 0.0, 0.0, 1.0];
    let (screen_width, screen_height) = {
        let (w, h) = display.get_framebuffer_dimensions();
        (w as f32, h as f32)
    };
    let origin = point(0.0, 0.0);
    let vertices: Vec<TextVertex> = glyphs.iter().flat_map(|g| {
        if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
            let gl_rect = Rect {
                min: origin
                    + (vector(screen_rect.min.x as f32 / screen_width - 0.5,
                              1.0 - screen_rect.min.y as f32 / screen_height - 0.5)) * 2.0,
                max: origin
                    + (vector(screen_rect.max.x as f32 / screen_width - 0.5,
                              1.0 - screen_rect.max.y as f32 / screen_height - 0.5)) * 2.0
            };
            arrayvec::ArrayVec::<[TextVertex; 6]>::from([
                TextVertex {
                    position: [gl_rect.min.x, gl_rect.max.y],
                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    colour: colour
                },
                TextVertex {
                    position: [gl_rect.min.x,  gl_rect.min.y],
                    tex_coords: [uv_rect.min.x, uv_rect.min.y],
                    colour: colour
                },
                TextVertex {
                    position: [gl_rect.max.x,  gl_rect.min.y],
                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    colour: colour
                },
                TextVertex {
                    position: [gl_rect.max.x,  gl_rect.min.y],
                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    colour: colour },
                TextVertex {
                    position: [gl_rect.max.x, gl_rect.max.y],
                    tex_coords: [uv_rect.max.x, uv_rect.max.y],
                    colour: colour
                },
                TextVertex {
                    position: [gl_rect.min.x, gl_rect.max.y],
                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    colour: colour
                }])
        } else {
            arrayvec::ArrayVec::new()
        }
    }).collect();

    glium::VertexBuffer::new(
        &display,
        &vertices).unwrap()
}
