use std::fmt;
use super::{TwoDTransformable, ToRenderables};
use crate::rendering::*;
use crate::collision::{ToCollisionObjects, CollisionObject};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg, Div};
use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn norm(&self) -> f64 {
        return f64::sqrt(self.norm_squared());
    }

    pub fn norm_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn normalized(&self) -> Self {
        (1.0 / self.norm()) * self
    }

    pub fn dot(&self, other: &Point3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point3) -> Point3 {
        Point3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn interpolate(&self, other: &Self, point: f64) -> Self {
        (1.0 - point) * self + point * other
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, point: Point3) -> Point3 {
        Point3 {
            x: self * point.x,
            y: self * point.y,
            z: self * point.z
        }
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, scale: f64) -> Point3 {
        Point3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale
        }
    }
}

impl<'a> Mul<&'a Point3> for f64 {
    type Output = Point3;

    fn mul(self, point: &'a Point3) -> Point3 {
        Point3 {
            x: self * point.x,
            y: self * point.y,
            z: self * point.z
        }
    }
}

impl Mul<Point3> for Point3 {
    type Output = Point3;

    fn mul(self, point: Point3) -> Point3 {
        Point3 {
            x: self.x * point.x,
            y: self.y * point.y,
            z: self.z * point.z
        }
    }
}

impl<'a> Mul<&'a Point3> for Point3 {
    type Output = Point3;

    fn mul(self, point: &'a Point3) -> Point3 {
        Point3 {
            x: self.x * point.x,
            y: self.y * point.y,
            z: self.z * point.z
        }
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Self {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a> Add<&'a Point3> for Point3 {
    type Output = Point3;

    fn add(self, other: &'a Point3) -> Self {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign<Point3> for Point3 {
    fn add_assign(&mut self, other: Point3){
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Self {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<'a> Sub<&'a Point3> for Point3 {
    type Output = Point3;

    fn sub(self, other: &'a Point3) -> Self {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl SubAssign<Point3> for Point3 {
    fn sub_assign(&mut self, other: Point3){
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self {
        Self{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<[f64; 3]> for Point3 {
    fn from(arr: [f64; 3]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2]
        }
    }
}

impl From<Point3> for [f64; 3] {
    fn from(point: Point3) -> Self {
        [point.x, point.y, point.z]
    }
}

impl From<Point3> for [f32; 3] {
    fn from(point: Point3) -> Self {
        [point.x as f32, point.y as f32, point.z as f32]
    }
}