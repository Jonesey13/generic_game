use std::fmt;
use crate::geometry::*;
use crate::rendering::*;
use crate::collision::{ToCollisionObjects, CollisionObject};

#[derive(Clone)]
pub struct Circle{
    pub rad: f64,
    pub center: Point
}

impl Circle {
    pub fn new(rad: f64, center: Point) -> Circle {
        Circle{
            rad: rad,
            center: center
        }
    }

    pub fn shifted_by(&self, shift: Point) -> Circle {
        let mut out = self.clone();
        out.shift_by(shift);
        out
    }

    pub fn get_movement_line(&self, other: &Circle) -> Line {
        Line::new(self.center, other.center)
    }
}

impl TwoDTransformable for Circle {
    fn shift_by(&mut self, shift: Point) {
        self.center = self.center + shift;
    }

    fn rotate_at_center(&mut self, _: f64) {}

    fn rotate_at_origin(&mut self, rotation_angle: f64) {
        self.center = Rotation::new(rotation_angle) * self.center;
    }

    fn get_center(&self) -> Point {
        self.center
    }

    fn scale_by(&mut self, scale_factor: f64) {
        self.rad = self.rad * scale_factor;
    }
}

impl ToRenderables for Circle {
    fn to_renderables(&self, color: Color, depth: f64, fixed: bool) -> Vec<Box<StandardRenderable>> {
        vec![
            Box::new(CircleRenderable {
                radius: self.rad,
                pos: Point3::new(self.center.x, self.center.y, depth),
                color,
                fixed
            })
        ]
    }
}

impl Circle {
    pub fn render_collision_details(&self, coll_dir: Point, color: Color, depth: f64, fixed: bool) 
    -> Vec<Box<StandardRenderable>> {
        let coll_location = self.center + self.rad * coll_dir;
        let location_renderable: Box<dyn ToRenderables> = Box::new(coll_location);

        let direction_renderable: Box<StandardRenderable> = Box::new(
            Arrow::new_for_coll_test(
                    coll_location,
                    coll_dir,
                    color,
                    depth,
                    fixed
            )
        );

        let mut renderables = location_renderable.to_renderables(color, depth, fixed);
        renderables.push(direction_renderable);
        renderables
    }
}

impl ToCollisionObjects for Circle {
    fn to_collision_objects(&self) -> Vec<CollisionObject> {
        vec![
            CollisionObject::Circ(self.clone())
        ]
    }
}

impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle: radius: {}, center: {{ x: {}, y: {} }}", self.rad, self.center.x, self.center.y)
    }
}
