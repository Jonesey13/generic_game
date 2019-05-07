use std::fmt;
use super::{TwoDTransformable, ToRenderables};
use crate::rendering::*;
use crate::collision::{ToCollisionObjects, CollisionObject};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg, Div};
use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix3 {
    pub xx: f64,
    pub xy: f64,
    pub xz: f64,
    pub yx: f64,
    pub yy: f64,
    pub yz: f64,
    pub zx: f64,
    pub zy: f64,
    pub zz: f64
}

impl Matrix3 {
    pub fn new(
        xx: f64,
        xy: f64,
        xz: f64,
        yx: f64,
        yy: f64,
        yz: f64,
        zx: f64,
        zy: f64,
        zz: f64
    ) -> Self {
        Self {
            xx,
            xy,
            xz,
            yx,
            yy,
            yz,
            zx,
            zy,
            zz,
        }
    }

    pub fn from_array(arr: [[f64; 3]; 3]) -> Self {
        Self {
            xx: arr[0][0],
            xy: arr[1][0],
            xz: arr[2][0],
            yx: arr[0][1],
            yy: arr[1][1],
            yz: arr[2][1],
            zx: arr[0][2],
            zy: arr[1][2],
            zz: arr[2][2]
        }
    }

    pub fn from_columns(first: Point3, second: Point3, third: Point3) -> Self {
        Self {
            xx: first.x,
            xy: second.x,
            xz: third.x,
            yx: first.y,
            yy: second.y,
            yz: third.y,
            zx: first.z,
            zy: second.z,
            zz: third.z
        }
    }

    pub fn to_columns(&self) -> (Point3, Point3, Point3) {
        (Point3::new(self.xx, self.yx, self.zx),
         Point3::new(self.xy, self.yy, self.zy),
         Point3::new(self.xz, self.yz, self.zz))
    }
}

impl Mul<Point3> for Matrix3 {
    type Output = Point3;

    fn mul(self, point: Point3) -> Point3 {
        Point3 {
            x: self.xx * point.x + self.xy * point.y + self.xz * point.z,
            y: self.yx * point.x + self.yy * point.y + self.yz * point.z,
            z: self.zx * point.x + self.zy * point.y + self.zz * point.z
        }
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, mat: Matrix3) -> Matrix3 {
        let (column1, column2, column3) = mat.to_columns(); 
        let first_column = self * column1;
        let second_column = self * column2;
        let third_column = self * column3;
        Matrix3::from_columns(first_column, second_column, third_column)
    }
}