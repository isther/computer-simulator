use super::{display_ram::DisplayRAM, Peripheral};
use crate::{
    components::{
        ANDGate3, ANDGate5, ANDGate8, Bit, Bus, Component, IOBus, Settable, Updatable, BUS_WIDTH,
    },
    gates::NOT,
};
use std::{cell::RefCell, rc::Rc};

// [cpu] -------> display adapter --------> display RAM <--------- screen control ---------> [screenChannel]
//       write                     write                   read                     write
struct DisplayAdapter {
    io_bus: Option<Rc<RefCell<IOBus>>>,
    main_bus: Option<Rc<RefCell<Bus>>>,
    screen_bus: Rc<RefCell<Bus>>,
    display_ram: Option<Rc<RefCell<DisplayRAM>>>,
    display_adapter_active_bit: Bit,
    input_mar_out_bus: Rc<RefCell<Bus>>,
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
            io_bus: None,
            main_bus: None,
            screen_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            display_ram: None,
            display_adapter_active_bit: Bit::new(),
            input_mar_out_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
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

        self.input_mar_set_gate.update(
            self.io_bus.as_ref().unwrap().borrow().is_data_mode(),
            self.io_bus.as_ref().unwrap().borrow().is_set(),
            self.io_bus.as_ref().unwrap().borrow().is_output_mode(),
            self.display_adapter_active_bit.get(),
            self.input_mar_set_not_gates[0].get(),
        );

        if self.input_mar_set_gate.get() {
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .input_address_register
                .set();
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .input_address_register
                .update();
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .input_address_register
                .unset();
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .input_address_register
                .update();
            self.toggle_write_to_ram();
        }
    }

    fn write_to_display_ram(&mut self) {
        // if writeToRAM == true then put bus contents in RAM

        self.display_ram_set_gate.update(
            self.io_bus.as_ref().unwrap().borrow().is_data_mode(),
            self.io_bus.as_ref().unwrap().borrow().is_set(),
            self.io_bus.as_ref().unwrap().borrow().is_output_mode(),
            self.display_adapter_active_bit.get(),
            self.write_to_ram.get(),
        );

        if self.display_ram_set_gate.get() {
            self.display_ram.as_ref().unwrap().borrow_mut().set();
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .update_incoming();
            self.display_ram.as_ref().unwrap().borrow_mut().unset();
            self.display_ram
                .as_ref()
                .unwrap()
                .borrow_mut()
                .update_incoming();
            self.toggle_write_to_ram();
        }
    }
}

impl Peripheral for DisplayAdapter {
    fn connect(&mut self, io_bus: Rc<RefCell<IOBus>>, main_bus: Rc<RefCell<Bus>>) {
        self.io_bus = Some(io_bus.clone());
        self.main_bus = Some(main_bus.clone());
        self.display_ram = Some(Rc::new(RefCell::new(DisplayRAM::new(
            main_bus.clone(),
            self.screen_bus.clone(),
        ))));

        self.display_adapter_active_bit.update(false, true);
        self.display_adapter_active_bit.update(false, false);

        self.write_to_ram.update(false, true);
        self.write_to_ram.update(false, false);
    }

    fn update(&mut self) {
        // check if bus = 0x0007

        self.address_select_not_gates[0]
            .update(self.main_bus.as_ref().unwrap().borrow().get_output_wire(8));
        self.address_select_not_gates[1]
            .update(self.main_bus.as_ref().unwrap().borrow().get_output_wire(9));
        self.address_select_not_gates[2]
            .update(self.main_bus.as_ref().unwrap().borrow().get_output_wire(10));
        self.address_select_not_gates[3]
            .update(self.main_bus.as_ref().unwrap().borrow().get_output_wire(11));
        self.address_select_not_gates[4]
            .update(self.main_bus.as_ref().unwrap().borrow().get_output_wire(12));
        self.address_select_and_gate.update(
            self.address_select_not_gates[0].get(),
            self.address_select_not_gates[1].get(),
            self.address_select_not_gates[2].get(),
            self.address_select_not_gates[3].get(),
            self.address_select_not_gates[4].get(),
            self.main_bus.as_ref().unwrap().borrow().get_output_wire(13),
            self.main_bus.as_ref().unwrap().borrow().get_output_wire(14),
            self.main_bus.as_ref().unwrap().borrow().get_output_wire(15),
        );
        self.is_address_output_mode_gate.update(
            self.io_bus.as_ref().unwrap().borrow().is_set(),
            self.io_bus.as_ref().unwrap().borrow().is_address_mode(),
            self.io_bus.as_ref().unwrap().borrow().is_output_mode(),
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

struct ScreenControl {
    adapter: DisplayAdapter,
    input_bus: Option<Bus>,
    // 	outputChan  *[160][240]byte
    // clock: ticker,
    quit: bool,
    //y, x
    output: [[u8; 240]; 160],
}

impl ScreenControl {
    //TODO:sync
}
