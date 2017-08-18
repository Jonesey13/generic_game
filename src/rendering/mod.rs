pub mod primitives;
pub mod renderables;
pub mod shaders;
pub mod glium_renderer;
pub mod render_by_shaders;
pub mod glium_buffer;
use glium::backend::glutin_backend::GlutinFacade;

pub use rendering::primitives::{
    Primitive, Rectangle, Polygon, Circle, BezierRect, BezierSubrect, PolarPixel, PlainText, BezierQuadControl, TextAlign, BezierLogic};
pub use rendering::renderables::{Renderable, Line};

use games::view_details;

pub trait Renderer {
    fn init(&mut self) {}
    fn load_renderables(&mut self, _: Vec<Box<renderables::Renderable>>) {}
    fn render(&mut self) {}
    fn set_worldview(&mut self, _: view_details::ViewDetails) {}
    fn get_glutin_window(&mut self) -> Option<&mut GlutinFacade> { None }
    fn get_window_spec(&self) -> WindowSpec { WindowSpec::default() }
}

#[allow(dead_code)]
pub struct RendererStub;

impl Renderer for RendererStub {}

#[derive(Copy, Clone, Debug, Default)]
pub struct WindowSpec {
    pub aspect_ratio: f64
}