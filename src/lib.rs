#![allow(dead_code)]
#![allow(unused_imports)]


#[macro_use]
extern crate glium;
use time;
#[macro_use]
extern crate bitflags;
use unicode_normalization;
use rusttype;
#[macro_use]
extern crate lazy_static;

use winapi;
use image;

pub mod rendering;
pub mod input;
pub mod handler_basic;
pub mod games;
pub mod collision;
pub mod geometry;
pub mod utils;
pub mod debug;
pub mod window;
pub mod animation;

use crate::debug::*;
use std::env;
use std::io::*;

/// Handler
pub trait Handler {
    fn should_exit(&self) -> bool { false }
    fn on_exit(&mut self) {}
    fn init(&mut self) {}
    fn update_input(&mut self) {}
    fn update_logic(&mut self) {}
    fn update_rendering(&mut self) {}
}

#[allow(dead_code)]
pub struct HandlerStub;

impl Handler for HandlerStub{}
