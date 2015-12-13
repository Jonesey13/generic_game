pub mod renderables;
pub mod rectangle;

pub trait Renderer {
    fn init(&mut self) {}
    fn load_renderables(&mut self, Vec<Box<renderables::Renderable>>) {}
    fn render(&mut self) {}
}

pub struct RendererStub;

impl Renderer for RendererStub {}
