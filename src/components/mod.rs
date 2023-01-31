mod adder;

const BUS_WIDTH: i32 = 16;

trait Component {
    fn connect_output(&mut self, component: Box<dyn Component>);
    fn set_input_wire(&mut self, i: i32, value: bool);
    fn get_output_wire(&self, i: i32) -> bool;
}

struct EmptyComponent {}

impl EmptyComponent {
    fn new() -> Self {
        Self {}
    }
}

impl Component for EmptyComponent {
    fn connect_output(&mut self, _: Box<dyn Component>) {}
    fn set_input_wire(&mut self, _: i32, _: bool) {}
    fn get_output_wire(&self, _: i32) -> bool {
        todo!()
    }
}

fn set_input_value(c: &mut dyn Component, value: i32, start: i32, end: i32) {
    let mut x: u16 = 0;
    let mut i = start - 1;
    while i >= end {
        match value & (1 << x) {
            0 => {
                c.set_input_wire(i, false);
            }
            _ => {
                c.set_input_wire(i, true);
            }
        }

        i = i - 1;
        x = x + 1;
    }
}

fn set_component_value_16(c: &mut dyn Component, value: i32) {
    set_input_value(c, value, 16, 0);
}

fn set_component_value_32(c: &mut dyn Component, input_a: i32, input_b: i32) {
    set_input_value(c, input_a, 16, 0);
    set_input_value(c, input_b, 32, 16);
}

fn get_output_value(c: &mut dyn Component, output_bits: i32) -> i32 {
    let mut x: u16 = 0;
    let mut i = output_bits - 1;
    let mut result: i32 = 0;
    while i >= 0 {
        match c.get_output_wire(i) {
            true => result = result | (1 << x),
            false => result = result ^ (result & (1 << x)),
        };
        x += 1;
        i -= 1;
    }
    result
}
