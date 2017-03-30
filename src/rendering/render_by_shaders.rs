use glium;
use super::shaders::Shaders;
use glium::index::PrimitiveType;

pub trait GliumRenderable {
    type Vertex: glium::vertex::Vertex;
    fn get_shaders() -> Shaders { Shaders::None }
    fn get_primitive_type() -> PrimitiveType { PrimitiveType::Points }
    fn get_vertex(&self) -> Self::Vertex;
}
