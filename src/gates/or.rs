use super::Wire;

#[derive(Debug)]
struct OR {
    output: Wire,
}

impl OR {
    fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    fn get(&self) -> bool {
        self.output.get()
    }

    fn update(&mut self, a: bool, b: bool) -> bool {
        self.output.update(a | b);
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or_gate() {
        let mut or_gate = OR::new();
        assert_eq!(or_gate.update(false, false), false);
        assert_eq!(or_gate.update(false, true), true);
        assert_eq!(or_gate.update(true, false), true);
        assert_eq!(or_gate.update(true, true), true);
    }
}
