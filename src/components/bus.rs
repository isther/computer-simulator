use super::Component;
use crate::gates::Wire;
use std::fmt::Display;

#[derive(Clone)]
pub struct Bus {
    wires: Box<Vec<Wire>>,
    width: i32,
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
}

impl Component for Bus {
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.wires[i as usize].update(value)
    }
    fn get_output_wire(&self, i: i32) -> bool {
        self.wires[i as usize].get()
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "bus: {}",
            String::from_iter(self.wires.iter().map(|v| format!("{}", v.get() as u32))),
        )
    }
}
