use super::Game;
use na::{Vector1, Vector2, Vector3, Vector4, Rotation2, Matrix2, Matrix4};
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
            rot: Rotation2::new(0.0),
            pos: Vector3::new(0.25, 0.25, 0.1),
            color: Vector4::new(0.0, 1.0, 0.0, 1.0)
        };
        let circ = Circle {
            radius: 0.25,
            pos: Vector3::new(-0.25, -0.25, 0.1),
            color: Vector4::new(1.0, 0.0, 0.0, 1.0)
        };
        let text = PlainText {
            content: "hello there!".to_string(),
            position: Vector2::new(0.0, 0.0),
            scale: Vector2::new(1.0, 1.0),
            transform: *Rotation2::new(1.0).matrix(),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0)
        };
        
        vec![Box::new(rect), Box::new(circ), Box::new(text)]
    }
}
