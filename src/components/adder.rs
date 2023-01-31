use super::{get_output_value, set_component_value_32, Component, EmptyComponent, BUS_WIDTH};
use crate::gates::{Wire, AND, OR, XOR};
use std::fmt::Display;

struct Adder {
    inputs: [Wire; 32],
    carry_in: Wire,
    adds: [FullAdder; 16],
    carry_out: Wire,
    outputs: [Wire; 16],
    next: Box<dyn Component>,
}

impl Adder {
    fn new() -> Self {
        Self {
            inputs: (0..32)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            carry_in: Wire::new("Z".to_string(), false),
            adds: (0..16)
                .map(|_| FullAdder::new())
                .collect::<Vec<FullAdder>>()
                .try_into()
                .unwrap(),
            carry_out: Wire::new("Z".to_string(), false),
            outputs: (0..16)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            next: Box::new(EmptyComponent::new()),
        }
    }

    fn get_carry_out(&self) -> bool {
        self.carry_out.get()
    }

    fn update(&mut self, carry_in: bool) {
        self.carry_in.update(carry_in);

        let mut a_wire: i32 = 31;
        let mut b_wire: i32 = 15;
        let mut i: i32 = self.adds.len() as i32 - 1;
        while i >= 0 {
            let a_val = self.inputs[a_wire as usize].get();
            let b_val = self.inputs[b_wire as usize].get();

            self.adds[i as usize].update(a_val, b_val, self.carry_in.get());
            self.outputs[i as usize].update(self.adds[i as usize].get_sum());
            self.carry_out.update(self.adds[i as usize].get_carry_out());
            self.carry_in.update(self.adds[i as usize].get_carry_out());

            a_wire = a_wire - 1;
            b_wire = b_wire - 1;
            i = i - 1;
        }
    }
}

impl Display for Adder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "inputs: {}
inputsa: {}
inputsb: {}
carry_in: {}
outputs: {}
carry_out: {}",
            String::from_iter(self.inputs.iter().map(|v| format!("{}", v.get() as u32))),
            String::from_iter(
                self.inputs[0..16]
                    .iter()
                    .map(|v| format!("{}", v.get() as u32))
            ),
            String::from_iter(
                self.inputs[16..32]
                    .iter()
                    .map(|v| format!("{}", v.get() as u32))
            ),
            self.carry_in.get() as u32,
            String::from_iter(self.outputs.iter().map(|v| format!("{}", v.get() as u32))),
            self.carry_out.get() as u32,
        )
    }
}

impl Component for Adder {
    fn connect_output(&mut self, component: Box<dyn Component>) {
        self.next = component
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }
    fn set_input_wire(&mut self, i: i32, value: bool) {
        self.inputs[i as usize].update(value)
    }
}

#[derive(Debug)]
struct FullAdder {
    input_a: Wire,
    input_b: Wire,
    carry_in: Wire,

    xor1: XOR,
    xor2: XOR,
    and1: AND,
    and2: AND,
    or: OR,

    carry_out: Wire,
    sum: Wire,
}

impl FullAdder {
    fn new() -> Self {
        Self {
            input_a: Wire::new("A".to_string(), false),
            input_b: Wire::new("B".to_string(), false),
            carry_in: Wire::new("C".to_string(), false),
            xor1: XOR::new(),
            xor2: XOR::new(),
            and1: AND::new(),
            and2: AND::new(),
            or: OR::new(),
            carry_out: Wire::new("CO".to_string(), false),
            sum: Wire::new("SO".to_string(), false),
        }
    }

    fn update(&mut self, value_a: bool, value_b: bool, carry_in: bool) {
        self.input_a.update(value_a);
        self.input_b.update(value_b);
        self.carry_in.update(carry_in);

        self.xor1.update(self.input_a.get(), self.input_b.get());
        self.xor2.update(self.xor1.get(), self.carry_in.get());

        self.sum.update(self.xor2.get());

        self.and1.update(self.carry_in.get(), self.xor1.get());
        self.and2.update(self.input_a.get(), self.input_b.get());

        self.or.update(self.and1.get(), self.and2.get());
        self.carry_out.update(self.or.get())
    }

    fn get_sum(&self) -> bool {
        self.sum.get()
    }

    fn get_carry_out(&self) -> bool {
        self.carry_out.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_one_full_adder(
        a: bool,
        b: bool,
        carry_in: bool,
        sum: bool,
        carry_out: bool,
        code: &str,
    ) {
        let mut full_adder = FullAdder::new();
        full_adder.update(a, b, carry_in);
        assert_eq!(full_adder.get_sum(), sum, "{}", code);
        assert_eq!(full_adder.get_carry_out(), carry_out, "{}", code);
    }

    #[test]
    fn test_full_adder() {
        test_one_full_adder(false, false, false, false, false, "1");
        test_one_full_adder(true, false, false, true, false, "2");
        test_one_full_adder(false, true, false, true, false, "3");
        test_one_full_adder(false, false, true, true, false, "4");
        test_one_full_adder(true, true, false, false, true, "5");
        test_one_full_adder(false, true, true, false, true, "6");
        test_one_full_adder(true, false, true, false, true, "7");
        test_one_full_adder(true, true, true, true, true, "8");
    }

    fn test_one_16_adder(input_a: i32, input_b: i32, carry_in: bool, output: i32, carry_out: bool) {
        let mut adder = Box::new(Adder::new());
        set_component_value_32(adder.as_mut(), input_a, input_b);
        adder.update(carry_in);

        let adder_output = get_output_value(adder.as_mut(), BUS_WIDTH);
        let adder_carry_out = adder.get_carry_out();
        assert_eq!(adder_output, output);
        assert_eq!(adder_carry_out, carry_out);

        println!("{}", adder);
        println!(
            "{}+{}+{}={} *** {}",
            input_a, input_b, carry_in as i32, adder_output, adder_carry_out as i32
        )
    }

    #[test]
    fn test_16_adder() {
        test_one_16_adder(0, 0, false, 0, false);
        test_one_16_adder(1, 0, false, 1, false);
        test_one_16_adder(0, 1, false, 1, false);
        test_one_16_adder(1, 1, false, 2, false);
        test_one_16_adder(64, 64, false, 128, false);
        test_one_16_adder(127, 128, false, 255, false);
        test_one_16_adder(32768, 32767, false, 65535, false);

        test_one_16_adder(0, 0, true, 1, false);
        test_one_16_adder(1, 1, true, 3, false);
        test_one_16_adder(0xFFFF, 0, true, 0, true);
        test_one_16_adder(32768, 32768, false, 0, true);
        test_one_16_adder(32769, 32768, false, 1, true);
        test_one_16_adder(65535, 2, false, 1, true);
    }
}
