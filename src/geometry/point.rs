use na::{Vector2, Vector3, Vector4, Rotation2};
use std::fmt;
use super::{TwoDTransformable, ToRenderables};
use rendering;
use collision::{ToCollisionObjects, CollisionObject};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn norm(&self) -> f64 {
        return f64::sqrt(self.norm_squared());
    }

    pub fn norm_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn normalized(&self) -> Self {
        (1.0 / self.norm()) * self
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn zero() -> Point {
        Point::new(0.0, 0.0)
    }

    pub fn x() -> Point {
        Self::new(1.0, 0.0)
    }

    pub fn y() -> Point {
        Self::new(0.0, 1.0)
    }
}

impl Mul<Point> for Rotation2<f64> {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        let vector = Vector2::new(point.x, point.y);

        let rotated_vector = self * vector;
        Point::new(rotated_vector.x, rotated_vector.y)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        Point {
            x: self * point.x,
            y: self * point.y
        }
    }
}

impl<'a> Mul<&'a Point> for f64 {
    type Output = Point;

    fn mul(self, point: &'a Point) -> Point {
        Point {
            x: self * point.x,
            y: self * point.y
        }
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        Point {
            x: self.x * point.x,
            y: self.y * point.y
        }
    }
}

impl<'a> Mul<&'a Point> for Point {
    type Output = Point;

    fn mul(self, point: &'a Point) -> Point {
        Point {
            x: self.x * point.x,
            y: self.y * point.y
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point){
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<'a> Sub<&'a Point> for Point {
    type Output = Point;

    fn sub(self, other: &'a Point) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point){
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self {
        Self{
            x: -self.x,
            y: -self.y
        }
    }
}

impl TwoDTransformable for Point {
    fn shift_by(&mut self, shift: Point) {
        *self += shift;
    }

    fn rotate(&mut self, _: f64) {}
}

impl ToRenderables for Point {
    fn to_renderables(&self, color: Vector4<f64>, depth: f64, fixed: bool) -> Vec<Box<rendering::StandardRenderable>> {
        vec![
            Box::new(rendering::Circle {
                radius: 0.01,
                pos: Vector3::new(self.x, self.y, depth),
                color,
                fixed
            })
        ]
    }
}

impl ToCollisionObjects for Point {
    fn to_collision_objects(&self) -> Vec<CollisionObject> {
        vec![
            CollisionObject::Point(*self)
        ]
    }
}

impl Point {
    pub fn render_collision_details(&self, coll_dir: Point, color: Vector4<f64>, depth: f64, fixed: bool) 
    -> Vec<Box<rendering::StandardRenderable>> {
        let mut renderables = self.to_renderables(color, depth, fixed);

        renderables.push(
            Box::new(rendering::Arrow::new_for_coll_test(
                *self,
                coll_dir,
                color,
                depth,
                fixed
        )));

        renderables
    }
}

impl From<[f64; 2]> for Point {
    fn from(arr: [f64; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1]
        }
    }
}

impl From<Point> for [f64; 2] {
    fn from(point: Point) -> Self {
        [point.x, point.y]
    }
}

impl From<Point> for [f32; 2] {
    fn from(point: Point) -> Self {
        [point.x as f32, point.y as f32]
    }
}