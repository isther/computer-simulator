use super::{Component, BUS_WIDTH};
use crate::gates::{Wire, AND, NOT, OR, XOR};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ANDer {
    inputs: [Wire; (BUS_WIDTH * 2) as usize],
    pub gates: [AND; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Arc<Mutex<dyn Component>>>,
}

impl ANDer {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH * 2)
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

    pub fn update(&mut self) {
        let mut a_wire = BUS_WIDTH;
        let mut b_wire = 0;
        for i in 0..self.gates.len() {
            self.gates[i].update(
                self.inputs[a_wire as usize].get(),
                self.inputs[b_wire as usize].get(),
            );
            self.outputs[i].update(self.gates[i].get());

            a_wire += 1;
            b_wire += 1;
        }
    }
}

impl Component for ANDer {
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
pub struct NOTer {
    inputs: [Wire; BUS_WIDTH as usize],
    pub gates: [NOT; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Arc<Mutex<dyn Component>>>,
}

impl NOTer {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            gates: (0..BUS_WIDTH)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
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

    pub fn update(&mut self) {
        for i in 0..self.gates.len() {
            self.gates[i].update(self.inputs[i].get());
            self.outputs[i].update(self.gates[i].get());
        }
    }
}

impl Component for NOTer {
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
pub struct ORer {
    inputs: [Wire; (BUS_WIDTH * 2) as usize],
    pub gates: [OR; BUS_WIDTH as usize],
    pub outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Arc<Mutex<dyn Component>>>,
}

impl ORer {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH * 2)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            gates: (0..BUS_WIDTH)
                .map(|_| OR::new())
                .collect::<Vec<OR>>()
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

    pub fn update(&mut self) {
        let mut a_wire = BUS_WIDTH;
        let mut b_wire = 0;
        for i in 0..self.gates.len() {
            self.gates[i].update(
                self.inputs[a_wire as usize].get(),
                self.inputs[b_wire as usize].get(),
            );
            self.outputs[i].update(self.gates[i].get());

            a_wire += 1;
            b_wire += 1;
        }
    }
}

impl Component for ORer {
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
pub struct XORer {
    inputs: [Wire; (BUS_WIDTH * 2) as usize],
    pub gates: [XOR; BUS_WIDTH as usize],
    outputs: [Wire; BUS_WIDTH as usize],
    next: Option<Arc<Mutex<dyn Component>>>,
}

impl XORer {
    pub fn new() -> Self {
        Self {
            inputs: (0..BUS_WIDTH * 2)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            gates: (0..BUS_WIDTH)
                .map(|_| XOR::new())
                .collect::<Vec<XOR>>()
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

    pub fn update(&mut self) {
        let mut a_wire = BUS_WIDTH;
        let mut b_wire = 0;
        for i in 0..self.gates.len() {
            self.gates[i].update(
                self.inputs[a_wire as usize].get(),
                self.inputs[b_wire as usize].get(),
            );
            self.outputs[i].update(self.gates[i].get());

            a_wire += 1;
            b_wire += 1;
        }
    }
}

impl Component for XORer {
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

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_ander() {
        let test_fn = |input_a: i32, input_b: i32, output: i32| {
            let mut ander = Box::new(ANDer::new());
            set_component_value_32(ander.as_mut(), input_a, input_b);
            ander.update();

            assert_eq!(get_output_value(ander.as_mut(), BUS_WIDTH), output);
        };

        test_fn(0xFFFF, 0xFFFF, 0xFFFF);
        test_fn(0x0000, 0xFFFF, 0x0000);
        test_fn(0xFF00, 0xFFFF, 0xFF00);
        test_fn(0xFF00, 0x00FF, 0x0000);
    }

    #[test]
    fn test_noter() {
        let test_fn = |input: i32, output: i32| {
            let mut noter = Box::new(NOTer::new());
            set_component_value_16(noter.as_mut(), input);
            noter.update();

            assert_eq!(get_output_value(noter.as_mut(), BUS_WIDTH), output);
        };

        test_fn(0xFFFF, 0x0000);
        test_fn(0xFF00, 0x00FF);
        test_fn(0x0000, 0xFFFF);
    }

    #[test]
    fn test_orer() {
        let test_fn = |input_a: i32, input_b: i32, output: i32| {
            let mut orer = Box::new(ORer::new());
            set_component_value_32(orer.as_mut(), input_a, input_b);
            orer.update();

            assert_eq!(get_output_value(orer.as_mut(), BUS_WIDTH), output);
        };

        test_fn(0xFFFF, 0x0000, 0xFFFF);
        test_fn(0x0000, 0xFFFF, 0xFFFF);
        test_fn(0xFF00, 0x00FF, 0xFFFF);
        test_fn(0xFF00, 0x0000, 0xFF00);
        test_fn(0x0000, 0x0000, 0x0000);
    }

    #[test]
    fn test_xorer() {
        let test_fn = |input_a: i32, input_b: i32, output: i32| {
            let mut xorer = Box::new(XORer::new());
            set_component_value_32(xorer.as_mut(), input_a, input_b);
            xorer.update();

            assert_eq!(get_output_value(xorer.as_mut(), BUS_WIDTH), output);
        };

        test_fn(0xFFFF, 0x0000, 0xFFFF);
        test_fn(0x0000, 0xFFFF, 0xFFFF);
        test_fn(0xFF00, 0x00FF, 0xFFFF);
        test_fn(0xFF00, 0x0000, 0xFF00);
        test_fn(0x0000, 0x0000, 0x0000);
        test_fn(0xFFFF, 0xFFFF, 0x0000);
    }
}
