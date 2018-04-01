pub mod primitives;
pub mod renderables;
pub mod shaders;
pub mod glium_renderer;
pub mod render_by_shaders;
pub mod glium_buffer;
pub mod display_settings;
use glium::Display;

pub use rendering::primitives::{
    StandardPrimitive, Rectangle, TextureRect, Polygon, CirclePart, BezierRect, PolarPixel, PlainText, BezierQuadControl, TextAlign,
    BezierBranchRect, BezierBranchCirc};
pub use rendering::renderables::{Renderable, StandardRenderable, Line, LineShape, Arrow, Circle, BoxBorder, Annulus, AnnularSegment};
pub use self::display_settings::DisplaySettings;
pub use self::glium_renderer::{GliumRenderer};

use games::view_details;
use glium::glutin::EventsLoop;

pub trait Renderer {
    type Primitive;
    fn init(&mut self) {}
    fn load_renderables(&mut self, _: Vec<Box<renderables::Renderable<Primitive=Self::Primitive>>>) {}
    fn render(&mut self) {}
    fn set_worldview(&mut self, _: view_details::ViewDetails) {}
    fn get_events_loop(&mut self) -> Option<&mut EventsLoop> { None }
    fn get_window_spec(&self) -> WindowSpec { WindowSpec::default() }
    fn reset(&mut self, _display_settings: DisplaySettings) {}
}

#[derive(Copy, Clone, Debug, Default)]
pub struct WindowSpec {
    pub aspect_ratio: f64
}