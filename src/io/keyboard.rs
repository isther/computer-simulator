use crate::{
    components::{
        ANDGate3, ANDGate8, Bit, Bus, Component, Enableable, IOBus, Mode, Register, Settable,
        Updatable, BUS_WIDTH,
    },
    gates::{AND, NOT},
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

// [cpu] <-------------> keyboard adapter <----------- keyboard <----------- [keyPressChannel]
//         read/write                        write                 notify
pub struct KeyboardAdapter {
    pub keyboard_in_bus: Arc<Mutex<Bus>>,

    io_bus: Rc<RefCell<IOBus>>,
    main_bus: Arc<Mutex<Bus>>,

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
    pub fn new() -> Self {
        Self {
            keyboard_in_bus: Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
            io_bus: Rc::new(RefCell::new(IOBus::new())),
            main_bus: Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
            memory_bit: Bit::new(),
            key_code_register: Register::new(
                "",
                Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
                Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
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

    fn connect(&mut self, io_bus: Rc<RefCell<IOBus>>, main_bus: Arc<Mutex<Bus>>) {
        self.io_bus = io_bus;
        self.main_bus = main_bus;

        self.memory_bit.update(false, true);
        self.memory_bit.update(false, false);
        self.key_code_register =
            Register::new("KCR", self.keyboard_in_bus.clone(), self.main_bus.clone());
    }

    fn update(&mut self) {
        self.update_key_code_reg();

        let main_bus = self.main_bus.lock().unwrap();
        self.not_gates_for_and_gate1[0].update(main_bus.get_output_wire(8));
        self.not_gates_for_and_gate1[1].update(main_bus.get_output_wire(9));
        self.not_gates_for_and_gate1[2].update(main_bus.get_output_wire(10));
        self.not_gates_for_and_gate1[3].update(main_bus.get_output_wire(11));

        self.and_gate1.update(
            self.not_gates_for_and_gate1[0].get(),
            self.not_gates_for_and_gate1[1].get(),
            self.not_gates_for_and_gate1[2].get(),
            self.not_gates_for_and_gate1[3].get(),
            main_bus.get_output_wire(12),
            main_bus.get_output_wire(13),
            main_bus.get_output_wire(14),
            main_bus.get_output_wire(15),
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
            self.keyboard_in_bus.lock().unwrap().set_value(0x00);
            self.key_code_register.update();
            self.key_code_register.unset();
            self.key_code_register.update();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KeyPress {
    pub value: i32,
    pub is_down: bool,
}

pub struct Keyboard {
    out_bus: Option<Arc<Mutex<Bus>>>,
    key_press: mpsc::Receiver<KeyPress>,
    quit: Arc<tokio::sync::Notify>,
}

impl Keyboard {
    pub fn connect(&mut self, bus: Arc<Mutex<Bus>>) -> &mut Self {
        println!("Connecting keyboard to bus");
        self.out_bus = Some(bus);
        self
    }

    pub async fn run(&mut self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
            tokio::select! {
                Some(key_press) = self.key_press.recv() => {
                    println!("Key press: {:?}",key_press);
                    if key_press.is_down {
                        match &self.out_bus {
                            Some(bus)=>{
                                bus.lock().unwrap().set_value(key_press.value as u16);
                            },
                            None=>{println!("No bus, value: {}",key_press.value)},
                        }
                    }
                },
                _ = self.quit.notified() => {
                    println!("Stopping keyboard");
                    return;
                },
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Notify;

    #[test]
    fn test_key_adapter() {
        let io_bus = Rc::new(RefCell::new(IOBus::new()));
        let main_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut key_adapter = KeyboardAdapter::new();
        key_adapter.connect(io_bus.clone(), main_bus.clone());
        main_bus.lock().unwrap().set_value(0x000F);
        key_adapter
            .keyboard_in_bus
            .lock()
            .unwrap()
            .set_value(0x1234);
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

        assert_eq!(main_bus.lock().unwrap().get_value(), 0x1234);
        assert_eq!(key_adapter.key_code_register.value(), 0x0000);
    }

    #[tokio::test]
    async fn test_key_board() {
        let out_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let (keypress_sender, keypress_receiver) = mpsc::channel(32);
        let quit = Arc::new(Notify::new());

        let out_bus2 = out_bus.clone();
        let quit2 = quit.clone();

        tokio::spawn(async move {
            println!("Starting keyboard");
            Keyboard {
                out_bus: None,
                key_press: keypress_receiver,
                quit: quit2,
            }
            .connect(out_bus2)
            .run()
            .await;
        });

        for i in 0..10 {
            keypress_sender
                .send(KeyPress {
                    value: i,
                    is_down: true,
                })
                .await
                .unwrap();
            println!("Sent keypress");
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            assert_eq!(i, out_bus.lock().unwrap().get_value() as i32);
        }
        quit.notify_one();
        println!("Quit");

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
}
