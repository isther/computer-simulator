use super::{Bit16, Bus, Component, Enabler, Wire, BUS_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub set: Wire,
    pub enable: Wire,
    pub word: Bit16,
    pub enabler: Rc<RefCell<Enabler>>,
    pub outputs: [Wire; BUS_WIDTH as usize],
    pub input_bus: Rc<RefCell<Bus>>,
    pub output_bus: Rc<RefCell<Bus>>,
}

impl Register {
    pub fn new(name: &str, input_bus: Rc<RefCell<Bus>>, output_bus: Rc<RefCell<Bus>>) -> Self {
        let mut res = Self {
            name: name.to_string(),
            set: Wire::new("S".to_string(), false),
            enable: Wire::new("E".to_string(), false),
            word: Bit16::new(),
            enabler: Rc::new(RefCell::new(Enabler::new())),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            input_bus,
            output_bus,
        };

        res.word.connect_output(res.enabler.clone());

        res
    }

    pub fn bit(&self, index: i32) -> bool {
        self.word.get_output_wire(index)
    }

    pub fn enable(&mut self) {
        self.enable.update(true)
    }

    pub fn disable(&mut self) {
        self.enable.update(false)
    }

    pub fn set(&mut self) {
        self.set.update(true)
    }

    pub fn unset(&mut self) {
        self.set.update(false)
    }

    pub fn update(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.word
                .set_input_wire(i, self.input_bus.borrow().get_output_wire(i))
        }

        self.word.update(self.set.get());
        self.enabler.borrow_mut().update(self.enable.get());

        for i in 0..self.enabler.borrow().outputs.len() {
            self.outputs[i].update(self.enabler.borrow().outputs[i].get())
        }

        if self.enable.get() {
            for i in (0..BUS_WIDTH).rev() {
                self.output_bus
                    .borrow_mut()
                    .set_input_wire(i, self.outputs[i as usize].get())
            }
        }
    }

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        let mut x: u16 = 0;

        for i in (0..BUS_WIDTH).rev() {
            match self.word.get_output_wire(i) {
                true => value = value | (1 << x),
                false => value = value ^ (value & (1 << x)),
            }
            x += 1;
        }
        value
    }
}
