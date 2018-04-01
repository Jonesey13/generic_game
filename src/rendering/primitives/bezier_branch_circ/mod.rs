use na;
use na::{Vector2, Vector3, Matrix2, Vector4, Rotation2};
use num::Zero;
use rendering::render_by_shaders::GliumStandardPrimitive;
use rendering::shaders::Shaders;
use glium;
use na::normalize;
use debug::*;
use super::{BezierRect};

#[derive(Copy, Clone, Debug)]
pub struct BezierBranchCirc {
    pub branch: BezierRect,
    pub radius: f64,
    pub branch_logical_length: f64,
    pub branch_logical_height_left: f64,
    pub branch_logical_height_right: f64,
    pub logical_pos: Vector3<f64>,
    pub color: Vector4<f64>
}

impl BezierBranchCirc {
    pub fn new (
        branch: BezierRect,
        radius: f64,
        branch_logical_length: f64,
        branch_logical_height_left: f64,
        branch_logical_height_right: f64,
        logical_pos: Vector3<f64>,
        color: Vector4<f64>
    ) -> BezierBranchCirc {
        BezierBranchCirc { 
            branch,
            radius,
            branch_logical_length,
            branch_logical_height_left,
            branch_logical_height_right,
            logical_pos,
            color: color,
        }
    }
}

impl GliumStandardPrimitive for BezierBranchCirc {
    type Vertex = BezierBranchCircVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexTesselationFragment(
            include_str!("bezier_branch_circ.vs"),
            include_str!("bezier_branch_circ.tcs"),
            include_str!("bezier_branch_circ.tes"),
            include_str!("bezier_branch_circ.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }

    fn get_primitive_type() -> glium::index::PrimitiveType {
        glium::index::PrimitiveType::Patches{ vertices_per_patch: 1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BezierBranchCircVertex {
    pub c0: [f32; 2],
    pub c1: [f32; 2],
    pub c2: [f32; 2],
    pub branch_height: f32,
    pub branch_pos: [f32; 3],
    pub radius: f32,
    pub branch_logical_length: f32,
    pub branch_logical_height_left: f32,
    pub branch_logical_height_right: f32,
    pub logical_pos: [f32; 3],
    pub color: [f32; 4]
}

implement_vertex!(
    BezierBranchCircVertex, 
    c0, 
    c1, 
    c2, 
    branch_height, 
    branch_pos,
    radius,
    branch_logical_length,
    branch_logical_height_left,
    branch_logical_height_right,
    logical_pos, 
    color
);

impl From<BezierBranchCirc> for BezierBranchCircVertex {
    fn from(circ: BezierBranchCirc) -> Self {
        let output = BezierBranchCircVertex {
            c0: *na::convert::<_, Vector2<f32>>(circ.branch.control.one).as_ref(),
            c1: *na::convert::<_, Vector2<f32>>(circ.branch.control.two).as_ref(),
            c2: *na::convert::<_, Vector2<f32>>(circ.branch.control.three).as_ref(),
            branch_height: circ.branch.height as f32,
            branch_pos: *na::convert::<_, Vector3<f32>>(circ.branch.pos).as_ref(),
            radius: circ.radius as f32,
            branch_logical_length: circ.branch_logical_length as f32,
            branch_logical_height_left: circ.branch_logical_height_left as f32,
            branch_logical_height_right: circ.branch_logical_height_right as f32,
            logical_pos: *na::convert::<_, Vector3<f32>>(circ.logical_pos).as_ref(),
            color: *na::convert::<_, Vector4<f32>>(circ.color).as_ref(),
        };
        output
    }
}
