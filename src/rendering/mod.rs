pub mod renderables;
pub mod rectangle;
pub mod text;
pub mod circle;
pub mod shaders;
pub mod glium_renderer;
pub mod render_by_shaders;
mod conversion_tools;
pub mod glium_buffer;
pub mod polar_pixel;
pub mod bezier_rect;
pub mod bezier_subrect;
pub mod polygon;
use glium::backend::glutin_backend::GlutinFacade;

pub use rendering::rectangle::Rectangle;
pub use rendering::circle::Circle;
pub use rendering::text::{PlainText, TextAlign};
pub use rendering::polar_pixel::PolarPixel;
pub use rendering::bezier_rect::BezierRect;
pub use rendering::bezier_subrect::{BezierSubrect, BezierLogic};
pub use rendering::bezier_rect::BezierQuadControl;
pub use rendering::renderables::Renderable;
pub use rendering::renderables::RenderType;
pub use rendering::polygon::Polygon;

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