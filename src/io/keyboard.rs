use super::{
    ANDGate3, ANDGate8, Bit, Bus, Component, Enableable, IOBus, Mode, Register, Settable,
    Updatable, AND, BUS_WIDTH, NOT,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

// [cpu] <-------------> keyboard adapter <----------- keyboard <----------- [keyPressChannel]
//         read/write                        write                 notify
struct KeyboardAdapter {
    keyboard_in_bus: Rc<RefCell<Bus>>,

    io_bus: Rc<RefCell<IOBus>>,
    main_bus: Rc<RefCell<Bus>>,

    memory_bit: Bit,
    key_code_register: Register,

    and_gate1: ANDGate8,
    not_gates_for_and_gate1: [NOT; 4],

    and_gate2: ANDGate3,
    and_gate3: ANDGate3,
    not_gates_for_and_gate3: [NOT; 2],
    and_gate4: AND,
}

impl KeyboardAdapter {
    fn new() -> Self {
        Self {
            keyboard_in_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            io_bus: Rc::new(RefCell::new(IOBus::new())),
            main_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            memory_bit: Bit::new(),
            key_code_register: Register::new(
                "",
                Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
                Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            ),
            and_gate1: ANDGate8::new(),
            not_gates_for_and_gate1: (0..4)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            and_gate2: ANDGate3::new(),
            and_gate3: ANDGate3::new(),
            not_gates_for_and_gate3: (0..2)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            and_gate4: AND::new(),
        }
    }

    fn connect(&mut self, io_bus: Rc<RefCell<IOBus>>, main_bus: Rc<RefCell<Bus>>) {
        self.io_bus = io_bus;
        self.main_bus = main_bus;

        self.memory_bit.update(false, true);
        self.memory_bit.update(false, false);
        self.key_code_register =
            Register::new("KCR", self.keyboard_in_bus.clone(), self.main_bus.clone());
    }

    fn update(&mut self) {
        self.update_key_code_reg();

        self.not_gates_for_and_gate1[0].update(self.main_bus.borrow().get_output_wire(8));
        self.not_gates_for_and_gate1[1].update(self.main_bus.borrow().get_output_wire(9));
        self.not_gates_for_and_gate1[2].update(self.main_bus.borrow().get_output_wire(10));
        self.not_gates_for_and_gate1[3].update(self.main_bus.borrow().get_output_wire(11));

        self.and_gate1.update(
            self.not_gates_for_and_gate1[0].get(),
            self.not_gates_for_and_gate1[1].get(),
            self.not_gates_for_and_gate1[2].get(),
            self.not_gates_for_and_gate1[3].get(),
            self.main_bus.borrow().get_output_wire(12),
            self.main_bus.borrow().get_output_wire(13),
            self.main_bus.borrow().get_output_wire(14),
            self.main_bus.borrow().get_output_wire(15),
        );

        self.and_gate2.update(
            self.io_bus.borrow().get_output_wire(Mode::ClockSet.into()),
            self.io_bus
                .borrow()
                .get_output_wire(Mode::DataOrAddress.into()),
            self.io_bus.borrow().get_output_wire(Mode::Mode.into()),
        );
        self.memory_bit
            .update(self.and_gate1.get(), self.and_gate2.get());

        self.not_gates_for_and_gate3[0].update(
            self.io_bus
                .borrow()
                .get_output_wire(Mode::DataOrAddress.into()),
        );
        self.not_gates_for_and_gate3[1]
            .update(self.io_bus.borrow().get_output_wire(Mode::Mode.into()));

        self.and_gate3.update(
            self.io_bus
                .borrow()
                .get_output_wire(Mode::ClockEnable.into()),
            self.not_gates_for_and_gate3[0].get(),
            self.not_gates_for_and_gate3[1].get(),
        );

        self.and_gate4
            .update(self.memory_bit.get(), self.and_gate3.get());
    }

    fn update_key_code_reg(&mut self) {
        if self.and_gate4.get() {
            self.key_code_register.set();

            self.key_code_register.enable();
            self.key_code_register.update();
            self.key_code_register.disable();

            // clear the register once everything is out
            self.keyboard_in_bus.borrow_mut().set_value(0x00);
            self.key_code_register.update();
            self.key_code_register.unset();
            self.key_code_register.update();
        }
    }
}

#[derive(Clone)]
pub struct KeyPress {
    pub value: i32,
    pub is_down: bool,
}

struct Keyboard {
    out_bus: Rc<RefCell<Bus>>,
    key_press: Rc<RefCell<KeyPress>>,
    quit: Rc<RefCell<bool>>,
}

impl Keyboard {
    fn new(key_press: Rc<RefCell<KeyPress>>, quit: Rc<RefCell<bool>>) -> Self {
        Self {
            out_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            key_press,
            quit,
        }
    }

    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        println!("Connecting keyboard to bus");
        self.out_bus = bus
    }
    fn run(&mut self) {
        loop {
            sleep(Duration::from_millis(33));

            match self.quit.borrow().eq(&true) {
                true => println!("Stopping keyboard"),
                false => {
                    let key = &*self.key_press.borrow();
                    if key.is_down {
                        self.out_bus.borrow_mut().set_value(key.value as u16);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_adapter() {
        let io_bus = Rc::new(RefCell::new(IOBus::new()));
        let main_bus = Rc::new(RefCell::new(Bus::new(BUS_WIDTH)));
        let mut key_adapter = KeyboardAdapter::new();
        key_adapter.connect(io_bus.clone(), main_bus.clone());

        main_bus.borrow_mut().set_value(0x000F);
        key_adapter.keyboard_in_bus.borrow_mut().set_value(0x1234);
        key_adapter.update();

        io_bus.borrow_mut().set();
        io_bus.borrow_mut().update(true, true);
        key_adapter.update();

        io_bus.borrow_mut().unset();
        key_adapter.update();

        io_bus.borrow_mut().enable();
        io_bus.borrow_mut().update(false, false);
        key_adapter.update();

        key_adapter.update();

        assert_eq!(main_bus.borrow().get_value(), 0x1234);
        assert_eq!(key_adapter.key_code_register.value(), 0x0000);
    }
}
