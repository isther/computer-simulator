use super::{Bit, Wire, AND, NOT, OR};
use std::fmt::Display;

pub struct Stepper {
    pub input_or_gates: [OR; 2],
    pub reset: Wire,
    pub reset_not_gate: NOT,
    pub clock_in: Wire,
    pub clock_in_not_gate: NOT,
    pub bits: [Bit; 12],
    pub outputs: [Wire; 7],
    pub output_or_gate: OR,
    pub output_and_gates: [AND; 5],
    pub output_not_gates: [NOT; 6],
}

impl Stepper {
    pub fn new() -> Self {
        Self {
            bits: (0..12)
                .map(|_| Bit::new())
                .collect::<Vec<Bit>>()
                .try_into()
                .unwrap(),
            reset: Wire::new("R".to_string(), false),
            reset_not_gate: NOT::new(),
            clock_in: Wire::new("C".to_string(), false),
            clock_in_not_gate: NOT::new(),
            input_or_gates: (0..2)
                .map(|_| OR::new())
                .collect::<Vec<OR>>()
                .try_into()
                .unwrap(),
            outputs: (0..7)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            output_or_gate: OR::new(),
            output_and_gates: (0..5)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            output_not_gates: (0..6)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn update(&mut self, clock_in: bool) {
        self.clock_in.update(clock_in);
        self.reset.update(self.outputs[6].get());

        self.step();

        if self.outputs[6].get() {
            self.reset.update(self.outputs[6].get());
            self.step();
        }
    }

    pub fn step(&mut self) {
        self.clock_in_not_gate.update(self.clock_in.get());
        self.reset_not_gate.update(self.reset.get());

        self.input_or_gates[0].update(self.reset.get(), self.clock_in_not_gate.get());
        self.input_or_gates[1].update(self.reset.get(), self.clock_in.get());

        self.bits[0].update(self.reset_not_gate.get(), self.input_or_gates[0].get());
        self.bits[1].update(self.bits[0].get(), self.input_or_gates[1].get());
        self.output_not_gates[0].update(self.bits[1].get());
        self.output_or_gate
            .update(self.output_not_gates[0].get(), self.reset.get());

        self.bits[2].update(self.bits[1].get(), self.input_or_gates[0].get());
        self.bits[3].update(self.bits[2].get(), self.input_or_gates[1].get());
        self.output_not_gates[1].update(self.bits[3].get());
        self.output_and_gates[0].update(self.output_not_gates[1].get(), self.bits[1].get());

        self.bits[4].update(self.bits[3].get(), self.input_or_gates[0].get());
        self.bits[5].update(self.bits[4].get(), self.input_or_gates[1].get());
        self.output_not_gates[2].update(self.bits[5].get());
        self.output_and_gates[1].update(self.output_not_gates[2].get(), self.bits[3].get());

        self.bits[6].update(self.bits[5].get(), self.input_or_gates[0].get());
        self.bits[7].update(self.bits[6].get(), self.input_or_gates[1].get());
        self.output_not_gates[3].update(self.bits[7].get());
        self.output_and_gates[2].update(self.output_not_gates[3].get(), self.bits[5].get());

        self.bits[8].update(self.bits[7].get(), self.input_or_gates[0].get());
        self.bits[9].update(self.bits[8].get(), self.input_or_gates[1].get());
        self.output_not_gates[4].update(self.bits[9].get());
        self.output_and_gates[3].update(self.output_not_gates[4].get(), self.bits[7].get());

        self.bits[10].update(self.bits[9].get(), self.input_or_gates[0].get());
        self.bits[11].update(self.bits[10].get(), self.input_or_gates[1].get());
        self.output_not_gates[5].update(self.bits[11].get());
        self.output_and_gates[4].update(self.output_not_gates[5].get(), self.bits[9].get());

        self.outputs[0].update(self.output_or_gate.get());
        self.outputs[1].update(self.output_and_gates[0].get());
        self.outputs[2].update(self.output_and_gates[1].get());
        self.outputs[3].update(self.output_and_gates[2].get());
        self.outputs[4].update(self.output_and_gates[3].get());
        self.outputs[5].update(self.output_and_gates[4].get());
        self.outputs[6].update(self.bits[11].get());
    }

    pub fn get_output_wire(&self, index: i32) -> bool {
        self.outputs[index as usize].get()
    }
}

impl Display for Stepper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for i in 0..(self.outputs.len() - 1) {
            match self.outputs[i].get() {
                true => result.push_str("* "),
                false => result.push_str("- "),
            }
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_stepper() {
        let test_one_stepper = |cycles: u16, expected: i32| {
            let stepper = Rc::new(RefCell::new(Stepper::new()));
            for _ in 0..cycles {
                stepper.borrow_mut().update(true);
                stepper.borrow_mut().update(false);
            }
            println!("Stepper: {}", stepper.borrow());

            assert_eq!(expected, get_output(stepper.clone()));
        };

        test_one_stepper(0, -1);
        test_one_stepper(1, 1);
        test_one_stepper(2, 2);
        test_one_stepper(3, 3);
        test_one_stepper(4, 4);
        test_one_stepper(5, 5);
        test_one_stepper(6, 6);
    }

    fn get_output(stepper: Rc<RefCell<Stepper>>) -> i32 {
        for i in 0..7 {
            if stepper.borrow().get_output_wire(i) {
                return i + 1;
            }
        }
        return -1;
    }
}
