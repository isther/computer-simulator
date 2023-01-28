use super::Wire;

#[derive(Debug)]
struct NAND {
    output: Wire,
}

impl NAND {
    fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    fn get(&self) -> bool {
        self.output.get()
    }

    fn update(&mut self, a: bool, b: bool) -> bool {
        self.output.update(!(a & b));
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_gate() {
        let mut nand_gate = NAND::new();
        assert_eq!(nand_gate.update(false, false), true);
        assert_eq!(nand_gate.update(false, true), true);
        assert_eq!(nand_gate.update(true, false), true);
        assert_eq!(nand_gate.update(true, true), false);
    }
}
