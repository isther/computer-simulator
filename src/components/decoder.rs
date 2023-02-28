use super::{ANDGate3, ANDGate4};
use crate::gates::{Wire, AND, NOT};

#[derive(Debug)]
pub struct Decoder2x4 {
    input_a: Wire,
    input_b: Wire,

    not_gates: [NOT; 2],
    and_gates: [AND; 4],
    outputs: [Wire; 4],
}

impl Decoder2x4 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("Z".to_string(), false),
            input_b: Wire::new("Z".to_string(), false),
            not_gates: (0..2)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            and_gates: (0..4)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            outputs: (0..4)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn get_output_wire(&self, index: i32) -> bool {
        self.outputs[index as usize].get()
    }

    pub fn update(&mut self, input_a: bool, input_b: bool) {
        self.input_a.update(input_a);
        self.input_b.update(input_b);

        self.not_gates[0].update(self.input_a.get());
        self.not_gates[1].update(self.input_b.get());

        self.and_gates[0].update(self.not_gates[0].get(), self.not_gates[1].get());
        self.and_gates[1].update(self.not_gates[0].get(), self.input_b.get());
        self.and_gates[2].update(self.input_a.get(), self.not_gates[1].get());
        self.and_gates[3].update(self.input_a.get(), self.input_b.get());

        for i in 0..self.outputs.len() {
            self.outputs[i].update(self.and_gates[i].get());
        }
    }
}

#[derive(Debug, Clone)]
pub struct Decoder3x8 {
    pub input_a: Wire,
    pub input_b: Wire,
    pub input_c: Wire,

    pub not_gates: [NOT; 3],
    pub and_gates: [ANDGate3; 8],
    pub outputs: [Wire; 8],
}

