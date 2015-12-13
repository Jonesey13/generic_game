use na::Vec3;
use num::Zero;
use glium::Program;
use super::renderables::Renderable;

static RECTANGLE_VERTEX_SHADER: &'static str = include_str!("rectangle.vs");
static RECTANGLE_GEOMETRY_SHADER: &'static str = include_str!("rectangle.ges");
static RECTANGLE_FRAGMENT_SHADER: &'static str = include_str!("rectangle.fs");

pub struct Rectangle {
    length: f64,  /// x-axis
    height: f64,  /// y-axis
    rot: f64,  /// anti-clockwise angle w.r.t. positive z-axis
    pos: [f64; 3],
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            length: f64,  /// x-axis
            height: f64,  /// y-axis
            rot: f64,  /// anti-clockwise angle w.r.t. positive z-axis
            pos: [f64; 3],
        }
    }

}

impl Renderable for Rectangle {
    fn get_shaders(&self) -> Shaders { Shaders::None }
}
