use super::{display_ram::DisplayRAM, Peripheral};
use crate::computer::{
    components::{
        ANDGate3, ANDGate5, ANDGate8, Bit, Bus, Component, IOBus, Settable, Updatable, BUS_WIDTH,
    },
    gates::NOT,
};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, Notify};

// [cpu] -------> display adapter --------> display RAM <--------- screen control ---------> [screenChannel]
//       write                     write                   read                     write
pub struct DisplayAdapter {
    io_bus: Arc<Mutex<IOBus>>,
    main_bus: Arc<Mutex<Bus>>,
    screen_bus: Arc<Mutex<Bus>>,
    display_ram: Option<DisplayRAM>,
    display_adapter_active_bit: Bit,
    pub input_mar_out_bus: Bus,
    address_select_and_gate: ANDGate8,
    address_select_not_gates: [NOT; 5],
    is_address_output_mode_gate: ANDGate3,
    input_mar_set_gate: ANDGate5,
    input_mar_set_not_gates: [NOT; 2],
    write_to_ram: Bit,
    write_to_ram_toggle_gate: NOT,
    display_ram_set_gate: ANDGate5,
}

impl DisplayAdapter {
    pub fn new() -> Self {
        DisplayAdapter {
            io_bus: Arc::new(Mutex::new(IOBus::new())),
            main_bus: Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
            screen_bus: Arc::new(Mutex::new(Bus::new(BUS_WIDTH))),
            display_ram: None,
            display_adapter_active_bit: Bit::new(),
            input_mar_out_bus: Bus::new(BUS_WIDTH),
            address_select_and_gate: ANDGate8::new(),
            address_select_not_gates: (0..5)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            is_address_output_mode_gate: ANDGate3::new(),
            input_mar_set_gate: ANDGate5::new(),
            input_mar_set_not_gates: (0..2)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            write_to_ram: Bit::new(),
            write_to_ram_toggle_gate: NOT::new(),
            display_ram_set_gate: ANDGate5::new(),
        }
    }

    fn toggle_write_to_ram(&mut self) {
        self.write_to_ram_toggle_gate
            .update(self.write_to_ram.get());
        self.write_to_ram
            .update(self.write_to_ram_toggle_gate.get(), true);
        self.write_to_ram.update(false, false);
    }

    fn write_to_input_mar(&mut self) {
        // if writeToRAM == false then put bus contents in Input-MAR
        self.input_mar_set_not_gates[0].update(self.write_to_ram.get());

        let io_bus = self.io_bus.clone();
        let io_bus = io_bus.lock().unwrap();
        self.input_mar_set_gate.update(
            io_bus.is_data_mode(),
            io_bus.is_set(),
            io_bus.is_output_mode(),
            self.display_adapter_active_bit.get(),
            self.input_mar_set_not_gates[0].get(),
        );

        if self.input_mar_set_gate.get() {
            self.display_ram
                .as_mut()
                .unwrap()
                .input_address_register
                .set();
            self.display_ram
                .as_mut()
                .unwrap()
                .input_address_register
                .update();
            self.display_ram
                .as_mut()
                .unwrap()
                .input_address_register
                .unset();
            self.display_ram
                .as_mut()
                .unwrap()
                .input_address_register
                .update();
            self.toggle_write_to_ram();
        }
    }

    fn write_to_display_ram(&mut self) {
        // if writeToRAM == true then put bus contents in RAM
        let io_bus = self.io_bus.clone();
        let io_bus = io_bus.lock().unwrap();
        self.display_ram_set_gate.update(
            io_bus.is_data_mode(),
            io_bus.is_set(),
            io_bus.is_output_mode(),
            self.display_adapter_active_bit.get(),
            self.write_to_ram.get(),
        );

        if self.display_ram_set_gate.get() {
            self.display_ram.as_mut().unwrap().set();
            self.display_ram.as_mut().unwrap().update_incoming();
            self.display_ram.as_mut().unwrap().unset();
            self.display_ram.as_mut().unwrap().update_incoming();
            self.toggle_write_to_ram();
        }
    }
}

impl Peripheral for DisplayAdapter {
    fn connect(&mut self, io_bus: Arc<Mutex<IOBus>>, main_bus: Arc<Mutex<Bus>>) {
        self.io_bus = io_bus.clone();
        self.main_bus = main_bus.clone();
        self.display_ram = Some(DisplayRAM::new(main_bus.clone(), self.screen_bus.clone()));

        self.display_adapter_active_bit.update(false, true);
        self.display_adapter_active_bit.update(false, false);

        self.write_to_ram.update(false, true);
        self.write_to_ram.update(false, false);
    }

