use super::Component;
use crate::gates::Wire;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct Bus {
    wires: Box<Vec<Wire>>,
    pub width: i32,
}

impl Bus {
    pub fn new(width: i32) -> Self {
        Self {
            wires: Box::new(
                (0..width)
                    .map(|_| Wire::new("Z".to_string(), false))
                    .collect::<Vec<Wire>>()
                    .try_into()
                    .unwrap(),
            ),
            width,
        }
    }

    pub fn set_value(&mut self, value: u16) {
        let mut x: u16 = 0;
        for i in (0..self.width).rev() {
            match value & (1 << x) {
                0 => self.set_input_wire(i, false),
                _ => self.set_input_wire(i, true),
            }
            x += 1;
        }
    }

    pub fn get_value(&self) -> u16 {
        let mut x: u16 = 0;
        let mut result: u16 = 0;
        for i in (0..self.wires.len()).rev() {
            match self.get_output_wire(i as i32) {
                true => result = result | (1 << x),
                false => result = result & (result ^ (1 << x)),
            };
            x += 1;
        }

        result
    }
}

impl Component for Bus {
    fn connect_output(&mut self, component: Arc<Mutex<dyn Component>>) {}
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.wires[i as usize].update(value)
    }
    fn get_output_wire(&self, i: i32) -> bool {
        self.wires[i as usize].get()
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bus: {:>#06X}", self.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::BUS_WIDTH;

    #[test]
    fn test_bus() {
        let test_one_bus = |input: u16, ans: u16| {
            let mut bus = Bus::new(BUS_WIDTH);
            bus.set_value(input);

            assert_eq!(bus.get_value(), ans)
        };

        test_one_bus(0x0000, 0x0000);
        test_one_bus(0x00FF, 0x00FF);
        test_one_bus(0xFF00, 0xFF00);
        test_one_bus(0xFFFF, 0xFFFF);
    }
}
