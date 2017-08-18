use super::Renderer;
use super::shaders::make_program_from_shaders;
use rendering::primitives::rectangle::{Rectangle, RectangleVertex};
use rendering::primitives::circle::{Circle, CircleVertex};
use rendering::primitives::polygon::{Polygon, PolygonVertex};
use rendering::primitives::text::{RenderText, TextBuffer, PlainText};
use rendering::primitives::polar_pixel::{PolarBuffer, PolarPixel, PolarPixelVertex};
use rendering::primitives::Primitive; 
use rendering::renderables::Renderable;
use super::glium_buffer::{GliumBuffer, BasicBuffer};
use super::render_by_shaders::GliumPrimitive;
use super::{BezierRect, BezierSubrect};
use glium;
use glium::Frame;
use glium::backend::glutin_backend::GlutinFacade;
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
    polygon_buffer: BasicBuffer<Polygon>,
    bezier_rect_buffer: BasicBuffer<BezierRect>,
    bezier_subrect_buffer: BasicBuffer<BezierSubrect>,
    polar_buffer: PolarBuffer,
    text_processor: TextBuffer<'a, PlainText>,
    view_details: view_details::ViewDetails,
}

impl<'a> GliumRenderer<'a> {
    pub fn new(res: (u32, u32)) -> GliumRenderer<'a> {
        let display = Box::new(glium::glutin::WindowBuilder::new()
                               //.with_fullscreen(glium::glutin::get_primary_monitor())
                               .with_dimensions(res.0, res.1)
                               .with_multisampling(2)
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
            polygon_buffer: BasicBuffer::<Polygon>::new(&display),
            bezier_rect_buffer: BasicBuffer::<BezierRect>::new(&display),
            bezier_subrect_buffer: BasicBuffer::<BezierSubrect>::new(&display),
            polar_buffer: PolarBuffer::new(&display),
            text_processor: TextBuffer::new(display),
            view_details: view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default())
        }
    }

    fn flush_buffers(&mut self) {
        self.rect_buffer.flush_buffer();
        self.circ_buffer.flush_buffer();
        self.polar_buffer.flush_buffer();
        self.polygon_buffer.flush_buffer();
        self.bezier_rect_buffer.flush_buffer();
        self.bezier_subrect_buffer.flush_buffer();
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
                    view.up_vector,
                    view.use_aspect_ratio),
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
        for mut renderable in renderables {
            for primitive in renderable.get_primitives() {
                match primitive {
                        Primitive::Rect(rectangle) => self.rect_buffer.load_renderable(rectangle),
                        Primitive::Circ(circle) => self.circ_buffer.load_renderable(circle),
                        Primitive::Text(text) => self.text_processor.load_renderable(text),
                        Primitive::BezierRect(bezier_rect) => self.bezier_rect_buffer.load_renderable(bezier_rect),
                        Primitive::BezierSubrect(bezier_subrect) => self.bezier_subrect_buffer.load_renderable(bezier_subrect),
                        Primitive::PolarPix(polar) => self.polar_buffer.load_renderable(polar),
                        Primitive::Poly(polygon) => self.polygon_buffer.load_renderable(polygon)
                }
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
        self.polygon_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);        
        self.bezier_rect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.bezier_subrect_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.polar_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        self.text_processor.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
        target.finish().unwrap();
        self.flush_buffers();
        debug_clock_stop("Render::glium_render");
    }

    fn set_worldview(&mut self, view_details: view_details::ViewDetails) {
        self.view_details = view_details;
    }

    fn get_glutin_window(&mut self) -> Option<&mut GlutinFacade> {
        Some(&mut self.display)
    }

    fn get_window_spec(&self) -> super::WindowSpec {
        let (width, height) = self.display.get_window().unwrap().get_inner_size().unwrap();

        super::WindowSpec {
            aspect_ratio: width as f64 / height as f64
        }
    }
}

