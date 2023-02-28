use crate::components::{
    ANDGate3, ANDGate8, Bit, Bus, Component, Enableable, IOBus, Mode, Register, Settable,
    Updatable, BUS_WIDTH,
};
use crate::gates::{AND, NOT};
use std::cell::RefCell;
use std::rc::Rc;

mod keyboard;

pub trait Peripheral {
    fn connect(&mut self, io_bus: Rc<RefCell<IOBus>>, bus: Rc<RefCell<Bus>>);
    fn update(&mut self);
}
