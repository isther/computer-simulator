use crate::components::{
    ANDGate3, ANDGate8, Bit, Bus, Component, Enableable, IOBus, Mode, Register, Settable,
    Updatable, BUS_WIDTH,
};
use crate::gates::{AND, NOT};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

mod display;
mod display_ram;
mod keyboard;

pub use display::{DisplayAdapter, ScreenControl};
pub use keyboard::{Keyboard, KeyboardAdapter};

pub trait Peripheral {
    fn connect(&mut self, io_bus: Arc<Mutex<IOBus>>, bus: Arc<Mutex<Bus>>);
    fn update(&mut self);
}
