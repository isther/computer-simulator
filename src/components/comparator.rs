use super::{ANDGate3, Component, BUS_WIDTH};
use crate::gates::{Wire, AND, NOT, OR, XOR};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Comparator {
    pub inputs: [Wire; (BUS_WIDTH * 2) as usize],
    pub equal_in: Wire,
    pub a_is_larger_in: Wire,
    pub compares: [Compare2; BUS_WIDTH as usize],
    pub outputs: [Wire; BUS_WIDTH as usize],
    pub equal_out: Wire,
    pub a_is_larger_out: Wire,
    pub next: Option<Arc<Mutex<dyn Component>>>,
}

impl Comparator {
    pub fn new() -> Self {
        Self {
            inputs: (0..2 * BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            equal_in: Wire::new("Z".to_string(), false),
            a_is_larger_in: Wire::new("Z".to_string(), false),
            compares: (0..BUS_WIDTH)
                .map(|_| Compare2::new())
                .collect::<Vec<Compare2>>()
                .try_into()
                .unwrap(),
            outputs: (0..BUS_WIDTH)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            equal_out: Wire::new("Z".to_string(), false),
            a_is_larger_out: Wire::new("Z".to_string(), false),
            next: None,
        }
    }

    pub fn update(&mut self) {
        self.equal_in.update(true);
        self.a_is_larger_in.update(false);

        // top 16 bits are b, bottom 16 bits are a
        let mut a_wire = 0;
        let mut b_wire = BUS_WIDTH as usize;

        for i in 0..self.compares.len() {
            self.compares[i].update(
                self.inputs[a_wire].get(),
                self.inputs[b_wire].get(),
                self.equal_in.get(),
                self.a_is_larger_in.get(),
            );
            self.outputs[i].update(self.compares[i].get());
            self.equal_out.update(self.compares[i].equal());
            self.a_is_larger_out.update(self.compares[i].larger());

            self.equal_in.update(self.compares[i].equal());
            self.a_is_larger_in.update(self.compares[i].larger());
            a_wire += 1;
            b_wire += 1;
        }
    }

    pub fn equal(&self) -> bool {
        self.equal_out.get()
    }

    pub fn larger(&self) -> bool {
        self.a_is_larger_out.get()
    }
}

impl Component for Comparator {
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

#[derive(Debug, Clone)]
pub struct Compare2 {
    pub input_a: Wire,
    pub input_b: Wire,
    pub xor1: XOR,
    pub not1: NOT,
    pub and1: AND,
    pub and_gate3: ANDGate3,
    pub or1: OR,
    pub out: Wire,
    pub equal_in: Wire,
    pub equal_out: Wire,
    pub is_larger_in: Wire,
    pub is_larger_out: Wire,
}

impl Compare2 {
    fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            xor1: XOR::new(),
            not1: NOT::new(),
            and1: AND::new(),
            and_gate3: ANDGate3::new(),
            or1: OR::new(),
            out: Wire::new("Z".to_string(), false),
            equal_in: Wire::new("EqualIn".to_string(), false),
            equal_out: Wire::new("EqualOut".to_string(), false),
            is_larger_in: Wire::new("LargerIn".to_string(), false),
            is_larger_out: Wire::new("LargetOut".to_string(), false),
        }
    }

    fn equal(&self) -> bool {
        self.equal_out.get()
    }

    fn larger(&self) -> bool {
        self.is_larger_out.get()
    }

    fn get(&self) -> bool {
        self.out.get()
    }

    fn update(&mut self, input_a: bool, input_b: bool, equal_in: bool, is_larger_in: bool) {
        self.input_a.update(input_a);
        self.input_b.update(input_b);
        self.equal_in.update(equal_in);
        self.is_larger_in.update(is_larger_in);

        self.xor1.update(self.input_a.get(), self.input_b.get());
        self.not1.update(self.xor1.get());
        self.and1.update(self.not1.get(), self.equal_in.get());
        self.equal_out.update(self.and1.get());

        self.and_gate3
            .update(self.equal_in.get(), self.input_a.get(), self.xor1.get());
        self.or1
            .update(self.and_gate3.get(), self.is_larger_in.get());
        self.is_larger_out.update(self.or1.get());
        self.out.update(self.xor1.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::set_component_value_32;

    #[test]
    fn test_comparator() {
        let test_comparator =
            |input_a: i32, input_b: i32, expected_is_equal: bool, expected_is_larger: bool| {
                let mut comparator = Box::new(Comparator::new());
                set_component_value_32(comparator.as_mut(), input_a, input_b);

                comparator.update();

                assert_eq!(comparator.equal(), expected_is_equal);
                assert_eq!(comparator.larger(), expected_is_larger);
            };

        test_comparator(0, 0, true, false);
        test_comparator(1, 0, false, true);
        test_comparator(0, 1, false, false);
        test_comparator(0xFFFF, 0xFFFF, true, false);
        test_comparator(0xFF00, 0x00FF, false, true);
    }
}
