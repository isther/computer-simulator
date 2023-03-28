use crate::{
    components::{Bus, Settable, Updatable, BUS_WIDTH},
    cpu::CPU,
    io::{DisplayAdapter, Keyboard, KeyboardAdapter, ScreenControl},
    memory::Memory64K,
};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Notify};

const CODE_REGION_START: u16 = 0x0500;

struct PrintStateConfig {
    print_state: bool,
    print_state_every: u16,
}

struct SimpleComputer {
    main_bus: Arc<Mutex<Bus>>,
    cpu: CPU,
    memory: Arc<Mutex<Memory64K>>,
    display_adapter: Arc<Mutex<DisplayAdapter>>,
    screen_control: Arc<Mutex<ScreenControl>>,
    keyboard_adapter: KeyboardAdapter,
    screen_channel: mpsc::Sender<[[u8; 240]; 160]>,
    quit_channel: Arc<Notify>,
}

impl SimpleComputer {
    fn new(screen_channel: mpsc::Sender<[[u8; 240]; 160]>, quit_channel: Arc<Notify>) -> Self {
        let main_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let memory = Arc::new(Mutex::new(Memory64K::new(main_bus.clone())));
        let display_adapter = Arc::new(Mutex::new(DisplayAdapter::new()));
        let mut res = Self {
            main_bus: main_bus.clone(),
            cpu: CPU::new(main_bus.clone(), memory.clone()),
            memory: memory.clone(),
            display_adapter: display_adapter.clone(),
            screen_control: Arc::new(Mutex::new(ScreenControl::new(
                display_adapter.clone(),
                screen_channel.clone(),
                quit_channel.clone(),
            ))),
            keyboard_adapter: KeyboardAdapter::new(),
            screen_channel,
            quit_channel,
        };
        res.cpu.connect_peripheral(res.display_adapter.clone());
        res
    }

    fn connect_keyboard(&mut self, keyboard: &Arc<Mutex<Keyboard>>) {
        keyboard
            .lock()
            .unwrap()
            .connect(self.keyboard_adapter.keyboard_in_bus.clone());
    }

    fn load_to_ram(&mut self, offset: u16, values: Vec<u16>) {
        if offset < 0x0500 {
            panic!("0x0000 - 0x04FF is a reserved memory area");
        }
        if offset > 0xFEFF {
            panic!("0xFEFF - 0xFFFF is a reserved memory area");
        }
        println!(
            "Loading {} words to RAM at offset 0x{:X}",
            values.len(),
            offset
        );

        for i in 0..values.len() {
            self.put_value_in_ram(offset + i as u16, values[i]);
        }
    }

    fn put_value_in_ram(&mut self, address: u16, value: u16) {
        self.memory.lock().unwrap().address_register.set();
        self.main_bus.lock().unwrap().set_value(address);
        self.memory.lock().unwrap().update();

        self.memory.lock().unwrap().address_register.unset();
        self.memory.lock().unwrap().update();

        self.main_bus.lock().unwrap().set_value(value);
        self.memory.lock().unwrap().set();
        self.memory.lock().unwrap().update();

        self.memory.lock().unwrap().address_register.unset();
        self.memory.lock().unwrap();
    }

    async fn run(&mut self, tick_interval: Arc<Notify>, print_state_config: PrintStateConfig) {
        println!("Starting computer....");
        self.put_value_in_ram(0xFEFE, 0x0040); //JMP back to code region start if IAR reaches the end
        self.put_value_in_ram(0xFEFF, CODE_REGION_START);

        // start at offet of user code
        self.cpu.set_iar(CODE_REGION_START);

        //BUG: self.screen_control.lock().unwrap().run();

        let mut steps = 0;
        loop {
            // 		<-tickInterval
            self.cpu.step();

            if print_state_config.print_state {
                if steps % print_state_config.print_state_every == 0 {
                    println!(
                        "COMPUTER\n-----------------------------------------------------------"
                    );
                    println!(
                        "Cycle count = {}, step count = {}, printing state every {} steps",
                        steps / 6,
                        steps,
                        print_state_config.print_state_every
                    );
                    println!("CPU\n----------------------------------------");
                    println!("{}", self.cpu);
                    println!();
                }
            }
            steps += 1;
        }
    }
}
