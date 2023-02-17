use super::Wire;

#[derive(Debug)]
pub struct NOR {
    pub output: Wire,
}

impl NOR {
    pub fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, a: bool, b: bool) -> bool {
        self.output.update(!(a | b));
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nor_gate() {
        let mut nor_gate = NOR::new();
        assert_eq!(nor_gate.update(false, false), true);
        assert_eq!(nor_gate.update(false, true), false);
        assert_eq!(nor_gate.update(true, false), false);
        assert_eq!(nor_gate.update(true, true), false);
    }
}
