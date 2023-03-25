use crate::{
    components::{Bus, Decoder8x256, Register, Updatable},
    gates::Wire,
    memory::Cell,
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

// Display RAM is special as the writes (inputs) and reads (outputs) are two separate
// units that operate independently.
pub struct DisplayRAM {
    pub input_address_register: Register,
    input_row_decoder: Decoder8x256,
    input_col_decoder: Decoder8x256,

    output_address_register: Register,
    output_row_decoder: Decoder8x256,
    output_col_decoder: Decoder8x256,

    data: Vec<Vec<Cell>>,
    set: Wire,
    enable: Wire,
    input_bus: Arc<Mutex<Bus>>,
    output_bus: Arc<Mutex<Bus>>,
}

impl DisplayRAM {
    pub fn new(input_bus: Arc<Mutex<Bus>>, output_bus: Arc<Mutex<Bus>>) -> Self {
        Self {
            input_address_register: Register::new("IMAR", input_bus.clone(), output_bus.clone()),
            input_row_decoder: Decoder8x256::new(),
            input_col_decoder: Decoder8x256::new(),

            output_address_register: Register::new("OMAR", input_bus.clone(), output_bus.clone()),
            output_row_decoder: Decoder8x256::new(),
            output_col_decoder: Decoder8x256::new(),
            // 0xF0 x 0xA0
            data: (0..256)
                .map(|_| {
                    (0..256)
                        .map(|_| Cell::new(input_bus.clone(), output_bus.clone()))
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>(),
            set: Wire::new("S".to_string(), false),
            enable: Wire::new("E".to_string(), false),
            input_bus: input_bus.clone(),
            output_bus: output_bus.clone(),
        }
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

    pub fn update_incoming(&mut self) {
        self.input_address_register.update();
        self.input_row_decoder.update(
            self.input_address_register.bit(0),
            self.input_address_register.bit(1),
            self.input_address_register.bit(2),
            self.input_address_register.bit(3),
            self.input_address_register.bit(4),
            self.input_address_register.bit(5),
            self.input_address_register.bit(6),
            self.input_address_register.bit(7),
        );
        self.input_col_decoder.update(
            self.input_address_register.bit(8),
            self.input_address_register.bit(9),
            self.input_address_register.bit(10),
            self.input_address_register.bit(11),
            self.input_address_register.bit(12),
            self.input_address_register.bit(13),
            self.input_address_register.bit(14),
            self.input_address_register.bit(15),
        );

        self.data[self.input_row_decoder.index() as usize][self.input_col_decoder.index() as usize]
            .update(self.set.get(), false)
    }

    pub fn update_outgoing(&mut self) {
        self.output_address_register.update();
        self.output_row_decoder.update(
            self.output_address_register.bit(0),
            self.output_address_register.bit(1),
            self.output_address_register.bit(2),
            self.output_address_register.bit(3),
            self.output_address_register.bit(4),
            self.output_address_register.bit(5),
            self.output_address_register.bit(6),
            self.output_address_register.bit(7),
        );

        self.output_row_decoder.update(
            self.output_address_register.bit(8),
            self.output_address_register.bit(9),
            self.output_address_register.bit(10),
            self.output_address_register.bit(11),
            self.output_address_register.bit(12),
            self.output_address_register.bit(13),
            self.output_address_register.bit(14),
            self.output_address_register.bit(15),
        );
        self.data[self.output_row_decoder.index() as usize]
            [self.output_col_decoder.index() as usize]
            .update(false, self.enable.get())
    }
}
