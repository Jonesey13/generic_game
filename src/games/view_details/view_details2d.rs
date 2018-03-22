use na::{Vector2, Rotation2};
use num::Zero;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct ViewDetails2D {
    pub camera_pos: Vector2<f64>,
    pub up_vector: Vector2<f64>,
    pub viewport_height: f64,
    pub viewport_length: f64,
    pub use_aspect_ratio: bool
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
            viewport_height: 1.0,
            viewport_length: 1.0,
            use_aspect_ratio: true
        }
    }
}
