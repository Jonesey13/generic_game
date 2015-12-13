pub trait Renderable {
    fn get_shaders(&self) -> Shaders { Shaders::None }
}

pub struct RenderableStub;

impl Renderable for RenderableStub {}

pub enum Shaders {
    None,
    VertexFragment(&'static str, &'static str),
    VertexGeometryFragment(&'static str, &'static str, &'static str),
    VertexTesselationFragment(&'static str, &'static str, &'static str, &'static str),
    VertexTesselationGeometryFragment(&'static str, &'static str, &'static str, &'static str, &'static str),
}
