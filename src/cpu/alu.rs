use crate::{
    components::{
        ANDer, Adder, Bus, Comparator, Component, Decoder3x8, Enabler, IsZero, LeftShifter, NOTer,
        ORer, RightShifter, XORer, BUS_WIDTH,
    },
    gates::{Wire, AND},
};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    ADD = 0,
    SHL = 1,
    SHR = 2,
    NOT = 3,
    AND = 4,
    OR = 5,
    XOR = 6,
    CMP = 7,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::ADD => "ADD",
                Operation::SHL => "SHL",
                Operation::SHR => "SHR",
                Operation::NOT => "NOT",
                Operation::AND => "AND",
                Operation::OR => "OR",
                Operation::XOR => "XOR",
                Operation::CMP => "CMP",
            }
        )
    }
}

impl From<i32> for Operation {
    fn from(value: i32) -> Self {
        match value {
            0 => Operation::ADD,
            1 => Operation::SHL,
            2 => Operation::SHR,
            3 => Operation::NOT,
            4 => Operation::AND,
            5 => Operation::OR,
            6 => Operation::XOR,
            7 => Operation::CMP,
            _ => Operation::CMP,
        }
    }
}

impl From<Operation> for i32 {
    fn from(value: Operation) -> Self {
        match value {
            Operation::ADD => 0,
            Operation::SHL => 1,
            Operation::SHR => 2,
            Operation::NOT => 3,
            Operation::AND => 4,
            Operation::OR => 5,
            Operation::XOR => 6,
            Operation::CMP => 7,
        }
    }
}

pub struct ALU {
    pub input_a_bus: Arc<Mutex<Bus>>,
    pub input_b_bus: Arc<Mutex<Bus>>,
    pub output_bus: Arc<Mutex<Bus>>,
    pub flags_output_bus: Arc<Mutex<Bus>>,
    pub op: [Wire; 3],

    pub carry_in: Wire,

    pub carry_out: Wire,
    pub a_is_larger: Wire,
    pub is_equal: Wire,

    pub op_decoder: Decoder3x8,

    pub comparator: Arc<Mutex<Comparator>>,
    pub xorer: Arc<Mutex<XORer>>,
    pub orer: Arc<Mutex<ORer>>,
    pub ander: Arc<Mutex<ANDer>>,
    pub notter: Arc<Mutex<NOTer>>,
    pub left_shifer: Arc<Mutex<LeftShifter>>,
    pub right_shifer: Arc<Mutex<RightShifter>>,
    pub adder: Arc<Mutex<Adder>>,

    pub is_zero: IsZero,
    pub enablers: Box<Vec<Enabler>>, //7
    pub and_gates: [AND; 3],
}

impl Display for ALU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flags_output_bus = self.flags_output_bus.lock().unwrap();
        write!(f, "ALU OP: {:?} A: {:>#06X} B: {:>#06X} OUT: {:>#06X} carryin: {} carryout: {} larger: {} eq: {} zero: {}",
            Operation::from(self.op_decoder.index()),
            self.input_a_bus.lock().unwrap().get_value(),
            self.input_b_bus.lock().unwrap().get_value(),
            self.output_bus.lock().unwrap().get_value(),
            self.carry_in.get() as i32,
            flags_output_bus.get_output_wire(0) as i32,
            flags_output_bus.get_output_wire(1) as i32,
            flags_output_bus.get_output_wire(2) as i32,
            flags_output_bus.get_output_wire(3) as i32,
        )
    }
}

