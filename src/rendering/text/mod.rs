use na::{Vec2, Vec4, Mat2, Diag, ToHomogeneous};
use glium;
use glium_text;
use super::conversion_tools::mat4_64_to_32;
use super::renderables::{Renderable, RenderType};

pub static OPEN_SANS: &'static[u8] = include_bytes!("OpenSans.ttf");

#[derive(Clone)]
pub struct PlainText {
    pub content: String,
    pub position: Vec2<f64>, //Bottom Left
    pub scale: Vec2<f64>, // Applied First
    pub transform: Mat2<f64>, //Applied Second
    pub color: Vec4<f64>
}

pub trait RenderText {
    fn render(&mut self, target: &mut glium::Frame, txt_sys: &glium_text::TextSystem, font_text: &glium_text::FontTexture);
}

impl RenderText for PlainText {
    fn render(&mut self, target: &mut glium::Frame, txt_sys: &glium_text::TextSystem, font_texture: &glium_text::FontTexture) {
        let render_text = glium_text::TextDisplay::new(txt_sys, font_texture, &self.content);
        let scale_mat = Mat2::from_diag(&self.scale);
        let scale_and_transform = self.transform * scale_mat;
        let mut mat = scale_and_transform.to_homogeneous().to_homogeneous();
        mat.m14 = self.position.x;
        mat.m24 = self.position.y;
        let color = (self.color.x as f32, self.color.y as f32, self.color.z as f32, self.color.w as f32);
        glium_text::draw(&render_text, txt_sys, target, mat4_64_to_32(*mat.as_ref()), color);
    }
}

impl Renderable for PlainText {
    fn get_type(&self) -> RenderType { RenderType::Txt(self.clone()) }
}