impl Decoder3x8 {
    pub fn new() -> Self {
        Self {
            input_a: Wire::new("Z".to_string(), false),
            input_b: Wire::new("Z".to_string(), false),
            input_c: Wire::new("Z".to_string(), false),
            not_gates: (0..3)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            and_gates: (0..8)
                .map(|_| ANDGate3::new())
                .collect::<Vec<ANDGate3>>()
                .try_into()
                .unwrap(),
            outputs: (0..8)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn get_output_wire(&self, index: i32) -> bool {
        self.outputs[index as usize].get()
    }

    // Returns the index which is enabled
    pub fn index(&self) -> i32 {
        for i in 0..self.outputs.len() {
            if self.outputs[i].get() {
                return i as i32;
            }
        }
        0
    }

    pub fn update(&mut self, input_a: bool, input_b: bool, input_c: bool) {
        self.input_a.update(input_a);
        self.input_b.update(input_b);
        self.input_c.update(input_c);

        self.not_gates[0].update(self.input_a.get());
        self.not_gates[1].update(self.input_b.get());
        self.not_gates[2].update(self.input_c.get());

        self.and_gates[0].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            self.not_gates[2].get(),
        );
        self.and_gates[1].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            self.input_c.get(),
        );
        self.and_gates[2].update(
            self.not_gates[0].get(),
            self.input_b.get(),
            self.not_gates[2].get(),
        );
        self.and_gates[3].update(
            self.not_gates[0].get(),
            self.input_b.get(),
            self.input_c.get(),
        );

        self.and_gates[4].update(
            self.input_a.get(),
            self.not_gates[1].get(),
            self.not_gates[2].get(),
        );
        self.and_gates[5].update(
            self.input_a.get(),
            self.not_gates[1].get(),
            self.input_c.get(),
        );
        self.and_gates[6].update(
            self.input_a.get(),
            self.input_b.get(),
            self.not_gates[2].get(),
        );
        self.and_gates[7].update(self.input_a.get(), self.input_b.get(), self.input_c.get());

        for i in 0..self.outputs.len() {
            self.outputs[i].update(self.and_gates[i].get())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Decoder4x16 {
    pub not_gates: [NOT; 4],
    pub and_gates: [ANDGate4; 16],
    pub outputs: [Wire; 16],
    pub index: i32,
}

impl Decoder4x16 {
    fn new() -> Self {
        Self {
            not_gates: (0..4)
                .map(|_| NOT::new())
                .collect::<Vec<NOT>>()
                .try_into()
                .unwrap(),
            and_gates: (0..16)
                .map(|_| ANDGate4::new())
                .collect::<Vec<ANDGate4>>()
                .try_into()
                .unwrap(),
            outputs: (0..16)
                .map(|_| Wire::new("Z".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
            index: 0,
        }
    }

    fn index(&self) -> i32 {
        self.index
    }

    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
    }

    fn update(&mut self, input_a: bool, input_b: bool, input_c: bool, input_d: bool) {
        self.not_gates[0].update(input_a);
        self.not_gates[1].update(input_b);
        self.not_gates[2].update(input_c);
        self.not_gates[3].update(input_d);

        self.and_gates[0].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            self.not_gates[2].get(),
            self.not_gates[3].get(),
        );

        self.and_gates[1].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            self.not_gates[2].get(),
            input_d,
        );

        self.and_gates[2].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            input_c,
            self.not_gates[3].get(),
        );

        self.and_gates[3].update(
            self.not_gates[0].get(),
            self.not_gates[1].get(),
            input_c,
            input_d,
        );

        self.and_gates[4].update(
            self.not_gates[0].get(),
            input_b,
            self.not_gates[2].get(),
            self.not_gates[3].get(),
        );

        self.and_gates[5].update(
            self.not_gates[0].get(),
            input_b,
            self.not_gates[2].get(),
            input_d,
        );

        self.and_gates[6].update(
            self.not_gates[0].get(),
            input_b,
            input_c,
            self.not_gates[3].get(),
        );

        self.and_gates[7].update(self.not_gates[0].get(), input_b, input_c, input_d);

        self.and_gates[8].update(
            input_a,
            self.not_gates[1].get(),
            self.not_gates[2].get(),
            self.not_gates[3].get(),
        );

        self.and_gates[9].update(
            input_a,
            self.not_gates[1].get(),
            self.not_gates[2].get(),
            input_d,
        );

        self.and_gates[10].update(
            input_a,
            self.not_gates[1].get(),
            input_c,
            self.not_gates[3].get(),
        );

        self.and_gates[11].update(input_a, self.not_gates[1].get(), input_c, input_d);

        self.and_gates[12].update(
            input_a,
            input_b,
            self.not_gates[2].get(),
            self.not_gates[3].get(),
        );

        self.and_gates[13].update(input_a, input_b, self.not_gates[2].get(), input_d);

        self.and_gates[14].update(input_a, input_b, input_c, self.not_gates[3].get());

        self.and_gates[15].update(input_a, input_b, input_c, input_d);

        self.index = 0;
        for i in 0..self.outputs.len() {
            self.outputs[i].update(self.and_gates[i].get());
            if self.outputs[i].get() {
                self.index += i as i32;
            }
        }
    }
}

pub struct Decoder8x256 {
    decoder_selector: Decoder4x16,
    decoders_4x16: [Decoder4x16; 16],
    index: i32,
}

impl Decoder8x256 {
    pub fn new() -> Self {
        Self {
            decoder_selector: Decoder4x16::new(),
            decoders_4x16: (0..16)
                .map(|_| Decoder4x16::new())
                .collect::<Vec<Decoder4x16>>()
                .try_into()
                .unwrap(),
            index: 0,
        }
    }

    pub fn index(&self) -> i32 {
        self.index
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
        self.index = 0;

        self.decoder_selector.update(e, f, g, h);
        for i in 0..16 {
            self.update_decoder(a, b, c, d, i, 16 * i);
        }
    }

