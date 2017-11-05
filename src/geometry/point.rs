use na::{Vector2, Vector3, Vector4};
use std::fmt;
use super::{TwoDTransformable, ToRenderable};
use rendering;
use collision;

#[derive(Clone, Debug)]
pub struct Point {
    pos: Vector2<f64>
}

impl Point {
    pub fn new(pos: Vector2<f64>) -> Point {
        Point {
            pos
        }
    }
}

impl TwoDTransformable for Point {
    fn shift_by(&mut self, shift: Vector2<f64>) {
        self.pos += shift;
    }

    fn rotate(&mut self, _: f64) {}
}

impl ToRenderable for Point {
    fn to_renderable(&self, color: Vector4<f64>, depth: f64, _: bool) -> Box<rendering::Renderable> {
        Box::new(rendering::Circle {
            radius: 0.01,
            pos: Vector3::new(self.pos.x, self.pos.y, depth),
            color
        })
    }
}

impl collision::CollObj for Point {
    fn get_object_pair(&self, other: &Self) -> collision::CollisionObjectState {
        collision::CollisionObjectState::Point(self.pos, other.pos)
    }

    fn render_collision_details(&self, collision_details: collision::CollisionDetails, colour: Vector4<f64>, depth: f64, fixed: bool) 
    -> Vec<Box<rendering::Renderable>> {
        match collision_details {
            collision::CollisionDetails::Point(dir) => vec![
                self.to_renderable(colour, depth, fixed),
                Box::new(rendering::Arrow::new_for_coll_test(
                    self.pos,
                    dir,
                    colour,
                    depth,
                    fixed
                ))],
            _ => return vec![]
        }
    }
}