    fn update(&mut self) {
        // check if bus = 0x0007
        self.address_select_not_gates[0].update(self.main_bus.lock().unwrap().get_output_wire(8));
        self.address_select_not_gates[1].update(self.main_bus.lock().unwrap().get_output_wire(9));
        self.address_select_not_gates[2].update(self.main_bus.lock().unwrap().get_output_wire(10));
        self.address_select_not_gates[3].update(self.main_bus.lock().unwrap().get_output_wire(11));
        self.address_select_not_gates[4].update(self.main_bus.lock().unwrap().get_output_wire(12));

        let main_bus = self.main_bus.clone();
        let main_bus = main_bus.lock().unwrap();
        self.address_select_and_gate.update(
            self.address_select_not_gates[0].get(),
            self.address_select_not_gates[1].get(),
            self.address_select_not_gates[2].get(),
            self.address_select_not_gates[3].get(),
            self.address_select_not_gates[4].get(),
            main_bus.get_output_wire(13),
            main_bus.get_output_wire(14),
            main_bus.get_output_wire(15),
        );

        let io_bus = self.io_bus.clone();
        let io_bus = io_bus.lock().unwrap();
        self.is_address_output_mode_gate.update(
            io_bus.is_set(),
            io_bus.is_address_mode(),
            io_bus.is_output_mode(),
        );

        self.display_adapter_active_bit.update(
            self.address_select_and_gate.get(),
            self.is_address_output_mode_gate.get(),
        );

        // switch between writing to display RAM and writing to address register
        match self.write_to_ram.get() {
            true => {
                self.write_to_display_ram();
            }
            false => {
                self.write_to_input_mar();
            }
        }
    }
}

#[derive(Clone)]
pub struct ScreenControl {
    adapter: Arc<Mutex<DisplayAdapter>>,
    input_bus: Option<Bus>,
    output_chan: mpsc::Sender<[[u8; 240]; 160]>,
    clock: u64,
    quit: Arc<Notify>,
    //y, x
    output: [[u8; 240]; 160],
}

impl ScreenControl {
    pub fn new(
        adapter: Arc<Mutex<DisplayAdapter>>,
        output_chan: mpsc::Sender<[[u8; 240]; 160]>,
        quit: Arc<Notify>,
    ) -> ScreenControl {
        ScreenControl {
            adapter,
            input_bus: None,
            output_chan,
            clock: 33,
            quit,
            output: [[0; 240]; 160],
        }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select!(
                _ = self.quit.notified() => {
                    println!("Stopping keyboard");
                    return;
                },
                else =>{
                    tokio::time::sleep(tokio::time::Duration::from_millis(self.clock)).await;
                    self.update();
                    self.output_chan.send(self.output).await.unwrap();
                },
            )
        }
    }

    fn update(&mut self) {
        let width_in_bytes = 30; // 30 * 8 = 240
        let mut y = 0;

        for vertical in (0..0x12C0)
            .filter(|x| x % width_in_bytes == 0)
            .collect::<Vec<u16>>()
        {
            let mut x = 0;
            for horizontal in 0..width_in_bytes {
                self.set_output_ram_address(vertical + horizontal);
                self.render_pixels_from_ram(&y, &mut x);
                x += 8;
            }
            y += 1;
        }
    }

    fn set_output_ram_address(&mut self, address: u16) {
        self.adapter
            .lock()
            .unwrap()
            .screen_bus
            .lock()
            .unwrap()
            .set_value(address);
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .input_address_register
            .set();
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .input_address_register
            .update();
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .input_address_register
            .unset();
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .input_address_register
            .update();
    }

    fn render_pixels_from_ram(&mut self, y: &u16, x: &mut u16) {
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .enable();
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .update_outgoing();

        for b in 8..16 {
            match self
                .adapter
                .lock()
                .unwrap()
                .screen_bus
                .lock()
                .unwrap()
                .get_output_wire(b)
            {
                true => self.output[*y as usize][*x as usize] = 0x01,
                false => self.output[*y as usize][*x as usize] = 0x00,
            }
            *x += 1;
        }

        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .disable();
        self.adapter
            .lock()
            .unwrap()
            .display_ram
            .as_mut()
            .unwrap()
            .update_outgoing();
    }
}
