use crate::gates::{Wire, AND, OR};

#[derive(Debug, Clone)]
pub struct ANDGate3 {
    pub input_a: Wire,
    pub input_b: Wire,
    pub input_c: Wire,
    pub and_a: AND,
    pub and_b: AND,
    pub output: Wire,
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
    pub input_a: Wire,
    pub input_b: Wire,
    pub input_c: Wire,
    pub input_d: Wire,
    pub and_a: AND,
    pub and_b: AND,
    pub and_c: AND,
    pub output: Wire,
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

pub struct ANDGate8 {
    and_a: AND,
    and_b: AND,
    and_c: AND,
    and_d: AND,
    and_e: AND,
    and_f: AND,
    and_g: AND,
    output: Wire,
}

impl ANDGate8 {
    pub fn new() -> Self {
        Self {
            and_a: AND::new(),
            and_b: AND::new(),
            and_c: AND::new(),
            and_d: AND::new(),
            and_e: AND::new(),
            and_f: AND::new(),
            and_g: AND::new(),
            output: Wire::new("O".to_string(), false),
        }
    }

    pub fn update(
        &mut self,
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        e: bool,
        f: bool,
        g: bool,
        h: bool,
    ) {
        self.and_a.update(a, b);
        self.and_b.update(self.and_a.get(), c);
        self.and_c.update(self.and_b.get(), d);
        self.and_d.update(self.and_c.get(), e);
        self.and_e.update(self.and_d.get(), f);
        self.and_f.update(self.and_e.get(), g);
        self.and_g.update(self.and_f.get(), h);
        self.output.update(self.and_g.get())
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }
}

pub struct ORGate3 {
    pub input_a: Wire,
    pub input_b: Wire,
    pub input_c: Wire,
    or_a: OR,
    or_b: OR,
    output: Wire,
}

impl ORGate3 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            or_a: OR::new(),
            or_b: OR::new(),
            output: Wire::new("O".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, input_a: bool, input_b: bool, input_c: bool) {
        self.or_a.update(input_a, input_b);
        self.or_b.update(self.or_a.get(), input_c);

        self.output.update(self.or_b.get());
    }
}

pub struct ORGate4 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,
    input_d: Wire,
    or_a: OR,
    or_b: OR,
    or_c: OR,
    output: Wire,
}

impl ORGate4 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            input_d: Wire::new("D".to_string(), false),
            or_a: OR::new(),
            or_b: OR::new(),
            or_c: OR::new(),
            output: Wire::new("O".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, input_a: bool, input_b: bool, input_c: bool, input_d: bool) {
        self.or_a.update(input_a, input_b);
        self.or_b.update(self.or_a.get(), input_c);
        self.or_c.update(self.or_b.get(), input_d);

        self.output.update(self.or_c.get());
    }
}

pub struct ORGate5 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,
    input_d: Wire,
    input_e: Wire,
    or_a: OR,
    or_b: OR,
    or_c: OR,
    or_d: OR,
    output: Wire,
}

impl ORGate5 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            input_d: Wire::new("D".to_string(), false),
            input_e: Wire::new("E".to_string(), false),
            or_a: OR::new(),
            or_b: OR::new(),
            or_c: OR::new(),
            or_d: OR::new(),
            output: Wire::new("O".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(
        &mut self,
        input_a: bool,
        input_b: bool,
        input_c: bool,
        input_d: bool,
        input_e: bool,
    ) {
        self.or_a.update(input_a, input_b);
        self.or_b.update(self.or_a.get(), input_c);
        self.or_c.update(self.or_b.get(), input_d);
        self.or_d.update(self.or_c.get(), input_e);

        self.output.update(self.or_d.get());
    }
}

pub struct ORGate6 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,
    input_d: Wire,
    input_e: Wire,
    input_f: Wire,
    or_a: OR,
    or_b: OR,
    or_c: OR,
    or_d: OR,
    or_e: OR,
    output: Wire,
}

impl ORGate6 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            input_c: Wire::new("C".to_string(), false),
            input_d: Wire::new("D".to_string(), false),
            input_e: Wire::new("E".to_string(), false),
            input_f: Wire::new("F".to_string(), false),
            or_a: OR::new(),
            or_b: OR::new(),
            or_c: OR::new(),
            or_d: OR::new(),
            or_e: OR::new(),
            output: Wire::new("O".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(
        &mut self,
        input_a: bool,
        input_b: bool,
        input_c: bool,
        input_d: bool,
        input_e: bool,
        input_f: bool,
    ) {
        self.or_a.update(input_a, input_b);
        self.or_b.update(self.or_a.get(), input_c);
        self.or_c.update(self.or_b.get(), input_d);
        self.or_d.update(self.or_c.get(), input_e);
        self.or_e.update(self.or_d.get(), input_f);

        self.output.update(self.or_e.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_gate_3() {
        let test_one_and_gate_3 = |input_a: bool, input_b: bool, input_c: bool, output: bool| {
            let mut and_gate_3 = ANDGate3::new();
            and_gate_3.update(input_a, input_b, input_c);
            assert_eq!(and_gate_3.get(), output);
        };
        test_one_and_gate_3(false, false, false, false);
        test_one_and_gate_3(false, false, true, false);
        test_one_and_gate_3(false, true, false, false);
        test_one_and_gate_3(false, true, true, false);
        test_one_and_gate_3(true, false, false, false);
        test_one_and_gate_3(true, false, true, false);
        test_one_and_gate_3(true, true, false, false);
        test_one_and_gate_3(true, true, true, true);
    }

    #[test]
    fn test_and_gate4() {}

    #[test]
    fn test_and_gate8() {}
}
