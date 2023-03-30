#![allow(unused)]
#![feature(async_closure)]

#[macro_use]
extern crate glium;

mod assembler;
mod generator;

mod instructions;

mod computer;

mod glfw;

pub use assembler::Assembler;
pub use computer::{Computer, Keyboard, PrintStateConfig};
pub use generator::get_instructions;
pub use glfw::glfw_run;
