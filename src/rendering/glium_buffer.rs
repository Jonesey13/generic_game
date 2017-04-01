use super::shaders::make_program_from_shaders;
use super::rectangle::{Rectangle, RectangleVertex};
use super::circle::{Circle, CircleVertex};
use super::text::{RenderText, PlainText};
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::GliumRenderable;
use glium;
use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface, DrawParameters, Depth, DepthTest, Program};
use na;
use na::Matrix4;
use num::One;
use rusttype;
use games::view_details::ViewDetails;
use utils::transforms_2d;

pub trait GliumBuffer<T: GliumRenderable> {
    fn load_renderable(&mut self, renderable: T) {
        self.get_vertices().push(renderable.get_vertex());
    }

    fn get_vertices(&mut self) -> &mut Vec<T::Vertex>;

    fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &GlutinFacade,
        view_details: ViewDetails,
        draw_params: &DrawParameters,
        uniforms: &Unif,
    );

    fn flush_buffer(&mut self);
}

#[derive(Debug)]
pub struct BasicBuffer<T: GliumRenderable> {
    vertices: Vec<T::Vertex>,
    program: Program,
    primitive_type: PrimitiveType,
}

impl<T: GliumRenderable> GliumBuffer<T> for BasicBuffer<T> {
    fn get_vertices(&mut self) -> &mut Vec<T::Vertex> {
        &mut self.vertices
    }

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
}

impl<T: GliumRenderable> BasicBuffer<T> {
    pub fn new(display: &GlutinFacade) -> Self {
        BasicBuffer {
            vertices: Vec::new(),
            program: make_program_from_shaders(T::get_shaders(), display),
            primitive_type: T::get_primitive_type(),
        }
    }
}
