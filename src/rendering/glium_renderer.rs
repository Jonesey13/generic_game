use super::Renderer;
use super::shaders::make_program_from_shaders;
use super::rectangle::RectangleVertex;
use super::circle::CircleVertex;
use super::text::RenderText;
use super::text::OPEN_SANS;
use super::renderables::{Renderable, RenderVertex};
use glium_text;
use glium;
use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface, DrawParameters, Depth, DepthTest, Program};

pub struct GliumRenderer<'a> {
    display: Box<GlutinFacade>,
    draw_params: DrawParameters<'a>,
    rect_buffer: Option<Buffer<RectangleVertex>>,
    circ_buffer: Option<Buffer<CircleVertex>>,
    text_processor: TextProcessor
}

impl<'a> GliumRenderer<'a> {
    pub fn new(res: (u32, u32)) -> GliumRenderer<'a> {
        let display = Box::new(glium::glutin::WindowBuilder::new().with_dimensions(res.0,res.1).build_glium().unwrap());
        let draw_params = DrawParameters {
            depth: Depth { test: DepthTest::IfLessOrEqual, write: true,..Default::default()},
            ..Default::default()
        };
        GliumRenderer {
            display: display.clone(),
            draw_params: draw_params,
            rect_buffer: None,
            circ_buffer: None,
            text_processor: TextProcessor::new(OPEN_SANS, 120, display),
        }
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    fn load_renderables(&mut self, renderables: Vec<Box<Renderable>>) {
        for renderable in renderables {
            match renderable.get_vertex() {
                RenderVertex::Rect(vertex) => load_vertex(vertex, &mut self.rect_buffer, renderable, &self.display),
                RenderVertex::Circ(vertex) => load_vertex(vertex, &mut self.circ_buffer, renderable, &self.display),
                RenderVertex::Text(text_box) => self.text_processor.push_text(text_box),
                _ => (),
            }
        }
    }

    fn render(&mut self) {

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);
        draw_at_target(&mut target, self.rect_buffer.take(), &self.display, &self.draw_params);
        draw_at_target(&mut target, self.circ_buffer.take(), &self.display, &self.draw_params);
        draw_text_at_target(&mut target, &mut self.text_processor);

        target.finish().unwrap();
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

fn load_vertex<T>(vertex: T, buffer: &mut Option<Buffer<T>>, renderable: Box<Renderable>, display: &GlutinFacade) {
    if let Some(ref mut buff) = *buffer {
        buff.push_vertex(vertex);
    }
    else {
        *buffer = Some(Buffer::new(
            make_program_from_shaders(renderable.get_shaders(), display),
            renderable.get_primitive_type()));
        if let Some(ref mut buffer) = *buffer { buffer.push_vertex(vertex);};
    };
}

fn draw_at_target<T: glium::vertex::Vertex>(target: &mut Frame, buffer: Option<Buffer<T>>, display: &GlutinFacade, draw_params: &DrawParameters) {
    if let Some(buffer) = buffer {
        let vertex_buffer = glium::VertexBuffer::new(display, &buffer.vertices).unwrap();
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(buffer.primitive_type),
                    &buffer.program,
                    &glium::uniforms::EmptyUniforms,
                    draw_params).unwrap();
    }
}

struct TextBuffer {
    pub buffer: Vec<Box<RenderText>>,
}

struct TextProcessor {
    text_objects: Option<TextBuffer>,
    txt_sys: glium_text::TextSystem,
    font_text: glium_text::FontTexture
}

impl TextProcessor {
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

    pub fn push_text(&mut self, text: Box<RenderText>) {
        if let Some(ref mut buffer) = self.text_objects {
            buffer.buffer.push(text);
        }
        else {
            self.text_objects = Some(TextBuffer{ buffer: vec![text]});
        }
    }
}

fn draw_text_at_target(target: &mut Frame, processor: &mut TextProcessor) {
    let buffer = processor.text_objects.take();
    if let Some(buffer) = buffer {
        for mut render_text in buffer.buffer {
            render_text.render(target, &processor.txt_sys, &processor.font_text);
        }
    }
}
