use crate::computer::components::{Bus, IOBus};
use std::sync::{Arc, Mutex};

mod display;
mod display_ram;
mod keyboard;

pub use display::{DisplayAdapter, ScreenControl};
pub use keyboard::{KeyPress, Keyboard, KeyboardAdapter};

pub trait Peripheral: Send {
    fn connect(&mut self, io_bus: Arc<Mutex<IOBus>>, bus: Arc<Mutex<Bus>>);
    fn update(&mut self);
}
