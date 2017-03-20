pub mod renderables;
pub mod rectangle;
pub mod text;
pub mod circle;
pub mod shaders;
pub mod glium_renderer;
pub mod render_by_shaders;
mod conversion_tools;

pub trait Renderer {
    fn init(&mut self) {}
    fn load_renderables(&mut self, _: Vec<Box<renderables::Renderable>>) {}
    fn render(&mut self) {}
    fn set_worldview(&mut self, _: [[f32; 4]; 4]) {}
}

#[allow(dead_code)]
pub struct RendererStub;

impl Renderer for RendererStub {}
