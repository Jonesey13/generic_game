use super::Game;
use na::{Vec1, Vec2, Vec3, Vec4, Rot2, Mat2};
use num::{Zero, One} ;
use rendering::renderables::Renderable;
use rendering::rectangle::Rectangle;
use rendering::circle::Circle;
use rendering::text::PlainText;

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
        let text = PlainText {
            content: "omg!".to_string(),
            position: Vec2::zero(),
            scale: Vec2::new(100.0, 100.0),
            transform: *Rot2::new(Vec1::zero()).submat(),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0)
        };
        
        vec![Box::new(rect), Box::new(circ), Box::new(text)]
    }
}