impl ALU {
    pub fn new(
        input_a_bus: Arc<Mutex<Bus>>,
        input_b_bus: Arc<Mutex<Bus>>,
        output_bus: Arc<Mutex<Bus>>,
        flags_output_bus: Arc<Mutex<Bus>>,
    ) -> Self {
        Self {
            input_a_bus,
            input_b_bus,
            output_bus,
            flags_output_bus,
            op: (0..3)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            carry_in: Wire::new("carry_in".to_string(), false),
            carry_out: Wire::new("carry_out".to_string(), false),
            a_is_larger: Wire::new("a_is_larger".to_string(), false),
            is_equal: Wire::new("is_equal".to_string(), false),
            op_decoder: Decoder3x8::new(),
            comparator: Arc::new(Mutex::new(Comparator::new())),
            xorer: Arc::new(Mutex::new(XORer::new())),
            orer: Arc::new(Mutex::new(ORer::new())),
            ander: Arc::new(Mutex::new(ANDer::new())),
            notter: Arc::new(Mutex::new(NOTer::new())),
            left_shifer: Arc::new(Mutex::new(LeftShifter::new())),
            right_shifer: Arc::new(Mutex::new(RightShifter::new())),
            adder: Arc::new(Mutex::new(Adder::new())),
            is_zero: IsZero::new(),
            enablers: Box::new(
                (0..7)
                    .map(|_| Enabler::new())
                    .collect::<Vec<Enabler>>()
                    .try_into()
                    .unwrap(),
            ),
            and_gates: (0..3)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn set_wire_on_component<T>(&self, c: Arc<Mutex<T>>)
    where
        T: Component,
    {
        for i in (0..BUS_WIDTH).rev() {
            c.lock().unwrap().set_input_wire(
                i as i32,
                self.input_a_bus.lock().unwrap().get_output_wire(i),
            );
        }

        for i in (BUS_WIDTH..BUS_WIDTH * 2).rev() {
            c.lock().unwrap().set_input_wire(
                i as i32,
                self.input_b_bus.lock().unwrap().get_output_wire(i - 16),
            );
        }
    }

    pub fn wire_to_enabler<T>(&mut self, c: Arc<Mutex<T>>, enabler_index: i32)
    where
        T: Component,
    {
        for i in 0..BUS_WIDTH {
            self.enablers[enabler_index as usize]
                .set_input_wire(i, c.lock().unwrap().get_output_wire(i))
        }
    }
}

// Update
impl ALU {
    pub fn update(&mut self) {
        self.update_op_decoder();
        let enabler: Operation = self.op_decoder.index().into();

        self.update_comparator();

        match enabler {
            Operation::ADD => self.update_adder(),
            Operation::SHL => self.update_left_shifter(),
            Operation::SHR => self.update_right_shifter(),
            Operation::NOT => self.update_notter(),
            Operation::AND => self.update_ander(),
            Operation::OR => self.update_orer(),
            Operation::XOR => self.update_xorer(),
            Operation::CMP => self.update_comparator(),
        }

        if enabler != Operation::CMP {
            self.enablers[enabler as usize].update(true);

            match enabler {
                Operation::ADD => {
                    self.and_gates[0].update(
                        self.adder.lock().unwrap().get_carry_out(),
                        self.op_decoder.get_output_wire(Operation::ADD.into()),
                    );
                    self.carry_out.update(self.and_gates[0].get());
                }
                Operation::SHR => {
                    self.and_gates[1].update(
                        self.right_shifer.lock().unwrap().get(),
                        self.op_decoder.get_output_wire(Operation::SHR.into()),
                    );
                    self.carry_out.update(self.and_gates[1].get());
                }
                Operation::SHL => {
                    self.and_gates[2].update(
                        self.left_shifer.lock().unwrap().get(),
                        self.op_decoder.get_output_wire(Operation::SHL.into()),
                    );
                    self.carry_out.update(self.and_gates[2].get());
                }
                _ => {}
            }

            for i in 0..BUS_WIDTH {
                self.is_zero
                    .set_input_wire(i, self.enablers[enabler as usize].get_output_wire(i));
                self.output_bus
                    .lock()
                    .unwrap()
                    .set_input_wire(i, self.enablers[enabler as usize].get_output_wire(i));
            }
        } else {
            for i in 0..BUS_WIDTH {
                self.is_zero.set_input_wire(i, true);
                self.output_bus.lock().unwrap().set_input_wire(i, false);
            }
        }
        self.is_zero.update();

        self.flags_output_bus
            .lock()
            .unwrap()
            .set_input_wire(0, self.carry_out.get());
        self.flags_output_bus
            .lock()
            .unwrap()
            .set_input_wire(1, self.a_is_larger.get());
        self.flags_output_bus
            .lock()
            .unwrap()
            .set_input_wire(2, self.is_equal.get());
        self.flags_output_bus
            .lock()
            .unwrap()
            .set_input_wire(3, self.is_zero.get_output_wire(0))
    }

    fn update_op_decoder(&mut self) {
        self.op_decoder
            .update(self.op[2].get(), self.op[1].get(), self.op[0].get())
    }

    fn update_comparator(&mut self) {
        self.set_wire_on_component(self.comparator.clone());
        self.comparator.lock().unwrap().update();
        self.a_is_larger
            .update(self.comparator.lock().unwrap().larger());
        self.is_equal
            .update(self.comparator.lock().unwrap().equal());
    }

    fn update_xorer(&mut self) {
        self.set_wire_on_component(self.xorer.clone());
        self.xorer.lock().unwrap().update();
        self.wire_to_enabler(self.xorer.clone(), 6);
    }

    fn update_orer(&mut self) {
        self.set_wire_on_component(self.orer.clone());
        self.orer.lock().unwrap().update();
        self.wire_to_enabler(self.orer.clone(), 5);
    }

    fn update_ander(&mut self) {
        self.set_wire_on_component(self.ander.clone());
        self.ander.lock().unwrap().update();
        self.wire_to_enabler(self.ander.clone(), 4);
    }

    fn update_notter(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.notter
                .lock()
                .unwrap()
                .set_input_wire(i, self.input_a_bus.lock().unwrap().get_output_wire(i))
        }
        self.notter.lock().unwrap().update();
        self.wire_to_enabler(self.notter.clone(), 3);
    }

    fn update_right_shifter(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.right_shifer
                .lock()
                .unwrap()
                .set_input_wire(i, self.input_a_bus.lock().unwrap().get_output_wire(i));
        }
        self.right_shifer
            .lock()
            .unwrap()
            .update(self.carry_in.get());
        self.wire_to_enabler(self.right_shifer.clone(), 2);
    }