    fn update_decoder(
        &mut self,
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        decoer_index: i32,
        output_wire_start: i32,
    ) {
        if self.decoder_selector.get_output_wire(decoer_index) {
            self.decoders_4x16[decoer_index as usize].update(a, b, c, d);
            for i in 0..16 {
                if self.decoders_4x16[decoer_index as usize].outputs[i].get() {
                    self.index = output_wire_start + i as i32;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_3x8() {
        let check_decoder_3x8 = |input_a: bool, input_b: bool, input_c: bool, ans: i32| {
            let mut expected: [bool; 16] = [false; 16];
            expected[ans as usize] = true;

            let mut decoder_3x8 = Decoder3x8::new();
            decoder_3x8.update(input_a, input_b, input_c);
            for i in 0..decoder_3x8.outputs.len() {
                assert_eq!(decoder_3x8.outputs[i].get(), expected[i])
            }
        };

        check_decoder_3x8(false, false, false, 0);
        check_decoder_3x8(false, false, true, 1);
        check_decoder_3x8(false, true, false, 2);
        check_decoder_3x8(false, true, true, 3);

        check_decoder_3x8(true, false, false, 4);
        check_decoder_3x8(true, false, true, 5);
        check_decoder_3x8(true, true, false, 6);
        check_decoder_3x8(true, true, true, 7);
    }

    #[test]
    fn test_decoder_4x16() {
        let check_decoder_4x16 =
            |input_a: bool, input_b: bool, input_c: bool, input_d: bool, ans: i32| {
                let mut expected: [bool; 16] = [false; 16];
                expected[ans as usize] = true;

                let mut decoder_4x16 = Decoder4x16::new();
                decoder_4x16.update(input_a, input_b, input_c, input_d);
                for i in 0..decoder_4x16.outputs.len() {
                    assert_eq!(decoder_4x16.outputs[i].get(), expected[i])
                }
            };

        check_decoder_4x16(false, false, false, false, 0);
        check_decoder_4x16(false, false, false, true, 1);
        check_decoder_4x16(false, false, true, false, 2);
        check_decoder_4x16(false, false, true, true, 3);
        check_decoder_4x16(false, true, false, false, 4);
        check_decoder_4x16(false, true, false, true, 5);
        check_decoder_4x16(false, true, true, false, 6);
        check_decoder_4x16(false, true, true, true, 7);
        check_decoder_4x16(true, false, false, false, 8);
        check_decoder_4x16(true, false, false, true, 9);
        check_decoder_4x16(true, false, true, false, 10);
        check_decoder_4x16(true, false, true, true, 11);
        check_decoder_4x16(true, true, false, false, 12);
        check_decoder_4x16(true, true, false, true, 13);
        check_decoder_4x16(true, true, true, false, 14);
        check_decoder_4x16(true, true, true, true, 15);
    }

    #[test]
    fn test_decoder_8x256() {
        let check_decoder_8x256 = |input_a: bool,
                                   input_b: bool,
                                   input_c: bool,
                                   input_d: bool,
                                   input_e: bool,
                                   input_f: bool,
                                   input_g: bool,
                                   input_h: bool,
                                   ans: i32| {
            let mut decoder_8x256 = Decoder8x256::new();
            decoder_8x256.update(
                input_a, input_b, input_c, input_d, input_e, input_f, input_g, input_h,
            );
            assert_eq!(decoder_8x256.index(), ans, "err at {}", ans)
        };

        check_decoder_8x256(false, false, false, false, false, false, false, false, 0);
        check_decoder_8x256(false, false, false, true, false, false, false, false, 1);
        check_decoder_8x256(false, false, true, false, false, false, false, false, 2);
        check_decoder_8x256(false, false, true, true, false, false, false, false, 3);
        check_decoder_8x256(false, true, false, false, false, false, false, false, 4);
        check_decoder_8x256(false, true, false, true, false, false, false, false, 5);
        check_decoder_8x256(false, true, true, false, false, false, false, false, 6);
        check_decoder_8x256(false, true, true, true, false, false, false, false, 7);
        check_decoder_8x256(true, false, false, false, false, false, false, false, 8);
        check_decoder_8x256(true, false, false, true, false, false, false, false, 9);
        check_decoder_8x256(true, false, true, false, false, false, false, false, 10);
        check_decoder_8x256(true, false, true, true, false, false, false, false, 11);
        check_decoder_8x256(true, true, false, false, false, false, false, false, 12);
        check_decoder_8x256(true, true, false, true, false, false, false, false, 13);
        check_decoder_8x256(true, true, true, false, false, false, false, false, 14);
        check_decoder_8x256(true, true, true, true, false, false, false, false, 15);
        check_decoder_8x256(false, false, false, false, false, false, false, true, 16);
        check_decoder_8x256(false, false, false, true, false, false, false, true, 17);
        check_decoder_8x256(false, false, true, false, false, false, false, true, 18);
        check_decoder_8x256(false, false, true, true, false, false, false, true, 19);
        check_decoder_8x256(false, true, false, false, false, false, false, true, 20);
        check_decoder_8x256(false, true, false, true, false, false, false, true, 21);
        check_decoder_8x256(false, true, true, false, false, false, false, true, 22);
        check_decoder_8x256(false, true, true, true, false, false, false, true, 23);
        check_decoder_8x256(true, false, false, false, false, false, false, true, 24);
        check_decoder_8x256(true, false, false, true, false, false, false, true, 25);
        check_decoder_8x256(true, false, true, false, false, false, false, true, 26);
        check_decoder_8x256(true, false, true, true, false, false, false, true, 27);
        check_decoder_8x256(true, true, false, false, false, false, false, true, 28);
        check_decoder_8x256(true, true, false, true, false, false, false, true, 29);
        check_decoder_8x256(true, true, true, false, false, false, false, true, 30);
        check_decoder_8x256(true, true, true, true, false, false, false, true, 31);
        check_decoder_8x256(false, false, false, false, false, false, true, false, 32);
        check_decoder_8x256(false, false, false, true, false, false, true, false, 33);
        check_decoder_8x256(false, false, true, false, false, false, true, false, 34);
        check_decoder_8x256(false, false, true, true, false, false, true, false, 35);
        check_decoder_8x256(false, true, false, false, false, false, true, false, 36);
        check_decoder_8x256(false, true, false, true, false, false, true, false, 37);
        check_decoder_8x256(false, true, true, false, false, false, true, false, 38);
        check_decoder_8x256(false, true, true, true, false, false, true, false, 39);
        check_decoder_8x256(true, false, false, false, false, false, true, false, 40);
        check_decoder_8x256(true, false, false, true, false, false, true, false, 41);
        check_decoder_8x256(true, false, true, false, false, false, true, false, 42);
        check_decoder_8x256(true, false, true, true, false, false, true, false, 43);
        check_decoder_8x256(true, true, false, false, false, false, true, false, 44);
        check_decoder_8x256(true, true, false, true, false, false, true, false, 45);
        check_decoder_8x256(true, true, true, false, false, false, true, false, 46);
        check_decoder_8x256(true, true, true, true, false, false, true, false, 47);
        check_decoder_8x256(false, false, false, false, false, false, true, true, 48);
        check_decoder_8x256(false, false, false, true, false, false, true, true, 49);
        check_decoder_8x256(false, false, true, false, false, false, true, true, 50);
        check_decoder_8x256(false, false, true, true, false, false, true, true, 51);
        check_decoder_8x256(false, true, false, false, false, false, true, true, 52);
        check_decoder_8x256(false, true, false, true, false, false, true, true, 53);
        check_decoder_8x256(false, true, true, false, false, false, true, true, 54);
        check_decoder_8x256(false, true, true, true, false, false, true, true, 55);
        check_decoder_8x256(true, false, false, false, false, false, true, true, 56);
        check_decoder_8x256(true, false, false, true, false, false, true, true, 57);
        check_decoder_8x256(true, false, true, false, false, false, true, true, 58);
        check_decoder_8x256(true, false, true, true, false, false, true, true, 59);
        check_decoder_8x256(true, true, false, false, false, false, true, true, 60);
        check_decoder_8x256(true, true, false, true, false, false, true, true, 61);
        check_decoder_8x256(true, true, true, false, false, false, true, true, 62);
        check_decoder_8x256(true, true, true, true, false, false, true, true, 63);
        check_decoder_8x256(false, false, false, false, false, true, false, false, 64);
        check_decoder_8x256(false, false, false, true, false, true, false, false, 65);
        check_decoder_8x256(false, false, true, false, false, true, false, false, 66);
        check_decoder_8x256(false, false, true, true, false, true, false, false, 67);
        check_decoder_8x256(false, true, false, false, false, true, false, false, 68);
        check_decoder_8x256(false, true, false, true, false, true, false, false, 69);
        check_decoder_8x256(false, true, true, false, false, true, false, false, 70);
        check_decoder_8x256(false, true, true, true, false, true, false, false, 71);
        check_decoder_8x256(true, false, false, false, false, true, false, false, 72);
        check_decoder_8x256(true, false, false, true, false, true, false, false, 73);
        check_decoder_8x256(true, false, true, false, false, true, false, false, 74);
        check_decoder_8x256(true, false, true, true, false, true, false, false, 75);
        check_decoder_8x256(true, true, false, false, false, true, false, false, 76);
        check_decoder_8x256(true, true, false, true, false, true, false, false, 77);
        check_decoder_8x256(true, true, true, false, false, true, false, false, 78);
        check_decoder_8x256(true, true, true, true, false, true, false, false, 79);
        check_decoder_8x256(false, false, false, false, false, true, false, true, 80);
        check_decoder_8x256(false, false, false, true, false, true, false, true, 81);
        check_decoder_8x256(false, false, true, false, false, true, false, true, 82);
        check_decoder_8x256(false, false, true, true, false, true, false, true, 83);
        check_decoder_8x256(false, true, false, false, false, true, false, true, 84);
        check_decoder_8x256(false, true, false, true, false, true, false, true, 85);
        check_decoder_8x256(false, true, true, false, false, true, false, true, 86);
        check_decoder_8x256(false, true, true, true, false, true, false, true, 87);
        check_decoder_8x256(true, false, false, false, false, true, false, true, 88);
        check_decoder_8x256(true, false, false, true, false, true, false, true, 89);
        check_decoder_8x256(true, false, true, false, false, true, false, true, 90);
        check_decoder_8x256(true, false, true, true, false, true, false, true, 91);
        check_decoder_8x256(true, true, false, false, false, true, false, true, 92);
        check_decoder_8x256(true, true, false, true, false, true, false, true, 93);
        check_decoder_8x256(true, true, true, false, false, true, false, true, 94);
        check_decoder_8x256(true, true, true, true, false, true, false, true, 95);
        check_decoder_8x256(false, false, false, false, false, true, true, false, 96);
        check_decoder_8x256(false, false, false, true, false, true, true, false, 97);
        check_decoder_8x256(false, false, true, false, false, true, true, false, 98);
        check_decoder_8x256(false, false, true, true, false, true, true, false, 99);
        check_decoder_8x256(false, true, false, false, false, true, true, false, 100);
        check_decoder_8x256(false, true, false, true, false, true, true, false, 101);
        check_decoder_8x256(false, true, true, false, false, true, true, false, 102);
        check_decoder_8x256(false, true, true, true, false, true, true, false, 103);
        check_decoder_8x256(true, false, false, false, false, true, true, false, 104);
        check_decoder_8x256(true, false, false, true, false, true, true, false, 105);
        check_decoder_8x256(true, false, true, false, false, true, true, false, 106);
        check_decoder_8x256(true, false, true, true, false, true, true, false, 107);
        check_decoder_8x256(true, true, false, false, false, true, true, false, 108);
        check_decoder_8x256(true, true, false, true, false, true, true, false, 109);
        check_decoder_8x256(true, true, true, false, false, true, true, false, 110);
        check_decoder_8x256(true, true, true, true, false, true, true, false, 111);
        check_decoder_8x256(false, false, false, false, false, true, true, true, 112);
        check_decoder_8x256(false, false, false, true, false, true, true, true, 113);
        check_decoder_8x256(false, false, true, false, false, true, true, true, 114);
        check_decoder_8x256(false, false, true, true, false, true, true, true, 115);
        check_decoder_8x256(false, true, false, false, false, true, true, true, 116);
        check_decoder_8x256(false, true, false, true, false, true, true, true, 117);
        check_decoder_8x256(false, true, true, false, false, true, true, true, 118);
        check_decoder_8x256(false, true, true, true, false, true, true, true, 119);
        check_decoder_8x256(true, false, false, false, false, true, true, true, 120);
        check_decoder_8x256(true, false, false, true, false, true, true, true, 121);
        check_decoder_8x256(true, false, true, false, false, true, true, true, 122);
        check_decoder_8x256(true, false, true, true, false, true, true, true, 123);
        check_decoder_8x256(true, true, false, false, false, true, true, true, 124);
        check_decoder_8x256(true, true, false, true, false, true, true, true, 125);
        check_decoder_8x256(true, true, true, false, false, true, true, true, 126);
        check_decoder_8x256(true, true, true, true, false, true, true, true, 127);
        check_decoder_8x256(false, false, false, false, true, false, false, false, 128);
        check_decoder_8x256(false, false, false, true, true, false, false, false, 129);
        check_decoder_8x256(false, false, true, false, true, false, false, false, 130);
        check_decoder_8x256(false, false, true, true, true, false, false, false, 131);
        check_decoder_8x256(false, true, false, false, true, false, false, false, 132);
        check_decoder_8x256(false, true, false, true, true, false, false, false, 133);
        check_decoder_8x256(false, true, true, false, true, false, false, false, 134);
        check_decoder_8x256(false, true, true, true, true, false, false, false, 135);
        check_decoder_8x256(true, false, false, false, true, false, false, false, 136);
        check_decoder_8x256(true, false, false, true, true, false, false, false, 137);
        check_decoder_8x256(true, false, true, false, true, false, false, false, 138);
        check_decoder_8x256(true, false, true, true, true, false, false, false, 139);
        check_decoder_8x256(true, true, false, false, true, false, false, false, 140);
        check_decoder_8x256(true, true, false, true, true, false, false, false, 141);
        check_decoder_8x256(true, true, true, false, true, false, false, false, 142);
        check_decoder_8x256(true, true, true, true, true, false, false, false, 143);
        check_decoder_8x256(false, false, false, false, true, false, false, true, 144);
        check_decoder_8x256(false, false, false, true, true, false, false, true, 145);
        check_decoder_8x256(false, false, true, false, true, false, false, true, 146);
        check_decoder_8x256(false, false, true, true, true, false, false, true, 147);
        check_decoder_8x256(false, true, false, false, true, false, false, true, 148);
        check_decoder_8x256(false, true, false, true, true, false, false, true, 149);
        check_decoder_8x256(false, true, true, false, true, false, false, true, 150);
        check_decoder_8x256(false, true, true, true, true, false, false, true, 151);
        check_decoder_8x256(true, false, false, false, true, false, false, true, 152);
        check_decoder_8x256(true, false, false, true, true, false, false, true, 153);
        check_decoder_8x256(true, false, true, false, true, false, false, true, 154);
        check_decoder_8x256(true, false, true, true, true, false, false, true, 155);
        check_decoder_8x256(true, true, false, false, true, false, false, true, 156);
        check_decoder_8x256(true, true, false, true, true, false, false, true, 157);
        check_decoder_8x256(true, true, true, false, true, false, false, true, 158);
        check_decoder_8x256(true, true, true, true, true, false, false, true, 159);
        check_decoder_8x256(false, false, false, false, true, false, true, false, 160);
        check_decoder_8x256(false, false, false, true, true, false, true, false, 161);
        check_decoder_8x256(false, false, true, false, true, false, true, false, 162);
        check_decoder_8x256(false, false, true, true, true, false, true, false, 163);
        check_decoder_8x256(false, true, false, false, true, false, true, false, 164);
        check_decoder_8x256(false, true, false, true, true, false, true, false, 165);
        check_decoder_8x256(false, true, true, false, true, false, true, false, 166);
        check_decoder_8x256(false, true, true, true, true, false, true, false, 167);
        check_decoder_8x256(true, false, false, false, true, false, true, false, 168);
        check_decoder_8x256(true, false, false, true, true, false, true, false, 169);
        check_decoder_8x256(true, false, true, false, true, false, true, false, 170);
        check_decoder_8x256(true, false, true, true, true, false, true, false, 171);
        check_decoder_8x256(true, true, false, false, true, false, true, false, 172);
        check_decoder_8x256(true, true, false, true, true, false, true, false, 173);
        check_decoder_8x256(true, true, true, false, true, false, true, false, 174);
        check_decoder_8x256(true, true, true, true, true, false, true, false, 175);
        check_decoder_8x256(false, false, false, false, true, false, true, true, 176);
        check_decoder_8x256(false, false, false, true, true, false, true, true, 177);
        check_decoder_8x256(false, false, true, false, true, false, true, true, 178);
        check_decoder_8x256(false, false, true, true, true, false, true, true, 179);
        check_decoder_8x256(false, true, false, false, true, false, true, true, 180);
        check_decoder_8x256(false, true, false, true, true, false, true, true, 181);
        check_decoder_8x256(false, true, true, false, true, false, true, true, 182);
        check_decoder_8x256(false, true, true, true, true, false, true, true, 183);
        check_decoder_8x256(true, false, false, false, true, false, true, true, 184);
        check_decoder_8x256(true, false, false, true, true, false, true, true, 185);
        check_decoder_8x256(true, false, true, false, true, false, true, true, 186);
        check_decoder_8x256(true, false, true, true, true, false, true, true, 187);
        check_decoder_8x256(true, true, false, false, true, false, true, true, 188);
        check_decoder_8x256(true, true, false, true, true, false, true, true, 189);
        check_decoder_8x256(true, true, true, false, true, false, true, true, 190);
        check_decoder_8x256(true, true, true, true, true, false, true, true, 191);
        check_decoder_8x256(false, false, false, false, true, true, false, false, 192);
        check_decoder_8x256(false, false, false, true, true, true, false, false, 193);
        check_decoder_8x256(false, false, true, false, true, true, false, false, 194);
        check_decoder_8x256(false, false, true, true, true, true, false, false, 195);
        check_decoder_8x256(false, true, false, false, true, true, false, false, 196);
        check_decoder_8x256(false, true, false, true, true, true, false, false, 197);
        check_decoder_8x256(false, true, true, false, true, true, false, false, 198);
        check_decoder_8x256(false, true, true, true, true, true, false, false, 199);
        check_decoder_8x256(true, false, false, false, true, true, false, false, 200);
        check_decoder_8x256(true, false, false, true, true, true, false, false, 201);
        check_decoder_8x256(true, false, true, false, true, true, false, false, 202);
        check_decoder_8x256(true, false, true, true, true, true, false, false, 203);
        check_decoder_8x256(true, true, false, false, true, true, false, false, 204);
        check_decoder_8x256(true, true, false, true, true, true, false, false, 205);
        check_decoder_8x256(true, true, true, false, true, true, false, false, 206);
        check_decoder_8x256(true, true, true, true, true, true, false, false, 207);
        check_decoder_8x256(false, false, false, false, true, true, false, true, 208);
        check_decoder_8x256(false, false, false, true, true, true, false, true, 209);
        check_decoder_8x256(false, false, true, false, true, true, false, true, 210);
        check_decoder_8x256(false, false, true, true, true, true, false, true, 211);
        check_decoder_8x256(false, true, false, false, true, true, false, true, 212);
        check_decoder_8x256(false, true, false, true, true, true, false, true, 213);
        check_decoder_8x256(false, true, true, false, true, true, false, true, 214);
        check_decoder_8x256(false, true, true, true, true, true, false, true, 215);
        check_decoder_8x256(true, false, false, false, true, true, false, true, 216);
        check_decoder_8x256(true, false, false, true, true, true, false, true, 217);
        check_decoder_8x256(true, false, true, false, true, true, false, true, 218);
        check_decoder_8x256(true, false, true, true, true, true, false, true, 219);
        check_decoder_8x256(true, true, false, false, true, true, false, true, 220);
        check_decoder_8x256(true, true, false, true, true, true, false, true, 221);
        check_decoder_8x256(true, true, true, false, true, true, false, true, 222);
        check_decoder_8x256(true, true, true, true, true, true, false, true, 223);
        check_decoder_8x256(false, false, false, false, true, true, true, false, 224);
        check_decoder_8x256(false, false, false, true, true, true, true, false, 225);
        check_decoder_8x256(false, false, true, false, true, true, true, false, 226);
        check_decoder_8x256(false, false, true, true, true, true, true, false, 227);
        check_decoder_8x256(false, true, false, false, true, true, true, false, 228);
        check_decoder_8x256(false, true, false, true, true, true, true, false, 229);
        check_decoder_8x256(false, true, true, false, true, true, true, false, 230);
        check_decoder_8x256(false, true, true, true, true, true, true, false, 231);
        check_decoder_8x256(true, false, false, false, true, true, true, false, 232);
        check_decoder_8x256(true, false, false, true, true, true, true, false, 233);
        check_decoder_8x256(true, false, true, false, true, true, true, false, 234);
        check_decoder_8x256(true, false, true, true, true, true, true, false, 235);
        check_decoder_8x256(true, true, false, false, true, true, true, false, 236);
        check_decoder_8x256(true, true, false, true, true, true, true, false, 237);
        check_decoder_8x256(true, true, true, false, true, true, true, false, 238);
        check_decoder_8x256(true, true, true, true, true, true, true, false, 239);
        check_decoder_8x256(false, false, false, false, true, true, true, true, 240);
        check_decoder_8x256(false, false, false, true, true, true, true, true, 241);
        check_decoder_8x256(false, false, true, false, true, true, true, true, 242);
        check_decoder_8x256(false, false, true, true, true, true, true, true, 243);
        check_decoder_8x256(false, true, false, false, true, true, true, true, 244);
        check_decoder_8x256(false, true, false, true, true, true, true, true, 245);
        check_decoder_8x256(false, true, true, false, true, true, true, true, 246);
        check_decoder_8x256(false, true, true, true, true, true, true, true, 247);
        check_decoder_8x256(true, false, false, false, true, true, true, true, 248);
        check_decoder_8x256(true, false, false, true, true, true, true, true, 249);
        check_decoder_8x256(true, false, true, false, true, true, true, true, 250);
        check_decoder_8x256(true, false, true, true, true, true, true, true, 251);
        check_decoder_8x256(true, true, false, false, true, true, true, true, 252);
        check_decoder_8x256(true, true, false, true, true, true, true, true, 253);
        check_decoder_8x256(true, true, true, false, true, true, true, true, 254);
        check_decoder_8x256(true, true, true, true, true, true, true, true, 255);
    }
}
