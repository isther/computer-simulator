use super::{Enableable, Settable};
use crate::computer::gates::Wire;

pub enum Mode {
    ClockSet,
    ClockEnable,
    Mode,
    DataOrAddress,
}

impl From<usize> for Mode {
    fn from(value: usize) -> Self {
        match value {
            0 => Mode::ClockSet,
            1 => Mode::ClockEnable,
            2 => Mode::Mode,
            3 => Mode::DataOrAddress,
            _ => Mode::Mode,
        }
    }
}

impl From<Mode> for i32 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::ClockSet => 0,
            Mode::ClockEnable => 1,
            Mode::Mode => 2,
            Mode::DataOrAddress => 3,
        }
    }
}

pub struct IOBus {
    wires: [Wire; 4],
}

impl IOBus {
    pub fn new() -> Self {
        Self {
            wires: (0..4)
                .map(|_| Wire::new("".to_string(), false))
                .collect::<Vec<Wire>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn is_set(&self) -> bool {
        self.wires[Mode::ClockSet as usize].get()
    }

    pub fn is_enable(&self) -> bool {
        self.wires[Mode::ClockEnable as usize].get()
    }

    pub fn is_input_mode(&self) -> bool {
        self.wires[Mode::Mode as usize].get() == false
    }

    pub fn is_output_mode(&self) -> bool {
        self.wires[Mode::Mode as usize].get() == true
    }

    pub fn is_data_mode(&self) -> bool {
        self.wires[Mode::DataOrAddress as usize].get() == false
    }

    pub fn is_address_mode(&self) -> bool {
        self.wires[Mode::DataOrAddress as usize].get() == true
    }

    /// mode:
    ///     false: input
    ///     true: output
    /// data_or_address:
    ///     false: data
    ///     true: address
    pub fn update(&mut self, mode: bool, data_or_address: bool) {
        self.wires[Mode::Mode as usize].update(mode);
        self.wires[Mode::DataOrAddress as usize].update(data_or_address);
    }

    pub fn get_output_wire(&self, index: i32) -> bool {
        self.wires[index as usize].get()
    }
}

impl Enableable for IOBus {
    fn enable(&mut self) {
        self.wires[Mode::ClockEnable as usize].update(true)
    }

    fn disable(&mut self) {
        self.wires[Mode::ClockEnable as usize].update(false)
    }
}

impl Settable for IOBus {
    fn set(&mut self) {
        self.wires[Mode::ClockSet as usize].update(true)
    }

    fn unset(&mut self) {
        self.wires[Mode::ClockSet as usize].update(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_bus() {
        let mut io_bus = IOBus::new();
        io_bus.set();
        assert_eq!(io_bus.is_set(), true);

        io_bus.unset();
        assert_eq!(io_bus.is_set(), false);

        io_bus.enable();
        assert_eq!(io_bus.is_enable(), true);

        io_bus.disable();
        assert_eq!(io_bus.is_enable(), false);

        io_bus.update(false, false);
        assert_eq!(io_bus.is_input_mode(), true);
        assert_eq!(io_bus.is_data_mode(), true);

        io_bus.update(true, true);
        assert_eq!(io_bus.is_output_mode(), true);
        assert_eq!(io_bus.is_address_mode(), true);
    }
}
