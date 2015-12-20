use super::rectangle::RectangleVertex;
use super::shaders::Shaders;
use glium::index::PrimitiveType;

pub trait Renderable {
    fn get_shaders(&self) -> Shaders { Shaders::None }
    fn get_vertex(&self) -> RenderVertex { RenderVertex::None }
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Points }
}

#[allow(dead_code)]
pub struct RenderableStub;

impl Renderable for RenderableStub {}

pub enum RenderVertex {
    None,
    Rect(RectangleVertex),
}
