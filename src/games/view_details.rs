use na::{Vector2, Vector3, Rotation2};
use num::Zero;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub enum ViewDetails {
    TwoDim(ViewDetails2D),
    ThreeDim(ViewDetails3D)
}

#[derive(Copy, Clone, Debug)]
pub struct ViewDetails2D {
    pub camera_pos: Vector2<f64>,
    pub up_vector: Vector2<f64>,
    pub viewport_height: f64
}

impl ViewDetails2D {
    pub fn get_rotation_angle(&self) -> f64 {
        (self.up_vector.y).atan2(self.up_vector.x) - PI / 2.0
    }

    pub fn set_rotation_angle(&mut self, angle: f64) {
        let rot_mat = Rotation2::new(angle);
        self.up_vector = rot_mat * Vector2::y();
    }
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

#[derive(Copy, Clone, Debug)]
pub struct ViewDetails3D {
    pub view_dir: Vector3<f64>, // Eye Direction
    pub up_vector: Vector3<f64>, // Vertical Direction
    pub eye_position: Vector3<f64>,
    pub scale: f64, // Height of view
}
