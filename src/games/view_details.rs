use na::{Vector2, Vector3};
use num::Zero;

pub enum ViewDetails {
    TwoDim(ViewDetails2D),
    ThreeDim(ViewDetails3D)
}

#[derive(Clone, Debug)]
pub struct ViewDetails2D {
    pub camera_pos: Vector2<f64>,
    pub up_vector: Vector2<f64>,
    pub viewport_height: f64
}

impl Default for ViewDetails2D {
    fn default() -> ViewDetails2D {
        ViewDetails2D {
            camera_pos: Vector2::zero(),
            up_vector: Vector2::y(),
            viewport_height: 1.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct ViewDetails3D {
    pub view_dir: Vector3<f64>, // Eye Direction
    pub up_vector: Vector3<f64>, // Vertical Direction
    pub eye_position: Vector3<f64>,
    pub scale: f64, // Height of view
}
