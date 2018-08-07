use std::fmt;
use super::{TwoDTransformable, ToRenderables};
use rendering::*;
use collision::{ToCollisionObjects, CollisionObject};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg, Div};
use ::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Matrix4 {
    pub xx: f64,
    pub xy: f64,
    pub xz: f64,
    pub xw: f64,
    pub yx: f64,
    pub yy: f64,
    pub yz: f64,
    pub yw: f64,
    pub zx: f64,
    pub zy: f64,
    pub zz: f64,
    pub zw: f64,
    pub wx: f64,
    pub wy: f64,
    pub wz: f64,
    pub ww: f64
}

impl Matrix4 {
    pub fn new(
        xx: f64,
        xy: f64,
        xz: f64,
        xw: f64,
        yx: f64,
        yy: f64,
        yz: f64,
        yw: f64,
        zx: f64,
        zy: f64,
        zz: f64,
        zw: f64,
        wx: f64,
        wy: f64,
        wz: f64,
        ww: f64
    ) -> Self {
        Self {
            xx,
            xy,
            xz,
            xw,
            yx,
            yy,
            yz,
            yw,
            zx,
            zy,
            zz,
            zw,
            wx,
            wy,
            wz,
            ww,
        }
    }

    pub fn one() -> Matrix4 {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn as_32_array(&self) -> [[f32; 4]; 4] {
        [
            [self.xx as f32, self.yx as f32, self.zx as f32, self.wx as f32],
            [self.xy as f32, self.yy as f32, self.zy as f32, self.wy as f32],
            [self.xz as f32, self.yz as f32, self.zz as f32, self.wz as f32],
            [self.xw as f32, self.yw as f32, self.zw as f32, self.ww as f32]
        ]
    }
}
