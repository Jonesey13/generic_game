use na;
use na::{Vector2, Vector3, Matrix2, Vector4, Rotation2};
use num::Zero;
use rendering::render_by_shaders::GliumPrimitive;
use rendering::shaders::Shaders;
use glium;
use na::normalize;
use debug::*;
use super::{BezierRect};

#[derive(Copy, Clone, Debug)]
pub struct BezierBranchRect {
    pub branch: BezierRect,
    pub width: f64,
    pub height: f64,
    pub branch_logical_length: f64,
    pub branch_logical_height_left: f64,
    pub branch_logical_height_right: f64,
    pub logical_pos: Vector3<f64>,
    pub color: Vector4<f64>
}

impl BezierBranchRect {
    pub fn new (
        branch: BezierRect,
        width: f64,
        height: f64,
        branch_logical_length: f64,
        branch_logical_height_left: f64,
        branch_logical_height_right: f64,
        logical_pos: Vector3<f64>,
        color: Vector4<f64>
    ) -> BezierBranchRect {
        BezierBranchRect { 
            branch,
            width,
            height,
            branch_logical_length,
            branch_logical_height_left,
            branch_logical_height_right,
            logical_pos,
            color: color,
        }
    }
}

impl GliumPrimitive for BezierBranchRect {
    type Vertex = BezierBranchRectVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("bezier_branch_rect.vs"),
            include_str!("bezier_branch_rect.tcs"),
            include_str!("bezier_branch_rect.tes"),
            include_str!("bezier_branch_rect.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BezierBranchRectVertex {
    pub c0: [f32; 2],
    pub c1: [f32; 2],
    pub c2: [f32; 2],
    pub branch_height: f32,
    pub branch_pos: [f32; 3],
    pub width: f32,
    pub height: f32,
    pub branch_logical_length: f32,
    pub branch_logical_height_left: f32,
    pub branch_logical_height_right: f32,
    pub logical_pos: [f32; 3],
    pub color: [f32; 4]
}

implement_vertex!(
    BezierBranchRectVertex, 
    c0, 
    c1, 
    c2, 
    branch_height, 
    branch_pos,
    width,
    height,
    branch_logical_length,
    branch_logical_height_left,
    branch_logical_height_right,
    logical_pos, 
    color
);

impl From<BezierBranchRect> for BezierBranchRectVertex {
    fn from(rect: BezierBranchRect) -> Self {
        let output = BezierBranchRectVertex {
            c0: *na::convert::<_, Vector2<f32>>(rect.branch.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(rect.branch.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(rect.branch.control.three).as_ref(),
            branch_height: rect.branch.height as f32,
            branch_pos: *na::convert::<_, Vector3<f32>>(rect.branch.pos).as_ref(),
            width: rect.width as f32,
            height: rect.height as f32,
            branch_logical_length: rect.branch_logical_length as f32,
            branch_logical_height_left: rect.branch_logical_height_left as f32,
            branch_logical_height_right: rect.branch_logical_height_right as f32,
            logical_pos: *na::convert::<_, Vector3<f32>>(rect.logical_pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(rect.color).as_ref(),
        };
        output
    }
}
