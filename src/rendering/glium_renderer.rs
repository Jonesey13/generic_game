use super::Renderer;
use super::shaders::make_program_from_shaders;
use super::rectangle::RectangleVertex;
use super::renderables::{Renderable, RenderVertex};
use glium;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface, DrawParameters, Depth, DepthTest, Program};

pub struct GliumRenderer<'a> {
    display: GlutinFacade,
    draw_params: DrawParameters<'a>,
    rect_buffer: Option<Buffer<RectangleVertex>>,
}

impl<'a> GliumRenderer<'a> {
    pub fn new(res: (u32, u32)) -> GliumRenderer<'a> {
        let display = glium::glutin::WindowBuilder::new().with_dimensions(res.0,res.1).build_glium().unwrap();
        // let draw_params = DrawParameters {
        //     depth: Depth { test: DepthTest::IfLessOrEqual, write: true,..Default::default()},
        //     ..Default::default()
        // };
        let draw_params = glium::draw_parameters::DrawParameters::default();
        GliumRenderer {
            display: display,
            draw_params: draw_params,
            rect_buffer: None,
        }
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    fn load_renderables(&mut self, renderables: Vec<Box<Renderable>>) {
        for renderable in renderables {
            match renderable.get_vertex() {
                RenderVertex::Rect(vertex) => if let Some(ref mut buff) = self.rect_buffer {
                    buff.push_vertex(vertex);
                        }
                    else {
                        self.rect_buffer = Some(Buffer::new(
                            make_program_from_shaders(renderable.get_shaders(), &self.display),
                            renderable.get_primitive_type()));
                        if let Some(ref mut buffer) = self.rect_buffer{ buffer.push_vertex(vertex);};
                    },
                    _ => (),
            }
        }
    }

    fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);

        if let Some(ref mut buffer) = self.rect_buffer {
            println!("here!");
            println!("{:?}", buffer);
            let vertex_buffer = glium::VertexBuffer::new(&self.display, &buffer.vertices).unwrap();
            target.draw(&vertex_buffer,
                        &glium::index::NoIndices(buffer.primitive_type),
                        &buffer.program,
                        &glium::uniforms::EmptyUniforms,
                        &self.draw_params).unwrap();
            target.finish().unwrap();
        }
        self.rect_buffer = None;
    }
}

#[derive(Debug)]
struct Buffer<T> {
    vertices: Vec<T>,
    program: Program,
    primitive_type: PrimitiveType,
}

impl<T> Buffer<T> {
    pub fn new(program: Program, primitive_type: PrimitiveType) -> Self {
        Buffer {
            vertices: Vec::new(),
            program: program,
            primitive_type: primitive_type,
        }
    }
    pub fn push_vertex(&mut self, vertex: T) {
        self.vertices.push(vertex);
    }
}
