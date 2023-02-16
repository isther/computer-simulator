use super::{ANDGate3, ANDGate4, Component};
use crate::gates::{Wire, NOT};
#[derive(Debug, Clone)]
pub struct Decoder3x8 {
    input_a: Wire,
    input_b: Wire,
    input_c: Wire,

    not_gates: [NOT; 3],
    and_gates: [ANDGate3; 8],
    outputs: [Wire; 8],
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
struct Decoder4x16 {
    not_gates: [NOT; 4],
    and_gates: [ANDGate4; 16],
    outputs: [Wire; 16],
    index: i32,
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

impl Component for Decoder4x16 {
    fn set_input_wire(&mut self, _: i32, _: bool) {}
    fn get_output_wire(&self, i: i32) -> bool {
        self.outputs[i as usize].get()
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

    fn check_decoder_3x8(input_a: bool, input_b: bool, input_c: bool, ans: i32) {
        let mut expected: [bool; 16] = [false; 16];
        expected[ans as usize] = true;

        let mut decoder_3x8 = Decoder3x8::new();
        decoder_3x8.update(input_a, input_b, input_c);
        for i in 0..decoder_3x8.outputs.len() {
            assert_eq!(decoder_3x8.outputs[i].get(), expected[i])
        }
    }

    #[test]
    fn test_decoder_3x8() {
        check_decoder_3x8(false, false, false, 0);
        check_decoder_3x8(false, false, true, 1);
        check_decoder_3x8(false, true, false, 2);
        check_decoder_3x8(false, true, true, 3);

        check_decoder_3x8(true, false, false, 4);
        check_decoder_3x8(true, false, true, 5);
        check_decoder_3x8(true, true, false, 6);
        check_decoder_3x8(true, true, true, 7);
    }
}
