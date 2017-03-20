use super::BACKGROUND_LAYER;
use rendering::rectangle::Rectangle;
use na::{Vector1, Vector2, Vector3, Vector4, Rotation2};
use num::Zero;

pub struct Board {
    pub length: f64,
    pub width: f64,
    pub color: Vector4<f64>,
}

impl Board {
    pub fn new(length: f64, width: f64, color: Vector4<f64>) -> Board {
        Board {
            length: length,
            width: width,
            color: color,
        }
    }

    pub fn no_board() -> Board {
        Board {
            length: 0.0,
            width: 0.0,
            color: Vector4::zero(),
        }
    }
}

impl Board {
    pub fn render(&self) -> Rectangle {
        Rectangle {
            length: self.length,
            height: self.width,
            rot: Rotation2::new(0.0),
            pos: Vector3::new(0.0, 0.0, BACKGROUND_LAYER),
            color: self.color
        }
    }

}
