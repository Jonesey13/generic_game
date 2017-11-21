use na::{Vector2, Vector3, Vector4};
use std::fmt;
use super::{TwoDTransformable, ToRenderables};
use rendering;
use collision::{ToCollisionObjects, CollisionObject};

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

impl ToRenderables for Point {
    fn to_renderables(&self, colour: Vector4<f64>, depth: f64, _: bool) -> Vec<Box<rendering::Renderable>> {
        vec![
            Box::new(rendering::Circle {
                radius: 0.01,
                pos: Vector3::new(self.pos.x, self.pos.y, depth),
                colour
            })
        ]
    }
}

impl ToCollisionObjects for Point {
    fn to_collision_objects(&self) -> Vec<CollisionObject> {
        vec![
            CollisionObject::Point(self.pos)
        ]
    }
}

impl Point {
    pub fn render_collision_details(&self, coll_dir: Vector2<f64>, colour: Vector4<f64>, depth: f64, fixed: bool) 
    -> Vec<Box<rendering::Renderable>> {
        let mut renderables = self.to_renderables(colour, depth, fixed);

        renderables.push(
            Box::new(rendering::Arrow::new_for_coll_test(
                self.pos,
                coll_dir,
                colour,
                depth,
                fixed
        )));

        renderables
    }
}