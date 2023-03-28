#![allow(unused)]
#![feature(async_closure)]

mod assembler;
mod generator;

mod instructions;

mod components;
mod cpu;
mod gates;
mod io;
mod memory;

mod computer;
mod glfw_io;

pub use assembler::Assembler;
pub use generator::get_instructions;
