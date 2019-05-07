pub mod primitives;
pub mod renderables;
pub mod shaders;
pub mod glium_renderer;
pub mod render_by_shaders;
pub mod glium_buffer;
pub mod display_settings;
pub mod color;

use glium::Display;

pub use crate::rendering::primitives::*;
pub use self::renderables::*;
pub use self::display_settings::DisplaySettings;
pub use self::glium_renderer::{GliumRenderer};
pub use self::color::Color;
pub use self::shaders::*;
pub use self::render_by_shaders::*;
pub use self::glium_buffer::*;
pub use self::renderables::text::*;

use crate::games::view_details;
use glium::glutin::EventsLoop;

pub trait Renderer {
    type Primitive;
    fn init(&mut self) {}
    fn load_renderables(&mut self, _: Vec<Box<dyn renderables::Renderable<Self::Primitive>>>) {}
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