use super::Wire;

#[derive(Debug)]
struct NOR {
    output: Wire,
}

impl NOR {
    fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    fn get(&self) -> bool {
        self.output.get()
    }

    fn update(&mut self, a: bool, b: bool) -> bool {
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
