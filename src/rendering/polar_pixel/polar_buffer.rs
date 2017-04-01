use na::{Vector2, Vector4, Matrix2};
use glium;
use glium::{Surface, Frame, DrawParameters};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use super::{PolarPixel, PolarPixelVertex};
use rendering;
use rendering::shaders;
use rendering::glium_buffer::GliumBuffer;
use rendering::render_by_shaders::GliumRenderable;
use games::view_details;
use games::view_details::ViewDetails;
use rendering::shaders::make_program_from_shaders;

pub struct PolarBuffer {
    vertices: Vec<PolarPixelVertex>,
    program: glium::Program,
    primitive_type: PrimitiveType
}

impl GliumBuffer<PolarPixel> for PolarBuffer {
    fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &GlutinFacade,
        _: ViewDetails,
        draw_params: &DrawParameters,
        uniforms: &Unif,
    ) {
        if !self.vertices.is_empty() {
            let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
            target.draw(&vertex_buffer,
                        &glium::index::NoIndices(self.primitive_type),
                        &self.program,
                        uniforms,
                        draw_params).unwrap();
        }
    }

    fn flush_buffer(&mut self) {
        self.vertices = Vec::new();
    }

    fn get_vertices(&mut self) -> &mut Vec<PolarPixelVertex> {
        &mut self.vertices
    }
}

impl PolarBuffer {
    pub fn new(display: &GlutinFacade) -> Self {
        PolarBuffer {
            vertices: Vec::new(),
            program: make_program_from_shaders(PolarPixel::get_shaders(), display),
            primitive_type: PolarPixel::get_primitive_type(),
        }
    }
}
