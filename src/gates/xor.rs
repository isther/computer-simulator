use super::Wire;

#[derive(Debug, Clone)]
pub struct XOR {
    output: Wire,
}

impl XOR {
    pub fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, a: bool, b: bool) -> bool {
        self.output.update(!((!a & !b) || (a & b)));
        // g.output.Update(!((!inputA && !inputB) || (inputA && inputB)))
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_gate() {
        let mut xor_gate = XOR::new();
        assert_eq!(xor_gate.update(false, false), false);
        assert_eq!(xor_gate.update(false, true), true);
        assert_eq!(xor_gate.update(true, false), true);
        assert_eq!(xor_gate.update(true, true), false);
    }
}
