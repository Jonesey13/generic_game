use super::BACKGROUND_LAYER;
use rendering::rectangle::Rectangle;
use na::{Vec1, Vec2, Vec3, Vec4, Rot2};
use num::Zero;

pub struct Board {
    pub length: f64,
    pub width: f64,
    pub color: Vec4<f64>,
}

impl Board {
    pub fn new(length: f64, width: f64, color: Vec4<f64>) -> Board {
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
            color: Vec4::zero(),
        }
    }
}

impl Board {
    pub fn render(&self) -> Rectangle {
        Rectangle {
            length: self.length,
            height: self.width,
            rot: Rot2::new(Vec1::zero()),
            pos: Vec3::new(0.0, 0.0, BACKGROUND_LAYER),
            color: self.color
        }
    }

}
