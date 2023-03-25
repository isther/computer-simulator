#![allow(unused)]

mod assembler;
mod generator;

mod instructions;

mod components;
mod cpu;
mod gates;
mod io;
mod memory;

mod computer;

pub use assembler::Assembler;
pub use generator::get_instructions;

use std::{cell::RefCell, rc::Rc, sync::Arc};
use tokio::sync::Mutex;
