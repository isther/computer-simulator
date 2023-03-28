use crate::gates::{Wire, AND, NAND, NOT, OR};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

mod adder;
mod bus;
mod busone;
mod comparator;
mod decoder;
mod gaters;
mod gates;
mod iobus;
mod register;
mod stepper;
mod storage;

pub use adder::Adder;
pub use bus::Bus;
pub use busone::BusOne;
pub use comparator::Comparator;
pub use decoder::{Decoder2x4, Decoder3x8, Decoder8x256};
pub use gaters::{ANDer, NOTer, ORer, XORer};
pub use gates::{ANDGate3, ANDGate4, ANDGate5, ANDGate8, ORGate3, ORGate4, ORGate5, ORGate6};
pub use iobus::{IOBus, Mode};
pub use register::Register;
pub use stepper::Stepper;
pub use storage::{Bit, Word};

pub const BUS_WIDTH: i32 = 16;

pub trait Enableable {
    fn enable(&mut self);
    fn disable(&mut self);
}

pub trait Settable {
    fn set(&mut self);
    fn unset(&mut self);
}

pub trait Updatable {
    fn update(&mut self);
}

pub trait Component: ComponentClone + Send {
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>);
    fn set_input_wire(&mut self, i: i32, value: bool);
    fn get_output_wire(&self, i: i32) -> bool;
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
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

fn set_input_value<T>(c: &mut T, value: i32, start: i32, end: i32)
where
    T: Component,
{
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

fn set_component_value_16<T>(c: &mut T, value: i32)
where
    T: Component,
{
    set_input_value(c, value, 16, 0);
}

fn set_component_value_32<T>(c: &mut T, input_a: i32, input_b: i32)
where
    T: Component,
{
    set_input_value(c, input_a, 16, 0);
    set_input_value(c, input_b, 32, 16);
}

fn get_output_value<T>(c: &T, output_bits: i32) -> i32
where
    T: Component,
{
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
    pub gates: [AND; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Arc<Mutex<dyn Component>>>,
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
            Some(next) => {
                for i in 0..self.outputs.len() {
                    next.lock()
                        .unwrap()
                        .set_input_wire(i as i32, self.outputs[i].get());
                }
            }
            None => {}
        }
    }
}

impl Debug for Enabler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for i in 0..self.outputs.len() {
            match self.get_output_wire(i as i32) {
                true => result += "1",
                false => result += "0",
            }
        }
        write!(f, r#"Enabler: {}"#, result)
    }
}

impl Component for Enabler {
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>) {
        self.next = Some(component)
    }
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
    pub shift_in: Wire,
    pub shift_out: Wire,
    next: Option<Arc<Mutex<dyn Component>>>,
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
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>) {
        self.next = Some(component)
    }
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
    pub shift_in: Wire,
    pub shift_out: Wire,
    next: Option<Arc<Mutex<dyn Component>>>,
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
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>) {
        self.next = Some(component)
    }
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
    pub orer: ORer,
    pub not_gate: NOT,
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
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>) {}
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
        let mut enabler = Enabler::new();
        for i in 0..enabler.inputs.len() {
            enabler.set_input_wire(i as i32, true);
        }

        enabler.update(false);
        for i in 0..enabler.outputs.len() {
            assert_eq!(enabler.get_output_wire(i as i32), false);
        }

        enabler.update(true);
        for i in 0..enabler.outputs.len() {
            assert_eq!(enabler.get_output_wire(i as i32), true);
        }
    }

    #[test]
    fn test_left_shifter() {
        let test_left_shifter =
            |input: i32, shift_in: bool, expected_output: i32, expected_shift_out: bool| {
                let mut left_shifter = Box::new(LeftShifter::new());
                set_component_value_16(left_shifter.as_mut(), input);
                left_shifter.update(shift_in);
                assert_eq!(
                    get_output_value(left_shifter.as_ref(), BUS_WIDTH),
                    expected_output,
                );
                assert_eq!(left_shifter.shift_out.get(), expected_shift_out);
            };
        for i in 0..0x7FFF {
            test_left_shifter(i, false, i * 2, false);
        }

        test_left_shifter(0, false, 0, false);
        test_left_shifter(0x8000, false, 0, true);
        test_left_shifter(0xEEEF, false, 0xDDDE, true);
        test_left_shifter(0xFFFF, false, 0xFFFE, true);
        test_left_shifter(0x0000, true, 0x0001, false);
        test_left_shifter(0x8000, true, 0x0001, true);
    }

    #[test]
    fn test_right_shifter() {
        let test_right_shifter =
            |input: i32, shift_in: bool, expected_output: i32, expected_shift_out: bool| {
                let mut right_shifter = Box::new(RightShifter::new());
                set_component_value_16(right_shifter.as_mut(), input);
                right_shifter.update(shift_in);
                assert_eq!(
                    get_output_value(right_shifter.as_ref(), BUS_WIDTH),
                    expected_output,
                );
                assert_eq!(right_shifter.shift_out.get(), expected_shift_out);
            };
        let mut i: i32 = 0x8000;
        while i > 1 {
            test_right_shifter(i, false, i / 2, false);
            i /= 2;
        }

        test_right_shifter(0, false, 0, false);
        test_right_shifter(0x0001, false, 0x0000, true);
        test_right_shifter(0x8000, false, 0x4000, false);
        test_right_shifter(0xEEEF, false, 0x7777, true);
        test_right_shifter(0xFFFF, false, 0x7FFF, true);
        test_right_shifter(0x0000, true, 0x8000, false);
        test_right_shifter(0x8000, true, 0xC000, false);
        test_right_shifter(0x4AAA, true, 0xA555, false);
    }

    #[test]
    fn test_is_zero() {
        let mut is_zero = IsZero::new();

        for i in 0..BUS_WIDTH {
            is_zero.set_input_wire(i, false);
        }
        is_zero.update();
        assert_eq!(is_zero.output.get(), true);

        for i in 0..BUS_WIDTH {
            is_zero.set_input_wire(i, true);
        }
        is_zero.update();
        assert_eq!(is_zero.output.get(), false);

        for i in 0..BUS_WIDTH {
            let mut is_zero = IsZero::new();
            is_zero.set_input_wire(i, true);
            is_zero.update();
            assert_eq!(is_zero.output.get(), false);
        }
    }
}
