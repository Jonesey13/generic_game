use super::shaders::make_program_from_shaders;
use crate::rendering::*;
use glium;
use glium::Frame;
use glium::{Display, Surface, DrawParameters, Depth, DepthTest, Program};
use rusttype;
use crate::games::view_details::ViewDetails;
use crate::utils::transforms_2d;

pub trait GliumBuffer<T: GliumStandardPrimitive> {
    fn load_renderable(&mut self, renderable: T) {
        self.get_vertices().append(&mut renderable.get_vertex());
    }

    fn get_vertices(&mut self) -> &mut Vec<T::Vertex>;

    fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &Display,
        view_details: ViewDetails,
        draw_params: &DrawParameters<'_>,
        uniforms: &Unif,
    );

    fn flush_buffer(&mut self);
}

#[derive(Debug)]
pub struct BasicBuffer<T: GliumStandardPrimitive> {
    vertices: Vec<T::Vertex>,
    program: Program,
    primitive_type: glium::index::PrimitiveType,
}

impl<T: GliumStandardPrimitive> GliumBuffer<T> for BasicBuffer<T> {
    fn get_vertices(&mut self) -> &mut Vec<T::Vertex> {
        &mut self.vertices
    }

    fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &Display,
        _: ViewDetails,
        draw_params: &DrawParameters<'_>,
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
}

impl<T: GliumStandardPrimitive> BasicBuffer<T> {
    pub fn new(display: &Display) -> Self {
        BasicBuffer {
            vertices: Vec::new(),
            program: make_program_from_shaders(T::get_shaders(), display),
            primitive_type: T::get_primitive_type(),
        }
    }
}
