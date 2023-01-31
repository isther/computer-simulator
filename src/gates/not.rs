use super::Wire;

#[derive(Debug)]
pub struct NOT {
    output: Wire,
}

impl NOT {
    pub fn new() -> Self {
        Self {
            output: Wire::new("Z".to_string(), false),
        }
    }

    pub fn get(&self) -> bool {
        self.output.get()
    }

    pub fn update(&mut self, a: bool) -> bool {
        self.output.update(!a);
        self.output.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_gate() {
        let mut not_gate = NOT::new();
        assert_eq!(not_gate.update(false), true);
        assert_eq!(not_gate.update(true), false);
    }
}