    fn update_left_shifter(&mut self) {
        for i in (0..BUS_WIDTH).rev() {
            self.left_shifer
                .lock()
                .unwrap()
                .set_input_wire(i, self.input_a_bus.lock().unwrap().get_output_wire(i));
        }
        self.left_shifer.lock().unwrap().update(self.carry_in.get());
        self.wire_to_enabler(self.left_shifer.clone(), 1);
    }

    fn update_adder(&mut self) {
        self.set_wire_on_component(self.adder.clone());
        self.adder.lock().unwrap().update(self.carry_in.get());
        self.wire_to_enabler(self.adder.clone(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_add() {
        let o = Operation::ADD;
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x0001, 0x0001, false, 0x0002, true, false, false, false);
        op_test(o, 0x00FF, 0x0000, false, 0x00FF, false, true, false, false);
        op_test(o, 0x0000, 0x00FF, false, 0x00FF, false, false, false, false);
        op_test(o, 0xFF00, 0x00FF, false, 0xFFFF, false, true, false, false);
        op_test(o, 0xFFFF, 0x0001, false, 0x0000, false, true, true, true);

        op_test(o, 0x0000, 0x0000, true, 0x0001, true, false, false, false);
        op_test(o, 0x0001, 0x0001, true, 0x0003, true, false, false, false);
        op_test(o, 0xFFFF, 0x0000, true, 0x0000, false, true, true, true);
    }

    #[test]
    fn test_alu_shl() {
        let o = Operation::SHL;
        let mut i: u16 = 1;
        while i < 0x7FFF {
            op_test(o, i, i, false, i * 2, true, false, false, false);
            op_test(o, i, 0x00, false, i * 2, false, true, false, false);
            i *= 2;
        }

        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x0059, 0x0059, false, 0x00B2, true, false, false, false);
        op_test(o, 0x0004, 0x0001, false, 0x0008, false, true, false, false);

        op_test(o, 0x0073, 0x0000, false, 0x00E6, false, true, false, false);

        op_test(o, 0xAA00, 0x0001, false, 0x5400, false, true, true, false);

        op_test(o, 0x004A, 0x0001, true, 0x0095, false, true, false, false);

        op_test(o, 0x0000, 0x0005, false, 0x0000, false, false, false, true);
    }

    #[test]
    fn test_alu_shr() {
        let o = Operation::SHR;
        let mut i: u16 = 0x8000;
        while i > 1 {
            op_test(o, i, i, false, i / 2, true, false, false, false);
            op_test(o, i, 0x00, false, i / 2, false, true, false, false);
            i /= 2;
        }
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x0056, 0x0056, false, 0x002B, true, false, false, false);
        op_test(o, 0x0004, 0x0001, false, 0x0002, false, true, false, false);
        op_test(o, 0x0072, 0x0000, false, 0x0039, false, true, false, false);

        op_test(o, 0x00A1, 0x0000, false, 0x0050, false, true, true, false);

        op_test(o, 0x4A00, 0x0001, true, 0xA500, false, true, false, false);

        op_test(o, 0x0000, 0x0005, false, 0x0000, false, false, false, true);
    }

