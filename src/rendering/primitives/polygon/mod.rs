use crate::rendering::*;
use crate::geometry::*;

/// IMPORTANT: Must form a star domain at its center
#[derive(Clone, Debug)]
pub struct Polygon {
    pub corners: Vec<Point>, /// defined anti-clockwise
    pub center: Point,
    pub rot: Rotation,  /// anti-clockwise angle w.r.t. positive z-axis
    pub pos: Point3,
    pub color: Color,
    pub fixed: bool
}

impl Polygon {
    pub fn new_regular(
            corners: Vec<Point>,
            center: Point,
            pos: Point3, 
            color: Color,
            fixed: bool
        ) -> Polygon {
        Polygon {
            corners,
            center,
            rot: Rotation::new(0.0),
            pos,
            color,
            fixed,
        }
    }

    pub fn get_vertices(self) -> Vec<PolygonVertex> {
        let mut output: Vec<PolygonVertex> = vec![];
        let corners_it_shift = self.corners.clone().into_iter().cycle().skip(1);
        for (corner1, corner2) in self.corners.into_iter().zip(corners_it_shift) {
            output.push(PolygonVertex {
                corner1: corner1.into(),
                corner2: corner2.into(),
                center: self.center.into(),
                rot: self.rot.get_matrix_f32(),
                pos: self.pos.into(),
                color: self.color.get_array_f32(),
                fixed_pos: self.fixed as u32
            });
        }
        output
    }
}

impl GliumStandardPrimitive for Polygon {
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
