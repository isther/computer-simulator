mod and;
mod nand;
mod nor;
mod not;
mod or;
mod xor;

pub use and::AND;
pub use nand::NAND;
pub use nor::NOR;
pub use not::NOT;
pub use or::OR;
pub use xor::XOR;

#[derive(Debug, Clone)]
pub struct Wire {
    name: String,
    value: bool,
}

impl Wire {
    pub fn new(name: String, value: bool) -> Self {
        Self { name, value }
    }

    pub fn get(&self) -> bool {
        self.value
    }

    pub fn update(&mut self, value: bool) {
        self.value = value
    }
}
