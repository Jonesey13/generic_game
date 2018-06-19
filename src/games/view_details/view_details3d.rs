use na::{Vector3, Rotation};

#[derive(Copy, Clone, Debug)]
pub struct ViewDetails3D {
    pub view_dir: Vector3<f64>, // Eye Direction
    pub up_vector: Vector3<f64>, // Vertical Direction
    pub eye_position: Vector3<f64>,
    pub scale: f64, // Height of view
}
