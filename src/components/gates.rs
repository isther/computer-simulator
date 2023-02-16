use crate::gates::{Wire, AND};

#[derive(Debug, Clone)]
pub struct ANDGate3 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,
    and_a: AND,
    and_b: AND,
    output: Wire,
}

impl ANDGate3 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            and_a: AND::new(),
            and_b: AND::new(),
            output: Wire::new("D".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, input_a: bool, input_b: bool, input_c: bool) {
        self.and_a.update(input_a, input_b);
        self.and_b.update(self.and_a.get(), input_c);

        self.output.update(self.and_b.get())
    }
}

#[derive(Debug, Clone)]
pub struct ANDGate4 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,
    input_d: Wire,
    and_a: AND,
    and_b: AND,
    and_c: AND,
    output: Wire,
}

impl ANDGate4 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            input_d: Wire::new("D".to_string(), false),
            output: Wire::new("O".to_string(), false),
            and_a: AND::new(),
            and_b: AND::new(),
            and_c: AND::new(),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, input_a: bool, input_b: bool, input_c: bool, input_d: bool) {
        self.and_a.update(input_a, input_b);
        self.and_b.update(self.and_a.get(), input_c);
        self.and_c.update(self.and_b.get(), input_d);
        self.output.update(self.and_c.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_one_and_gate_3(input_a: bool, input_b: bool, input_c: bool, output: bool) {
        let mut and_gate_3 = ANDGate3::new();
        and_gate_3.update(input_a, input_b, input_c);
        assert_eq!(and_gate_3.get(), output);
    }

    #[test]
    fn test_and_gate_3() {
        test_one_and_gate_3(false, false, false, false);
        test_one_and_gate_3(false, false, true, false);
        test_one_and_gate_3(false, true, false, false);
        test_one_and_gate_3(false, true, true, false);
        test_one_and_gate_3(true, false, false, false);
        test_one_and_gate_3(true, false, true, false);
        test_one_and_gate_3(true, true, false, false);
        test_one_and_gate_3(true, true, true, true);
    }
}
