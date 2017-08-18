use na;
use na::{Vector2, Matrix2, Vector4, Rotation2};
use num::Zero;
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;
use glium;
use na::normalize;
use debug::*;

#[derive(Copy, Clone, Debug)]
pub struct BezierRect {
    pub control: BezierQuadControl,
    pub vert_dir: Vector2<f64>,
    pub width: f64,
    pub pos: Vector2<f64>,
    pub color: Vector4<f64>
}

#[derive(Copy, Clone, Debug)]
pub struct BezierQuadControl {
    pub one: Vector2<f64>,
    pub two: Vector2<f64>,
    pub three: Vector2<f64>
}

impl BezierRect {
    pub fn new (
        control_points: BezierQuadControl,
        vert_dir: Vector2<f64>,
        width: f64,
        pos: Vector2<f64>,
        color: Vector4<f64>
    ) -> BezierRect {
        let normalised_vert = normalize(&vert_dir);
        
        BezierRect { 
            control: control_points,
            vert_dir: normalised_vert,
            width: width,
            pos: pos,
            color: color,
        }
    }
}

impl GliumPrimitive for BezierRect {
    type Vertex = BezierRectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("bezier_rect.vs"),
            include_str!("bezier_rect.tcs"),
            include_str!("bezier_rect.tes"),
            include_str!("bezier_rect.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BezierRectVertex {
    pub c0: [f32; 2],
    pub c1: [f32; 2],
    pub c2: [f32; 2],
    pub vert_dir: [f32; 2], 
    pub width: f32,
    pub pos: [f32; 2],
    pub color: [f32; 4]
}

implement_vertex!(BezierRectVertex, c0, c1, c2, vert_dir, width, pos, color);

impl From<BezierRect> for BezierRectVertex {
    fn from(rect: BezierRect) -> Self {
        //debug(&format!("{:?}", rect));
        let output = BezierRectVertex {
            c0: *na::convert::<_, Vector2<f32>>(rect.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(rect.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(rect.control.three).as_ref(),
            vert_dir: *na::convert::<_, Vector2<f32>>(rect.vert_dir).as_ref(),
            width: rect.width as f32,
            pos: *na::convert::<_, Vector2<f32>>(rect.pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(rect.color).as_ref(),
        };
        //debug(&format!("{:?}", output));
        output
    }
}
