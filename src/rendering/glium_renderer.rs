use super::Renderer;
use super::shaders::make_program_from_shaders;
use super::rectangle::{Rectangle, RectangleVertex};
use super::circle::{Circle, CircleVertex};
use super::text::{RenderText, TextBuffer, PlainText};
use super::polar_pixel::{PolarBuffer, PolarPixel, PolarPixelVertex};
use super::glium_buffer::{GliumBuffer, BasicBuffer};
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
use games::view_details;
use utils::transforms_2d;
use debug::*;

pub struct GliumRenderer<'a> {
    display: Box<GlutinFacade>,
    draw_params: DrawParameters<'a>,
    rect_buffer: BasicBuffer<Rectangle>,
    circ_buffer: BasicBuffer<Circle>,
    polar_buffer: PolarBuffer,
    text_processor: TextBuffer<'a, PlainText>,
    view_details: view_details::ViewDetails,
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
            rect_buffer: BasicBuffer::<Rectangle>::new(&display),
            circ_buffer: BasicBuffer::<Circle>::new(&display),
            polar_buffer: PolarBuffer::new(&display),
            text_processor: TextBuffer::new(display),
            view_details: view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default())
        }
    }

    fn flush_buffers(&mut self) {
        self.rect_buffer.flush_buffer();
        self.circ_buffer.flush_buffer();
        self.polar_buffer.flush_buffer();
        self.text_processor.flush_buffer();
    }
    
    pub fn create_worldview_mat(view_details: view_details::ViewDetails, aspect_ratio: f64) ->  [[f32;4]; 4] {
        let view_mat = match
            view_details {
                view_details::ViewDetails::TwoDim(ref view) =>
                transforms_2d::build_worldview_mat(
                    view.camera_pos,
                    view.viewport_height,
                    aspect_ratio,
                    view.up_vector),
                view_details::ViewDetails::ThreeDim(_) => panic!("3D mode not supported!"),
                _ => Matrix4::one()
        };
        let single_mat: Matrix4<f32> = na::convert(view_mat);
        *single_mat.as_ref()
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    fn load_renderables(&mut self, renderables: Vec<Box<Renderable>>) {
        debug_clock_start("Render::glium_load");
        for renderable in renderables {
            match renderable.get_type() {
                RenderType::Rect(rectangle) => self.rect_buffer.load_renderable(rectangle),
                RenderType::Circ(circle) => self.circ_buffer.load_renderable(circle),
                RenderType::Text(text) => self.text_processor.load_renderable(text),
                RenderType::PolarPix(polar) => self.polar_buffer.load_renderable(polar)
            }
        }
        debug_clock_stop("Render::glium_load");
    }

    fn render(&mut self) {
        debug_clock_start("Render::glium_render");
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f64 / height as f64;
        let uniforms = uniform! {
            screen_width: width,
            screen_height: height,
            aspect_ratio: aspect_ratio as f32,
            world_view: GliumRenderer::create_worldview_mat(self.view_details, aspect_ratio)
        };
        
        self.rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.circ_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.polar_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.text_processor.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        target.finish().unwrap();
        self.flush_buffers();
        debug_clock_stop("Render::glium_render");
    }

    fn set_worldview(&mut self, view_details: view_details::ViewDetails) {
        self.view_details = view_details;
    }
}

