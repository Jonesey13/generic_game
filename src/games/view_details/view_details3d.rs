use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct ViewDetails3D {
    pub view_dir: Point3, // Eye Direction
    pub up_vector: Point3, // Vertical Direction
    pub eye_position: Point3,
    pub scale: f64, // Height of view
}
