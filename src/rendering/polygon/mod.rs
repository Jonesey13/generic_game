use na::{Vector1, Vector2, Vector3, Vector4, Rotation2, convert};
use num::Zero;
use super::renderables::{Renderable, RenderType};
use super::render_by_shaders::GliumRenderable;
use super::shaders::Shaders;
use super::conversion_tools::*;

/// IMPORTANT: Must form a star domain at its center
#[derive(Clone)]
pub struct Polygon {
    pub corners: Vec<Vector2<f64>>, /// defined anti-clockwise
    pub center: Vector2<f64>,
    pub rot: Rotation2<f64>,  /// anti-clockwise angle w.r.t. positive z-axis
    pub pos: Vector3<f64>,
    pub color: Vector4<f64>,
    pub fixed: bool
}

impl Polygon {
    pub fn new_regular(
            corners: Vec<Vector2<f64>>,
            center: Vector2<f64>,
            pos: Vector3<f64>, 
            color: Vector4<f64>
        ) -> Polygon {
        Polygon {
            corners,
            center,
            rot: Rotation2::new(0.0),
            pos,
            color,
            fixed: false
        }
    }

    pub fn new_regular_fixed(
            corners: Vec<Vector2<f64>>,
            center: Vector2<f64>,            
            pos: Vector3<f64>, 
            color: Vector4<f64>
        ) -> Polygon {
        Polygon {
            corners,
            center,
            rot: Rotation2::new(0.0),
            pos,
            color,
            fixed: true
        }
    }

    pub fn get_vertices(self) -> Vec<PolygonVertex> {
        let mut output: Vec<PolygonVertex> = vec![];
        let corners_it_shift = self.corners.clone().into_iter().cycle().skip(1);
        for (corner1, corner2) in self.corners.into_iter().zip(corners_it_shift) {
            output.push(PolygonVertex {
                corner1: *convert::<_, Vector2<f32>>(corner1).as_ref(),
                corner2: *convert::<_, Vector2<f32>>(corner2).as_ref(),
                center: *convert::<_, Vector2<f32>>(self.center).as_ref(),
                rot: mat2_64_to_32(*self.rot.matrix().as_ref()),
                pos: vec3_64_to_32(*self.pos.as_ref()),
                color: vec4_64_to_32(*self.color.as_ref()),
                fixed_pos: self.fixed as u32
            });
        }
        output
    }
}

impl Renderable for Polygon {
    fn get_type(&self) -> RenderType { RenderType::Poly(self.clone()) }
}

impl GliumRenderable for Polygon {
    type Vertex = PolygonVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("polygon.vs"),
            include_str!("polygon.ges"),
            include_str!("polygon.fs")
        )
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { self.get_vertices() }
}

#[derive(Copy, Clone, Debug)]
pub struct PolygonVertex {
    pub corner1: [f32; 2],
    pub corner2: [f32; 2],
    pub center: [f32; 2],
    pub rot: [[f32; 2]; 2],
    pub pos: [f32; 3],
    pub color: [f32; 4],
    pub fixed_pos: u32
}

implement_vertex!(PolygonVertex, corner1, corner2, center, rot, pos, color, fixed_pos);
