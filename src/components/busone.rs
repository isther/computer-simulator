use super::{Bus, Component, Enableable, Updatable, Wire, AND, BUS_WIDTH, NOT, OR};
use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct BusOne {
    pub input_bus: Arc<Mutex<Bus>>,
    pub output_bus: Arc<Mutex<Bus>>,
    pub inputs: [Wire; BUS_WIDTH as usize],
    pub bus1: Wire,
    pub and_gates: [AND; (BUS_WIDTH - 1) as usize],
    pub not_gate: NOT,
    pub or_gate: OR,
    pub outputs: [Wire; BUS_WIDTH as usize],
    pub next: Option<Rc<RefCell<dyn Component>>>,
}

impl BusOne {
    pub fn new(input_bus: Arc<Mutex<Bus>>, output_bus: Arc<Mutex<Bus>>) -> Self {
        Self {
            input_bus,
            output_bus,
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            bus1: Wire::new("Z".to_string(), false),
            and_gates: (0..BUS_WIDTH - 1)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            not_gate: NOT::new(),
            or_gate: OR::new(),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            next: None,
        }
    }

    pub fn value(&self) -> u16 {
        let mut value: u16 = 0;
        let mut x: i32 = 0;
        for i in (0..BUS_WIDTH).rev() {
            match self.outputs[i as usize].get() {
                true => value = value | (1 << (x as u16)),
                false => value = value & (value ^ (1 << (x as u16))),
            }
            x += 1;
        }
        value
    }
}

impl Display for BusOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BusOne: {} ", self.value())
    }
}

impl Component for BusOne {
    fn connect_output(&mut self, component: Rc<RefCell<dyn Component>>) {
        self.next = Some(component)
    }

    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }
}

impl Updatable for BusOne {
    fn update(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.inputs[i as usize].update(self.input_bus.lock().unwrap().get_output_wire(i))
        }
        self.not_gate.update(self.bus1.get());

        for i in 0..self.and_gates.len() {
            self.and_gates[i].update(self.inputs[i].get(), self.not_gate.get());
        }
        self.or_gate.update(self.inputs[15].get(), self.bus1.get());

        for i in 0..(self.outputs.len() - 1) {
            self.outputs[i].update(self.and_gates[i].get());
        }
        self.outputs[15].update(self.or_gate.get());

        for i in (0..BUS_WIDTH).rev() {
            self.output_bus
                .lock()
                .unwrap()
                .set_input_wire(i, self.outputs[i as usize].get())
        }
    }
}

impl Enableable for BusOne {
    fn enable(&mut self) {
        self.bus1.update(true)
    }

    fn disable(&mut self) {
        self.bus1.update(false)
    }
}
