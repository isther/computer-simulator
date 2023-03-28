use super::{Component, Wire, NAND};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Bit {
    pub gates: [NAND; 4],
    pub wire_o: Wire,
}

impl Bit {
    pub fn new() -> Self {
        Self {
            gates: (0..4)
                .map(|_| NAND::new())
                .collect::<Vec<NAND>>()
                .try_into()
                .unwrap(),
            wire_o: Wire::new("O".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.wire_o.get()
    }

    pub fn update(&mut self, wire_i: bool, wire_s: bool) {
        for _ in 0..2 {
            self.gates[0].update(wire_i, wire_s);
            self.gates[1].update(self.gates[0].get(), wire_s);
            self.gates[2].update(self.gates[0].get(), self.gates[3].get());
            self.gates[3].update(self.gates[2].get(), self.gates[1].get());
            self.wire_o.update(self.gates[2].get());
        }
    }
}

#[derive(Clone)]
pub struct Word {
    inputs: [Wire; 16],
    pub bits: [Bit; 16],
    outputs: [Wire; 16],
    next: Option<Arc<Mutex<dyn Component>>>,
}

impl Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(self.bits.iter().map(|b| format!("{}", b.get() as u32)))
        )
    }
}

impl Word {
    pub fn new() -> Self {
        Self {
            inputs: (0..16)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            bits: (0..16)
                .map(|_| Bit::new())
                .collect::<Vec<Bit>>()
                .try_into()
                .unwrap(),
            outputs: (0..16)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            next: None,
        }
    }

    pub fn update(&mut self, set: bool) {
        for i in 0..self.inputs.len() {
            self.bits[i].update(self.inputs[i].get(), set);
            self.outputs[i].update(self.bits[i].get());
        }

        match &self.next {
            Some(next) => {
                for i in 0..self.outputs.len() {
                    next.lock()
                        .unwrap()
                        .set_input_wire(i as i32, self.outputs[i].get());
                }
            }
            _ => {}
        };
    }
}

impl Component for Word {
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
    use super::*;

    #[test]
    fn test_bit() {
        let mut bit = Bit::new();

        bit.update(false, true);
        assert_eq!(bit.get(), false);

        bit.update(false, false);
        assert_eq!(bit.get(), false);

        bit.update(true, true);
        assert_eq!(bit.get(), true);

        bit.update(false, false);
        assert_eq!(bit.get(), true);
    }
}
