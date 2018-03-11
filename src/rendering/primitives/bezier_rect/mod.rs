use na;
use na::{Vector2, Vector3, Matrix2, Vector4, Rotation2};
use num::Zero;
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;
use glium;
use na::normalize;
use debug::*;

#[derive(Copy, Clone, Debug)]
pub struct BezierRect {
    pub control: BezierQuadControl,
    pub height: f64,
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>
}

#[derive(Copy, Clone, Debug)]
pub struct BezierQuadControl {
    pub one: Vector2<f64>,
    pub two: Vector2<f64>,
    pub three: Vector2<f64>
}

impl BezierQuadControl {
    pub fn new(one: Vector2<f64>, two: Vector2<f64>, three: Vector2<f64>) -> Self {
        Self {
            one,
            two,
            three
        }
    }

    pub fn new_linear(pos: Vector2<f64>, change: Vector2<f64>) -> Self {
        Self {
            one: pos,
            two: pos + change / 2.0,
            three: pos + change
        }
    }
}

impl BezierRect {
    /// Intended for Standalone use
    pub fn new_with_color (
        control_points: BezierQuadControl,
        height: f64,
        pos: Vector3<f64>,
        color: Vector4<f64>
    ) -> BezierRect {
        BezierRect { 
            control: control_points,
            height: height,
            pos: pos,
            color: color,
        }
    }

    /// Intended for Other Bezier Primitive Types
    pub fn new (
        control_points: BezierQuadControl,
        height: f64,
        pos: Vector3<f64>
    ) -> BezierRect {
        BezierRect { 
            control: control_points,
            height: height,
            pos: pos,
            color: Vector4::zero(),
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
    pub height: f32,
    pub pos: [f32; 3],
    pub color: [f32; 4]
}

implement_vertex!(BezierRectVertex, c0, c1, c2, height, pos, color);

impl From<BezierRect> for BezierRectVertex {
    fn from(rect: BezierRect) -> Self {
        //debug(&format!("{:?}", rect));
        let output = BezierRectVertex {
            c0: *na::convert::<_, Vector2<f32>>(rect.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(rect.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(rect.control.three).as_ref(),
            height: rect.height as f32,
            pos: *na::convert::<_, Vector3<f32>>(rect.pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(rect.color).as_ref(),
        };
        //debug(&format!("{:?}", output));
        output
    }
}
