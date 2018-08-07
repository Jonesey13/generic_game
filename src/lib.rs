#![allow(dead_code)]
#![allow(unused_imports)]

extern crate multiinput;
#[macro_use]
extern crate glium;
extern crate time;
#[macro_use]
extern crate bitflags;
extern crate unicode_normalization;
extern crate rusttype;
#[macro_use]
extern crate lazy_static;
extern crate libloading;
extern crate winapi;
extern crate image;

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

use debug::*;
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
