use super::{Component, Wire, NAND};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Bit {
    pub gates: [NAND; 4],
    pub wire_o: Wire,
}

impl Bit {
    fn new() -> Self {
        Self {
            gates: (0..4)
                .map(|_| NAND::new())
                .collect::<Vec<NAND>>()
                .try_into()
                .unwrap(),
            wire_o: Wire::new("O".to_string(), false),
        }
    }

    fn get(&self) -> bool {
        self.wire_o.get()
    }

    fn update(&mut self, wire_i: bool, wire_s: bool) {
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
pub struct Bit16 {
    inputs: [Wire; 16],
    pub bits: [Bit; 16],
    outputs: [Wire; 16],
    next: Option<Rc<RefCell<dyn Component>>>,
}

//TODO:Debug info
impl Debug for Bit16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Bit16 {
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
                    next.borrow_mut()
                        .set_input_wire(i as i32, self.outputs[i].get());
                }
            }
            _ => {}
        };
    }
}

impl Component for Bit16 {
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
