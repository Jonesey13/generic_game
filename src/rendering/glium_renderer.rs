use super::Renderer;
use super::shaders::make_program_from_shaders;
use super::rectangle::{Rectangle, RectangleVertex};
use super::circle::{Circle, CircleVertex};
use super::text::{RenderText, TextProcessor, PlainText};
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::RenderByShaders;
use glium;
use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface, DrawParameters, Depth, DepthTest, Program};
use rusttype;

pub struct GliumRenderer<'a> {
    display: Box<GlutinFacade>,
    draw_params: DrawParameters<'a>,
    rect_buffer: Buffer<Rectangle>,
    circ_buffer: Buffer<Circle>,
    text_processor: TextProcessor<'a, PlainText>,
    global_uniforms: GlobalUniforms
}

impl<'a> GliumRenderer<'a> {
    pub fn new(res: (u32, u32)) -> GliumRenderer<'a> {
        let display = Box::new(glium::glutin::WindowBuilder::new()
                               .with_dimensions(res.0, res.1)
                               //.with_multisampling(4)
                               .build_glium().unwrap());
        let draw_params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLessOrEqual,
                write: true,..Default::default()
            },
            ..Default::default()
        };
        GliumRenderer {
            display: display.clone(),
            draw_params: draw_params,
            rect_buffer: create_buffer::<Rectangle>(&display),
            circ_buffer: create_buffer::<Circle>(&display),
            text_processor: TextProcessor::new(display),
            global_uniforms: Default::default()
        }
    }

    fn flush_buffers(&mut self) {
        self.rect_buffer.vertices = None;
        self.circ_buffer.vertices = None;
        self.text_processor.text_objects = None;
    }

    fn build_uniforms(&mut self, target: &glium::Frame) {
        let (width, height) = target.get_dimensions();
        self.global_uniforms =
            GlobalUniforms {
                screen_width: width,
                screen_height: height,
                aspect_ratio: width as f32 / height as f32
            };
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    fn load_renderables(&mut self, renderables: Vec<Box<Renderable>>) {
        for renderable in renderables {
            match renderable.get_type() {
                RenderType::Rect(rectangle) => self.rect_buffer.load_renderable(rectangle),
                RenderType::Circ(circle) => self.circ_buffer.load_renderable(circle),
                RenderType::Text(text) => self.text_processor.push_text(text)
            }
        }
    }

    fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        self.build_uniforms(&target);
        self.rect_buffer.draw_at_target(&mut target, &self.display, &self.draw_params, &self.global_uniforms);
        self.circ_buffer.draw_at_target(&mut target, &self.display, &self.draw_params, &self.global_uniforms);
        self.text_processor.draw_text_at_target(&mut target, &self.display);
        target.finish().unwrap();
        self.flush_buffers();
    }
}

#[derive(Debug)]
struct Buffer<T: RenderByShaders> {
    vertices: Option<Vec<T::Vertex>>,
    program: Program,
    primitive_type: PrimitiveType,
}

impl<T: RenderByShaders> Buffer<T> {
    pub fn new(program: Program, primitive_type: PrimitiveType) -> Self {
        Buffer {
            vertices: None,
            program: program,
            primitive_type: primitive_type,
        }
    }


    pub fn load_renderable(&mut self, renderable: T) {
        if let Some(ref mut vertices) = self.vertices {
            vertices.push(renderable.get_vertex());
        }
        else {
            self.vertices = Some(vec![renderable.get_vertex()]);
        };
    }

    pub fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &GlutinFacade,
        draw_params: &DrawParameters,
        uniforms: &Unif,
    ) {
        if let Some(vertices) = self.vertices.take() {
            let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
            target.draw(&vertex_buffer,
                        &glium::index::NoIndices(self.primitive_type),
                        &self.program,
                        uniforms,
                        draw_params).unwrap();
        }
    }
}

fn create_buffer<T: RenderByShaders>(display: &GlutinFacade) -> Buffer<T>
{
    Buffer {
        vertices: None,
        program: make_program_from_shaders(T::get_shaders(), display),
        primitive_type: T::get_primitive_type(),
    }
}

#[derive(Copy, Clone, Default)]
struct GlobalUniforms {
    screen_width: u32,
    screen_height: u32,
    aspect_ratio: f32
}

impl glium::uniforms::Uniforms for GlobalUniforms {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut f: F) {

        f("screen_width", glium::uniforms::UniformValue::UnsignedInt(self.screen_width));
        f("screen_height", glium::uniforms::UniformValue::UnsignedInt(self.screen_height));
        f("aspect_ratio", glium::uniforms::UniformValue::Float(self.aspect_ratio));
    }
}
