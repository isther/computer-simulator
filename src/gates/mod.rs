mod not;
mod and;
mod nand;
mod or;
mod nor;
mod xor;

#[derive(Debug)]
struct Wire {
    name: String,
    value: bool,
}

impl Wire {
    fn new(name: String, value: bool) -> Self {
        Self { name, value }
    }

    fn get(&self) -> bool {
        self.value
    }

    fn update(&mut self, value: bool) {
        self.value = value
    }
}
