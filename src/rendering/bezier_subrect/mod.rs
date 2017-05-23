use na;
use na::{Vector2, Matrix2, Vector4, Rotation2};
use num::Zero;
use super::renderables::{Renderable, RenderType, BezierRect, Rectangle};
use super::render_by_shaders::GliumRenderable;
use super::shaders::Shaders;
use glium;
use glium::index::PrimitiveType;
use super::conversion_tools::*;
use na::normalize;
use debug::*;

#[derive(Copy, Clone, Debug)]
pub struct BezierSubrect {
    pub bezier: BezierRect,
    pub logic: BezierLogic,
    pub length: f64,
    pub height: f64,
    pub sub_pos: Vector2<f64>,
    pub color: Vector4<f64>
}

impl BezierSubrect {
    pub fn new (
        bezier: BezierRect,
        logic: BezierLogic,
        length: f64,
        height: f64,
        sub_pos: Vector2<f64>,
        color: Vector4<f64>
    ) -> BezierSubrect {
        BezierSubrect {
            bezier,
            logic,
            length,
            height,
            sub_pos,
            color
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BezierLogic {
    pub length_left: f64,
    pub length_right: f64,
    pub height_left: f64,
    pub height_right: f64
}

impl BezierLogic {
    pub fn new (
        length_left: f64,
        length_right: f64,
        height_left: f64,
        height_right: f64
    ) -> BezierLogic {
        BezierLogic {
            length_left,
            length_right,
            height_left,
            height_right,
        }
    }
}

impl Renderable for BezierSubrect {
    fn get_type(&self) -> RenderType { RenderType::BezierSubrect(self.clone()) }
}

impl GliumRenderable for BezierSubrect {
    type Vertex = BezierSubrectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("bezier_subrect.vs"),
            include_str!("bezier_subrect.tcs"),
            include_str!("bezier_subrect.tes"),
            include_str!("bezier_subrect.fs"))
    }

    fn get_vertex(&self) -> Self::Vertex { self.clone().into() }

    fn get_primitive_type() -> PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BezierSubrectVertex {
    pub c0: [f32; 2],
    pub c1: [f32; 2],
    pub c2: [f32; 2],
    pub vert_dir: [f32; 2], 
    pub bezier_width: f32,
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub logic_length_left: f32,
    pub logic_length_right: f32,
    pub logic_height_left: f32,
    pub logic_height_right: f32,
    pub length: f32,
    pub height: f32,
    pub sub_pos: [f32; 2]
}

implement_vertex!(
    BezierSubrectVertex,
    c0,
    c1,
    c2,
    vert_dir,
    bezier_width,
    pos,
    color,
    logic_length_left,
    logic_length_right,
    logic_height_left,
    logic_height_right,
    length,
    height,
    sub_pos,
);

impl From<BezierSubrect> for BezierSubrectVertex {
    fn from(rect: BezierSubrect) -> Self {
        //debug(&format!("{:?}", rect));
        let output = BezierSubrectVertex {
            c0: *na::convert::<_, Vector2<f32>>(rect.bezier.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(rect.bezier.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(rect.bezier.control.three).as_ref(),
            vert_dir: *na::convert::<_, Vector2<f32>>(rect.bezier.vert_dir).as_ref(),
            bezier_width: rect.bezier.width as f32,
            logic_length_left: rect.logic.length_left as f32,
            logic_length_right: rect.logic.length_right as f32,
            logic_height_left: rect.logic.height_left as f32,
            logic_height_right: rect.logic.height_right as f32,
            pos: *na::convert::<_, Vector2<f32>>(rect.bezier.pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(rect.color).as_ref(),
            length: rect.length as f32,
            height: rect.height as f32,
            sub_pos: *na::convert::<_, Vector2<f32>>(rect.sub_pos).as_ref(),
        };
        //debug(&format!("{:?}", output));
        output
    }
}
