use crate::components::{Bus, Decoder8x256, Register};
use crate::gates::Wire;
use crate::gates::AND;
use std::cell::RefCell;

pub const BUS_WIDTH: i32 = 16;

#[derive(Debug)]
struct Cell<'a> {
    value: Register<'a>,
    gates: [AND; 3],
}

impl<'a> Cell<'a> {
    fn new(input_bus: &'a RefCell<Bus>, output_bus: &'a RefCell<Bus>) -> Self {
        Self {
            value: Register::new("", input_bus, output_bus),
            gates: (0..3)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
        }
    }

    fn update(&mut self, set: bool, enable: bool) {
        self.gates[0].update(true, true);
        self.gates[1].update(self.gates[0].get(), set);
        self.gates[2].update(self.gates[0].get(), enable);

        match self.gates[1].get() {
            true => self.value.set(),
            false => self.value.unset(),
        }

        match self.gates[2].get() {
            true => self.value.enable(),
            false => self.value.disable(),
        }
    }
}

struct Memory64K<'a> {
    address_register: Register<'a>,
    row_decoder: Decoder8x256,
    col_decoder: Decoder8x256,
    // data: [[Cell; 256]; 256],
    data: [Cell<'a>; 256],
    set: Wire,
    enable: Wire,
    bus: &'a RefCell<Bus>,
}

impl<'a> Memory64K<'a> {
    fn new(bus: &'a RefCell<Bus>) -> Self {
        Self {
            address_register: Register::new("MAR", bus, bus),
            row_decoder: Decoder8x256::new(),
            col_decoder: Decoder8x256::new(),
            data: (0..256)
                .map(|_| Cell::new(bus, bus))
                .collect::<Vec<Cell>>()
                .try_into()
                .unwrap(),
            set: Wire::new("Z".to_string(), false),
            enable: Wire::new("Z".to_string(), false),
            bus,
        }
    }
}
