use crate::components::{Bus, Decoder8x256, Register};
use crate::gates::Wire;
use crate::gates::AND;
use std::cell::RefCell;

pub const BUS_WIDTH: i32 = 16;

struct Cell<'a> {
    value: Register<'a>,
    gates: [AND; 3],
}

impl<'a> Cell<'a> {
    fn new(input_bus: &'a RefCell<Bus>, output_bus: &'a RefCell<Bus>) -> Self {
        Self {
            value: Register::new("", input_bus, output_bus),
            gates: (0..3)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
        }
    }

    fn update(&mut self, set: bool, enable: bool) {
        self.gates[0].update(true, true);
        self.gates[1].update(self.gates[0].get(), set);
        self.gates[2].update(self.gates[0].get(), enable);

        match self.gates[1].get() {
            true => self.value.set(),
            false => self.value.unset(),
        }

        match self.gates[2].get() {
            true => self.value.enable(),
            false => self.value.disable(),
        }
    }
}

struct Memory64K<'a> {
    address_register: Register<'a>,
    row_decoder: Decoder8x256,
    col_decoder: Decoder8x256,
    // data: [[Cell<'a>; 256]; 256],
    data: [Cell<'a>; 256],
    set: Wire,
    enable: Wire,
    bus: &'a RefCell<Bus>,
}

impl<'a> Memory64K<'a> {
    fn new(bus: &'a RefCell<Bus>) -> Self {
        Self {
            address_register: Register::new("MAR", bus, bus),
            row_decoder: Decoder8x256::new(),
            col_decoder: Decoder8x256::new(),
            data: (0..256)
                .map(|_| Cell::new(bus, bus))
                .collect::<Vec<Cell>>()
                .try_into()
                .unwrap(),
            set: Wire::new("Z".to_string(), false),
            enable: Wire::new("Z".to_string(), false),
            bus,
        }
    }
    // func NewMemory64K(bus *components.Bus) *Memory64K {
    // 	m := new(Memory64K)
    // 	m.AddressRegister = *components.NewRegister("MAR", bus, bus)
    // 	m.rowDecoder = *components.NewDecoder8x256()
    // 	m.colDecoder = *components.NewDecoder8x256()
    // 	m.bus = bus
    //
    // 	for i := 0; i < 256; i++ {
    // 		for j := 0; j < 256; j++ {
    // 			m.data[i][j] = *NewCell(bus, bus)
    // 		}
    // 	}
    //
    // 	return m
    // }
    //
}

// func (m *Memory64K) Enable() {
// 	m.enable.Update(true)
// }
//
// func (m *Memory64K) Disable() {
// 	m.enable.Update(false)
// }
//
// func (m *Memory64K) Set() {
// 	m.set.Update(true)
// }
//
// func (m *Memory64K) Unset() {
// 	m.set.Update(false)
// }
//
// func (m *Memory64K) Update() {
// 	m.AddressRegister.Update()
// 	m.rowDecoder.Update(
// 		m.AddressRegister.Bit(0),
// 		m.AddressRegister.Bit(1),
// 		m.AddressRegister.Bit(2),
// 		m.AddressRegister.Bit(3),
// 		m.AddressRegister.Bit(4),
// 		m.AddressRegister.Bit(5),
// 		m.AddressRegister.Bit(6),
// 		m.AddressRegister.Bit(7),
// 	)
// 	m.colDecoder.Update(
// 		m.AddressRegister.Bit(8),
// 		m.AddressRegister.Bit(9),
// 		m.AddressRegister.Bit(10),
// 		m.AddressRegister.Bit(11),
// 		m.AddressRegister.Bit(12),
// 		m.AddressRegister.Bit(13),
// 		m.AddressRegister.Bit(14),
// 		m.AddressRegister.Bit(15),
// 	)
//
// 	var row int = m.rowDecoder.Index()
// 	var col int = m.colDecoder.Index()
//
// 	m.data[row][col].Update(m.set.Get(), m.enable.Get())
// }
//
// func (m *Memory64K) String() string {
// 	var row int = m.rowDecoder.Index()
// 	var col int = m.colDecoder.Index()
//
// 	var builder strings.Builder
// 	builder.WriteString(fmt.Sprint("Memory\n--------------------------------------\n"))
// 	builder.WriteString(fmt.Sprintf("RD: %d\tCD: %d\tS: %v\tE: %v\t%s\n", row, col, m.set.Get(), m.enable.Get(), m.AddressRegister.String()))
//
// 	for i := 0; i < 256; i++ {
// 		for j := 0; j < 256; j++ {
// 			val := m.data[i][j].value.Value()
// 			if val <= 0x000F {
// 				builder.WriteString(fmt.Sprintf("0x000%X\t", val))
// 			} else if val <= 0x00FF {
// 				builder.WriteString(fmt.Sprintf("0x00%X\t", val))
// 			} else if val <= 0x0FFF {
// 				builder.WriteString(fmt.Sprintf("0x0%X\t", val))
// 			} else {
// 				builder.WriteString(fmt.Sprintf("0x%X\t", val))
// 			}
// 		}
// 		builder.WriteString(fmt.Sprint("\n"))
//
// 	}
// 	return builder.String()
//
// }
