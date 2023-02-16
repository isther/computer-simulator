use crate::gates::{Wire, AND, NAND, NOT};

mod adder;
mod bus;
mod comparator;
mod decoder;
mod gaters;
mod gates;
mod register;
mod storage;

pub use adder::Adder;
pub use bus::Bus;
pub use comparator::Comparator;
pub use decoder::{Decoder3x8, Decoder8x256};
pub use gaters::{ANDer, NOTer, ORer, XORer};
pub use gates::{ANDGate3, ANDGate4};
pub use register::Register;
pub use storage::Bit16;

pub const BUS_WIDTH: i32 = 16;

pub trait Component: ComponentClone {
    // fn connect_output(&mut self, component: Box<dyn Component>);
    fn set_input_wire(&mut self, i: i32, value: bool);
    fn get_output_wire(&self, i: i32) -> bool;
}

trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    // T: 'static + Study + Clone,
    T: 'static + Component + Clone,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}

fn set_input_value(c: &mut dyn Component, value: i32, start: i32, end: i32) {
    let mut x: u16 = 0;
    for i in (end..start).rev() {
        match value & (1 << x) {
            0 => {
                c.set_input_wire(i, false);
            }
            _ => {
                c.set_input_wire(i, true);
            }
        }

        x = x + 1;
    }
}

fn set_component_value_16(c: &mut dyn Component, value: i32) {
    set_input_value(c, value, 16, 0);
}

fn set_component_value_32(c: &mut dyn Component, input_a: i32, input_b: i32) {
    set_input_value(c, input_a, 16, 0);
    set_input_value(c, input_b, 32, 16);
}

fn get_output_value(c: &mut dyn Component, output_bits: i32) -> i32 {
    let mut x: u16 = 0;
    let mut result: i32 = 0;
    for i in (0..output_bits).rev() {
        match c.get_output_wire(i) {
            true => result = result | (1 << x),
            false => result = result ^ (result & (1 << x)),
        };
        x += 1;
    }
    result
}

#[derive(Clone)]
pub struct Enabler {
    inputs: [Wire; BUS_WIDTH as usize],
    gates: [AND; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Box<dyn Component>>,
}

impl Enabler {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            gates: (0..BUS_WIDTH)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            next: None,
        }
    }

    pub fn update(&mut self, enable: bool) {
        for i in 0..self.gates.len() {
            self.gates[i].update(self.inputs[i].get(), enable);
            self.outputs[i].update(self.gates[i].get());
        }

        match &self.next {
            Some(_) => {
                for i in 0..self.outputs.len() {
                    self.next
                        .as_mut()
                        .unwrap()
                        .set_input_wire(i as i32, self.outputs[i].get());
                }
            }
            None => {}
        }
    }
}

impl Component for Enabler {
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }
}

#[derive(Clone)]
pub struct LeftShifter {
    inputs: [Wire; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    shift_in: Wire,
    shift_out: Wire,
    next: Option<Box<dyn Component>>,
}

impl LeftShifter {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            shift_in: Wire::new("Z".to_string(), false),
            shift_out: Wire::new("Z".to_string(), false),
            next: None,
        }
    }

    pub fn get(&self) -> bool {
        self.shift_out.get()
    }

    pub fn update(&mut self, shift_in: bool) {
        self.shift_in.update(shift_in);
        self.shift_out.update(self.inputs[0].get());

        for i in 1..self.inputs.len() {
            self.outputs[i - 1].update(self.inputs[i].get());
        }

        self.outputs[15].update(self.shift_in.get());
    }
}

impl Component for LeftShifter {
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }
}

#[derive(Clone)]
pub struct RightShifter {
    inputs: [Wire; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    shift_in: Wire,
    shift_out: Wire,
    next: Option<Box<dyn Component>>,
}

impl RightShifter {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            shift_in: Wire::new("Z".to_string(), false),
            shift_out: Wire::new("Z".to_string(), false),
            next: None,
        }
    }

    pub fn get(&self) -> bool {
        self.shift_out.get()
    }

    pub fn update(&mut self, shift_in: bool) {
        self.shift_in.update(shift_in);
        self.outputs[0].update(self.shift_in.get());

        for i in 1..self.outputs.len() {
            self.outputs[i].update(self.inputs[i - 1].get());
        }

        self.shift_out.update(self.inputs[15].get());
    }
}

impl Component for RightShifter {
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }
}

#[derive(Clone)]
pub struct IsZero {
    inputs: [Wire; BUS_WIDTH as usize],
    orer: ORer,
    not_gate: NOT,
    output: Wire,
}

impl IsZero {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            orer: ORer::new(),
            not_gate: NOT::new(),
            output: Wire::new("Z".to_string(), false),
        }
    }

    pub fn reset(&mut self) {
        self.output.update(false)
    }

    pub fn update(&mut self) {
        for i in 0..self.inputs.len() {
            self.orer.set_input_wire(i as i32, self.inputs[i].get());
            self.orer
                .set_input_wire(i as i32 + BUS_WIDTH, self.inputs[i].get());
        }

        self.orer.update();

        for i in 0..self.orer.outputs.len() {
            match self.orer.outputs[i].get() {
                true => {
                    self.not_gate.update(true);
                    self.output.update(self.not_gate.get());
                    break;
                }
                false => self.not_gate.update(false),
            };
        }
        self.output.update(self.not_gate.get());
    }
}

impl Component for IsZero {
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }
    fn get_output_wire(&self, _: i32) -> bool {
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enabler() {
        todo!()
    }

    #[test]
    fn test_left_shifter() {
        todo!()
    }

    #[test]
    fn test_right_shifter() {
        todo!()
    }

    #[test]
    fn test_is_zero() {
        todo!()
    }
}
