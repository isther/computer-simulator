use super::{Bus, Component, Enableable, Enabler, Settable, Updatable, Wire, Word, BUS_WIDTH};
use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub set: Wire,
    pub enable: Wire,
    pub word: Word,
    pub enabler: Rc<RefCell<Enabler>>,
    pub outputs: [Wire; BUS_WIDTH as usize],
    pub input_bus: Arc<Mutex<Bus>>,
    pub output_bus: Arc<Mutex<Bus>>,
}

impl Register {
    pub fn new(name: &str, input_bus: Arc<Mutex<Bus>>, output_bus: Arc<Mutex<Bus>>) -> Self {
        let mut res = Self {
            name: name.to_string(),
            set: Wire::new("S".to_string(), false),
            enable: Wire::new("E".to_string(), false),
            word: Word::new(),
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

    pub fn word_value(&self) -> u16 {
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

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        let mut x: u16 = 0;

        for i in (0..BUS_WIDTH).rev() {
            match self.outputs[i as usize].get() {
                true => value = value | (1 << x),
                false => value = value ^ (value & (1 << x)),
            }
            x += 1;
        }
        value
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} E: {} S: {}",
            self.name,
            self.value(),
            self.enable.get(),
            self.set.get()
        )
    }
}

impl Enableable for Register {
    fn enable(&mut self) {
        self.enable.update(true)
    }

    fn disable(&mut self) {
        self.enable.update(false)
    }
}

impl Settable for Register {
    fn set(&mut self) {
        self.set.update(true)
    }

    fn unset(&mut self) {
        self.set.update(false)
    }
}

impl Updatable for Register {
    fn update(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.word
                .set_input_wire(i, self.input_bus.lock().unwrap().get_output_wire(i));
        }

        self.word.update(self.set.get());
        self.enabler.borrow_mut().update(self.enable.get());

        for i in 0..self.enabler.borrow().outputs.len() {
            self.outputs[i].update(self.enabler.borrow().outputs[i].get())
        }

        if self.enable.get() {
            for i in (0..BUS_WIDTH).rev() {
                self.output_bus
                    .lock()
                    .unwrap()
                    .set_input_wire(i, self.outputs[i as usize].get())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_disable_output_zero() {
        let bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut register = Register::new("test", bus.clone(), bus.clone());

        bus.lock().unwrap().set_value(0xABCD);
        register.set();
        register.enable();
        register.update();
        assert_eq!(register.value(), 0xABCD);

        register.disable();
        register.update();

        bus.lock().unwrap().set_value(0x00FF);

        assert_eq!(bus.lock().unwrap().get_value(), 0x00FF);
        assert_eq!(register.value(), 0x0000);
    }

    #[test]
    fn test_register_set() {
        let bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut register = Register::new("test", bus.clone(), bus.clone());
        register.disable();

        bus.lock().unwrap().set_value(0x00FF);
        register.set();
        register.update();

        bus.lock().unwrap().set_value(0x0EEE);
        register.unset();
        register.update();
        assert_eq!(register.word_value(), 0x00FF, "value should not change");

        bus.lock().unwrap().set_value(0xFF00);
        register.set();
        register.update();
        assert_eq!(register.value(), 0x0000, "value should change");
    }

    #[test]
    fn test_register_enable_output() {
        let bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut register = Register::new("test", bus.clone(), bus.clone());

        bus.lock().unwrap().set_value(0xABCD);
        register.set();
        register.disable();
        register.update();

        register.enable();
        register.update();

        bus.lock().unwrap().set_value(0x00FF);

        assert_eq!(bus.lock().unwrap().get_value(), 0x00FF);
        assert_eq!(register.value(), 0xABCD);
    }
}