    #[test]
    fn test_alu_not() {
        let o = Operation::NOT;
        op_test(o, 0x0000, 0x0000, false, 0xFFFF, true, false, false, false);
        op_test(o, 0x00FF, 0x0000, false, 0xFF00, false, true, false, false);
        op_test(o, 0xFFFF, 0x0000, false, 0x0000, false, true, false, true);
        op_test(o, 0xFFFF, 0x00FF, false, 0x0000, false, true, false, true);
        op_test(o, 0xFFFF, 0x00FF, true, 0x0000, false, true, false, true);
        op_test(o, 0xA9A9, 0x5757, true, 0x5656, false, true, false, false);
    }

    #[test]
    fn test_alu_and() {
        let o = Operation::AND;
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x00FF, 0x0000, false, 0x0000, false, true, false, true);
        op_test(o, 0xFFFF, 0x0000, false, 0x0000, false, true, false, true);
        op_test(o, 0xFFFF, 0x00FF, false, 0x00FF, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, true, 0x00FF, false, true, false, false);
        op_test(o, 0xA9A9, 0x5757, true, 0x0101, false, true, false, false);
    }

    #[test]
    fn test_alu_or() {
        let o = Operation::OR;
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x00FF, 0x0000, false, 0x00FF, false, true, false, false);
        op_test(o, 0xFFFF, 0x0000, false, 0xFFFF, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, false, 0xFFFF, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, true, 0xFFFF, false, true, false, false);
        op_test(o, 0xA9A9, 0x5757, true, 0xFFFF, false, true, false, false);
    }

    #[test]
    fn test_alu_xor() {
        let o = Operation::XOR;
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, true);
        op_test(o, 0x00FF, 0x0000, false, 0x00FF, false, true, false, false);
        op_test(o, 0xFFFF, 0x0000, false, 0xFFFF, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, false, 0xFF00, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, true, 0xFF00, false, true, false, false);
        op_test(o, 0xA9A9, 0x5757, true, 0xFEFE, false, true, false, false);
    }

    #[test]
    fn test_alu_cmp() {
        let o = Operation::CMP;
        op_test(o, 0x0000, 0x0000, false, 0x0000, true, false, false, false);
        op_test(o, 0x00FF, 0x0000, false, 0x0000, false, true, false, false);
        op_test(o, 0xFFFF, 0x0000, false, 0x0000, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, false, 0x0000, false, true, false, false);
        op_test(o, 0xFFFF, 0x00FF, true, 0x0000, false, true, false, false);
        op_test(o, 0xA9A9, 0x5757, true, 0x0000, false, true, false, false);
    }

    fn op_test(
        op: Operation,
        input_a: u16,
        input_b: u16,
        carry_in: bool,

        expected_output: u16,
        expected_equal: bool,
        expected_is_larger: bool,
        expected_carry_out: bool,
        expected_zero: bool,
    ) {
        let input_a_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let input_b_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let output_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let flags_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut alu = Box::new(ALU::new(
            input_a_bus.clone(),
            input_b_bus.clone(),
            output_bus.clone(),
            flags_bus.clone(),
        ));

        input_a_bus.lock().unwrap().set_value(input_a);
        input_b_bus.lock().unwrap().set_value(input_b);

        set_operation(alu.as_mut(), op as u16);

        alu.carry_in.update(carry_in);
        alu.update();

        let output = output_bus.lock().unwrap().get_value();
        let carry_out = alu.flags_output_bus.lock().unwrap().get_output_wire(0);
        let is_larger = alu.flags_output_bus.lock().unwrap().get_output_wire(1);
        let equal = alu.flags_output_bus.lock().unwrap().get_output_wire(2);
        let zero = alu.flags_output_bus.lock().unwrap().get_output_wire(3);

        assert_eq!(
            expected_output, output,
            "output: {:#X} expected: {:#X}",
            output, expected_output
        );
        assert_eq!(
            expected_carry_out, carry_out,
            "carry_out: {} expected: {}",
            carry_out, expected_carry_out
        );
        assert_eq!(
            expected_is_larger, is_larger,
            "is_larger: {} expected: {}",
            is_larger, expected_is_larger
        );
        assert_eq!(
            expected_equal, equal,
            "equal: {} expected: {}",
            equal, expected_equal
        );
        assert_eq!(
            expected_zero, zero,
            "zero: {} expected: {}",
            zero, expected_zero
        );
    }

    fn set_operation(alu: &mut ALU, vlaue: u16) {
        let vlaue = vlaue & 0x07;
        for i in (0..=2).rev() {
            match vlaue & (1 << i) {
                0 => alu.op[i].update(false),
                _ => alu.op[i].update(true),
            }
        }
    }
}
