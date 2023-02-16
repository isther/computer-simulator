use super::{Bit16, Bus, Component, Enabler, Wire, BUS_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Register<'a> {
    name: String,
    set: Wire,
    enable: Wire,
    word: Bit16,
    enabler: Rc<RefCell<Box<Enabler>>>,
    outputs: [Wire; BUS_WIDTH as usize],
    input_bus: &'a RefCell<Bus>,
    output_bus: &'a RefCell<Bus>,
}

impl<'a> Register<'a> {
    pub fn new(name: &str, input_bus: &'a RefCell<Bus>, output_bus: &'a RefCell<Bus>) -> Self {
        let mut res = Self {
            name: name.to_string(),
            set: Wire::new("S".to_string(), false),
            enable: Wire::new("E".to_string(), false),
            word: Bit16::new(),
            enabler: Rc::new(RefCell::new(Box::new(Enabler::new()))),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            input_bus,
            output_bus,
        };

        //TODO:word connect to enabler;

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
