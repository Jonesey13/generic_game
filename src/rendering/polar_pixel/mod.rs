use na::{Vector2, Vector4};
use na;
use num::Zero;
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::GliumRenderable;
use super::shaders::Shaders;
use glium;
use glium::index::PrimitiveType;
use super::conversion_tools::*;
mod polar_buffer;
pub use self::polar_buffer::PolarBuffer;

#[derive(Copy, Clone)]
pub struct PolarPixel {
    pub radial: [f64; 2],
    pub angle: [f64; 2],
    pub color: [f64; 4]
}

impl Renderable for PolarPixel {
    fn get_type(&self) -> RenderType { RenderType::PolarPix(self.clone()) }
}

impl GliumRenderable for PolarPixel {
    type Vertex = PolarPixelVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("polar.vs"),
            include_str!("polar.ges"),
            include_str!("polar.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }
}

implement_vertex!(PolarPixelVertex, radial, angle, color);

#[derive(Copy, Clone, Debug)]
pub struct PolarPixelVertex {
    pub radial: [f64; 2],
    pub angle: [f64; 2],
    pub color: [f64; 4]
}

impl From<PolarPixel> for PolarPixelVertex {
    fn from(pol: PolarPixel) -> Self {
        PolarPixelVertex {
            radial: pol.radial,
            angle: pol.angle,
            color: pol.color
        }
    }
}
