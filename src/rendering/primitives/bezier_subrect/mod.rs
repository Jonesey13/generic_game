use na;
use na::{Vector2, Matrix2, Vector4, Rotation2};
use num::Zero;
use rendering::primitives::{Primitive, BezierRect, Rectangle};
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;
use glium;
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
    pub length: f64,
    pub width_left: f64,
    pub width_right: f64
}

impl BezierLogic {
    pub fn new (
        length: f64,
        width_left: f64,
        width_right: f64
    ) -> BezierLogic {
        BezierLogic {
            length,
            width_left,
            width_right,
        }
    }
}

impl GliumPrimitive for BezierSubrect {
    type Vertex = BezierSubrectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("bezier_subrect.vs"),
            include_str!("bezier_subrect.tcs"),
            include_str!("bezier_subrect.tes"),
            include_str!("bezier_subrect.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
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
    pub logic_length: f32,
    pub logic_width_left: f32,
    pub logic_width_right: f32,
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
    logic_length,
    logic_width_left,
    logic_width_right,
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
            logic_length: rect.logic.length as f32,
            logic_width_left: rect.logic.width_left as f32,
            logic_width_right: rect.logic.width_right as f32,
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