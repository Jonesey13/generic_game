use super::Game;
use na::{Vec1, Vec2, Vec3, Vec4, Rot2};
use num::Zero;
use rendering::renderables::Renderable;
use rendering::rectangle::Rectangle;
use rendering::circle::Circle;

#[allow(dead_code)]
pub struct PrimitiveTestGame;

impl Game for PrimitiveTestGame {
    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let rect = Rectangle {
            length: 0.5,
            height: 0.5,
            rot: Rot2::new(Vec1::zero()),
            pos: Vec3::new(0.25, 0.25, 0.1),
            color: Vec4::new(0.0, 1.0, 0.0, 1.0)
        };
        let circ = Circle {
            radius: 0.25,
            pos: Vec3::new(-0.25, -0.25, 0.1),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0)
        };
        vec![Box::new(rect), Box::new(circ)]
    }
}
