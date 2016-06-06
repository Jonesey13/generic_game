use super::Renderer;
use super::shaders::make_program_from_shaders;
use super::rectangle::{Rectangle, RectangleVertex};
use super::circle::{Circle, CircleVertex};
use super::text::PlainText;
use super::text::RenderText;
use super::text::OPEN_SANS;
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::RenderByShaders;
use glium_text;
use glium;
use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface, DrawParameters, Depth, DepthTest, Program};

pub struct GliumRenderer<'a> {
    display: Box<GlutinFacade>,
    draw_params: DrawParameters<'a>,
    rect_buffer: Buffer<Rectangle>,
    circ_buffer: Buffer<Circle>,
    text_processor: TextProcessor<PlainText>
}

impl<'a> GliumRenderer<'a> {
    pub fn new(res: (u32, u32)) -> GliumRenderer<'a> {
        let display = Box::new(glium::glutin::WindowBuilder::new().with_dimensions(res.0, res.1).build_glium().unwrap());
        let draw_params = DrawParameters {
            depth: Depth { test: DepthTest::IfLessOrEqual, write: true,..Default::default()},
            ..Default::default()
        };
        GliumRenderer {
            display: display.clone(),
            draw_params: draw_params,
            rect_buffer: create_buffer::<Rectangle>(&display),
            circ_buffer: create_buffer::<Circle>(&display),
            text_processor: TextProcessor::new(OPEN_SANS, 120, display),
        }
    }

    fn flush_buffers(&mut self) {
        self.rect_buffer.vertices = None;
        self.circ_buffer.vertices = None;
        self.text_processor.text_objects = None;
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    fn load_renderables(&mut self, renderables: Vec<Box<Renderable>>) {
        for renderable in renderables {
            match renderable.get_type() {
                RenderType::Rect(rectangle) => self.rect_buffer.load_renderable(rectangle),
                RenderType::Circ(circle) => self.circ_buffer.load_renderable(circle),
                RenderType::Txt(text) => self.text_processor.push_text(text)
            }
        }
    }

    fn render(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);
        self.rect_buffer.draw_at_target(&mut target, &self.display, &self.draw_params);
        self.circ_buffer.draw_at_target(&mut target, &self.display, &self.draw_params);
        self.text_processor.draw_text_at_target(&mut target);
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

    pub fn draw_at_target(&mut self, target: &mut Frame, display: &GlutinFacade, draw_params: &DrawParameters) {
        if let Some(vertices) = self.vertices.take() {
            let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
            target.draw(&vertex_buffer,
                        &glium::index::NoIndices(self.primitive_type),
                        &self.program,
                        &glium::uniforms::EmptyUniforms,
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

struct TextProcessor<T: RenderText> {
    text_objects: Option<Vec<T>>,
    txt_sys: glium_text::TextSystem,
    font_text: glium_text::FontTexture
}

impl<T: RenderText> TextProcessor<T> {
    pub fn new(font_string: &'static[u8], font_size: u32, display: Box<GlutinFacade>) -> Self {
        let font = match glium_text::FontTexture::new(&*display, font_string, font_size) {
            Ok(fnt) => fnt,
            Err(_) => panic!("Error Loading Font!")
        };
        let txt_system = glium_text::TextSystem::new(&*display);

        TextProcessor {
            text_objects: None,
            txt_sys: txt_system,
            font_text: font
        }
    }

    pub fn push_text(&mut self, text: T) {
        if let Some(ref mut buffer) = self.text_objects {
            buffer.push(text);
        }
        else {
            self.text_objects = Some(vec![text]);
        }
    }

    pub fn draw_text_at_target(&mut self, target: &mut Frame) {
        let buffer = self.text_objects.take();
        if let Some(buffer) = buffer {
            for mut render_text in buffer {
                render_text.render(target, &self.txt_sys, &self.font_text);
            }
        }
    }
}