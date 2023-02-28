use super::{Bus, Component, Enableable, Enabler, Settable, Updatable, Wire, Word, BUS_WIDTH};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub set: Wire,
    pub enable: Wire,
    pub word: Rc<RefCell<Word>>,
    pub enabler: Rc<RefCell<Enabler>>,
    pub outputs: [Wire; BUS_WIDTH as usize],
    pub input_bus: Rc<RefCell<Bus>>,
    pub output_bus: Rc<RefCell<Bus>>,
}

impl Register {
    pub fn new(name: &str, input_bus: Rc<RefCell<Bus>>, output_bus: Rc<RefCell<Bus>>) -> Self {
        let res = Self {
            name: name.to_string(),
            set: Wire::new("S".to_string(), false),
            enable: Wire::new("E".to_string(), false),
            word: Rc::new(RefCell::new(Word::new())),
            enabler: Rc::new(RefCell::new(Enabler::new())),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            input_bus,
            output_bus,
        };

        res.word.borrow_mut().connect_output(res.enabler.clone());

        res
    }

    pub fn bit(&self, index: i32) -> bool {
        self.word.borrow().get_output_wire(index)
    }

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        let mut x: u16 = 0;

        for i in (0..BUS_WIDTH).rev() {
            match self.word.borrow().get_output_wire(i) {
                true => value = value | (1 << x),
                false => value = value ^ (value & (1 << x)),
            }
            x += 1;
        }
        value
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
                .borrow_mut()
                .set_input_wire(i, self.input_bus.borrow().get_output_wire(i));
        }

        self.word.borrow_mut().update(self.set.get());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_set() {
        let bus = Rc::new(RefCell::new(Bus::new(BUS_WIDTH)));
        let register = Rc::new(RefCell::new(Register::new(
            "test",
            bus.clone(),
            bus.clone(),
        )));
        register.borrow_mut().disable();

        set_bus_value(&mut bus.borrow_mut(), 0x00FF);
        register.borrow_mut().set();
        register.borrow_mut().update();
        assert_eq!(get_component_value(register.borrow().word.clone()), 0x00FF);

        set_bus_value(&mut bus.borrow_mut(), 0x0EEE);
        register.borrow_mut().unset();
        register.borrow_mut().update();
        assert_eq!(
            get_component_value(register.borrow().word.clone()),
            0x00FF,
            "value should not change"
        );

        set_bus_value(&mut bus.borrow_mut(), 0xFF00);
        register.borrow_mut().set();
        register.borrow_mut().update();
        assert_eq!(
            get_component_value(register.borrow().word.clone()),
            0xFF00,
            "value should change"
        );
    }

    #[test]
    fn test_register_disable_output_zero() {
        let bus = Rc::new(RefCell::new(Bus::new(BUS_WIDTH)));
        let register = Rc::new(RefCell::new(Register::new(
            "test",
            bus.clone(),
            bus.clone(),
        )));
        set_bus_value(&mut bus.borrow_mut(), 0xABCD);
        register.borrow_mut().set();
        register.borrow_mut().enable();
        register.borrow_mut().update();

        register.borrow_mut().disable();
        register.borrow_mut().update();

        set_bus_value(&mut bus.borrow_mut(), 0x00FF);

        assert_eq!(get_bus_value(&bus.borrow()), 0x00FF);
        assert_eq!(get_register_output(&register.borrow()), 0x0000);
    }

    #[test]
    fn test_register_enable_output() {
        let bus = Rc::new(RefCell::new(Bus::new(BUS_WIDTH)));
        let register = Rc::new(RefCell::new(Register::new(
            "test",
            bus.clone(),
            bus.clone(),
        )));
        set_bus_value(&mut bus.borrow_mut(), 0xABCD);
        register.borrow_mut().set();
        register.borrow_mut().disable();
        register.borrow_mut().update();

        register.borrow_mut().enable();
        register.borrow_mut().update();

        set_bus_value(&mut bus.borrow_mut(), 0x00FF);

        assert_eq!(get_bus_value(&bus.borrow()), 0x00FF);
        assert_eq!(get_register_output(&register.borrow()), 0xABCD);
    }

    fn set_bus_value(b: &mut Bus, value: u16) {
        for i in (0..BUS_WIDTH).rev() {
            match value & (1 << i as u16) {
                0 => b.set_input_wire(i, false),
                _ => b.set_input_wire(i, true),
            }
        }
    }

    fn get_bus_value(b: &Bus) -> u16 {
        let mut result: u16 = 0;
        for i in (0..BUS_WIDTH).rev() {
            match b.get_output_wire(i) {
                true => result = result | (1 << i as u16),
                false => result = result & (result ^ 1 << i as u16),
            }
        }
        result
    }

    fn get_register_output(b: &Register) -> u16 {
        let mut result: u16 = 0;
        for i in (0..BUS_WIDTH).rev() {
            match b.outputs[i as usize].get() {
                true => result = result | (1 << i as u16),
                false => result = result & (result ^ 1 << i as u16),
            }
        }
        result
    }

    fn get_component_value<T>(component: Rc<RefCell<T>>) -> u16
    where
        T: Component,
    {
        let mut result: u16 = 0;
        for i in (0..BUS_WIDTH).rev() {
            match component.borrow().get_output_wire(i) {
                true => result = result | (1 << i as u16),
                false => result = result & (result ^ 1 << i as u16),
            }
        }
        result
    }
}
